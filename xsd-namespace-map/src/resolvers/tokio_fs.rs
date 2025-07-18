use url::Url;

use crate::resolvers::AsyncXmlResolver;

pub struct TokioFsSchemaResolver {}

impl TokioFsSchemaResolver {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for TokioFsSchemaResolver {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: xmlity::DeserializeOwned> AsyncXmlResolver<T> for TokioFsSchemaResolver {
    type Error = std::io::Error;

    async fn resolve_document(&self, location: &Url) -> Result<T, Self::Error> {
        if !location.scheme().eq_ignore_ascii_case("file") {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Only file URLs are supported",
            ));
        }

        let schema_text = tokio::fs::read_to_string(location.path()).await?;

        let document: T = xmlity_quick_xml::from_str(schema_text.as_str()).unwrap();

        Ok(document)
    }
}
