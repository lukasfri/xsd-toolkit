//! Tests for the `ExpandRestrictionFragments` transformer in the `xsd-fragments` library.
use std::str::FromStr;

use xsd_fragments::{
    transformers::context::complex::ExpandRestrictionFragments, FragmentedXsdDocumentKey,
    XmlnsContext,
};

use pretty_assertions::assert_eq;
use url::Url;
use xsd::xs;

const BASIC_CHILD_ONLY_EXPAND_RESTRICTION_INPUT: &str = r###"
<xs:schema xmlns:xs="http://www.w3.org/2001/XMLSchema" targetNamespace="http://localhost" xmlns="http://localhost">
  <xs:complexType name="ProductType">
    <xs:complexContent>
      <xs:restriction base="xs:anyType">
        <xs:sequence>
          <xs:element name="number" type="xs:integer" />
          <xs:element name="name" type="xs:string" />
        </xs:sequence>
      </xs:restriction>
    </xs:complexContent>
  </xs:complexType>
  <xs:complexType name="ShirtType">
    <xs:complexContent>
      <xs:restriction base="ProductType">
        <xs:sequence>
          <xs:element name="number" type="xs:integer" />
        </xs:sequence>
      </xs:restriction>
    </xs:complexContent>
  </xs:complexType>
</xs:schema>
"###;

const BASIC_CHILD_ONLY_EXPAND_RESTRICTION_EXPECTED: &str = r###"
<xs:schema xmlns:xs="http://www.w3.org/2001/XMLSchema" targetNamespace="http://localhost" xmlns="http://localhost">
  <xs:complexType name="ProductType">
    <xs:complexContent>
      <xs:restriction base="xs:anyType">
        <xs:sequence>
          <xs:element name="number" type="xs:integer" />
          <xs:element name="name" type="xs:string" />
        </xs:sequence>
      </xs:restriction>
    </xs:complexContent>
  </xs:complexType>
  <xs:complexType name="ShirtType">
    <xs:complexContent>
      <xs:restriction base="xs:anyType">
        <xs:sequence>
          <xs:element name="number" type="xs:integer" />
        </xs:sequence>
      </xs:restriction>
    </xs:complexContent>
  </xs:complexType>
</xs:schema>
"###;

const BASIC_ATTRIBUTE_ONLY_EXPAND_RESTRICTION_INPUT: &str = r###"
<xs:schema xmlns:xs="http://www.w3.org/2001/XMLSchema" targetNamespace="http://localhost" xmlns="http://localhost">
  <xs:complexType name="ProductType">
    <xs:complexContent>
      <xs:restriction base="xs:anyType">
        <xs:attribute name="number" type="xs:integer" use="optional" />
        <xs:attribute name="name" type="xs:string" use="required" />
      </xs:restriction>
    </xs:complexContent>
  </xs:complexType>
  <xs:complexType name="ShirtType">
    <xs:complexContent>
      <xs:restriction base="ProductType">
        <xs:attribute name="number" type="xs:integer" use="required" />
      </xs:restriction>
    </xs:complexContent>
  </xs:complexType>
</xs:schema>
"###;

const BASIC_ATTRIBUTE_ONLY_EXPAND_RESTRICTION_EXPECTED: &str = r###"
<xs:schema xmlns:xs="http://www.w3.org/2001/XMLSchema" targetNamespace="http://localhost" xmlns="http://localhost">
  <xs:complexType name="ProductType">
    <xs:complexContent>
      <xs:restriction base="xs:anyType">
        <xs:attribute name="number" type="xs:integer" use="optional" />
        <xs:attribute name="name" type="xs:string" use="required" />
      </xs:restriction>
    </xs:complexContent>
  </xs:complexType>
  <xs:complexType name="ShirtType">
    <xs:complexContent>
      <xs:restriction base="xs:anyType">
        <xs:attribute name="number" type="xs:integer" use="required" />
        <xs:attribute name="name" type="xs:string" use="required" />
      </xs:restriction>
    </xs:complexContent>
  </xs:complexType>
</xs:schema>
"###;

#[rstest::rstest]
#[case::basic_child_only_expand_restriction(
    BASIC_CHILD_ONLY_EXPAND_RESTRICTION_INPUT,
    BASIC_CHILD_ONLY_EXPAND_RESTRICTION_EXPECTED
)]
#[case::basic_attribute_only_expand_restriction(
    BASIC_ATTRIBUTE_ONLY_EXPAND_RESTRICTION_INPUT,
    BASIC_ATTRIBUTE_ONLY_EXPAND_RESTRICTION_EXPECTED
)]
fn expand_restriction_fragments(#[case] input: &str, #[case] output: &str) {
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
        .context_transform(ExpandRestrictionFragments::new())
        .unwrap();

    assert_eq!(changed, TransformChange::Changed);

    let changed = ctx
        .context_transform(ExpandRestrictionFragments::new())
        .unwrap();

    assert_eq!(changed, TransformChange::Unchanged);

    let actual = ctx.export_schema(&id).unwrap();

    assert_eq!(actual, expected);
}
