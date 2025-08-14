use xsd::UrlExt;

use crate::{
    fragments::{
        complex::{ComplexOffsetableExt, RedefinableId, RedefineFragment, TopLevelTypeId},
        FragmentAccess, FragmentIdx, FragmentedXsdDocumentIdx,
    },
    transformers::{
        context::{complex::ExpandRestrictionFragments, simple::ExpandSimpleRestriction},
        TransformChange, XmlnsContextTransformer, XmlnsContextTransformerContext,
    },
    FragmentedXsdDocumentKey,
};

#[non_exhaustive]
pub struct ExpandRedefineFragments {}

#[derive(Debug, thiserror::Error)]
/// Error type for the [`ExpandRedefineFragments`] transformer.
pub enum Error {
    #[error("Schema not found: {0}")]
    SchemaNotFound(FragmentedXsdDocumentIdx),
    #[error("Complex fragment error: {0}")]
    ComplexFragment(#[from] crate::fragments::complex::Error),
    #[error("Simple fragment error: {0}")]
    SimpleFragment(#[from] crate::fragments::simple::Error),
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
        redefine_fragment_id: &FragmentIdx<RedefineFragment>,
    ) -> Result<TransformChange, Error> {
        let redefine_fragment = ctx
            .get_complex_fragment(&redefine_fragment_id)
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
            .resolve_xml_url(&redefine_fragment.schema_location)
            .expect("Expected a valid URL");

        let key = FragmentedXsdDocumentKey(location_url);

        let schema_location = ctx
            .xmlns_context
            .namespace_idxs
            .get(&key)
            .cloned()
            .ok_or_else(|| Error::SchemaNotFound(target_idx.clone()))?;

        let redefined_complex_types = redefine_fragment
            .redefineable
            .iter()
            .filter_map(|redefinable| match redefinable {
                RedefinableId::ComplexType(fragment_idx) => Some(fragment_idx),
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

        let redefined_simple_types = redefine_fragment
            .redefineable
            .iter()
            .filter_map(|redefinable| match redefinable {
                RedefinableId::SimpleType(fragment_idx) => Some(fragment_idx),
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

        let redefined_attribute_groups = redefine_fragment
            .redefineable
            .iter()
            .filter_map(|redefinable| match redefinable {
                RedefinableId::AttributeGroup(fragment_idx) => Some(fragment_idx),
                _ => None,
            })
            .map(|fragment_idx| {
                ctx.get_complex_fragment(&fragment_idx)
                    .expect("Expected fragment to be found")
            })
            .map(|fragment| fragment.name.clone())
            .collect::<Vec<_>>();

        let redefined_groups = redefine_fragment
            .redefineable
            .iter()
            .filter_map(|redefinable| match redefinable {
                RedefinableId::Group(fragment_idx) => Some(fragment_idx),
                _ => None,
            })
            .map(|fragment_idx| {
                ctx.get_complex_fragment(&fragment_idx)
                    .expect("Expected fragment to be found")
            })
            .map(|fragment| fragment.name.clone())
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

        if redefined_document
            .target_namespace
            .as_ref()
            .is_some_and(|ns| Some(ns) != target_document.target_namespace.as_ref())
        {
            todo!(
                "Handle error for merging namespaces with different namespaces: {:?} and {:?}",
                target_document.target_namespace,
                redefined_document.target_namespace
            );
        }

        let offsets = target_document.compiler.merge_with(
            &redefined_document.compiler,
            &redefined_document.target_namespace,
            &target_document.target_namespace,
        )?;

        for (name, top_level_type) in &redefined_document.top_level_types {
            target_document
                .top_level_types
                .entry(name.clone())
                .or_insert_with(|| {
                    top_level_type
                        .clone()
                        .with_offset(
                            &redefined_document.compiler.namespace_idx,
                            &target_document.compiler.namespace_idx,
                            &offsets,
                        )
                        .with_remapped_namespace(
                            &redefined_document.target_namespace,
                            &target_document.target_namespace,
                        )
                });
        }

        for (name, top_level_element) in &redefined_document.top_level_elements {
            target_document
                .top_level_elements
                .entry(name.clone())
                .or_insert_with(|| {
                    top_level_element
                        .clone()
                        .with_offset(
                            &redefined_document.compiler.namespace_idx,
                            &target_document.compiler.namespace_idx,
                            &offsets,
                        )
                        .with_remapped_namespace(
                            &redefined_document.target_namespace,
                            &target_document.target_namespace,
                        )
                });
        }

        for (name, top_level_attribute) in &redefined_document.top_level_attributes {
            target_document
                .top_level_attributes
                .entry(name.clone())
                .or_insert_with(|| {
                    top_level_attribute
                        .clone()
                        .with_offset(
                            &redefined_document.compiler.namespace_idx,
                            &target_document.compiler.namespace_idx,
                            &offsets,
                        )
                        .with_remapped_namespace(
                            &redefined_document.target_namespace,
                            &target_document.target_namespace,
                        )
                });
        }

        for (name, top_level_group) in &redefined_document.top_level_groups {
            target_document
                .top_level_groups
                .entry(name.clone())
                .or_insert_with(|| {
                    top_level_group
                        .clone()
                        .with_offset(
                            &redefined_document.compiler.namespace_idx,
                            &target_document.compiler.namespace_idx,
                            &offsets,
                        )
                        .with_remapped_namespace(
                            &redefined_document.target_namespace,
                            &target_document.target_namespace,
                        )
                });
        }

        for (name, top_level_attribute_group) in &redefined_document.top_level_attribute_groups {
            target_document
                .top_level_attribute_groups
                .entry(name.clone())
                .or_insert_with(|| {
                    top_level_attribute_group
                        .clone()
                        .with_offset(
                            &redefined_document.compiler.namespace_idx,
                            &target_document.compiler.namespace_idx,
                            &offsets,
                        )
                        .with_remapped_namespace(
                            &redefined_document.target_namespace,
                            &target_document.target_namespace,
                        )
                });
        }

        redefined_document
            .compositions
            .iter()
            .copied()
            .map(|a| {
                a.with_offset(
                    &redefined_document.compiler.namespace_idx,
                    &target_document.compiler.namespace_idx,
                    &offsets,
                )
                .with_remapped_namespace(
                    &redefined_document.target_namespace,
                    &target_document.target_namespace,
                )
            })
            .for_each(|composition_id| {
                target_document.compositions.push_front(composition_id);
            });

        redefine_fragment
            .redefineable
            .iter()
            .copied()
            .into_iter()
            .rev()
            .for_each(|redefinable| {
                match redefinable {
                    RedefinableId::ComplexType(root_fragment) => {
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
                    RedefinableId::SimpleType(root_fragment) => {
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
                    RedefinableId::AttributeGroup(root_fragment) => {
                        let fragment = target_document
                            .compiler
                            .get_fragment(&root_fragment)
                            .expect("Expected fragment to be found");

                        let name = fragment.name.clone();

                        target_document
                            .top_level_attribute_groups
                            .insert(name, root_fragment);
                    }
                    RedefinableId::Group(root_fragment) => {
                        let fragment = target_document
                            .compiler
                            .get_fragment(&root_fragment)
                            .expect("Expected fragment to be found");

                        let name = fragment.name.clone();

                        target_document.top_level_groups.insert(name, root_fragment);
                    }
                    RedefinableId::Notation => todo!(),
                }

                target_document.schema_tops.push_front(
                    crate::fragments::complex::SchemaTopId::Redefinable(redefinable),
                );
            });

        redefined_document
            .schema_tops
            .iter()
            .rev()
            .copied()
            .map(|a| {
                a.with_offset(
                    &redefined_document.compiler.namespace_idx,
                    &target_document.compiler.namespace_idx,
                    &offsets,
                )
                .with_remapped_namespace(
                    &redefined_document.target_namespace,
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

                            !redefined_complex_types.contains(name)
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

                            !redefined_simple_types.contains(name)
                        }
                        RedefinableId::AttributeGroup(fragment_idx) => {
                            let fragment = target_document
                                .compiler
                                .get_fragment(&fragment_idx)
                                .expect("Expected fragment to be found");

                            let name = &fragment.name;

                            !redefined_attribute_groups.contains(name)
                        }
                        RedefinableId::Group(fragment_idx) => {
                            let fragment = target_document
                                .compiler
                                .get_fragment(&fragment_idx)
                                .expect("Expected fragment to be found");

                            let name = &fragment.name;

                            !redefined_groups.contains(name)
                        }
                        RedefinableId::Notation => todo!(),
                    }
                }
                _ => true,
            })
            .for_each(|schema_top_id| {
                target_document.schema_tops.push_front(schema_top_id);
            });

        redefine_fragment
            .redefineable
            .iter()
            .copied()
            .into_iter()
            .rev()
            .for_each(|redefinable| {
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

                match redefinable {
                    RedefinableId::ComplexType(root_fragment) => {
                        let fragment = target_document
                            .compiler
                            .get_fragment(&root_fragment)
                            .expect("Expected fragment to be found");

                        let name = fragment
                            .name
                            .clone()
                            .expect("Top level type should have a name");

                        let TopLevelTypeId::ComplexType(base_fragment) = *redefined_document
                            .top_level_types
                            .get(&name)
                            .expect("Expected base fragment with same local name to be found")
                        else {
                            panic!("Expected base fragment to also be a complex type");
                        };

                        let base_fragment = base_fragment.with_offset(
                            &redefined_document.compiler.namespace_idx,
                            &target_document.compiler.namespace_idx,
                            &offsets,
                        ).with_remapped_namespace(
                            &redefined_document.target_namespace,
                            &target_document.target_namespace,
                        );

                        let base_fragment = target_document
                            .compiler
                            .get_fragment(&base_fragment)
                            .expect("Expected base fragment to be found");

                        //TODO: Only transform if reference will disappear.
                        match fragment.content {
                            crate::fragments::complex::ComplexTypeModelId::SimpleContent(_) =>  {},
                            crate::fragments::complex::ComplexTypeModelId::ComplexContent(fragment_idx) => {
                                let fragment = target_document.compiler.get_fragment(&fragment_idx).expect("Expected fragment to be found");
                                match fragment.content_fragment {
                                    crate::fragments::complex::ComplexContentChildId::Extension(_) => {
                                        todo!("Handle extension of complex type in redefine");
                                    },
                                    crate::fragments::complex::ComplexContentChildId::Restriction(child_fragment_idx) => {
                                        let crate::fragments::complex::ComplexTypeModelId::ComplexContent(base_complex_content_id) =
                                            base_fragment.content else {
                                            panic!("Expected base fragment to have complex content");
                                        };

                                        let base_content_fragment = target_document
                                            .compiler
                                            .get_fragment(&base_complex_content_id)
                                            .expect("Expected base content fragment to be found");

                                        let crate::fragments::complex::ComplexContentChildId::Restriction(base_restriction_id) =
                                            base_content_fragment.content_fragment else {
                                            panic!("Expected base content fragment to have restriction");
                                        }; //TODO: Handle extension of complex type in redefine

                                        let _todo_use_value = ExpandRestrictionFragments::expand_restriction_from_base(ctx, &child_fragment_idx, &base_restriction_id)
                                            .expect("Expected restriction to be expanded");
                                    },
                                }
                            },
                            crate::fragments::complex::ComplexTypeModelId::Other { .. } => {},
                        }
                    }

                        //TODO: Only transform if reference will disappear.
                    RedefinableId::SimpleType(root_fragment) => {
                        let fragment = target_document
                            .compiler
                            .simple_type_compiler
                            .get_fragment(&root_fragment)
                            .expect("Expected fragment to be found");

                        let name = fragment
                            .name
                            .clone()
                            .expect("Top level type should have a name");

                        let TopLevelTypeId::SimpleType(base_fragment_idx) = redefined_document
                            .top_level_types
                            .get(&name)
                            .expect("Expected base fragment with same local name to be found")
                        else {
                            panic!("Expected base fragment to also be a simple type");
                        };

                        let base_fragment_idx = base_fragment_idx.with_offset(
                            &redefined_document.compiler.namespace_idx,
                            &target_document.compiler.namespace_idx,
                            &offsets,
                        ).with_remapped_namespace(
                            &redefined_document.target_namespace,
                            &target_document.target_namespace,
                        );

                        let base_fragment = target_document
                            .compiler
                            .simple_type_compiler
                            .get_fragment(&base_fragment_idx)
                            .expect("Expected base fragment to be found");

                        match base_fragment.simple_derivation {
                            crate::fragments::simple::SimpleDerivation::Restriction(fragment_idx) => {
                                let _todo_use_value = ExpandSimpleRestriction::flatten_restriction_with_base(
                                    ctx,
                                    &root_fragment,
                                    &base_fragment_idx
                                ).expect("Expected restriction to be expanded");
                            },
                            crate::fragments::simple::SimpleDerivation::List(fragment_idx) =>  {},
                            crate::fragments::simple::SimpleDerivation::Union(fragment_idx) => {},
                        }


                    }
                    RedefinableId::AttributeGroup(root_fragment) => {
                    }
                    RedefinableId::Group(root_fragment) => {

                    }
                    RedefinableId::Notation => todo!(),
                }
            });

        Ok(TransformChange::Changed)
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
