use crate::{misc::GeneratedFragment, NamedTypeClass, Result};

use std::collections::HashMap;

use syn::Ident;
use xmlity::{ExpandedName, LocalName, XmlNamespace};
use xsd_type_compiler::simple;

pub struct SimpleTypeFragmentGenerator<'a> {
    pub context: &'a xsd_type_compiler::simple::SimpleTypeFragmentCompiler,
    pub bound_namespaces: &'a HashMap<XmlNamespace<'static>, Ident>,
}

impl SimpleTypeFragmentGenerator<'_> {
    pub fn generate(&self) -> Result<SimpleGeneratedNamespaceTypes> {
        Ok(SimpleGeneratedNamespaceTypes::new(
            self.context.namespace.clone(),
        ))
    }
}

pub struct SimpleGeneratedNamespaceTypes {
    pub namespace: XmlNamespace<'static>,
    pub types: HashMap<LocalName<'static>, GeneratedFragment>,
}

impl SimpleGeneratedNamespaceTypes {
    pub fn new(namespace: XmlNamespace<'static>) -> Self {
        Self {
            namespace,
            types: HashMap::new(),
        }
    }
}

pub trait Context {
    fn get_named_type(
        &self,
        expanded_name: &ExpandedName<'_>,
        class: NamedTypeClass,
    ) -> Result<Option<syn::Type>>;

    fn get_simple_fragment(
        &self,
        fragment_id: &simple::FragmentId,
    ) -> Result<Option<GeneratedFragment>>;

    fn namespace(&self) -> &XmlNamespace<'_>;
}

pub trait SimpleTypeToRustType {
    fn generate_simple_rust_types<C: Context>(&self, context: &C) -> Result<GeneratedFragment>;
}

impl SimpleTypeToRustType for simple::SimpleTypeFragment {
    fn generate_simple_rust_types<C: Context>(&self, context: &C) -> Result<GeneratedFragment> {
        match self {
            simple::SimpleTypeFragment::Restriction(restriction) => todo!(),
            simple::SimpleTypeFragment::List { item_ident } => todo!(),
            simple::SimpleTypeFragment::Union { fragments } => todo!(),
            simple::SimpleTypeFragment::Facet(facet) => todo!(),
        }
    }
}

#[cfg(test)]
mod tests {}
