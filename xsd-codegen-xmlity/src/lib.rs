mod complex;
pub mod misc;
mod simple;
mod types;

use std::collections::HashMap;

use complex::ComplexTypeToRustType;
use misc::{GeneratedFragment, TypeReference};
use quote::format_ident;
use simple::SimpleTypeToRustType;
use syn::{parse_quote, Ident, Item};
use types::TypeItem;
use xmlity::{ExpandedName, XmlNamespace};
use xsd_type_compiler::CompiledNamespace;

#[derive(Debug, Clone, PartialEq)]
pub struct Error {}
pub type Result<T> = std::result::Result<T, Error>;

pub struct Generator<'a> {
    pub context: &'a xsd_type_compiler::XmlnsContext,
    pub bound_namespaces: HashMap<XmlNamespace<'static>, Ident>,
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
}

impl<'a> GeneratorContext<'a> {
    fn new(generator: &'a Generator<'a>, namespace: &'a XmlNamespace<'a>) -> Self {
        Self {
            generator,
            namespace,
        }
    }

    fn current_namespace(&self) -> &CompiledNamespace {
        self.generator
            .context
            .namespaces
            .get(self.namespace)
            .unwrap()
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
    fn get_complex_fragment(
        &self,
        fragment_id: &xsd_type_compiler::complex::FragmentId,
    ) -> Result<Option<misc::GeneratedFragment>> {
        let fragment = self
            .current_namespace()
            .complex_type
            .fragments
            .get(&fragment_id.1);

        let Some(fragment) = fragment else {
            return Ok(None);
        };

        fragment.generate_complex_rust_types(self).map(Some)
    }
}

impl<'a> Generator<'a> {
    pub fn new(context: &'a xsd_type_compiler::XmlnsContext) -> Self {
        Self {
            context,
            bound_namespaces: HashMap::new(),
        }
    }

    pub fn generate_namespace(&self, namespace: &xmlity::XmlNamespace<'_>) -> Result<Vec<Item>> {
        let mut items = Vec::new();

        let compiled_namespace = self.context.namespaces.get(namespace).unwrap(); // TODO: handle this error properly with a better error messa

        for local_name in compiled_namespace.top_level_types.keys() {
            let expanded_name = ExpandedName::new(local_name.as_ref(), Some(namespace.as_ref()));
            let item = self.generate_top_level_type(&expanded_name)?;
            items.push(item)
        }

        for local_name in compiled_namespace.top_level_attributes.keys() {
            let expanded_name = ExpandedName::new(local_name.as_ref(), Some(namespace.as_ref()));
            let item = self.generate_top_level_type(&expanded_name)?;
            items.push(item)
        }

        for local_name in compiled_namespace.top_level_elements.keys() {
            let expanded_name = ExpandedName::new(local_name.as_ref(), Some(namespace.as_ref()));
            let item = self.generate_top_level_type(&expanded_name)?;
            items.push(item)
        }

        //TODO
        Ok(vec![])
    }

    pub fn generate_top_level_type(
        &self,
        name: &xmlity::ExpandedName<'_>,
    ) -> Result<(TypeReference<'static>, Vec<Item>)> {
        let compiled_namespace = self
            .context
            .namespaces
            .get(name.namespace().unwrap())
            .unwrap_or_else(|| panic!("namespace not found: {}", name.namespace().unwrap())); // TODO: handle this error properly with a better error messa

        let top_level_type = compiled_namespace
            .top_level_types
            .get(name.local_name())
            .unwrap(); // TODO: handle this error properly with a better error messa

        match top_level_type {
            xsd_type_compiler::TopLevelType::Simple(top_level_simple_type) => todo!(),
            xsd_type_compiler::TopLevelType::Complex(top_level_complex_type) => {
                let fragment = compiled_namespace
                    .complex_type
                    .get_fragment(&top_level_complex_type.root_fragment)
                    .unwrap();

                let context = GeneratorContext::new(self, name.namespace().unwrap());

                let type_ = fragment.generate_complex_rust_types(&context)?;
                let (type_, items) = type_
                    .type_
                    .into_type(&format_ident!("{}", name.local_name().to_string()), None);

                Ok((type_, items))
            }
        }
    }

    pub fn generate_top_level_attribute(
        &self,
        name: &xmlity::ExpandedName<'_>,
    ) -> Result<misc::GeneratedFragment> {
        Ok(todo!())
    }

    pub fn generate_top_level_element(
        &self,
        name: &xmlity::ExpandedName<'_>,
    ) -> Result<misc::GeneratedFragment> {
        Ok(todo!())
    }
}
