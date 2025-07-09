use xmlity::{ExpandedName, LocalName, XmlNamespace, XmlValue};
use xmlity_ns_xs as xs;

#[rstest::rstest]
#[case::open_attrs(XSD_OPEN_ATTRS, None)]
#[case::annotated(XSD_ANNOTATED, None)]
#[case::attribute(XSD_ATTRIBUTE, None)]
#[case::top_level_attribute(XSD_TOP_LEVEL_ATTRIBUTE, None)]
#[case::assertion(XSD_ASSERTION, None)]
#[case::complex_base_type(XSD_COMPLEX_BASE_TYPE, None)]
#[case::top_level_complex_type(XSD_TOP_LEVEL_COMPLEX_TYPE, None)]
#[case::local_complex_type(XSD_LOCAL_COMPLEX_TYPE, None)]
#[case::restriction_type(XSD_RESTRICTION_TYPE, None)]
#[case::complex_restriction_type(XSD_COMPLEX_RESTRICTION_TYPE, None)]
#[case::extension_type(XSD_EXTENSION_TYPE, None)]
#[case::simple_restriction_type(XSD_SIMPLE_RESTRICTION_TYPE, None)]
#[case::simple_extension_type(XSD_SIMPLE_EXTENSION_TYPE, None)]
#[case::element(XSD_ELEMENT, None)]
#[case::top_level_element(XSD_TOP_LEVEL_ELEMENT, None)]
#[case::local_element(XSD_LOCAL_ELEMENT, None)]
#[case::alt_type(XSD_ALT_TYPE, None)]
#[case::group(XSD_GROUP, None)]
#[case::real_group(XSD_REAL_GROUP, None)]
#[case::named_group(XSD_NAMED_GROUP, None)]
#[case::group_ref(XSD_GROUP_REF, None)]
#[case::explicit_group(XSD_EXPLICIT_GROUP, None)]
#[case::simple_explicit_group(XSD_SIMPLE_EXPLICIT_GROUP, None)]
#[case::all(XSD_ALL, Some(xsd_all()))]
#[case::wildcard(XSD_WILDCARD, None)]
#[case::attribute_group(XSD_ATTRIBUTE_GROUP, None)]
#[case::named_attribute_group(XSD_NAMED_ATTRIBUTE_GROUP, None)]
#[case::attribute_group_ref(XSD_ATTRIBUTE_GROUP_REF, None)]
#[case::keybase(XSD_KEYBASE, None)]
#[case::any_type(XSD_ANY_TYPE, Some(xsd_any_type()))]
#[case::simple_base_type(XSD_SIMPLE_BASE_TYPE, None)]
#[case::top_level_simple_type(XSD_TOP_LEVEL_SIMPLE_TYPE, None)]
#[case::local_simple_type(XSD_LOCAL_SIMPLE_TYPE, None)]
#[case::facet(XSD_FACET, None)]
#[case::no_fixed_facet(XSD_NO_FIXED_FACET, None)]
#[case::num_facet(XSD_NUM_FACET, None)]
#[case::int_facet(XSD_INT_FACET, None)]
#[ntest::timeout(1000)]
fn deserialize(#[case] xml: &str, #[case] expected: Option<xs::ComplexType>) {
    let xml = xml.trim();
    let element: xs::ComplexType = xmlity_quick_xml::de::from_str(xml).unwrap();

    if let Some(expected) = expected {
        pretty_assertions::assert_eq!(element, expected);
    }
}

const XSD_OPEN_ATTRS: &str = r###"
<xs:complexType xmlns:xs="http://www.w3.org/2001/XMLSchema" name="openAttrs">
  <xs:annotation>
    <xs:documentation>
      This type is extended by almost all schema types
      to allow attributes from other namespaces to be
      added to user schemas.
    </xs:documentation>
  </xs:annotation>
  <xs:complexContent>
    <xs:restriction base="xs:anyType">
      <xs:anyAttribute namespace="##other" processContents="lax"/>
    </xs:restriction>
  </xs:complexContent>
</xs:complexType>
"###;

const XSD_ANNOTATED: &str = r###"
<xs:complexType xmlns:xs="http://www.w3.org/2001/XMLSchema" name="annotated">
  <xs:annotation>
    <xs:documentation>
      This type is extended by all types which allow annotation
      other than &lt;schema> itself
    </xs:documentation>
  </xs:annotation>
  <xs:complexContent>
    <xs:extension base="xs:openAttrs">
      <xs:sequence>
        <xs:element ref="xs:annotation" minOccurs="0"/>
      </xs:sequence>
      <xs:attribute name="id" type="xs:ID"/>
    </xs:extension>
  </xs:complexContent>
</xs:complexType>
"###;

