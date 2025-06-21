use xmlity::{ExpandedName, LocalName, XmlNamespace};
use xs_raw::{xs, xs_custom};

const XSD_SCHEMA: &str = r###"
<xs:element xmlns:xs="http://www.w3.org/2001/XMLSchema" name="schema" id="schema">
  <xs:annotation>
    <xs:documentation
          source="../structures/structures.html#element-schema"/>
  </xs:annotation>
  <xs:complexType>
    <xs:complexContent>
      <xs:extension base="xs:openAttrs">
        <xs:sequence>
          <xs:group ref="xs:composition" minOccurs="0" maxOccurs="unbounded"/>
          <xs:sequence minOccurs="0">
            <xs:element ref="xs:defaultOpenContent"/>
            <xs:element ref="xs:annotation" minOccurs="0"
                        maxOccurs="unbounded"/>
          </xs:sequence>
          <xs:sequence minOccurs="0" maxOccurs="unbounded">
            <xs:group ref="xs:schemaTop"/>
            <xs:element ref="xs:annotation" minOccurs="0"
                        maxOccurs="unbounded"/>
          </xs:sequence>
        </xs:sequence>
        <xs:attribute name="targetNamespace" type="xs:anyURI"/>
        <xs:attribute name="version" type="xs:token"/>
        <xs:attribute name="finalDefault" type="xs:fullDerivationSet"
                      default="" use="optional"/>
        <xs:attribute name="blockDefault" type="xs:blockSet" default=""
                      use="optional"/>
        <xs:attribute name="attributeFormDefault" type="xs:formChoice"
                      default="unqualified" use="optional"/>
        <xs:attribute name="elementFormDefault" type="xs:formChoice"
                      default="unqualified" use="optional"/>
        <xs:attribute name="defaultAttributes" type="xs:QName"/>
        <xs:attribute name="xpathDefaultNamespace" type="xs:xpathDefaultNamespace"
                      default="##local" use="optional"/>
        <xs:attribute name="id" type="xs:ID"/>
        <xs:attribute ref="xml:lang"/>
      </xs:extension>
    </xs:complexContent>
  </xs:complexType>
  <xs:key name="element">
    <xs:selector xpath="xs:element"/>
    <xs:field xpath="@name"/>
  </xs:key>
  <xs:key name="attribute">
    <xs:selector xpath="xs:attribute"/>
    <xs:field xpath="@name"/>
  </xs:key>
  <xs:key name="type">
    <xs:selector xpath="xs:complexType|xs:simpleType"/>
    <xs:field xpath="@name"/>
  </xs:key>
  <xs:key name="group">
    <xs:selector xpath="xs:group"/>
    <xs:field xpath="@name"/>
  </xs:key>
  <xs:key name="attributeGroup">
    <xs:selector xpath="xs:attributeGroup"/>
    <xs:field xpath="@name"/>
  </xs:key>
  <xs:key name="notation">
    <xs:selector xpath="xs:notation"/>
    <xs:field xpath="@name"/>
  </xs:key>
  <xs:key name="identityConstraint">
    <xs:selector xpath=".//xs:key|.//xs:unique|.//xs:keyref"/>
    <xs:field xpath="@name"/>
  </xs:key>
</xs:element>
"###;

const XSD_ANY_ATTRIBUTE: &str = r###"
<xs:element xmlns:xs="http://www.w3.org/2001/XMLSchema" name="anyAttribute"  id="anyAttribute">
  <xs:annotation>
    <xs:documentation
          source="../structures/structures.html#element-anyAttribute"/>
  </xs:annotation>
  <xs:complexType>
    <xs:complexContent>
      <xs:extension base="xs:wildcard">
        <xs:attribute name="notQName" type="xs:qnameListA"
                      use="optional"/>
      </xs:extension>
    </xs:complexContent>
  </xs:complexType>
</xs:element>
"###;

fn xsd_any_attribute() -> xs::Element {
    xs::types::TopLevelElement::builder()
        .name(LocalName::new_dangerous("anyAttribute"))
        .id("anyAttribute".to_string())
        .annotation(
            xs::Annotation::builder()
                .annotation(vec![xs::Documentation::builder()
                    .source(xs_custom::TargetNamespace(XmlNamespace::new_dangerous(
                        "../structures/structures.html#element-anyAttribute",
                    )))
                    .build()
                    .into()])
                .build()
                .into(),
        )
        .type_(
            xs::types::LocalComplexType::builder()
                .complex_type_model(
                    xs::groups::ComplexTypeModel(
                        xs::ComplexContent::builder()
                            .child_1(
                                xs::types::ExtensionType::builder()
                                    .base(xs_custom::QName(ExpandedName::new(
                                        LocalName::new_dangerous("wildcard"),
                                        Some(XmlNamespace::XS),
                                    )))
                                    .attr_decls(
                                        xs::groups::AttrDecls::builder()
                                            .attribute(vec![xs::types::Attribute::builder()
                                                .name(LocalName::new_dangerous("notQName"))
                                                .type_(xs_custom::QName(ExpandedName::new(
                                                    LocalName::new_dangerous("qnameListA"),
                                                    Some(XmlNamespace::XS),
                                                )))
                                                // .use_(xs::AttributeUseType::Optional)
                                                .use_("optional".to_string())
                                                .build()
                                                .into()])
                                            .build()
                                            .into(),
                                    )
                                    .assertions(xs::groups::Assertions::builder().build().into())
                                    .build()
                                    .into(),
                            )
                            .build()
                            .into(),
                    )
                    .into(),
                )
                .build()
                .into(),
        )
        .build()
        .into()
}

