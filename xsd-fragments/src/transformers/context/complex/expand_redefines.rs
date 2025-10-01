use xmlity::ExpandedName;
use xsd::UrlExt;

use crate::{
    fragments::{
        complex::{AllFragment, AttributeDeclarationId, ChoiceFragment, ComplexOffsetable, ComplexOffsetableExt, IdOffsets, NamedGroupTypeContentId, NestedParticleId, RedefinableId, RedefineFragment, SequenceFragment, TopLevelTypeId},
        FragmentAccess, FragmentIdx, FragmentedXsdDocumentIdx,
    },
    transformers::{
        context::{
            complex::{
                ExpandAttributeDeclarations, ExpandExtensionFragments, ExpandRestrictionFragments,
            },
            simple::ExpandSimpleRestriction,
        },
        TransformChange, XmlnsContextTransformer, XmlnsContextTransformerContext,
    },
    FragmentedXsdDocumentKey,
};

/// This transformer expands the redefine fragments in the XML Schema context.
#[non_exhaustive]
pub struct ExpandRedefineFragments {}

#[derive(Debug, thiserror::Error)]
/// Error type for the [`ExpandRedefineFragments`] transformer.
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
    /// Error indicating an attribute group expansion error.
    #[error("Attribute group expansion error: {0}")]
    ExpandAttributeGroupError(#[from] crate::transformers::context::complex::ExpandAttributeDeclarationsError),
}

pub trait BTreeMapExt<K, V> {
    fn get_multiple_mut<const N: usize>(
        &mut self,
        keys: &[K; N],
    ) -> [Option<&mut V>; N];
}

impl<K, V> BTreeMapExt<K, V> for std::collections::BTreeMap<K, V> where K: std::cmp::Ord {
    fn get_multiple_mut<const N: usize>(
        &mut self,
        keys: &[K; N],
    ) -> [Option<&mut V>; N] {
        let mut result: [Option<&mut V>; N] = [(); N].map(|_| None);

        for (k, v) in self.iter_mut() {
            let index = keys.iter().position(|key| key == k);
            if let Some(i) = index {
                result[i] = Some(v);
            }
        }

        result
    }
}

