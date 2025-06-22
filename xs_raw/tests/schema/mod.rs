use xmlity::types::utils::XmlRoot;
// use xsd::schema as xs;
use xs_raw::{xs, xs_custom};

pub mod top_level_attribute_groups;
pub mod top_level_complex_type;
pub mod top_level_element;
pub mod top_level_group;
pub mod top_level_simple_type;

#[test]
fn schema_deserialize() {
    let xml = include_str!("../../../schemas/XMLSchema.xsd");

    let _schema: XmlRoot<xs::Schema> = xmlity_quick_xml::from_str(xml).unwrap();
}

const CHOICE_XML: &str = r###"
<xs:choice xmlns:xs="http://www.w3.org/2001/XMLSchema">
    <xs:element name="restriction" type="xs:complexRestrictionType"/>
    <xs:element name="extension" type="xs:extensionType"/>
</xs:choice>
"###;
const EXTENSION2_XML: &str = r###"
<xs:extension xmlns:xs="http://www.w3.org/2001/XMLSchema" base="xs:annotated">
<xs:b/>
<xs:attribute name="mixed" type="xs:boolean">
    <xs:annotation>
    <xs:documentation>
Overrides any setting on complexType parent.</xs:documentation>
    </xs:annotation>
</xs:attribute>
</xs:extension>
"###;
const EXTENSION_XML: &str = r###"
<xs:extension xmlns:xs="http://www.w3.org/2001/XMLSchema" base="xs:annotated">
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
"###;
const COMPLEX_CONTENT_XML: &str = r###"
<xs:complexContent xmlns:xs="http://www.w3.org/2001/XMLSchema">
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
"###;
const COMPLEX_TYPE_XML: &str = r###"
<xs:complexType xmlns:xs="http://www.w3.org/2001/XMLSchema">
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
"###;

const ELEMENT_XML: &str = r###"
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

#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::bon::Builder,
    ::core::cmp::PartialEq,
)]
#[xvalue(order = "strict")]
pub struct XmlValueDocChild {
    pub child_0: ::xmlity::XmlValue,
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::bon::Builder,
    ::core::cmp::PartialEq,
)]
#[xelement(
    name = "documentation",
    namespace = "http://www.w3.org/2001/XMLSchema",
    allow_unknown_attributes = "any"
)]
pub struct Documentation {
    #[xattribute(name = "source", optional, default)]
    pub source: ::core::option::Option<String>,
    #[builder(default)]
    pub particle: ::std::vec::Vec<XmlValueDocChild>,
}

#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::bon::Builder,
    ::core::cmp::PartialEq,
)]
#[xelement(
    name = "annotation",
    namespace = "http://www.w3.org/2001/XMLSchema",
    allow_unknown_attributes = "any"
)]
pub struct Annotation {
    #[xattribute(name = "id", optional, default)]
    pub id: Option<String>,
    #[xvalue(default)]
    #[builder(default)]
    pub annotation: Vec<Documentation>,
}

#[derive(
    ::core::fmt::Debug,
    ::xmlity::SerializationGroup,
    ::xmlity::DeserializationGroup,
    ::bon::Builder,
    ::core::cmp::PartialEq,
)]
#[xgroup(children_order = "strict")]
pub struct AttributeT {
    #[xattribute(name = "name", optional, default)]
    pub name: Option<::xmlity::LocalName<'static>>,
    #[xattribute(name = "type", optional, default)]
    pub type_: Option<xs_custom::QName>,
    #[xvalue(default)]
    pub annotation: Option<Box<Annotation>>,
}

#[derive(
    ::core::fmt::Debug, ::xmlity::Serialize, ::xmlity::Deserialize, ::core::cmp::PartialEq,
)]
pub enum Attribute {
    #[xelement(
        name = "attribute",
        namespace = "http://www.w3.org/2001/XMLSchema",
        allow_unknown_attributes = "any"
    )]
    Attribute(#[xgroup] Box<AttributeT>),
    #[xelement(
        name = "attributeGroup",
        namespace = "http://www.w3.org/2001/XMLSchema",
        allow_unknown_attributes = "any"
    )]
    AttributeGroup,
}