const XSD_COMPLEX_CONTENT: &str = r###"
<xs:element xmlns:xs="http://www.w3.org/2001/XMLSchema" name="complexContent" id="complexContent">
  <xs:annotation>
    <xs:documentation
          source="../structures/structures.html#element-complexContent"/>
  </xs:annotation>
  <xs:complexType>
    <xs:complexContent>
      <xs:extension base="xs:annotated">
        <xs:choice>
          <xs:element name="restriction" type="xs:complexRestrictionType"/>
          <xs:element name="extension" type="xs:extensionType"/>
        </xs:choice>
        <xs:attribute name="mixed" type="xs:boolean">
          <xs:annotation>
            <xs:documentation>
      Overrides any setting on complexType parent.</xs:documentation>
          </xs:annotation>
        </xs:attribute>
      </xs:extension>
    </xs:complexContent>
  </xs:complexType>
</xs:element>
"###;

const XSD_OPEN_CONTENT: &str = r###"
<xs:element xmlns:xs="http://www.w3.org/2001/XMLSchema" name="openContent" id="openContent">
  <xs:annotation>
    <xs:documentation
          source="../structures/structures.html#element-openContent"/>
  </xs:annotation>
  <xs:complexType>
    <xs:complexContent>
      <xs:extension base="xs:annotated">
        <xs:sequence>
          <xs:element name="any" minOccurs="0" type="xs:wildcard"/>
        </xs:sequence>
        <xs:attribute name="mode" default="interleave" use="optional">
          <xs:simpleType>
            <xs:restriction base="xs:NMTOKEN">
              <xs:enumeration value="none"/>
              <xs:enumeration value="interleave"/>
              <xs:enumeration value="suffix"/>
            </xs:restriction>
          </xs:simpleType>
        </xs:attribute>

      </xs:extension>
    </xs:complexContent>
  </xs:complexType>
</xs:element>
"###;

const XSD_DEFAULT_OPEN_CONTENT: &str = r###"
<xs:element xmlns:xs="http://www.w3.org/2001/XMLSchema" name="defaultOpenContent" id="defaultOpenContent">
  <xs:annotation>
    <xs:documentation
          source="../structures/structures.html#element-defaultOpenContent"/>
  </xs:annotation>
  <xs:complexType>
    <xs:complexContent>
      <xs:extension base="xs:annotated">
        <xs:sequence>
          <xs:element name="any" type="xs:wildcard"/>
        </xs:sequence>
        <xs:attribute name="appliesToEmpty" type="xs:boolean"
                      default="false" use="optional"/>
        <xs:attribute name="mode" default="interleave" use="optional">
          <xs:simpleType>
            <xs:restriction base="xs:NMTOKEN">
              <xs:enumeration value="interleave"/>
              <xs:enumeration value="suffix"/>
            </xs:restriction>
          </xs:simpleType>
        </xs:attribute>

      </xs:extension>
    </xs:complexContent>
  </xs:complexType>
</xs:element>
"###;

const XSD_SIMPLE_CONTENT: &str = r###"
<xs:element xmlns:xs="http://www.w3.org/2001/XMLSchema" name="simpleContent" id="simpleContent">
  <xs:annotation>
    <xs:documentation
          source="../structures/structures.html#element-simpleContent"/>
  </xs:annotation>
  <xs:complexType>
    <xs:complexContent>
      <xs:extension base="xs:annotated">
        <xs:choice>
          <xs:element name="restriction" type="xs:simpleRestrictionType"/>
          <xs:element name="extension" type="xs:simpleExtensionType"/>
        </xs:choice>
      </xs:extension>
    </xs:complexContent>
  </xs:complexType>
</xs:element>
"###;

const XSD_COMPLEX_TYPE: &str = r###"
<xs:element xmlns:xs="http://www.w3.org/2001/XMLSchema" name="complexType" type="xs:topLevelComplexType" id="complexType">
  <xs:annotation>
    <xs:documentation
          source="../structures/structures.html#element-complexType"/>
  </xs:annotation>
</xs:element>
"###;

const XSD_ELEMENT: &str = r###"
<xs:element xmlns:xs="http://www.w3.org/2001/XMLSchema" name="element" type="xs:topLevelElement" id="element">
  <xs:annotation>
    <xs:documentation
          source="../structures/structures.html#element-element"/>
  </xs:annotation>
</xs:element>
"###;

const XSD_ALL: &str = r###"
<xs:element xmlns:xs="http://www.w3.org/2001/XMLSchema" name="all" type="xs:all" id="all">
  <xs:annotation>
    <xs:documentation source="../structures/structures.html#element-all"/>
  </xs:annotation>
</xs:element>
"###;

const XSD_CHOICE: &str = r###"
<xs:element xmlns:xs="http://www.w3.org/2001/XMLSchema" name="choice" type="xs:explicitGroup" id="choice">
  <xs:annotation>
    <xs:documentation
          source="../structures/structures.html#element-choice"/>
  </xs:annotation>
</xs:element>
"###;

const XSD_SEQUENCE: &str = r###"
<xs:element xmlns:xs="http://www.w3.org/2001/XMLSchema" name="sequence" type="xs:explicitGroup" id="sequence">
  <xs:annotation>
    <xs:documentation
          source="../structures/structures.html#element-sequence"/>
  </xs:annotation>