const XSD_ATTRIBUTE: &str = r###"
<xs:complexType xmlns:xs="http://www.w3.org/2001/XMLSchema" name="attribute">
  <xs:complexContent>
    <xs:extension base="xs:annotated">
      <xs:sequence>
        <xs:element name="simpleType" type="xs:localSimpleType" minOccurs="0"/>
      </xs:sequence>
      <xs:attributeGroup ref="xs:defRef"/>
      <xs:attribute name="type" type="xs:QName"/>
      <xs:attribute name="use" default="optional" use="optional">
        <xs:simpleType>
          <xs:restriction base="xs:NMTOKEN">
            <xs:enumeration value="prohibited"/>
            <xs:enumeration value="optional"/>
            <xs:enumeration value="required"/>
          </xs:restriction>
        </xs:simpleType>
      </xs:attribute>
      <xs:attribute name="default" type="xs:string"/>
      <xs:attribute name="fixed" type="xs:string"/>
      <xs:attribute name="form" type="xs:formChoice"/>
      <xs:attribute name="targetNamespace" type="xs:anyURI"/>

      <xs:attribute name="inheritable" type="xs:boolean"/>
    </xs:extension>
  </xs:complexContent>
</xs:complexType>
"###;

const XSD_TOP_LEVEL_ATTRIBUTE: &str = r###"
<xs:complexType xmlns:xs="http://www.w3.org/2001/XMLSchema" name="topLevelAttribute">
  <xs:complexContent>
    <xs:restriction base="xs:attribute">
      <xs:sequence>
        <xs:element ref="xs:annotation" minOccurs="0"/>
        <xs:element name="simpleType" type="xs:localSimpleType" minOccurs="0"/>
      </xs:sequence>
      <xs:attribute name="ref" use="prohibited"/>
      <xs:attribute name="form" use="prohibited"/>
      <xs:attribute name="use" use="prohibited"/>
      <xs:attribute name="targetNamespace" use="prohibited"/>
      <xs:attribute name="name" type="xs:NCName" use="required"/>
      <xs:attribute name="inheritable" type="xs:boolean"/>
      <xs:anyAttribute namespace="##other" processContents="lax"/>
    </xs:restriction>
  </xs:complexContent>
</xs:complexType>
"###;

const XSD_ASSERTION: &str = r###"
<xs:complexType xmlns:xs="http://www.w3.org/2001/XMLSchema" name="assertion">
  <xs:complexContent>
    <xs:extension base="xs:annotated">
      <xs:attribute name="test" type="xs:string"/>
      <xs:attribute name="xpathDefaultNamespace" type="xs:xpathDefaultNamespace"/>
    </xs:extension>
  </xs:complexContent>
</xs:complexType>
"###;

const XSD_COMPLEX_BASE_TYPE: &str = r###"
<xs:complexType xmlns:xs="http://www.w3.org/2001/XMLSchema" name="complexBaseType" abstract="true">
  <xs:complexContent>
    <xs:extension base="xs:annotated">
      <xs:group ref="xs:complexTypeModel"/>
      <xs:attribute name="name" type="xs:NCName">
        <xs:annotation>
          <xs:documentation>
    Will be restricted to required or prohibited</xs:documentation>
        </xs:annotation>
      </xs:attribute>
      <xs:attribute name="mixed" type="xs:boolean" use="optional">
        <xs:annotation>
          <xs:documentation>
    Not allowed if simpleContent child is chosen.
    May be overridden by setting on complexContent child.</xs:documentation>
        </xs:annotation>
      </xs:attribute>
      <xs:attribute name="abstract" type="xs:boolean" default="false"
                    use="optional"/>
      <xs:attribute name="final" type="xs:derivationSet"/>
      <xs:attribute name="block" type="xs:derivationSet"/>
      <xs:attribute name="defaultAttributesApply" type="xs:boolean"
                    default="true" use="optional"/>
    </xs:extension>
  </xs:complexContent>
</xs:complexType>
"###;

const XSD_TOP_LEVEL_COMPLEX_TYPE: &str = r###"
<xs:complexType xmlns:xs="http://www.w3.org/2001/XMLSchema" name="topLevelComplexType">
  <xs:complexContent>
    <xs:restriction base="xs:complexBaseType">
      <xs:sequence>
        <xs:element ref="xs:annotation" minOccurs="0"/>
        <xs:group ref="xs:complexTypeModel"/>
      </xs:sequence>
      <xs:attribute name="name" type="xs:NCName" use="required"/>
      <xs:anyAttribute namespace="##other" processContents="lax"/>
    </xs:restriction>
  </xs:complexContent>
</xs:complexType>
"###;

const XSD_LOCAL_COMPLEX_TYPE: &str = r###"
<xs:complexType xmlns:xs="http://www.w3.org/2001/XMLSchema" name="localComplexType">
  <xs:complexContent>
    <xs:restriction base="xs:complexBaseType">
      <xs:sequence>
        <xs:element ref="xs:annotation" minOccurs="0"/>
        <xs:group ref="xs:complexTypeModel"/>
      </xs:sequence>
      <xs:attribute name="name" use="prohibited"/>
      <xs:attribute name="abstract" use="prohibited"/>
      <xs:attribute name="final" use="prohibited"/>
      <xs:attribute name="block" use="prohibited"/>
      <xs:anyAttribute namespace="##other" processContents="lax"/>
    </xs:restriction>
  </xs:complexContent>
</xs:complexType>
"###;

