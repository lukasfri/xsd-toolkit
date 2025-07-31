//! This module contains the code for loading and managing XML namespaces.
use std::{collections::HashMap, future::Future, ops::Deref, path::PathBuf};

use crate::{link::UrlExt, xs, XmlSchema};
use url::Url;
use xmlity::{ExpandedName, XmlNamespace};

#[derive(Debug, Clone, PartialEq)]
pub struct SchemaLocation {
    pub schema: XmlSchema,
}

pub struct XmlSchemaSet {
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

pub enum TopLevelType<'a> {
    SimpleType(&'a xs::types::TopLevelSimpleType),
    ComplexType(&'a xs::types::TopLevelComplexType),
}

impl XmlSchemaSet {
    pub fn new() -> Self {
        Self {
            locations: HashMap::new(),
        }
    }
}

impl Default for XmlSchemaSet {
    fn default() -> Self {
        Self::new()
    }
}

impl XmlSchemaSet {
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
            .try_for_each(|url| url.map(|url| self.inform_location(&url)))
    }

    fn load_location_internal(&mut self, url: &Url, schema: xs::Schema) {
        let schema = XmlSchema::new(schema);

        let imports = schema
            .imports()
            .map(|a| {
                let a = match a {
                    xs::Import::Import(a) => a,
                    _ => panic!("Expected an import, but found: {:?}", a),
                };

                let namespace = a
                    .namespace
                    .as_ref()
                    .map(|ns| XmlNamespace::new(ns.to_owned()))
                    .transpose()
                    .expect("Failed to parse namespace")
                    .unwrap_or_else(|| XmlNamespace::new_dangerous(""));

                let location = a
                    .schema_location
                    .as_ref()
                    .map(|sl| url.resolve_xml_url(sl).unwrap());

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
            .map(|a| match a {
                xs::Include::Include(a) => a,
                _ => panic!("Expected an include, but found: {:?}", a),
            })
            .map(|a| url.resolve_xml_url(&a.schema_location).unwrap())
            .collect::<Vec<_>>();

        includes.iter().for_each(|location| {
            self.inform_location(location);
        });

        let location = SchemaLocation { schema };

        self.locations.insert(url.clone(), Some(location));
    }

    pub fn load_location<R: Fn(&Url) -> Result<xs::Schema, E>, E>(
        &mut self,
        resolver: &R,
        url: &Url,
    ) -> Result<bool, E> {
        if self.locations.get(url).is_some_and(|loc| loc.is_some()) {
            // Already loaded, no need to load again
            return Ok(false);
        }

        let schema = (resolver)(url)?;

        self.load_location_internal(url, schema);

        Ok(true)
    }

    pub async fn load_location_async<
        F: Future<Output = Result<xs::Schema, E>>,
        E,
        R: Fn(&Url) -> F,
    >(
        &mut self,
        resolver: &R,
        url: &Url,
    ) -> Result<bool, E> {
        if self.locations.get(url).is_some_and(|loc| loc.is_some()) {
            // Already loaded, no need to load again
            return Ok(false);
        }

        let schema = (resolver)(url).await?;

        self.load_location_internal(url, schema);

        Ok(true)
    }

    pub fn explore_locations<'a, E, R: Fn(&Url) -> Result<xs::Schema, E>>(
        &'a mut self,
        resolver: &'a R,
    ) -> impl Iterator<Item = Result<Url, E>> + 'a {
        std::iter::from_fn(|| {
            let url = self
                .locations
                .iter()
                .find_map(|(url, location)| location.is_none().then(|| url.clone()))?;

            match self.load_location(resolver, &url) {
                Ok(loaded) => {
                    debug_assert!(loaded, "Location should be newly loaded since it was None");
                    Some(Ok(url))
                }
                Err(e) => Some(Err(e)),
            }
        })
    }

    // fn resolve_document(&self, location: &Url) -> Result<T, Self::Error>;
    pub fn explore_locations_async<
        'a,
        F: Future<Output = Result<xs::Schema, E>>,
        E,
        R: Fn(&Url) -> F,
    >(
        &'a mut self,
        resolver: &'a R,
    ) -> impl futures::Stream<Item = Result<Url, E>> + 'a {
        futures::stream::unfold(self, |this| async {
            let url = this
                .locations
                .iter()
                .find_map(|(url, location)| location.is_none().then(|| url.clone()))?;

            match this.load_location_async(resolver, &url).await {
                Ok(loaded) => {
                    debug_assert!(loaded, "Location should be newly loaded since it was None");
                    Some((Ok(url), this))
                }
                Err(e) => Some((Err(e), this)),
            }
        })
    }

    pub fn resolve_type(&self, name: &ExpandedName<'_>) -> Option<TopLevelType<'_>> {
        self.locations
            .iter()
            .filter_map(|(_, location)| location.as_ref().map(|loc| &loc.schema))
            .filter(|schema| schema.namespace().as_ref() == name.namespace())
            .flat_map(|schema| schema.redefinable())
            .find_map(|a| match a {
                xmlity_ns_xs::groups::Redefinable::SimpleType(simple_type) => {
                    match simple_type.deref() {
                        xmlity_ns_xs::SimpleType::SimpleType(simple_type) => {
                            if simple_type.name == *name.local_name() {
                                Some(TopLevelType::SimpleType(simple_type.deref()))
                            } else {
                                None
                            }
                        }
                        _ => None,
                    }
                }
                xmlity_ns_xs::groups::Redefinable::ComplexType(complex_type) => {
                    match complex_type.deref() {
                        xmlity_ns_xs::ComplexType::ComplexType(complex_type) => {
                            if complex_type.name == *name.local_name() {
                                Some(TopLevelType::ComplexType(complex_type.deref()))
                            } else {
                                None
                            }
                        }
                        _ => None,
                    }
                }
                _ => None,
            })
    }

    fn namespace_schemas<'a>(
        &'a self,
        namespace: Option<XmlNamespace<'static>>,
    ) -> impl Iterator<Item = &'a XmlSchema> + 'a {
        self.locations
            .values()
            .filter_map(|location| location.as_ref().map(|loc| &loc.schema))
            .filter(move |schema| schema.namespace().as_ref() == namespace.as_ref())
    }