</xs:element>
"###;

const XSD_GROUP: &str = r###"
<xs:element xmlns:xs="http://www.w3.org/2001/XMLSchema" name="group" type="xs:namedGroup" id="group">
  <xs:annotation>
    <xs:documentation source="../structures/structures.html#element-group"/>
  </xs:annotation>
</xs:element>
"###;

const XSD_ANY: &str = r###"
<xs:element xmlns:xs="http://www.w3.org/2001/XMLSchema" name="any" id="any">
  <xs:annotation>
    <xs:documentation source="../structures/structures.html#element-any"/>
  </xs:annotation>
  <xs:complexType>
    <xs:complexContent>
      <xs:extension base="xs:wildcard">
        <xs:attribute name="notQName" type="xs:qnameList"
                      use="optional"/>
        <xs:attributeGroup ref="xs:occurs"/>
      </xs:extension>
    </xs:complexContent>
  </xs:complexType>
</xs:element>
"###;

const XSD_ATTRIBUTE: &str = r###"
<xs:element xmlns:xs="http://www.w3.org/2001/XMLSchema" name="attribute" type="xs:topLevelAttribute" id="attribute">
  <xs:annotation>
    <xs:documentation
          source="../structures/structures.html#element-attribute"/>
  </xs:annotation>
</xs:element>
"###;

const XSD_ATTRIBUTE_GROUP: &str = r###"
<xs:element xmlns:xs="http://www.w3.org/2001/XMLSchema" name="attributeGroup" type="xs:namedAttributeGroup"
            id="attributeGroup">
  <xs:annotation>
    <xs:documentation
          source="../structures/structures.html#element-attributeGroup"/>
  </xs:annotation>
</xs:element>
"###;

const XSD_INCLUDE: &str = r###"
<xs:element xmlns:xs="http://www.w3.org/2001/XMLSchema" name="include" id="include">
  <xs:annotation>
    <xs:documentation
          source="../structures/structures.html#element-include"/>
  </xs:annotation>
  <xs:complexType>
    <xs:complexContent>
      <xs:extension base="xs:annotated">
        <xs:attribute name="schemaLocation" type="xs:anyURI" use="required"/>
      </xs:extension>
    </xs:complexContent>
  </xs:complexType>
</xs:element>
"###;

const XSD_REDEFINE: &str = r###"
<xs:element xmlns:xs="http://www.w3.org/2001/XMLSchema" name="redefine" id="redefine">
  <xs:annotation>
    <xs:documentation
          source="../structures/structures.html#element-redefine"/>
  </xs:annotation>
  <xs:complexType>
    <xs:complexContent>
      <xs:extension base="xs:openAttrs">
        <xs:choice minOccurs="0" maxOccurs="unbounded">
          <xs:element ref="xs:annotation"/>
          <xs:group ref="xs:redefinable"/>
        </xs:choice>
        <xs:attribute name="schemaLocation" type="xs:anyURI" use="required"/>
        <xs:attribute name="id" type="xs:ID"/>
      </xs:extension>
    </xs:complexContent>
  </xs:complexType>
</xs:element>
"###;

const XSD_OVERRIDE: &str = r###"
<xs:element xmlns:xs="http://www.w3.org/2001/XMLSchema" name="override" id="override">
  <xs:annotation>
    <xs:documentation
          source="../structures/structures.html#element-override"/>
  </xs:annotation>
  <xs:complexType>
    <xs:complexContent>
      <xs:extension base="xs:openAttrs">
        <xs:sequence>
          <xs:element ref="xs:annotation" minOccurs="0"/>
          <xs:group ref="xs:schemaTop" minOccurs="0" maxOccurs="unbounded"/>
        </xs:sequence>
        <xs:attribute name="schemaLocation" type="xs:anyURI" use="required"/>
        <xs:attribute name="id" type="xs:ID"/>
      </xs:extension>
    </xs:complexContent>
  </xs:complexType>
</xs:element>
"###;

const XSD_IMPORT: &str = r###"
<xs:element xmlns:xs="http://www.w3.org/2001/XMLSchema" name="import" id="import">
  <xs:annotation>
    <xs:documentation
          source="../structures/structures.html#element-import"/>
  </xs:annotation>
  <xs:complexType>
    <xs:complexContent>
      <xs:extension base="xs:annotated">
        <xs:attribute name="namespace" type="xs:anyURI"/>
        <xs:attribute name="schemaLocation" type="xs:anyURI"/>
      </xs:extension>
    </xs:complexContent>
  </xs:complexType>
</xs:element>
"###;

const XSD_SELECTOR: &str = r###"
<xs:element xmlns:xs="http://www.w3.org/2001/XMLSchema" name="selector" id="selector">
  <xs:annotation>
    <xs:documentation
          source="../structures/structures.html#element-selector"/>
  </xs:annotation>
  <xs:complexType>
    <xs:complexContent>
      <xs:extension base="xs:annotated">
        <xs:attribute name="xpath" use="required">
          <xs:simpleType>
            <xs:annotation>
              <xs:documentation>A subset of XPath expressions for use
in selectors</xs:documentation>
              <xs:documentation>A utility type, not for public