const XSD_RESTRICTION_TYPE: &str = r###"
<xs:complexType xmlns:xs="http://www.w3.org/2001/XMLSchema" name="restrictionType">
  <xs:complexContent>
    <xs:extension base="xs:annotated">
      <xs:sequence>
        <xs:choice minOccurs="0">
          <xs:sequence>
            <xs:element ref="xs:openContent" minOccurs="0"/>
            <xs:group ref="xs:typeDefParticle"/>
          </xs:sequence>
          <xs:group ref="xs:simpleRestrictionModel"/>
        </xs:choice>
        <xs:group ref="xs:attrDecls"/>
        <xs:group ref="xs:assertions"/>
      </xs:sequence>
      <xs:attribute name="base" type="xs:QName" use="required"/>
    </xs:extension>
  </xs:complexContent>
</xs:complexType>
"###;

const XSD_COMPLEX_RESTRICTION_TYPE: &str = r###"
<xs:complexType xmlns:xs="http://www.w3.org/2001/XMLSchema" name="complexRestrictionType">
  <xs:complexContent>
    <xs:restriction base="xs:restrictionType">
      <xs:sequence>
        <xs:element ref="xs:annotation" minOccurs="0"/>
        <xs:choice minOccurs="0">
          <xs:annotation>
            <xs:documentation>This choice is added simply to
                  make this a valid restriction per the REC</xs:documentation>
          </xs:annotation>

          <xs:sequence>
            <xs:element ref="xs:openContent" minOccurs="0"/>
            <xs:group ref="xs:typeDefParticle"/>
          </xs:sequence>
        </xs:choice>
        <xs:group ref="xs:attrDecls"/>
        <xs:group ref="xs:assertions"/>
      </xs:sequence>
      <xs:anyAttribute namespace="##other" processContents="lax"/>
    </xs:restriction>
  </xs:complexContent>
</xs:complexType>
"###;

const XSD_EXTENSION_TYPE: &str = r###"
<xs:complexType xmlns:xs="http://www.w3.org/2001/XMLSchema" name="extensionType">
  <xs:complexContent>
    <xs:extension base="xs:annotated">
      <xs:sequence>
        <xs:element ref="xs:openContent" minOccurs="0"/>
        <xs:group ref="xs:typeDefParticle" minOccurs="0"/>
        <xs:group ref="xs:attrDecls"/>
        <xs:group ref="xs:assertions"/>
      </xs:sequence>
      <xs:attribute name="base" type="xs:QName" use="required"/>

    </xs:extension>
  </xs:complexContent>
</xs:complexType>
"###;

const XSD_SIMPLE_RESTRICTION_TYPE: &str = r###"
<xs:complexType xmlns:xs="http://www.w3.org/2001/XMLSchema" name="simpleRestrictionType">
  <xs:complexContent>
    <xs:restriction base="xs:restrictionType">
      <xs:sequence>
        <xs:element ref="xs:annotation" minOccurs="0"/>
        <xs:choice minOccurs="0">
          <xs:annotation>
            <xs:documentation>This choice is added simply to
                  make this a valid restriction per the REC</xs:documentation>
          </xs:annotation>
          <xs:group ref="xs:simpleRestrictionModel"/>
        </xs:choice>
        <xs:group ref="xs:attrDecls"/>
        <xs:group ref="xs:assertions"/>
      </xs:sequence>
      <xs:anyAttribute namespace="##other" processContents="lax"/>
    </xs:restriction>
  </xs:complexContent>
</xs:complexType>
"###;

const XSD_SIMPLE_EXTENSION_TYPE: &str = r###"
<xs:complexType xmlns:xs="http://www.w3.org/2001/XMLSchema" name="simpleExtensionType">
  <xs:complexContent>
    <xs:restriction base="xs:extensionType">
      <xs:sequence>
        <xs:annotation>
          <xs:documentation>
    No typeDefParticle group reference</xs:documentation>
        </xs:annotation>
        <xs:element ref="xs:annotation" minOccurs="0"/>
        <xs:group ref="xs:attrDecls"/>
        <xs:group ref="xs:assertions"/>
      </xs:sequence>
      <xs:anyAttribute namespace="##other" processContents="lax"/>
    </xs:restriction>
  </xs:complexContent>
</xs:complexType>
"###;

const XSD_ELEMENT: &str = r###"
<xs:complexType xmlns:xs="http://www.w3.org/2001/XMLSchema" name="element" abstract="true">
  <xs:annotation>
    <xs:documentation>
  The element element can be used either
  at the top level to define an element-type binding globally,
  or within a content model to either reference a globally-defined
  element or type or declare an element-type binding locally.
  The ref form is not allowed at the top level.</xs:documentation>
  </xs:annotation>
  <xs:complexContent>
    <xs:extension base="xs:annotated">
      <xs:sequence>
        <xs:choice minOccurs="0">
          <xs:element name="simpleType" type="xs:localSimpleType"/>
          <xs:element name="complexType" type="xs:localComplexType"/>
        </xs:choice>
        <xs:element name="alternative" type="xs:altType"
                  minOccurs="0" maxOccurs="unbounded"/>
        <xs:group ref="xs:identityConstraint" minOccurs="0"
                  maxOccurs="unbounded"/>
      </xs:sequence>
      <xs:attributeGroup ref="xs:defRef"/>
      <xs:attribute name="type" type="xs:QName"/>

      <xs:attribute name="substitutionGroup">
        <xs:simpleType>
        <xs:list itemType="xs:QName"/>
        </xs:simpleType>
      </xs:attribute>
      <xs:attributeGroup ref="xs:occurs"/>
      <xs:attribute name="default" type="xs:string"/>
      <xs:attribute name="fixed" type="xs:string"/>
      <xs:attribute name="nillable" type="xs:boolean" use="optional"/>
      <xs:attribute name="abstract" type="xs:boolean" default="false"
                    use="optional"/>
      <xs:attribute name="final" type="xs:derivationSet"/>
      <xs:attribute name="block" type="xs:blockSet"/>
      <xs:attribute name="form" type="xs:formChoice"/>
      <xs:attribute name="targetNamespace" type="xs:anyURI"/>
    </xs:extension>
  </xs:complexContent>
