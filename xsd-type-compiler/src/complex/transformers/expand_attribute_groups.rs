use std::collections::VecDeque;

use crate::{
    complex::{
        AttributeDeclarationId, ComplexTypeFragmentCompiler, ComplexTypeModelId,
        ComplexTypeRootFragment, ExtensionFragment, FragmentAccess, FragmentIdx,
        RestrictionFragment,
    },
    transformers::{Context, TransformChange, XmlnsContextTransformer},
};

trait HasAttributeDeclarations {
    fn attribute_declarations(&self) -> Option<&VecDeque<AttributeDeclarationId>>;

    fn attribute_declarations_mut(&mut self) -> Option<&mut VecDeque<AttributeDeclarationId>>;
}

impl HasAttributeDeclarations for ExtensionFragment {
    fn attribute_declarations(&self) -> Option<&VecDeque<AttributeDeclarationId>> {
        Some(&self.attribute_declarations)
    }

    fn attribute_declarations_mut(&mut self) -> Option<&mut VecDeque<AttributeDeclarationId>> {
        Some(&mut self.attribute_declarations)
    }
}
impl HasAttributeDeclarations for RestrictionFragment {
    fn attribute_declarations(&self) -> Option<&VecDeque<AttributeDeclarationId>> {
        Some(&self.attribute_declarations)
    }

    fn attribute_declarations_mut(&mut self) -> Option<&mut VecDeque<AttributeDeclarationId>> {
        Some(&mut self.attribute_declarations)
    }
}
impl HasAttributeDeclarations for ComplexTypeRootFragment {
    fn attribute_declarations(&self) -> Option<&VecDeque<AttributeDeclarationId>> {
        match &self.content {
            ComplexTypeModelId::Other { attributes, .. } => Some(attributes),
            _ => None,
        }
    }

    fn attribute_declarations_mut(&mut self) -> Option<&mut VecDeque<AttributeDeclarationId>> {
        match &mut self.content {
            ComplexTypeModelId::Other { attributes, .. } => Some(attributes),
            _ => None,
        }
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

        let Some(unexpanded_fragments) = fragment.attribute_declarations().clone() else {
            return Ok(change);
        };

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

            // todo!()
            //TODO
        }

        let fragment = context.get_complex_fragment_mut(fragment_id).unwrap();

        *fragment
            .attribute_declarations_mut()
            .expect("If it was none, it should've returned earlier.") =
            attributes.into_iter().map(Into::into).collect();

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

        changed |= Self::expand_fragments_with_attribute_declarations::<ComplexTypeRootFragment>(
            &mut context,
        )?;

        Ok(changed)
    }
}
