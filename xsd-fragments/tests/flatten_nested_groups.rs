//! Tests for the `FlattenNestedSequences` transformer in the `xsd-fragments` library.
use std::str::FromStr;

use xsd_fragments::{
    transformers::local::complex::FlattenNestedSequences, FragmentedXsdDocumentKey, XmlnsContext,
};

use pretty_assertions::assert_eq;
use url::Url;
use xsd::xs;

const FLATTEN_NESTED_SEQUENCES_INPUT: &str = r###"
<xs:schema xmlns:xs="http://www.w3.org/2001/XMLSchema">
  <xs:complexType name="ShirtType">
    <xs:complexContent>
      <xs:restriction base="xs:anyType">
        <xs:sequence>
          <xs:sequence>
            <xs:element name="number" type="xs:integer"/>
            <xs:element name="name" type="xs:string"/>
          </xs:sequence>
          <xs:choice maxOccurs="unbounded">
            <xs:element name="size" type="xs:integer"/>
            <xs:element name="color" type="xs:string"/>
          </xs:choice>
        </xs:sequence>
      </xs:restriction>
    </xs:complexContent>
  </xs:complexType>
</xs:schema>
"###;

const FLATTEN_NESTED_SEQUENCES_EXPECTED: &str = r###"
<xs:schema xmlns:xs="http://www.w3.org/2001/XMLSchema">
  <xs:complexType name="ShirtType">
    <xs:complexContent>
      <xs:restriction base="xs:anyType">
        <xs:sequence>
          <xs:element name="number" type="xs:integer"/>
          <xs:element name="name" type="xs:string"/>
          <xs:choice maxOccurs="unbounded">
            <xs:element name="size" type="xs:integer"/>
            <xs:element name="color" type="xs:string"/>
          </xs:choice>
        </xs:sequence>
      </xs:restriction>
    </xs:complexContent>
  </xs:complexType>
</xs:schema>
"###;

const DO_NOT_FLATTEN_SEQUENCES_WITH_OCCURS_INPUT: &str = r###"
<xs:schema xmlns:xs="http://www.w3.org/2001/XMLSchema">
  <xs:complexType name="ShirtType">
    <xs:complexContent>
      <xs:restriction base="xs:anyType">
        <xs:sequence>
          <xs:sequence maxOccurs="unbounded">
            <xs:element name="number" type="xs:integer"/>
            <xs:element name="name" type="xs:string"/>
          </xs:sequence>
          <xs:choice maxOccurs="unbounded">
            <xs:element name="size" type="xs:integer"/>
            <xs:element name="color" type="xs:string"/>
          </xs:choice>
        </xs:sequence>
      </xs:restriction>
    </xs:complexContent>
  </xs:complexType>
</xs:schema>
"###;

const DO_NOT_FLATTEN_SEQUENCES_WITH_OCCURS_EXPECTED: &str = r###"
<xs:schema xmlns:xs="http://www.w3.org/2001/XMLSchema">
  <xs:complexType name="ShirtType">
    <xs:complexContent>
      <xs:restriction base="xs:anyType">
        <xs:sequence>
          <xs:sequence maxOccurs="unbounded">
            <xs:element name="number" type="xs:integer"/>
            <xs:element name="name" type="xs:string"/>
          </xs:sequence>
          <xs:choice maxOccurs="unbounded">
            <xs:element name="size" type="xs:integer"/>
            <xs:element name="color" type="xs:string"/>
          </xs:choice>
        </xs:sequence>
      </xs:restriction>
    </xs:complexContent>
  </xs:complexType>
</xs:schema>
"###;

#[rstest::rstest]
#[case::flatten_nested_sequences(FLATTEN_NESTED_SEQUENCES_INPUT, FLATTEN_NESTED_SEQUENCES_EXPECTED)]
#[case::do_not_flatten_sequences_with_occurs(
    DO_NOT_FLATTEN_SEQUENCES_WITH_OCCURS_INPUT,
    DO_NOT_FLATTEN_SEQUENCES_WITH_OCCURS_EXPECTED
)]
fn flatten_nested_groups(#[case] input: &str, #[case] output: &str) {
    use xsd_fragments::transformers::TransformChange;

    let input: xs::Schema =
        xmlity_quick_xml::from_str(input.trim()).expect("Failed to parse XML Schema");

    let expected: xs::Schema =
        xmlity_quick_xml::from_str(output.trim()).expect("Failed to parse XML Schema");

    let mut ctx = XmlnsContext::new();

    let (id, _) = ctx
        .import_schema(
            FragmentedXsdDocumentKey(Url::from_str("http://example.com/test").unwrap()),
            &input,
        )
        .expect("Failed to import schema");

    let changed = ctx
        .local_transform_all(&FlattenNestedSequences::new())
        .unwrap();

    let expected_change = if input == expected {
        TransformChange::Unchanged
    } else {
        TransformChange::Changed
    };

    assert_eq!(changed, expected_change);

    let actual = ctx.export_schema(&id).unwrap();

    assert_eq!(actual, expected);
}
