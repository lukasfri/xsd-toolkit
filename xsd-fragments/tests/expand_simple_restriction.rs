//! Tests for the `ExpandSimpleRestriction` transformer in the `xsd-fragments` library.
use std::{collections::HashSet, str::FromStr};

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

#[rstest::rstest]
#[case::restrict_union_test_1(RESTRICT_UNION_TEST_1_INPUT, RESTRICT_UNION_TEST_1_EXPECTED)]
#[case::restrict_union_test_2(RESTRICT_UNION_TEST_2_INPUT, RESTRICT_UNION_TEST_2_EXPECTED)]
#[ignore = "Currently does not pass due to restriction expansion not being fully featured yet"]
fn expand_simple_restriction(#[case] input: &str, #[case] output: &str) {
    use xsd_fragments::transformers::TransformChange;

    let input: xs::Schema =
        xmlity_quick_xml::from_str(input.trim()).expect("Failed to parse XML Schema");

    let expected: xs::Schema =
        xmlity_quick_xml::from_str(output.trim()).expect("Failed to parse XML Schema");

    let allowed_bases: HashSet<ExpandedName<'static>> = [&xsn::NMTOKEN, &xsn::NON_NEGATIVE_INTEGER]
        .into_iter()
        .map(|name| (*name).clone())
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
