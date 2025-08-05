pub mod attributes {
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::SerializeAttribute,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xattribute(name = "nil", namespace = "http://www.w3.org/2001/XMLSchema-instance")]
    pub struct Nil(pub String);
    impl ::core::convert::From<String> for Nil {
        fn from(value: String) -> Self {
            Nil(value)
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::SerializeAttribute,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xattribute(
        name = "noNamespaceSchemaLocation",
        namespace = "http://www.w3.org/2001/XMLSchema-instance"
    )]
    pub struct NoNamespaceSchemaLocation(pub String);
    impl ::core::convert::From<String> for NoNamespaceSchemaLocation {
        fn from(value: String) -> Self {
            NoNamespaceSchemaLocation(value)
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::SerializeAttribute,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xattribute(
        name = "schemaLocation",
        namespace = "http://www.w3.org/2001/XMLSchema-instance"
    )]
    pub struct SchemaLocation(pub String);
    impl ::core::convert::From<String> for SchemaLocation {
        fn from(value: String) -> Self {
            SchemaLocation(value)
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::SerializeAttribute,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xattribute(name = "type", namespace = "http://www.w3.org/2001/XMLSchema-instance")]
    pub struct Type(pub String);
    impl ::core::convert::From<String> for Type {
        fn from(value: String) -> Self {
            Type(value)
        }
    }
}
