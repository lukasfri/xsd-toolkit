use xmlity::{ExpandedName, LocalName, XmlNamespace, XmlValue};
use xsd::schema as xs;

const XSD_COMPOSITION: &str = r###"
<xs:group xmlns:xs="http://www.w3.org/2001/XMLSchema" name="composition">
  <xs:choice>
    <xs:element ref="xs:include"/>
    <xs:element ref="xs:import"/>
    <xs:element ref="xs:redefine"/>
    <xs:element ref="xs:override"/>
    <xs:element ref="xs:annotation"/>
  </xs:choice>
</xs:group>
"###;

const XSD_SCHEMA_TOP: &str = r###"
<xs:group xmlns:xs="http://www.w3.org/2001/XMLSchema" name="schemaTop">
  <xs:annotation>
    <xs:documentation>
  This group is for the
  elements which occur freely at the top level of schemas.
  All of their types are based on the "annotated" type by extension.</xs:documentation>
  </xs:annotation>
  <xs:choice>
    <xs:group ref="xs:redefinable"/>
    <xs:element ref="xs:element"/>
    <xs:element ref="xs:attribute"/>
    <xs:element ref="xs:notation"/>
  </xs:choice>
</xs:group>
"###;

const XSD_REDEFINABLE: &str = r###"
<xs:group xmlns:xs="http://www.w3.org/2001/XMLSchema" name="redefinable">
  <xs:annotation>
    <xs:documentation>
  This group is for the
  elements which can self-redefine (see &lt;redefine> below).</xs:documentation>
  </xs:annotation>
  <xs:choice>
    <xs:element ref="xs:simpleType"/>
    <xs:element ref="xs:complexType"/>
    <xs:element ref="xs:group"/>
    <xs:element ref="xs:attributeGroup"/>
  </xs:choice>
</xs:group>
"###;

const XSD_TYPEDEF_PARTICLE: &str = r###"
<xs:group xmlns:xs="http://www.w3.org/2001/XMLSchema" name="typeDefParticle">
  <xs:annotation>
    <xs:documentation>
  'complexType' uses this</xs:documentation>
  </xs:annotation>
  <xs:choice>
    <xs:element name="group" type="xs:groupRef"/>
    <xs:element ref="xs:all"/>
    <xs:element ref="xs:choice"/>
    <xs:element ref="xs:sequence"/>
  </xs:choice>
</xs:group>
"###;

const XSD_NESTED_PARTICLE: &str = r###"
<xs:group xmlns:xs="http://www.w3.org/2001/XMLSchema" name="nestedParticle">
  <xs:choice>
    <xs:element name="element" type="xs:localElement"/>
    <xs:element name="group" type="xs:groupRef"/>

    <xs:element ref="xs:choice"/>
    <xs:element ref="xs:sequence"/>
    <xs:element ref="xs:any"/>
  </xs:choice>
</xs:group>
"###;

const XSD_PARTICLE: &str = r###"
<xs:group xmlns:xs="http://www.w3.org/2001/XMLSchema" name="particle">
  <xs:choice>
    <xs:element name="element" type="xs:localElement"/>
    <xs:element name="group" type="xs:groupRef"/>
    <xs:element ref="xs:all"/>
    <xs:element ref="xs:choice"/>
    <xs:element ref="xs:sequence"/>
    <xs:element ref="xs:any"/>
  </xs:choice>
</xs:group>
"###;

const XSD_ATTR_DECLS: &str = r###"
<xs:group xmlns:xs="http://www.w3.org/2001/XMLSchema" name="attrDecls">
  <xs:sequence>
    <xs:choice minOccurs="0" maxOccurs="unbounded">
      <xs:element name="attribute" type="xs:attribute"/>
      <xs:element name="attributeGroup" type="xs:attributeGroupRef"/>
    </xs:choice>
    <xs:element ref="xs:anyAttribute" minOccurs="0"/>
  </xs:sequence>
</xs:group>
"###;

const XSD_ASSERTIONS: &str = r###"
<xs:group xmlns:xs="http://www.w3.org/2001/XMLSchema" name="assertions">
  <xs:sequence>
    <xs:element name="assert" type="xs:assertion"
                minOccurs="0" maxOccurs="unbounded"/>
  </xs:sequence>
</xs:group>
"###;

