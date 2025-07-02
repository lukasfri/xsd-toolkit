use super::xs;
use xmlity::{ExpandedName, LocalName, XmlNamespace};

#[rstest::rstest]
#[case::occurs(XSD_OCCURS, None)]
#[case::def_ref(XSD_DEF_REF, None)]
#[case::any_attr_group(XSD_ANY_ATTR_GROUP, Some(xsd_any_attr_group()))]
#[ntest::timeout(1000)]
fn deserialize(#[case] xml: &str, #[case] expected: Option<xs::AttributeGroup>) {
    let xml = xml.trim();
    let element: xs::AttributeGroup = xmlity_quick_xml::de::from_str(xml).unwrap();

    if let Some(expected) = expected {
        pretty_assertions::assert_eq!(element, expected);
    }
}

const XSD_OCCURS: &str = r###"
<xs:attributeGroup xmlns:xs="http://www.w3.org/2001/XMLSchema" name="occurs">
  <xs:annotation>
    <xs:documentation>
  for all particles</xs:documentation>
  </xs:annotation>
  <xs:attribute name="minOccurs" type="xs:nonNegativeInteger" default="1"
                use="optional"/>
  <xs:attribute name="maxOccurs" type="xs:allNNI" default="1" use="optional"/>
</xs:attributeGroup>
"###;

const XSD_DEF_REF: &str = r###"
<xs:attributeGroup xmlns:xs="http://www.w3.org/2001/XMLSchema" name="defRef">
  <xs:annotation>
    <xs:documentation>
  for element, group and attributeGroup,
  which both define and reference</xs:documentation>
  </xs:annotation>
  <xs:attribute name="name" type="xs:NCName"/>
  <xs:attribute name="ref" type="xs:QName"/>
</xs:attributeGroup>
"###;

const XSD_ANY_ATTR_GROUP: &str = r###"
<xs:attributeGroup xmlns:xs="http://www.w3.org/2001/XMLSchema" name="anyAttrGroup">
  <xs:attribute name="namespace" type="xs:namespaceList"
                use="optional"/>
  <xs:attribute name="notNamespace" use="optional">
    <xs:simpleType>
      <xs:restriction base="xs:basicNamespaceList">
        <xs:minLength value="1"/>
      </xs:restriction>
    </xs:simpleType>
  </xs:attribute>
  <xs:attribute name="processContents" default="strict" use="optional">
    <xs:simpleType>
      <xs:restriction base="xs:NMTOKEN">
        <xs:enumeration value="skip"/>
        <xs:enumeration value="lax"/>
        <xs:enumeration value="strict"/>
      </xs:restriction>
    </xs:simpleType>
  </xs:attribute>
</xs:attributeGroup>
"###;

fn xsd_any_attr_group() -> xs::AttributeGroup {
    xs::AttributeGroup(Box::new(
        xs::types::NamedAttributeGroup::builder()
            .name(LocalName::new_dangerous("anyAttrGroup"))
            .attr_decls(Box::new(
                xs::groups::AttrDecls::builder()
                    .attribute(vec![
                        xs::types::Attribute::builder()
                            .name(LocalName::new_dangerous("namespace"))
                            .type_(xs::types::QName(ExpandedName::new(
                                LocalName::new_dangerous("namespaceList"),
                                Some(XmlNamespace::XS),
                            )))
                            .use_(xs::types::attribute_items::UseValue::Optional)
                            .build()
                            .into(),
                        xs::types::Attribute::builder()
                            .name(LocalName::new_dangerous("notNamespace"))
                            .use_(xs::types::attribute_items::UseValue::Optional)
                            .simple_type(
                                xs::types::LocalSimpleType::builder()
                                    .simple_derivation(Box::new(
                                        xs::Restriction::builder()
                                            .base(xs::types::QName(ExpandedName::new(
                                                LocalName::new_dangerous("basicNamespaceList"),
                                                Some(XmlNamespace::XS),
                                            )))
                                            .simple_restriction_model(
                                                xs::groups::SimpleRestrictionModel::builder()
                                                    .child_1(vec![xs::Facet::from(xs::MinLength(
                                                        xs::types::NumFacet::builder()
                                                            .value(1)
                                                            .build()
                                                            .into(),
                                                    ))
                                                    .into()])
                                                    .build(),
                                            )
                                            .build()
                                            .into(),
                                    ))
                                    .build()
                                    .into(),
                            )
                            .build()
                            .into(),
                        xs::types::Attribute::builder()
                            .name(LocalName::new_dangerous("processContents"))
                            .default("strict".to_string())
                            .use_(xs::types::attribute_items::UseValue::Optional)
                            .simple_type(
                                xs::types::LocalSimpleType::builder()
                                    .simple_derivation(Box::new(
                                        xs::Restriction::builder()
                                            .base(xs::types::QName(ExpandedName::new(
                                                LocalName::new_dangerous("NMTOKEN"),
                                                Some(XmlNamespace::XS),
                                            )))
                                            .simple_restriction_model(
                                                xs::groups::SimpleRestrictionModel::builder()
                                                    .child_1(vec![
                                                        xs::Facet::from(xs::Enumeration(
                                                            xs::types::NoFixedFacet::builder()
                                                                .value("skip".to_string())
                                                                .build()
                                                                .into(),
                                                        ))
                                                        .into(),
                                                        xs::Facet::from(xs::Enumeration(
                                                            xs::types::NoFixedFacet::builder()
                                                                .value("lax".to_string())
                                                                .build()
                                                                .into(),
                                                        ))
                                                        .into(),
                                                        xs::Facet::from(xs::Enumeration(
                                                            xs::types::NoFixedFacet::builder()
                                                                .value("strict".to_string())
                                                                .build()
                                                                .into(),
                                                        ))
                                                        .into(),
                                                    ])
                                                    .build(),
                                            )
                                            .build()
                                            .into(),
                                    ))
                                    .build()
                                    .into(),
                            )
                            .build()
                            .into(),
                    ])
                    .build(),
            ))
            .build(),
    ))
}
