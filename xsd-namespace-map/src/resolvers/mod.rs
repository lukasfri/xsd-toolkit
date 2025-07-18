use url::Url;

mod chain;
pub mod reqwest;
pub mod std_fs;
pub mod tokio_fs;
pub use chain::{PossibleResolver, PossibleResolverExt};
mod cache;
pub use cache::CacheSchemaResolver;
use xmlity::DeserializeOwned;

pub trait XmlResolver<T: DeserializeOwned> {
    type Error: std::error::Error + Send + Sync;

    fn resolve_document(&self, location: &Url) -> Result<T, Self::Error>;
}

#[allow(async_fn_in_trait)]
pub trait AsyncXmlResolver<T: DeserializeOwned> {
    type Error: std::error::Error + Send + Sync;

    async fn resolve_document(&self, location: &Url) -> Result<T, Self::Error>;
}
