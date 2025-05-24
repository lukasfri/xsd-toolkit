mod complex;
pub mod misc;
mod simple;
pub mod templates;

use std::collections::HashMap;

use complex::{ToTypeTemplate, ToTypeTemplateData};
use misc::TypeReference;
use quote::format_ident;
use simple::{Context, SimpleTypeToRustType};
use syn::{parse_quote, Ident, Item, ItemMod};
use xmlity::{ExpandedName, XmlNamespace};
use xsd_type_compiler::{
    complex::{ComplexTypeFragmentCompiler, FragmentAccess},
    CompiledNamespace,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Error {}
pub type Result<T> = std::result::Result<T, Error>;

pub struct Generator<'a> {
    pub context: &'a xsd_type_compiler::XmlnsContext,
    pub bound_namespaces: HashMap<XmlNamespace<'static>, Ident>,
    pub bound_types: HashMap<ExpandedName<'static>, syn::Type>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum NamedTypeClass {
    Type,
    Element,
    Attribute,
}

struct GeneratorContext<'a> {
    generator: &'a Generator<'a>,
    namespace: &'a XmlNamespace<'a>,
    items: Vec<Item>,
    mod_name: Option<Ident>,
}

impl<'a> GeneratorContext<'a> {
    fn new(
        generator: &'a Generator<'a>,
        namespace: &'a XmlNamespace<'a>,
        mod_name: Option<Ident>,
    ) -> Self {
        Self {
            generator,
            namespace,
            items: Vec::new(),
            mod_name,
        }
    }

    fn current_namespace(&self) -> &CompiledNamespace {
        self.generator
            .context
            .namespaces
            .get(self.namespace)
            .unwrap()
    }

    fn sub_context(&self, mod_name: Option<Ident>) -> Self {
        Self {
            generator: self.generator,
            namespace: self.namespace,
            items: Vec::new(),
            mod_name,
        }
    }

    fn finish(self) -> std::result::Result<ItemMod, Vec<Item>> {
        let items = self.items;

        if items.is_empty() {
            return Err(items);
        }

        let Some(mod_name) = self.mod_name else {
            return Err(items);
        };

        Ok(parse_quote! {
            pub mod #mod_name {
                #(#items)*
            }
        })
    }
}

