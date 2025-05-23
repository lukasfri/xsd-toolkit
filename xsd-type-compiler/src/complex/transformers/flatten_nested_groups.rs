use std::collections::VecDeque;

use crate::complex::{
    AllFragment, ChoiceFragment, FragmentIdx, GroupTypeContentId, SequenceFragment,
};

use crate::transformers::{Context, TransformChange, XmlnsContextTransformer};

#[non_exhaustive]
pub struct FlattenNestedSequences {}

impl FlattenNestedSequences {
    pub fn new() -> Self {
        Self {}
    }

    fn flatten_sequence(
        ctx: &mut Context,
        fragment_idx: &FragmentIdx<SequenceFragment>,
    ) -> Result<TransformChange, <Self as XmlnsContextTransformer>::Error> {
        let SequenceFragment { fragments, .. } = ctx.get_complex_fragment(&fragment_idx).unwrap();

        let mut flattened = TransformChange::default();

        let mut new_fragments = VecDeque::new();
        for fragment_id in fragments {
            let GroupTypeContentId::Sequence(seq_fragment_id) = fragment_id else {
                new_fragments.push_back(fragment_id.clone());
                continue;
            };

            let SequenceFragment {
                fragments: sub_fragments,
                max_occurs,
                min_occurs,
            } = ctx.get_complex_fragment(seq_fragment_id).unwrap();

            if max_occurs.is_some() || min_occurs.is_some() {
                new_fragments.push_back(fragment_id.clone());
                continue;
            }

            new_fragments.extend(sub_fragments.iter().cloned());
            flattened = TransformChange::Changed;
        }

        let fragment = ctx.get_complex_fragment_mut(&fragment_idx).unwrap();
        fragment.fragments = new_fragments;

        Ok(flattened)
    }
}

impl XmlnsContextTransformer for FlattenNestedSequences {
    type Error = ();

    fn transform(self, mut ctx: Context<'_>) -> Result<TransformChange, Self::Error> {
        ctx.iter_complex_fragment_ids()
            .into_iter()
            .map(|f| Self::flatten_sequence(&mut ctx, &f))
            .collect()
    }
}

impl FlattenNestedChoices {
    pub fn new() -> Self {
        Self {}
    }

    fn flatten_choice(
        ctx: &mut Context,
        fragment_idx: &FragmentIdx<ChoiceFragment>,
    ) -> Result<TransformChange, <Self as XmlnsContextTransformer>::Error> {
        let ChoiceFragment { fragments, .. } = ctx.get_complex_fragment(&fragment_idx).unwrap();

        let mut flattened = TransformChange::default();

        let mut new_fragments = VecDeque::new();
        for fragment_id in fragments {
            let GroupTypeContentId::Choice(choice_fragment_id) = fragment_id else {
                new_fragments.push_back(fragment_id.clone());
                continue;
            };

            let ChoiceFragment {
                fragments: sub_fragments,
                max_occurs,
                min_occurs,
            } = ctx.get_complex_fragment(choice_fragment_id).unwrap();

            if max_occurs.is_some() || min_occurs.is_some() {
                new_fragments.push_back(fragment_id.clone());
                continue;
            }

            new_fragments.extend(sub_fragments.iter().cloned());
            flattened = TransformChange::Changed;
        }

        let fragment = ctx.get_complex_fragment_mut(&fragment_idx).unwrap();
        fragment.fragments = new_fragments;

        Ok(flattened)
    }
}

#[non_exhaustive]
pub struct FlattenNestedChoices {}

impl XmlnsContextTransformer for FlattenNestedChoices {
    type Error = ();

    fn transform(self, mut ctx: Context<'_>) -> Result<TransformChange, Self::Error> {
        ctx.iter_complex_fragment_ids()
            .into_iter()
            .map(|f| Self::flatten_choice(&mut ctx, &f))
            .collect()
    }
}

impl FlattenNestedAll {
    pub fn new() -> Self {
        Self {}
    }

    fn flatten_all(
        ctx: &mut Context,
        fragment_idx: &FragmentIdx<AllFragment>,
    ) -> Result<TransformChange, <Self as XmlnsContextTransformer>::Error> {
        let AllFragment { fragments, .. } = ctx.get_complex_fragment(&fragment_idx).unwrap();

        let mut flattened = TransformChange::default();

        let mut new_fragments = VecDeque::new();
        for fragment_id in fragments {
            let GroupTypeContentId::All(all_fragment_id) = fragment_id else {
                new_fragments.push_back(fragment_id.clone());
                continue;
            };

            let AllFragment {
                fragments: sub_fragments,
                max_occurs,
                min_occurs,
            } = ctx.get_complex_fragment(all_fragment_id).unwrap();

            if max_occurs.is_some() || min_occurs.is_some() {
                new_fragments.push_back(fragment_id.clone());
                continue;
            }

            new_fragments.extend(sub_fragments.iter().cloned());
            flattened = TransformChange::Changed;
        }

        let fragment = ctx.get_complex_fragment_mut(&fragment_idx).unwrap();
        fragment.fragments = new_fragments;

        Ok(flattened)
    }
}

#[non_exhaustive]
pub struct FlattenNestedAll {}

impl XmlnsContextTransformer for FlattenNestedAll {
    type Error = ();

