use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
    str::FromStr,
};

use url::Url;
use xmlity::XmlNamespace;

use crate::resolvers::{AsyncXmlSchemaResolver, XmlSchemaResolver};

pub mod resolvers;

#[derive(Debug, thiserror::Error)]
pub enum Error {}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SchemaHash(pub [u8; 32]);

#[derive(Debug, Clone, PartialEq)]
pub struct SchemaLocation {
    pub schema: xsd::XmlSchema,
    pub imports: HashMap<XmlNamespace<'static>, Option<Url>>,
    pub includes: HashSet<Url>,
}

pub struct XmlNamespaceMap {
    pub locations: HashMap<Url, Option<SchemaLocation>>,
}

#[derive(Debug, thiserror::Error)]
pub enum GlobError {
    #[error("Pattern error: {0}")]
    Pattern(#[from] glob::PatternError),
    #[error("Glob error at index {index}: {error}")]
    Glob {
        index: usize,
        error: glob::GlobError,
    },
    #[error("Failed to parse URL")]
    UrlParse { path: PathBuf },
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

    pub fn inform_glob_pattern(&mut self, glob_pattern: &str) -> Result<(), GlobError> {
        glob::glob(glob_pattern)?
            .into_iter()
            .enumerate()
            .map(|(i, path)| {
                path.map_err(|e| GlobError::Glob { index: i, error: e })
                    .and_then(|path| {
                        let path = if path.is_absolute() {
                            path
                        } else {
                            std::env::current_dir()
                                .map_err(|_| GlobError::UrlParse { path: path.clone() })?
                                .join(path)
                        };
                        Url::from_file_path(&path).map_err(|()| GlobError::UrlParse { path })
                    })
            })
            .map(|url| url.map(|url| self.inform_location(&url)))
            .collect::<Result<(), _>>()
    }

    fn load_location_internal(&mut self, url: &Url, schema: xsd::XmlSchema) {
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

    pub fn load_location<T: XmlSchemaResolver>(&mut self, resolver: &T, url: &Url) {
        if self.locations.get(url).is_some_and(|loc| loc.is_some()) {
            // Already loaded, no need to load again
            return;
        }

        let schema = resolver.resolve_schema(url).unwrap();

        self.load_location_internal(url, schema);
    }

    pub async fn load_location_async<T: AsyncXmlSchemaResolver>(
        &mut self,
        resolver: &T,
        url: &Url,
    ) {
        if self.locations.get(url).is_some_and(|loc| loc.is_some()) {
            // Already loaded, no need to load again
            return;
        }

        let schema = resolver.resolve_schema(url).await.unwrap();

        self.load_location_internal(url, schema);
    }

    pub fn explore_locations<T: XmlSchemaResolver>(&mut self, resolver: T) {
        while let Some(url) = self
            .locations
            .iter()
            .find_map(|(url, location)| location.is_none().then(|| url.clone()))
        {
            self.load_location(&resolver, &url);
        }
    }

    pub async fn explore_locations_async<T: AsyncXmlSchemaResolver>(&mut self, resolver: T) {
        while let Some(url) = self
            .locations
            .iter()
            .find_map(|(url, location)| location.is_none().then(|| url.clone()))
        {
            self.load_location_async(&resolver, &url).await;
        }
    }
}