impl ExpandRedefineFragments {
    /// Creates a new instance of the [`ExpandRedefineFragments`] transformer.
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {}
    }

    fn expand_nested_particle_id(
        ctx: &mut XmlnsContextTransformerContext<'_>,
        id: &NestedParticleId,
        allowed_name: &ExpandedName<'_>,
        target: &FragmentedXsdDocumentIdx,
        new: &FragmentedXsdDocumentIdx,
        offsets: &IdOffsets,
    ) -> Result<NestedParticleId, Error> { 
        match id {
            NestedParticleId::Element(fragment_idx) => {
                //TODO: Might need to expand group references in element declarations
                Ok(NestedParticleId::Element(*fragment_idx))
            },
            NestedParticleId::Group(fragment_idx) =>  Self::expand_group_ref_fragment(ctx, fragment_idx, allowed_name, target, new, offsets),
            NestedParticleId::Choice(fragment_idx) =>  Self::expand_choice_fragment(ctx, fragment_idx, allowed_name, target, new, offsets).map(NestedParticleId::Choice),
            NestedParticleId::Sequence(fragment_idx) => Self::expand_sequence_fragment(ctx, fragment_idx, allowed_name, target, new, offsets).map(NestedParticleId::Sequence),
            NestedParticleId::Any(fragment_idx) => todo!(),
        }
    }

    fn expand_group_ref_fragment(
        ctx: &mut XmlnsContextTransformerContext<'_>,
        fragment_idx: &FragmentIdx<crate::fragments::complex::GroupRefFragment>,
        allowed_name: &ExpandedName<'_>,
        target: &FragmentedXsdDocumentIdx,
        new: &FragmentedXsdDocumentIdx,
        offsets: &IdOffsets,
    ) -> Result<NestedParticleId, Error> {
        let fragment = ctx.get_complex_fragment(fragment_idx)
            .expect("Expected fragment to be found");

        let group_ref_fragment = fragment.clone();

        if &group_ref_fragment.ref_ != allowed_name {
            return Ok(NestedParticleId::Group(*fragment_idx));
        }

        let Some(named_group) = ctx.get_named_group(target, &group_ref_fragment.ref_) else {
            return Ok(NestedParticleId::Group(*fragment_idx));
        };

        let named_group = named_group.with_offset(target, new, offsets);

        let named_group = ctx.get_complex_fragment(&named_group)
            .expect("Expected fragment to be found");

            match named_group.content {
                NamedGroupTypeContentId::All(fragment_idx) => todo!(),
                NamedGroupTypeContentId::Sequence(fragment_idx) => Ok(NestedParticleId::Sequence(fragment_idx)),
                NamedGroupTypeContentId::Choice(fragment_idx) => Ok(NestedParticleId::Choice(fragment_idx)),
            }
    }

    fn expand_all_fragment(
        ctx: &mut XmlnsContextTransformerContext<'_>,
        fragment_idx: &FragmentIdx<AllFragment>,
        allowed_name: &ExpandedName<'_>,
        target: &FragmentedXsdDocumentIdx,
        new: &FragmentedXsdDocumentIdx,
        offsets: &IdOffsets,
    ) -> Result<FragmentIdx<AllFragment>, Error> {
        let fragment = ctx.get_complex_fragment(fragment_idx)
            .expect("Expected fragment to be found");

        let all_fragment = fragment.clone();

        let mut change = TransformChange::Unchanged;

        let new_fragments = all_fragment.fragments
        .into_iter()
        .map(|a| Self::expand_nested_particle_id(ctx, &a, allowed_name, target, new, offsets).inspect(|new_id| {
            if *new_id != a {
                change.mark_changed();
            }
        }))
        .collect::<Result<_, _>>()?;

        if change.is_changed() {
            let ns = ctx
                .xmlns_context
                .namespaces
                .get_mut(&fragment_idx.namespace_idx())
                .expect("Namespace not found in context");

            let new_idx = ns.compiler.push_fragment(AllFragment {
                fragments: new_fragments,
                max_occurs: all_fragment.max_occurs,
                min_occurs: all_fragment.min_occurs,
            });

            Ok(new_idx)

        } else {
            Ok(*fragment_idx)
        }
    }

    fn expand_sequence_fragment(
        ctx: &mut XmlnsContextTransformerContext<'_>,
        fragment_idx: &FragmentIdx<SequenceFragment>,
        allowed_name: &ExpandedName<'_>,
        target: &FragmentedXsdDocumentIdx,
        new: &FragmentedXsdDocumentIdx,
        offsets: &IdOffsets,
    ) -> Result<FragmentIdx<SequenceFragment>, Error> {
        let fragment = ctx.get_complex_fragment(fragment_idx)
            .expect("Expected fragment to be found");

            let sequence_fragment = fragment.clone();

        let mut change = TransformChange::Unchanged;
        
        let new_fragments = sequence_fragment.fragments
        .into_iter()
        .map(|a| Self::expand_nested_particle_id(ctx, &a, allowed_name, target, new, offsets).inspect(|new_id| {
            if *new_id != a {
                change.mark_changed();
            }
        }))
        .collect::<Result<_, _>>()?;

        if change.is_changed() {
            let ns = ctx
                .xmlns_context
                .namespaces
                .get_mut(&fragment_idx.namespace_idx())
                .expect("Namespace not found in context");

            let new_idx = ns.compiler.push_fragment(SequenceFragment {
                id: sequence_fragment.id,
                fragments: new_fragments,
                max_occurs: sequence_fragment.max_occurs,
                min_occurs: sequence_fragment.min_occurs,
            });

            Ok(new_idx)

        } else {
            Ok(*fragment_idx)
        }
    }

    fn expand_choice_fragment(
        ctx: &mut XmlnsContextTransformerContext<'_>,
        fragment_idx: &FragmentIdx<ChoiceFragment>,
        allowed_name: &ExpandedName<'_>,
        target: &FragmentedXsdDocumentIdx,
        new: &FragmentedXsdDocumentIdx,
        offsets: &IdOffsets,
    ) -> Result<FragmentIdx<ChoiceFragment>, Error> {
        let fragment = ctx.get_complex_fragment(fragment_idx)
            .expect("Expected fragment to be found");

            let choice_fragment = fragment.clone();

        let mut change = TransformChange::Unchanged;

        let new_fragments = choice_fragment.fragments
        .into_iter()
        .map(|a| Self::expand_nested_particle_id(ctx, &a, allowed_name, target, new, offsets).inspect(|new_id| {
            if *new_id != a {
                change.mark_changed();
            }
        }))
        .collect::<Result<_, _>>()?;

        if change.is_changed() {
            let ns = ctx
                .xmlns_context
                .namespaces
                .get_mut(&fragment_idx.namespace_idx())
                .expect("Namespace not found in context");

            let new_idx = ns.compiler.push_fragment(ChoiceFragment {
                fragments: new_fragments,
                max_occurs: choice_fragment.max_occurs,
                min_occurs: choice_fragment.min_occurs,
            });

            Ok(new_idx)
        } else {
            Ok(*fragment_idx)
        }
    }

    fn expand_group_references(
        ctx : &mut XmlnsContextTransformerContext<'_>,
        id: &NamedGroupTypeContentId,
        allowed_name: &ExpandedName<'_>,
        target: &FragmentedXsdDocumentIdx,
        new: &FragmentedXsdDocumentIdx,
        offsets: &IdOffsets,
    ) -> Result<NamedGroupTypeContentId, Error> { 
        match id {
            NamedGroupTypeContentId::All(fragment_idx) => {
                Self::expand_all_fragment(ctx, fragment_idx, allowed_name, target, new, offsets).map(NamedGroupTypeContentId::All)
            },
            NamedGroupTypeContentId::Sequence(fragment_idx) => {
                Self::expand_sequence_fragment(ctx, fragment_idx, allowed_name, target, new, offsets).map(NamedGroupTypeContentId::Sequence)
            },
            NamedGroupTypeContentId::Choice(fragment_idx) => {
                Self::expand_choice_fragment(ctx, fragment_idx, allowed_name, target, new, offsets).map(NamedGroupTypeContentId::Choice)
            },
        }

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
            .find(|(_, idx)| **idx == redefine_fragment_id.namespace_idx())
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

        let [redefined_document, target_document] = ctx.xmlns_context.namespaces.get_multiple_mut(&[schema_location, *target_idx]);
        
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

        let old_base_url = &current_fragment_location.0;
        let new_base_url = &key.0;

        let offsets = target_document.compiler.merge_with(
            &redefined_document.compiler,
            &redefined_document.target_namespace,
            &target_document.target_namespace,
            old_base_url,
            new_base_url,
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
                let [redefined_document, target_document] = ctx.xmlns_context.namespaces.get_multiple_mut(&[schema_location, *target_idx]);

                let redefined_document =
                    redefined_document.expect("Expected redefined document to be found");
                let target_document = target_document.expect("Expected target document to be found");

                match redefinable {
                    RedefinableId::ComplexType(root_fragment_idx) => {
                        let root_fragment = target_document
                            .compiler
                            .get_fragment(&root_fragment_idx)
                            .expect("Expected fragment to be found");
                        //TODO: Only transform if reference will disappear.
                        match root_fragment.content {
                            crate::fragments::complex::ComplexTypeModelId::SimpleContent(_) =>  {},
                            crate::fragments::complex::ComplexTypeModelId::ComplexContent(fragment_idx) => {
                                let fragment = target_document.compiler.get_fragment(&fragment_idx).expect("Expected fragment to be found");
                                match fragment.content_fragment {
                                    crate::fragments::complex::ComplexContentChildId::Extension(extension_idx) => {
                                        let extension = target_document
                                            .compiler
                                            .get_fragment(&extension_idx)
                                            .expect("Expected extension fragment to be found");

                                        assert_eq!(*extension.base.namespace(), redefined_document.target_namespace, "Right now we're assuming that the base is in the same namespace as the redefine fragment");

                                        let TopLevelTypeId::ComplexType(base_fragment_idx) = *redefined_document
                                            .top_level_types
                                            .get(&extension.base.local_name())
                                            .expect("Expected base fragment with same local name to be found")
                                        else {
                                            panic!("Expected base fragment to also be a complex type");
                                        };

                                        let base_fragment_idx = base_fragment_idx.with_offset(
                                            &redefined_document.compiler.namespace_idx,
                                            &target_document.compiler.namespace_idx,
                                            &offsets,
                                        ).with_remapped_namespace(
                                            &redefined_document.target_namespace,
                                            &target_document.target_namespace,
                                        );

                                        let _todo_use_value = ExpandExtensionFragments::expand_extension_from_base(
                                            ctx,
                                            &fragment_idx,
                                            &base_fragment_idx,
                                        ).expect("Expected extension to be expanded");
                                    },
                                    crate::fragments::complex::ComplexContentChildId::Restriction(restriction_idx) => {
                                        let restriction = target_document
                                            .compiler
                                            .get_fragment(&restriction_idx)
                                            .expect("Expected restriction fragment to be found");

                                        assert_eq!(*restriction.base.namespace(), redefined_document.target_namespace, "Right now we're assuming that the base is in the same namespace as the redefine fragment");

                                        let TopLevelTypeId::ComplexType(base_fragment_idx) = *redefined_document
                                            .top_level_types
                                            .get(restriction.base.local_name())
                                            .expect("Expected base fragment with same local name to be found")
                                        else {
                                            panic!("Expected base fragment to also be a complex type");
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
                                            .get_fragment(&base_fragment_idx)
                                            .expect("Expected base fragment to be found");


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

                                        let _todo_use_value = ExpandRestrictionFragments::expand_restriction_from_base(ctx, &restriction_idx, &base_restriction_id)
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
                            crate::fragments::simple::SimpleDerivation::Restriction(_fragment_idx) => {
                                let _todo_use_value = ExpandSimpleRestriction::flatten_restriction_with_base(
                                    ctx,
                                    &root_fragment,
                                    &base_fragment_idx
                                ).expect("Expected restriction to be expanded");
                            },
                            crate::fragments::simple::SimpleDerivation::List(_fragment_idx) =>  {},
                            crate::fragments::simple::SimpleDerivation::Union(_fragment_idx) => {},
                        }
                    }
                    RedefinableId::AttributeGroup(root_fragment) => {
                        let root_fragment = target_document
                            .compiler
                            .get_fragment(&root_fragment)
                            .expect("Expected fragment to be found");

                        let this_name = &root_fragment.name.clone();

                        let attr_decls_id = root_fragment.attr_decls;

                        let attr_decls =  ctx.get_complex_fragment(&attr_decls_id)
                            .expect("Expected fragment to be found");

                        let possible_index = attr_decls.declarations.iter().enumerate().filter_map(|(i, a)| {
                            match a {
                                crate::fragments::complex::AttributeDeclarationId::Attribute(_) => None,
                                crate::fragments::complex::AttributeDeclarationId::AttributeGroupRef(fragment_idx) => Some((i, fragment_idx)),
                            }
                        })
                        .map(|(i, id)| {
                            let fragment = ctx.get_complex_fragment(id)
                                .expect("Expected fragment to be found");

                            (i, &fragment.ref_)
                        })
                        .find_map(|(i, name)| {
                            (name.local_name() == this_name).then_some(i)
                        });

                        let Some(index) = possible_index else {
                            return;
                        };

                        let mut before = attr_decls.declarations.clone();
                        let mut after = before.split_off(index);

                        let AttributeDeclarationId::AttributeGroupRef(ref_id) = after.pop_front().expect("Expected to pop front") else {
                            unreachable!("Only finding attribute group refs");
                        };
                        
                        ExpandAttributeDeclarations::merge_attribute_group_decl(
                            &mut before,
                            ctx,
                            &ref_id,
                Some(&schema_location)
                        ).expect("Expected to expand attribute group");

                        after.iter().try_for_each(|a| match a {
                            AttributeDeclarationId::Attribute(fragment_idx) => {
                                ExpandAttributeDeclarations::add_attribute(&mut before, ctx, fragment_idx)
                            },
                            AttributeDeclarationId::AttributeGroupRef(fragment_idx) => {
                                ExpandAttributeDeclarations::add_group_ref(&mut before, ctx, fragment_idx)
                            },
                        }).expect("Expected to expand all attributes");

                        let attr_decls = ctx.get_complex_fragment_mut(&attr_decls_id)
                            .expect("Expected fragment to be found");

                        before.iter_mut().for_each(|a| a.offset(
                            &schema_location,
                            target_idx,
                            &offsets,
                        ));

                        attr_decls.declarations = before;
                    }
                    RedefinableId::Group(root_fragment_idx) => {
                        let root_fragment = target_document
                            .compiler
                            .get_fragment(&root_fragment_idx)
                            .expect("Expected fragment to be found");

                        let this_name = root_fragment.name.clone();

                        let content_id = root_fragment.content;

                        let allowed_name = ExpandedName::new(this_name, redefined_document.target_namespace.clone());

                        // TODO: Transform node to change all references to the redefined group to the content of the redefined group.
                        let transformed_content_id = Self::expand_group_references(ctx, &content_id, &allowed_name, &schema_location, target_idx, &offsets).expect("Expected to expand group references");
                        
                        let root_fragment = ctx.get_complex_fragment_mut(&root_fragment_idx)
                            .expect("Expected fragment to be found");

                        root_fragment.content = transformed_content_id;

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
