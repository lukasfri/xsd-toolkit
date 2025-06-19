use std::collections::VecDeque;
use std::convert::Infallible;

use crate::complex::{
    AllFragment, ChoiceFragment, FragmentIdx, GroupTypeContentId, SequenceFragment,
};

use crate::transformers::{TransformChange, XmlnsLocalTransformer, XmlnsLocalTransformerContext};

#[non_exhaustive]
pub struct FlattenNestedSequences {}

impl FlattenNestedSequences {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {}
    }

    fn flatten_sequence(
        ctx: &mut XmlnsLocalTransformerContext,
        fragment_idx: &FragmentIdx<SequenceFragment>,
    ) -> Result<TransformChange, <Self as XmlnsLocalTransformer>::Error> {
        let SequenceFragment { fragments, .. } = ctx.get_complex_fragment(fragment_idx).unwrap();

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

        let fragment = ctx.get_complex_fragment_mut(fragment_idx).unwrap();
        fragment.fragments = new_fragments;

        Ok(flattened)
    }
}

impl XmlnsLocalTransformer for FlattenNestedSequences {
    type Error = Infallible;

    fn transform(
        self,
        mut ctx: XmlnsLocalTransformerContext<'_>,
    ) -> Result<TransformChange, Self::Error> {
        ctx.iter_complex_fragment_ids()
            .into_iter()
            .map(|f| Self::flatten_sequence(&mut ctx, &f))
            .collect()
    }
}

#[non_exhaustive]
pub struct FlattenNestedChoices {}

impl FlattenNestedChoices {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {}
    }

    fn flatten_choice(
        ctx: &mut XmlnsLocalTransformerContext,
        fragment_idx: &FragmentIdx<ChoiceFragment>,
    ) -> Result<TransformChange, <Self as XmlnsLocalTransformer>::Error> {
        let ChoiceFragment { fragments, .. } = ctx.get_complex_fragment(fragment_idx).unwrap();

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

        let fragment = ctx.get_complex_fragment_mut(fragment_idx).unwrap();
        fragment.fragments = new_fragments;

        Ok(flattened)
    }
}

impl XmlnsLocalTransformer for FlattenNestedChoices {
    type Error = Infallible;

    fn transform(
        self,
        mut ctx: XmlnsLocalTransformerContext<'_>,
    ) -> Result<TransformChange, Self::Error> {
        ctx.iter_complex_fragment_ids()
            .into_iter()
            .map(|f| Self::flatten_choice(&mut ctx, &f))
            .collect()
    }
}

#[non_exhaustive]
pub struct FlattenNestedAll {}

impl FlattenNestedAll {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {}
    }

    fn flatten_all(
        ctx: &mut XmlnsLocalTransformerContext,
        fragment_idx: &FragmentIdx<AllFragment>,
    ) -> Result<TransformChange, <Self as XmlnsLocalTransformer>::Error> {
        let AllFragment { fragments, .. } = ctx.get_complex_fragment(fragment_idx).unwrap();

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

        let fragment = ctx.get_complex_fragment_mut(fragment_idx).unwrap();
        fragment.fragments = new_fragments;

        Ok(flattened)
    }
}

impl XmlnsLocalTransformer for FlattenNestedAll {
    type Error = Infallible;

