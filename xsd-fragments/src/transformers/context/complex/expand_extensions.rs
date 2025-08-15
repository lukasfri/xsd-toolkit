use std::collections::BTreeMap;
use std::collections::VecDeque;

use xmlity::ExpandedName;

use crate::fragments::complex::AssertionsFragment;
use crate::fragments::complex::AttributeDeclarationId;
use crate::fragments::complex::AttributeDeclarationsFragment;
use crate::fragments::complex::ComplexContentChildId;
use crate::fragments::complex::ComplexContentFragment;
use crate::fragments::complex::ComplexTypeModelId;
use crate::fragments::complex::ComplexTypeRootFragment;
use crate::fragments::complex::ExtensionFragment;
use crate::fragments::complex::LocalAttributeFragment;
use crate::fragments::complex::LocalAttributeFragmentTypeMode;
use crate::fragments::complex::RestrictionFragment;
use crate::fragments::complex::SequenceFragment;
use crate::fragments::complex::TopLevelTypeId;
use crate::fragments::FragmentAccess;
use crate::fragments::FragmentIdx;
use crate::transformers::TransformChange;
use crate::transformers::XmlnsContextTransformer;
use crate::transformers::XmlnsContextTransformerContext;
use xsd::xsn;

/// Expands restriction and extension fragments to their base fragments, with the modifications applied.
#[non_exhaustive]
pub struct ExpandExtensionFragments {}

#[derive(Debug, thiserror::Error)]
/// Error type for the [`ExpandExtensionFragments`] transformer.
pub enum Error {
    /// Base type was not found in the context.
    #[error("Base {base} not found in the context")]
    BaseNotFound {
        /// The base type that was not found.
        base: ExpandedName<'static>,
    },
    /// Base type is not a complex type.
    #[error("Base {base} is not a complex type")]
    BaseNotComplexType {
        /// The base type that is not a complex type.
        base: ExpandedName<'static>,
    },
    /// Base attribute group exists.
    #[error("Base attribute group exists")]
    BaseAttributeGroupExists {
        //TODO: Add more context.
    },
    /// Child attribute group exists.
    #[error("Child attribute group exists")]
    ChildAttributeGroupExists {
        //TODO: Add more context.
    },
}

