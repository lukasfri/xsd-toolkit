use std::{collections::BTreeMap, ops::Deref};

use url::Url;
use xmlity::XmlNamespace;
use xsd::{xs, UrlExt};

use crate::{fragments::FragmentedXsdDocumentIdx, Error, FragmentedXsdDocument};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum FragmentedXsdDocumentKey {
    /// Represents a namespace that is defined by its original URL.
    /// This is used for namespaces that are not redefined.
    Original(Url),
    /// Represents a namespace that is redefined by another URL.
    /// This is used for namespaces that have been redefined in a different context.
    Redefined {
        /// The original URL of the namespace.
        original: Url,
        /// The URL of the redefining context.
        redefiner: Url,
    },
}

impl std::fmt::Display for FragmentedXsdDocumentKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FragmentedXsdDocumentKey::Original(url) => write!(f, "{}", url),
            FragmentedXsdDocumentKey::Redefined {
                original: redefined_from,
                redefiner: redefined_to,
            } => write!(f, "{} -> {}", redefined_from, redefined_to),
        }
    }
}

#[derive(Debug)]
/// Context for managing XML Schema namespaces and their compiled representations.
pub struct XmlnsContext {
    /// Map of namespace indices to their compiled representations.
    pub namespaces: BTreeMap<FragmentedXsdDocumentIdx, FragmentedXsdDocument>,
    /// Map of namespace URIs to their indices.
    pub namespace_idxs: BTreeMap<FragmentedXsdDocumentKey, FragmentedXsdDocumentIdx>,
    namespace_id_count: usize,
}

impl XmlnsContext {
    /// Creates a new empty [`XmlnsContext`].
    pub fn new() -> Self {
        Self {
            namespaces: BTreeMap::new(),
            namespace_idxs: BTreeMap::new(),
            namespace_id_count: 0,
        }
    }

    fn generate_fragment_id(&mut self) -> FragmentedXsdDocumentIdx {
        let fragment_id = FragmentedXsdDocumentIdx::new(self.namespace_id_count);
        self.namespace_id_count += 1;
        fragment_id
    }

    /// Initializes a new namespace in the context and returns a mutable reference to it.
    fn init_compiled_namespace(
        &mut self,
        key: FragmentedXsdDocumentKey,
        namespace: XmlNamespace<'static>,
    ) -> (FragmentedXsdDocumentIdx, &mut FragmentedXsdDocument) {
        let namespace_idx = self.generate_fragment_id();
        self.namespace_idxs.insert(key, namespace_idx);

        let namespace = FragmentedXsdDocument::new(namespace_idx, namespace);

        self.namespaces.insert(namespace_idx, namespace);

        (
            namespace_idx,
            self.namespaces
                .get_mut(&namespace_idx)
                .expect("Just inserted namespace"),
        )
    }

    fn init_copy_namespace(
        &mut self,
        key: FragmentedXsdDocumentKey,
        copy_key: &FragmentedXsdDocumentKey,
    ) -> (FragmentedXsdDocumentIdx, &mut FragmentedXsdDocument) {
        let namespace_idx = self.generate_fragment_id();
        self.namespace_idxs.insert(key, namespace_idx);

        let copy_namespace = self
            .get_namespace(copy_key)
            .expect("Expected a namespace to copy from");

        let namespace = FragmentedXsdDocument::clone_with_namespace(copy_namespace, namespace_idx);

        self.namespaces.insert(namespace_idx, namespace);

        (
            namespace_idx,
            self.namespaces
                .get_mut(&namespace_idx)
                .expect("Just inserted namespace"),
        )
    }

    pub fn init_namespace(
        &mut self,
        location: Url,
        namespace: XmlNamespace<'static>,
    ) -> (FragmentedXsdDocumentIdx, &mut FragmentedXsdDocument) {
        let key = FragmentedXsdDocumentKey::Original(location);
        self.init_compiled_namespace(key, namespace)
    }

    pub fn get_namespace(
        &self,
        namespace: &FragmentedXsdDocumentKey,
    ) -> Option<&FragmentedXsdDocument> {
        let namespace_idx = self.namespace_idxs.get(namespace)?;

        self.namespaces.get(namespace_idx)
    }

    pub fn get_namespace_mut(
        &mut self,
        namespace: &FragmentedXsdDocumentKey,
    ) -> Option<&mut FragmentedXsdDocument> {
        let namespace_idx = self.namespace_idxs.get(namespace)?;

        self.namespaces.get_mut(namespace_idx)
    }

