//! Tests for the `ExpandShortFormComplexTypes` transformer in the `xsd-fragments` library.

use xsd_fragments::fragments::{complex::SchemaFragment, FragmentedXsdDocumentIdx};

use pretty_assertions::assert_eq;
use xsd::xs;

const SHORT_FORM_INPUT: &str = r###"
<xs:schema xmlns:xs="http://www.w3.org/2001/XMLSchema">
  <xs:complexType name="length">
    <xs:sequence>
      <xs:element name="size" type="xs:nonNegativeInteger"/>
      <xs:element name="unit" type="xs:NMTOKEN"/>
    </xs:sequence>
  </xs:complexType>
</xs:schema>
"###;

const SHORT_FORM_EXPECTED: &str = r###"
<xs:schema xmlns:xs="http://www.w3.org/2001/XMLSchema">
  <xs:complexType name="length">
    <xs:complexContent>
      <xs:restriction base="xs:anyType">
        <xs:sequence>
          <xs:element name="size" type="xs:nonNegativeInteger"/>
          <xs:element name="unit" type="xs:NMTOKEN"/>
        </xs:sequence>
      </xs:restriction>
    </xs:complexContent>
  </xs:complexType>
</xs:schema>
"###;

#[rstest::rstest]
#[case::short_form(SHORT_FORM_INPUT, SHORT_FORM_EXPECTED)]
fn specification_1(#[case] input: &str, #[case] output: &str) {
    use xsd_fragments::transformers::local::complex::ExpandShortFormComplexTypes;

    let input: xs::Schema =
        xmlity_quick_xml::from_str(input.trim()).expect("Failed to parse XML Schema");

    let expected: xs::Schema =
        xmlity_quick_xml::from_str(output.trim()).expect("Failed to parse XML Schema");

    let mut schema = SchemaFragment::from_schema(&input, FragmentedXsdDocumentIdx::new(0)).unwrap();

    schema
        .transform(ExpandShortFormComplexTypes::new())
        .unwrap();

    let actual = schema.to_schema().unwrap();

    assert_eq!(actual, expected);
}
