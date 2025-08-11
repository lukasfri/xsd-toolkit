use std::collections::BTreeMap;
use std::collections::VecDeque;

use crate::fragments::complex::AttributeDeclarationId;
use crate::fragments::complex::AttributeDeclarationsFragment;
use crate::fragments::complex::ComplexContentChildId;
use crate::fragments::complex::ComplexTypeModelId;
use crate::fragments::complex::ComplexTypeRootFragment;
use crate::fragments::complex::LocalAttributeFragment;
use crate::fragments::complex::LocalAttributeFragmentTypeMode;
use crate::fragments::complex::RestrictionFragment;
use crate::fragments::complex::TopLevelTypeId;
use crate::fragments::FragmentIdx;
use xmlity::ExpandedName;
use xsd::xsn;

use crate::transformers::TransformChange;
use crate::transformers::XmlnsContextTransformer;
use crate::transformers::XmlnsContextTransformerContext;

/// Expands restriction and extension fragments to their base fragments, with the modifications applied.
#[non_exhaustive]
pub struct ExpandRestrictionFragments {}

/// Error type for the [`ExpandRestrictionFragments`] transformer.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Tried to restrict a reference attribute with a base attribute that is not a reference.
    #[error(
        "Cannot restrict reference attribute {attribute:?} with base attribute {base_attribute:?}"
    )]
    CannotRestrictReferenceAttribute {
        /// The attribute that was attempted to be restricted.
        attribute: FragmentIdx<LocalAttributeFragment>,
        /// The base attribute that was attempted to be used for restriction.
        base_attribute: FragmentIdx<LocalAttributeFragment>,
    },
    /// Cannot handle attribute group references in attribute declarations.
    #[error(
        "Cannot handle attribute group reference in attribute declarations. This transformer does not support attribute group references."
    )]
    CannotHandleAttributeGroupRef {},
    /// Cannot restrict a base type to a simple type, only complex types can be restricted.
    #[error("Cannot restrict base type {base:?} to a simple type. Only complex types can be restricted.")]
    BaseCannotBeSimpleType {
        /// The base type that was attempted to be restricted.
        base: ExpandedName<'static>,
    },
    /// The base type that was attempted to be restricted cannot be simple content.
    #[error("Base type {base:?} cannot be simple content. Only complex types with complex content can be restricted.")]
    BaseContentCannotBeSimpleContent {
        /// The base type that was attempted to be restricted.
        base: ExpandedName<'static>,
    },
    /// The base type that was attempted to be restricted cannot be standalone.
    #[error("Base type {base:?} cannot be standalone. Only complex types with complex content can be restricted.")]
    BaseContentCannotBeStandalone {
        /// The base type that was attempted to be restricted.
        base: ExpandedName<'static>,
    },
    /// Cannot restrict a base type to an extension type, only complex types can be extended.
    #[error("Cannot restrict base type {base:?} to an extension type. Only complex types can be extended.")]
    BaseCannotBeExtensionType {
        /// The base type that was attempted to be restricted.
        base: ExpandedName<'static>,
    },
    /// The base type does not exist in the namespace, so it cannot be expanded.
    #[error("Base type {base:?} does not exist in the namespace. Cannot expand restriction.")]
    BaseDoesNotExist {
        /// The base type that was attempted to be expanded.
        base: ExpandedName<'static>,
    },
}