    /// Gets a reference to a compiled namespace by its URI.
    pub fn get_namespace_direct(&self, namespace: &Url) -> Option<&FragmentedXsdDocument> {
        self.get_namespace(&FragmentedXsdDocumentKey::Original(namespace.clone()))
    }

    /// Gets a mutable reference to a compiled namespace by its URI.
    pub fn get_namespace_direct_mut(
        &mut self,
        namespace: &Url,
    ) -> Option<&mut FragmentedXsdDocument> {
        self.get_namespace_mut(&FragmentedXsdDocumentKey::Original(namespace.clone()))
    }

    pub fn import_to_compiled_namespace(
        &mut self,
        map: &xsd::set::XmlSchemaSet,
        root_key: &FragmentedXsdDocumentKey,
        current_fragment_location: &Url,
    ) -> Result<(), Error> {
        let location = map
            .locations
            .get(current_fragment_location)
            .expect("Expected a location for the current fragment URL")
            .as_ref()
            .unwrap();

        let namespace = self
            .get_namespace_mut(root_key)
            .expect("Expected a namespace for the root key");

        namespace.import_schema(&location.schema).unwrap();

        let namespace = namespace.namespace.clone();

        location
            .schema
            .compositions()
            .try_for_each::<_, Result<(), Error>>(|a| match a {
                xs::groups::Composition::Import(import) => {
                    let xs::Import::Import(import) = import.deref() else {
                        return Ok(());
                    };

                    let Some(schema_location) = import.schema_location.as_ref() else {
                        return Ok(());
                    };

                    let import_namespace = import
                        .namespace
                        .clone()
                        .map(XmlNamespace::new)
                        .transpose()
                        .expect("Expected a valid namespace")
                        .unwrap_or_else(|| namespace.clone());

                    let location_url = current_fragment_location
                        .resolve_xml_url(schema_location)
                        .expect("Expected a valid URL");

                    let (ns_id, _) =
                        self.import_namespace_map(map, &location_url, Some(&import_namespace))?;

                    let namespace = self
                        .get_namespace_mut(root_key)
                        .expect("Expected a namespace for the root key");

                    namespace
                        .namespace_references
                        .insert(import_namespace, ns_id);

                    Ok(())
                }
                xs::groups::Composition::Include(include) => {
                    let xs::Include::Include(include) = include.deref() else {
                        return Ok(());
                    };

                    let location_url = current_fragment_location
                        .resolve_xml_url(&include.schema_location)
                        .expect("Expected a valid URL");

                    self.import_to_compiled_namespace(map, root_key, &location_url)?;

                    Ok(())
                }
                xs::groups::Composition::Redefine(redefine) => {
                    let xs::Redefine::Redefine(redefine) = redefine.deref() else {
                        return Ok(());
                    };

                    let location_url = current_fragment_location
                        .resolve_xml_url(&redefine.schema_location)
                        .expect("Expected a valid URL");

                    self.import_namespace_map(map, &location_url, Some(&namespace))?;

                    let (ns_id, redefineable_namespace) = self.init_copy_namespace(
                        FragmentedXsdDocumentKey::Redefined {
                            original: location_url.clone(),
                            redefiner: current_fragment_location.clone(),
                        },
                        &FragmentedXsdDocumentKey::Original(location_url.clone()),
                    );

                    redefineable_namespace.import_redefine(redefine)?;

                    let redefineable_namespace = redefineable_namespace.namespace.clone();

                    let namespace = self
                        .get_namespace_mut(root_key)
                        .expect("Expected a namespace for the root key");

                    namespace
                        .namespace_references
                        .insert(redefineable_namespace, ns_id);

                    Ok(())
                }
                xs::groups::Composition::Override(override_) => {
                    let xs::Override::Override(override_) = override_.deref() else {
                        return Ok(());
                    };

                    todo!()
                }
                _ => Ok(()),
            })?;

        Ok(())
    }

