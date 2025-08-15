use xsd::UrlExt;

use crate::{
    fragments::{
        complex::{
            ComplexOffsetableExt, OverrideFragment, RedefinableId, SchemaTopId, TopLevelTypeId,
        },
        FragmentAccess, FragmentIdx, FragmentedXsdDocumentIdx,
    },
    transformers::{TransformChange, XmlnsContextTransformer, XmlnsContextTransformerContext},
    FragmentedXsdDocumentKey,
};

/// This transformer expands the override fragments in the XML Schema context.
#[non_exhaustive]
pub struct ExpandOverrideFragments {}

#[derive(Debug, thiserror::Error)]
/// Error type for the [`ExpandOverrideFragments`] transformer.
pub enum Error {
    /// Error indicating that a schema was not found in the context.
    #[error("Schema not found: {0}")]
    SchemaNotFound(FragmentedXsdDocumentIdx),
    /// Error indicating a complex fragment error.
    #[error("Complex fragment error: {0}")]
    ComplexFragment(#[from] crate::fragments::complex::Error),
    /// Error indicating a simple fragment error.
    #[error("Simple fragment error: {0}")]
    SimpleFragment(#[from] crate::fragments::simple::Error),
}

impl ExpandOverrideFragments {
    /// Creates a new instance of the [`ExpandOverrideFragments`] transformer.
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {}
    }

    fn expand_schema_override(
        ctx: &mut XmlnsContextTransformerContext<'_>,
        target_idx: &FragmentedXsdDocumentIdx,
        override_fragment_id: &FragmentIdx<OverrideFragment>,
    ) -> Result<TransformChange, Error> {
        let override_fragment = ctx
            .get_complex_fragment(&override_fragment_id)
            .expect("Expected include to be found")
            .clone();

        let current_fragment_location = ctx
            .xmlns_context
            .namespace_idxs
            .iter()
            .find(|(_, idx)| *idx == target_idx)
            .map(|(location, _)| location)
            .expect("Expected current fragment location to be found");

        let location_url = current_fragment_location
            .0
            .resolve_xml_url(&override_fragment.schema_location)
            .expect("Expected a valid URL");

        let key = FragmentedXsdDocumentKey(location_url);

        let schema_location = ctx
            .xmlns_context
            .namespace_idxs
            .get(&key)
            .cloned()
            .ok_or_else(|| Error::SchemaNotFound(target_idx.clone()))?;

        let overriden_complex_types = override_fragment
            .schema_tops
            .iter()
            .filter_map(|redefinable| match redefinable {
                SchemaTopId::Redefinable(RedefinableId::ComplexType(fragment_idx)) => {
                    Some(fragment_idx)
                }
                _ => None,
            })
            .map(|fragment_idx| {
                ctx.get_complex_fragment(&fragment_idx)
                    .expect("Expected fragment to be found")
            })
            .map(|fragment| {
                fragment
                    .name
                    .clone()
                    .expect("Expected fragment to have a name")
            })
            .collect::<Vec<_>>();

        let overriden_simple_types = override_fragment
            .schema_tops
            .iter()
            .filter_map(|redefinable| match redefinable {
                SchemaTopId::Redefinable(RedefinableId::SimpleType(fragment_idx)) => {
                    Some(fragment_idx)
                }
                _ => None,
            })
            .map(|fragment_idx| {
                ctx.get_simple_fragment(&fragment_idx)
                    .expect("Expected fragment to be found")
            })
            .map(|fragment| {
                fragment
                    .name
                    .clone()
                    .expect("Expected fragment to have a name")
            })
            .collect::<Vec<_>>();

        let overriden_attribute_groups = override_fragment
            .schema_tops
            .iter()
            .filter_map(|redefinable| match redefinable {
                SchemaTopId::Redefinable(RedefinableId::AttributeGroup(fragment_idx)) => {
                    Some(fragment_idx)
                }
                _ => None,
            })
            .map(|fragment_idx| {
                ctx.get_complex_fragment(&fragment_idx)
                    .expect("Expected fragment to be found")
            })
            .map(|fragment| fragment.name.clone())
            .collect::<Vec<_>>();

        let overriden_groups = override_fragment
            .schema_tops
            .iter()
            .filter_map(|redefinable| match redefinable {
                SchemaTopId::Redefinable(RedefinableId::Group(fragment_idx)) => Some(fragment_idx),
                _ => None,
            })
            .map(|fragment_idx| {
                ctx.get_complex_fragment(&fragment_idx)
                    .expect("Expected fragment to be found")
            })
            .map(|fragment| fragment.name.clone())
            .collect::<Vec<_>>();

        let overriden_elements = override_fragment
            .schema_tops
            .iter()
            .filter_map(|schema_top| match schema_top {
                SchemaTopId::Element(fragment_idx) => Some(fragment_idx),
                _ => None,
            })
            .map(|fragment_idx| {
                ctx.get_complex_fragment(&fragment_idx)
                    .expect("Expected fragment to be found")
            })
            .map(|fragment| fragment.name.clone())
            .collect::<Vec<_>>();

        let overriden_attributes = override_fragment
            .schema_tops
            .iter()
            .filter_map(|schema_top| match schema_top {
                SchemaTopId::Attribute(fragment_idx) => Some(fragment_idx),
                _ => None,
            })
            .map(|fragment_idx| {
                ctx.get_complex_fragment(&fragment_idx)
                    .expect("Expected fragment to be found")
            })
            .map(|fragment| fragment.name.clone())
            .collect::<Vec<_>>();

        let (overriden_document, target_document) = ctx.xmlns_context.namespaces.iter_mut().fold(
            (None, None),
            |(overriden, target_document), (key, namespace)| {
                if key == &schema_location {
                    (Some(namespace), target_document)
                } else if key == target_idx {
                    (overriden, Some(namespace))
                } else {
                    (overriden, target_document)
                }
            },
        );

        let overriden_document =
            overriden_document.expect("Expected overriden document to be found");
        let target_document = target_document.expect("Expected target document to be found");

        if overriden_document
            .target_namespace
            .as_ref()
            .is_some_and(|ns| Some(ns) != target_document.target_namespace.as_ref())
        {
            todo!(
                "Handle error for merging namespaces with different namespaces: {:?} and {:?}",
                target_document.target_namespace,
                overriden_document.target_namespace
            );
        }

        let offsets = target_document.compiler.merge_with(
            &overriden_document.compiler,
            &overriden_document.target_namespace,
            &target_document.target_namespace,
        )?;

        for (name, top_level_type) in &overriden_document.top_level_types {
            target_document
                .top_level_types
                .entry(name.clone())
                .or_insert_with(|| {
                    top_level_type
                        .clone()
                        .with_offset(
                            &overriden_document.compiler.namespace_idx,
                            &target_document.compiler.namespace_idx,
                            &offsets,
                        )
                        .with_remapped_namespace(
                            &overriden_document.target_namespace,
                            &target_document.target_namespace,
                        )
                });
        }

        for (name, top_level_element) in &overriden_document.top_level_elements {
            target_document
                .top_level_elements
                .entry(name.clone())
                .or_insert_with(|| {
                    top_level_element
                        .clone()
                        .with_offset(
                            &overriden_document.compiler.namespace_idx,
                            &target_document.compiler.namespace_idx,
                            &offsets,
                        )
                        .with_remapped_namespace(
                            &overriden_document.target_namespace,
                            &target_document.target_namespace,
                        )
                });
        }

        for (name, top_level_attribute) in &overriden_document.top_level_attributes {
            target_document
                .top_level_attributes
                .entry(name.clone())
                .or_insert_with(|| {
                    top_level_attribute
                        .clone()
                        .with_offset(
                            &overriden_document.compiler.namespace_idx,
                            &target_document.compiler.namespace_idx,
                            &offsets,
                        )
                        .with_remapped_namespace(
                            &overriden_document.target_namespace,
                            &target_document.target_namespace,
                        )
                });
        }

        for (name, top_level_group) in &overriden_document.top_level_groups {
            target_document
                .top_level_groups
                .entry(name.clone())
                .or_insert_with(|| {
                    top_level_group
                        .clone()
                        .with_offset(
                            &overriden_document.compiler.namespace_idx,
                            &target_document.compiler.namespace_idx,
                            &offsets,
                        )
                        .with_remapped_namespace(
                            &overriden_document.target_namespace,
                            &target_document.target_namespace,
                        )
                });
        }

        for (name, top_level_attribute_group) in &overriden_document.top_level_attribute_groups {
            target_document
                .top_level_attribute_groups
                .entry(name.clone())
                .or_insert_with(|| {
                    top_level_attribute_group
                        .clone()
                        .with_offset(
                            &overriden_document.compiler.namespace_idx,
                            &target_document.compiler.namespace_idx,
                            &offsets,
                        )
                        .with_remapped_namespace(
                            &overriden_document.target_namespace,
                            &target_document.target_namespace,
                        )
                });
        }

        overriden_document
            .compositions
            .iter()
            .copied()
            .map(|a| {
                a.with_offset(
                    &overriden_document.compiler.namespace_idx,
                    &target_document.compiler.namespace_idx,
                    &offsets,
                )
                .with_remapped_namespace(
                    &overriden_document.target_namespace,
                    &target_document.target_namespace,
                )
            })
            .for_each(|composition_id| {
                target_document.compositions.push_front(composition_id);
            });

        override_fragment
            .schema_tops
            .iter()
            .copied()
            .into_iter()
            .rev()
            .for_each(|schema_top| {
                match schema_top {
                    SchemaTopId::Redefinable(RedefinableId::ComplexType(root_fragment)) => {
                        let fragment = target_document
                            .compiler
                            .get_fragment(&root_fragment)
                            .expect("Expected fragment to be found");

                        let name = fragment
                            .name
                            .clone()
                            .expect("Top level type should have a name");

                        target_document
                            .top_level_types
                            .insert(name, TopLevelTypeId::ComplexType(root_fragment));
                    }
                    SchemaTopId::Redefinable(RedefinableId::SimpleType(root_fragment)) => {
                        let fragment = target_document
                            .compiler
                            .simple_type_compiler
                            .get_fragment(&root_fragment)
                            .expect("Expected fragment to be found");

                        let name = fragment
                            .name
                            .clone()
                            .expect("Top level type should have a name");

                        target_document
                            .top_level_types
                            .insert(name, TopLevelTypeId::SimpleType(root_fragment));
                    }
                    SchemaTopId::Redefinable(RedefinableId::AttributeGroup(root_fragment)) => {
                        let fragment = target_document
                            .compiler
                            .get_fragment(&root_fragment)
                            .expect("Expected fragment to be found");

                        let name = fragment.name.clone();

                        target_document
                            .top_level_attribute_groups
                            .insert(name, root_fragment);
                    }
                    SchemaTopId::Redefinable(RedefinableId::Group(root_fragment)) => {
                        let fragment = target_document
                            .compiler
                            .get_fragment(&root_fragment)
                            .expect("Expected fragment to be found");

                        let name = fragment.name.clone();

                        target_document.top_level_groups.insert(name, root_fragment);
                    }
                    SchemaTopId::Redefinable(RedefinableId::Notation) => todo!(),
                    SchemaTopId::Element(element) => {
                        let fragment = target_document
                            .compiler
                            .get_fragment(&element)
                            .expect("Expected fragment to be found");

                        let name = fragment.name.clone();

                        target_document.top_level_elements.insert(name, element);
                    }
                    SchemaTopId::Attribute(attribute) => {
                        let fragment = target_document
                            .compiler
                            .get_fragment(&attribute)
                            .expect("Expected fragment to be found");

                        let name = fragment.name.clone();

                        target_document.top_level_attributes.insert(name, attribute);
                    }
                    SchemaTopId::Notation => todo!(),
                }

                target_document.schema_tops.push_front(schema_top);
            });

        overriden_document
            .schema_tops
            .iter()
            .rev()
            .copied()
            .map(|a| {
                a.with_offset(
                    &overriden_document.compiler.namespace_idx,
                    &target_document.compiler.namespace_idx,
                    &offsets,
                )
                .with_remapped_namespace(
                    &overriden_document.target_namespace,
                    &target_document.target_namespace,
                )
            })
            .filter(|id| match id {
                crate::fragments::complex::SchemaTopId::Redefinable(redefinable_id) => {
                    match redefinable_id {
                        RedefinableId::ComplexType(fragment_idx) => {
                            let fragment = target_document
                                .compiler
                                .get_fragment(&fragment_idx)
                                .expect("Expected fragment to be found");

                            let name = fragment
                                .name
                                .as_ref()
                                .expect("Top level type should have a name");

                            !overriden_complex_types.contains(name)
                        }
                        RedefinableId::SimpleType(fragment_idx) => {
                            let fragment = target_document
                                .compiler
                                .simple_type_compiler
                                .get_fragment(&fragment_idx)
                                .expect("Expected fragment to be found");

                            let name = fragment
                                .name
                                .as_ref()
                                .expect("Top level type should have a name");

                            !overriden_simple_types.contains(name)
                        }
                        RedefinableId::AttributeGroup(fragment_idx) => {
                            let fragment = target_document
                                .compiler
                                .get_fragment(&fragment_idx)
                                .expect("Expected fragment to be found");

                            let name = &fragment.name;

                            !overriden_attribute_groups.contains(name)
                        }
                        RedefinableId::Group(fragment_idx) => {
                            let fragment = target_document
                                .compiler
                                .get_fragment(&fragment_idx)
                                .expect("Expected fragment to be found");

                            let name = &fragment.name;

                            !overriden_groups.contains(name)
                        }
                        RedefinableId::Notation => todo!(),
                    }
                }
                SchemaTopId::Element(fragment_idx) => {
                    let fragment = target_document
                        .compiler
                        .get_fragment(&fragment_idx)
                        .expect("Expected fragment to be found");

                    let name = fragment.name.clone();

                    !overriden_elements.contains(&name)
                }
                SchemaTopId::Attribute(fragment_idx) => {
                    let fragment = target_document
                        .compiler
                        .get_fragment(&fragment_idx)
                        .expect("Expected fragment to be found");

                    let name = fragment.name.clone();

                    !overriden_attributes.contains(&name)
                }
                SchemaTopId::Notation => todo!(),
            })
            .for_each(|schema_top_id| {
                target_document.schema_tops.push_front(schema_top_id);
            });

        Ok(TransformChange::Changed)
    }

    fn expand_schema_overrides(
        ctx: &mut XmlnsContextTransformerContext<'_>,
        document_idx: &FragmentedXsdDocumentIdx,
    ) -> Result<TransformChange, <Self as XmlnsContextTransformer>::Error> {
        let namespace = ctx
            .xmlns_context
            .namespaces
            .get_mut(document_idx)
            .ok_or_else(|| Error::SchemaNotFound(document_idx.clone()))?;

        let (compositions, overrides) = namespace
            .compositions
            .drain(..)
            .map(|a| match a {
                crate::fragments::complex::CompositionId::Override(fragment_idx) => {
                    (None, Some(fragment_idx))
                }
                _ => (Some(a), None),
            })
            .unzip::<_, _, Vec<_>, Vec<_>>();

        namespace.compositions = compositions.into_iter().flatten().collect();

        overrides
            .into_iter()
            .flatten()
            .map(|r| Self::expand_schema_override(ctx, document_idx, &r))
            .collect()
    }
}

impl XmlnsContextTransformer for ExpandOverrideFragments {
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
            .map(|document_idx| Self::expand_schema_overrides(&mut context, &document_idx))
            .collect::<Result<TransformChange, Self::Error>>()
    }
}
