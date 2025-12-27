use std::str::FromStr;

use pretty_assertions::assert_eq;

use syn::{parse_quote, File};
use url::Url;
use xmlity::{ExpandedName, LocalName, XmlNamespace};
use xsd::xs;
use xsd_codegen_xmlity::{misc::TypeReference, BoundType, Generator, TypeType};
use xsd_fragments::{FragmentedXsdDocumentKey, XmlnsContext};

const EMPTY_SEQUENCE_ELEMENT_INPUT: &str = r###"
<xs:schema xmlns:xs="http://www.w3.org/2001/XMLSchema"
           xmlns:ns="http://example.com"
           targetNamespace="http://example.com">
  <xs:element name="SimpleSequence">
    <xs:complexType>
      <xs:complexContent>
        <xs:restriction base="xs:anyType">
          <xs:sequence/>
        </xs:restriction>
      </xs:complexContent>
    </xs:complexType>
  </xs:element>
</xs:schema>
"###;

#[rustfmt::skip]
fn empty_sequence_element_expected() -> File {
    parse_quote!(
        pub mod simple_sequence_items {
            #[derive(
                ::core::fmt::Debug,
                ::xmlity::SerializationGroup,
                ::xmlity::DeserializationGroup
            )]
            #[xgroup(children_order = "strict")]
            pub struct SimpleSequence;
        }

        #[derive(::core::fmt::Debug, ::xmlity::Serialize, ::xmlity::Deserialize)]
        pub enum SimpleSequence {
            #[xelement(name = "SimpleSequence", namespace = "http://example.com", allow_unknown_attributes = "any")]
            SimpleSequence(#[xgroup] simple_sequence_items::SimpleSequence),
            Dynamic(
                ::xmlity_ns::SubstitutionGroup<test_ns::SimpleSequence>,
            ),
        }
    )
}

const TWO_CHILD_SEQUENCE_ELEMENT_INPUT: &str = r###"
<xs:schema xmlns:xs="http://www.w3.org/2001/XMLSchema"
           xmlns:ns="http://example.com"
           targetNamespace="http://example.com">
  <xs:element name="SimpleSequence">
    <xs:complexType>
      <xs:sequence>
        <xs:element name="a" type="xs:integer"/>
        <xs:element name="b" type="xs:string"/>
      </xs:sequence>
    </xs:complexType>
  </xs:element>
</xs:schema>
"###;

#[rustfmt::skip]
fn two_child_sequence_element_expected() -> File {
    parse_quote!(
        pub mod simple_sequence_items {
            #[derive(
                ::core::fmt::Debug,
                ::xmlity::SerializationGroup,
                ::xmlity::DeserializationGroup
            )]
            #[xgroup(children_order = "strict")]
            pub struct SimpleSequence {
                #[xelement(name = "a", namespace = "http://example.com")]
                pub a: i32,
                #[xelement(name = "b", namespace = "http://example.com")]
                pub b: String,
            }
        }

        #[derive(::core::fmt::Debug, ::xmlity::Serialize, ::xmlity::Deserialize)]
        pub enum SimpleSequence {
            #[xelement(
                name = "SimpleSequence",
                namespace = "http://example.com",
                allow_unknown_attributes = "any"
            )]
            SimpleSequence(#[xgroup] simple_sequence_items::SimpleSequence),
            Dynamic(
                ::xmlity_ns::SubstitutionGroup<test_ns::SimpleSequence>,
            ),
        }
    )
}

const TWO_ATTRIBUTE_SEQUENCE_ELEMENT_INPUT: &str = r###"
<xs:schema xmlns:xs="http://www.w3.org/2001/XMLSchema"
           xmlns:ns="http://example.com"
           targetNamespace="http://example.com">
  <xs:element name="SimpleSequence">
    <xs:complexType>
      <xs:sequence>
      </xs:sequence>
      <xs:attribute name="a" type="xs:integer" use="required"/>
      <xs:attribute name="b" type="xs:string" use="required"/>
    </xs:complexType>
  </xs:element>
</xs:schema>
"###;