</xs:complexType>
"###;

const XSD_TOP_LEVEL_ELEMENT: &str = r###"
<xs:complexType xmlns:xs="http://www.w3.org/2001/XMLSchema" name="topLevelElement">
  <xs:complexContent>
    <xs:restriction base="xs:element">
      <xs:sequence>
        <xs:element ref="xs:annotation" minOccurs="0"/>
        <xs:choice minOccurs="0">
          <xs:element name="simpleType" type="xs:localSimpleType"/>
          <xs:element name="complexType" type="xs:localComplexType"/>
        </xs:choice>
        <xs:element name="alternative" type="xs:altType"
                  minOccurs="0" maxOccurs="unbounded"/>
        <xs:group ref="xs:identityConstraint" minOccurs="0"
                  maxOccurs="unbounded"/>
      </xs:sequence>
      <xs:attribute name="ref" use="prohibited"/>
      <xs:attribute name="form" use="prohibited"/>
      <xs:attribute name="targetNamespace" use="prohibited"/>
      <xs:attribute name="minOccurs" use="prohibited"/>
      <xs:attribute name="maxOccurs" use="prohibited"/>
      <xs:attribute name="name" type="xs:NCName" use="required"/>
      <xs:anyAttribute namespace="##other" processContents="lax"/>
    </xs:restriction>
  </xs:complexContent>
</xs:complexType>
"###;

const XSD_LOCAL_ELEMENT: &str = r###"
<xs:complexType xmlns:xs="http://www.w3.org/2001/XMLSchema" name="localElement">
  <xs:complexContent>
    <xs:restriction base="xs:element">
      <xs:sequence>
        <xs:element ref="xs:annotation" minOccurs="0"/>
        <xs:choice minOccurs="0">
          <xs:element name="simpleType" type="xs:localSimpleType"/>
          <xs:element name="complexType" type="xs:localComplexType"/>
        </xs:choice>
        <xs:element name="alternative" type="xs:altType"
                  minOccurs="0" maxOccurs="unbounded"/>
        <xs:group ref="xs:identityConstraint" minOccurs="0"
                  maxOccurs="unbounded"/>
      </xs:sequence>
      <xs:attribute name="substitutionGroup" use="prohibited"/>
      <xs:attribute name="final" use="prohibited"/>
      <xs:attribute name="abstract" use="prohibited"/>
      <xs:anyAttribute namespace="##other" processContents="lax"/>
    </xs:restriction>
  </xs:complexContent>
</xs:complexType>
"###;

const XSD_ALT_TYPE: &str = r###"
<xs:complexType xmlns:xs="http://www.w3.org/2001/XMLSchema" name="altType">
  <xs:annotation>
    <xs:documentation>
      This type is used for 'alternative' elements.
    </xs:documentation>
  </xs:annotation>
  <xs:complexContent>
    <xs:extension base="xs:annotated">
      <xs:choice minOccurs="0">
        <xs:element name="simpleType" type="xs:localSimpleType"/>
        <xs:element name="complexType" type="xs:localComplexType"/>
      </xs:choice>
      <xs:attribute name="test" type="xs:string" use="optional"/>
      <xs:attribute name="type" type="xs:QName" use="optional"/>
      <xs:attribute name="xpathDefaultNamespace" type="xs:xpathDefaultNamespace"/>
    </xs:extension>
  </xs:complexContent>
</xs:complexType>
"###;

const XSD_GROUP: &str = r###"
<xs:complexType xmlns:xs="http://www.w3.org/2001/XMLSchema" name="group" abstract="true">
  <xs:annotation>
    <xs:documentation>
  group type for explicit groups, named top-level groups and
  group references</xs:documentation>
  </xs:annotation>
  <xs:complexContent>
    <xs:extension base="xs:annotated">

        <xs:group ref="xs:particle" minOccurs="0" maxOccurs="unbounded"/>

      <xs:attributeGroup ref="xs:defRef"/>
      <xs:attributeGroup ref="xs:occurs"/>
    </xs:extension>
  </xs:complexContent>
</xs:complexType>
"###;

