use std::collections::VecDeque;

use crate::{
    complex::{
        AttributeDeclarationId, ComplexTypeFragmentCompiler, ExtensionFragment, FragmentAccess,
        FragmentIdx, RestrictionFragment,
    },
    transformers::{Context, TransformChange, XmlnsContextTransformer},
};

trait HasAttributeDeclarations {
    fn attribute_declarations(&self) -> &VecDeque<AttributeDeclarationId>;

    fn attribute_declarations_mut(&mut self) -> &mut VecDeque<AttributeDeclarationId>;
}

impl HasAttributeDeclarations for ExtensionFragment {
    fn attribute_declarations(&self) -> &VecDeque<AttributeDeclarationId> {
        &self.attribute_declarations
    }

    fn attribute_declarations_mut(&mut self) -> &mut VecDeque<AttributeDeclarationId> {
        &mut self.attribute_declarations
    }
}
impl HasAttributeDeclarations for RestrictionFragment {
    fn attribute_declarations(&self) -> &VecDeque<AttributeDeclarationId> {
        &self.attribute_declarations
    }

    fn attribute_declarations_mut(&mut self) -> &mut VecDeque<AttributeDeclarationId> {
        &mut self.attribute_declarations
    }
}

#[non_exhaustive]
pub struct ExpandAttributeGroups {}

impl ExpandAttributeGroups {
    pub fn new() -> Self {
        Self {}
    }

    fn expand_fragment_with_attribute_declarations<F: HasAttributeDeclarations>(
        context: &mut Context<'_>,
        fragment_id: &FragmentIdx<F>,
    ) -> Result<TransformChange, ()>
    where
        ComplexTypeFragmentCompiler: FragmentAccess<F>,
    {
        let mut change = TransformChange::default();

        let fragment = context.get_complex_fragment(fragment_id).unwrap();

        let unexpanded_fragments = fragment.attribute_declarations().clone();

        let (attributes, attribute_groups) = unexpanded_fragments
            .iter()
            .map(|a| match a {
                AttributeDeclarationId::Attribute(fragment_idx) => (Some(*fragment_idx), None),
                AttributeDeclarationId::AttributeGroupRef(fragment_idx) => {
                    (None, Some(*fragment_idx))
                }
            })
            .unzip::<_, _, Vec<_>, Vec<_>>();

        let attributes: Vec<_> = attributes.into_iter().flatten().collect();

        for _attribute_group in attribute_groups.into_iter().flatten() {
            change |= TransformChange::Changed;

            todo!()
        }

        let fragment = context.get_complex_fragment_mut(fragment_id).unwrap();

        *fragment.attribute_declarations_mut() = attributes.into_iter().map(Into::into).collect();

        Ok(change)
    }

    fn expand_fragments_with_attribute_declarations<F: HasAttributeDeclarations + 'static>(
        context: &mut Context<'_>,
    ) -> Result<TransformChange, ()>
    where
        ComplexTypeFragmentCompiler: FragmentAccess<F>,
    {
        context
            .iter_complex_fragment_ids::<F>()
            .iter()
            .map(|fragment_id| {
                Self::expand_fragment_with_attribute_declarations(context, fragment_id)
            })
            .collect()
    }
}

impl XmlnsContextTransformer for ExpandAttributeGroups {
    type Error = ();

    fn transform(self, mut context: Context<'_>) -> Result<TransformChange, Self::Error> {
        let mut changed = TransformChange::default();

        changed |=
            Self::expand_fragments_with_attribute_declarations::<ExtensionFragment>(&mut context)?;

        changed |= Self::expand_fragments_with_attribute_declarations::<RestrictionFragment>(
            &mut context,
        )?;

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
