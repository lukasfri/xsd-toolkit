use pretty_assertions::assert_eq;
use syn::parse_quote;
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
                                        .type_(xs::Type(xs::QName(integer_expanded_name.clone())))
                                        .min_occurs(xs::MinOccurs(0))
                                        .build()
                                        .into(),
                                    xs::LocalElement::builder()
                                        .name(xs::Name(LocalName::new_dangerous("name")))
                                        .type_(xs::Type(xs::QName(string_expanded_name.clone())))
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
    let namespace_lit = namespace.as_str();

    let mut compiled_namespace = CompiledNamespace::new(namespace.clone());

    let product_type = compiled_namespace
        .import_top_level_complex_type(&product_type)
        .unwrap()
        .into_owned();

    let mut context = XmlnsContext::new();

    context.add_namespace(compiled_namespace);

    let mut generator = Generator::new(&context);

    generator.bind_type(integer_expanded_name, parse_quote!(i32));
    generator.bind_type(string_expanded_name, parse_quote!(String));

    let (_, actual_code) = generator.generate_top_level_type(&product_type).unwrap();

    // Expected generated code
    let expected_code: syn::ItemStruct = syn::parse_quote!(
        #[derive(
            Debug, PartialEq, Eq, Clone, xmlity::SerializationGroup, xmlity::DeserializationGroup,
        )]
        #[xgroup(children_order = "strict")]
        pub struct ProductType {
            #[xelement(name = "number", namespace = #namespace_lit)]
            pub number: Option<i32>,
            #[xelement(name = "name", namespace = #namespace_lit)]
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
