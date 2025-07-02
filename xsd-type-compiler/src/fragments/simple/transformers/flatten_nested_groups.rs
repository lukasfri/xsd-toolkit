use std::collections::VecDeque;
use std::convert::Infallible;

use crate::{
    fragments::{
        simple::{SimpleDerivation, UnionFragment},
        FragmentIdx,
    },
    transformers::{TransformChange, XmlnsLocalTransformer, XmlnsLocalTransformerContext},
};

pub struct FlattenNestedUnions;

impl FlattenNestedUnions {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {}
    }

    fn flatten_union(
        ctx: &mut XmlnsLocalTransformerContext,
        fragment_idx: &FragmentIdx<UnionFragment>,
    ) -> Result<TransformChange, <Self as XmlnsLocalTransformer>::Error> {
        let UnionFragment {
            member_types,
            simple_types,
            ..
        } = ctx.get_simple_fragment(fragment_idx).unwrap();

        let mut flattened = TransformChange::default();

        let mut new_member_types = member_types.clone();
        let mut new_simple_types = VecDeque::new();
        for fragment_id in simple_types {
            let simple_type = ctx.get_simple_fragment(fragment_id).unwrap();

            let SimpleDerivation::Union(union_fragment_id) = simple_type.simple_derivation else {
                new_simple_types.push_back(fragment_id.clone());
                continue;
            };

            let UnionFragment {
                member_types: sub_member_types,
                simple_types: sub_simple_types,
            } = ctx.get_simple_fragment(&union_fragment_id).unwrap();

            new_member_types.extend(sub_member_types.iter().cloned());
            new_simple_types.extend(sub_simple_types.iter().cloned());
            flattened = TransformChange::Changed;
        }

        let fragment = ctx.get_simple_fragment_mut(fragment_idx).unwrap();
        fragment.simple_types = new_simple_types;

        Ok(flattened)
    }
}

impl XmlnsLocalTransformer for FlattenNestedUnions {
    type Error = Infallible;

    fn transform(
        self,
        mut ctx: XmlnsLocalTransformerContext<'_>,
    ) -> Result<TransformChange, Self::Error> {
        ctx.iter_simple_fragment_ids()
            .into_iter()
            .map(|f| Self::flatten_union(&mut ctx, &f))
            .collect()
    }
}