const XSD_COMPLEX_TYPE_MODEL: &str = r###"
<xs:group xmlns:xs="http://www.w3.org/2001/XMLSchema" name="complexTypeModel">
  <xs:choice>
    <xs:element ref="xs:simpleContent"/>
    <xs:element ref="xs:complexContent"/>
    <xs:sequence>
      <xs:annotation>
        <xs:documentation>
  This branch is short for
  &lt;complexContent>
  &lt;restriction base="xs:anyType">
  ...
  &lt;/restriction>
  &lt;/complexContent></xs:documentation>
      </xs:annotation>
      <xs:element ref="xs:openContent" minOccurs="0"/>
      <xs:group ref="xs:typeDefParticle" minOccurs="0"/>
      <xs:group ref="xs:attrDecls"/>
      <xs:group ref="xs:assertions"/>
    </xs:sequence>
  </xs:choice>
</xs:group>
"###;

const XSD_ALL_MODEL: &str = r###"
<xs:group xmlns:xs="http://www.w3.org/2001/XMLSchema" name="allModel">
  <xs:sequence>
    <xs:element ref="xs:annotation" minOccurs="0"/>
    <xs:choice minOccurs="0" maxOccurs="unbounded">
      <xs:annotation>
        <xs:documentation>This choice with min/max is here to
                        avoid a pblm with the Elt:All/Choice/Seq
                        Particle derivation constraint</xs:documentation>
      </xs:annotation>
      <xs:element name="element" type="xs:localElement"/>
      <xs:element ref="xs:any"/>
      <xs:element name="group">
        <xs:complexType>
          <xs:complexContent>
            <xs:restriction base="xs:groupRef">
              <xs:sequence>
                <xs:element ref="xs:annotation" minOccurs="0"/>
              </xs:sequence>
              <xs:attribute name="minOccurs" fixed="1" type="xs:nonNegativeInteger"/>
              <xs:attribute name="maxOccurs" fixed="1" type="xs:nonNegativeInteger"/>
            </xs:restriction>
          </xs:complexContent>
        </xs:complexType>
      </xs:element>
    </xs:choice>
  </xs:sequence>
</xs:group>
"###;

fn xsd_all_model() -> xs::GroupType {
    xs::GroupType::builder()
        .name(LocalName::new_dangerous("allModel"))
        .content(
            xs::SequenceType::builder()
                .content(vec![
                    xs::LocalElement::builder()
                        .ref_(xs::QName(ExpandedName::new(
                            LocalName::new_dangerous("annotation"),
                            Some(XmlNamespace::XS),
                        )))
                        .min_occurs(xs::MinOccurs(0))
                        .build()
                        .into(),
                    xs::ChoiceType::builder()
                        .min_occurs(xs::MinOccurs(0))
                        .max_occurs(xs::MaxOccurs(xs::MaxOccursValue::Unbounded))
                        .annotation(
                            xs::Annotation::builder()
                            .content(vec![
                              xs::Documentation::builder()
                                .any(vec![XmlValue::Text(xmlity::xml!("This choice with min/max is here to\n                        avoid a pblm with the Elt:All/Choice/Seq\n                        Particle derivation constraint"))])
                                .build()
                                .into()
                            ]).build()
                        )
                        .content(vec![
                            xs::LocalElement::builder()
                                .name(LocalName::new_dangerous("element"))
                                .type_(xs::QName(ExpandedName::new(LocalName::new_dangerous("localElement"), Some(XmlNamespace::XS))))
                                .build()
                                .into(),
                            xs::LocalElement::builder()
                                .ref_(xs::QName(ExpandedName::new(
                                    LocalName::new_dangerous("any"),
                                    Some(XmlNamespace::XS),
                                )))
                                .build()
                                .into(),
                            xs::LocalElement::builder()
                                .name(LocalName::new_dangerous("group"))
                                .type_choice(
                                    xs::LocalComplexType::builder()
                                        .content(
                                            xs::ComplexContent::builder()
                                                .content(
                                                    xs::ComplexRestrictionType::builder()
                                                    .base(xs::QName(ExpandedName::new(LocalName::new_dangerous("groupRef"), Some(XmlNamespace::XS))))
                                                    .particle(
                                                        xs::SequenceType::builder()
                                                            .content(vec![
                                                              xs::LocalElement::builder()
                                                              .ref_(xs::QName(ExpandedName::new(LocalName::new_dangerous("annotation"), Some(XmlNamespace::XS))))
                                                              .min_occurs(xs::MinOccurs(0)).build().into()
                                                            ])
                                                            .build()
                                                            .into()
                                                    )
                                                    .attributes(vec![
                                                        xs::LocalAttribute::builder()
                                                        .name(LocalName::new_dangerous("minOccurs"))
                                                        .fixed(xs::Fixed("1".to_string()))  
                                                        .type_(xs::QName(ExpandedName::new(LocalName::new_dangerous("nonNegativeInteger"), Some(XmlNamespace::XS))))
                                                        .build()
                                                        .into(),
                                                        xs::LocalAttribute::builder()
                                                        .name(LocalName::new_dangerous("maxOccurs"))
                                                        .fixed(xs::Fixed("1".to_string()))  
                                                        .type_(xs::QName(ExpandedName::new(LocalName::new_dangerous("nonNegativeInteger"), Some(XmlNamespace::XS))))
                                                        .build()
                                                        .into(),
                                                    ])
                                                    .build()
                                                    .into()
                                                )
                                            .build()
                                            .into()
                                        )
                                        .build()
                                        .into()
                                )
                                .build()
                                .into(),
                        ])
                        .build()
                        .into(),
                ])
                .build()
                .into(),
        )
        .build()
}

