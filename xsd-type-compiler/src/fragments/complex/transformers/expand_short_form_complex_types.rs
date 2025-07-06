use crate::{
    fragments::complex::{
        ComplexContentChildId, ComplexTypeModelId, ComplexTypeRootFragment, FragmentAccess,
        FragmentIdx, RestrictionFragment,
    },
    transformers::{TransformChange, XmlnsLocalTransformer, XmlnsLocalTransformerContext},
};
use xsd::xsn;

/// This transformer expands the short form of complex types (particles as a direct descendent of `complexType`) into the `complexContent` form.
///
/// ### Example
/// ```xml
/// <xs:complexType name="length">
///     <xs:sequence>
///         <xs:element name="size" type="xs:nonNegativeInteger"/>
///         <xs:element name="unit" type="xs:NMTOKEN"/>
///     </xs:sequence>
/// </xs:complexType>
/// ```
/// becomes
/// ```xml
/// <xs:complexType name="length">
///     <xs:complexContent>
///         <xs:restriction base="xs:anyType">
///             <xs:sequence>
///                 <xs:element name="size" type="xs:nonNegativeInteger"/>
///                 <xs:element name="unit" type="xs:NMTOKEN"/>
///             </xs:sequence>
///         </xs:restriction>
///     </xs:complexContent>
/// </xs:complexType>
/// ```
/// Above example taken from the examples of the XML Specification.
#[non_exhaustive]
pub struct ExpandShortFormComplexTypes {}

#[derive(Debug, thiserror::Error)]
pub enum Error {}

impl ExpandShortFormComplexTypes {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {}
    }

    pub fn expand_short_form_complex_type(
        ctx: &mut XmlnsLocalTransformerContext<'_>,
        fragment_id: &FragmentIdx<ComplexTypeRootFragment>,
    ) -> Result<TransformChange, Error> {
        let root_fragment = ctx
            .get_complex_fragment::<ComplexTypeRootFragment>(fragment_id)
            .unwrap();

        let ComplexTypeModelId::Other {
            particle,
            attr_decls,
        } = &root_fragment.content
        else {
            return Ok(TransformChange::Unchanged);
        };
        let content_fragment = *particle;
        let attribute_declarations = *attr_decls;

        let compiler = &mut ctx.current_namespace_mut().complex_type;

        let complex_content = compiler.push_fragment(RestrictionFragment {
            base: xsn::ANY_TYPE.clone(),
            content_fragment,
            attribute_declarations,
        });

        let complex_content =
            compiler.push_fragment(crate::fragments::complex::ComplexContentFragment {
                content_fragment: ComplexContentChildId::Restriction(complex_content),
                mixed: None,
            });

        let root_fragment = ctx.get_complex_fragment_mut(fragment_id).unwrap();

        root_fragment.content = ComplexTypeModelId::ComplexContent(complex_content);

        Ok(TransformChange::Changed)
    }
}

impl XmlnsLocalTransformer for &ExpandShortFormComplexTypes {
    type Error = Error;

    fn transform(
        self,
        mut ctx: XmlnsLocalTransformerContext<'_>,
    ) -> Result<TransformChange, Self::Error> {
        ctx.iter_complex_fragment_ids()
            .into_iter()
            .map(|f| ExpandShortFormComplexTypes::expand_short_form_complex_type(&mut ctx, &f))
            .collect()
    }
}

impl XmlnsLocalTransformer for ExpandShortFormComplexTypes {
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
    use pretty_assertions::assert_eq;

    use xmlity::{LocalName, XmlNamespace};
    use xsd::xs;
    use xsd::xsn;

    use crate::{
        fragments::complex::transformers::expand_short_form_complex_types::ExpandShortFormComplexTypes,
        transformers::TransformChange, XmlnsContext,
    };

    #[test]
    fn specification_1() {
        const TEST_NAMESPACE: XmlNamespace<'static> =
            XmlNamespace::new_dangerous("http://localhost");
        let mut ctx = XmlnsContext::new();
        let ns = ctx.init_namespace(TEST_NAMESPACE);

        // Common for both
        let sequence = xs::Sequence(
            xs::types::ExplicitGroup::builder()
                .nested_particle(vec![
                    xs::types::LocalElement::builder()
                        .name(LocalName::new_dangerous("size"))
                        .type_attribute(xs::types::QName(xsn::NON_NEGATIVE_INTEGER.clone()))
                        .build()
                        .into(),
                    xs::types::LocalElement::builder()
                        .name(LocalName::new_dangerous("unit"))
                        .type_attribute(xs::types::QName(xsn::NMTOKEN.clone()))
                        .build()
                        .into(),
                ])
                .build()
                .into(),
        );

        // <xs:complexType name="length">
        //     <xs:sequence>
        //         <xs:element name="size" type="xs:nonNegativeInteger"/>
        //         <xs:element name="unit" type="xs:NMTOKEN"/>
        //     </xs:sequence>
        // </xs:complexType>
        let input = xs::types::TopLevelComplexType::builder()
            .name(LocalName::new_dangerous("length"))
            .complex_type_model(
                Box::new(
                xs::groups::complex_type_model_items::complex_type_model_variants::Variant2
                ::builder()
                .type_def_particle(Box::new(sequence.clone().into()))
                .attr_decls(xs::groups::AttrDecls::builder().build())
                .assertions(xs::groups::Assertions::builder().build())
                .build()
                .into()
            )
            )
            .build()
            .into();

        // <xs:complexType name="length">
        //     <xs:complexContent>
        //         <xs:restriction base="xs:anyType">
        //             <xs:sequence>
        //                 <xs:element name="size" type="xs:nonNegativeInteger"/>
        //                 <xs:element name="unit" type="xs:NMTOKEN"/>
        //             </xs:sequence>
        //         </xs:restriction>
        //     </xs:complexContent>
        // </xs:complexType>
        let expected_output: xs::ComplexType = xs::types::TopLevelComplexType::builder()
            .name(LocalName::new_dangerous("length"))
            .complex_type_model(Box::new(
                xs::ComplexContent::builder()
                    .child_1(
                        xs::types::ComplexRestrictionType::builder()
                            .base(xs::types::QName(xsn::ANY_TYPE.clone()))
                            .child_1(
                                xs::types::complex_restriction_type_items::Child1::builder()
                                    .type_def_particle(Box::new(sequence.clone().into()))
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

        let length = ns
            .import_top_level_complex_type(&input)
            .unwrap()
            .into_owned();

        let transform_changed = ns.transform(ExpandShortFormComplexTypes::new()).unwrap();

        assert_eq!(transform_changed, TransformChange::Changed);

        let transform_changed = ns.transform(ExpandShortFormComplexTypes::new()).unwrap();

        assert_eq!(transform_changed, TransformChange::Unchanged);

        let actual_output = ns
            .export_top_level_complex_type(length.local_name())
            .unwrap()
            .unwrap();

        assert_eq!(expected_output, actual_output);
    }
}
