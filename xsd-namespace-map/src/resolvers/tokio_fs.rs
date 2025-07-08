use url::Url;
use xmlity::types::utils::{XmlRoot, XmlRootTop};
use xsd::xs;

use crate::resolvers::AsyncXmlSchemaResolver;

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

impl AsyncXmlSchemaResolver for TokioFsSchemaResolver {
    type Error = std::io::Error;

    async fn resolve_schema(&self, location: &Url) -> Result<xsd::XmlSchema, Self::Error> {
        if !location.scheme().eq_ignore_ascii_case("file") {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Only file URLs are supported",
            ));
        }

        let schema_text = tokio::fs::read_to_string(location.path()).await?;

        let schema: XmlRoot<xs::Schema> = xmlity_quick_xml::from_str(schema_text.as_str()).unwrap();
        let schema = schema
            .elements
            .into_iter()
            .find_map(|a| match a {
                XmlRootTop::Value(v) => Some(v),
                _ => None,
            })
            .unwrap();
        let schema = xsd::XmlSchema::new(schema);

        Ok(schema)
    }
}
