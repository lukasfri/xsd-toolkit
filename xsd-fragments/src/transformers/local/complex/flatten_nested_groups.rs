use std::collections::VecDeque;

use crate::fragments::{
    complex::{ChoiceFragment, NestedParticleId, SequenceFragment},
    FragmentIdx,
};

use crate::transformers::{TransformChange, XmlnsLocalTransformer, XmlnsLocalTransformerContext};

/// Transformer for flattening nested sequences.
#[non_exhaustive]
pub struct FlattenNestedSequences {}

/// Error type for [`FlattenNestedSequences`] operations.
#[derive(Debug, thiserror::Error)]
pub enum FlattenNestedSequencesError {}

impl FlattenNestedSequences {
    /// Creates a new [`FlattenNestedSequences`] transformer.
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {}
    }

    fn flatten_sequence(
        ctx: &mut XmlnsLocalTransformerContext,
        fragment_idx: &FragmentIdx<SequenceFragment>,
    ) -> Result<TransformChange, <Self as XmlnsLocalTransformer>::Error> {
        let SequenceFragment { fragments, .. } = ctx
            .get_complex_fragment(fragment_idx)
            .expect("Fragment not found in compiler.");

        let mut flattened = TransformChange::default();

        let mut new_fragments = VecDeque::new();
        for fragment_id in fragments {
            let NestedParticleId::Sequence(seq_fragment_id) = fragment_id else {
                new_fragments.push_back(*fragment_id);
                continue;
            };

            let SequenceFragment {
                id: _,
                fragments: sub_fragments,
                max_occurs,
                min_occurs,
            } = ctx
                .get_complex_fragment(seq_fragment_id)
                .expect("Fragment not found in compiler.");

            if max_occurs.is_some() || min_occurs.is_some() {
                new_fragments.push_back(*fragment_id);
                continue;
            }

            new_fragments.extend(sub_fragments.iter().cloned());
            flattened = TransformChange::Changed;
        }

        let fragment = ctx
            .get_complex_fragment_mut(fragment_idx)
            .expect("Fragment not found in compiler.");
        fragment.fragments = new_fragments;

        Ok(flattened)
    }
}

impl XmlnsLocalTransformer for &FlattenNestedSequences {
    type Error = FlattenNestedSequencesError;

    fn transform(
        self,
        mut ctx: XmlnsLocalTransformerContext<'_>,
    ) -> Result<TransformChange, Self::Error> {
        ctx.iter_complex_fragment_ids()
            .collect::<Vec<_>>()
            .into_iter()
            .map(|f| FlattenNestedSequences::flatten_sequence(&mut ctx, &f))
            .collect()
    }
}

impl XmlnsLocalTransformer for FlattenNestedSequences {
    type Error = FlattenNestedSequencesError;

    fn transform(
        self,
        ctx: XmlnsLocalTransformerContext<'_>,
    ) -> Result<TransformChange, Self::Error> {
        (&self).transform(ctx)
    }
}

/// Error type for [`FlattenNestedChoices`] operations.
#[derive(Debug, thiserror::Error)]
pub enum FlattenNestedChoicesError {}

/// Transformer for flattening nested choices.
#[non_exhaustive]
pub struct FlattenNestedChoices {}

impl FlattenNestedChoices {
    /// Creates a new [`FlattenNestedChoices`] transformer.
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {}
    }

    fn flatten_choice(
        ctx: &mut XmlnsLocalTransformerContext,
        fragment_idx: &FragmentIdx<ChoiceFragment>,
    ) -> Result<TransformChange, <Self as XmlnsLocalTransformer>::Error> {
        let ChoiceFragment { fragments, .. } = ctx
            .get_complex_fragment(fragment_idx)
            .expect("Fragment not found in compiler.");

        let mut flattened = TransformChange::default();

        let mut new_fragments = VecDeque::new();
        for fragment_id in fragments {
            let NestedParticleId::Choice(choice_fragment_id) = fragment_id else {
                new_fragments.push_back(*fragment_id);
                continue;
            };

            let ChoiceFragment {
                fragments: sub_fragments,
                max_occurs,
                min_occurs,
            } = ctx
                .get_complex_fragment(choice_fragment_id)
                .expect("Fragment not found in compiler.");

            if max_occurs.is_some() || min_occurs.is_some() {
                new_fragments.push_back(*fragment_id);
                continue;
            }

            new_fragments.extend(sub_fragments.iter().cloned());
            flattened = TransformChange::Changed;
        }

        let fragment = ctx
            .get_complex_fragment_mut(fragment_idx)
            .expect("Fragment not found in compiler.");
        fragment.fragments = new_fragments;

        Ok(flattened)
    }
}

impl XmlnsLocalTransformer for &FlattenNestedChoices {
    type Error = FlattenNestedChoicesError;

    fn transform(
        self,
        mut ctx: XmlnsLocalTransformerContext<'_>,
    ) -> Result<TransformChange, Self::Error> {
        ctx.iter_complex_fragment_ids()
            .collect::<Vec<_>>()
            .into_iter()
            .map(|f| FlattenNestedChoices::flatten_choice(&mut ctx, &f))
            .collect()
    }
}

impl XmlnsLocalTransformer for FlattenNestedChoices {
    type Error = FlattenNestedChoicesError;

    fn transform(
        self,
        ctx: XmlnsLocalTransformerContext<'_>,
    ) -> Result<TransformChange, Self::Error> {
        (&self).transform(ctx)
    }
}
