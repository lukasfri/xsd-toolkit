use std::collections::VecDeque;

use crate::{
    complex::{
        ComplexContentChildId, ComplexTypeModelId, ComplexTypeRootFragment, FragmentAccess,
        FragmentIdx, RestrictionFragment, ANY_TYPE_EXPANDED_NAME,
    },
    transformers::{Context, TransformChange, XmlnsContextTransformer},
};

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

impl ExpandShortFormComplexTypes {
    pub fn new() -> Self {
        Self {}
    }

    pub fn expand_short_form_complex_type(
        ctx: &mut Context<'_>,
        fragment_id: &FragmentIdx<ComplexTypeRootFragment>,
    ) -> Result<TransformChange, ()> {
        let root_fragment = ctx
            .get_complex_fragment::<ComplexTypeRootFragment>(&fragment_id)
            .unwrap();

        let ComplexTypeModelId::Other { particle } = &root_fragment.content else {
            return Ok(TransformChange::Unchanged);
        };
        let content_fragment = Some(particle.clone());

        let compiler = &mut ctx.current_namespace_mut().complex_type;

        let complex_content = compiler.push_fragment(RestrictionFragment {
            base: ANY_TYPE_EXPANDED_NAME.clone(),
            content_fragment,
            attribute_declarations: VecDeque::new(),
        });

        let complex_content = compiler.push_fragment(crate::complex::ComplexContentFragment {
            content_fragment: ComplexContentChildId::Restriction(complex_content),
            mixed: None,
        });

        let root_fragment = ctx.get_complex_fragment_mut(&fragment_id).unwrap();

        root_fragment.content = ComplexTypeModelId::ComplexContent(complex_content);

        Ok(TransformChange::Changed)
    }
}

impl XmlnsContextTransformer for ExpandShortFormComplexTypes {
    type Error = ();

    fn transform(self, mut ctx: Context<'_>) -> Result<TransformChange, Self::Error> {
        ctx.iter_complex_fragment_ids()
            .into_iter()
            .map(|f| Self::expand_short_form_complex_type(&mut ctx, &f))
            .collect()
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
        transformers::TransformChange,
        CompiledNamespace, XmlnsContext,
    };

    #[test]
    fn specification_1() {
        let mut xmlns_context = XmlnsContext::new();

        let namespace = XmlNamespace::new_dangerous("http://localhost");
        let mut compiled_namespace = CompiledNamespace::new(namespace.clone());

        // Common for both
        let sequence = SequenceType::builder()
            .content(vec![
                LocalElement::builder()
                    .name(Name(LocalName::new_dangerous("size")))
                    .type_(Type(QName(ExpandedName::new(
                        LocalName::new_dangerous("nonNegativeInteger"),
                        Some(XmlNamespace::XMLNS),
                    ))))
                    .build()
                    .into(),
                LocalElement::builder()
                    .name(Name(LocalName::new_dangerous("unit")))
                    .type_(Type(QName(ExpandedName::new(
                        LocalName::new_dangerous("NMTOKEN"),
                        Some(XmlNamespace::XMLNS),
                    ))))
                    .build()
                    .into(),
            ])
            .build();

        // <xs:complexType name="length">
        //     <xs:sequence>
        //         <xs:element name="size" type="xs:nonNegativeInteger"/>
        //         <xs:element name="unit" type="xs:NMTOKEN"/>
        //     </xs:sequence>
        // </xs:complexType>
        let input = TopLevelComplexType::builder()
            .name(LocalName::new_dangerous("length"))
            .content(ComplexTypeModel::Other {
                open_content: None,
                type_def_particle: Some(TypeDefParticle::Sequence(sequence.clone())),
            })
            .build();

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
        let expected_output = TopLevelComplexType::builder()
            .name(LocalName::new_dangerous("length"))
            .content(
                ComplexContent::builder()
                    .content(
                        ComplexRestrictionType::builder()
                            .base(Base(QName(ANY_TYPE_EXPANDED_NAME.clone())))
                            .particle(sequence.into())
                            .build()
                            .into(),
                    )
                    .build()
                    .into(),
            )
            .build();

        let length = compiled_namespace
            .import_top_level_complex_type(&input)
            .unwrap()
            .into_owned();

        xmlns_context.add_namespace(compiled_namespace);

        let transform_changed = xmlns_context
            .transform(&namespace, ExpandShortFormComplexTypes::new())
            .unwrap();

        assert_eq!(transform_changed, TransformChange::Changed);

        let transform_changed = xmlns_context
            .transform(&namespace, ExpandShortFormComplexTypes::new())
            .unwrap();

        assert_eq!(transform_changed, TransformChange::Unchanged);

        let output_namespace = xmlns_context
            .namespaces
            .get(length.namespace().unwrap())
            .unwrap();

        println!("{:#?}", output_namespace);

        let actual_output = output_namespace
            .export_top_level_complex_type(length.local_name())
            .unwrap()
            .unwrap();

        assert_eq!(expected_output, actual_output);
    }
}