const XSD_IDENTITY_CONSTRAINT: &str = r###"
<xs:group xmlns:xs="http://www.w3.org/2001/XMLSchema" name="identityConstraint">
  <xs:annotation>
    <xs:documentation>The three kinds of identity constraints, all with
                    type of or derived from 'keybase'.
  </xs:documentation>
  </xs:annotation>
  <xs:choice>
    <xs:element ref="xs:unique"/>
    <xs:element ref="xs:key"/>
    <xs:element ref="xs:keyref"/>
  </xs:choice>
</xs:group>
"###;

const XSD_SIMPLE_DERIVATION: &str = r###"
<xs:group xmlns:xs="http://www.w3.org/2001/XMLSchema" name="simpleDerivation">
  <xs:choice>
    <xs:element ref="xs:restriction"/>
    <xs:element ref="xs:list"/>
    <xs:element ref="xs:union"/>
  </xs:choice>
</xs:group>
"###;

const XSD_SIMPLE_RESTRICTION_MODEL: &str = r###"
<xs:group xmlns:xs="http://www.w3.org/2001/XMLSchema" name="simpleRestrictionModel">
  <xs:sequence>
    <xs:element name="simpleType" type="xs:localSimpleType" minOccurs="0"/>
    <xs:choice minOccurs="0"
        maxOccurs="unbounded">
      <xs:element ref="xs:facet"/>
      <xs:any processContents="lax"
          namespace="##other"/>
    </xs:choice>
  </xs:sequence>
</xs:group>
"###;

#[rstest::rstest]
#[case::composition(XSD_COMPOSITION, None)]
#[case::schema_top(XSD_SCHEMA_TOP, None)]
#[case::redefinable(XSD_REDEFINABLE, None)]
#[case::type_def_particle(XSD_TYPEDEF_PARTICLE, None)]
#[case::nested_particle(XSD_NESTED_PARTICLE, None)]
#[case::particle(XSD_PARTICLE, None)]
#[case::attr_decls(XSD_ATTR_DECLS, None)]
#[case::assertions(XSD_ASSERTIONS, None)]
#[case::complex_type_model(XSD_COMPLEX_TYPE_MODEL, None)]
#[case::all_model(XSD_ALL_MODEL, Some(xsd_all_model()))]
#[case::identity_constraint(XSD_IDENTITY_CONSTRAINT, None)]
#[case::simple_derivation(XSD_SIMPLE_DERIVATION, None)]
#[case::simple_restriction_model(XSD_SIMPLE_RESTRICTION_MODEL, None)]
fn deserialize(#[case] xml: &str, #[case] expected: Option<xs::GroupType>) {
    let xml = xml.trim();
    let element: xs::GroupType = xmlity_quick_xml::de::from_str(xml).unwrap();

    if let Some(expected) = expected {
        pretty_assertions::assert_eq!(element, expected);
    }
}
