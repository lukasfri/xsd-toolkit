use std::str::FromStr;

use pretty_assertions::assert_eq;

use syn::{parse_quote, File};
use url::Url;
use xmlity::{ExpandedName, LocalName, XmlNamespace};
use xsd::xs;
use xsd_fragments::XmlnsContext;

const THREE_CHOICE_SEQUENCE_DEEP_COMPLEX_TYPE_INPUT: &str = r###"
<xs:schema xmlns:xs="http://www.w3.org/2001/XMLSchema"
           xmlns:ns="http://example.com"
           targetNamespace="http://example.com">
  <xs:complexType name="SimpleSequence">
    <xs:sequence>
      <xs:choice>
        <xs:sequence>
          <xs:element name="a" type="xs:integer"/>
        </xs:sequence>
        <xs:element name="b" type="xs:string"/>
      </xs:choice>
      <xs:element name="c" type="xs:string"/>
    </xs:sequence>
  </xs:complexType>
</xs:schema>
"###;

#[rustfmt::skip]
fn three_choice_sequence_deep_complex_type_expected() -> File {
    parse_quote!(
        pub mod simple_sequence_items {
            pub mod child_0_variants {
                #[derive(::core::fmt::Debug, ::xmlity::Serialize, ::xmlity::Deserialize)]
                #[xvalue(order = "strict")]
                pub struct A {
                    #[xelement(name = "a", namespace = "http://example.com")]
                    pub a: i32,
                }
            }

            #[derive(::core::fmt::Debug, ::xmlity::Serialize, ::xmlity::Deserialize)]
            pub enum Child0 {
                A(::std::boxed::Box<child_0_variants::A>),
                #[xelement(
                    name = "b",
                    namespace = "http://example.com",
                    allow_unknown_attributes = "any"
                )]
                B(String),
            }
        }

        #[derive(
            ::core::fmt::Debug, ::xmlity::SerializationGroup, ::xmlity::DeserializationGroup
        )]
        #[xgroup(children_order = "strict")]
        pub struct SimpleSequence {
            pub child_0: simple_sequence_items::Child0,
            #[xelement(name = "c", namespace = "http://example.com")]
            pub c: String,
        }
    )
}

const SIMPLE_SEQUENCE_DEEP_TOP_LEVEL_INPUT: &str = r###"
<xs:schema xmlns:xs="http://www.w3.org/2001/XMLSchema"
           xmlns:ns="http://example.com"
           targetNamespace="http://example.com">
  <xs:complexType name="SimpleSequence">
    <xs:sequence>
      <xs:sequence minOccurs="0">
          <xs:element name="a" type="xs:integer" minOccurs="0"/>
          <xs:element name="b" type="xs:string"/>
      </xs:sequence>
      <xs:element name="c" type="xs:string"/>
    </xs:sequence>
  </xs:complexType>
</xs:schema>
"###;

#[rustfmt::skip]
fn two_sequence_deep_top_level() -> File {
    parse_quote!(
        pub mod simple_sequence_items {
            #[derive(::core::fmt::Debug, ::xmlity::Serialize, ::xmlity::Deserialize)]
            #[xvalue(order = "strict")]
            pub struct Child0 {
                #[xelement(name = "a", namespace = "http://example.com", optional)]
                pub a: ::core::option::Option<i32>,
                #[xelement(name = "b", namespace = "http://example.com")]
                pub b: String,
            }
        }

        #[derive(::core::fmt::Debug, ::xmlity::SerializationGroup, ::xmlity::DeserializationGroup)]
        #[xgroup(children_order = "strict")]
        pub struct SimpleSequence {
            #[xvalue(default)]
            pub child_0: ::core::option::Option<simple_sequence_items::Child0>,
            #[xelement(name = "c", namespace = "http://example.com")]
            pub c: String,
        }
    )
}

#[rstest::rstest]
#[case::three_choice_sequence_deep_complex_type(
    THREE_CHOICE_SEQUENCE_DEEP_COMPLEX_TYPE_INPUT,
    three_choice_sequence_deep_complex_type_expected(),
    ExpandedName::new(
                LocalName::new_dangerous("SimpleSequence"),
                Some(XmlNamespace::new_dangerous("http://example.com")),
            ),
    parse_quote!(::std::boxed::Box<SimpleSequence>)
)]
#[case::two_sequence_deep_top_level(
    SIMPLE_SEQUENCE_DEEP_TOP_LEVEL_INPUT,
    two_sequence_deep_top_level(),
    ExpandedName::new(
                LocalName::new_dangerous("SimpleSequence"),
                Some(XmlNamespace::new_dangerous("http://example.com")),
            ),
    parse_quote!(::std::boxed::Box<SimpleSequence>)
)]
fn group_generation(
    #[case] input: &str,
    #[case] expected_code: File,
    #[case] expanded_name: ExpandedName,
    #[case] expected_type: syn::Type,
) {
    use xsd_codegen_xmlity::Generator;
    use xsd_fragments::FragmentedXsdDocumentKey;

    let input: xs::Schema =
        xmlity_quick_xml::from_str(input.trim()).expect("Failed to parse XML Schema");

    let mut ctx = XmlnsContext::new();

    let (id, _) = ctx
        .import_schema(
            FragmentedXsdDocumentKey(Url::from_str("http://example.com/test").unwrap()),
            &input,
        )
        .expect("Failed to import schema");

    let mut generator = Generator::new(&ctx);

    generator.bind_types(xsd_codegen_xmlity::binds::StdXsdTypes);

    let (ty, items) = generator
        .generate_type(&id, &expanded_name)
        .expect("Failed to generate group type");

    let actual = prettyplease::unparse(&syn::File {
        shebang: None,
        attrs: Vec::new(),
        items,
    });

    let expected_code = prettyplease::unparse(&expected_code);

    assert_eq!(actual, expected_code);

    assert_eq!(ty.ty.into_type(None), expected_type);
}
