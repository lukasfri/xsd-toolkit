use xsd::UrlExt;

use crate::{
    fragments::{
        complex::{RedefineFragment, TopLevelTypeId},
        FragmentIdx, FragmentedXsdDocumentIdx,
    },
    transformers::{TransformChange, XmlnsContextTransformer, XmlnsContextTransformerContext},
    FragmentedXsdDocumentKey,
};

#[non_exhaustive]
pub struct ExpandRedefineFragments {}

#[derive(Debug, thiserror::Error)]
/// Error type for the [`ExpandRedefineFragments`] transformer.
pub enum Error {
    #[error("Schema not found: {0}")]
    SchemaNotFound(FragmentedXsdDocumentIdx),
}

impl ExpandRedefineFragments {
    /// Creates a new instance of the [`ExpandRedefineFragments`] transformer.
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {}
    }

    fn expand_schema_redefine(
        ctx: &mut XmlnsContextTransformerContext<'_>,
        target_idx: &FragmentedXsdDocumentIdx,
        redefine_fragment: &FragmentIdx<RedefineFragment>,
    ) -> Result<TransformChange, Error> {
        let fragment = ctx
            .get_complex_fragment(&redefine_fragment)
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

        let redefinable_items = fragment
            .redefineable
            .iter()
            .copied()
            .map(|redefinable| {
                let name = match redefinable {
                    crate::fragments::complex::RedefinableId::ComplexType(fragment_idx) => {
                        let fragment = ctx
                            .get_complex_fragment(&fragment_idx)
                            .expect("Expected fragment to be found");

                        fragment
                            .name
                            .clone()
                            .expect("Expected fragment to have a name")
                    }
                    crate::fragments::complex::RedefinableId::SimpleType(fragment_idx) => {
                        let fragment = ctx
                            .get_simple_fragment(&fragment_idx)
                            .expect("Expected fragment to be found");

                        fragment
                            .name
                            .clone()
                            .expect("Expected fragment to have a name")
                    }
                    crate::fragments::complex::RedefinableId::AttributeGroup(fragment_idx) => {
                        let fragment = ctx
                            .get_complex_fragment(&fragment_idx)
                            .expect("Expected fragment to be found");

                        fragment.name.clone()
                    }
                    crate::fragments::complex::RedefinableId::Group(fragment_idx) => {
                        let fragment = ctx
                            .get_complex_fragment(&fragment_idx)
                            .expect("Expected fragment to be found");

                        fragment.name.clone()
                    }
                };

                (name, redefinable)
            })
            .collect::<Vec<_>>();

        let (redefined_document, target_document) = ctx.xmlns_context.namespaces.iter_mut().fold(
            (None, None),
            |(redefined, target_document), (key, namespace)| {
                if key == &schema_location {
                    (Some(namespace), target_document)
                } else if key == target_idx {
                    (redefined, Some(namespace))
                } else {
                    (redefined, target_document)
                }
            },
        );

        let redefined_document =
            redefined_document.expect("Expected redefined document to be found");
        let target_document = target_document.expect("Expected target document to be found");

        target_document.merge_with(redefined_document).unwrap();

        redefinable_items
            .into_iter()
            .for_each(|(name, redefinable)| match redefinable {
                crate::fragments::complex::RedefinableId::ComplexType(root_fragment) => {
                    target_document
                        .top_level_types
                        .insert(name, TopLevelTypeId::ComplexType(root_fragment));
                }
                crate::fragments::complex::RedefinableId::SimpleType(root_fragment) => {
                    target_document
                        .top_level_types
                        .insert(name, TopLevelTypeId::SimpleType(root_fragment));
                }
                crate::fragments::complex::RedefinableId::AttributeGroup(root_fragment) => {
                    target_document
                        .top_level_attribute_groups
                        .insert(name, root_fragment);
                }
                crate::fragments::complex::RedefinableId::Group(root_fragment) => {
                    target_document.top_level_groups.insert(name, root_fragment);
                }
            });

        Ok(TransformChange::Changed)

        // let location_url = current_fragment_location
        //     .resolve_xml_url(&redefine.schema_location)
        //     .expect("Expected a valid URL");

        // let redefine_namespace = map
        //     .locations
        //     .get(&location_url)
        //     .and_then(|a| {
        //         a.as_ref()
        //             .and_then(|a| a.schema.namespace().map(|ns| ns.into_owned()))
        //     })
        //     .or_else(|| namespace.clone());

        // if redefine_namespace != *namespace {
        //     // According to the specification, a redefine can only be applied to the same namespace.
        //     todo!("Handle error for redefine in different namespace");
        // }

        // let (redefine_key, _) =
        //     self.import_namespace_map(map, &location_url, namespace.as_ref())?;

        // // If the redefine namespace is the same as the current namespace, merge into the current namespace.
        // let (_ns_id, redefineable_namespace) = self.merge_with(root_idx, &redefine_key)?;

        // redefineable_namespace.import_redefine(redefine)?;
    }

    fn expand_schema_redefines(
        ctx: &mut XmlnsContextTransformerContext<'_>,
        document_idx: &FragmentedXsdDocumentIdx,
    ) -> Result<TransformChange, <Self as XmlnsContextTransformer>::Error> {
        let namespace = ctx
            .xmlns_context
            .namespaces
            .get_mut(document_idx)
            .ok_or_else(|| Error::SchemaNotFound(document_idx.clone()))?;

        let (compositions, redefines) = namespace
            .compositions
            .drain(..)
            .map(|a| match a {
                crate::fragments::complex::CompositionId::Redefine(fragment_idx) => {
                    (None, Some(fragment_idx))
                }
                _ => (Some(a), None),
            })
            .unzip::<_, _, Vec<_>, Vec<_>>();

        namespace.compositions = compositions.into_iter().flatten().collect();

        redefines
            .into_iter()
            .flatten()
            .map(|r| Self::expand_schema_redefine(ctx, document_idx, &r))
            .collect()
    }
}

impl XmlnsContextTransformer for ExpandRedefineFragments {
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
            .map(|document_idx| Self::expand_schema_redefines(&mut context, &document_idx))
            .collect::<Result<TransformChange, Self::Error>>()
    }
}
