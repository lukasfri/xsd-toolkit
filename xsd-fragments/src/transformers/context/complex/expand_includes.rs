use xsd::UrlExt;

use crate::{
    fragments::{complex::IncludeFragment, FragmentIdx, FragmentedXsdDocumentIdx},
    transformers::{TransformChange, XmlnsContextTransformer, XmlnsContextTransformerContext},
    FragmentedXsdDocumentKey,
};

#[non_exhaustive]
pub struct ExpandIncludeFragments {}

#[derive(Debug, thiserror::Error)]
/// Error type for the [`ExpandIncludeFragments`] transformer.
pub enum Error {
    #[error("Schema not found: {0}")]
    SchemaNotFound(FragmentedXsdDocumentIdx),
}

impl ExpandIncludeFragments {
    /// Creates a new instance of the [`ExpandIncludeFragments`] transformer.
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {}
    }

    fn expand_schema_include(
        ctx: &mut XmlnsContextTransformerContext<'_>,
        target_idx: &FragmentedXsdDocumentIdx,
        include_fragment: &FragmentIdx<IncludeFragment>,
    ) -> Result<TransformChange, Error> {
        let fragment = ctx
            .get_complex_fragment(&include_fragment)
            .expect("Expected include to be found");

        let current_fragment_location = ctx
            .xmlns_context
            .namespace_idxs
            .iter()
            .find(|(_, idx)| *idx == target_idx)
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
            .ok_or_else(|| Error::SchemaNotFound(target_idx.clone()))?;

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

        target_document.merge_with(included_document).unwrap();

        Ok(TransformChange::Changed)
    }

    fn expand_schema_includes(
        ctx: &mut XmlnsContextTransformerContext<'_>,
        document_idx: &FragmentedXsdDocumentIdx,
    ) -> Result<TransformChange, Error> {
        let namespace = ctx
            .xmlns_context
            .namespaces
            .get_mut(document_idx)
            .ok_or_else(|| Error::SchemaNotFound(document_idx.clone()))?;

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
            .map(|document_idx| Self::expand_schema_includes(&mut context, &document_idx))
            .collect::<Result<TransformChange, Self::Error>>()
    }
}