impl ExpandExtensionFragments {
    /// Creates a new instance of the [`ExpandExtensionFragments`] transformer.
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {}
    }

    fn expand_attribute(
        ctx: &mut XmlnsContextTransformerContext<'_>,
        child_attribute: &FragmentIdx<LocalAttributeFragment>,
        base_attribute: &FragmentIdx<LocalAttributeFragment>,
    ) -> Result<(), <Self as XmlnsContextTransformer>::Error> {
        let base_attribute = ctx
            .get_complex_fragment(base_attribute)
            .expect("Fragment not found in compiler.")
            .clone();
        let child_attribute = ctx
            .get_complex_fragment_mut(child_attribute)
            .expect("Fragment not found in compiler.");

        if child_attribute.use_.is_none() {
            child_attribute.use_ = base_attribute.use_;
        }

        match (base_attribute.type_mode, &mut child_attribute.type_mode) {
            (
                LocalAttributeFragmentTypeMode::Declared(base),
                LocalAttributeFragmentTypeMode::Declared(child),
            ) => {
                if child.type_.is_none() {
                    child.type_ = base.type_;
                }

                Ok(())
            }
            (
                LocalAttributeFragmentTypeMode::Reference(_base),
                LocalAttributeFragmentTypeMode::Reference(_child),
            ) => Ok(()),
            _ => unreachable!(
                "Cannot expand attributes with different type modes. These should never intersect."
            ),
        }
    }

    fn expand_expanded_attributes(
        ctx: &mut XmlnsContextTransformerContext<'_>,
        child_attributes: FragmentIdx<AttributeDeclarationsFragment>,
        base_attributes: FragmentIdx<AttributeDeclarationsFragment>,
    ) -> Result<FragmentIdx<AttributeDeclarationsFragment>, <Self as XmlnsContextTransformer>::Error>
    {
        fn resolve_attr_name(
            ctx: &XmlnsContextTransformerContext,
            a: &FragmentIdx<LocalAttributeFragment>,
        ) -> ExpandedName<'static> {
            let fragment = ctx
                .get_complex_fragment(a)
                .expect("Fragment not found in compiler.");
            match &fragment.type_mode {
                LocalAttributeFragmentTypeMode::Declared(local) => {
                    ExpandedName::new(local.name.clone(), None)
                }
                LocalAttributeFragmentTypeMode::Reference(ref_) => ref_.ref_.clone(),
            }
        }

        let base_attribute_fragment = ctx
            .get_complex_fragment(&base_attributes)
            .expect("Fragment not found in compiler.")
            .clone();
        let child_attribute_fragment = ctx
            .get_complex_fragment(&child_attributes)
            .expect("Fragment not found in compiler.")
            .clone();

        let resolved_base_attributes = base_attribute_fragment
            .declarations
            .iter()
            .map(|a| match a {
                AttributeDeclarationId::Attribute(a) => Ok((*a, resolve_attr_name(ctx, a))),
                AttributeDeclarationId::AttributeGroupRef(_) => {
                    Err(Error::BaseAttributeGroupExists {})
                }
            })
            .collect::<Result<BTreeMap<_, _>, _>>()?;
        let resolved_child_attributes = child_attribute_fragment
            .declarations
            .iter()
            .map(|a| match a {
                AttributeDeclarationId::Attribute(a) => Ok((*a, resolve_attr_name(ctx, a))),
                AttributeDeclarationId::AttributeGroupRef(_) => {
                    Err(Error::ChildAttributeGroupExists {})
                }
            })
            .collect::<Result<BTreeMap<_, _>, _>>()?;

        let mut new_attribute_declarations = VecDeque::new();

        for base_attribute in base_attribute_fragment.declarations.iter() {
            let AttributeDeclarationId::Attribute(base_attribute) = base_attribute else {
                unreachable!("Can only expand attributes, not attribute groups.");
            };

            let base_attribute_name = resolved_base_attributes
                .get(base_attribute)
                .expect("Fragment not found in compiler.");

            let Some((matching_child_attribute, _)) = resolved_child_attributes
                .iter()
                .find(|(_, name)| *name == base_attribute_name)
            else {
                new_attribute_declarations
                    .push_back(AttributeDeclarationId::Attribute(*base_attribute));
                continue;
            };

            Self::expand_attribute(ctx, matching_child_attribute, base_attribute)?;

            new_attribute_declarations
                .push_back(AttributeDeclarationId::Attribute(*matching_child_attribute));
        }

        // Now we iterate through children attributes and only add those that have not been added yet because they were in the base.
        for child_attribute in child_attribute_fragment.declarations.iter() {
            let AttributeDeclarationId::Attribute(child_attribute) = child_attribute else {
                unreachable!("Can only expand attributes, not attribute groups.");
            };

            let child_attribute_name = resolved_child_attributes
                .get(child_attribute)
                .expect("Fragment not found in compiler.");

            if resolved_base_attributes
                .iter()
                .any(|(_, name)| name == child_attribute_name)
            {
                continue;
            }

            new_attribute_declarations
                .push_back(AttributeDeclarationId::Attribute(*child_attribute));
        }

        let child_attribute_fragment = ctx
            .get_complex_fragment_mut(&child_attributes)
            .expect("Fragment not found in compiler.");
        child_attribute_fragment.declarations = new_attribute_declarations;

        Ok(child_attributes)
    }

    fn expand_expanded_assertions(
        ctx: &mut XmlnsContextTransformerContext<'_>,
        child_assertions: FragmentIdx<AssertionsFragment>,
        base_assertions: FragmentIdx<AssertionsFragment>,
    ) -> Result<FragmentIdx<AssertionsFragment>, <Self as XmlnsContextTransformer>::Error> {
        let base_assertions = ctx
            .get_complex_fragment(&base_assertions)
            .expect("Fragment not found in compiler.")
            .assertions
            .clone();

        let child_assertions_fragment = ctx
            .get_complex_fragment_mut(&child_assertions)
            .expect("Fragment not found in compiler.");

        base_assertions.into_iter().for_each(|value| {
            child_assertions_fragment.assertions.push_front(value);
        });

        Ok(child_assertions)
    }

    pub fn expand_extension_from_base(
        ctx: &mut XmlnsContextTransformerContext<'_>,
        child_complex_content_fragment_idx: &FragmentIdx<ComplexContentFragment>,
        base_fragment: &FragmentIdx<ComplexTypeRootFragment>,
    ) -> Result<TransformChange, <Self as XmlnsContextTransformer>::Error> {
        let child_complex_content_fragment = ctx
            .get_complex_fragment(child_complex_content_fragment_idx)
            .expect("Fragment not found in compiler.");
        let child_fragment_idx = match child_complex_content_fragment.content_fragment {
            ComplexContentChildId::Extension(fragment_idx) => fragment_idx,
            ComplexContentChildId::Restriction(_) => {
                return Ok(TransformChange::Unchanged);
            }
        };

        let child_fragment = ctx
            .get_complex_fragment(&child_fragment_idx)
            .expect("Fragment not found in compiler.");

        let base_root_fragment = ctx
            .get_complex_fragment::<ComplexTypeRootFragment>(&base_fragment)
            .expect("Fragment not found in compiler.");

        let (base_content_content_fragment_id, base_content_base, base_attributes, base_assertions) =
            match base_root_fragment.content {
                ComplexTypeModelId::Other {
                    particle,
                    attr_decls,
                    assertions,
                    ..
                } => (particle, xsn::ANY_TYPE.clone(), attr_decls, assertions),
                ComplexTypeModelId::SimpleContent(_) => {
                    //TODO
                    return Ok(TransformChange::Unchanged);
                }
                ComplexTypeModelId::ComplexContent(base_complex_content_id) => {
                    let base_content_fragment = ctx
                        .get_complex_fragment(&base_complex_content_id)
                        .expect("Fragment not found in compiler.");

                    // Checks if base content is either a restriction of xs:anyType or an extension. If it is a non-anyType restriction, we cannot expand it since it could create a type that is not a valid derivative of the base's base type.

                    match base_content_fragment.content_fragment {
                        ComplexContentChildId::Extension(fragment_idx) => {
                            let base_extension_fragment = ctx
                                .get_complex_fragment::<ExtensionFragment>(&fragment_idx)
                                .expect("Fragment not found in compiler.");
                            (
                                base_extension_fragment.content_fragment,
                                base_extension_fragment.base.clone(),
                                base_extension_fragment.attribute_declarations,
                                base_extension_fragment.assertions.clone(),
                            )
                        }
                        ComplexContentChildId::Restriction(fragment_idx) => {
                            let base_restriction_fragment = ctx
                                .get_complex_fragment::<RestrictionFragment>(&fragment_idx)
                                .expect("Fragment not found in compiler.");

                            if base_restriction_fragment.base != *xsn::ANY_TYPE {
                                // Cannot expand a restriction of a non-anyType type.
                                return Ok(TransformChange::Unchanged);
                            }

                            (
                                base_restriction_fragment.content_fragment,
                                base_restriction_fragment.base.clone(),
                                base_restriction_fragment.attribute_declarations,
                                base_restriction_fragment.assertions.clone(),
                            )
                        }
                    }
                }
            };

        let child_attributes = child_fragment.attribute_declarations;

        let child_assertions = child_fragment.assertions;

        let new_content_fragment = child_fragment
            .content_fragment
            .map(|child_content_content_fragment_id| {
                let Some(base_content_content_fragment_id) = base_content_content_fragment_id
                else {
                    return child_content_content_fragment_id;
                };

                let new_content_fragment = SequenceFragment {
                    id: None,
                    max_occurs: None,
                    min_occurs: None,
                    fragments: VecDeque::from([
                        base_content_content_fragment_id
                            .try_into()
                            .expect("TODO: Error handling for conversion failure"),
                        child_content_content_fragment_id
                            .try_into()
                            .expect("TODO: Error handling for conversion failure"),
                    ]),
                };

                let ns = &mut ctx
                    .xmlns_context
                    .namespaces
                    .get_mut(&child_complex_content_fragment_idx.namespace_idx())
                    .unwrap();

                let new_content_fragment = ns.compiler.push_fragment(new_content_fragment);

                new_content_fragment.into()
            })
            .or(base_content_content_fragment_id);

        let new_attribute_declarations =
            Self::expand_expanded_attributes(ctx, child_attributes, base_attributes)?;

        let new_assertions =
            Self::expand_expanded_assertions(ctx, child_assertions, base_assertions)?;

        let new_child_content = RestrictionFragment {
            base: base_content_base,
            content_fragment: new_content_fragment,
            attribute_declarations: new_attribute_declarations,
            assertions: new_assertions,
        };

        let ns = &mut ctx
            .xmlns_context
            .namespaces
            .get_mut(&child_complex_content_fragment_idx.namespace_idx())
            .unwrap();

        let new_child_content = ns.compiler.push_fragment(new_child_content);

        ctx.get_complex_fragment_mut(child_complex_content_fragment_idx)
            .expect("Fragment not found in compiler.")
            .content_fragment = ComplexContentChildId::Restriction(new_child_content);

        Ok(TransformChange::Changed)
    }

    fn expand_extension(
        ctx: &mut XmlnsContextTransformerContext<'_>,
        child_complex_content_fragment_idx: &FragmentIdx<ComplexContentFragment>,
    ) -> Result<TransformChange, <Self as XmlnsContextTransformer>::Error> {
        let child_complex_content_fragment = ctx
            .get_complex_fragment(child_complex_content_fragment_idx)
            .expect("Fragment not found in compiler.");
        let child_fragment_idx = match child_complex_content_fragment.content_fragment {
            ComplexContentChildId::Extension(fragment_idx) => fragment_idx,
            ComplexContentChildId::Restriction(_) => {
                return Ok(TransformChange::Unchanged);
            }
        };

        let child_fragment = ctx
            .get_complex_fragment(&child_fragment_idx)
            .expect("Fragment not found in compiler.");

        let base = child_fragment.base.clone();

        if base == *xsn::ANY_TYPE {
            return Ok(TransformChange::Unchanged);
        }

        let base_fragment = ctx
            .get_named_type(&child_fragment_idx.namespace_idx(), &base)
            .ok_or(Error::BaseNotFound { base: base.clone() })?;

        let base_fragment = match base_fragment {
            TopLevelTypeId::ComplexType(complex) => *complex,
            TopLevelTypeId::SimpleType(_) => {
                return Err(Error::BaseNotComplexType { base: base.clone() });
            }
        };

        Self::expand_extension_from_base(ctx, child_complex_content_fragment_idx, &base_fragment)
    }
}

impl XmlnsContextTransformer for ExpandExtensionFragments {
    type Error = Error;

    fn transform(
        self,
        mut ctx: XmlnsContextTransformerContext<'_>,
    ) -> Result<TransformChange, Self::Error> {
        ctx.iter_complex_fragment_ids()
            .collect::<Vec<_>>()
            .into_iter()
            .map(|f| Self::expand_extension(&mut ctx, &f))
            .collect()
    }
}
