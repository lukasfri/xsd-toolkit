use std::collections::HashMap;

use crate::resolvers::XmlSchemaResolver;
use derive_more::{Display, Error};
use url::Url;

#[derive(Debug, Display, Error)]
#[display("Not in cache.")]
pub struct CacheError;

pub struct CacheSchemaResolver {
    caches: HashMap<Url, xsd::XmlSchema>,
}

impl CacheSchemaResolver {
    pub fn new(caches: HashMap<Url, xsd::XmlSchema>) -> Self {
        Self { caches }
    }
}

impl XmlSchemaResolver for CacheSchemaResolver {
    type Error = CacheError;

    fn resolve_schema(&self, location: &Url) -> Result<xsd::XmlSchema, Self::Error> {
        self.caches.get(location).cloned().ok_or(CacheError)
    }
}
