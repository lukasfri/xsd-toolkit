use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use url::Url;
use xmlity::{
    types::utils::{XmlRoot, XmlRootTop},
    XmlNamespace,
};
use xsd::xs;

#[derive(Debug, thiserror::Error)]
pub enum Error {}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SchemaHash(pub [u8; 32]);

#[allow(async_fn_in_trait)]
pub trait XmlSchemaResolver {
    type Error: std::error::Error + Send + Sync;

    async fn resolve_schema(&self, location: &Url) -> Result<xsd::XmlSchema, Self::Error>;
}

pub struct ReqwestXmlSchemaResolver {
    client: reqwest::Client,
}

impl ReqwestXmlSchemaResolver {
    pub fn new(client: reqwest::Client) -> Self {
        Self { client }
    }
}

impl XmlSchemaResolver for ReqwestXmlSchemaResolver {
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

#[derive(Debug, Clone, PartialEq)]
pub struct SchemaLocation {
    pub schema: xsd::XmlSchema,
    pub imports: HashMap<XmlNamespace<'static>, Option<Url>>,
    pub includes: HashSet<Url>,
}

pub struct XmlNamespaceMap {
    pub locations: HashMap<Url, Option<SchemaLocation>>,
}

impl XmlNamespaceMap {
    pub fn new() -> Self {
        Self {
            locations: HashMap::new(),
        }
    }

    pub fn inform_location(&mut self, location: &Url) {
        if !self.locations.contains_key(location) {
            // If the location is not already present, insert it with None
            self.locations.insert(location.clone(), None);
        }
    }

    pub fn inform_locations<T: IntoIterator<Item = Url>>(&mut self, locations: T) {
        locations.into_iter().for_each(|location| {
            self.inform_location(&location);
        });
    }

    pub async fn load_location<T: XmlSchemaResolver>(&mut self, resolver: &T, url: &Url) {
        if self.locations.get(url).is_some_and(|loc| loc.is_some()) {
            // Already loaded, no need to load again
            return;
        }

        let schema = resolver.resolve_schema(url).await.unwrap();

        let imports = schema
            .imports()
            .map(|a| {
                let namespace = a
                    .namespace
                    .as_ref()
                    .map(|ns| ns.0.clone())
                    .unwrap_or_else(|| XmlNamespace::new_dangerous(""));

                let location = a.schema_location.as_ref().map(|sl| {
                    let sl = sl.0.as_str();

                    // If sub_location is relative, resolve it against the base URL
                    let url = if sl.starts_with("http://") || sl.starts_with("https://") {
                        Url::from_str(sl)
                    } else {
                        url.join(sl)
                    };

                    url.unwrap_or_else(|_| {
                        panic!("Failed to parse schema location: {}", sl);
                    })
                });

                (namespace, location)
            })
            .collect::<HashMap<_, _>>();

        imports
            .iter()
            .filter_map(|(_, location)| location.as_ref())
            .for_each(|location| {
                self.inform_location(location);
            });

        let includes = schema
            .includes()
            .map(|a| Url::from_str(a.schema_location.0.as_str()).unwrap())
            .collect::<HashSet<_>>();

        let location = SchemaLocation {
            schema,
            imports,
            includes,
        };

        self.locations.insert(url.clone(), Some(location));
    }

    pub async fn explore_locations<T: XmlSchemaResolver>(&mut self, resolver: T) {
        while let Some(url) = self
            .locations
            .iter()
            .find_map(|(url, location)| location.is_none().then(|| url.clone()))
        {
            self.load_location(&resolver, &url).await;
        }
    }
}

pub fn get_all_urls_in_glob(
    glob_pattern: &str,
) -> Result<HashSet<Url>, Box<dyn std::error::Error>> {
    glob::glob(glob_pattern)?
        .into_iter()
        .map(|path| {
            path.map(|p| Url::from_file_path(p).unwrap())
                .map_err(From::from)
        })
        .collect::<Result<HashSet<_>, _>>()
}
