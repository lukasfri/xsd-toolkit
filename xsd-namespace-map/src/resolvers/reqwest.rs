use url::Url;

use crate::resolvers::{AsyncXmlResolver, XmlResolver};

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

impl<T: xmlity::DeserializeOwned> XmlResolver<T> for BlockingReqwestXmlSchemaResolver {
    type Error = reqwest::Error;

    fn resolve_document(&self, location: &Url) -> Result<T, Self::Error> {
        let response = self.client.get(location.as_str()).send()?;
        let schema_text = response.text()?;

        let document: T = xmlity_quick_xml::from_str(schema_text.as_str()).unwrap();

        Ok(document)
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

impl<T: xmlity::DeserializeOwned> AsyncXmlResolver<T> for ReqwestXmlSchemaResolver {
    type Error = reqwest::Error;

    async fn resolve_document(&self, location: &Url) -> Result<T, Self::Error> {
        let response = self.client.get(location.as_str()).send().await?;
        let schema_text = response.text().await?;

        let document: T = xmlity_quick_xml::from_str(schema_text.as_str()).unwrap();

        Ok(document)
    }
}
