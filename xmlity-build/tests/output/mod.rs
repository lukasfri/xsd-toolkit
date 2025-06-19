pub mod xhtml;
pub mod xs;

pub mod xml {
    use xmlity::{Deserialize, SerializeAttribute, XmlNamespace};

    #[derive(Debug, SerializeAttribute, Deserialize)]
    #[xattribute(name = "space", namespace_expr = XmlNamespace::XML)]
    pub struct Space(pub String);

    #[derive(Debug, SerializeAttribute, Deserialize)]
    #[xattribute(name = "lang", namespace_expr = XmlNamespace::XML)]
    pub struct Lang(pub String);
}

#[test]
fn head() {
    const XML: &str = r###"
<head xmlns="http://www.w3.org/1999/xhtml">
  <title>Title of document</title>
</head>
"###;

    let html: xhtml::Head = xmlity_quick_xml::from_str(XML.trim()).unwrap();
    println!("{html:#?}");
}

#[test]
fn html() {
    const XML: &str = r###"
<html xmlns="http://www.w3.org/1999/xhtml">
<head>
  <title>Title of document</title>
</head>
<body>

  some content here...

</body>
</html>
"###;

    let html: xhtml::Html = xmlity_quick_xml::from_str(XML.trim()).unwrap();
    println!("{html:#?}");
}

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

#[test]
fn xsd_import() {
    let import: xs::Element = xmlity_quick_xml::from_str(XSD_IMPORT.trim()).unwrap();
    println!("{import:#?}");
}

const XSD_IMPORT_LOCAL_COMPLEX_TYPE: &str = r###"
<xs:complexContent xmlns:xs="http://www.w3.org/2001/XMLSchema">
    <xs:extension base="xs:annotated">
        <xs:attribute name="namespace" type="xs:anyURI"/>
        <xs:attribute name="schemaLocation" type="xs:anyURI"/>
    </xs:extension>
</xs:complexContent>
"###;

#[test]
fn xsd_import_content() {
    let import: crate::xs::ComplexContent =
        xmlity_quick_xml::from_str(XSD_IMPORT_LOCAL_COMPLEX_TYPE.trim()).unwrap();
    println!("{import:#?}");
}

const XSD_ATTRIBUTE: &str = r###"<xs:attribute xmlns:xs="http://www.w3.org/2001/XMLSchema" name="namespace" type="xs:anyURI"/>"###;

#[test]
fn xsd_attribute() {
    let attr_decls: crate::xs::groups::AttrDecls =
        xmlity_quick_xml::from_str(XSD_ATTRIBUTE).unwrap();
    println!("{attr_decls:#?}");
    let attr: crate::xs::groups::attr_decls_items::Attribute =
        xmlity_quick_xml::from_str(XSD_ATTRIBUTE).unwrap();
    println!("{attr:#?}");
}