#[rustfmt::skip]
fn two_attribute_sequence_element_expected() -> File {
    parse_quote!(
        pub mod simple_sequence_items {
            #[derive(
                ::core::fmt::Debug,
                ::xmlity::SerializationGroup,
                ::xmlity::DeserializationGroup
            )]
            #[xgroup(children_order = "strict")]
            pub struct SimpleSequence {
                #[xattribute(name = "a")]
                pub a: i32,
                #[xattribute(name = "b")]
                pub b: String,
            }
        }

        #[derive(::core::fmt::Debug, ::xmlity::Serialize, ::xmlity::Deserialize)]
        pub enum SimpleSequence {
            #[xelement(
                name = "SimpleSequence",
                namespace = "http://example.com",
                allow_unknown_attributes = "any"
            )]
            SimpleSequence(#[xgroup] simple_sequence_items::SimpleSequence),
            Dynamic(
                ::xmlity_ns::SubstitutionGroup<test_ns::SimpleSequence>,
            ),
        }
    )
}

const TWO_SEQUENCE_DEEP_ELEMENT_INPUT: &str = r###"
<xs:schema xmlns:xs="http://www.w3.org/2001/XMLSchema"
           xmlns:ns="http://example.com"
           targetNamespace="http://example.com">
  <xs:element name="SimpleSequence">
    <xs:complexType>
      <xs:sequence>
        <xs:sequence>
          <xs:element name="a" type="xs:integer"/>
          <xs:element name="b" type="xs:string"/>
        </xs:sequence>
        <xs:element name="c" type="xs:string"/>
      </xs:sequence>
    </xs:complexType>
  </xs:element>
</xs:schema>
"###;

#[rustfmt::skip]
fn two_sequence_deep_element_expected() -> File {
    parse_quote!(
        pub mod simple_sequence_items {
            #[derive(::core::fmt::Debug, ::xmlity::Serialize, ::xmlity::Deserialize)]
            #[xvalue(order = "strict")]
            pub struct Child0 {
                #[xelement(name = "a", namespace = "http://example.com")]
                pub a: i32,
                #[xelement(name = "b", namespace = "http://example.com")]
                pub b: String,
            }

            #[derive(
                ::core::fmt::Debug, ::xmlity::SerializationGroup, ::xmlity::DeserializationGroup
            )]
            #[xgroup(children_order = "strict")]
            pub struct SimpleSequence {
                pub child_0: Child0,
                #[xelement(name = "c", namespace = "http://example.com")]
                pub c: String,
            }
        }

        #[derive(::core::fmt::Debug, ::xmlity::Serialize, ::xmlity::Deserialize)]
        pub enum SimpleSequence {
            #[xelement(
                name = "SimpleSequence",
                namespace = "http://example.com",
                allow_unknown_attributes = "any"
            )]
            SimpleSequence(#[xgroup] simple_sequence_items::SimpleSequence),
            Dynamic(::xmlity_ns::SubstitutionGroup<test_ns::SimpleSequence>),
        }
    )
}

const TWO_ATTRIBUTE_TWO_CHILDREN_SEQUENCE_ELEMENT_INPUT: &str = r###"
<xs:schema xmlns:xs="http://www.w3.org/2001/XMLSchema"
           xmlns:ns="http://example.com"
           targetNamespace="http://example.com">
  <xs:element name="SimpleSequence">
    <xs:complexType>
      <xs:sequence>
        <xs:element name="a" type="xs:integer"/>
        <xs:element name="b" type="xs:string"/>
      </xs:sequence>
      <xs:attribute name="c" type="xs:integer" use="required"/>
      <xs:attribute name="d" type="xs:string" use="required"/>
    </xs:complexType>
  </xs:element>
</xs:schema>
"###;

#[rustfmt::skip]
fn two_attribute_two_children_sequence_element_expected() -> File {
    parse_quote!(
        pub mod simple_sequence_items {
            #[derive(
                ::core::fmt::Debug,
                ::xmlity::SerializationGroup,
                ::xmlity::DeserializationGroup
            )]
            #[xgroup(children_order = "strict")]
            pub struct SimpleSequence {
                #[xattribute(name = "c")]
                pub c: i32,
                #[xattribute(name = "d")]
                pub d: String,
                #[xelement(name = "a", namespace = "http://example.com")]
                pub a: i32,
                #[xelement(name = "b", namespace = "http://example.com")]
                pub b: String,
            }
        }

        #[derive(::core::fmt::Debug, ::xmlity::Serialize, ::xmlity::Deserialize)]
        pub enum SimpleSequence {
            #[xelement(
                name = "SimpleSequence",
                namespace = "http://example.com",
                allow_unknown_attributes = "any"
            )]
            SimpleSequence(#[xgroup] simple_sequence_items::SimpleSequence),
            Dynamic(
                ::xmlity_ns::SubstitutionGroup<test_ns::SimpleSequence>,
            ),
        }
    )
}

