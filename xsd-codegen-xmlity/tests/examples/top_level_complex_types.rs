use pretty_assertions::assert_eq;
use xmlity::{LocalName, XmlNamespace};
use xsd::schema as xs;
use xsd::schema_names as xsn;
use xsd_codegen_xmlity::Generator;
use xsd_type_compiler::{CompiledNamespace, XmlnsContext};

#[test]
fn top_level_complex_type_sequence_test() {
    let product_type = xs::TopLevelComplexType::builder()
        .name(LocalName::new_dangerous("ProductType"))
        .content(
            xs::ComplexContent::builder()
                .content(
                    xs::ComplexRestrictionType::builder()
                        .base(xs::QName(xsn::ANY_TYPE.clone()))
                        .particle(
                            xs::SequenceType::builder()
                                .content(vec![
                                    xs::LocalElement::builder()
                                        .name(LocalName::new_dangerous("number"))
                                        .type_(xs::QName(xsn::INTEGER.clone()))
                                        .min_occurs(xs::MinOccurs(0))
                                        .build()
                                        .into(),
                                    xs::LocalElement::builder()
                                        .name(LocalName::new_dangerous("name"))
                                        .type_(xs::QName(xsn::STRING.clone()))
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

    let mut generator = Generator::new(&context);

    generator.bind_types(xsd_codegen_xmlity::binds::StdXsdTypes);

    let (_, actual_code) = generator.generate_top_level_type(&product_type).unwrap();

    // Expected generated code
    #[rustfmt::skip]
    let expected_code: syn::ItemStruct = syn::parse_quote!(
        #[derive(::core::fmt::Debug, ::xmlity::SerializationGroup, ::xmlity::DeserializationGroup)]
        #[xgroup(children_order = "strict")]
        pub struct ProductType {
            #[xelement(name = "number", namespace = "http://example.com", optional, default)]
            pub number: Option<i32>,
            #[xelement(name = "name", namespace = "http://example.com")]
            pub name: String,
        }
    );

    assert_eq!(actual_code, vec![expected_code.into()]);
}

#[test]
fn top_level_complex_type_attributes_test() {
    let product_type = xs::TopLevelComplexType::builder()
        .name(LocalName::new_dangerous("ProductType"))
        .content(
            xs::ComplexContent::builder()
                .content(
                    xs::ComplexRestrictionType::builder()
                        .base(xs::QName(xsn::ANY_TYPE.clone()))
                        .attributes(vec![
                            xs::LocalAttribute::builder()
                                .name(LocalName::new_dangerous("number"))
                                .type_(xs::QName(xsn::INTEGER.clone()))
                                .use_(xs::AttributeUseType::Optional)
                                .build()
                                .into(),
                            xs::LocalAttribute::builder()
                                .name(LocalName::new_dangerous("name"))
                                .type_(xs::QName(xsn::STRING.clone()))
                                .use_(xs::AttributeUseType::Required)
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

    let namespace = XmlNamespace::new_dangerous("http://example.com");

    let mut compiled_namespace = CompiledNamespace::new(namespace.clone());

    let product_type = compiled_namespace
        .import_top_level_complex_type(&product_type)
        .unwrap()
        .into_owned();

    let mut context = XmlnsContext::new();

    context.add_namespace(compiled_namespace);

    let mut generator = Generator::new(&context);

    generator.bind_types(xsd_codegen_xmlity::binds::StdXsdTypes);

    let (_, actual_code) = generator.generate_top_level_type(&product_type).unwrap();

    // Expected generated code
    #[rustfmt::skip]
    let expected_code: syn::ItemStruct = syn::parse_quote!(
        #[derive(::core::fmt::Debug, ::xmlity::SerializationGroup, ::xmlity::DeserializationGroup)]
        pub struct ProductType {
            #[xattribute(name = "number", optional, default)]
            pub number: Option<i32>,
            #[xattribute(name = "name")]
            pub name: String,
        }
    );

    assert_eq!(actual_code, vec![expected_code.into()]);
}
