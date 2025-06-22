use xmlity::{ExpandedName, LocalName, XmlNamespace};
use xs_raw::{xs, xs_custom};

#[rstest::rstest]
#[case::form_choice(XSD_FORM_CHOICE, None)]
#[case::reduced_derivation_control(XSD_REDUCED_DERIVATION_CONTROL, None)]
#[case::derivation_set(XSD_DERIVATION_SET, None)]
#[case::type_derivation_control(XSD_TYPE_DERIVATION_CONTROL, Some(xsd_type_derivation_control()))]
#[case::full_derivation_set(XSD_FULL_DERIVATION_SET, None)]
#[case::all_nni(XSD_ALL_NNI, None)]
#[case::block_set(XSD_BLOCK_SET, Some(xsd_block_set()))]
#[case::namespace_list(XSD_NAMESPACE_LIST, None)]
#[case::basic_namespace_list(XSD_BASIC_NAMESPACE_LIST, None)]
#[case::special_namespace_list(XSD_SPECIAL_NAMESPACE_LIST, None)]
#[case::qname_list(XSD_QNAME_LIST, None)]
#[case::qname_list_a(XSD_QNAME_LIST_A, None)]
#[case::xpath_default_namespace(XSD_XPATH_DEFAULT_NAMESPACE, None)]
#[case::public(XSD_PUBLIC, None)]
#[case::derivation_control(XSD_DERIVATION_CONTROL, None)]
#[case::simple_derivation_set(XSD_SIMPLE_DERIVATION_SET, None)]
#[ntest::timeout(1000)]
fn deserialize(#[case] xml: &str, #[case] expected: Option<xs::SimpleType>) {
    let xml = xml.trim();
    let element: xs::SimpleType = xmlity_quick_xml::de::from_str(xml).unwrap();

    if let Some(expected) = expected {
        pretty_assertions::assert_eq!(element, expected);
    }
}

const XSD_FORM_CHOICE: &str = r###"
<xs:simpleType xmlns:xs="http://www.w3.org/2001/XMLSchema" name="formChoice">
  <xs:annotation>
    <xs:documentation>
  A utility type, not for public use</xs:documentation>
  </xs:annotation>
  <xs:restriction base="xs:NMTOKEN">
    <xs:enumeration value="qualified"/>
    <xs:enumeration value="unqualified"/>
  </xs:restriction>
</xs:simpleType>
"###;

const XSD_REDUCED_DERIVATION_CONTROL: &str = r###"
<xs:simpleType xmlns:xs="http://www.w3.org/2001/XMLSchema" name="reducedDerivationControl">
  <xs:annotation>
    <xs:documentation>
  A utility type, not for public use</xs:documentation>
  </xs:annotation>
  <xs:restriction base="xs:derivationControl">
    <xs:enumeration value="extension"/>
    <xs:enumeration value="restriction"/>
  </xs:restriction>
</xs:simpleType>
"###;

const XSD_DERIVATION_SET: &str = r###"
<xs:simpleType xmlns:xs="http://www.w3.org/2001/XMLSchema" name="derivationSet">
  <xs:annotation>
    <xs:documentation>
  A utility type, not for public use</xs:documentation>
    <xs:documentation>
  #all or (possibly empty) subset of {extension, restriction}</xs:documentation>
  </xs:annotation>
  <xs:union>
    <xs:simpleType>
      <xs:restriction base="xs:token">
        <xs:enumeration value="#all"/>
      </xs:restriction>
    </xs:simpleType>
    <xs:simpleType>
      <xs:list itemType="xs:reducedDerivationControl"/>
    </xs:simpleType>
  </xs:union>
</xs:simpleType>
"###;

const XSD_TYPE_DERIVATION_CONTROL: &str = r###"
<xs:simpleType xmlns:xs="http://www.w3.org/2001/XMLSchema" name="typeDerivationControl">
  <xs:annotation>
    <xs:documentation>
  A utility type, not for public use</xs:documentation>
  </xs:annotation>
  <xs:restriction base="xs:derivationControl">
    <xs:enumeration value="extension"/>
    <xs:enumeration value="restriction"/>
    <xs:enumeration value="list"/>
    <xs:enumeration value="union"/>
  </xs:restriction>