const SIMPLE_REFERENCE_TYPE_TOP_LEVEL_ELEMENT_INPUT: &str = r###"
<xs:schema xmlns:xs="http://www.w3.org/2001/XMLSchema"
           xmlns:ns="http://example.com"
           targetNamespace="http://example.com">
  <xs:element name="SimpleSequence" type="xs:string"/>
</xs:schema>
"###;

fn simple_reference_type_top_level_element_expected() -> File {
    parse_quote!(
        #[derive(::core::fmt::Debug, ::xmlity::Serialize, ::xmlity::Deserialize)]
        pub enum SimpleSequence {
            #[xelement(
                name = "SimpleSequence",
                namespace = "http://example.com",
                allow_unknown_attributes = "any"
            )]
            SimpleSequence(String),
            Dynamic(::xmlity_ns::SubstitutionGroup<test_ns::SimpleSequence>),
        }
    )
}

#[rstest::rstest]
#[case::empty_sequence_element(EMPTY_SEQUENCE_ELEMENT_INPUT, empty_sequence_element_expected(), ExpandedName::new(
    LocalName::new("SimpleSequence").unwrap(),
    Some(XmlNamespace::new("http://example.com").unwrap())
), syn::parse_quote!(SimpleSequence))]
#[case::two_child_sequence_element(
    TWO_CHILD_SEQUENCE_ELEMENT_INPUT,
    two_child_sequence_element_expected(),
    ExpandedName::new(
        LocalName::new("SimpleSequence").unwrap(),
        Some(XmlNamespace::new("http://example.com").unwrap())
    ),
    syn::parse_quote!(SimpleSequence)
)]
#[case::two_attribute_sequence_element(
    TWO_ATTRIBUTE_SEQUENCE_ELEMENT_INPUT,
    two_attribute_sequence_element_expected(),
    ExpandedName::new(
        LocalName::new("SimpleSequence").unwrap(),
        Some(XmlNamespace::new("http://example.com").unwrap())
    ),
    syn::parse_quote!(SimpleSequence)
)]
#[case::two_sequence_deep_element(
    TWO_SEQUENCE_DEEP_ELEMENT_INPUT,
    two_sequence_deep_element_expected(),
    ExpandedName::new(
        LocalName::new("SimpleSequence").unwrap(),
        Some(XmlNamespace::new("http://example.com").unwrap())
    ),
    syn::parse_quote!(SimpleSequence)
)]
#[case::two_attribute_two_children_sequence_element(
    TWO_ATTRIBUTE_TWO_CHILDREN_SEQUENCE_ELEMENT_INPUT,
    two_attribute_two_children_sequence_element_expected(),
    ExpandedName::new(
        LocalName::new("SimpleSequence").unwrap(),
        Some(XmlNamespace::new("http://example.com").unwrap())
    ),
    syn::parse_quote!(SimpleSequence)
)]
#[case::simple_reference_type_top_level_element(
    SIMPLE_REFERENCE_TYPE_TOP_LEVEL_ELEMENT_INPUT,
    simple_reference_type_top_level_element_expected(),
    ExpandedName::new(
        LocalName::new("SimpleSequence").unwrap(),
        Some(XmlNamespace::new("http://example.com").unwrap())
    ),
    syn::parse_quote!(SimpleSequence)
)]
fn element_generation(
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

    generator.bind_namespace_idx(id, parse_quote!(test_ns));

    generator.bind_types(xsd_codegen_xmlity::binds::StdXsdTypes);

    let (ty, items) = generator
        .generate_element(&id, &expanded_name)
        .expect("Failed to generate element");

    let actual = prettyplease::unparse(&syn::File {
        shebang: None,
        attrs: Vec::new(),
        items,
    });

    let expected_code = prettyplease::unparse(&expected_code);

    assert_eq!(actual, expected_code);

    assert_eq!(ty.into_type(None), expected_type);
}

