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
        namespace: Option<XmlNamespace<'static>>,
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
        new_key: FragmentedXsdDocumentKey,
        target: &FragmentedXsdDocumentIdx,
    ) -> (FragmentedXsdDocumentIdx, &mut FragmentedXsdDocument) {
        let namespace_idx = self.generate_fragment_id();
        self.namespace_idxs.insert(new_key, namespace_idx);

        let copy_namespace = self
            .namespaces
            .get(target)
            .expect("Expected a namespace to copy from");

        let namespace = FragmentedXsdDocument::clone_with_namespace(copy_namespace, namespace_idx);

        let document = self.namespaces.entry(namespace_idx).or_insert(namespace);

        (namespace_idx, document)
    }

    fn merge_with(
        &mut self,
        source: &FragmentedXsdDocumentIdx,
        target: &FragmentedXsdDocumentIdx,
    ) -> Result<(FragmentedXsdDocumentIdx, &mut FragmentedXsdDocument), Error> {
        let (source_namespace, target_namespace) = self.namespaces.iter_mut().fold(
            (None, None),
            |(source_namespace, target_namespace), (key, val)| {
                if key == source {
                    (Some(val), target_namespace)
                } else if key == target {
                    (source_namespace, Some(val))
                } else {
                    (source_namespace, target_namespace)
                }
            },
        );

        let source_namespace = source_namespace.ok_or_else(|| Error::UndefinedNamespace)?;
        let target_namespace = target_namespace.ok_or_else(|| Error::UndefinedNamespace)?;

        target_namespace.merge_with(&source_namespace)?;

        Ok((*target, target_namespace))
    }

    pub fn init_namespace(
        &mut self,
        location: Url,
        namespace: Option<XmlNamespace<'static>>,
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

    pub fn resolve_ref_namespace<'a>(
        &'a self,
        resolve_from: &'a FragmentedXsdDocumentIdx,
        referenced_namespace: Option<&XmlNamespace<'a>>,
    ) -> Option<&'a FragmentedXsdDocumentIdx> {
        let compiled_namespace = self.namespaces.get(resolve_from)?;

        if referenced_namespace == compiled_namespace.namespace.as_ref() {
            Some(resolve_from)
        } else {
            let referenced_ns = compiled_namespace
                .namespace_references
                .get(referenced_namespace?)?;

            Some(referenced_ns)
        }
    }

    pub fn get_name(
        &self,
        namespace_idx: &FragmentedXsdDocumentIdx,
    ) -> Option<&FragmentedXsdDocumentKey> {
        self.namespace_idxs.iter().find_map(|(key, idx)| {
            if idx == namespace_idx {
                Some(key)
            } else {
                None
            }
        })
    }

    /// Gets a mutable reference to a compiled namespace by its URI.
    pub fn get_namespace_direct_mut(
        &mut self,
        namespace: &Url,
    ) -> Option<&mut FragmentedXsdDocument> {
        self.get_namespace_mut(&FragmentedXsdDocumentKey::Original(namespace.clone()))
    }

    fn import_import_to_compiled_namespace(
        &mut self,
        map: &xsd::set::XmlSchemaSet,
        root_idx: &FragmentedXsdDocumentIdx,
        current_fragment_location: &Url,
        namespace: &Option<XmlNamespace<'static>>,
        import: &xs::import_items::Import,
    ) -> Result<(), Error> {
        let Some(schema_location) = import.schema_location.as_ref() else {
            return Ok(());
        };

        let import_namespace = import
            .namespace
            .clone()
            .map(XmlNamespace::new)
            .transpose()
            .expect("Expected a valid namespace")
            .or_else(|| namespace.clone());

        let location_url = current_fragment_location
            .resolve_xml_url(schema_location)
            .expect("Expected a valid URL");

        let (ns_id, _) =
            self.import_namespace_map(map, &location_url, import_namespace.as_ref())?;

        let namespace = self
            .namespaces
            .get_mut(root_idx)
            .expect("Expected a namespace for the root key");

        namespace
            .namespace_references
            .insert(import_namespace.expect("Expected a namespace"), ns_id);

        Ok(())
    }

    fn import_include_to_compiled_namespace(
        &mut self,
        map: &xsd::set::XmlSchemaSet,
        root_idx: &FragmentedXsdDocumentIdx,
        current_fragment_location: &Url,
        namespace: &Option<XmlNamespace<'static>>,
        include: &xs::include_items::Include,
    ) -> Result<(), Error> {
        let location_url = current_fragment_location
            .resolve_xml_url(&include.schema_location)
            .expect("Expected a valid URL");

        self.import_to_compiled_namespace(map, root_idx, &location_url)?;

        Ok(())
    }

    fn import_redefine_to_compiled_namespace(
        &mut self,
        map: &xsd::set::XmlSchemaSet,
        root_idx: &FragmentedXsdDocumentIdx,
        current_fragment_location: &Url,
        namespace: &Option<XmlNamespace<'static>>,
        redefine: &xs::redefine_items::Redefine,
    ) -> Result<(), Error> {
        let location_url = current_fragment_location
            .resolve_xml_url(&redefine.schema_location)
            .expect("Expected a valid URL");

        let redefine_namespace = map
            .locations
            .get(&location_url)
            .and_then(|a| {
                a.as_ref()
                    .and_then(|a| a.schema.namespace().map(|ns| ns.into_owned()))
            })
            .or_else(|| namespace.clone());

        if redefine_namespace != *namespace {
            // According to the specification, a redefine can only be applied to the same namespace.
            todo!("Handle error for redefine in different namespace");
        }

        let (redefine_key, _) =
            self.import_namespace_map(map, &location_url, namespace.as_ref())?;

        // If the redefine namespace is the same as the current namespace, merge into the current namespace.
        let (_ns_id, redefineable_namespace) = self.merge_with(root_idx, &redefine_key)?;

        redefineable_namespace.import_redefine(redefine)?;

        Ok(())
    }

    fn import_override_to_compiled_namespace(
        &mut self,
        map: &xsd::set::XmlSchemaSet,
        root_idx: &FragmentedXsdDocumentIdx,
        current_fragment_location: &Url,
        namespace: &Option<XmlNamespace<'static>>,
        override_: &xs::override_items::Override,
    ) -> Result<(), Error> {
        let location_url = current_fragment_location
            .resolve_xml_url(&override_.schema_location)
            .expect("Expected a valid URL");

        let redefine_namespace = map
            .locations
            .get(&location_url)
            .and_then(|a| {
                a.as_ref()
                    .and_then(|a| a.schema.namespace().map(|ns| ns.into_owned()))
            })
            .or_else(|| namespace.clone());

        if redefine_namespace != *namespace {
            // According to the specification, a redefine can only be applied to the same namespace.
            todo!("Handle error for redefine in different namespace");
        }

        let (redefine_key, _) =
            self.import_namespace_map(map, &location_url, namespace.as_ref())?;

        // If the redefine namespace is the same as the current namespace, merge into the current namespace.
        let (_ns_id, redefineable_namespace) = self.merge_with(root_idx, &redefine_key)?;

        redefineable_namespace.import_override(override_)?;

        Ok(())
    }

    fn import_composition_to_compiled_namespace(
        &mut self,
        map: &xsd::set::XmlSchemaSet,
        root_idx: &FragmentedXsdDocumentIdx,
        current_fragment_location: &Url,
        namespace: &Option<XmlNamespace<'static>>,
        a: &xs::groups::Composition,
    ) -> Result<(), Error> {
        match a {
            xs::groups::Composition::Import(import) => {
                let xs::Import::Import(import) = import.deref() else {
                    return Ok(());
                };

                self.import_import_to_compiled_namespace(
                    map,
                    root_idx,
                    current_fragment_location,
                    namespace,
                    import,
                )
            }
            xs::groups::Composition::Include(include) => {
                let xs::Include::Include(include) = include.deref() else {
                    return Ok(());
                };

                self.import_include_to_compiled_namespace(
                    map,
                    root_idx,
                    current_fragment_location,
                    namespace,
                    include,
                )
            }
            xs::groups::Composition::Redefine(redefine) => {
                let xs::Redefine::Redefine(redefine) = redefine.deref() else {
                    return Ok(());
                };

                self.import_redefine_to_compiled_namespace(
                    map,
                    root_idx,
                    current_fragment_location,
                    namespace,
                    redefine,
                )
            }
            xs::groups::Composition::Override(override_) => {
                let xs::Override::Override(override_) = override_.deref() else {
                    return Ok(());
                };

                self.import_override_to_compiled_namespace(
                    map,
                    root_idx,
                    current_fragment_location,
                    namespace,
                    override_,
                )
            }
            _ => Ok(()),
        }
    }

    fn import_schema_to_compiled_namespace(
        &mut self,
        map: &xsd::set::XmlSchemaSet,
        root_idx: &FragmentedXsdDocumentIdx,
        current_fragment_location: &Url,
        schema: &xsd::XmlSchema,
    ) -> Result<(), Error> {
        let namespace = self
            .namespaces
            .get_mut(root_idx)
            .expect("Expected a namespace for the root key");

        namespace.import_schema(&schema).unwrap();

        let namespace = namespace.namespace.clone();

        schema
            .compositions()
            .try_for_each::<_, Result<(), Error>>(|a| {
                self.import_composition_to_compiled_namespace(
                    map,
                    root_idx,
                    current_fragment_location,
                    &namespace,
                    a,
                )
            })?;

        Ok(())
    }

    /// Imports a schema into the compiled namespace, resolving the current fragment location.
    pub fn import_to_compiled_namespace(
        &mut self,
        map: &xsd::set::XmlSchemaSet,
        root_idx: &FragmentedXsdDocumentIdx,
        current_fragment_location: &Url,
    ) -> Result<(), Error> {
        let location = map
            .locations
            .get(current_fragment_location)
            .expect("Expected a location for the current fragment URL")
            .as_ref()
            .unwrap();

        self.import_schema_to_compiled_namespace(
            map,
            root_idx,
            current_fragment_location,
            &location.schema,
        )
    }

    /// Imports a namespace map into the compiled namespace, resolving the current fragment location.
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
            .or_else(|| with_namespace.map(|ns| ns.clone()));

        let location_key = FragmentedXsdDocumentKey::Original(location_url.clone());

        let (ns_id, _) =
            self.init_compiled_namespace(location_key.clone(), current_namespace.clone());

        let location_idx = self
            .namespace_idxs
            .get(&location_key)
            .expect("Expected a namespace index for the location key")
            .clone();

        self.import_to_compiled_namespace(map, &location_idx, location_url)?;

        let current_namespace = self.namespaces.get_mut(&ns_id).unwrap();

        Ok((ns_id, current_namespace))
    }
}

impl Default for XmlnsContext {
    fn default() -> Self {
        Self::new()
    }
}
