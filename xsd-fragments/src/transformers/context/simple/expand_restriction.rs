use std::collections::HashSet;

use xmlity::ExpandedName;

use crate::fragments::{
    complex::TopLevelTypeId,
    simple::{RestrictionFragment, SimpleDerivation, SimpleTypeRootFragment},
    FragmentIdx,
};
use crate::transformers::context::{XmlnsContextTransformer, XmlnsContextTransformerContext};
use crate::transformers::TransformChange;

/// Transformer for expanding simple type restrictions.
pub struct ExpandSimpleRestriction<'a> {
    allowed_bases: &'a HashSet<ExpandedName<'static>>,
}

/// Error types for simple restriction expansion.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Base type not found.
    #[error("Base {base} not found in the context")]
    BaseNotFound {
        /// The base type name that was not found.
        base: ExpandedName<'static>,
    },
    /// Base type is not a simple type.
    #[error("Base {base} is not a simple type")]
    BaseNotSimpleType {
        /// The base type name that is not simple.
        base: ExpandedName<'static>,
    },
}

impl<'a> ExpandSimpleRestriction<'a> {
    /// Creates a new [`ExpandSimpleRestriction`] transformer.
    pub fn new(allowed_bases: &'a HashSet<ExpandedName<'static>>) -> Self {
        Self { allowed_bases }
    }

    fn flatten_restriction(
        &self,
        ctx: &mut XmlnsContextTransformerContext,
        fragment_idx: &FragmentIdx<SimpleTypeRootFragment>,
    ) -> Result<TransformChange, <Self as XmlnsContextTransformer>::Error> {
        let simple_type = ctx
            .get_simple_fragment(fragment_idx)
            .expect("Fragment not found in compiler.");

        let SimpleDerivation::Restriction(restriction_fragment_idx) = simple_type.simple_derivation
        else {
            // If the simple type is not a restriction, we skip it
            return Ok(TransformChange::default());
        };

        let RestrictionFragment { base, .. } = ctx
            .get_simple_fragment(&restriction_fragment_idx)
            .expect("Fragment not found in compiler.");

        let Some(base) = base.as_ref() else {
            // If the base is not set, we skip it
            return Ok(TransformChange::default());
        };

        if self.allowed_bases.iter().any(|b| b == base) {
            // If the base is not in the allowed bases, we skip it
            return Ok(TransformChange::default());
        }

        let TopLevelTypeId::SimpleType(base_simple_type) = ctx
            .get_named_type(&fragment_idx.namespace_idx(), base)
            .ok_or(Error::BaseNotFound { base: base.clone() })?
        else {
            return Err(Error::BaseNotSimpleType { base: base.clone() });
        };

        let base_fragment: &SimpleTypeRootFragment = ctx
            .get_simple_fragment(&base_simple_type)
            .expect("Base fragment should exist");

        match base_fragment.simple_derivation {
            SimpleDerivation::Restriction(base_restriction) => {
                let base_restriction = ctx
                    .get_simple_fragment(&base_restriction)
                    .expect("Base restriction should exist")
                    .clone();

                // We need to replace the base with the base restriction and then flatten the facets
                let fragment = ctx
                    .get_simple_fragment_mut(&restriction_fragment_idx)
                    .expect("Fragment not found in compiler.");
                fragment.base = base_restriction.base.clone();

                Ok(TransformChange::Changed)
            }
            SimpleDerivation::List(list_fragment_idx) => {
                let simple_type = ctx
                    .get_simple_fragment_mut(fragment_idx)
                    .expect("Base union should exist");

                simple_type.simple_derivation = SimpleDerivation::List(list_fragment_idx);

                Ok(TransformChange::Changed)
            }
            SimpleDerivation::Union(union_fragment_idx) => {
                // For now we simply flatten to the union
                let simple_type = ctx
                    .get_simple_fragment_mut(fragment_idx)
                    .expect("Base union should exist");

                simple_type.simple_derivation = SimpleDerivation::Union(union_fragment_idx);

                Ok(TransformChange::Changed)
            }
        }
    }
}

impl XmlnsContextTransformer for ExpandSimpleRestriction<'_> {
    type Error = Error;

    fn transform(
        self,
        mut ctx: XmlnsContextTransformerContext<'_>,
    ) -> Result<TransformChange, Self::Error> {
        ctx.iter_simple_fragment_ids()
            .collect::<Vec<_>>()
            .into_iter()
            .map(|f| self.flatten_restriction(&mut ctx, &f))
            .collect()
    }
}
