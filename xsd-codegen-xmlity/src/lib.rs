pub mod binds;
mod complex;
pub mod misc;
mod simple;
pub mod templates;

use std::collections::HashMap;

use complex::{Scope, ToTypeTemplate, ToTypeTemplateData};
use inflector::Inflector;
use misc::TypeReference;
use quote::format_ident;
use syn::{parse_quote, Ident, Item, ItemMod};
use xmlity::{ExpandedName, LocalName, XmlNamespace};
use xsd_type_compiler::{
    complex::{ComplexTypeFragmentCompiler, FragmentAccess},
    CompiledNamespace,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Error {}
pub type Result<T> = std::result::Result<T, Error>;

pub struct Generator<'a> {
    pub context: &'a xsd_type_compiler::XmlnsContext,
    pub bound_namespaces: HashMap<XmlNamespace<'static>, syn::Path>,
    pub bound_types: HashMap<ExpandedName<'static>, BoundType>,
    pub bound_elements: HashMap<ExpandedName<'static>, syn::Type>,
    pub bound_attributes: HashMap<ExpandedName<'static>, syn::Type>,
    pub bound_groups: HashMap<ExpandedName<'static>, syn::Type>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TypeType {
    Simple,
    Complex,
}

pub trait ToIdentTypesExt {
    fn to_item_ident(&self) -> Ident;
    fn to_field_ident(&self) -> Ident;
    fn to_variant_ident(&self) -> Ident;
    fn to_path_ident(&self) -> Ident;
}

impl ToIdentTypesExt for LocalName<'_> {
    fn to_item_ident(&self) -> Ident {
        format_ident!(
            "{}",
            misc::unkeywordify(self.to_string().to_pascal_case().as_str())
        )
    }
    fn to_field_ident(&self) -> Ident {
        format_ident!(
            "{}",
            misc::unkeywordify(self.to_string().to_snake_case().as_str())
        )
    }

    fn to_variant_ident(&self) -> Ident {
        format_ident!(
            "{}",
            misc::unkeywordify(self.to_string().to_pascal_case().as_str())
        )
    }

    fn to_path_ident(&self) -> Ident {
        format_ident!(
            "{}",
            misc::unkeywordify(self.to_string().to_snake_case().as_str())
        )
    }
}

impl ToIdentTypesExt for Ident {
    fn to_item_ident(&self) -> Ident {
        format_ident!(
            "{}",
            misc::unkeywordify(self.to_string().to_pascal_case().as_str())
        )
    }

    fn to_field_ident(&self) -> Ident {
        format_ident!(
            "{}",
            misc::unkeywordify(self.to_string().to_snake_case().as_str())
        )
    }

    fn to_variant_ident(&self) -> Ident {
        format_ident!(
            "{}",
            misc::unkeywordify(self.to_string().to_pascal_case().as_str())
        )
    }

    fn to_path_ident(&self) -> Ident {
        format_ident!(
            "{}",
            misc::unkeywordify(self.to_string().to_snake_case().as_str())
        )
    }
}

struct GeneratorContext<'a> {
    generator: &'a Generator<'a>,
    namespace: &'a XmlNamespace<'a>,
    suggested_ident: Ident,
}

impl<'a> GeneratorContext<'a> {
    pub fn new(
        generator: &'a Generator<'a>,
        namespace: &'a XmlNamespace<'a>,
        suggested_ident: Ident,
    ) -> Self {
        Self {
            generator,
            namespace,
            suggested_ident,
        }
    }

    pub fn current_namespace(&self) -> &CompiledNamespace {
        self.generator
            .context
            .namespaces
            .get(self.namespace)
            .unwrap()
    }
}

impl<'c> complex::Context for GeneratorContext<'c> {
    type SubContext = GeneratorContext<'c>;

