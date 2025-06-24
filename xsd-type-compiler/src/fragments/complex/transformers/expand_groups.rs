use std::{collections::VecDeque, convert::Infallible};

use crate::{
    fragments::complex::{
        AllFragment, ChoiceFragment, ComplexTypeFragmentCompiler, ComplexTypeModelId,
        ComplexTypeRootFragment, ExtensionFragment, FragmentAccess, FragmentIdx, NestedParticleId,
        RestrictionFragment, SequenceFragment, TypeDefParticleId,
    },
    transformers::{TransformChange, XmlnsLocalTransformer, XmlnsLocalTransformerContext},
};

trait HasGroupContent {
    fn group_content(&self) -> &VecDeque<NestedParticleId>;

    fn group_content_mut(&mut self) -> &mut VecDeque<NestedParticleId>;
}

impl HasGroupContent for AllFragment {
    fn group_content(&self) -> &VecDeque<NestedParticleId> {
        &self.fragments
    }

    fn group_content_mut(&mut self) -> &mut VecDeque<NestedParticleId> {
        &mut self.fragments
    }
}
impl HasGroupContent for SequenceFragment {
    fn group_content(&self) -> &VecDeque<NestedParticleId> {
        &self.fragments
    }

    fn group_content_mut(&mut self) -> &mut VecDeque<NestedParticleId> {
        &mut self.fragments
    }
}

impl HasGroupContent for ChoiceFragment {
    fn group_content(&self) -> &VecDeque<NestedParticleId> {
        &self.fragments
    }

    fn group_content_mut(&mut self) -> &mut VecDeque<NestedParticleId> {
        &mut self.fragments
    }
}

trait HasTypeDefParticle {
    fn type_def_particle(&self) -> Option<&TypeDefParticleId>;

    fn type_def_particle_mut(&mut self) -> Option<&mut TypeDefParticleId>;
}

impl HasTypeDefParticle for ExtensionFragment {
    fn type_def_particle(&self) -> Option<&TypeDefParticleId> {
        self.content_fragment.as_ref()
    }

    fn type_def_particle_mut(&mut self) -> Option<&mut TypeDefParticleId> {
        self.content_fragment.as_mut()
    }
}

impl HasTypeDefParticle for RestrictionFragment {
    fn type_def_particle(&self) -> Option<&TypeDefParticleId> {
        self.content_fragment.as_ref()
    }

    fn type_def_particle_mut(&mut self) -> Option<&mut TypeDefParticleId> {
        self.content_fragment.as_mut()
    }
}

impl HasTypeDefParticle for ComplexTypeRootFragment {
    fn type_def_particle(&self) -> Option<&TypeDefParticleId> {
        match &self.content {
            ComplexTypeModelId::Other { particle, .. } => particle.as_ref(),
            _ => None,
        }
    }

    fn type_def_particle_mut(&mut self) -> Option<&mut TypeDefParticleId> {
        match &mut self.content {
            ComplexTypeModelId::Other { particle, .. } => particle.as_mut(),
            _ => None,
        }
    }
}

#[non_exhaustive]
pub struct ExpandGroups {}

impl ExpandGroups {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {}
    }

    pub fn expand_group_content(
        _context: &mut XmlnsLocalTransformerContext<'_>,
        _group_contents: NestedParticleId,
    ) -> NestedParticleId {
        todo!()
    }

    pub fn expand_type_def_particle(
        _context: &mut XmlnsLocalTransformerContext<'_>,
        _group_contents: TypeDefParticleId,
    ) -> TypeDefParticleId {
        todo!()
    }

    fn expand_fragment_with_group_content<F: HasGroupContent>(
        context: &mut XmlnsLocalTransformerContext<'_>,
        fragment_id: &FragmentIdx<F>,
    ) -> Result<TransformChange, <Self as XmlnsLocalTransformer>::Error>
    where
        ComplexTypeFragmentCompiler: FragmentAccess<F>,
    {
        let mut change = TransformChange::default();

        let fragment = context.get_complex_fragment(fragment_id).unwrap();

        let unexpanded_fragments = fragment.group_content().clone();

        let expanded_fragments = unexpanded_fragments
            .into_iter()
            .map(|f| {
                let res = Self::expand_group_content(context, f);

                change |= TransformChange::from(res != f);

                res
            })
            .collect();

        let fragment = context.get_complex_fragment_mut(fragment_id).unwrap();

        *fragment.group_content_mut() = expanded_fragments;

        Ok(change)
    }

    fn expand_fragments_group_content<F: HasGroupContent + 'static>(
        ctx: &mut XmlnsLocalTransformerContext<'_>,
    ) -> Result<TransformChange, <Self as XmlnsLocalTransformer>::Error>
    where
        ComplexTypeFragmentCompiler: FragmentAccess<F>,
    {
        ctx.iter_complex_fragment_ids()
            .into_iter()
            .map(|f| Self::expand_fragment_with_group_content(ctx, &f))
            .collect()
    }

    fn expand_fragment_with_type_def_particle<F: HasTypeDefParticle>(
        context: &mut XmlnsLocalTransformerContext<'_>,
        fragment_id: &FragmentIdx<F>,
    ) -> Result<TransformChange, <Self as XmlnsLocalTransformer>::Error>
    where
        ComplexTypeFragmentCompiler: FragmentAccess<F>,
    {
        let fragment = context.get_complex_fragment(fragment_id).unwrap();

        let Some(unexpanded_fragment) = fragment.type_def_particle().copied() else {
            return Ok(TransformChange::Unchanged);
        };

        let expanded_fragment = Self::expand_type_def_particle(context, unexpanded_fragment);

        let fragment = context.get_complex_fragment_mut(fragment_id).unwrap();

        *fragment.type_def_particle_mut().expect("Already checked") = expanded_fragment;

        Ok(TransformChange::from(
            expanded_fragment != unexpanded_fragment,
        ))
    }

    fn expand_fragments_type_def_particle<F: HasTypeDefParticle + 'static>(
        ctx: &mut XmlnsLocalTransformerContext<'_>,
    ) -> Result<TransformChange, <Self as XmlnsLocalTransformer>::Error>
    where
        ComplexTypeFragmentCompiler: FragmentAccess<F>,
    {
        ctx.iter_complex_fragment_ids()
            .iter()
            .map(|f| Self::expand_fragment_with_type_def_particle(ctx, f))
            .collect()
    }
}

impl XmlnsLocalTransformer for ExpandGroups {
    type Error = Infallible;

    fn transform(
        self,
        mut ctx: XmlnsLocalTransformerContext<'_>,
    ) -> Result<TransformChange, Self::Error> {
        let mut changed = TransformChange::default();

        changed |= Self::expand_fragments_group_content::<AllFragment>(&mut ctx)?;

        changed |= Self::expand_fragments_group_content::<SequenceFragment>(&mut ctx)?;

        changed |= Self::expand_fragments_group_content::<ChoiceFragment>(&mut ctx)?;

        changed |= Self::expand_fragments_type_def_particle::<ExtensionFragment>(&mut ctx)?;

        changed |= Self::expand_fragments_type_def_particle::<RestrictionFragment>(&mut ctx)?;

        changed |= Self::expand_fragments_type_def_particle::<ComplexTypeRootFragment>(&mut ctx)?;

        Ok(changed)
    }
}
