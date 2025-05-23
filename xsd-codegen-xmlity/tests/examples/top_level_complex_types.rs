use xmlity::{ExpandedName, LocalName, XmlNamespace};
use xsd::schema as xs;
use xsd_codegen_xmlity::Generator;
use xsd_type_compiler::{complex::ANY_TYPE_EXPANDED_NAME, CompiledNamespace, XmlnsContext};

#[test]
fn top_level_complex_type_sequence_test() {
    let integer_expanded_name = ExpandedName::new(
        LocalName::new_dangerous("integer"),
        XmlNamespace::XMLNS.into(),
    );
    let string_expanded_name = ExpandedName::new(
        LocalName::new_dangerous("string"),
        XmlNamespace::XMLNS.into(),
    );

    let product_type = xs::TopLevelComplexType::builder()
        .name(LocalName::new_dangerous("ProductType"))
        .content(
            xs::ComplexContent::builder()
                .content(
                    xs::ComplexRestrictionType::builder()
                        .base(xs::Base(xs::QName(ANY_TYPE_EXPANDED_NAME.clone())))
                        .particle(
                            xs::SequenceType::builder()
                                .content(vec![
                                    xs::LocalElement::builder()
                                        .name(xs::Name(LocalName::new_dangerous("number")))
                                        .type_(xs::Type(xs::QName(integer_expanded_name)))
                                        .build()
                                        .into(),
                                    xs::LocalElement::builder()
                                        .name(xs::Name(LocalName::new_dangerous("name")))
                                        .type_(xs::Type(xs::QName(string_expanded_name)))
                                        .build()
                                        .into(),
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

    let namespace = XmlNamespace::new_dangerous("http://example.com");

    let mut compiled_namespace = CompiledNamespace::new(namespace.clone());

    let product_type = compiled_namespace
        .import_top_level_complex_type(&product_type)
        .unwrap()
        .into_owned();

    let mut context = XmlnsContext::new();

    context.add_namespace(compiled_namespace);

    let generator = Generator::new(&context);

    let (_, actual_code) = generator.generate_top_level_type(&product_type).unwrap();

    // Expected generated code
    let expected_code: syn::ItemStruct = syn::parse_quote!(
        #[derive(
            Debug, PartialEq, Eq, Clone, xmlity::SerializationGroup, xmlity::DeserializationGroup,
        )]
        #[xgroup(children_order = "strict")]
        pub struct ProductType {
            #[xelement(name = "number")]
            pub number: Option<String>,
            #[xelement(name = "name")]
            pub name: String,
        }
    );

    assert_eq!(actual_code, vec![expected_code.into()]);
}

#[test]
fn top_level_complex_type_attributes_test() {
    let integer_expanded_name = ExpandedName::new(
        LocalName::new_dangerous("integer"),
        XmlNamespace::XMLNS.into(),
    );
    let string_expanded_name = ExpandedName::new(
        LocalName::new_dangerous("string"),
        XmlNamespace::XMLNS.into(),
    );

    let product_type = xs::TopLevelComplexType::builder()
        .name(LocalName::new_dangerous("ProductType"))
        .content(
            xs::ComplexContent::builder()
                .content(
                    xs::ComplexRestrictionType::builder()
                        .base(xs::Base(xs::QName(ANY_TYPE_EXPANDED_NAME.clone())))
                        .attributes(vec![
                            xs::LocalAttribute::builder()
                                .name(xs::Name(LocalName::new_dangerous("number")))
                                .type_(xs::Type(xs::QName(integer_expanded_name.clone())))
                                .use_(xs::AttrUse(xs::AttributeUseType::Optional))
                                .build()
                                .into(),
                            xs::LocalAttribute::builder()
                                .name(xs::Name(LocalName::new_dangerous("name")))
                                .type_(xs::Type(xs::QName(string_expanded_name.clone())))
                                .use_(xs::AttrUse(xs::AttributeUseType::Required))
                                .build()
                                .into(),
                        ])
                        .build()
                        .into(),
                )
                .build()
                .into(),
        )
        .build();

    // Expected generated code
    let code: syn::ItemStruct = syn::parse_quote!(
        #[derive(
            Debug, PartialEq, Eq, Clone, xmlity::SerializationGroup, xmlity::DeserializationGroup,
        )]
        #[xgroup(attribute_order = "none")]
        pub struct ProductType {
            #[attribute(name = "number", optional)]
            pub number: Option<String>,
            #[attribute(name = "name")]
            pub name: String,
        }
    );
}
