use std::fmt;

use quote::format_ident;
use syn::Ident;

pub struct WrappingNamingStrategy(Box<dyn Fn(&str) -> String + Send + Sync>);

impl fmt::Debug for WrappingNamingStrategy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("WrappingNamingStrategy").finish()
    }
}

impl WrappingNamingStrategy {
    pub fn new<F: Fn(&str) -> String + Send + Sync + 'static>(f: F) -> Self {
        Self(Box::new(f))
    }

    pub fn wrap(&self, name: &str) -> String {
        (self.0)(name)
    }

    pub fn wrap_ident(&self, ident: &Ident) -> Ident {
        let wrapped_name = self.wrap(&ident.to_string());
        format_ident!("{}", wrapped_name)
    }
}

pub struct IndexedNamingStrategy(Box<dyn Fn(usize) -> String + Send + Sync>);

impl fmt::Debug for IndexedNamingStrategy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("IndexedNamingStrategy").finish()
    }
}

impl IndexedNamingStrategy {
    pub fn new<F: Fn(usize) -> String + Send + Sync + 'static>(f: F) -> Self {
        Self(Box::new(f))
    }

    pub fn name_for_index(&self, index: usize) -> String {
        (self.0)(index)
    }

    pub fn ident_for_index(&self, index: usize) -> Ident {
        let name = self.name_for_index(index);
        format_ident!("{}", name)
    }
}