    pub fn import_namespace_map(
        &mut self,
        map: &xsd::set::XmlSchemaSet,
        location_url: &url::Url,
        with_namespace: Option<&XmlNamespace<'static>>,
    ) -> Result<(FragmentedXsdDocumentIdx, &mut FragmentedXsdDocument), Error> {
        if let Some(compiled_namespace) = self
            .namespace_idxs
            .get(&FragmentedXsdDocumentKey::Original(location_url.clone()))
        {
            let compiled_namespace = *compiled_namespace;
            let namespace = self.namespaces.get_mut(&compiled_namespace).unwrap();
            return Ok((compiled_namespace, namespace));
        }

        let location = map
            .locations
            .get(location_url)
            .expect("Expected a location for the origin URL");

        let location = location
            .as_ref()
            .expect("Expected the origin location to be loaded");

        let current_namespace = location
            .schema
            .namespace()
            .map(|ns| ns.into_owned())
            .or_else(|| with_namespace.map(|ns| ns.clone()))
            .expect("Namespace must be defined for the location");

        let location_key = FragmentedXsdDocumentKey::Original(location_url.clone());

        let (ns_id, _) =
            self.init_compiled_namespace(location_key.clone(), current_namespace.clone());

        self.import_to_compiled_namespace(map, &location_key, location_url)?;

        let current_namespace = self.namespaces.get_mut(&ns_id).unwrap();

        Ok((ns_id, current_namespace))
    }

    // /// Imports a namespace map with all its dependencies from the schema set.
    // fn import_namespace_map_inner(
    //     &mut self,
    //     map: &xsd::set::XmlSchemaSet,
    //     location_url: &url::Url,
    //     known_namespace: Option<XmlNamespace<'_>>,
    //     imported_urls: &mut Vec<Url>,
    // ) -> Result<(), Error> {
    //     if imported_urls.contains(location_url) {
    //         return Ok(());
    //     }
    //     imported_urls.push(location_url.clone());

    //     let location = map
    //         .locations
    //         .get(location_url)
    //         .expect("Expected a location for the origin URL");

    //     let location = location
    //         .as_ref()
    //         .expect("Expected the origin location to be loaded");

    //     let current_namespace = location
    //         .schema
    //         .namespace()
    //         .or_else(|| known_namespace.as_ref().map(|a| a.as_ref()));

    //     self.import_schema(
    //         current_namespace.as_ref().map(|a| a.as_ref()),
    //         &location.schema,
    //     )
    //     .inspect_err(|e| println!("Failed to import schema: {} ({})", e, location_url))?;

    //     location
    //         .schema
    //         .imports()
    //         .filter_map(|a| match a {
    //             xs::Import::Import(include) => Some(include),
    //             _ => None,
    //         })
    //         .try_for_each(|a| {
    //             let Some(schema_location) = a.schema_location.as_ref() else {
    //                 return Ok(());
    //             };

    //             let location_url = location_url
    //                 .resolve_xml_url(schema_location)
    //                 .expect("Expected a valid URL");

    //             let namespace = a
    //                 .namespace
    //                 .as_ref()
    //                 .map(XmlNamespace::new)
    //                 .transpose()
    //                 .expect("Expected a valid namespace");

    //             self.import_namespace_map_inner(map, &location_url, namespace, imported_urls)
    //         })?;

    //     location
    //         .schema
    //         .includes()
    //         .filter_map(|a| match a {
    //             xs::Include::Include(include) => Some(include),
    //             _ => None,
    //         })
    //         .try_for_each(|a| {
    //             let location_url = location_url
    //                 .resolve_xml_url(&a.schema_location)
    //                 .expect("Expected a valid URL");

    //             self.import_namespace_map_inner(
    //                 map,
    //                 &location_url,
    //                 current_namespace.as_ref().map(|a| a.as_ref()),
    //                 imported_urls,
    //             )
    //         })?;

    //     //TODO: Import redefines and overrides

    //     Ok(())
    // }

    // /// Imports an XML Schema into the context, creating or updating the relevant namespace.
    // pub fn import_schema(
    //     &mut self,
    //     known_namespace: Option<XmlNamespace<'_>>,
    //     schema: &xsd::XmlSchema,
    // ) -> Result<(), Error> {
    //     let namespace = schema
    //         .namespace()
    //         .or(known_namespace)
    //         .ok_or(Error::UndefinedNamespace)?;

    //     let compiled_namespace =
    //         if let Some(compiled_namespace) = self.get_namespace_mut(&namespace) {
    //             compiled_namespace
    //         } else {
    //             self.init_namespace(namespace.into_owned())
    //         };

    //     compiled_namespace.import_schema(schema)
    // }
}

impl Default for XmlnsContext {
    fn default() -> Self {
        Self::new()
    }
}