impl simple::Context for GeneratorContext<'_> {
    fn get_named_type(
        &self,
        expanded_name: &ExpandedName<'_>,
        class: NamedTypeClass,
    ) -> Result<Option<syn::Type>> {
        let namespace_ident = self
            .generator
            .bound_namespaces
            .get(expanded_name.namespace().unwrap());

        let Some(namespace_ident) = namespace_ident else {
            return Ok(None);
        };

        let local_name_ident = Ident::new(
            &expanded_name.local_name().to_string(),
            proc_macro2::Span::call_site(),
        );

        Ok(Some(parse_quote!(#namespace_ident::#local_name_ident)))
    }

    fn get_simple_fragment(
        &self,
        fragment_id: &xsd_type_compiler::simple::FragmentId,
    ) -> Result<Option<misc::GeneratedFragment>> {
        let fragment = self
            .current_namespace()
            .complex_type
            .simple_type_fragments
            .fragments
            .get(&fragment_id.1);

        let Some(fragment) = fragment else {
            return Ok(None);
        };

        fragment.generate_simple_rust_types(self).map(Some)
    }

    fn namespace(&self) -> &XmlNamespace<'_> {
        self.namespace
    }
}

impl complex::Context for GeneratorContext<'_> {
    fn resolve_fragment<F>(&self, fragment: &F) -> Result<ToTypeTemplateData<F::TypeTemplate>>
    where
        F: ToTypeTemplate,
    {
        fragment.to_type_template(self)
    }

    fn resolve_fragment_id<F>(
        &self,
        fragment_id: &xsd_type_compiler::complex::FragmentIdx<F>,
    ) -> Result<ToTypeTemplateData<F::TypeTemplate>>
    where
        F: ToTypeTemplate,
        ComplexTypeFragmentCompiler: FragmentAccess<F>,
    {
        let fragment = self
            .current_namespace()
            .complex_type
            .get_fragment(fragment_id)
            .unwrap();

        fragment.to_type_template(self)
    }

    fn resolve_named_type(&self, name: &ExpandedName<'_>) -> Result<syn::Type> {
        if let Some(bound_type) = self.generator.bound_types.get(name) {
            return Ok(bound_type.clone());
        }

        todo!()
    }

    fn to_expanded_name(&self, local_name: xmlity::LocalName<'static>) -> ExpandedName<'static> {
        ExpandedName::new(local_name, Some(self.namespace().clone().into_owned()))
    }
    // fn resolve_fragment<F>(
    //     &self,
    //     fragment_id: &xsd_type_compiler::complex::FragmentIdx<F>,
    // ) -> Result<Option<misc::GeneratedFragment>>
    // where
    //     xsd_type_compiler::complex::ComplexTypeFragmentCompiler: FragmentAccess<F>,
    // {
    //     // let fragment = self
    //     //     .current_namespace()
    //     //     .complex_type
    //     //     .fragments
    //     //     .get(&fragment_id.1);

    //     // let Some(fragment) = fragment else {
    //     //     return Ok(None);
    //     // };

    //     // fragment.generate_complex_rust_types(self).map(Some)
    //     todo!()
    // }
}

impl<'a> Generator<'a> {
    pub fn new(context: &'a xsd_type_compiler::XmlnsContext) -> Self {
        Self {
            context,
            bound_namespaces: HashMap::new(),
            bound_types: HashMap::new(),
        }
    }

    pub fn bind_type(&mut self, name: ExpandedName<'static>, ty: syn::Type) {
        self.bound_types.insert(name, ty);
    }

    pub fn generate_namespace(&self, namespace: &xmlity::XmlNamespace<'_>) -> Result<Vec<Item>> {
        let mut items = Vec::new();

        let compiled_namespace = self.context.namespaces.get(namespace).unwrap(); // TODO: handle this error properly with a better error messa

        for local_name in compiled_namespace.top_level_types.keys() {
            let expanded_name = ExpandedName::new(local_name.as_ref(), Some(namespace.as_ref()));
            let (_, i) = self.generate_top_level_type(&expanded_name)?;
            items.extend(i)
        }

        for local_name in compiled_namespace.top_level_attributes.keys() {
            let expanded_name = ExpandedName::new(local_name.as_ref(), Some(namespace.as_ref()));
            let (_, i) = self.generate_top_level_type(&expanded_name)?;
            items.extend(i)
        }

        for local_name in compiled_namespace.top_level_elements.keys() {
            let expanded_name = ExpandedName::new(local_name.as_ref(), Some(namespace.as_ref()));
            let (_, i) = self.generate_top_level_type(&expanded_name)?;
            items.extend(i)
        }

        //TODO
        Ok(items)
    }

    pub fn generate_top_level_type(
        &self,
        name: &xmlity::ExpandedName<'_>,
    ) -> Result<(TypeReference<'static>, Vec<Item>)> {
        let compiled_namespace = self
            .context
            .namespaces
            .get(name.namespace().unwrap())
            .unwrap_or_else(|| panic!("namespace not found: {}", name.namespace().unwrap()));
        // TODO: handle this error properly with a better error messa

        let type_ = compiled_namespace
            .top_level_types
            .get(name.local_name())
            .unwrap(); // TODO: handle this error properly with a better error messa

        match type_ {
            xsd_type_compiler::TopLevelType::Simple(type_) => todo!(),
            xsd_type_compiler::TopLevelType::Complex(type_) => {
                let fragment = compiled_namespace
                    .complex_type
                    .get_fragment(&type_.root_fragment)
                    .unwrap();

                let local_name = name.local_name().to_string();
                let module_name = format_ident!("{local_name}_items");
                let context =
                    GeneratorContext::new(self, name.namespace().unwrap(), Some(module_name));

                let type_ = fragment.to_type_template(&context)?;
                let struct_name = format_ident!("{local_name}");
                let item = type_.template.to_struct(&struct_name);
                let mut items = context
                    .finish()
                    .map(|i| vec![Item::Mod(i)])
                    .unwrap_or_else(std::convert::identity);

                items.push(Item::Struct(item));

                let type_ = TypeReference::new_prefix(parse_quote!(#struct_name));

                Ok((type_, items))
            }
        }
    }

    pub fn generate_top_level_attribute(
        &self,
        name: &xmlity::ExpandedName<'_>,
    ) -> Result<(TypeReference<'static>, Vec<Item>)> {
        let compiled_namespace = self
            .context
            .namespaces
            .get(name.namespace().unwrap())
            .unwrap_or_else(|| panic!("namespace not found: {}", name.namespace().unwrap()));
        // TODO: handle this error properly with a better error messa

        let attribute = compiled_namespace
            .top_level_attributes
            .get(name.local_name())
            .unwrap(); // TODO: handle this error properly with a better error messa

        let fragment = compiled_namespace
            .complex_type
            .get_fragment(&attribute.root_fragment)
            .unwrap();

        let local_name = name.local_name().to_string();
        let module_name = format_ident!("{local_name}_items");
        let context = GeneratorContext::new(self, name.namespace().unwrap(), Some(module_name));

        let type_ = fragment.to_type_template(&context)?;
        let struct_name = format_ident!("{local_name}");
        let item = type_.template.to_struct(&struct_name);
        let mut items = context
            .finish()
            .map(|i| vec![Item::Mod(i)])
            .unwrap_or_else(std::convert::identity);

        items.push(Item::Struct(item));

        let type_ = TypeReference::new_prefix(parse_quote!(#struct_name));

        Ok((type_, items))
    }

    pub fn generate_top_level_element(
        &self,
        name: &xmlity::ExpandedName<'_>,
    ) -> Result<(TypeReference<'static>, Vec<Item>)> {
        let compiled_namespace = self
            .context
            .namespaces
            .get(name.namespace().unwrap())
            .unwrap_or_else(|| panic!("namespace not found: {}", name.namespace().unwrap()));
        // TODO: handle this error properly with a better error messa

        let element = compiled_namespace
            .top_level_elements
            .get(name.local_name())
            .unwrap(); // TODO: handle this error properly with a better error messa

        let fragment = compiled_namespace
            .complex_type
            .get_fragment(&element.root_fragment)
            .unwrap();

        let local_name = name.local_name().to_string();
        let module_name = format_ident!("{local_name}_items");
        let context = GeneratorContext::new(self, name.namespace().unwrap(), Some(module_name));

        let type_ = fragment.to_type_template(&context)?;
        let struct_name = format_ident!("{local_name}");
        let item = type_.template.to_struct(&struct_name);
        let mut items = context
            .finish()
            .map(|i| vec![Item::Mod(i)])
            .unwrap_or_else(std::convert::identity);

        items.push(Item::Struct(item));

        let type_ = TypeReference::new_prefix(parse_quote!(#struct_name));

        Ok((type_, items))
    }
}
