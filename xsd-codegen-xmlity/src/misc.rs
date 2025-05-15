use std::{collections::HashMap, fmt::Debug};

use dyn_clone::DynClone;
use syn::{Path, Type, TypePath};
use xmlity::{LocalName, XmlNamespace};

use crate::types::TypeRecord;

trait TypeReferenceTrait: FnOnce(Option<&Path>) -> Type + DynClone {}

dyn_clone::clone_trait_object!(TypeReferenceTrait);

impl<F> TypeReferenceTrait for F where F: FnOnce(Option<&Path>) -> Type + DynClone {}

#[derive(Clone)]
pub struct TypeReference<'a>(Box<dyn TypeReferenceTrait + 'a>);

impl Debug for TypeReference<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TypeReference").finish()
    }
}

impl<'a> TypeReference<'a> {
    pub fn new<F>(f: F) -> Self
    where
        F: TypeReferenceTrait + 'a,
    {
        Self(Box::new(f))
    }

    pub fn new_prefix(type_: TypePath) -> Self {
        Self::new(move |path| {
            if let Some(path) = path {
                syn::parse_quote! { #path::#type_ }
            } else {
                syn::parse_quote! { #type_ }
            }
        })
    }

    pub fn new_static(type_: Type) -> Self {
        Self::new(move |_| type_)
    }

    pub fn into_type(self, path: Option<&Path>) -> Type {
        (self.0)(path)
    }
}

#[derive(Debug)]
pub struct GeneratedNamespaceTypes {
    pub namespace: XmlNamespace<'static>,
    pub types: HashMap<LocalName<'static>, GeneratedFragment>,
}

impl GeneratedNamespaceTypes {
    pub fn new(namespace: XmlNamespace<'static>) -> Self {
        Self {
            namespace,
            types: HashMap::new(),
        }
    }
}

#[derive(Debug)]
pub struct GeneratedFragment {
    pub type_: TypeRecord,
}
