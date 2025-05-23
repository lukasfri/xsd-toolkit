use syn::parse_quote;
use xmlity::{ExpandedName, LocalName, XmlNamespace};
use xsd::schema as xs;
use xsd_type_compiler::complex::ANY_TYPE_EXPANDED_NAME;

#[test]
fn reference_element() {
    let product_type = xs::TopLevelElement::builder()
        .name(xs::Name(LocalName::new_dangerous("ProductType")))
        .type_(xs::Type(xs::QName(ExpandedName::new(
            LocalName::new_dangerous("ProductType"),
            Some(XmlNamespace::XMLNS),
        ))))
        .build();

    let complex_type_path: syn::Type = parse_quote!(types::ProductType);

    let type_resolver = || complex_type_path.clone();

    // Expected generated code
    let code: syn::ItemStruct = syn::parse_quote!(
        #[derive(Debug, PartialEq, Eq, Clone, xmlity::Serialize, xmlity::Deserialize)]
        #[xelement(name = "ProductType")]
        pub struct ProductType(#[xgroup] #complex_type_path);
    );
}

#[test]
fn local_complex_type_sequence_element() {
    let integer_expanded_name = ExpandedName::new(
        LocalName::new_dangerous("integer"),
        XmlNamespace::XMLNS.into(),
    );
    let string_expanded_name = ExpandedName::new(
        LocalName::new_dangerous("string"),
        XmlNamespace::XMLNS.into(),
    );

    let product_type = xs::TopLevelElement::builder()
        .name(xs::Name(LocalName::new_dangerous("ProductType")))
        .type_choice(
            xs::LocalComplexType::builder()
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
                .build()
                .into(),
        )
        .build();

    let complex_type_path: syn::Type = parse_quote!(types::ProductType);

    let type_resolver = || complex_type_path.clone();

    // Expected generated code
    let code: syn::ItemStruct = syn::parse_quote!(
        #[derive(Debug, PartialEq, Eq, Clone, xmlity::Serialize, xmlity::Deserialize)]
        #[xelement(name = "ProductType", children_order = "strict")]
        pub struct ProductType {
            #[xelement(name = "number")]
            pub number: String,
            #[xelement(name = "name")]
            pub name: String,
        }
    );
}

#[test]
fn top_level_element_local_complex_type() {
    let integer_expanded_name = ExpandedName::new(
        LocalName::new_dangerous("integer"),
        XmlNamespace::XMLNS.into(),
    );

    let string_expanded_name = ExpandedName::new(
        LocalName::new_dangerous("string"),
        XmlNamespace::XMLNS.into(),
    );

    let product_type = xs::TopLevelElement::builder()
        .name(xs::Name(LocalName::new_dangerous("ProductType")))
        .type_choice(
            xs::LocalComplexType::builder()
                .content(
                    xs::ComplexContent::builder()
                        .content(
                            xs::ComplexRestrictionType::builder()
                                .base(xs::Base(xs::QName(ANY_TYPE_EXPANDED_NAME.clone())))
                                .attributes(vec![
                                    xs::LocalAttribute::builder()
                                        .name(xs::Name(LocalName::new_dangerous("number")))
                                        .type_(xs::Type(xs::QName(integer_expanded_name)))
                                        .build()
                                        .into(),
                                    xs::LocalAttribute::builder()
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

    let complex_type_path: syn::Type = parse_quote!(types::ProductType);

    let type_resolver = || complex_type_path.clone();

    // Expected generated code
    let code: syn::ItemStruct = syn::parse_quote!(
        #[derive(Debug, PartialEq, Eq, Clone, xmlity::Serialize, xmlity::Deserialize)]
        #[xelement(name = "ProductType")]
        pub struct ProductType {
            #[xattribute(name = "number")]
            pub number: String,
            #[xattribute(name = "name")]
            pub name: String,
        }
    );
}
