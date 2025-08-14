//! Tests for the `ExpandExtensionFragments` transformer in the `xsd-fragments` library.
use std::str::FromStr;

use xsd_fragments::{
    transformers::context::complex::ExpandExtensionFragments, FragmentedXsdDocumentKey,
    XmlnsContext,
};

use pretty_assertions::assert_eq;
use url::Url;
use xsd::xs;

const BASIC_EXTENSION_INPUT: &str = r###"
<xs:schema xmlns:xs="http://www.w3.org/2001/XMLSchema">
  <xs:complexType name="ProductType">
    <xs:complexContent>
      <xs:restriction base="xs:anyType">
        <xs:sequence>
          <xs:element name="number" type="xs:integer"/>
          <xs:element name="name" type="xs:string"/>
        </xs:sequence>
      </xs:restriction>
    </xs:complexContent>
  </xs:complexType>
  <xs:complexType name="ShirtType">
    <xs:complexContent>
      <xs:extension base="ProductType">
        <xs:sequence>
          <xs:element name="size" type="xs:string"/>
        </xs:sequence>
      </xs:extension>
    </xs:complexContent>
  </xs:complexType>
</xs:schema>
"###;

const BASIC_EXTENSION_EXPECTED: &str = r###"
<xs:schema xmlns:xs="http://www.w3.org/2001/XMLSchema">
  <xs:complexType name="ProductType">
    <xs:complexContent>
      <xs:restriction base="xs:anyType">
        <xs:sequence>
          <xs:element name="number" type="xs:integer"/>
          <xs:element name="name" type="xs:string"/>
        </xs:sequence>
      </xs:restriction>
    </xs:complexContent>
  </xs:complexType>
  <xs:complexType name="ShirtType">
    <xs:complexContent>
      <xs:restriction base="xs:anyType">
        <xs:sequence>
          <xs:sequence>
            <xs:element name="number" type="xs:integer"/>
            <xs:element name="name" type="xs:string"/>
          </xs:sequence>
          <xs:sequence>
            <xs:element name="size" type="xs:string"/>
          </xs:sequence>
        </xs:sequence>
      </xs:restriction>
    </xs:complexContent>
  </xs:complexType>
</xs:schema>
"###;

const ATTRIBUTE_EXTENSION_INPUT: &str = r###"
<xs:schema xmlns:xs="http://www.w3.org/2001/XMLSchema">
  <xs:complexType name="ProductType">
    <xs:complexContent>
      <xs:restriction base="xs:anyType">
        <xs:attribute name="id" type="xs:string" use="required"/>
      </xs:restriction>
    </xs:complexContent>
  </xs:complexType>
  <xs:complexType name="ShirtType">
    <xs:complexContent>
      <xs:extension base="ProductType">
        <xs:attribute name="size" type="xs:string" use="optional"/>
      </xs:extension>
    </xs:complexContent>
  </xs:complexType>
</xs:schema>
"###;

const ATTRIBUTE_EXTENSION_EXPECTED: &str = r###"
<xs:schema xmlns:xs="http://www.w3.org/2001/XMLSchema">
  <xs:complexType name="ProductType">
    <xs:complexContent>
      <xs:restriction base="xs:anyType">
        <xs:attribute name="id" type="xs:string" use="required"/>
      </xs:restriction>
    </xs:complexContent>
  </xs:complexType>
  <xs:complexType name="ShirtType">
    <xs:complexContent>
      <xs:restriction base="xs:anyType">
        <xs:attribute name="id" type="xs:string" use="required"/>
        <xs:attribute name="size" type="xs:string" use="optional"/>
      </xs:restriction>
    </xs:complexContent>
  </xs:complexType>
</xs:schema>
"###;

const EXPAND_EXTENSION_TYPE_ELEMENT_NO_FRAGMENT_INPUT: &str = r###"
<xs:schema xmlns:xs="http://www.w3.org/2001/XMLSchema">
  <xs:complexType name="Block">
    <xs:choice minOccurs="0" maxOccurs="unbounded">
      <xs:group ref="block"/>
      <xs:element ref="form"/>
      <xs:group ref="misc"/>
    </xs:choice>
  </xs:complexType>
  <xs:element name="noscript">
    <xs:complexType>
      <xs:complexContent>
        <xs:extension base="Block">
        </xs:extension>
      </xs:complexContent>
    </xs:complexType>
  </xs:element>
</xs:schema>
"###;

const EXPAND_EXTENSION_TYPE_ELEMENT_NO_FRAGMENT_EXPECTED: &str = r###"
<xs:schema xmlns:xs="http://www.w3.org/2001/XMLSchema">
  <xs:complexType name="Block">
    <xs:choice minOccurs="0" maxOccurs="unbounded">
      <xs:group ref="block"/>
      <xs:element ref="form"/>
      <xs:group ref="misc"/>
    </xs:choice>
  </xs:complexType>
  <xs:element name="noscript">
    <xs:complexType>
      <xs:complexContent>
        <xs:restriction base="xs:anyType">
          <xs:choice minOccurs="0" maxOccurs="unbounded">
            <xs:group ref="block"/>
            <xs:element ref="form"/>
            <xs:group ref="misc"/>
          </xs:choice>
        </xs:restriction>
      </xs:complexContent>
    </xs:complexType>
  </xs:element>
</xs:schema>
"###;

