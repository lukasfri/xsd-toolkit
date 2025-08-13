use std::{collections::BTreeMap, ops::Deref};

use url::Url;
use xmlity::XmlNamespace;
use xsd::{xs, UrlExt};

use crate::{
    fragments::{
        complex::{IncludeFragment, RedefineFragment, SchemaFragment},
        FragmentAccess, FragmentedXsdDocumentIdx,
    },
    Error,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct FragmentedXsdDocumentKey(pub Url);

impl std::fmt::Display for FragmentedXsdDocumentKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
/// Context for managing XML Schema namespaces and their compiled representations.
pub struct XmlnsContext {
    /// Map of namespace indices to their compiled representations.
    pub namespaces: BTreeMap<FragmentedXsdDocumentIdx, SchemaFragment>,
    /// Map of namespace URIs to their indices.
    pub namespace_idxs: BTreeMap<FragmentedXsdDocumentKey, FragmentedXsdDocumentIdx>,
    namespace_id_count: usize,

    pub global_namespaces: BTreeMap<XmlNamespace<'static>, FragmentedXsdDocumentIdx>,
}

impl XmlnsContext {
    /// Creates a new empty [`XmlnsContext`].
    pub fn new() -> Self {
        Self {
            namespaces: BTreeMap::new(),
            namespace_idxs: BTreeMap::new(),
            namespace_id_count: 0,
            global_namespaces: BTreeMap::new(),
        }
    }

    fn generate_fragment_id(&mut self) -> FragmentedXsdDocumentIdx {
        let fragment_id = FragmentedXsdDocumentIdx::new(self.namespace_id_count);
        self.namespace_id_count += 1;
        fragment_id
    }

    /// Initializes a new namespace in the context and returns a mutable reference to it.
    pub fn import_schema(
        &mut self,
        key: FragmentedXsdDocumentKey,
        xs: &xs::Schema,
    ) -> Result<(FragmentedXsdDocumentIdx, &mut SchemaFragment), Error> {
        let namespace_idx = self.generate_fragment_id();
        self.namespace_idxs.insert(key, namespace_idx);

        let namespace = SchemaFragment::from_schema(xs, namespace_idx).unwrap();

        self.namespaces.insert(namespace_idx, namespace);

        Ok((
            namespace_idx,
            self.namespaces
                .get_mut(&namespace_idx)
                .expect("Just inserted namespace"),
        ))
    }

    pub fn export_schema(&mut self, key: &FragmentedXsdDocumentIdx) -> Result<xs::Schema, Error> {
        let Some(schema) = self.namespaces.get(key) else {
            return Err(Error::UndefinedNamespace);
        };
        let schema = schema.to_schema()?;

        Ok(schema)
    }

    fn merge_with(
        &mut self,
        source: &FragmentedXsdDocumentIdx,
        target: &FragmentedXsdDocumentIdx,
    ) -> Result<(FragmentedXsdDocumentIdx, &mut SchemaFragment), Error> {
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

    pub fn resolve_schema(&self, namespace: &FragmentedXsdDocumentKey) -> Option<&SchemaFragment> {
        let namespace_idx = self.namespace_idxs.get(namespace)?;

        self.namespaces.get(namespace_idx)
    }

    pub fn resolve_schema_mut(
        &mut self,
        namespace: &FragmentedXsdDocumentKey,
    ) -> Option<&mut SchemaFragment> {
        let namespace_idx = self.namespace_idxs.get(namespace)?;

        self.namespaces.get_mut(namespace_idx)
    }

    /// Gets a reference to a compiled namespace by its URI.
    pub fn get_namespace_direct(&self, namespace: &Url) -> Option<&SchemaFragment> {
        self.resolve_schema(&FragmentedXsdDocumentKey(namespace.clone()))
    }

    pub fn resolve_ref_namespace<'a>(
        &'a self,
        resolve_from: &'a FragmentedXsdDocumentIdx,
        referenced_namespace: &Option<XmlNamespace<'a>>,
    ) -> Option<&'a FragmentedXsdDocumentIdx> {
        if let Some(global_namespace) = referenced_namespace
            .as_ref()
            .and_then(|ns| self.global_namespaces.get(ns))
        {
            return Some(global_namespace);
        }

        let location = self.namespace_idxs.iter().find_map(|(key, idx)| {
            if idx == resolve_from {
                Some(key)
            } else {
                None
            }
        })?;
        let compiled_namespace = self.namespaces.get(resolve_from)?;

        if compiled_namespace.target_namespace == *referenced_namespace {
            Some(resolve_from)
        } else {
            // let referenced_ns = compiled_namespace.imports.get(referenced_namespace?)?;
            let referenced_ns = compiled_namespace
                .compositions
                .iter()
                .filter_map(|comp| {
                    if let crate::fragments::complex::CompositionId::Import(import) = comp {
                        Some(import)
                    } else {
                        None
                    }
                })
                .find_map(|import| {
                    let import = compiled_namespace.compiler.get_fragment(import)?;

                    let Some(schema_location) = import.schema_location.as_ref() else {
                        return None;
                    };

                    let location = location.0.resolve_xml_url(schema_location).unwrap();

                    let referenced_ns = self
                        .namespace_idxs
                        .get(&FragmentedXsdDocumentKey(location))?;

                    if import.namespace == *referenced_namespace {
                        Some(referenced_ns)
                    } else {
                        let schema = self
                            .namespaces
                            .get(referenced_ns)
                            .expect("Expected referenced namespace to be found");

                        if schema.target_namespace == *referenced_namespace {
                            Some(referenced_ns)
                        } else {
                            None
                        }
                    }
                })?;

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

    /// Imports a namespace map into the compiled namespace, resolving the current fragment location.
    pub fn import_namespace_map(
        &mut self,
        map: &xsd::set::XmlSchemaSet,
        location_url: &url::Url,
    ) -> Result<(FragmentedXsdDocumentIdx, &mut SchemaFragment), Error> {
        if let Some(compiled_namespace) = self
            .namespace_idxs
            .get(&FragmentedXsdDocumentKey(location_url.clone()))
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

        let location_key = FragmentedXsdDocumentKey(location_url.clone());

        self.import_schema(location_key.clone(), &location.schema.underlying_schema)
    }
}

impl Default for XmlnsContext {
    fn default() -> Self {
        Self::new()
    }
}