    fn transform(self, mut ctx: Context<'_>) -> Result<TransformChange, Self::Error> {
        ctx.iter_complex_fragment_ids()
            .into_iter()
            .map(|f| Self::flatten_all(&mut ctx, &f))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use xmlity::{ExpandedName, LocalName, XmlNamespace};
    use xsd::schema::{self as xs, MaxOccurs, MaxOccursValue};

    use crate::{
        complex::{transformers::FlattenNestedSequences, ANY_TYPE_EXPANDED_NAME},
        transformers::TransformChange,
        CompiledNamespace, XmlnsContext,
    };

    #[test]
    fn flatten_nested_sequences() {
        let test_namespace = XmlNamespace::new_dangerous("http://localhost");

        let number = xs::LocalElement::new_ref_typed(
            LocalName::new_dangerous("number"),
            ExpandedName::new(
                LocalName::new_dangerous("integer"),
                XmlNamespace::XMLNS.into(),
            ),
        );

        let name = xs::LocalElement::new_ref_typed(
            LocalName::new_dangerous("name"),
            ExpandedName::new(
                LocalName::new_dangerous("string"),
                XmlNamespace::XMLNS.into(),
            ),
        );

        let child_choice = xs::ChoiceType::builder()
            .max_occurs(MaxOccurs(MaxOccursValue::Unbounded))
            .content(vec![
                xs::LocalElement::new_ref_typed(
                    LocalName::new_dangerous("size"),
                    ExpandedName::new(
                        LocalName::new_dangerous("integer"),
                        XmlNamespace::XMLNS.into(),
                    ),
                )
                .into(),
                xs::LocalElement::new_ref_typed(
                    LocalName::new_dangerous("color"),
                    ExpandedName::new(
                        LocalName::new_dangerous("string"),
                        XmlNamespace::XMLNS.into(),
                    ),
                )
                .into(),
            ])
            .build();

        // ```xml
        // <xs:complexType name="ShirtType">
        //   <xs:restriction base="xs:anyType">
        //     <xs:sequence>
        //       <xs:sequence>
        //         <xs:element name="number" type="xs:integer"/>
        //         <xs:element name="name" type="xs:string"/>
        //       </xs:sequence>
        //       <xs:choice maxOccurs="unbounded">
        //         <xs:element name="size" type="xs:integer"/>
        //         <xs:element name="color" type="xs:string"/>
        //       </xs:choice>
        //     </xs:sequence>
        //   </xs:restriction>
        // </xs:complexType>
        // ```
        let non_flattened_shirt_type = xs::TopLevelComplexType::builder()
            .name(LocalName::new_dangerous("ShirtType"))
            .content(
                xs::ComplexContent::builder()
                    .content(
                        xs::ComplexRestrictionType::builder()
                            .base(xs::Base(xs::QName(ANY_TYPE_EXPANDED_NAME.clone())))
                            .particle(xs::TypeDefParticle::Sequence(
                                xs::SequenceType::builder()
                                    .content(vec![
                                        xs::SequenceType::builder()
                                            .content(vec![
                                                number.clone().into(),
                                                name.clone().into(),
                                            ])
                                            .build()
                                            .into(),
                                        child_choice.clone().into(),
                                    ])
                                    .build(),
                            ))
                            .build()
                            .into(),
                    )
                    .build()
                    .into(),
            )
            .build();

        // ```xml
        // <xs:complexType name="ShirtType">
        //   <xs:restriction base="xs:anyType">
        //     <xs:sequence>
        //       <xs:element name="number" type="xs:integer"/>
        //       <xs:element name="name" type="xs:string"/>
        //       <xs:choice maxOccurs="unbounded">
        //         <xs:element name="size" type="xs:integer"/>
        //         <xs:element name="color" type="xs:string"/>
        //       </xs:choice>
        //     </xs:sequence>
        //   </xs:restriction>
        // </xs:complexType>
        // ```
        let expected_flattened_shirt_type = xs::TopLevelComplexType::builder()
            .name(LocalName::new_dangerous("ShirtType"))
            .content(
                xs::ComplexContent::builder()
                    .content(
                        xs::ComplexRestrictionType::builder()
                            .base(xs::Base(xs::QName(ANY_TYPE_EXPANDED_NAME.clone())))
                            .particle(
                                xs::SequenceType::builder()
                                    .content(vec![
                                        number.clone().into(),
                                        name.clone().into(),
                                        child_choice.into(),
                                    ])
                                    .build()
                                    .into(),
                            )
                            .build()
                            .into(),
                    )
                    .build()
                    .into(),
            )
            .build();

        let mut xmlns_context = XmlnsContext::new();

        let mut compiled_namespace = CompiledNamespace::new(test_namespace.clone());

        compiled_namespace
            .import_top_level_complex_type(&non_flattened_shirt_type)
            .unwrap();

        xmlns_context.add_namespace(compiled_namespace);

        let transform_changed = xmlns_context
            .transform(&test_namespace, FlattenNestedSequences::new())
            .unwrap();

        assert_eq!(transform_changed, TransformChange::Changed);

        let transform_changed = xmlns_context
            .transform(&test_namespace, FlattenNestedSequences::new())
            .unwrap();

        assert_eq!(transform_changed, TransformChange::Unchanged);

        let compiled_namespace = xmlns_context.namespaces.get(&test_namespace).unwrap();

        let actual_flattened_shirt_type = compiled_namespace
            .export_top_level_complex_type(&LocalName::new_dangerous("ShirtType"))
            .unwrap()
            .unwrap();

        assert_eq!(expected_flattened_shirt_type, actual_flattened_shirt_type);
    }
}
