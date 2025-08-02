use crate::fragments::{
    complex::{self as cx, AttributeDeclarationId, AttributeDeclarationsFragment, AttributeUse},
    FragmentIdx,
};
use crate::transformers::{
    TransformChange, XmlnsContextTransformer, XmlnsContextTransformerContext,
};

#[derive(Debug, thiserror::Error)]
pub enum Error {}

#[non_exhaustive]
pub struct RemoveProhibitedAttributes {}

impl RemoveProhibitedAttributes {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {}
    }

    fn expand_attribute_declarations(
        context: &mut XmlnsContextTransformerContext<'_>,
        fragment_id: &FragmentIdx<AttributeDeclarationsFragment>,
    ) -> Result<TransformChange, <Self as XmlnsContextTransformer>::Error> {
        let mut change = TransformChange::default();

        let fragment = context
            .get_complex_fragment(fragment_id)
            .expect("Fragment not found in compiler.");

        let unfiltered_fragments = fragment.declarations.clone();

        let filtered_fragments = unfiltered_fragments
            .into_iter()
            .filter_map(|a| match a {
                AttributeDeclarationId::Attribute(fragment_idx) => {
                    let fragment = context
                        .get_complex_fragment::<cx::LocalAttributeFragment>(&fragment_idx)
                        .expect("Fragment not found in compiler.");

                    if fragment.use_ == Some(AttributeUse::Prohibited) {
                        change |= TransformChange::Changed;
                        None
                    } else {
                        Some(a)
                    }
                }
                a => Some(a),
            })
            .collect::<Vec<_>>();

        let fragment = context
            .get_complex_fragment_mut(fragment_id)
            .expect("Fragment not found in compiler.");

        fragment.declarations = filtered_fragments.into_iter().collect();

        Ok(change)
    }
}

impl XmlnsContextTransformer for &RemoveProhibitedAttributes {
    type Error = Error;

    fn transform(
        self,
        mut context: XmlnsContextTransformerContext<'_>,
    ) -> Result<TransformChange, Self::Error> {
        context
            .iter_complex_fragment_ids::<AttributeDeclarationsFragment>()
            .collect::<Vec<_>>()
            .into_iter()
            .map(|fragment_id| {
                RemoveProhibitedAttributes::expand_attribute_declarations(
                    &mut context,
                    &fragment_id,
                )
            })
            .collect()
    }
}

impl XmlnsContextTransformer for RemoveProhibitedAttributes {
    type Error = Error;

    fn transform(
        self,
        context: XmlnsContextTransformerContext<'_>,
    ) -> Result<TransformChange, Self::Error> {
        (&self).transform(context)
    }
}
