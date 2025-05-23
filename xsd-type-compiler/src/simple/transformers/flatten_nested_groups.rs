use std::collections::VecDeque;

use crate::simple::SimpleTypeFragment;
use crate::transformers::{Context, TransformChange, XmlnsContextTransformer};

pub struct FlattenNestedUnions;

impl XmlnsContextTransformer for FlattenNestedUnions {
    type Error = ();

    fn transform(self, mut ctx: Context<'_>) -> Result<TransformChange, Self::Error> {
        for fragment_idx in ctx.iter_simple_fragment_ids() {
            let fragment = ctx.get_simple_fragment(&fragment_idx).unwrap();
            let SimpleTypeFragment::Union { fragments } = fragment else {
                continue;
            };

            let mut new_fragments = VecDeque::new();

            for fragment_id in fragments {
                let fragment: &SimpleTypeFragment = ctx.get_simple_fragment(fragment_id).unwrap();
                let SimpleTypeFragment::Union {
                    fragments: sub_fragments,
                } = fragment
                //TODO: Review when to NOT flatten choices
                else {
                    new_fragments.push_back(fragment_id.clone());
                    continue;
                };
                new_fragments.extend(sub_fragments.iter().cloned());
            }

            let fragment = ctx.simple_fragment_mut(&fragment_idx).unwrap();
            let SimpleTypeFragment::Union { fragments } = fragment else {
                unreachable!()
            };
            *fragments = new_fragments;
        }

        Ok(todo!())
    }
}
