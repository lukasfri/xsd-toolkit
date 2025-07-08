use url::Url;

pub mod reqwest;
pub mod std_fs;
pub mod tokio_fs;

pub trait XmlSchemaResolver {
    type Error: std::error::Error + Send + Sync;

    fn resolve_schema(&self, location: &Url) -> Result<xsd::XmlSchema, Self::Error>;
}

#[allow(async_fn_in_trait)]
pub trait AsyncXmlSchemaResolver {
    type Error: std::error::Error + Send + Sync;

    async fn resolve_schema(&self, location: &Url) -> Result<xsd::XmlSchema, Self::Error>;
}
