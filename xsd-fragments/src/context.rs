use std::collections::BTreeMap;

use url::Url;
use xmlity::XmlNamespace;
use xsd::{xs, UrlExt};

use crate::{fragments::NamespaceIdx, CompiledNamespace, Error};

#[derive(Debug)]
/// Context for managing XML Schema namespaces and their compiled representations.
pub struct XmlnsContext {
    /// Map of namespace indices to their compiled representations.
    pub namespaces: BTreeMap<NamespaceIdx, CompiledNamespace>,
    /// Map of namespace URIs to their indices.
    pub namespace_idxs: BTreeMap<XmlNamespace<'static>, NamespaceIdx>,
    namespace_id_count: usize,
}

impl XmlnsContext {
    /// Creates a new empty XML namespace context.
    pub fn new() -> Self {
        Self {
            namespaces: BTreeMap::new(),
            namespace_idxs: BTreeMap::new(),
            namespace_id_count: 0,
        }
    }

    fn generate_fragment_id(&mut self) -> NamespaceIdx {
        let fragment_id = NamespaceIdx::new(self.namespace_id_count);
        self.namespace_id_count += 1;
        fragment_id
    }

    /// Initializes a new namespace in the context and returns a mutable reference to it.
    pub fn init_namespace(&mut self, namespace: XmlNamespace<'static>) -> &mut CompiledNamespace {
        let namespace_idx = self.generate_fragment_id();
        self.namespace_idxs.insert(namespace.clone(), namespace_idx);

        let namespace = CompiledNamespace::new(namespace, namespace_idx);

        self.namespaces.insert(namespace_idx, namespace);

        self.namespaces
            .get_mut(&namespace_idx)
            .expect("Just inserted namespace")
    }

    /// Gets a reference to a compiled namespace by its URI.
    pub fn get_namespace(&self, namespace: &XmlNamespace<'_>) -> Option<&CompiledNamespace> {
        let namespace_idx = self.namespace_idxs.get(namespace)?;

        self.namespaces.get(namespace_idx)
    }

    /// Gets a mutable reference to a compiled namespace by its URI.
    pub fn get_namespace_mut(
        &mut self,
        namespace: &XmlNamespace<'_>,
    ) -> Option<&mut CompiledNamespace> {
        let namespace_idx = self.namespace_idxs.get(namespace)?;

        self.namespaces.get_mut(namespace_idx)
    }

    /// Imports a redefine element into the context.
    pub fn import_redefine(&mut self, _redefine: &xs::Redefine) -> Result<(), Error> {
        // use xs::redefine_items::RedefineContent;
        // let redefine = match redefine {
        //     xs::Redefine::Redefine(redefine) => redefine,
        //     _ => panic!("Expected a redefine, but found: {:?}", redefine),
        // };

        // let namespace = &redefine.schema_location;

        // let compiled_namespace =
        //     self.get_namespace_mut(namespace)
        //         .ok_or(Error::NonExistentXmlNamespace {
        //             namespace: namespace.clone(),
        //         })?;

        // let redefineable_ = redefine.redefine_content.iter().filter_map(|r| match r {
        //     RedefineContent::Annotation(_) => None,
        //     RedefineContent::Redefinable(redefinable) => Some(&**redefinable),
        // });

        todo!("Implement import_redefine for XmlnsContext")
    }

    /// Imports a namespace map with all its dependencies from the schema set.
    pub fn import_namespace_map(
        &mut self,
        map: &xsd::set::XmlSchemaSet,
        location_url: &url::Url,
        known_namespace: Option<XmlNamespace<'_>>,
        imported_urls: &mut Vec<Url>,
    ) -> Result<(), Error> {
        if imported_urls.contains(location_url) {
            return Ok(());
        }
        imported_urls.push(location_url.clone());

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
            .or_else(|| known_namespace.as_ref().map(|a| a.as_ref()));

        self.import_schema(
            current_namespace.as_ref().map(|a| a.as_ref()),
            &location.schema,
        )
        .inspect_err(|e| println!("Failed to import schema: {} ({})", e, location_url))?;

        location
            .schema
            .imports()
            .filter_map(|a| match a {
                xs::Import::Import(include) => Some(include),
                _ => None,
            })
            .try_for_each(|a| {
                let Some(schema_location) = a.schema_location.as_ref() else {
                    return Ok(());
                };

                let location_url = location_url
                    .resolve_xml_url(schema_location)
                    .expect("Expected a valid URL");

                let namespace = a
                    .namespace
                    .as_ref()
                    .map(XmlNamespace::new)
                    .transpose()
                    .expect("Expected a valid namespace");

                self.import_namespace_map(map, &location_url, namespace, imported_urls)
            })?;

        location
            .schema
            .includes()
            .filter_map(|a| match a {
                xs::Include::Include(include) => Some(include),
                _ => None,
            })
            .try_for_each(|a| {
                let location_url = location_url
                    .resolve_xml_url(&a.schema_location)
                    .expect("Expected a valid URL");

                self.import_namespace_map(
                    map,
                    &location_url,
                    current_namespace.as_ref().map(|a| a.as_ref()),
                    imported_urls,
                )
            })?;

        //TODO: Import redefines and overrides

        Ok(())
    }

    /// Imports an XML Schema into the context, creating or updating the relevant namespace.
    pub fn import_schema(
        &mut self,
        known_namespace: Option<XmlNamespace<'_>>,
        schema: &xsd::XmlSchema,
    ) -> Result<(), Error> {
        let namespace = schema
            .namespace()
            .or(known_namespace)
            .ok_or(Error::UndefinedNamespace)?;

        let compiled_namespace =
            if let Some(compiled_namespace) = self.get_namespace_mut(&namespace) {
                compiled_namespace
            } else {
                self.init_namespace(namespace.into_owned())
            };

        compiled_namespace.import_schema(schema)
    }
}

impl Default for XmlnsContext {
    fn default() -> Self {
        Self::new()
    }
}