</xs:simpleType>
"###;

fn xsd_type_derivation_control() -> xs::SimpleType {
    xs::types::TopLevelSimpleType::builder()
        .name(LocalName::new_dangerous("typeDerivationControl"))
        .annotation(
            xs::Annotation::builder()
                .annotation(vec![xs::Documentation::builder()
                    .particle(vec![xs::documentation_items::Child0 {
                        child_0: xmlity::XmlValue::Text(xmlity::xml!(
                            "\n  A utility type, not for public use"
                        )),
                    }])
                    .build()
                    .into()])
                .build()
                .into(),
        )
        .simple_derivation(Box::new(
            xs::Restriction::builder()
                .base(xs_custom::QName(ExpandedName::new(
                    LocalName::new_dangerous("derivationControl"),
                    Some(XmlNamespace::XS),
                )))
                .simple_restriction_model(
                    xs::groups::SimpleRestrictionModel::builder()
                        .child_1(vec![
                            // xs::Enumeration::builder()
                            //     .value("extension".to_string())
                            //     .build()
                            //     .into(),
                            // xs::Enumeration::builder()
                            //     .value("restriction".to_string())
                            //     .build()
                            //     .into(),
                            // xs::Enumeration::builder()
                            //     .value("list".to_string())
                            //     .build()
                            //     .into(),
                            // xs::Enumeration::builder()
                            //     .value("union".to_string())
                            //     .build()
                            //     .into(),
                        ])
                        .build()
                        .into(),
                )
                .build()
                .into(),
        ))
        .build()
        .into()
}

const XSD_FULL_DERIVATION_SET: &str = r###"
<xs:simpleType xmlns:xs="http://www.w3.org/2001/XMLSchema" name="fullDerivationSet">
  <xs:annotation>
    <xs:documentation>
  A utility type, not for public use</xs:documentation>
    <xs:documentation>
  #all or (possibly empty) subset of {extension, restriction, list, union}</xs:documentation>
  </xs:annotation>
  <xs:union>
    <xs:simpleType>
      <xs:restriction base="xs:token">
        <xs:enumeration value="#all"/>
      </xs:restriction>
    </xs:simpleType>
    <xs:simpleType>
      <xs:list itemType="xs:typeDerivationControl"/>
    </xs:simpleType>
  </xs:union>
</xs:simpleType>
"###;

const XSD_ALL_NNI: &str = r###"
<xs:simpleType xmlns:xs="http://www.w3.org/2001/XMLSchema" name="allNNI">
  <xs:annotation>
    <xs:documentation>
  for maxOccurs</xs:documentation>
  </xs:annotation>
  <xs:union memberTypes="xs:nonNegativeInteger">
    <xs:simpleType>
      <xs:restriction base="xs:NMTOKEN">
        <xs:enumeration value="unbounded"/>
      </xs:restriction>
    </xs:simpleType>
  </xs:union>
</xs:simpleType>
"###;

const XSD_BLOCK_SET: &str = r###"
<xs:simpleType xmlns:xs="http://www.w3.org/2001/XMLSchema" name="blockSet">
  <xs:annotation>
    <xs:documentation>
  A utility type, not for public use</xs:documentation>
    <xs:documentation>
  #all or (possibly empty) subset of {substitution, extension,
  restriction}</xs:documentation>
  </xs:annotation>
  <xs:union>
    <xs:simpleType>
      <xs:restriction base="xs:token">
        <xs:enumeration value="#all"/>
      </xs:restriction>
    </xs:simpleType>
    <xs:simpleType>
      <xs:list>
        <xs:simpleType>
          <xs:restriction base="xs:derivationControl">
            <xs:enumeration value="extension"/>
            <xs:enumeration value="restriction"/>
            <xs:enumeration value="substitution"/>
          </xs:restriction>
        </xs:simpleType>
      </xs:list>
    </xs:simpleType>
  </xs:union>
</xs:simpleType>
"###;

