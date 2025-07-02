use std::convert::Infallible;

use xmlity::ExpandedName;

use crate::{
    fragments::{
        simple::{RestrictionFragment, SimpleDerivation, SimpleTypeRootFragment},
        FragmentIdx,
    },
    transformers::{TransformChange, XmlnsLocalTransformer, XmlnsLocalTransformerContext},
};

pub struct FlattenRestriction {
    allowed_bases: Vec<ExpandedName<'static>>,
}

impl FlattenRestriction {
    pub fn new(allowed_bases: Vec<ExpandedName<'static>>) -> Self {
        Self { allowed_bases }
    }

    fn flatten_restriction(
        &self,
        ctx: &mut XmlnsLocalTransformerContext,
        fragment_idx: &FragmentIdx<RestrictionFragment>,
    ) -> Result<TransformChange, <Self as XmlnsLocalTransformer>::Error> {
        let RestrictionFragment { base, .. } = ctx.get_simple_fragment(fragment_idx).unwrap();

        if self.allowed_bases.iter().any(|b| b == base) {
            // If the base is not in the allowed bases, we skip it
            return Ok(TransformChange::default());
        }

        let crate::TopLevelType::Simple(simple_type) = ctx
            .get_named_type(base)
            .expect("Base fragment should exist")
        else {
            todo!("base type {base} is not a simple type");
        };

        let base_fragment: &SimpleTypeRootFragment = ctx
            .get_simple_fragment(&simple_type.root_fragment)
            .expect("Base fragment should exist");

        let SimpleDerivation::Restriction(_) = &base_fragment.simple_derivation else {
            todo!("Base type {base} is not a restriction");
        };

        let mut _flattened = TransformChange::default();

        todo!("Flatten restriction for {base}")
    }
}

impl XmlnsLocalTransformer for FlattenRestriction {
    type Error = Infallible;

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