const XSD_REAL_GROUP: &str = r###"
<xs:complexType xmlns:xs="http://www.w3.org/2001/XMLSchema" name="realGroup">
  <xs:complexContent>
    <xs:restriction base="xs:group">
      <xs:sequence>
        <xs:element ref="xs:annotation" minOccurs="0"/>
        <xs:choice minOccurs="0" maxOccurs="1">
          <xs:element ref="xs:all"/>
          <xs:element ref="xs:choice"/>
          <xs:element ref="xs:sequence"/>
        </xs:choice>

      </xs:sequence>
      <xs:anyAttribute namespace="##other" processContents="lax"/>
    </xs:restriction>
  </xs:complexContent>
</xs:complexType>
"###;

const XSD_NAMED_GROUP: &str = r###"
<xs:complexType xmlns:xs="http://www.w3.org/2001/XMLSchema" name="namedGroup">
  <xs:complexContent>
    <xs:restriction base="xs:realGroup">
      <xs:sequence>
        <xs:element ref="xs:annotation" minOccurs="0"/>
        <xs:choice minOccurs="1" maxOccurs="1">
          <xs:element name="all">
            <xs:complexType>
              <xs:complexContent>
                <xs:restriction base="xs:all">
                  <xs:group ref="xs:allModel"/>
                  <xs:attribute name="minOccurs" use="prohibited"/>
                  <xs:attribute name="maxOccurs" use="prohibited"/>
                  <xs:anyAttribute namespace="##other" processContents="lax"/>
                </xs:restriction>
              </xs:complexContent>
            </xs:complexType>
          </xs:element>
          <xs:element name="choice" type="xs:simpleExplicitGroup"/>
          <xs:element name="sequence" type="xs:simpleExplicitGroup"/>
        </xs:choice>
      </xs:sequence>
      <xs:attribute name="name" type="xs:NCName" use="required"/>
      <xs:attribute name="ref" use="prohibited"/>
      <xs:attribute name="minOccurs" use="prohibited"/>
      <xs:attribute name="maxOccurs" use="prohibited"/>
      <xs:anyAttribute namespace="##other" processContents="lax"/>
    </xs:restriction>
  </xs:complexContent>
</xs:complexType>
"###;

const XSD_GROUP_REF: &str = r###"
<xs:complexType xmlns:xs="http://www.w3.org/2001/XMLSchema" name="groupRef">
  <xs:complexContent>
    <xs:restriction base="xs:realGroup">
      <xs:sequence>
        <xs:element ref="xs:annotation" minOccurs="0"/>
      </xs:sequence>
      <xs:attribute name="ref" type="xs:QName" use="required"/>
      <xs:attribute name="name" use="prohibited"/>
      <xs:anyAttribute namespace="##other" processContents="lax"/>
    </xs:restriction>
  </xs:complexContent>
</xs:complexType>
"###;

const XSD_EXPLICIT_GROUP: &str = r###"
<xs:complexType xmlns:xs="http://www.w3.org/2001/XMLSchema" name="explicitGroup">
  <xs:annotation>
    <xs:documentation>
  group type for the three kinds of group</xs:documentation>
  </xs:annotation>
  <xs:complexContent>
    <xs:restriction base="xs:group">
      <xs:sequence>
        <xs:element ref="xs:annotation" minOccurs="0"/>
        <xs:group ref="xs:nestedParticle" minOccurs="0" maxOccurs="unbounded"/>
      </xs:sequence>
      <xs:attribute name="name" use="prohibited"/>
      <xs:attribute name="ref" use="prohibited"/>
      <xs:anyAttribute namespace="##other" processContents="lax"/>
    </xs:restriction>
  </xs:complexContent>
</xs:complexType>
"###;

const XSD_SIMPLE_EXPLICIT_GROUP: &str = r###"
<xs:complexType xmlns:xs="http://www.w3.org/2001/XMLSchema" name="simpleExplicitGroup">
  <xs:complexContent>
    <xs:restriction base="xs:explicitGroup">
      <xs:sequence>
        <xs:element ref="xs:annotation" minOccurs="0"/>
        <xs:group ref="xs:nestedParticle" minOccurs="0" maxOccurs="unbounded"/>
      </xs:sequence>
      <xs:attribute name="minOccurs" use="prohibited"/>
      <xs:attribute name="maxOccurs" use="prohibited"/>
      <xs:anyAttribute namespace="##other" processContents="lax"/>
    </xs:restriction>
  </xs:complexContent>
</xs:complexType>
"###;

const XSD_ALL: &str = r###"
<xs:complexType xmlns:xs="http://www.w3.org/2001/XMLSchema" name="all">
  <xs:annotation>
    <xs:documentation>
  Only elements allowed inside</xs:documentation>
  </xs:annotation>
  <xs:complexContent>
    <xs:restriction base="xs:explicitGroup">
      <xs:group ref="xs:allModel"/>
      <xs:attribute name="minOccurs" default="1" use="optional">
        <xs:simpleType>
          <xs:restriction base="xs:nonNegativeInteger">
            <xs:enumeration value="0"/>
            <xs:enumeration value="1"/>
          </xs:restriction>
        </xs:simpleType>
      </xs:attribute>
      <xs:attribute name="maxOccurs" default="1" use="optional">
        <xs:simpleType>
          <xs:restriction base="xs:allNNI">
            <xs:enumeration value="0"/>
            <xs:enumeration value="1"/>
          </xs:restriction>
        </xs:simpleType>
      </xs:attribute>
      <xs:anyAttribute namespace="##other" processContents="lax"/>
    </xs:restriction>
  </xs:complexContent>
