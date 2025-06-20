pub mod xs;

pub mod xml {
    use xmlity::{Deserialize, SerializeAttribute, XmlNamespace};

    #[derive(Debug, SerializeAttribute, Deserialize, PartialEq)]
    #[xattribute(name = "space", namespace_expr = XmlNamespace::XML)]
    pub struct Space(pub String);

    #[derive(Debug, SerializeAttribute, Deserialize, PartialEq)]
    #[xattribute(name = "lang", namespace_expr = XmlNamespace::XML)]
    pub struct Lang(pub String);
}

mod qname {
    use core::fmt;
    use std::str::FromStr;

    use xmlity::{
        de::NamespaceContext, Deserialize, ExpandedName, LocalName, Prefix, Serialize, XmlNamespace,
    };

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct QName(pub ExpandedName<'static>);

    struct QNameVisitor;

    impl<'de> xmlity::de::Visitor<'de> for QNameVisitor {
        type Value = QName;
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a QName")
        }

        fn visit_text<E, V>(self, value: V) -> Result<Self::Value, E>
        where
            E: xmlity::de::Error,
            V: xmlity::de::XmlText<'de>,
        {
            let mut qname_parts = value.as_str().split(":");
            let first_part = qname_parts.next().expect("Always has at least one part.");
            let last_part = qname_parts.next();

            let expanded_name = match last_part {
                Some(last_part) => {
                    let prefix = Prefix::new(first_part).unwrap();
                    let local_name = LocalName::new(last_part).unwrap().into_owned();

                    let namespace = value
                        .namespace_context()
                        .resolve_prefix(prefix)
                        .unwrap()
                        .into_owned();

                    ExpandedName::new(local_name, Some(namespace))
                }
                None => {
                    let local_name = LocalName::new(first_part).unwrap().into_owned();

                    let default_namespace = value
                        .namespace_context()
                        .default_namespace()
                        .map(XmlNamespace::into_owned);

                    return Ok(QName(ExpandedName::new(local_name, default_namespace)));
                }
            };

            Ok(QName(expanded_name))
        }
    }

    impl<'de> Deserialize<'de> for QName {
        fn deserialize<D: xmlity::Deserializer<'de>>(reader: D) -> Result<Self, D::Error> {
            reader.deserialize_any(QNameVisitor)
        }
    }

    impl FromStr for QName {
        type Err = String;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            //TODO: REDO REQUIRES SUPPORT IN XMLITY
            let mut name_parts = s.split(":");
            let mut localname = name_parts.next().unwrap();

            if let Some(localname_new) = name_parts.next() {
                localname = localname_new;
            }
            let local_name = LocalName::new(localname).unwrap().into_owned();

            let expanded_name = if s.starts_with("xs:") {
                ExpandedName::new(local_name, Some(XmlNamespace::XS))
            } else if s.starts_with("xml:") {
                ExpandedName::new(local_name, Some(XmlNamespace::XML))
            } else {
                todo!()
            };

            Ok(QName(expanded_name))
        }
    }

    impl fmt::Display for QName {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    impl Serialize for QName {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: xmlity::Serializer,
        {
            serializer.serialize_text(&self.to_string())
        }
    }
}

pub use qname::QName;

#[cfg(test)]
mod tests {
    use super::*;

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
    #[ntest::timeout(1000)]
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
    #[ntest::timeout(1000)]
    fn xsd_import_content() {
        let import: crate::xs::ComplexContent =
            xmlity_quick_xml::from_str(XSD_IMPORT_LOCAL_COMPLEX_TYPE.trim()).unwrap();
        println!("{import:#?}");
    }

    const XSD_ATTRIBUTE: &str = r###"<xs:attribute xmlns:xs="http://www.w3.org/2001/XMLSchema" name="namespace" type="xs:anyURI"/>"###;

    #[test]
    #[ntest::timeout(1000)]
    fn xsd_attribute() {
        let attr_decls: crate::xs::groups::AttrDecls =
            xmlity_quick_xml::from_str(XSD_ATTRIBUTE).unwrap();
        println!("{attr_decls:#?}");
        let attr: crate::xs::groups::attr_decls_items::Attribute =
            xmlity_quick_xml::from_str(XSD_ATTRIBUTE).unwrap();
        println!("{attr:#?}");
    }
}
