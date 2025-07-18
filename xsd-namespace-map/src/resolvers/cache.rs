use std::collections::HashMap;

use crate::resolvers::XmlResolver;
use derive_more::{Display, Error};
use url::Url;

#[derive(Debug, Display, Error)]
#[display("Not in cache.")]
pub struct CacheError;

pub struct CacheSchemaResolver<T> {
    caches: HashMap<Url, T>,
}

impl<T> CacheSchemaResolver<T> {
    pub fn new(caches: HashMap<Url, T>) -> Self {
        Self { caches }
    }
}

impl<T: xmlity::DeserializeOwned + Clone> XmlResolver<T> for CacheSchemaResolver<T> {
    type Error = CacheError;

    fn resolve_document(&self, location: &Url) -> Result<T, Self::Error> {
        self.caches.get(location).cloned().ok_or(CacheError)
    }
}