</xs:complexType>
"###;

fn xsd_all() -> xs::ComplexType {
    xs::ComplexType(
        xs::types::TopLevelComplexType::builder()
            .name(LocalName::new_dangerous("all"))
            .annotation(
                xs::Annotation::builder()
                    .annotation(vec![xs::Documentation::builder()
                        .child_0(vec![xs::documentation_items::Child0 {
                            child_0: xmlity::XmlValue::Text(xmlity::xml!(
                                "\n  Only elements allowed inside"
                            )),
                        }])
                        .build()
                        .into()])
                    .build()
                    .into(),
            )
            .complex_type_model(Box::new(
                xs::ComplexContent::builder()
                    .child_1(
                        xs::types::ComplexRestrictionType::builder()
                            .base(xs::types::QName(ExpandedName::new(
                                LocalName::new_dangerous("explicitGroup"),
                                Some(XmlNamespace::XS),
                            )))
                            .child_1(
                              xs::types::complex_restriction_type_items::Child1
                              ::builder()
                              .type_def_particle(Box::new(
                                xs::types::GroupRef::builder()
                                    .ref_(xs::types::QName(ExpandedName::new(
                                        LocalName::new_dangerous("allModel"),
                                        Some(XmlNamespace::XS),
                                    )))
                                    .build()
                                    .into(),
                                    )).build().into()
                            )
                            .attr_decls(
                                xs::groups::AttrDecls::builder()
                                    .attribute(vec![
                                        xs::types::Attribute::builder()
                                            .name(LocalName::new_dangerous("minOccurs"))
                                            .default("1".to_string())
                                            // .use_(xs::AttributeUseType::Optional)
                                            .use_(xs::types::attribute_items::UseValue::Optional)
                                            .simple_type(
                                                xs::types::LocalSimpleType::builder()
                                                    .simple_derivation(
                                                      Box::new(
                                                      xs::Restriction::builder()
                                                            .base(xs::types::QName(
                                                                ExpandedName::new(
                                                                    LocalName::new_dangerous(
                                                                        "nonNegativeInteger",
                                                                    ),
                                                                    Some(XmlNamespace::XS),
                                                                ),
                                                            ))
                                                            .simple_restriction_model(
                                                              xs::groups::SimpleRestrictionModel::builder()
                                                                .child_1(vec![
                                                                    xs::Facet::from(xs::Enumeration(
                                                                        xs::types::NoFixedFacet::builder()
                                                                            .value("0".to_string())
                                                                            .build()
                                                                            .into(),
                                                                    ))
                                                                    .into(),
                                                                    xs::Facet::from(xs::Enumeration(
                                                                        xs::types::NoFixedFacet::builder()
                                                                            .value("1".to_string())
                                                                            .build()
                                                                            .into(),
                                                                    ))
                                                                    .into(),
                                                                ])
                                                                .build()
                                                                .into()
                                                            )
                                                            .build()
                                                            .into()
                                                          ),
                                                    )
                                                    .build()
                                                    .into(),
                                            )
                                            .build()
                                            .into(),
                                        xs::types::Attribute::builder()
                                            .name(LocalName::new_dangerous("maxOccurs"))
                                            .default("1".to_string())
                                            .use_(xs::types::attribute_items::UseValue::Optional)
                                            // .use_(xs::AttributeUseType::Optional)
                                            .simple_type(
                                                xs::types::LocalSimpleType::builder()
                                                    .simple_derivation(Box::new(
                                                        xs::Restriction::builder()
                                                            .base(xs::types::QName(
                                                                ExpandedName::new(
                                                                    LocalName::new_dangerous(
                                                                        "allNNI",
                                                                    ),
                                                                    Some(XmlNamespace::XS),
                                                                ),
                                                            ))
                                                            .simple_restriction_model(
                                                              xs::groups::SimpleRestrictionModel::builder()
                                                              .child_1(vec![
                                                                  xs::Facet::from(xs::Enumeration(
                                                                      xs::types::NoFixedFacet::builder()
                                                                          .value("0".to_string())
                                                                          .build()
                                                                          .into(),
                                                                  ))
                                                                  .into(),
                                                                  xs::Facet::from(xs::Enumeration(
                                                                      xs::types::NoFixedFacet::builder()
                                                                          .value("1".to_string())
                                                                          .build()
                                                                          .into(),
                                                                  ))
                                                                  .into(),
                                                              ])
                                                              .build()
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
                                    .any_attribute(
                                        xs::AnyAttribute::builder()
                                            .namespace(xs::types::NamespaceList::from(xs::types::SpecialNamespaceList::Other).into())
                                            .process_contents(
                                                xs::any_attribute_items::ProcessContentsValue::Lax, // xs::ProcessContentsType::Lax
                                            )
                                            .build().into(),
                                    )
                                    .build()
                                    .into(),
                            )
                            .assertions(xs::groups::Assertions::builder().build().into())
                            .build()
                            .into(),
                    )
                    .build()
                    .into(),
            ))
            .build()
            .into(),
    )
}

const XSD_WILDCARD: &str = r###"
<xs:complexType xmlns:xs="http://www.w3.org/2001/XMLSchema" name="wildcard">
  <xs:complexContent>ration::builder()
                                                            //     .value("0".to_string())
                                                            //     .build()
                                                            //     .into(),
                                                            // xs::Enumeration::builder()
                                                            //     .value("1".to_string())
                                                            //     .build()
                                                            //     .into(),
    <xs:extension base="xs:annotated">

        <xs:attributeGroup ref="xs:anyAttrGroup"/>
    </xs:extension>
  </xs:complexContent>
</xs:complexType>
"###;

const XSD_ATTRIBUTE_GROUP: &str = r###"
<xs:complexType xmlns:xs="http://www.w3.org/2001/XMLSchema" name="attributeGroup" abstract="true">
  <xs:complexContent>
    <xs:extension base="xs:annotated">

        <xs:group ref="xs:attrDecls"/>

      <xs:attributeGroup ref="xs:defRef"/>
    </xs:extension>
  </xs:complexContent>
</xs:complexType>
"###;

const XSD_NAMED_ATTRIBUTE_GROUP: &str = r###"
<xs:complexType xmlns:xs="http://www.w3.org/2001/XMLSchema" name="namedAttributeGroup">
  <xs:complexContent>
    <xs:restriction base="xs:attributeGroup">
      <xs:sequence>
        <xs:element ref="xs:annotation" minOccurs="0"/>
        <xs:group ref="xs:attrDecls"/>

      </xs:sequence>
      <xs:attribute name="name" type="xs:NCName" use="required"/>
      <xs:attribute name="ref" use="prohibited"/>
      <xs:anyAttribute namespace="##other" processContents="lax"/>
    </xs:restriction>
  </xs:complexContent>
</xs:complexType>
"###;

const XSD_ATTRIBUTE_GROUP_REF: &str = r###"
<xs:complexType xmlns:xs="http://www.w3.org/2001/XMLSchema" name="attributeGroupRef">
  <xs:complexContent>
    <xs:restriction base="xs:attributeGroup">
      <xs:sequence>
        <xs:element ref="xs:annotation" minOccurs="0"/>
      </xs:sequence>
      <xs:attribute name="ref" type="xs:QName" use="required"/>
      <xs:attribute name="name" use="prohibited"/>
      <xs:anyAttribute namespace="##other" processContents="lax"/>
    </xs:restriction>
  </xs:complexContent>
</xs:complexType>
"###;

const XSD_KEYBASE: &str = r###"
<xs:complexType xmlns:xs="http://www.w3.org/2001/XMLSchema" name="keybase">
  <xs:complexContent>
    <xs:extension base="xs:annotated">
      <xs:sequence minOccurs="0">
        <xs:element ref="xs:selector"/>
        <xs:element ref="xs:field" minOccurs="1" maxOccurs="unbounded"/>
      </xs:sequence>
      <xs:attribute name="name" type="xs:NCName"/>
      <xs:attribute name="ref" type="xs:QName"/>
    </xs:extension>
  </xs:complexContent>
</xs:complexType>
"###;

const XSD_ANY_TYPE: &str = r###"
<xs:complexType xmlns:xs="http://www.w3.org/2001/XMLSchema" name="anyType" mixed="true">
  <xs:annotation>
    <xs:documentation>
  Not the real urType, but as close an approximation as we can
  get in the XML representation</xs:documentation>
  </xs:annotation>
  <xs:sequence>
    <xs:any minOccurs="0" maxOccurs="unbounded" processContents="lax"/>
  </xs:sequence>
  <xs:anyAttribute processContents="lax"/>
</xs:complexType>
"###;

fn xsd_any_type() -> xs::ComplexType {
    xs::ComplexType(
        xs::types::TopLevelComplexType::builder()
            .name(LocalName::new_dangerous("anyType"))
            .mixed(true)
            .annotation(
                xs::Annotation::builder()
                    .annotation(vec![xs::Documentation::builder()
                        .child_0(vec![xs::documentation_items::Child0 {
                            child_0: XmlValue::Text(xmlity::xml!(
                                "\n  Not the real urType, but as close an approximation as we can
  get in the XML representation"
                            )),
                        }])
                        .build()
                        .into()])
                    .build()
                    .into(),
            )
            .complex_type_model(Box::new(
                xs::groups::complex_type_model_items::complex_type_model_variants::Variant2 {
                    open_content: None,
                    type_def_particle: Some(Box::new(
                        xs::Sequence(
                            xs::types::ExplicitGroup::builder()
                                .nested_particle(vec![xs::Any::builder()
                                    .min_occurs(0)
                                    .max_occurs(xs::types::AllNNI::from(xs::types::all_nni_items::all_nni_variants::Variant0::Unbounded).into())
                                    .process_contents(
                                        xs::any_items::ProcessContentsValue::Lax, // xs::ProcessContentsType::Lax
                                    )
                                    .build()
                                    .into()])
                                .build()
                                .into(),
                        )
                        .into(),
                    )),
                    attr_decls: xs::groups::AttrDecls::builder()
                        .any_attribute(
                            xs::AnyAttribute::builder()
                                .process_contents(
                                    xs::any_attribute_items::ProcessContentsValue::Lax, // xs::ProcessContentsType::Lax
                                )
                                .build()
                                .into(),
                        )
                        .build(),
                    assertions: xs::groups::Assertions::builder().build(),
                }
                .into(),
            ))
            .build()
            .into(),
    )
}

const XSD_SIMPLE_BASE_TYPE: &str = r###"
<xs:complexType xmlns:xs="http://www.w3.org/2001/XMLSchema" name="simpleBaseType" abstract="true">
  <xs:complexContent>
    <xs:extension base="xs:annotated">
      <xs:group ref="xs:simpleDerivation"/>
      <xs:attribute name="final" type="xs:simpleDerivationSet"/>
      <xs:attribute name="name" type="xs:NCName">
        <xs:annotation>
          <xs:documentation>
            Can be restricted to required or forbidden
          </xs:documentation>
        </xs:annotation>
      </xs:attribute>
    </xs:extension>
  </xs:complexContent>
</xs:complexType>
"###;

const XSD_TOP_LEVEL_SIMPLE_TYPE: &str = r###"
<xs:complexType xmlns:xs="http://www.w3.org/2001/XMLSchema" name="topLevelSimpleType">
  <xs:complexContent>
    <xs:restriction base="xs:simpleBaseType">
      <xs:sequence>
        <xs:element ref="xs:annotation" minOccurs="0"/>
        <xs:group ref="xs:simpleDerivation"/>
      </xs:sequence>
      <xs:attribute name="name" type="xs:NCName" use="required">
        <xs:annotation>
          <xs:documentation>
            Required at the top level
          </xs:documentation>
        </xs:annotation>
      </xs:attribute>
      <xs:anyAttribute namespace="##other" processContents="lax"/>
    </xs:restriction>
  </xs:complexContent>
</xs:complexType>
"###;

const XSD_LOCAL_SIMPLE_TYPE: &str = r###"
<xs:complexType xmlns:xs="http://www.w3.org/2001/XMLSchema" name="localSimpleType">
  <xs:complexContent>
    <xs:restriction base="xs:simpleBaseType">
      <xs:sequence>
        <xs:element ref="xs:annotation" minOccurs="0"/>
        <xs:group ref="xs:simpleDerivation"/>
      </xs:sequence>
      <xs:attribute name="name" use="prohibited">
        <xs:annotation>
          <xs:documentation>
            Forbidden when nested
          </xs:documentation>
        </xs:annotation>
      </xs:attribute>
      <xs:attribute name="final" use="prohibited"/>
      <xs:anyAttribute namespace="##other" processContents="lax"/>
    </xs:restriction>
  </xs:complexContent>
</xs:complexType>
"###;

const XSD_FACET: &str = r###"
<xs:complexType xmlns:xs="http://www.w3.org/2001/XMLSchema" name="facet">
  <xs:complexContent>
    <xs:extension base="xs:annotated">
      <xs:attribute name="value" use="required"/>
      <xs:attribute name="fixed" type="xs:boolean" default="false"
                    use="optional"/>
    </xs:extension>
  </xs:complexContent>
</xs:complexType>
"###;

const XSD_NO_FIXED_FACET: &str = r###"
<xs:complexType xmlns:xs="http://www.w3.org/2001/XMLSchema" name="noFixedFacet">
  <xs:complexContent>
    <xs:restriction base="xs:facet">
      <xs:sequence>
        <xs:element ref="xs:annotation" minOccurs="0"/>
      </xs:sequence>
      <xs:attribute name="fixed" use="prohibited"/>
      <xs:anyAttribute namespace="##other" processContents="lax"/>
    </xs:restriction>
  </xs:complexContent>
</xs:complexType>
"###;

const XSD_NUM_FACET: &str = r###"
<xs:complexType xmlns:xs="http://www.w3.org/2001/XMLSchema" name="numFacet">
  <xs:complexContent>
    <xs:restriction base="xs:facet">
      <xs:sequence>
        <xs:element ref="xs:annotation" minOccurs="0"/>
      </xs:sequence>
      <xs:attribute name="value"
          type="xs:nonNegativeInteger" use="required"/>
      <xs:anyAttribute namespace="##other" processContents="lax"/>
    </xs:restriction>
  </xs:complexContent>
</xs:complexType>
"###;

const XSD_INT_FACET: &str = r###"
<xs:complexType xmlns:xs="http://www.w3.org/2001/XMLSchema" name="intFacet">
  <xs:complexContent>
    <xs:restriction base="xs:facet">
      <xs:sequence>
        <xs:element ref="xs:annotation" minOccurs="0"/>
      </xs:sequence>
      <xs:attribute name="value" type="xs:integer" use="required"/>
      <xs:anyAttribute namespace="##other" processContents="lax"/>
    </xs:restriction>
  </xs:complexContent>
</xs:complexType>
"###;