fn xsd_block_set() -> xs::SimpleType {
    xs::types::TopLevelSimpleType::builder()
    .name(LocalName::new_dangerous("blockSet"))
    .annotation(
        xs::Annotation::builder()
            .annotation(vec![
                xs::Documentation::builder()
                    .particle(vec![xs::documentation_items::Child0 {
                            child_0:xmlity::XmlValue::Text(xmlity::xml!(
                        "\n  A utility type, not for public use"
                    )),
                        }])
                        .build()
                        .into(),
                xs::Documentation::builder()
                    .particle(vec![xs::documentation_items::Child0 {
                            child_0: xmlity::XmlValue::Text(xmlity::xml!(
                        "\n  #all or (possibly empty) subset of {substitution, extension,\n  restriction}"
                    )),
                        }])
                        .build()
                        .into(),
            ])
            .build()
            .into(),
    )
    .simple_derivation(Box::new(
        xs::Union::builder()
              .simple_type(vec![
                xs::types::LocalSimpleType::builder()
                    .simple_derivation(Box::new(
                        xs::Restriction::builder()
                            .base(xs_custom::QName(ExpandedName::new(
                                LocalName::new_dangerous("token"),
                                Some(XmlNamespace::XS),
                            )))
                            .simple_restriction_model(
                                xs::groups::SimpleRestrictionModel::builder()
                                    .child_1(vec![
                                        // xs::Enumeration::builder()
                                        //     .value("#all".to_string())
                                        //     .build()
                                        //     .into(),
                                    ])
                                    .build()
                                    .into(),
                            )
                            .build()
                            .into(),
                    ))
                    .build()
                    .into(),
                xs::types::LocalSimpleType::builder()
                    .simple_derivation(Box::new(
                        xs::List::builder()
                        .simple_type(
                            xs::types::LocalSimpleType::builder()
                            .simple_derivation(Box::new(
                                xs::Restriction::builder()
                                    .base(xs_custom::QName(ExpandedName::new(
                                        LocalName::new_dangerous("derivationControl"),
                                        Some(XmlNamespace::XS),
                                    )))
                                    .simple_restriction_model(
                                        xs::groups::SimpleRestrictionModel::builder()
                                            .child_1(vec![
                                                // xs::Enumeration::builder()
                                                //     .value("extension".to_string())
                                                //     .build()
                                                //     .into(),
                                                // xs::Enumeration::builder()
                                                //     .value("restriction".to_string())
                                                //     .build()
                                                //     .into(),
                                                // xs::Enumeration::builder()
                                                //     .value("substitution".to_string())
                                                //     .build()
                                                //     .into(),
                                            ])
                                            .build()
                                            .into(),
                                    )
                                    .build()
                                    .into(),
                            ))
                            .build().into()
                        ).build()
                        .into(),
                    ))
                    .build()
                    .into(),
            ])
            .build()
            .into(),
    ))
    .build()
    .into()
}

const XSD_NAMESPACE_LIST: &str = r###"
<xs:simpleType xmlns:xs="http://www.w3.org/2001/XMLSchema" name="namespaceList">
  <xs:annotation>
    <xs:documentation>
  A utility type, not for public use</xs:documentation>
  </xs:annotation>

  <xs:union memberTypes="xs:specialNamespaceList xs:basicNamespaceList" />
</xs:simpleType>
"###;

const XSD_BASIC_NAMESPACE_LIST: &str = r###"
<xs:simpleType xmlns:xs="http://www.w3.org/2001/XMLSchema" name="basicNamespaceList">
  <xs:annotation>
    <xs:documentation>
  A utility type, not for public use</xs:documentation>
  </xs:annotation>
  <xs:list>
    <xs:simpleType>
      <xs:union memberTypes="xs:anyURI">
        <xs:simpleType>
          <xs:restriction base="xs:token">
            <xs:enumeration value="##targetNamespace"/>
            <xs:enumeration value="##local"/>
          </xs:restriction>
        </xs:simpleType>
      </xs:union>
    </xs:simpleType>
  </xs:list>
</xs:simpleType>
"###;

const XSD_SPECIAL_NAMESPACE_LIST: &str = r###"
<xs:simpleType xmlns:xs="http://www.w3.org/2001/XMLSchema" name="specialNamespaceList">
  <xs:annotation>
    <xs:documentation>
  A utility type, not for public use</xs:documentation>
  </xs:annotation>
  <xs:restriction base="xs:token">
    <xs:enumeration value="##any"/>
    <xs:enumeration value="##other"/>
  </xs:restriction>
