//! Transformer for expanding attribute group references into individual attributes.

use std::collections::VecDeque;

use xmlity::ExpandedName;

use crate::fragments::{
    complex::{
        AttributeDeclarationId, AttributeDeclarationsFragment, AttributeGroupRefFragment,
        LocalAttributeFragment, LocalAttributeFragmentTypeMode,
    },
    FragmentAccess, FragmentIdx,
};
use crate::transformers::{
    TransformChange, XmlnsContextTransformer, XmlnsContextTransformerContext,
};

/// Transformer that expands [`AttributeGroupRefFragment`] into their constituent attributes.
#[non_exhaustive]
pub struct ExpandAttributeDeclarations {}

/// Error type for the [`ExpandAttributeDeclarations`] transformer.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    //TODO: Review if this is actually a requirement.
    /// When merging attributes, the type modes must be compatible.
    #[error("When merging, the attribute type modes must be the same")]
    MismatchedAttributeModes,
    /// When an attribute group is referenced, but the group is not found in the context.
    #[error("Named attribute group not found: {name}")]
    NamedAttributeGroupNotFound {
        /// The name of the attribute group that was not found.
        name: ExpandedName<'static>,
    },
}

impl ExpandAttributeDeclarations {
    /// Creates a new instance of the [`ExpandAttributeDeclarations`] transformer.
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {}
    }

    fn expand_attribute_declaration(
        context: &mut XmlnsContextTransformerContext<'_>,
        fragment_id: &FragmentIdx<AttributeDeclarationsFragment>,
    ) -> Result<TransformChange, <Self as XmlnsContextTransformer>::Error> {
        let mut change = TransformChange::default();

        let fragment = context
            .get_complex_fragment(fragment_id)
            .expect("Fragment not found in compiler.");

        let mut new_attributes = VecDeque::new();

        fn merge_attribute(
            ctx: &mut XmlnsContextTransformerContext<'_>,
            target_idx: &FragmentIdx<LocalAttributeFragment>,
            source_idx: &FragmentIdx<LocalAttributeFragment>,
        ) -> Result<FragmentIdx<LocalAttributeFragment>, Error> {
            let source = ctx
                .get_complex_fragment(source_idx)
                .expect("Fragment not found in compiler.")
                .clone();

            let mut changed = false;

            let mut target = ctx
                .get_complex_fragment(target_idx)
                .expect("Fragment not found in compiler.")
                .clone();

            if let Some(default) = &source.default {
                if source.default != target.default {
                    changed = true;
                    target.default = Some(default.clone());
                }
            }

            if let Some(use_) = source.use_ {
                if target.use_ != Some(use_.clone()) {
                    changed = true;
                    target.use_ = Some(use_);
                }
            }

            match (&mut target.type_mode, &source.type_mode) {
                (
                    LocalAttributeFragmentTypeMode::Declared(target_declared),
                    LocalAttributeFragmentTypeMode::Declared(source_declared),
                ) => {
                    assert_eq!(
                        target_declared.name, source_declared.name,
                        "When merging, the attribute names must be the same"
                    );

                    if let Some(type_) = source_declared.type_.clone() {
                        if target_declared.type_ != Some(type_.clone()) {
                            changed = true;
                            target_declared.type_ = Some(type_);
                        }
                    }
                }
                (
                    LocalAttributeFragmentTypeMode::Reference(target_reference),
                    LocalAttributeFragmentTypeMode::Reference(source_reference),
                ) => {
                    assert_eq!(
                        target_reference.ref_, source_reference.ref_,
                        "When merging, the attribute references must be the same"
                    );
                }
                _ => return Err(Error::MismatchedAttributeModes),
            };

            if changed {
                let ns = ctx
                    .xmlns_context
                    .namespaces
                    .get_mut(&target_idx.namespace_idx())
                    .expect("Namespace not found in context");

                let new_idx = ns.compiler.push_fragment(target);

                Ok(new_idx)
            } else {
                Ok(*target_idx)
            }
        }

        fn add_attribute(
            new_attributes: &mut VecDeque<AttributeDeclarationId>,
            ctx: &mut XmlnsContextTransformerContext<'_>,
            new_idx: &FragmentIdx<LocalAttributeFragment>,
        ) -> Result<(), Error> {
            let new = ctx
                .get_complex_fragment(new_idx)
                .expect("Fragment not found in compiler.");

            // Check if the attribute already exists in the new_attributes list
            let attribute_exists = new_attributes
                .iter()
                .enumerate()
                .filter_map(|(i, a)| match a {
                    AttributeDeclarationId::Attribute(a) => Some((i, a)),
                    _ => None,
                })
                .map(|(i, a)| {
                    (
                        i,
                        ctx.get_complex_fragment(a)
                            .expect("Fragment not found in compiler."),
                    )
                })
                .find(
                    |(_, existing)| match (&existing.type_mode, &new.type_mode) {
                        (
                            LocalAttributeFragmentTypeMode::Declared(existing),
                            LocalAttributeFragmentTypeMode::Declared(new),
                        ) => existing.name == new.name,
                        (
                            LocalAttributeFragmentTypeMode::Reference(existing),
                            LocalAttributeFragmentTypeMode::Reference(new),
                        ) => existing.ref_ == new.ref_,
                        _ => false,
                    },
                )
                .map(|(i, _)| i);

            // If the attribute does not exist, add it to the new_attributes list
            let Some(i) = attribute_exists else {
                new_attributes.push_back(AttributeDeclarationId::Attribute(*new_idx));
                return Ok(());
            };

            // Otherwise, merge the attributes
            let AttributeDeclarationId::Attribute(existing_idx) = new_attributes
                .get(i)
                .expect("Attribute must exist in the list since we just found it")
            else {
                unreachable!("Attribute must exist in the list since we just found it - we filtered out attribute groups")
            };

            let resulting_idx = merge_attribute(ctx, existing_idx, new_idx)?;
            new_attributes[i] = AttributeDeclarationId::Attribute(resulting_idx);

            Ok(())
        }

        fn add_group(
            new_attributes: &mut VecDeque<AttributeDeclarationId>,
            ctx: &mut XmlnsContextTransformerContext<'_>,
            fragment_idx: &FragmentIdx<AttributeGroupRefFragment>,
        ) {
            let possible = ctx
                .get_complex_fragment(fragment_idx)
                .expect("Fragment not found in compiler.")
                .clone();

            // Check if the attribute group already exists in the new_attributes list
            let group_exists = new_attributes
                .iter()
                .enumerate()
                .filter_map(|(i, a)| match a {
                    AttributeDeclarationId::AttributeGroupRef(a) => Some((i, a)),
                    _ => None,
                })
                .map(|(i, a)| {
                    (
                        i,
                        ctx.get_complex_fragment(a)
                            .expect("Fragment not found in compiler."),
                    )
                })
                .any(|(_, existing)| existing.ref_ == possible.ref_);

            // If the attribute group does not exist, add it to the new_attributes list
            if !group_exists {
                new_attributes.push_back(AttributeDeclarationId::AttributeGroupRef(*fragment_idx));
            }
        }

        // We iterate through all attributes and attribute groups, applying edits to already existing attributes.
        for attributes in fragment.declarations.clone().iter() {
            match attributes {
                AttributeDeclarationId::Attribute(fragment_idx) => {
                    add_attribute(&mut new_attributes, context, fragment_idx)?;
                }
                AttributeDeclarationId::AttributeGroupRef(fragment_idx) => {
                    change = TransformChange::Changed;

                    let attribute_fragment = context
                        .get_complex_fragment::<AttributeGroupRefFragment>(fragment_idx)
                        .expect("Fragment not found in compiler.");

                    let group = context
                        .get_named_attribute_group(
                            &fragment_idx.namespace_idx(),
                            &attribute_fragment.ref_,
                        )
                        .ok_or_else(|| Error::NamedAttributeGroupNotFound {
                            name: attribute_fragment.ref_.clone(),
                        })?;

                    let group = context
                        .get_complex_fragment(&group)
                        .expect("Fragment not found in compiler.");
                    let attr_decls = context
                        .get_complex_fragment(&group.attr_decls)
                        .expect("Fragment not found in compiler.");

                    for declaration in attr_decls.declarations.clone().iter() {
                        match declaration {
                            AttributeDeclarationId::Attribute(fragment_idx) => {
                                add_attribute(&mut new_attributes, context, fragment_idx)?;
                            }
                            AttributeDeclarationId::AttributeGroupRef(fragment_idx) => {
                                add_group(&mut new_attributes, context, fragment_idx);
                            }
                        }
                    }
                }
            }
        }

        let fragment = context
            .get_complex_fragment_mut(fragment_id)
            .expect("Fragment not found in compiler.");

        fragment.declarations = new_attributes;

        Ok(change)
    }
}

impl XmlnsContextTransformer for ExpandAttributeDeclarations {
    type Error = Error;

    fn transform(
        self,
        mut context: XmlnsContextTransformerContext<'_>,
    ) -> Result<TransformChange, Self::Error> {
        context
            .iter_complex_fragment_ids::<AttributeDeclarationsFragment>()
            .collect::<Vec<_>>()
            .iter()
            .map(|fragment_id| Self::expand_attribute_declaration(&mut context, fragment_id))
            .collect()
    }
}