use</xs:documentation>
            </xs:annotation>
            <xs:restriction base="xs:token"/>

          </xs:simpleType>
        </xs:attribute>
        <xs:attribute name="xpathDefaultNamespace" type="xs:xpathDefaultNamespace"/>
      </xs:extension>
    </xs:complexContent>
  </xs:complexType>
</xs:element>
"###;

const XSD_FIELD: &str = r###"
<xs:element xmlns:xs="http://www.w3.org/2001/XMLSchema" name="field" id="field">
  <xs:annotation>
    <xs:documentation source="../structures/structures.html#element-field"/>
  </xs:annotation>
  <xs:complexType>
    <xs:complexContent>
      <xs:extension base="xs:annotated">
        <xs:attribute name="xpath" use="required">
          <xs:simpleType>
            <xs:annotation>
              <xs:documentation>A subset of XPath expressions for use
in fields</xs:documentation>
              <xs:documentation>A utility type, not for public
use</xs:documentation>
            </xs:annotation>
            <xs:restriction base="xs:token"/>

          </xs:simpleType>
        </xs:attribute>
        <xs:attribute name="xpathDefaultNamespace" type="xs:xpathDefaultNamespace"/>
      </xs:extension>
    </xs:complexContent>
  </xs:complexType>
</xs:element>
"###;

const XSD_UNIQUE: &str = r###"
<xs:element xmlns:xs="http://www.w3.org/2001/XMLSchema" name="unique" type="xs:keybase" id="unique">
  <xs:annotation>
    <xs:documentation
          source="../structures/structures.html#element-unique"/>
  </xs:annotation>
</xs:element>
"###;

const XSD_KEY: &str = r###"
<xs:element xmlns:xs="http://www.w3.org/2001/XMLSchema" name="key" type="xs:keybase" id="key">
  <xs:annotation>
    <xs:documentation source="../structures/structures.html#element-key"/>
  </xs:annotation>
</xs:element>
"###;

const XSD_KEYREF: &str = r###"
<xs:element xmlns:xs="http://www.w3.org/2001/XMLSchema" name="keyref" id="keyref">
  <xs:annotation>
    <xs:documentation
          source="../structures/structures.html#element-keyref"/>
  </xs:annotation>
  <xs:complexType>
    <xs:complexContent>
      <xs:extension base="xs:keybase">
        <xs:attribute name="refer" type="xs:QName"/>
      </xs:extension>
    </xs:complexContent>
  </xs:complexType>
</xs:element>
"###;

const XSD_NOTATION: &str = r###"
<xs:element xmlns:xs="http://www.w3.org/2001/XMLSchema" name="notation" id="notation">
  <xs:annotation>
    <xs:documentation
          source="../structures/structures.html#element-notation"/>
  </xs:annotation>
  <xs:complexType>
    <xs:complexContent>
      <xs:extension base="xs:annotated">
        <xs:attribute name="name" type="xs:NCName" use="required"/>
        <xs:attribute name="public" type="xs:public"/>
        <xs:attribute name="system" type="xs:anyURI"/>
      </xs:extension>
    </xs:complexContent>
  </xs:complexType>
</xs:element>
"###;

const XSD_APPINFO: &str = r###"
<xs:element xmlns:xs="http://www.w3.org/2001/XMLSchema" name="appinfo" id="appinfo">
  <xs:annotation>
    <xs:documentation
          source="../structures/structures.html#element-appinfo"/>
  </xs:annotation>
  <xs:complexType mixed="true">
    <xs:sequence minOccurs="0" maxOccurs="unbounded">
      <xs:any processContents="lax"/>
    </xs:sequence>
    <xs:attribute name="source" type="xs:anyURI"/>
    <xs:anyAttribute namespace="##other" processContents="lax"/>
  </xs:complexType>
</xs:element>
"###;

const XSD_DOCUMENTATION: &str = r###"
<xs:element xmlns:xs="http://www.w3.org/2001/XMLSchema" name="documentation" id="documentation">
  <xs:annotation>
    <xs:documentation
          source="../structures/structures.html#element-documentation"/>
  </xs:annotation>
  <xs:complexType mixed="true">
    <xs:sequence minOccurs="0" maxOccurs="unbounded">
      <xs:any processContents="lax"/>
    </xs:sequence>
    <xs:attribute name="source" type="xs:anyURI"/>
    <xs:attribute ref="xml:lang"/>
    <xs:anyAttribute namespace="##other" processContents="lax"/>
  </xs:complexType>
</xs:element>
"###;

const XSD_ANNOTATION: &str = r###"
<xs:element xmlns:xs="http://www.w3.org/2001/XMLSchema" name="annotation" id="annotation">
  <xs:annotation>
    <xs:documentation
          source="../structures/structures.html#element-annotation"/>
  </xs:annotation>
  <xs:complexType>
    <xs:complexContent>
      <xs:extension base="xs:openAttrs">
        <xs:choice minOccurs="0" maxOccurs="unbounded">
          <xs:element ref="xs:appinfo"/>
          <xs:element ref="xs:documentation"/>
        </xs:choice>
        <xs:attribute name="id" type="xs:ID"/>
      </xs:extension>
    </xs:complexContent>
  </xs:complexType>
</xs:element>
"###;

const XSD_SIMPLE_TYPE: &str = r###"
<xs:element xmlns:xs="http://www.w3.org/2001/XMLSchema" name="simpleType" type="xs:topLevelSimpleType" id="simpleType">
  <xs:annotation>
    <xs:documentation
          source="http://www.w3.org/TR/xmlschema11-2/#element-simpleType"/>
  </xs:annotation>