#[test]
fn complex_reference_type_local_element() {
    let child_type_expanded_name = ExpandedName::new(
        LocalName::new("childType").unwrap(),
        XmlNamespace::XS.into(),
    );

    let schema_xml = r###"
    <xs:schema xmlns:xs="http://www.w3.org/2001/XMLSchema"
               xmlns:ns="http://example.com"
               targetNamespace="http://example.com">
      <xs:element name="SimpleSequence">
        <xs:complexType>
          <xs:complexContent>
            <xs:restriction base="xs:anyType">
              <xs:sequence>
                <xs:element name="a" type="xs:childType"/>
              </xs:sequence>
            </xs:restriction>
          </xs:complexContent>
        </xs:complexType>
      </xs:element>
    </xs:schema>
    "###;

    let schema: xs::Schema =
        xmlity_quick_xml::from_str(schema_xml.trim()).expect("Failed to parse XML Schema");

    let mut ctx = XmlnsContext::new();
    let (ns_id, _) = ctx
        .import_schema(
            FragmentedXsdDocumentKey(Url::from_str("http://example.com/test").unwrap()),
            &schema,
        )
        .unwrap();

    let mut generator = Generator::new(&ctx);

    generator.bind_namespace_idx(ns_id, parse_quote!(test_ns));

    generator.bind_type(
        child_type_expanded_name,
        BoundType {
            ty: TypeReference::new_static(parse_quote!(types::ChildType)),
            ty_type: TypeType::Complex,
            serialize_with: None,
            deserialize_with: None,
            default_with: None,
        },
    );

    let (type_, actual_items) = generator
        .generate_element(
            &ns_id,
            &ExpandedName::new(
                LocalName::new("SimpleSequence").unwrap(),
                Some(XmlNamespace::new("http://example.com").unwrap()),
            ),
        )
        .unwrap();

    let actual = prettyplease::unparse(&syn::File {
        shebang: None,
        attrs: Vec::new(),
        items: actual_items,
    });

    #[rustfmt::skip]
        let expected: syn::File = parse_quote!(
            pub mod simple_sequence_items {
                #[derive(
                    ::core::fmt::Debug,
                    ::xmlity::SerializationGroup,
                    ::xmlity::DeserializationGroup
                )]
                #[xgroup(children_order = "strict")]
                pub struct SimpleSequence {
                    #[xelement(name = "a", namespace = "http://example.com", group)]
                    pub a: types::ChildType,
                }
            }

            #[derive(::core::fmt::Debug, ::xmlity::Serialize, ::xmlity::Deserialize)]
            pub enum SimpleSequence {
                #[xelement(
                    name = "SimpleSequence",
                    namespace = "http://example.com",
                    allow_unknown_attributes = "any"
                )]
                SimpleSequence(#[xgroup] simple_sequence_items::SimpleSequence),
                Dynamic(
                    ::xmlity_ns::SubstitutionGroup<test_ns::SimpleSequence>,
                ),
            }
        );

    let expected = prettyplease::unparse(&expected);

    assert_eq!(actual, expected);
    assert_eq!(type_.into_type(None), parse_quote!(SimpleSequence));
}

