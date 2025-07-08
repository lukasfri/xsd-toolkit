use url::Url;
use xmlity::types::utils::{XmlRoot, XmlRootTop};
use xsd::xs;

use crate::resolvers::XmlSchemaResolver;

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

impl XmlSchemaResolver for StdFsSchemaResolver {
    type Error = std::io::Error;

    fn resolve_schema(&self, location: &Url) -> Result<xsd::XmlSchema, Self::Error> {
        if !location.scheme().eq_ignore_ascii_case("file") {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Only file URLs are supported",
            ));
        }

        println!("Resolving schema from file: {}", location.path());
        let schema_text = std::fs::read_to_string(location.path())?;

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
