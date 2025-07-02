pub mod xhtml;

pub mod xml {
    use xmlity::{Deserialize, SerializeAttribute, XmlNamespace};

    #[derive(Debug, Clone, SerializeAttribute, Deserialize, PartialEq)]
    #[xattribute(name = "space", namespace_expr = XmlNamespace::XML)]
    pub struct Space(pub String);

    #[derive(Debug, Clone, SerializeAttribute, Deserialize, PartialEq)]
    #[xattribute(name = "lang", namespace_expr = XmlNamespace::XML)]
    pub struct Lang(pub String);
}
