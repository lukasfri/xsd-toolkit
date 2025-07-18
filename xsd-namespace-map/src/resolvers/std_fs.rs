use crate::resolvers::XmlResolver;
use url::Url;

pub struct StdFsSchemaResolver {}

impl StdFsSchemaResolver {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for StdFsSchemaResolver {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: xmlity::DeserializeOwned> XmlResolver<T> for StdFsSchemaResolver {
    type Error = std::io::Error;

    fn resolve_document(&self, location: &Url) -> Result<T, Self::Error> {
        if !location.scheme().eq_ignore_ascii_case("file") {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Only file URLs are supported",
            ));
        }

        let schema_text = std::fs::read_to_string(location.path())?;

        let document: T = xmlity_quick_xml::from_str(schema_text.as_str()).unwrap();

        Ok(document)
    }
}