#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::bon::Builder,
    ::core::cmp::PartialEq,
)]
#[xvalue(order = "strict")]
pub struct AttrDecls {
    #[xvalue(default)]
    #[builder(default)]
    pub attribute: Vec<Attribute>,
    #[xvalue(default)]
    pub any_attribute: Option<Box<D>>,
}

#[derive(
    ::core::fmt::Debug,
    ::xmlity::SerializationGroup,
    ::xmlity::DeserializationGroup,
    ::bon::Builder,
    ::core::cmp::PartialEq,
)]
#[xgroup(children_order = "strict")]
pub struct ExtensionType {
    #[xattribute(name = "id", optional, default)]
    pub id: Option<String>,
    #[xattribute(name = "base")]
    pub base: xs_custom::QName,
    #[xvalue(default)]
    pub type_def_particle: Option<B>,
    pub attr_decls: Box<AttrDecls>,
}

#[derive(
    ::core::fmt::Debug, ::xmlity::Serialize, ::xmlity::Deserialize, ::core::cmp::PartialEq,
)]
#[xelement(
    name = "b",
    namespace = "http://www.w3.org/2001/XMLSchema",
    allow_unknown_attributes = "any"
)]
pub struct B;

#[derive(
    ::core::fmt::Debug, ::xmlity::Serialize, ::xmlity::Deserialize, ::core::cmp::PartialEq,
)]
#[xelement(
    name = "c",
    namespace = "http://www.w3.org/2001/XMLSchema",
    allow_unknown_attributes = "any"
)]
pub struct C;

#[derive(
    ::core::fmt::Debug, ::xmlity::Serialize, ::xmlity::Deserialize, ::core::cmp::PartialEq,
)]
#[xelement(
    name = "d",
    namespace = "http://www.w3.org/2001/XMLSchema",
    allow_unknown_attributes = "any"
)]
pub struct D;

#[derive(
    ::core::fmt::Debug, ::xmlity::Serialize, ::xmlity::Deserialize, ::core::cmp::PartialEq,
)]
#[xelement(
    name = "extension",
    namespace = "http://www.w3.org/2001/XMLSchema",
    allow_unknown_attributes = "any"
)]
pub struct Extension1(#[xgroup] Box<xs::types::ExtensionType>);

#[derive(
    ::core::fmt::Debug, ::xmlity::Serialize, ::xmlity::Deserialize, ::core::cmp::PartialEq,
)]
#[xelement(
    name = "extension",
    namespace = "http://www.w3.org/2001/XMLSchema",
    allow_unknown_attributes = "any"
)]
pub struct Extension2(#[xgroup] ExtensionType);

#[test]
#[ntest::timeout(1000)]
fn fix_test() {
    println!("Choice");

    let xml = CHOICE_XML.trim();
    let _: xs::Choice = xmlity_quick_xml::de::from_str(xml).unwrap();

    println!("TypeDefParticle");

    let xml = CHOICE_XML.trim();
    let _: xs::groups::TypeDefParticle = xmlity_quick_xml::de::from_str(xml).unwrap();

    println!("Extension2");

    let xml = EXTENSION2_XML.trim();
    let _: Extension2 = xmlity_quick_xml::de::from_str(xml).unwrap();

    println!("Extension1");

    let xml = EXTENSION_XML.trim();
    let _: Extension1 = xmlity_quick_xml::de::from_str(xml).unwrap();

    println!("Child1");

    let xml = EXTENSION_XML.trim();
    let _: xs::complex_content_items::Child1 = xmlity_quick_xml::de::from_str(xml).unwrap();

    println!("ComplexContent");

    let xml = COMPLEX_CONTENT_XML.trim();
    let _: xs::ComplexContent = xmlity_quick_xml::de::from_str(xml).unwrap();

    println!("Type");

    let xml = COMPLEX_TYPE_XML.trim();
    let _: xs::types::top_level_element_items::Type = xmlity_quick_xml::de::from_str(xml).unwrap();

    println!("Element");

    let xml = ELEMENT_XML.trim();
    let _: xs::Element = xmlity_quick_xml::de::from_str(xml).unwrap();
    println!("Done.");
}
