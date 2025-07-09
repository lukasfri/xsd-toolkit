use std::collections::VecDeque;

use xsd_fragments::fragments::{
    complex::{ChoiceFragment, NestedParticleId, SequenceFragment},
    FragmentIdx,
};

use crate::{TransformChange, XmlnsLocalTransformer, XmlnsLocalTransformerContext};

#[non_exhaustive]
pub struct FlattenNestedSequences {}

#[derive(Debug, thiserror::Error)]
pub enum Error {}

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
            let NestedParticleId::Sequence(seq_fragment_id) = fragment_id else {
                new_fragments.push_back(*fragment_id);
                continue;
            };

            let SequenceFragment {
                id: _,
                fragments: sub_fragments,
                max_occurs,
                min_occurs,
            } = ctx.get_complex_fragment(seq_fragment_id).unwrap();

            if max_occurs.is_some() || min_occurs.is_some() {
                new_fragments.push_back(*fragment_id);
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

impl XmlnsLocalTransformer for &FlattenNestedSequences {
    type Error = Error;

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
    type Error = Error;

    fn transform(
        self,
        ctx: XmlnsLocalTransformerContext<'_>,
    ) -> Result<TransformChange, Self::Error> {
        (&self).transform(ctx)
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
            let NestedParticleId::Choice(choice_fragment_id) = fragment_id else {
                new_fragments.push_back(*fragment_id);
                continue;
            };

            let ChoiceFragment {
                fragments: sub_fragments,
                max_occurs,
                min_occurs,
            } = ctx.get_complex_fragment(choice_fragment_id).unwrap();

            if max_occurs.is_some() || min_occurs.is_some() {
                new_fragments.push_back(*fragment_id);
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

impl XmlnsLocalTransformer for &FlattenNestedChoices {
    type Error = Error;

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
    type Error = Error;

    fn transform(
        self,
        ctx: XmlnsLocalTransformerContext<'_>,
    ) -> Result<TransformChange, Self::Error> {
        (&self).transform(ctx)
    }
}

#[cfg(test)]
mod tests {
    use crate::CompiledNamespaceExt;

    use super::*;
    use pretty_assertions::assert_eq;

    use xmlity::{LocalName, XmlNamespace};
    use xsd::xs::{self};
    use xsd::xsn;

    use xsd_fragments::XmlnsContext;

    #[test]
    fn flatten_nested_sequences() {
        const TEST_NAMESPACE: XmlNamespace<'static> =
            XmlNamespace::new_dangerous("http://localhost");

        let number = xs::types::LocalElement::builder()
            .name(LocalName::new_dangerous("number"))
            .type_attribute(xs::types::QName(xsn::INTEGER.clone()))
            .build();

        let name = xs::types::LocalElement::builder()
            .name(LocalName::new_dangerous("name"))
            .type_attribute(xs::types::QName(xsn::STRING.clone()))
            .build();

        let child_choice = xs::Choice(Box::new(
            xs::types::ExplicitGroup::builder()
                .max_occurs(
                    xs::types::AllNNI::from(
                        xs::types::all_nni_items::all_nni_variants::Variant0::Unbounded,
                    )
                    .into(),
                )
                .nested_particle(vec![
                    xs::types::LocalElement::builder()
                        .name(LocalName::new_dangerous("size"))
                        .type_attribute(xs::types::QName(xsn::INTEGER.clone()))
                        .build()
                        .into(),
                    xs::types::LocalElement::builder()
                        .name(LocalName::new_dangerous("color"))
                        .type_attribute(xs::types::QName(xsn::STRING.clone()))
                        .build()
                        .into(),
                ])
                .build()
                .into(),
        ));

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
        let non_flattened_shirt_type = xs::types::TopLevelComplexType::builder()
            .name(LocalName::new_dangerous("ShirtType"))
            .complex_type_model(Box::new(
                xs::ComplexContent::builder()
                    .child_1(
                        xs::types::ComplexRestrictionType::builder()
                            .base(xs::types::QName(xsn::ANY_TYPE.clone()))
                            .child_1(
                                xs::types::complex_restriction_type_items::Child1::builder()
                                    .type_def_particle(Box::new(
                                        xs::Sequence(Box::new(
                                            xs::types::ExplicitGroup::builder()
                                                .nested_particle(vec![
                                                    xs::Sequence(
                                                        xs::types::ExplicitGroup::builder()
                                                            .nested_particle(vec![
                                                                number.clone().into(),
                                                                name.clone().into(),
                                                            ])
                                                            .build()
                                                            .into(),
                                                    )
                                                    .into(),
                                                    child_choice.clone().into(),
                                                ])
                                                .build()
                                                .into(),
                                        ))
                                        .into(),
                                    ))
                                    .build()
                                    .into(),
                            )
                            .attr_decls(xs::groups::AttrDecls::builder().build().into())
                            .assertions(xs::groups::Assertions::builder().build().into())
                            .build()
                            .into(),
                    )
                    .build()
                    .into(),
            ))
            .build()
            .into();

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
        let expected_flattened_shirt_type: xs::ComplexType =
            xs::types::TopLevelComplexType::builder()
                .name(LocalName::new_dangerous("ShirtType"))
                .complex_type_model(Box::new(
                    xs::ComplexContent::builder()
                        .child_1(
                            xs::types::ComplexRestrictionType::builder()
                                .base(xs::types::QName(xsn::ANY_TYPE.clone()))
                                .child_1(
                                    xs::types::complex_restriction_type_items::Child1::builder()
                                        .type_def_particle(Box::new(
                                            xs::Sequence(Box::new(
                                                xs::types::ExplicitGroup::builder()
                                                    .nested_particle(vec![
                                                        number.clone().into(),
                                                        name.clone().into(),
                                                        child_choice.into(),
                                                    ])
                                                    .build()
                                                    .into(),
                                            ))
                                            .into(),
                                        ))
                                        .build()
                                        .into(),
                                )
                                .attr_decls(xs::groups::AttrDecls::builder().build().into())
                                .assertions(xs::groups::Assertions::builder().build().into())
                                .build()
                                .into(),
                        )
                        .build()
                        .into(),
                ))
                .build()
                .into();

        let mut ctx = XmlnsContext::new();
        let ns = ctx.init_namespace(TEST_NAMESPACE);

        ns.import_top_level_complex_type(&non_flattened_shirt_type)
            .unwrap();

        let transform_changed: TransformChange =
            ns.transform(FlattenNestedSequences::new()).unwrap();

        assert_eq!(transform_changed, TransformChange::Changed);

        let transform_changed = ns.transform(FlattenNestedSequences::new()).unwrap();

        assert_eq!(transform_changed, TransformChange::Unchanged);

        let actual_flattened_shirt_type = ns
            .export_top_level_complex_type(&LocalName::new_dangerous("ShirtType"))
            .unwrap()
            .unwrap();

        assert_eq!(expected_flattened_shirt_type, actual_flattened_shirt_type);
    }

    #[test]
    fn do_not_flatten_sequences_with_occurs() {
        const TEST_NAMESPACE: XmlNamespace<'static> =
            XmlNamespace::new_dangerous("http://localhost");

        let number = xs::types::LocalElement::builder()
            .name(LocalName::new_dangerous("number"))
            .type_attribute(xs::types::QName(xsn::INTEGER.clone()))
            .build();

        let name = xs::types::LocalElement::builder()
            .name(LocalName::new_dangerous("name"))
            .type_attribute(xs::types::QName(xsn::STRING.clone()))
            .build();

        let child_choice = xs::Choice(
            xs::types::ExplicitGroup::builder()
                .max_occurs(
                    xs::types::AllNNI::from(
                        xs::types::all_nni_items::all_nni_variants::Variant0::Unbounded,
                    )
                    .into(),
                )
                .nested_particle(vec![
                    xs::types::LocalElement::builder()
                        .name(LocalName::new_dangerous("size"))
                        .type_attribute(xs::types::QName(xsn::INTEGER.clone()))
                        .build()
                        .into(),
                    xs::types::LocalElement::builder()
                        .name(LocalName::new_dangerous("color"))
                        .type_attribute(xs::types::QName(xsn::STRING.clone()))
                        .build()
                        .into(),
                ])
                .build()
                .into(),
        );

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
        let non_flattened_shirt_type = xs::types::TopLevelComplexType::builder()
            .name(LocalName::new_dangerous("ShirtType"))
            .complex_type_model(Box::new(
                xs::ComplexContent::builder()
                    .child_1(
                        xs::types::ComplexRestrictionType::builder()
                            .base(xs::types::QName(xsn::ANY_TYPE.clone()))
                            .child_1(
                                xs::types::complex_restriction_type_items::Child1::builder()
                                    .type_def_particle(Box::new(
                                        xs::Sequence(
                                            xs::types::ExplicitGroup::builder()
                                                .nested_particle(vec![
                                                    xs::Sequence(Box::new(
                                                        xs::types::ExplicitGroup::builder()
                                                            .nested_particle(vec![
                                                                number.clone().into(),
                                                                name.clone().into(),
                                                            ])
                                                            .max_occurs(
                                                                xs::types::AllNNI::from(xs::types::all_nni_items::all_nni_variants::Variant0::Unbounded).into(),
                                                            )
                                                            .build()
                                                            .into(),
                                                    ))
                                                    .into(),
                                                    child_choice.clone().into(),
                                                ])
                                                .build()
                                                .into(),
                                        )
                                        .into(),
                                    ))
                                    .build()
                                    .into(),
                            )
                            .attr_decls(xs::groups::AttrDecls::builder().build().into())
                            .assertions(xs::groups::Assertions::builder().build().into())
                            .build()
                            .into(),
                    )
                    .build()
                    .into(),
            ))
            .build()
            .into();

        let mut ctx = XmlnsContext::new();
        let ns = ctx.init_namespace(TEST_NAMESPACE);

        ns.import_top_level_complex_type(&non_flattened_shirt_type)
            .unwrap();

        let transform_changed = ns.transform(FlattenNestedSequences::new()).unwrap();

        assert_eq!(transform_changed, TransformChange::Unchanged);

        let actual_flattened_shirt_type = ns
            .export_top_level_complex_type(&LocalName::new_dangerous("ShirtType"))
            .unwrap()
            .unwrap();

        assert_eq!(non_flattened_shirt_type, actual_flattened_shirt_type);
    }
}
