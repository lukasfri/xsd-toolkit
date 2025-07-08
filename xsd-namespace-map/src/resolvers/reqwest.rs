use url::Url;
use xmlity::types::utils::{XmlRoot, XmlRootTop};
use xsd::xs;

use crate::resolvers::{AsyncXmlSchemaResolver, XmlSchemaResolver};

pub struct BlockingReqwestXmlSchemaResolver {
    client: reqwest::blocking::Client,
}

impl BlockingReqwestXmlSchemaResolver {
    pub fn new(client: reqwest::blocking::Client) -> Self {
        Self { client }
    }
}

impl Default for BlockingReqwestXmlSchemaResolver {
    fn default() -> Self {
        Self::new(reqwest::blocking::Client::new())
    }
}

impl XmlSchemaResolver for BlockingReqwestXmlSchemaResolver {
    type Error = reqwest::Error;

    fn resolve_schema(&self, location: &Url) -> Result<xsd::XmlSchema, Self::Error> {
        let response = self.client.get(location.as_str()).send()?;
        let schema_text = response.text()?;

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

pub struct ReqwestXmlSchemaResolver {
    client: reqwest::Client,
}

impl ReqwestXmlSchemaResolver {
    pub fn new(client: reqwest::Client) -> Self {
        Self { client }
    }
}

impl Default for ReqwestXmlSchemaResolver {
    fn default() -> Self {
        Self::new(reqwest::Client::new())
    }
}

impl AsyncXmlSchemaResolver for ReqwestXmlSchemaResolver {
    type Error = reqwest::Error;

    async fn resolve_schema(&self, location: &Url) -> Result<xsd::XmlSchema, Self::Error> {
        let response = self.client.get(location.as_str()).send().await?;
        let schema_text = response.text().await?;

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