</xs:element>
"###;

const XSD_FACET: &str = r###"
<xs:element xmlns:xs="http://www.w3.org/2001/XMLSchema" name="facet" abstract="true">
  <xs:annotation>
    <xs:documentation>
      An abstract element, representing facets in general.
      The facets defined by this spec are substitutable for
      this element, and implementation-defined facets should
      also name this as a substitution-group head.
    </xs:documentation>
  </xs:annotation>
</xs:element>
"###;

const XSD_RESTRICTION: &str = r###"
<xs:element xmlns:xs="http://www.w3.org/2001/XMLSchema" name="restriction" id="restriction">
  <xs:complexType>
    <xs:annotation>
      <xs:documentation
            source="http://www.w3.org/TR/xmlschema11-2/#element-restriction">
        base attribute and simpleType child are mutually
        exclusive, but one or other is required
      </xs:documentation>
    </xs:annotation>
    <xs:complexContent>
      <xs:extension base="xs:annotated">
        <xs:group ref="xs:simpleRestrictionModel"/>
        <xs:attribute name="base" type="xs:QName" use="optional"/>
      </xs:extension>
    </xs:complexContent>
  </xs:complexType>
</xs:element>
"###;

const XSD_LIST: &str = r###"
<xs:element xmlns:xs="http://www.w3.org/2001/XMLSchema" name="list" id="list">
  <xs:complexType>
    <xs:annotation>
      <xs:documentation
            source="http://www.w3.org/TR/xmlschema11-2/#element-list">
        itemType attribute and simpleType child are mutually
        exclusive, but one or other is required
      </xs:documentation>
    </xs:annotation>
    <xs:complexContent>
      <xs:extension base="xs:annotated">
        <xs:sequence>
          <xs:element name="simpleType" type="xs:localSimpleType"
                      minOccurs="0"/>
        </xs:sequence>
        <xs:attribute name="itemType" type="xs:QName" use="optional"/>
      </xs:extension>
    </xs:complexContent>
  </xs:complexType>
</xs:element>
"###;

const XSD_UNION: &str = r###"
<xs:element xmlns:xs="http://www.w3.org/2001/XMLSchema" name="union" id="union">
  <xs:complexType>
    <xs:annotation>
      <xs:documentation
            source="http://www.w3.org/TR/xmlschema11-2/#element-union">
        memberTypes attribute must be non-empty or there must be
        at least one simpleType child
      </xs:documentation>
    </xs:annotation>
    <xs:complexContent>
      <xs:extension base="xs:annotated">
        <xs:sequence>
          <xs:element name="simpleType" type="xs:localSimpleType"
                      minOccurs="0" maxOccurs="unbounded"/>
        </xs:sequence>
        <xs:attribute name="memberTypes" use="optional">
          <xs:simpleType>
            <xs:list itemType="xs:QName"/>
          </xs:simpleType>
        </xs:attribute>
      </xs:extension>
    </xs:complexContent>
  </xs:complexType>
</xs:element>
"###;

const XSD_MIN_EXCLUSIVE: &str = r###"
<xs:element xmlns:xs="http://www.w3.org/2001/XMLSchema" name="minExclusive" type="xs:facet"
  id="minExclusive"
  substitutionGroup="xs:facet">
  <xs:annotation>
    <xs:documentation
          source="http://www.w3.org/TR/xmlschema11-2/#element-minExclusive"/>
  </xs:annotation>
</xs:element>
"###;

const XSD_MIN_INCLUSIVE: &str = r###"
<xs:element xmlns:xs="http://www.w3.org/2001/XMLSchema" name="minInclusive" type="xs:facet"
  id="minInclusive"
  substitutionGroup="xs:facet">
  <xs:annotation>
    <xs:documentation
          source="http://www.w3.org/TR/xmlschema11-2/#element-minInclusive"/>
  </xs:annotation>
</xs:element>
"###;

const XSD_MAX_EXCLUSIVE: &str = r###"
<xs:element xmlns:xs="http://www.w3.org/2001/XMLSchema" name="maxExclusive" type="xs:facet"
  id="maxExclusive"
  substitutionGroup="xs:facet">
  <xs:annotation>
    <xs:documentation
          source="http://www.w3.org/TR/xmlschema11-2/#element-maxExclusive"/>
  </xs:annotation>
</xs:element>
"###;

const XSD_MAX_INCLUSIVE: &str = r###"
<xs:element xmlns:xs="http://www.w3.org/2001/XMLSchema" name="maxInclusive" type="xs:facet"
  id="maxInclusive"
  substitutionGroup="xs:facet">
  <xs:annotation>
    <xs:documentation
          source="http://www.w3.org/TR/xmlschema11-2/#element-maxInclusive"/>
  </xs:annotation>
</xs:element>
"###;

const XSD_TOTAL_DIGITS: &str = r###"
<xs:element xmlns:xs="http://www.w3.org/2001/XMLSchema" name="totalDigits" id="totalDigits"
  substitutionGroup="xs:facet">
  <xs:annotation>
    <xs:documentation
          source="http://www.w3.org/TR/xmlschema11-2/#element-totalDigits"/>
  </xs:annotation>
  <xs:complexType>
    <xs:complexContent>
      <xs:restriction base="xs:numFacet">
        <xs:sequence>
          <xs:element ref="xs:annotation" minOccurs="0"/>
        </xs:sequence>
        <xs:attribute name="value" type="xs:positiveInteger" use="required"/>
        <xs:anyAttribute namespace="##other" processContents="lax"/>
      </xs:restriction>
    </xs:complexContent>
  </xs:complexType>
