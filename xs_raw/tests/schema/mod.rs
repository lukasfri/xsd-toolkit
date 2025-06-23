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

#[test]
fn devtest() {
    let xml = r###"<xs:annotation xmlns:xs="http://www.w3.org/2001/XMLSchema">
          <xs:documentation>
   This branch is short for
   &lt;complexContent>
   &lt;restriction base="xs:anyType">
   ...
   &lt;/restriction>
   &lt;/complexContent></xs:documentation>
        </xs:annotation>"###;

    let seq: xs::Annotation = xmlity_quick_xml::from_str(xml).unwrap();

    println!("{:?}", seq);

    let xml = r###"<xs:sequence xmlns:xs="http://www.w3.org/2001/XMLSchema" id="TestId">
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
      </xs:sequence>"###;

    let seq: xs::Sequence = xmlity_quick_xml::from_str(xml).unwrap();

    println!("{:?}", seq);
}
