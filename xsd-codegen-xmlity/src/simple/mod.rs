use crate::{misc::TypeReference, NamedTypeClass, Result};

use std::collections::HashMap;

use syn::{Ident, Item};
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
}

impl SimpleGeneratedNamespaceTypes {
    pub fn new(namespace: XmlNamespace<'static>) -> Self {
        Self { namespace }
    }
}

pub trait Context {
    fn get_named_type(
        &self,
        expanded_name: &ExpandedName<'_>,
        class: NamedTypeClass,
    ) -> Result<Option<syn::Type>>;

    fn namespace(&self) -> &XmlNamespace<'_>;
}

#[cfg(test)]
mod tests {}