impl ExpandRestrictionFragments {
    /// Creates a new instance of the [`ExpandRestrictionFragments`] transformer.
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {}
    }

    fn restrict_attribute(
        ctx: &mut XmlnsContextTransformerContext<'_>,
        attribute_id: &FragmentIdx<LocalAttributeFragment>,
        base_attribute_id: &FragmentIdx<LocalAttributeFragment>,
    ) -> Result<(), <Self as XmlnsContextTransformer>::Error> {
        let base_attribute = ctx
            .get_complex_fragment(base_attribute_id)
            .expect("Fragment not found in compiler.")
            .clone();
        let attribute = ctx
            .get_complex_fragment_mut(attribute_id)
            .expect("Fragment not found in compiler.");

        use LocalAttributeFragmentTypeMode as TypeMode;

        match (base_attribute.type_mode, &mut attribute.type_mode) {
            (TypeMode::Declared(decl_base_attribute), TypeMode::Declared(decl_attribute)) => {
                if attribute.use_.is_none() {
                    attribute.use_ = base_attribute.use_;
                }
                if decl_attribute.type_.is_none() {
                    decl_attribute.type_ = decl_base_attribute.type_;
                }
            }
            (TypeMode::Reference(_decl_base_attribute), TypeMode::Reference(_decl_attribute)) => {
                if attribute.use_.is_none() {
                    attribute.use_ = base_attribute.use_;
                }
            }
            _ => {
                return Err(Error::CannotRestrictReferenceAttribute {
                    attribute: *attribute_id,
                    base_attribute: *base_attribute_id,
                });
            }
        };

        Ok(())
    }

    fn expand_restricted_attributes(
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
                    Err(Error::CannotHandleAttributeGroupRef {})
                }
            })
            .collect::<Result<BTreeMap<_, _>, <Self as XmlnsContextTransformer>::Error>>()?;
        let resolved_child_attributes = child_attribute_fragment
            .declarations
            .iter()
            .map(|a| match a {
                AttributeDeclarationId::Attribute(a) => Ok((*a, resolve_attr_name(ctx, a))),
                AttributeDeclarationId::AttributeGroupRef(_) => {
                    Err(Error::CannotHandleAttributeGroupRef {})
                }
            })
            .collect::<Result<BTreeMap<_, _>, <Self as XmlnsContextTransformer>::Error>>()?;

        let mut new_attribute_declarations = VecDeque::new();

        for base_attribute in base_attribute_fragment.declarations.iter() {
            let AttributeDeclarationId::Attribute(base_attribute) = base_attribute else {
                unreachable!("If attribute group reference was present, it would have been handled in the previous map.");
            };

            let base_attribute_name = resolved_base_attributes
                .get(base_attribute)
                .expect("Attribute not found in resolved base attributes.");

            let Some((matching_child_attribute, _)) = resolved_child_attributes
                .iter()
                .find(|(_, name)| *name == base_attribute_name)
            else {
                new_attribute_declarations
                    .push_back(AttributeDeclarationId::Attribute(*base_attribute));
                continue;
            };

            Self::restrict_attribute(ctx, matching_child_attribute, base_attribute)?;

            new_attribute_declarations
                .push_back(AttributeDeclarationId::Attribute(*matching_child_attribute));
        }

        // Now we iterate through children attributes and only add those that have not been added yet because they were in the base.
        for child_attribute in child_attribute_fragment.declarations.iter() {
            let AttributeDeclarationId::Attribute(child_attribute) = child_attribute else {
                unreachable!("If attribute group reference was present, it would have been handled in the previous map.");
            };

            let child_attribute_name = resolved_child_attributes
                .get(child_attribute)
                .expect("Attribute not found in resolved child attributes.");

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

    fn expand_restriction(
        ctx: &mut XmlnsContextTransformerContext<'_>,
        child_fragment_idx: &FragmentIdx<RestrictionFragment>,
    ) -> Result<TransformChange, <Self as XmlnsContextTransformer>::Error> {
        let child_fragment = ctx
            .get_complex_fragment(child_fragment_idx)
            .expect("Fragment not found in compiler.");

        let base = child_fragment.base.clone();

        if base == *xsn::ANY_TYPE {
            return Ok(TransformChange::Unchanged);
        }

        let base_fragment = ctx
            .get_named_type(&child_fragment_idx.namespace_idx(), &base)
            .ok_or_else(|| Error::BaseDoesNotExist { base: base.clone() })?;

        let base_fragment = match base_fragment {
            TopLevelTypeId::ComplexType(complex) => complex,
            TopLevelTypeId::SimpleType(_) => {
                return Err(Error::BaseCannotBeSimpleType { base: base.clone() });
            }
        };

        let base_root_fragment = ctx
            .get_complex_fragment::<ComplexTypeRootFragment>(&base_fragment)
            .expect("Fragment not found in compiler.");

        let base_complex_content_id = match base_root_fragment.content {
            ComplexTypeModelId::ComplexContent(base_complex_content_id) => base_complex_content_id,
            ComplexTypeModelId::SimpleContent(_) => {
                return Err(Error::BaseContentCannotBeSimpleContent { base: base.clone() })
            }
            ComplexTypeModelId::Other { .. } => {
                return Err(Error::BaseContentCannotBeStandalone { base: base.clone() })
            }
        };

        let base_content_fragment = ctx
            .get_complex_fragment(&base_complex_content_id)
            .expect("Fragment not found in compiler.");

        let base_restriction_id = match base_content_fragment.content_fragment {
            ComplexContentChildId::Restriction(base_restriction_id) => base_restriction_id,
            ComplexContentChildId::Extension(_) => {
                return Err(Error::BaseCannotBeExtensionType { base: base.clone() });
            }
        };

        let base = ctx
            .get_complex_fragment(&base_restriction_id)
            .expect("Fragment not found in compiler.")
            .clone();

        let child = ctx
            .get_complex_fragment(child_fragment_idx)
            .expect("Fragment not found in compiler.");

        let new_attribute_declarations = Self::expand_restricted_attributes(
            ctx,
            child.attribute_declarations,
            base.attribute_declarations,
        )?;

        let child_restriction = ctx
            .get_complex_fragment_mut(child_fragment_idx)
            .expect("Fragment not found in compiler.");
        child_restriction.base = base.base.clone();
        child_restriction.attribute_declarations = new_attribute_declarations;

        Ok(TransformChange::Changed)
    }
}

impl XmlnsContextTransformer for ExpandRestrictionFragments {
    type Error = Error;

    fn transform(
        self,
        mut ctx: XmlnsContextTransformerContext<'_>,
    ) -> Result<TransformChange, Self::Error> {
        ctx.iter_complex_fragment_ids()
            .collect::<Vec<_>>()
            .into_iter()
            .map(|f| Self::expand_restriction(&mut ctx, &f))
            .collect()
    }
}