    fn namespace(&self) -> &XmlNamespace<'_> {
        self.namespace
    }

    fn sub_context(&self, suggested_ident: Ident) -> Self::SubContext {
        Self::new(self.generator, self.namespace, suggested_ident)
    }

    fn suggested_ident(&self) -> &Ident {
        &self.suggested_ident
    }

    fn resolve_fragment<F: ToTypeTemplate, S: Scope>(
        &self,
        fragment: &F,
        scope: &mut S,
    ) -> Result<ToTypeTemplateData<F::TypeTemplate>> {
        fragment.to_type_template(self, scope)
    }

    fn resolve_fragment_id<F: ToTypeTemplate, S: Scope>(
        &self,
        fragment_id: &xsd_type_compiler::complex::FragmentIdx<F>,
        scope: &mut S,
    ) -> Result<ToTypeTemplateData<F::TypeTemplate>>
    where
        ComplexTypeFragmentCompiler: FragmentAccess<F>,
    {
        let fragment = self
            .current_namespace()
            .complex_type
            .get_fragment(fragment_id)
            .unwrap();

        fragment.to_type_template(self, scope)
    }

    fn resolve_named_type(
        &self,
        name: &ExpandedName<'_>,
    ) -> Result<(TypeReference<'static>, TypeType)> {
        if let Some(bound_type) = self.generator.bound_types.get(&name.as_ref()).cloned() {
            let ty = TypeReference::new_static(bound_type.ty);

            return Ok((ty, bound_type.type_type));
        }

        let namespace_crate = self
            .generator
            .bound_namespaces
            .get(name.namespace().unwrap())
            .unwrap();

        let name = name.local_name().to_item_ident();
        let ty: syn::Type = parse_quote!(#namespace_crate::#name);

        Ok((TypeReference::new_static(ty), TypeType::Complex))
    }

    fn resolve_named_element(&self, name: &ExpandedName<'_>) -> Result<TypeReference<'static>> {
        if let Some(ty) = self.generator.bound_elements.get(&name.as_ref()).cloned() {
            let ty = TypeReference::new_static(ty);

            return Ok(ty);
        }

        let namespace_crate = self
            .generator
            .bound_namespaces
            .get(name.namespace().unwrap())
            .unwrap();

        let name = name.local_name().to_item_ident();
        let ty: syn::Type = parse_quote!(#namespace_crate::#name);

        Ok(TypeReference::new_static(ty))
    }

    fn resolve_named_attribute(&self, name: &ExpandedName<'_>) -> Result<TypeReference<'static>> {
        if let Some(ty) = self.generator.bound_attributes.get(&name.as_ref()).cloned() {
            let ty = TypeReference::new_static(ty);

            return Ok(ty);
        }

        let namespace_crate = self
            .generator
            .bound_namespaces
            .get(name.namespace().unwrap())
            .unwrap();

        let name = name.local_name().to_item_ident();
        let ty: syn::Type = parse_quote!(#namespace_crate::#name);

        Ok(TypeReference::new_static(ty))
    }

    fn to_expanded_name(&self, local_name: xmlity::LocalName<'static>) -> ExpandedName<'static> {
        ExpandedName::new(local_name, Some(self.namespace().clone().into_owned()))
    }

    fn resolve_named_group(&self, name: &ExpandedName<'_>) -> Result<TypeReference<'static>> {
        if let Some(ty) = self.generator.bound_groups.get(&name.as_ref()).cloned() {
            let ty = TypeReference::new_static(ty);

            return Ok(ty);
        }

        let namespace_crate = self
            .generator
            .bound_namespaces
            .get(name.namespace().unwrap())
            .unwrap();

        let name = name.local_name().to_item_ident();
        let ty: syn::Type = parse_quote!(#namespace_crate::#name);

        Ok(TypeReference::new_static(ty))
    }
}

#[derive(Debug)]
struct GeneratorScope {
    items: Vec<Item>,
}

impl complex::Scope for GeneratorScope {
    fn add_item<I: Into<Item>>(&mut self, item: I) -> Result<TypeReference<'static>> {
        let item: Item = item.into();

        let ident = match &item {
            Item::Struct(item) => &item.ident,
            Item::Enum(item) => &item.ident,
            Item::Mod(item) => &item.ident,
            _ => panic!("Unsupported item type"),
        };

        let ref_ = TypeReference::new_prefix(parse_quote!(#ident));

        self.items.push(item);

        Ok(ref_)
    }
}

impl GeneratorScope {
    fn new() -> Self {
        Self { items: Vec::new() }
    }

    fn finish(self) -> Vec<Item> {
        self.items
    }

    fn finish_mod(self, mod_name: &Ident) -> Option<ItemMod> {
        let items = self.finish();

        if items.is_empty() {
            return None;
        }

        Some(parse_quote!(
            pub mod #mod_name {
                #(#items)*
            }
        ))
    }
}

#[derive(Debug, Clone)]
pub struct BoundType {
    pub ty: syn::Type,
    pub type_type: TypeType,
    pub serialize_with: Option<syn::Path>,
    pub deserialize_with: Option<syn::Path>,
}

impl<'a> Generator<'a> {
    pub fn new(context: &'a xsd_type_compiler::XmlnsContext) -> Self {
        Self {
            context,
            bound_namespaces: HashMap::new(),
            bound_types: HashMap::new(),
            bound_elements: HashMap::new(),
            bound_attributes: HashMap::new(),
            bound_groups: HashMap::new(),
        }
    }

    pub fn bind_namespace(&mut self, namespace: XmlNamespace<'static>, path: syn::Path) {
        self.bound_namespaces.insert(namespace, path);
    }

    pub fn bind_type(&mut self, name: ExpandedName<'static>, bound_type: BoundType) {
        self.bound_types.insert(name, bound_type);
    }

    pub fn bind_types<T: IntoIterator<Item = (ExpandedName<'static>, BoundType)>>(
        &mut self,
        types: T,
    ) {
        for (name, bound_type) in types {
            self.bind_type(name, bound_type);
        }
    }

    pub fn bind_element(&mut self, name: ExpandedName<'static>, ty: syn::Type) {
        self.bound_elements.insert(name, ty);
    }

    pub fn bind_attribute(&mut self, name: ExpandedName<'static>, ty: syn::Type) {
        self.bound_attributes.insert(name, ty);
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
            xsd_type_compiler::TopLevelType::Simple(_type_) => todo!(),
            xsd_type_compiler::TopLevelType::Complex(type_) => {
                let fragment = compiled_namespace
                    .complex_type
                    .get_fragment(&type_.root_fragment)
                    .unwrap();

                let item_name = name.local_name().to_item_ident();
                let module_name = format_ident!("{}_items", name.local_name().to_path_ident());
                let context =
                    GeneratorContext::new(self, name.namespace().unwrap(), item_name.clone());
                let mut scope = GeneratorScope::new();

                let type_ = fragment.to_type_template(&context, &mut scope)?;
                let item = type_
                    .template
                    .to_struct(&item_name, Some(&parse_quote!(#module_name)));
                let mut items = scope
                    .finish_mod(&module_name)
                    .map(|i| vec![Item::Mod(i)])
                    .unwrap_or_default();

                items.push(Item::Struct(item));

                let type_ = TypeReference::new_prefix(parse_quote!(#item_name));

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

        let item_name = name.local_name().to_item_ident();
        let module_name = format_ident!("{}_items", name.local_name().to_path_ident());
        let context = GeneratorContext::new(self, name.namespace().unwrap(), item_name.clone());
        let mut scope = GeneratorScope::new();

        let type_ = fragment.to_type_template(&context, &mut scope)?;
        let item = type_
            .template
            .to_struct(&item_name, Some(&parse_quote!(#module_name)));
        let mut items = scope
            .finish_mod(&module_name)
            .map(|i| vec![Item::Mod(i)])
            .unwrap_or_default();

        items.push(Item::Struct(item));

        let type_ = TypeReference::new_prefix(parse_quote!(#item_name));

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

        let item_name = name.local_name().to_item_ident();
        let module_name = format_ident!("{}_items", name.local_name().to_path_ident());
        let context = GeneratorContext::new(self, name.namespace().unwrap(), item_name.clone());
        let mut scope = GeneratorScope::new();

        let type_ = fragment.to_type_template(&context, &mut scope)?;
        let item = type_
            .template
            .to_struct(&item_name, Some(&parse_quote!(#module_name)));
        let mut items = scope
            .finish_mod(&module_name)
            .map(|i| vec![Item::Mod(i)])
            .unwrap_or_default();

        items.push(Item::Struct(item));

        let type_ = TypeReference::new_prefix(parse_quote!(#item_name));

        Ok((type_, items))
    }
}