    pub fn resolve_type_inheritance<'a>(
        &'a self,
        name: &'a ExpandedName<'a>,
    ) -> impl Iterator<Item = (&'a ExpandedName<'a>, TopLevelType<'a>)> + 'a {
        struct ResolveRecursiveBase<'a> {
            name: Option<&'a ExpandedName<'a>>,
            xsd: &'a XmlSchemaSet,
        }

        impl<'a> Iterator for ResolveRecursiveBase<'a> {
            type Item = (&'a ExpandedName<'a>, TopLevelType<'a>);

            fn next(&mut self) -> Option<Self::Item> {
                let current_name = self.name.take()?;
                let (local_name, namespace) = current_name.clone().into_parts();

                let (type_, base) = self
                    .xsd
                    .namespace_schemas(namespace.map(|a| a.into_owned()))
                    .flat_map(|a| a.redefinable())
                    .find_map(move |redefinable| {
                        use xmlity_ns_xs::complex_content_items::Child1 as CCC1;
                        use xmlity_ns_xs::groups::{ComplexTypeModel, SimpleDerivation};
                        use xmlity_ns_xs::simple_content_items::Child1 as SCC1;
                        use xmlity_ns_xs::{ComplexContent as CC, SimpleContent as SC};

                        match redefinable {
                            xs::groups::Redefinable::SimpleType(simple_type) => {
                                match simple_type.deref() {
                                    xs::SimpleType::SimpleType(simple_type) => {
                                        if simple_type.name == local_name {
                                            let type_ =
                                                TopLevelType::SimpleType(simple_type.deref());

                                            let base = match simple_type.simple_derivation.deref() {
                                                SimpleDerivation::Restriction(restriction) => {
                                                    match restriction.deref() {
                                                        xmlity_ns_xs::Restriction::Restriction(
                                                            restriction,
                                                        ) => {
                                                            restriction.base.as_ref().map(|a| &a.0)
                                                        }
                                                        xmlity_ns_xs::Restriction::Dynamic(_) => {
                                                            None
                                                        }
                                                    }
                                                }
                                                SimpleDerivation::List(_) => None,
                                                SimpleDerivation::Union(_) => None,
                                            };

                                            Some((type_, base))
                                        } else {
                                            None
                                        }
                                    }
                                    _ => None,
                                }
                            }
                            xs::groups::Redefinable::ComplexType(complex_type) => {
                                match complex_type.deref() {
                                    xs::ComplexType::ComplexType(complex_type) => {
                                        if complex_type.name == local_name {
                                            let type_ =
                                                TopLevelType::ComplexType(complex_type.deref());

                                            let base = match complex_type.complex_type_model.deref()
                                            {
                                                ComplexTypeModel::SimpleContent(simple_content) => {
                                                    match simple_content.deref() {
                                                        SC::SimpleContent(simple_content) => {
                                                            match &simple_content.child_1 {
                                                                SCC1::Restriction(restriction) => {
                                                                    Some(&restriction.base.0)
                                                                }
                                                                SCC1::Extension(extension) => {
                                                                    Some(&extension.base.0)
                                                                }
                                                            }
                                                        }
                                                        SC::Dynamic(_) => None,
                                                    }
                                                }
                                                ComplexTypeModel::ComplexContent(
                                                    complex_content,
                                                ) => match complex_content.deref() {
                                                    CC::ComplexContent(complex_content) => {
                                                        match &complex_content.child_1 {
                                                            CCC1::Restriction(restriction) => {
                                                                Some(&restriction.base.0)
                                                            }
                                                            CCC1::Extension(extension) => {
                                                                Some(&extension.base.0)
                                                            }
                                                        }
                                                    }
                                                    CC::Dynamic(_) => None,
                                                },
                                                ComplexTypeModel::Variant2(_) => None,
                                            };

                                            Some((type_, base))
                                        } else {
                                            None
                                        }
                                    }
                                    _ => None,
                                }
                            }
                            _ => None,
                        }
                    })?;

                self.name = base;

                Some((current_name, type_))
            }
        }

        ResolveRecursiveBase {
            name: Some(name),
            xsd: self,
        }
    }

    pub fn resolve_element(&self, name: &ExpandedName<'_>) -> Option<&xs::types::TopLevelElement> {
        self.locations
            .iter()
            .filter_map(|(_, location)| location.as_ref().map(|loc| &loc.schema))
            .filter(|schema| schema.namespace().as_ref() == name.namespace())
            .flat_map(|schema| schema.top_level_elements())
            .find_map(|element| match element {
                xs::Element::Element(el) if el.name == *name.local_name() => Some(el.deref()),
                _ => None,
            })
    }

    pub fn resolve_attribute(
        &self,
        name: &ExpandedName<'_>,
    ) -> Option<&xs::types::TopLevelAttribute> {
        self.namespace_schemas(name.namespace().map(|a| a.clone().into_owned()))
            .flat_map(|schema| schema.top_level_attributes())
            .find_map(|attribute| match attribute {
                xs::Attribute::Attribute(attr) if attr.name == *name.local_name() => {
                    Some(attr.deref())
                }
                _ => None,
            })
    }
}