</xs:element>
"###;

fn xsd_total_digits() -> xs::Element {
    xs::types::TopLevelElement::builder()
        .name(LocalName::new_dangerous("totalDigits"))
        .id("totalDigits".to_string())
        .substitution_group("xs:facet".to_string())
        .annotation(
            xs::Annotation::builder()
                .annotation(vec![xs::Documentation::builder()
                    .source(xs_custom::TargetNamespace(XmlNamespace::new_dangerous( "http://www.w3.org/TR/xmlschema11-2/#element-totalDigits")))
                    .build()
                    .into()])
                .build()
                .into(),
        )
        .type_(
            xs::types::LocalComplexType::builder()
                .complex_type_model(
                    xs::groups::ComplexTypeModel(
                        xs::ComplexContent::builder()
                            .child_1(
                                xs::types::ComplexRestrictionType::builder()
                                    .base(xs_custom::QName(ExpandedName::new(
                                        LocalName::new_dangerous("numFacet"),
                                        Some(XmlNamespace::XS),
                                    )))
                                    .variant_0(
                                      xs::types::complex_restriction_type_items::variant_0_variants::Variant0::builder()
                                      .type_def_particle(
                                        Box::new(
                                          xs::groups::TypeDefParticle(
                                            xs::Sequence(Box::new(
                                              xs::types::ExplicitGroup::builder()
                                            .nested_particle(vec![xs::groups::NestedParticle(xs::types::LocalElement::builder()
                                                .ref_(xs_custom::QName(ExpandedName::new(
                                                    LocalName::new_dangerous("annotation"),
                                                    Some(XmlNamespace::XS),
                                                )))
                                                .min_occurs(0)
                                                .build()
                                                .into()).into()])
                                            .build())
                                            .into()).into()))
                                          ).build().into(),
                                    )
                                    .attr_decls(
                                        xs::groups::AttrDecls::builder()
                                            .attribute(vec![xs::types::Attribute::builder()
                                                .name(LocalName::new_dangerous("value"))
                                                .type_(xs_custom::QName(ExpandedName::new(
                                                    LocalName::new_dangerous("positiveInteger"),
                                                    Some(XmlNamespace::XS),
                                                )))
                                                // .use_(xs::AttributeUseType::Required)
                                                .use_("required".to_string())
                                                .build()
                                                .into()])
                                            .any_attribute(
                                                xs::AnyAttribute::builder()
                                                    .namespace(xs_custom::NamespaceListType::Other)
                                                    .process_contents("lax".to_string())
                                                    // .namespace(xs::NamespaceListType::Other)
                                                    // .process_contents(xs::ProcessContentsType::Lax)
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
                    )
                    .into(),
                )
                .build()
                .into(),
        )
        .build()
        .into()
}

const XSD_FRACTION_DIGITS: &str = r###"
<xs:element xmlns:xs="http://www.w3.org/2001/XMLSchema" name="fractionDigits" type="xs:numFacet"
  id="fractionDigits"
  substitutionGroup="xs:facet">
  <xs:annotation>
    <xs:documentation
          source="http://www.w3.org/TR/xmlschema11-2/#element-fractionDigits"/>
  </xs:annotation>
</xs:element>
"###;

const XSD_LENGTH: &str = r###"
<xs:element xmlns:xs="http://www.w3.org/2001/XMLSchema" name="length" type="xs:numFacet" id="length"
  substitutionGroup="xs:facet">
  <xs:annotation>
    <xs:documentation
          source="http://www.w3.org/TR/xmlschema11-2/#element-length"/>
  </xs:annotation>
</xs:element>
"###;

const XSD_MIN_LENGTH: &str = r###"
<xs:element xmlns:xs="http://www.w3.org/2001/XMLSchema" name="minLength" type="xs:numFacet"
  id="minLength"
  substitutionGroup="xs:facet">
  <xs:annotation>
    <xs:documentation
          source="http://www.w3.org/TR/xmlschema11-2/#element-minLength"/>
  </xs:annotation>
</xs:element>
"###;

const XSD_MAX_LENGTH: &str = r###"
<xs:element xmlns:xs="http://www.w3.org/2001/XMLSchema" name="maxLength" type="xs:numFacet"
  id="maxLength"
  substitutionGroup="xs:facet">
  <xs:annotation>
    <xs:documentation
          source="http://www.w3.org/TR/xmlschema11-2/#element-maxLength"/>
  </xs:annotation>
</xs:element>
"###;

const XSD_ENUMERATION: &str = r###"
<xs:element xmlns:xs="http://www.w3.org/2001/XMLSchema" name="enumeration" type="xs:noFixedFacet"
  id="enumeration"
  substitutionGroup="xs:facet">
  <xs:annotation>
    <xs:documentation
          source="http://www.w3.org/TR/xmlschema11-2/#element-enumeration"/>
  </xs:annotation>
</xs:element>
"###;

const XSD_WHITE_SPACE: &str = r###"
<xs:element xmlns:xs="http://www.w3.org/2001/XMLSchema" name="whiteSpace" id="whiteSpace"
  substitutionGroup="xs:facet">
  <xs:annotation>
    <xs:documentation
          source="http://www.w3.org/TR/xmlschema11-2/#element-whiteSpace"/>
  </xs:annotation>
  <xs:complexType>
    <xs:complexContent>
      <xs:restriction base="xs:facet">
        <xs:sequence>
          <xs:element ref="xs:annotation" minOccurs="0"/>
        </xs:sequence>
        <xs:attribute name="value" use="required">
          <xs:simpleType>
            <xs:restriction base="xs:NMTOKEN">
              <xs:enumeration value="preserve"/>
              <xs:enumeration value="replace"/>
              <xs:enumeration value="collapse"/>
            </xs:restriction>
          </xs:simpleType>
        </xs:attribute>
        <xs:anyAttribute namespace="##other" processContents="lax"/>
      </xs:restriction>
    </xs:complexContent>
  </xs:complexType>
</xs:element>
"###;

fn xsd_white_space() -> xs::Element {
    xs::Element(
        xs::types::TopLevelElement::builder()
            .name(LocalName::new_dangerous("whiteSpace"))
            .id("whiteSpace".to_string())
            .substitution_group("xs:facet".to_string())
            .annotation(
                xs::Annotation::builder()
                    .annotation(vec![xs::Documentation::builder()
                        .source(xs_custom::TargetNamespace(XmlNamespace::new_dangerous(
                            "http://www.w3.org/TR/xmlschema11-2/#element-whiteSpace",
                        )))
                        .build()
                        .into()])
                    .build()
                    .into(),
            )
            .type_(
                xs::types::LocalComplexType::builder()
                    .complex_type_model(
                        xs::groups::ComplexTypeModel(
                            xs::ComplexContent::builder()
                                .child_1(
                                    xs::types::ComplexRestrictionType::builder()
                                        .base(xs_custom::QName(ExpandedName::new(
                                            LocalName::new_dangerous("facet"),
                                            Some(XmlNamespace::XS),
                                        )))
                                        .variant_0(
                                          xs::types::complex_restriction_type_items::variant_0_variants::Variant0::builder()
                                          .type_def_particle(
                                            xs::groups::TypeDefParticle(
                                              
                                            xs::Sequence(
                                              xs::types::ExplicitGroup::builder()
                                                .nested_particle(vec![
                                                  xs::groups::NestedParticle(
                                                  
                                                  xs::types::LocalElement::builder()
                                                    .ref_(xs_custom::QName(ExpandedName::new(
                                                        LocalName::new_dangerous("annotation"),
                                                        Some(XmlNamespace::XS),
                                                    )))
                                                    .min_occurs(0)
                                                    .build()
                                                    .into()
                                                    )
                                                    .into()])
                                                .build()
                                                .into()
                                                  )
                                                .into(),
                                            )
                                            .into()
                                                 
                                          ).build().into()
                                        )
                                        .attr_decls(
                                            xs::groups::AttrDecls::builder()
                                                .attribute(vec![xs::types::Attribute::builder()
                                                    .name(LocalName::new_dangerous("value"))
                                                    // .use_(xs::AttributeUseType::Required)
                                                    .use_("required".to_string())
                                                    .simple_type(
                                                        xs::types::LocalSimpleType::builder()
                                                            .simple_derivation(
                                                              xs::groups::SimpleDerivation(
                                                                xs::Restriction::builder()
                                                                .base(xs_custom::QName(ExpandedName::new(
                                                                    LocalName::new_dangerous(
                                                                        "NMTOKEN",
                                                                    ),
                                                                    Some(XmlNamespace::XS),
                                                                )))
                                                                .simple_restriction_model(
                                                                  xs::groups::SimpleRestrictionModel::builder()
                                                                  .child_1(vec![
                                                                    //TODO
                                                                    // xs::Enumeration::builder()
                                                                    //     .value(
                                                                    //         "preserve".to_string(),
                                                                    //     )
                                                                    //     .build()
                                                                    //     .into(),
                                                                    // xs::Enumeration::builder()
                                                                    //     .value(
                                                                    //         "replace".to_string(),
                                                                    //     )
                                                                    //     .build()
                                                                    //     .into(),
                                                                    // xs::Enumeration::builder()
                                                                    //     .value(
                                                                    //         "collapse".to_string(),
                                                                    //     )
                                                                    //     .build()
                                                                    //     .into(),
                                                                ]).build()
                                                                .into()
                                                                )
                                                                .build()
                                                                .into())
                                                                .into()
                                                            )
                                                            .build()
                                                            .into(),
                                                    )
                                                    .build()
                                                    .into()])
                                                .any_attribute(
                                                    xs::AnyAttribute::builder()
                                                        .namespace(xs_custom::NamespaceListType::Other)
                                                        .process_contents(
                                                          "lax".to_string()
                                                            // xs::ProcessContentsType::Lax,
                                                        )
                                                        .build()
                                                        .into(),
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
                        )
                        .into(),
                    )
                    .build()
                    .into(),
            )
            .build()
            .into(),
    )
}

const XSD_PATTERN: &str = r###"
<xs:element xmlns:xs="http://www.w3.org/2001/XMLSchema" name="pattern" id="pattern"
  substitutionGroup="xs:facet">
  <xs:annotation>
    <xs:documentation
          source="http://www.w3.org/TR/xmlschema11-2/#element-pattern"/>
  </xs:annotation>
  <xs:complexType>
    <xs:complexContent>
      <xs:restriction base="xs:noFixedFacet">
        <xs:sequence>
          <xs:element ref="xs:annotation" minOccurs="0"/>
        </xs:sequence>
        <xs:attribute name="value" type="xs:string"
            use="required"/>
        <xs:anyAttribute namespace="##other"
            processContents="lax"/>
      </xs:restriction>
    </xs:complexContent>
  </xs:complexType>
</xs:element>
"###;

const XSD_ASSERTION: &str = r###"
<xs:element xmlns:xs="http://www.w3.org/2001/XMLSchema" name="assertion" type="xs:assertion"
            id="assertion" substitutionGroup="xs:facet">
  <xs:annotation>
    <xs:documentation
          source="http://www.w3.org/TR/xmlschema11-2/#element-assertion"/>
  </xs:annotation>
</xs:element>
"###;

const XSD_EXPLICIT_TIMEZONE: &str = r###"
<xs:element xmlns:xs="http://www.w3.org/2001/XMLSchema" name="explicitTimezone" id="explicitTimezone"
  substitutionGroup="xs:facet">
  <xs:annotation>
    <xs:documentation
          source="http://www.w3.org/TR/xmlschema11-2/#element-explicitTimezone"/>
  </xs:annotation>
  <xs:complexType>
    <xs:complexContent>
      <xs:restriction base="xs:facet">
        <xs:sequence>
          <xs:element ref="xs:annotation" minOccurs="0"/>
        </xs:sequence>
        <xs:attribute name="value" use="required">
          <xs:simpleType>
            <xs:restriction base="xs:NMTOKEN">
              <xs:enumeration value="optional"/>
              <xs:enumeration value="required"/>
              <xs:enumeration value="prohibited"/>
            </xs:restriction>
          </xs:simpleType>
        </xs:attribute>
        <xs:anyAttribute namespace="##other" processContents="lax"/>
      </xs:restriction>
    </xs:complexContent>
  </xs:complexType>
</xs:element>
"###;

#[rstest::rstest]
#[case::schema(XSD_SCHEMA, None)]
#[case::any_attribute(XSD_ANY_ATTRIBUTE, Some(xsd_any_attribute()))]
#[case::complex_content(XSD_COMPLEX_CONTENT, None)]
#[case::open_content(XSD_OPEN_CONTENT, None)]
#[case::default_open_content(XSD_DEFAULT_OPEN_CONTENT, None)]
#[case::simple_content(XSD_SIMPLE_CONTENT, None)]
#[case::complex_type(XSD_COMPLEX_TYPE, None)]
#[case::element(XSD_ELEMENT, None)]
#[case::all(XSD_ALL, None)]
#[case::choice(XSD_CHOICE, None)]
#[case::sequence(XSD_SEQUENCE, None)]
#[case::group(XSD_GROUP, None)]
#[case::any(XSD_ANY, None)]
#[case::attribute(XSD_ATTRIBUTE, None)]
#[case::attribute_group(XSD_ATTRIBUTE_GROUP, None)]
#[case::include(XSD_INCLUDE, None)]
#[case::redefine(XSD_REDEFINE, None)]
#[case::override_(XSD_OVERRIDE, None)]
#[case::import(XSD_IMPORT, None)]
#[case::selector(XSD_SELECTOR, None)]
#[case::field(XSD_FIELD, None)]
#[case::unique(XSD_UNIQUE, None)]
#[case::key(XSD_KEY, None)]
#[case::keyref(XSD_KEYREF, None)]
#[case::notation(XSD_NOTATION, None)]
#[case::appinfo(XSD_APPINFO, None)]
#[case::documentation(XSD_DOCUMENTATION, None)]
#[case::annotation(XSD_ANNOTATION, None)]
#[case::simple_type(XSD_SIMPLE_TYPE, None)]
#[case::facet(XSD_FACET, None)]
#[case::restriction(XSD_RESTRICTION, None)]
#[case::list(XSD_LIST, None)]
#[case::union(XSD_UNION, None)]
#[case::min_exclusive(XSD_MIN_EXCLUSIVE, None)]
#[case::min_inclusive(XSD_MIN_INCLUSIVE, None)]
#[case::max_exclusive(XSD_MAX_EXCLUSIVE, None)]
#[case::max_inclusive(XSD_MAX_INCLUSIVE, None)]
#[case::total_digits(XSD_TOTAL_DIGITS, Some(xsd_total_digits()))]
#[case::fraction_digits(XSD_FRACTION_DIGITS, None)]
#[case::length(XSD_LENGTH, None)]
#[case::min_length(XSD_MIN_LENGTH, None)]
#[case::max_length(XSD_MAX_LENGTH, None)]
#[case::enumeration(XSD_ENUMERATION, None)]
#[case::white_space(XSD_WHITE_SPACE, Some(xsd_white_space()))]
#[case::pattern(XSD_PATTERN, None)]
#[case::assertion(XSD_ASSERTION, None)]
#[case::explicit_timezone(XSD_EXPLICIT_TIMEZONE, None)]
#[ntest::timeout(1000)]
fn deserialize(#[case] xml: &str, #[case] expected: Option<xs::Element>) {
    let xml = xml.trim();
    let element: xs::Element = xmlity_quick_xml::de::from_str(xml).unwrap();

    if let Some(expected) = expected {
        pretty_assertions::assert_eq!(element, expected);
    }
}
