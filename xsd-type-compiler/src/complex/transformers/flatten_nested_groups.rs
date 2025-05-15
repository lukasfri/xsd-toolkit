use std::collections::VecDeque;

use crate::complex::{
    AllTypeFragment, ChoiceTypeFragment, ComplexTypeFragment, SequenceTypeFragment,
};

use crate::transformers::{Context, XmlnsContextTransformer};

pub struct FlattenNestedSequences;

impl XmlnsContextTransformer for FlattenNestedSequences {
    fn transform(self, mut ctx: Context<'_>) {
        for fragment_idx in ctx.iter_complex_fragment_ids() {
            let fragment = ctx.get_complex_fragment(&fragment_idx).unwrap();
            let ComplexTypeFragment::Sequence(SequenceTypeFragment { fragments }) = fragment else {
                continue;
            };

            let mut new_fragments = VecDeque::new();

            for fragment_id in fragments {
                let fragment: &ComplexTypeFragment = ctx.get_complex_fragment(fragment_id).unwrap();
                let ComplexTypeFragment::Sequence(SequenceTypeFragment {
                    fragments: sub_fragments,
                }) = fragment
                //TODO: Review when to NOT flatten choices
                else {
                    new_fragments.push_back(fragment_id.clone());
                    continue;
                };
                new_fragments.extend(sub_fragments.iter().cloned());
            }

            let fragment = ctx.get_complex_fragment_mut(&fragment_idx).unwrap();
            let ComplexTypeFragment::Sequence(SequenceTypeFragment { fragments }) = fragment else {
                unreachable!()
            };
            *fragments = new_fragments;
        }
    }
}

pub struct FlattenNestedChoices;

impl XmlnsContextTransformer for FlattenNestedChoices {
    fn transform(self, mut ctx: Context<'_>) {
        for fragment_idx in ctx.iter_complex_fragment_ids() {
            let fragment = ctx.get_complex_fragment(&fragment_idx).unwrap();
            let ComplexTypeFragment::Choice(ChoiceTypeFragment { fragments }) = fragment else {
                continue;
            };
            let mut new_fragments = VecDeque::new();
            for fragment_id in fragments {
                let fragment: &ComplexTypeFragment = ctx.get_complex_fragment(fragment_id).unwrap();
                let ComplexTypeFragment::Choice(ChoiceTypeFragment {
                    fragments: sub_fragments,
                }) = fragment
                //TODO: Review when to NOT flatten choices
                else {
                    new_fragments.push_back(fragment_id.clone());
                    continue;
                };
                new_fragments.extend(sub_fragments.iter().cloned());
            }

            let fragment = ctx.get_complex_fragment_mut(&fragment_idx).unwrap();
            let ComplexTypeFragment::Choice(ChoiceTypeFragment { fragments }) = fragment else {
                unreachable!()
            };
            *fragments = new_fragments;
        }
    }
}

pub struct FlattenNestedAll;

impl XmlnsContextTransformer for FlattenNestedAll {
    fn transform(self, mut ctx: Context<'_>) {
        for fragment_idx in ctx.iter_complex_fragment_ids() {
            let fragment = ctx.get_complex_fragment(&fragment_idx).unwrap();
            let ComplexTypeFragment::All(AllTypeFragment { fragments }) = fragment else {
                continue;
            };
            let mut new_fragments = VecDeque::new();
            for fragment_id in fragments {
                let fragment: &ComplexTypeFragment = ctx.get_complex_fragment(fragment_id).unwrap();
                let ComplexTypeFragment::All(AllTypeFragment {
                    fragments: sub_fragments,
                }) = fragment
                //TODO: Review when to NOT flatten choices
                else {
                    new_fragments.push_back(fragment_id.clone());
                    continue;
                };
                new_fragments.extend(sub_fragments.iter().cloned());
            }

            let fragment = ctx.get_complex_fragment_mut(&fragment_idx).unwrap();
            let ComplexTypeFragment::All(AllTypeFragment { fragments }) = fragment else {
                unreachable!()
            };
            *fragments = new_fragments;
        }
    }
}
