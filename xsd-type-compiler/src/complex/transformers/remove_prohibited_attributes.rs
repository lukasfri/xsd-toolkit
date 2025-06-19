use std::convert::Infallible;

use crate::{
    complex::{AttributeDeclarationId, AttributeDeclarationsFragment, AttributeUse, FragmentIdx},
    transformers::{TransformChange, XmlnsLocalTransformer, XmlnsLocalTransformerContext},
};

#[non_exhaustive]
pub struct RemoveProhibitedAttributes {}

impl RemoveProhibitedAttributes {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {}
    }

    fn expand_attribute_declarations(
        context: &mut XmlnsLocalTransformerContext<'_>,
        fragment_id: &FragmentIdx<AttributeDeclarationsFragment>,
    ) -> Result<TransformChange, <Self as XmlnsLocalTransformer>::Error> {
        let mut change = TransformChange::default();

        let fragment = context.get_complex_fragment(fragment_id).unwrap();

        let unfiltered_fragments = fragment.declarations.clone();

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

        fragment.declarations = filtered_fragments.into_iter().collect();

        Ok(change)
    }
}

impl XmlnsLocalTransformer for RemoveProhibitedAttributes {
    type Error = Infallible;

    fn transform(
        self,
        mut context: XmlnsLocalTransformerContext<'_>,
    ) -> Result<TransformChange, Self::Error> {
        context
            .iter_complex_fragment_ids::<AttributeDeclarationsFragment>()
            .iter()
            .map(|fragment_id| Self::expand_attribute_declarations(&mut context, fragment_id))
            .collect()
    }
}
