use std::collections::VecDeque;

use crate::{
    complex::{
        AttributeDeclarationId, AttributeUse, ComplexTypeFragmentCompiler, ExtensionFragment,
        FragmentAccess, FragmentIdx, RestrictionFragment,
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
pub struct RemoveProhibitedAttributes {}

impl RemoveProhibitedAttributes {
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

        let unfiltered_fragments = fragment.attribute_declarations().clone();

        let filtered_fragments = unfiltered_fragments
            .into_iter()
            .filter_map(|a| match a {
                AttributeDeclarationId::Attribute(fragment_idx) => {
                    let fragment = context
                        .get_complex_fragment::<crate::complex::LocalAttributeFragment>(
                            &fragment_idx,
                        )
                        .unwrap();

                    if fragment.use_ != Some(AttributeUse::Prohibited) {
                        Some(a)
                    } else {
                        change |= TransformChange::Changed;
                        None
                    }
                }
                a => Some(a),
            })
            .collect::<Vec<_>>();

        let fragment = context.get_complex_fragment_mut(fragment_id).unwrap();

        *fragment.attribute_declarations_mut() =
            filtered_fragments.into_iter().map(Into::into).collect();

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

impl XmlnsContextTransformer for RemoveProhibitedAttributes {
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
