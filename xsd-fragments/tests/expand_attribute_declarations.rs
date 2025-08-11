//! Tests for the `ExpandAttributeDeclarations` transformer in the `xsd-fragments` library.
use std::str::FromStr;

use xsd_fragments::{
    transformers::{context::complex::ExpandAttributeDeclarations, TransformChange},
    FragmentedXsdDocumentKey, XmlnsContext,
};

use pretty_assertions::assert_eq;
use url::Url;
use xsd::xs;

const ONE_ATTRIBUTE_GROUP_INPUT: &str = r###"
<xs:schema xmlns:xs="http://www.w3.org/2001/XMLSchema">
  <xs:attributeGroup name="test-attr-group">
    <xs:attribute name="test-attr" type="xs:string"/>
  </xs:attributeGroup>
  <xs:complexType name="test">
    <xs:complexContent>
      <xs:restriction base="xs:anyType">
        <xs:attributeGroup ref="test-attr-group"/>
      </xs:restriction>
    </xs:complexContent>
  </xs:complexType>
</xs:schema>
"###;

const ONE_ATTRIBUTE_GROUP_EXPECTED: &str = r###"
<xs:schema xmlns:xs="http://www.w3.org/2001/XMLSchema">
  <xs:attributeGroup name="test-attr-group">
    <xs:attribute name="test-attr" type="xs:string"/>
  </xs:attributeGroup>
  <xs:complexType name="test">
    <xs:complexContent>
      <xs:restriction base="xs:anyType">
        <xs:attribute name="test-attr" type="xs:string"/>
      </xs:restriction>
    </xs:complexContent>
  </xs:complexType>
</xs:schema>
"###;

const TWO_ATTRIBUTE_GROUP_INPUT: &str = r###"
<xs:schema xmlns:xs="http://www.w3.org/2001/XMLSchema">
  <xs:attributeGroup name="test-attr-group">
    <xs:attribute name="test-attr" type="xs:string"/>
  </xs:attributeGroup>
  <xs:complexType name="test">
    <xs:complexContent>
      <xs:restriction base="xs:anyType">
        <xs:attributeGroup ref="test-attr-group"/>
        <xs:attribute name="test-attr2" type="xs:string"/>
      </xs:restriction>
    </xs:complexContent>
  </xs:complexType>
</xs:schema>
"###;

const TWO_ATTRIBUTE_GROUP_EXPECTED: &str = r###"
<xs:schema xmlns:xs="http://www.w3.org/2001/XMLSchema">
  <xs:attributeGroup name="test-attr-group">
    <xs:attribute name="test-attr" type="xs:string"/>
  </xs:attributeGroup>
  <xs:complexType name="test">
    <xs:complexContent>
      <xs:restriction base="xs:anyType">
        <xs:attribute name="test-attr" type="xs:string"/>
        <xs:attribute name="test-attr2" type="xs:string"/>
      </xs:restriction>
    </xs:complexContent>
  </xs:complexType>
</xs:schema>
"###;

const SAME_ATTRIBUTE_OVERWRITES_VALUES_1_INPUT: &str = r###"
<xs:schema xmlns:xs="http://www.w3.org/2001/XMLSchema">
  <xs:attributeGroup name="test-attr-group">
    <xs:attribute name="test-attr" type="xs:string" use="prohibited"/>
  </xs:attributeGroup>
  <xs:complexType name="test">
    <xs:complexContent>
      <xs:restriction base="xs:anyType">
        <xs:attributeGroup ref="test-attr-group"/>
        <xs:attribute name="test-attr" type="xs:string" use="optional"/>
      </xs:restriction>
    </xs:complexContent>
  </xs:complexType>
</xs:schema>
"###;

const SAME_ATTRIBUTE_OVERWRITES_VALUES_1_EXPECTED: &str = r###"
<xs:schema xmlns:xs="http://www.w3.org/2001/XMLSchema">
  <xs:attributeGroup name="test-attr-group">
    <xs:attribute name="test-attr" type="xs:string" use="prohibited"/>
  </xs:attributeGroup>
  <xs:complexType name="test">
    <xs:complexContent>
      <xs:restriction base="xs:anyType">
        <xs:attribute name="test-attr" type="xs:string" use="optional"/>
      </xs:restriction>
    </xs:complexContent>
  </xs:complexType>
</xs:schema>
"###;

const SAME_ATTRIBUTE_OVERWRITES_VALUES_2_INPUT: &str = r###"
<xs:schema xmlns:xs="http://www.w3.org/2001/XMLSchema">
  <xs:attributeGroup name="test-attr-group">
    <xs:attribute name="test-attr" type="xs:string" use="prohibited"/>
  </xs:attributeGroup>
  <xs:complexType name="test">
    <xs:complexContent>
      <xs:restriction base="xs:anyType">
        <xs:attribute name="test-attr" type="xs:string" use="optional"/>
        <xs:attributeGroup ref="test-attr-group"/>
      </xs:restriction>
    </xs:complexContent>
  </xs:complexType>
</xs:schema>
"###;

const SAME_ATTRIBUTE_OVERWRITES_VALUES_2_EXPECTED: &str = r###"
<xs:schema xmlns:xs="http://www.w3.org/2001/XMLSchema">
  <xs:attributeGroup name="test-attr-group">
    <xs:attribute name="test-attr" type="xs:string" use="prohibited"/>
  </xs:attributeGroup>
  <xs:complexType name="test">
    <xs:complexContent>
      <xs:restriction base="xs:anyType">
        <xs:attribute name="test-attr" type="xs:string" use="prohibited"/>
      </xs:restriction>
    </xs:complexContent>
  </xs:complexType>
</xs:schema>
"###;

#[rstest::rstest]
#[case::one_attribute_group(ONE_ATTRIBUTE_GROUP_INPUT, ONE_ATTRIBUTE_GROUP_EXPECTED)]
#[case::two_attribute_groups(TWO_ATTRIBUTE_GROUP_INPUT, TWO_ATTRIBUTE_GROUP_EXPECTED)]
#[case::same_attribute_overwrites_values_1(
    SAME_ATTRIBUTE_OVERWRITES_VALUES_1_INPUT,
    SAME_ATTRIBUTE_OVERWRITES_VALUES_1_EXPECTED
)]
#[case::same_attribute_overwrites_values_2(
    SAME_ATTRIBUTE_OVERWRITES_VALUES_2_INPUT,
    SAME_ATTRIBUTE_OVERWRITES_VALUES_2_EXPECTED
)]
fn one_attribute_group(#[case] input: &str, #[case] output: &str) {
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
        .context_transform(ExpandAttributeDeclarations::new())
        .unwrap();

    assert_eq!(changed, TransformChange::Changed);

    let actual = ctx.export_schema(&id).unwrap();

    assert_eq!(actual, expected);
}