#[test]
fn complex_reference_type_top_level_element() {
    let child_type_expanded_name = ExpandedName::new(
        LocalName::new("childType").unwrap(),
        XmlNamespace::XS.into(),
    );

    let schema = r###"
    <xs:schema xmlns:xs="http://www.w3.org/2001/XMLSchema"
               xmlns:ns="http://example.com"
               targetNamespace="http://example.com">
      <xs:element name="SimpleSequence" type="xs:childType"/>
    </xs:schema>
    "###;

    let schema: xs::Schema =
        xmlity_quick_xml::from_str(schema.trim()).expect("Failed to parse XML Schema");

    let mut ctx = XmlnsContext::new();
    let (ns_id, _) = ctx
        .import_schema(
            FragmentedXsdDocumentKey(Url::parse("http://example.com/test.xsd").unwrap()),
            &schema,
        )
        .unwrap();

    let mut generator = Generator::new(&ctx);

    generator.bind_namespace_idx(ns_id, parse_quote!(test_ns));

    generator.bind_type(
        child_type_expanded_name,
        BoundType {
            ty: TypeReference::new_static(parse_quote!(types::ChildType)),
            ty_type: TypeType::Complex,
            serialize_with: None,
            deserialize_with: None,
            default_with: None,
        },
    );

    let (type_, actual_items) = generator
        .generate_element(
            &ns_id,
            &ExpandedName::new(
                LocalName::new("SimpleSequence").unwrap(),
                Some(XmlNamespace::new("http://example.com").unwrap()),
            ),
        )
        .unwrap();

    let actual = prettyplease::unparse(&syn::File {
        shebang: None,
        attrs: Vec::new(),
        items: actual_items.clone(),
    });

    #[rustfmt::skip]
    let expected: syn::File = parse_quote!(
        #[derive(::core::fmt::Debug, ::xmlity::Serialize, ::xmlity::Deserialize)]
        pub enum SimpleSequence {
            #[xelement(name = "SimpleSequence", namespace = "http://example.com", allow_unknown_attributes = "any")]
            SimpleSequence(#[xgroup] types::ChildType),
            Dynamic(
                ::xmlity_ns::SubstitutionGroup<test_ns::SimpleSequence>,
            ),
        }
    );

    let expected = prettyplease::unparse(&expected);

    assert_eq!(actual, expected);

    assert_eq!(type_.into_type(None), parse_quote!(SimpleSequence));
}

#[test]
fn element_ref_element() {
    let child_element_expanded_name = ExpandedName::new(
        LocalName::new("ChildElement").unwrap(),
        XmlNamespace::XS.into(),
    );

    let schema = r###"
    <xs:schema xmlns:xs="http://www.w3.org/2001/XMLSchema"
               xmlns:ns="http://example.com"
               targetNamespace="http://example.com">
      <xs:element name="SimpleSequence">
       <xs:complexType>
         <xs:complexContent>
           <xs:restriction base="xs:anyType">
             <xs:sequence>
               <xs:element ref="xs:ChildElement"/>
             </xs:sequence>
           </xs:restriction>
         </xs:complexContent>
       </xs:complexType>
      </xs:element>
    </xs:schema>
    "###;

    let schema: xs::Schema =
        xmlity_quick_xml::from_str(schema.trim()).expect("Failed to parse XML Schema");

    let mut ctx = XmlnsContext::new();
    let (ns_id, _) = ctx
        .import_schema(
            FragmentedXsdDocumentKey(Url::parse("http://example.com/test.xsd").unwrap()),
            &schema,
        )
        .unwrap();

    let mut generator = Generator::new(&ctx);

    generator.bind_namespace_idx(ns_id, parse_quote!(test_ns));

    generator.bind_element(
        child_element_expanded_name,
        TypeReference::new_static(parse_quote!(types::ChildElement)),
    );

    let (type_, actual_items) = generator
        .generate_element(
            &ns_id,
            &ExpandedName::new(
                LocalName::new("SimpleSequence").unwrap(),
                Some(XmlNamespace::new("http://example.com").unwrap()),
            ),
        )
        .unwrap();

    let actual = prettyplease::unparse(&syn::File {
        shebang: None,
        attrs: Vec::new(),
        items: actual_items.clone(),
    });

    #[rustfmt::skip]
        let expected: syn::File = parse_quote!(
            pub mod simple_sequence_items {
                #[derive(
                    ::core::fmt::Debug,
                    ::xmlity::SerializationGroup,
                    ::xmlity::DeserializationGroup
                )]
                #[xgroup(children_order = "strict")]
                pub struct SimpleSequence {
                    pub child_element: types::ChildElement,
                }
            }

            #[derive(::core::fmt::Debug, ::xmlity::Serialize, ::xmlity::Deserialize)]
            pub enum SimpleSequence {
                #[xelement(
                    name = "SimpleSequence",
                    namespace = "http://example.com",
                    allow_unknown_attributes = "any"
                )]
                SimpleSequence(#[xgroup] simple_sequence_items::SimpleSequence),
                Dynamic(
                    ::xmlity_ns::SubstitutionGroup<test_ns::SimpleSequence>,
                ),
            }
        );

    let expected = prettyplease::unparse(&expected);

    assert_eq!(actual, expected);

    assert_eq!(type_.into_type(None), parse_quote!(SimpleSequence));
}