</xs:simpleType>
"###;

const XSD_QNAME_LIST: &str = r###"
<xs:simpleType xmlns:xs="http://www.w3.org/2001/XMLSchema" name="qnameList">
  <xs:annotation>
    <xs:documentation>
      A utility type, not for public use
    </xs:documentation>
  </xs:annotation>
  <xs:list>
    <xs:simpleType>
      <xs:union memberTypes="xs:QName">
        <xs:simpleType>
          <xs:restriction base="xs:token">
            <xs:enumeration value="##defined"/>
            <xs:enumeration value="##definedSibling"/>
          </xs:restriction>
        </xs:simpleType>
      </xs:union>
    </xs:simpleType>
  </xs:list>
</xs:simpleType>
"###;

const XSD_QNAME_LIST_A: &str = r###"
<xs:simpleType xmlns:xs="http://www.w3.org/2001/XMLSchema" name="qnameListA">
  <xs:annotation>
    <xs:documentation>
      A utility type, not for public use
    </xs:documentation>
  </xs:annotation>
  <xs:list>
    <xs:simpleType>
      <xs:union memberTypes="xs:QName">
        <xs:simpleType>
          <xs:restriction base="xs:token">
            <xs:enumeration value="##defined"/>
          </xs:restriction>
        </xs:simpleType>
      </xs:union>
    </xs:simpleType>
  </xs:list>
</xs:simpleType>
"###;

const XSD_XPATH_DEFAULT_NAMESPACE: &str = r###"
<xs:simpleType xmlns:xs="http://www.w3.org/2001/XMLSchema" name="xpathDefaultNamespace">
  <xs:union memberTypes="xs:anyURI">
    <xs:simpleType>
      <xs:restriction base="xs:token">
        <xs:enumeration value="##defaultNamespace"/>
        <xs:enumeration value="##targetNamespace"/>
        <xs:enumeration value="##local"/>
      </xs:restriction>
    </xs:simpleType>
  </xs:union>
</xs:simpleType>
"###;

const XSD_PUBLIC: &str = r###"
<xs:simpleType xmlns:xs="http://www.w3.org/2001/XMLSchema" name="public">
  <xs:annotation>
    <xs:documentation>
  A utility type, not for public use</xs:documentation>
    <xs:documentation>
  A public identifier, per ISO 8879</xs:documentation>
  </xs:annotation>
  <xs:restriction base="xs:token"/>
</xs:simpleType>
"###;

const XSD_DERIVATION_CONTROL: &str = r###"
<xs:simpleType xmlns:xs="http://www.w3.org/2001/XMLSchema" name="derivationControl">
  <xs:annotation>
    <xs:documentation>
  A utility type, not for public use</xs:documentation>
  </xs:annotation>
  <xs:restriction base="xs:NMTOKEN">
    <xs:enumeration value="substitution"/>
    <xs:enumeration value="extension"/>
    <xs:enumeration value="restriction"/>
    <xs:enumeration value="list"/>
    <xs:enumeration value="union"/>
  </xs:restriction>
</xs:simpleType>
"###;

const XSD_SIMPLE_DERIVATION_SET: &str = r###"
<xs:simpleType xmlns:xs="http://www.w3.org/2001/XMLSchema" name="simpleDerivationSet">
  <xs:annotation>
    <xs:documentation>
  #all or (possibly empty) subset of {restriction, extension, union, list}
  </xs:documentation>
    <xs:documentation>
  A utility type, not for public use</xs:documentation>
  </xs:annotation>
  <xs:union>
    <xs:simpleType>
      <xs:restriction base="xs:token">
        <xs:enumeration value="#all"/>
      </xs:restriction>
    </xs:simpleType>
    <xs:simpleType>
      <xs:list>
        <xs:simpleType>
          <xs:restriction base="xs:derivationControl">
            <xs:enumeration value="list"/>
            <xs:enumeration value="union"/>
            <xs:enumeration value="restriction"/>
            <xs:enumeration value="extension"/>
          </xs:restriction>
        </xs:simpleType>
      </xs:list>
    </xs:simpleType>
  </xs:union>
</xs:simpleType>
"###;
