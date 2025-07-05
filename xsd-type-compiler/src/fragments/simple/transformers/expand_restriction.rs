use std::collections::HashSet;

use xmlity::ExpandedName;

use crate::{
    fragments::{
        simple::{RestrictionFragment, SimpleDerivation, SimpleTypeRootFragment},
        FragmentIdx,
    },
    transformers::{TransformChange, XmlnsLocalTransformer, XmlnsLocalTransformerContext},
};

pub struct ExpandSimpleRestriction<'a> {
    allowed_bases: &'a HashSet<ExpandedName<'static>>,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Base {base} not found in the context")]
    BaseNotFound { base: ExpandedName<'static> },
    #[error("Base {base} is not a simple type")]
    BaseNotSimpleType { base: ExpandedName<'static> },
    #[error("Base {base} is not a restriction")]
    BaseNotRestriction { base: ExpandedName<'static> },
}

impl<'a> ExpandSimpleRestriction<'a> {
    pub fn new(allowed_bases: &'a HashSet<ExpandedName<'static>>) -> Self {
        Self { allowed_bases }
    }

    fn flatten_restriction(
        &self,
        ctx: &mut XmlnsLocalTransformerContext,
        fragment_idx: &FragmentIdx<RestrictionFragment>,
    ) -> Result<TransformChange, <Self as XmlnsLocalTransformer>::Error> {
        let RestrictionFragment { base, .. } = ctx.get_simple_fragment(fragment_idx).unwrap();

        let Some(base) = base.as_ref() else {
            // If the base is not set, we skip it
            return Ok(TransformChange::default());
        };

        if self.allowed_bases.iter().any(|b| b == base) {
            // If the base is not in the allowed bases, we skip it
            return Ok(TransformChange::default());
        }

        let crate::TopLevelType::Simple(simple_type) = ctx
            .get_named_type(base)
            .ok_or(Error::BaseNotFound { base: base.clone() })?
        else {
            return Err(Error::BaseNotSimpleType { base: base.clone() });
        };

        let base_fragment: &SimpleTypeRootFragment = ctx
            .get_simple_fragment(&simple_type.root_fragment)
            .expect("Base fragment should exist");

        let SimpleDerivation::Restriction(base_restriction) = &base_fragment.simple_derivation
        else {
            return Err(Error::BaseNotRestriction { base: base.clone() });
        };

        let base_restriction = ctx
            .get_simple_fragment(base_restriction)
            .expect("Base restriction should exist")
            .clone();

        // We need to replace the base with the base restriction and then flatten the facets
        let fragment = ctx.get_simple_fragment_mut(fragment_idx).unwrap();
        fragment.base = base_restriction.base.clone();

        //TODO: Handle facets

        Ok(TransformChange::Changed)
    }
}

impl XmlnsLocalTransformer for ExpandSimpleRestriction<'_> {
    type Error = Error;

    fn transform(
        self,
        mut ctx: XmlnsLocalTransformerContext<'_>,
    ) -> Result<TransformChange, Self::Error> {
        ctx.iter_simple_fragment_ids()
            .into_iter()
            .map(|f| self.flatten_restriction(&mut ctx, &f))
            .collect()
    }
}