const EXPAND_HTMLX_A_INPUT: &str = r###"
<xs:schema xmlns:xs="http://www.w3.org/2001/XMLSchema">
  <xs:complexType name="a.content" mixed="true">
    <xs:choice minOccurs="0" maxOccurs="unbounded">
      <xs:group ref="special"/>
      <xs:group ref="fontstyle"/>
      <xs:group ref="phrase"/>
      <xs:group ref="inline.forms"/>
      <xs:group ref="misc.inline"/>
    </xs:choice>
  </xs:complexType>
  <xs:element name="a">
    <xs:complexType mixed="true">
      <xs:complexContent>
        <xs:extension base="a.content">
            <xs:attribute name="charset" type="Charset"/>
            <xs:attribute name="type" type="ContentType"/>
            <xs:attribute name="name" type="xs:NMTOKEN"/>
            <xs:attribute name="href" type="URI"/>
            <xs:attribute name="hreflang" type="LanguageCode"/>
            <xs:attribute name="rel" type="LinkTypes"/>
            <xs:attribute name="rev" type="LinkTypes"/>
            <xs:attribute name="shape" default="rect" type="Shape"/>
            <xs:attribute name="coords" type="Coords"/>
        </xs:extension>
      </xs:complexContent>
    </xs:complexType>
  </xs:element>
</xs:schema>
"###;

const EXPAND_HTMLX_A_EXPECTED: &str = r###"
<xs:schema xmlns:xs="http://www.w3.org/2001/XMLSchema">
  <xs:complexType name="a.content" mixed="true">
    <xs:choice minOccurs="0" maxOccurs="unbounded">
      <xs:group ref="special"/>
      <xs:group ref="fontstyle"/>
      <xs:group ref="phrase"/>
      <xs:group ref="inline.forms"/>
      <xs:group ref="misc.inline"/>
    </xs:choice>
  </xs:complexType>
  <xs:element name="a">
    <xs:complexType mixed="true">
      <xs:complexContent>
        <xs:restriction base="xs:anyType">
          <xs:choice minOccurs="0" maxOccurs="unbounded">
            <xs:group ref="special"/>
            <xs:group ref="fontstyle"/>
            <xs:group ref="phrase"/>
            <xs:group ref="inline.forms"/>
            <xs:group ref="misc.inline"/>
          </xs:choice>
          <xs:attribute name="charset" type="Charset"/>
          <xs:attribute name="type" type="ContentType"/>
          <xs:attribute name="name" type="xs:NMTOKEN"/>
          <xs:attribute name="href" type="URI"/>
          <xs:attribute name="hreflang" type="LanguageCode"/>
          <xs:attribute name="rel" type="LinkTypes"/>
          <xs:attribute name="rev" type="LinkTypes"/>
          <xs:attribute name="shape" default="rect" type="Shape"/>
          <xs:attribute name="coords" type="Coords"/>
        </xs:restriction>
      </xs:complexContent>
    </xs:complexType>
  </xs:element>
</xs:schema>
"###;

const EXPAND_REF_ATTRIBUTE_INPUT: &str = r###"
<xs:schema xmlns:xs="http://www.w3.org/2001/XMLSchema">
  <xs:complexType name="parent">
    <xs:complexContent>
      <xs:sequence/>
      <xs:attribute ref="attrs" use="required"/>
    </xs:complexContent>
  </xs:complexType>
  <xs:complexType name="child">
    <xs:complexContent>
      <xs:extension base="parent">
        <xs:sequence/>
        <xs:attribute ref="attrs" use="optional"/>
      </xs:extension>
    </xs:complexContent>
  </xs:complexType>
</xs:schema>
"###;

const EXPAND_REF_ATTRIBUTE_EXPECTED: &str = r###"
<xs:schema xmlns:xs="http://www.w3.org/2001/XMLSchema">
  <xs:complexType name="parent">
    <xs:complexContent>
      <xs:sequence/>
      <xs:attribute ref="attrs" use="required"/>
    </xs:complexContent>
  </xs:complexType>
  <xs:complexType name="child">
    <xs:complexContent>
      <xs:restriction base="xs:anyType">
        <xs:sequence/>
        <xs:attribute ref="attrs" use="optional"/>
      </xs:restriction>
    </xs:complexContent>
  </xs:complexType>
</xs:schema>
"###;

#[rstest::rstest]
#[case::basic_extension(BASIC_EXTENSION_INPUT, BASIC_EXTENSION_EXPECTED)]
#[case::attribute_extension(ATTRIBUTE_EXTENSION_INPUT, ATTRIBUTE_EXTENSION_EXPECTED)]
#[case::expand_extension_type_element_no_fragment(
    EXPAND_EXTENSION_TYPE_ELEMENT_NO_FRAGMENT_INPUT,
    EXPAND_EXTENSION_TYPE_ELEMENT_NO_FRAGMENT_EXPECTED
)]
#[case::expand_htmlx_a(EXPAND_HTMLX_A_INPUT, EXPAND_HTMLX_A_EXPECTED)]
#[case::expand_ref_attribute(EXPAND_REF_ATTRIBUTE_INPUT, EXPAND_REF_ATTRIBUTE_EXPECTED)]
fn expand_extension_fragments(#[case] input: &str, #[case] output: &str) {
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
        .context_transform(ExpandExtensionFragments::new())
        .unwrap();

    assert_eq!(changed, TransformChange::Changed);

    let actual = ctx.export_schema(&id).unwrap();

    assert_eq!(actual, expected);
}
