use xsd::UrlExt;

use crate::{
    fragments::{complex::IncludeFragment, FragmentIdx, FragmentedXsdDocumentIdx},
    transformers::{TransformChange, XmlnsContextTransformer, XmlnsContextTransformerContext},
    FragmentedXsdDocumentKey,
};

/// This transformer expands the include fragments in the XML Schema context.
#[non_exhaustive]
pub struct ExpandIncludeFragments {
    bypass: Vec<url::Url>,
}

#[derive(Debug, thiserror::Error)]
/// Error type for the [`ExpandIncludeFragments`] transformer.
pub enum Error {
    /// Error indicating that a schema was not found in the context.
    #[error("Schema not found for document {origin_document}: \"{missing_location}\"")]
    SchemaNotFound {
        origin_document: FragmentedXsdDocumentIdx,
        missing_location: url::Url,
    },
}

impl ExpandIncludeFragments {
    /// Creates a new instance of the [`ExpandIncludeFragments`] transformer.
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self { bypass: Vec::new() }
    }

    pub fn with_bypass(mut self, urls: Vec<url::Url>) -> Self {
        self.bypass = urls;
        self
    }

    fn expand_schema_include(
        ctx: &mut XmlnsContextTransformerContext<'_>,
        target_idx: &FragmentedXsdDocumentIdx,
        include_fragment_id: &FragmentIdx<IncludeFragment>,
    ) -> Result<TransformChange, Error> {
        let fragment = ctx
            .get_complex_fragment(&include_fragment_id)
            .expect("Expected include to be found");

        let current_fragment_location = ctx
            .xmlns_context
            .namespace_idxs
            .iter()
            .find(|(_, idx)| **idx == include_fragment_id.namespace_idx())
            .map(|(location, _)| location)
            .expect("Expected current fragment location to be found");

        let location_url = current_fragment_location
            .0
            .resolve_xml_url(&fragment.schema_location)
            .expect("Expected a valid URL");

        let key = FragmentedXsdDocumentKey(location_url);

        let schema_location = ctx
            .xmlns_context
            .namespace_idxs
            .get(&key)
            .cloned()
            .ok_or_else(|| Error::SchemaNotFound {
                origin_document: target_idx.clone(),
                missing_location: key.0.clone(),
            })?;

        let (included_document, target_document) = ctx.xmlns_context.namespaces.iter_mut().fold(
            (None, None),
            |(included, target_document), (key, namespace)| {
                if key == &schema_location {
                    (Some(namespace), target_document)
                } else if key == target_idx {
                    (included, Some(namespace))
                } else {
                    (included, target_document)
                }
            },
        );

        let included_document = included_document.expect("Expected included document to be found");
        let target_document = target_document.expect("Expected target document to be found");
        let target_url = &current_fragment_location.0;
        let other_url = &key.0;

        target_document
            .merge_with(included_document, target_url, other_url)
            .unwrap();

        Ok(TransformChange::Changed)
    }

    fn expand_schema_includes(
        &self,
        ctx: &mut XmlnsContextTransformerContext<'_>,
        document_idx: &FragmentedXsdDocumentIdx,
    ) -> Result<TransformChange, Error> {
        let url = ctx
            .xmlns_context
            .namespace_idxs
            .iter()
            .find_map(
                |(key, idx)| {
                    if idx == document_idx {
                        Some(key)
                    } else {
                        None
                    }
                },
            )
            .expect("Expected URL to be found");

        if self.bypass.contains(&url.0) {
            return Ok(TransformChange::Unchanged);
        }

        let namespace = ctx.xmlns_context.namespaces.get_mut(document_idx).expect(
            "Expected namespace to be found since we are expanding includes of the document",
        );

        let (compositions, includes) = namespace
            .compositions
            .drain(..)
            .map(|a| match a {
                crate::fragments::complex::CompositionId::Include(fragment_idx) => {
                    (None, Some(fragment_idx))
                }
                _ => (Some(a), None),
            })
            .unzip::<_, _, Vec<_>, Vec<_>>();

        namespace.compositions = compositions.into_iter().flatten().collect();

        includes
            .into_iter()
            .flatten()
            .map(|r| Self::expand_schema_include(ctx, document_idx, &r))
            .collect()
    }
}

impl XmlnsContextTransformer for ExpandIncludeFragments {
    type Error = Error;

    fn transform(
        self,
        mut context: XmlnsContextTransformerContext<'_>,
    ) -> Result<TransformChange, Self::Error> {
        let keys = context
            .xmlns_context
            .namespaces
            .keys()
            .copied()
            .collect::<Vec<_>>();

        keys.into_iter()
            .map(|document_idx| self.expand_schema_includes(&mut context, &document_idx))
            .collect::<Result<TransformChange, Self::Error>>()
    }
}
