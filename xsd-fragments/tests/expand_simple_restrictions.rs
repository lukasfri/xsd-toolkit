//! Tests for the `ExpandSimpleRestriction` transformer in the `xsd-fragments` library.
use std::{collections::HashSet, ops::Deref, str::FromStr};

use xsd_fragments::{
    transformers::context::simple::ExpandSimpleRestriction, FragmentedXsdDocumentKey, XmlnsContext,
};

use pretty_assertions::assert_eq;
use url::Url;
use xmlity::ExpandedName;
use xsd::{xs, xsn};

const RESTRICT_UNION_TEST_1_INPUT: &str = r###"
<xs:schema xmlns:xs="http://www.w3.org/2001/XMLSchema">
  <xs:simpleType name="allNNI">
    <xs:union memberTypes="xs:nonNegativeInteger">
      <xs:simpleType>
        <xs:restriction base="xs:NMTOKEN">
          <xs:enumeration value="unbounded"/>
        </xs:restriction>
      </xs:simpleType>
    </xs:union>
  </xs:simpleType>
  <xs:simpleType name="allNNIRestriction">
    <xs:restriction base="allNNI">
      <xs:enumeration value="0"/>
      <xs:enumeration value="1"/>
    </xs:restriction>
  </xs:simpleType>
</xs:schema>
"###;

const RESTRICT_UNION_TEST_1_EXPECTED: &str = r###"
<xs:schema xmlns:xs="http://www.w3.org/2001/XMLSchema">
  <xs:simpleType name="allNNI">
    <xs:union memberTypes="xs:nonNegativeInteger">
      <xs:simpleType>
        <xs:restriction base="xs:NMTOKEN">
          <xs:enumeration value="unbounded"/>
        </xs:restriction>
      </xs:simpleType>
    </xs:union>
  </xs:simpleType>
  <xs:simpleType name="allNNIRestriction">
    <xs:restriction base="xs:nonNegativeInteger">
      <xs:enumeration value="0"/>
      <xs:enumeration value="1"/>
    </xs:restriction>
  </xs:simpleType>
</xs:schema>
"###;

const RESTRICT_UNION_TEST_2_INPUT: &str = r###"
<xs:schema xmlns:xs="http://www.w3.org/2001/XMLSchema">
  <xs:simpleType name="customAllNNI">
    <xs:union memberTypes="xs:nonNegativeInteger xs:float">
      <xs:simpleType>
        <xs:restriction base="xs:NMTOKEN">
          <xs:enumeration value="unbounded"/>
        </xs:restriction>
      </xs:simpleType>
    </xs:union>
  </xs:simpleType>
  <xs:simpleType name="allNNIRestriction">
    <xs:restriction base="customAllNNI">
      <xs:enumeration value="0"/>
      <xs:enumeration value="1"/>
      <xs:enumeration value="2.0"/>
    </xs:restriction>
  </xs:simpleType>
</xs:schema>
"###;

const RESTRICT_UNION_TEST_2_EXPECTED: &str = r###"
<xs:schema xmlns:xs="http://www.w3.org/2001/XMLSchema">
  <xs:simpleType name="customAllNNI">
    <xs:union memberTypes="xs:nonNegativeInteger xs:float">
      <xs:simpleType>
        <xs:restriction base="xs:NMTOKEN">
          <xs:enumeration value="unbounded"/>
        </xs:restriction>
      </xs:simpleType>
    </xs:union>
  </xs:simpleType>
  <xs:simpleType name="allNNIRestriction">
    <xs:union>
      <xs:simpleType>
        <xs:restriction base="xs:nonNegativeInteger">
          <xs:enumeration value="0"/>
          <xs:enumeration value="1"/>
        </xs:restriction>
      </xs:simpleType>
      <xs:simpleType>
        <xs:restriction base="xs:float">
          <xs:enumeration value="2.0"/>
        </xs:restriction>
      </xs:simpleType>
    </xs:union>
  </xs:simpleType>
</xs:schema>
"###;

const RESTRICT_SIMPLE_RESTRICTION_INPUT: &str = r###"
<xs:schema xmlns:xs="http://www.w3.org/2001/XMLSchema">
  <xs:simpleType name="DressSizeType">
    <xs:restriction base="xs:integer">
      <xs:minInclusive value="0"/>
      <xs:maxInclusive value="18"/>
    </xs:restriction>
  </xs:simpleType>
  <xs:simpleType name="DressSizeRestriction">
    <xs:restriction base="DressSizeType">
      <xs:maxInclusive value="10"/>
    </xs:restriction>
  </xs:simpleType>
</xs:schema>
"###;

const RESTRICT_SIMPLE_RESTRICTION_EXPECTED: &str = r###"
<xs:schema xmlns:xs="http://www.w3.org/2001/XMLSchema">
  <xs:simpleType name="DressSizeType">
    <xs:restriction base="xs:integer">
      <xs:minInclusive value="0"/>
      <xs:maxInclusive value="18"/>
    </xs:restriction>
  </xs:simpleType>
  <xs:simpleType name="DressSizeRestriction">
    <xs:restriction base="xs:integer">
      <xs:minInclusive value="0"/>
      <xs:maxInclusive value="10"/>
    </xs:restriction>
  </xs:simpleType>
</xs:schema>
"###;

#[rstest::rstest]
#[ignore = "Currently does not pass due to restriction expansion not being fully featured yet"]
#[case::restrict_union_test_1(RESTRICT_UNION_TEST_1_INPUT, RESTRICT_UNION_TEST_1_EXPECTED, &[&xsn::NMTOKEN, &xsn::NON_NEGATIVE_INTEGER, &xsn::INTEGER])]
#[ignore = "Currently does not pass due to restriction expansion not being fully featured yet"]
#[case::restrict_union_test_2(RESTRICT_UNION_TEST_2_INPUT, RESTRICT_UNION_TEST_2_EXPECTED, &[&xsn::NMTOKEN, &xsn::NON_NEGATIVE_INTEGER, &xsn::INTEGER])]
#[case::restrict_simple_restriction(
    RESTRICT_SIMPLE_RESTRICTION_INPUT,
    RESTRICT_SIMPLE_RESTRICTION_EXPECTED,
    &[&xsn::INTEGER]
)]
fn expand_simple_restriction<'a, T: Deref<Target = ExpandedName<'a>>>(
    #[case] input: &str,
    #[case] output: &str,
    #[case] allowed_bases: &[&T],
) {
    use xsd_fragments::transformers::TransformChange;

    let input: xs::Schema =
        xmlity_quick_xml::from_str(input.trim()).expect("Failed to parse XML Schema");

    let expected: xs::Schema =
        xmlity_quick_xml::from_str(output.trim()).expect("Failed to parse XML Schema");

    let allowed_bases: HashSet<ExpandedName<'a>> = allowed_bases
        .into_iter()
        .map(|name| ((**name).deref()).clone())
        .collect();

    let mut ctx = XmlnsContext::new();

    let (id, _) = ctx
        .import_schema(
            FragmentedXsdDocumentKey(Url::from_str("http://example.com/test").unwrap()),
            &input,
        )
        .expect("Failed to import schema");

    let changed = ctx
        .context_transform(ExpandSimpleRestriction::new(&allowed_bases))
        .unwrap();

    assert_eq!(changed, TransformChange::Changed);

    let actual = ctx.export_schema(&id).unwrap();

    assert_eq!(actual, expected);
}
