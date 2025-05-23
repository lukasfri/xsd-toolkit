use std::collections::VecDeque;

use crate::{
    complex::{
        AllFragment, ChoiceFragment, ComplexTypeFragmentCompiler, ComplexTypeModelId,
        ComplexTypeRootFragment, ExtensionFragment, FragmentAccess, FragmentIdx,
        GroupTypeContentId, RestrictionFragment, SequenceFragment, TypeDefParticleId,
    },
    transformers::{Context, TransformChange, XmlnsContextTransformer},
};

trait HasGroupContent {
    fn group_content(&self) -> &VecDeque<GroupTypeContentId>;

    fn group_content_mut(&mut self) -> &mut VecDeque<GroupTypeContentId>;
}

impl HasGroupContent for AllFragment {
    fn group_content(&self) -> &VecDeque<GroupTypeContentId> {
        &self.fragments
    }

    fn group_content_mut(&mut self) -> &mut VecDeque<GroupTypeContentId> {
        &mut self.fragments
    }
}
impl HasGroupContent for SequenceFragment {
    fn group_content(&self) -> &VecDeque<GroupTypeContentId> {
        &self.fragments
    }

    fn group_content_mut(&mut self) -> &mut VecDeque<GroupTypeContentId> {
        &mut self.fragments
    }
}

impl HasGroupContent for ChoiceFragment {
    fn group_content(&self) -> &VecDeque<GroupTypeContentId> {
        &self.fragments
    }

    fn group_content_mut(&mut self) -> &mut VecDeque<GroupTypeContentId> {
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
            ComplexTypeModelId::Other { particle } => Some(particle),
            _ => None,
        }
    }

    fn type_def_particle_mut(&mut self) -> Option<&mut TypeDefParticleId> {
        match &mut self.content {
            ComplexTypeModelId::Other { particle } => Some(particle),
            _ => None,
        }
    }
}

#[non_exhaustive]
pub struct ExpandGroups {}

impl ExpandGroups {
    pub fn expand_group_content<'a>(
        ctx: &mut Context<'_>,
        group_contents: GroupTypeContentId,
    ) -> GroupTypeContentId {
        todo!()
    }

    pub fn expand_type_def_particle<'a>(
        ctx: &mut Context<'_>,
        group_contents: TypeDefParticleId,
    ) -> TypeDefParticleId {
        todo!()
    }

    fn expand_fragment_with_group_content<F: HasGroupContent>(
        context: &mut Context<'_>,
        fragment_id: &FragmentIdx<F>,
    ) -> Result<TransformChange, ()>
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
        ctx: &mut Context<'_>,
    ) -> Result<TransformChange, ()>
    where
        ComplexTypeFragmentCompiler: FragmentAccess<F>,
    {
        ctx.iter_complex_fragment_ids()
            .into_iter()
            .map(|f| Self::expand_fragment_with_group_content(ctx, &f))
            .collect()
    }

    fn expand_fragment_with_type_def_particle<F: HasTypeDefParticle>(
        context: &mut Context<'_>,
        fragment_id: &FragmentIdx<F>,
    ) -> Result<TransformChange, ()>
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
        ctx: &mut Context<'_>,
    ) -> Result<TransformChange, ()>
    where
        ComplexTypeFragmentCompiler: FragmentAccess<F>,
    {
        ctx.iter_complex_fragment_ids()
            .into_iter()
            .map(|f| Self::expand_fragment_with_type_def_particle(ctx, &f))
            .collect()
    }
}

impl XmlnsContextTransformer for ExpandGroups {
    type Error = ();

    fn transform(self, mut ctx: Context<'_>) -> Result<TransformChange, Self::Error> {
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

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use xmlity::{ExpandedName, LocalName, XmlNamespace};
    use xsd::schema::{
        Base, ComplexContent, ComplexRestrictionType, ComplexTypeModel, LocalElement, Name, QName,
        SequenceType, TopLevelComplexType, Type, TypeDefParticle,
    };

    use crate::{
        complex::{
            transformers::expand_short_form_complex_types::ExpandShortFormComplexTypes,
            ANY_TYPE_EXPANDED_NAME,
        },
        transformers::{Context, XmlnsContextTransformer},
        CompiledNamespace, XmlnsContext,
    };

    #[test]
    fn specification_1() {}
}