    fn transform(
        self,
        mut ctx: XmlnsLocalTransformerContext<'_>,
    ) -> Result<TransformChange, Self::Error> {
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
    use xsd::schema_names as xsn;

    use crate::{
        complex::transformers::FlattenNestedSequences, transformers::TransformChange,
        CompiledNamespace, XmlnsContext,
    };

    #[test]
    fn flatten_nested_sequences() {
        let test_namespace = XmlNamespace::new_dangerous("http://localhost");

        let number = xs::LocalElement::new_ref_typed(
            LocalName::new_dangerous("number"),
            ExpandedName::new(LocalName::new_dangerous("integer"), XmlNamespace::XS.into()),
        );

        let name =
            xs::LocalElement::new_ref_typed(LocalName::new_dangerous("name"), xsn::STRING.clone());

        let child_choice = xs::ChoiceType::builder()
            .max_occurs(MaxOccurs(MaxOccursValue::Unbounded))
            .content(vec![
                xs::LocalElement::new_ref_typed(
                    LocalName::new_dangerous("size"),
                    xsn::INTEGER.clone(),
                )
                .into(),
                xs::LocalElement::new_ref_typed(
                    LocalName::new_dangerous("color"),
                    xsn::STRING.clone(),
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
                            .base(xs::QName(xsn::ANY_TYPE.clone()))
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
                            .base(xs::QName(xsn::ANY_TYPE.clone()))
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

        let mut compiled_namespace = CompiledNamespace::new(test_namespace.clone());

        compiled_namespace
            .import_top_level_complex_type(&non_flattened_shirt_type)
            .unwrap();

        let transform_changed = compiled_namespace
            .transform(FlattenNestedSequences::new())
            .unwrap();

        assert_eq!(transform_changed, TransformChange::Changed);

        let transform_changed = compiled_namespace
            .transform(FlattenNestedSequences::new())
            .unwrap();

        assert_eq!(transform_changed, TransformChange::Unchanged);

        let mut xmlns_context = XmlnsContext::new();

        xmlns_context.add_namespace(compiled_namespace);

        let compiled_namespace = xmlns_context.namespaces.get(&test_namespace).unwrap();

        let actual_flattened_shirt_type = compiled_namespace
            .export_top_level_complex_type(&LocalName::new_dangerous("ShirtType"))
            .unwrap()
            .unwrap();

        assert_eq!(expected_flattened_shirt_type, actual_flattened_shirt_type);
    }

    #[test]
    fn do_not_flatten_sequences_with_occurs() {
        let test_namespace = XmlNamespace::new_dangerous("http://localhost");

        let number = xs::LocalElement::new_ref_typed(
            LocalName::new_dangerous("number"),
            ExpandedName::new(LocalName::new_dangerous("integer"), XmlNamespace::XS.into()),
        );

        let name =
            xs::LocalElement::new_ref_typed(LocalName::new_dangerous("name"), xsn::STRING.clone());

        let child_choice = xs::ChoiceType::builder()
            .max_occurs(MaxOccurs(MaxOccursValue::Unbounded))
            .content(vec![
                xs::LocalElement::new_ref_typed(
                    LocalName::new_dangerous("size"),
                    xsn::INTEGER.clone(),
                )
                .into(),
                xs::LocalElement::new_ref_typed(
                    LocalName::new_dangerous("color"),
                    xsn::STRING.clone(),
                )
                .into(),
            ])
            .build();

        // ```xml
        // <xs:complexType name="ShirtType">
        //   <xs:restriction base="xs:anyType">
        //     <xs:sequence>
        //       <xs:sequence maxOccurs="unbounded">
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
                            .base(xs::QName(xsn::ANY_TYPE.clone()))
                            .particle(xs::TypeDefParticle::Sequence(
                                xs::SequenceType::builder()
                                    .content(vec![
                                        xs::SequenceType::builder()
                                            .content(vec![
                                                number.clone().into(),
                                                name.clone().into(),
                                            ])
                                            .max_occurs(MaxOccurs(MaxOccursValue::Unbounded))
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

        let mut compiled_namespace = CompiledNamespace::new(test_namespace.clone());

        compiled_namespace
            .import_top_level_complex_type(&non_flattened_shirt_type)
            .unwrap();

        let transform_changed = compiled_namespace
            .transform(FlattenNestedSequences::new())
            .unwrap();

        assert_eq!(transform_changed, TransformChange::Unchanged);

        let mut xmlns_context = XmlnsContext::new();

        xmlns_context.add_namespace(compiled_namespace);

        let compiled_namespace = xmlns_context.namespaces.get(&test_namespace).unwrap();

        let actual_flattened_shirt_type = compiled_namespace
            .export_top_level_complex_type(&LocalName::new_dangerous("ShirtType"))
            .unwrap()
            .unwrap();

        assert_eq!(non_flattened_shirt_type, actual_flattened_shirt_type);
    }
}
