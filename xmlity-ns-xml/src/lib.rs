pub mod types {}
pub mod attributes {
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::SerializeAttribute,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xattribute(name = "base", namespace = "http://www.w3.org/XML/1998/namespace")]
    pub struct Base(pub String);
    impl ::core::convert::From<String> for Base {
        fn from(value: String) -> Self {
            Base(value)
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::SerializeAttribute,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xattribute(name = "id", namespace = "http://www.w3.org/XML/1998/namespace")]
    pub struct Id(pub String);
    impl ::core::convert::From<String> for Id {
        fn from(value: String) -> Self {
            Id(value)
        }
    }
    pub mod lang_items {
        pub mod variant_0_variants {
            #[derive(
                ::core::fmt::Debug,
                ::core::clone::Clone,
                ::core::marker::Copy,
                ::xmlity::Serialize,
                ::xmlity::Deserialize,
                ::core::cmp::PartialEq
            )]
            #[xvalue(with = variant_0_with)]
            pub enum Variant0 {
                Empty,
            }
            pub mod variant_0_with {
                pub fn deserialize<'de, D>(
                    deserializer: D,
                ) -> ::core::result::Result<super::Variant0, D::Error>
                where
                    D: ::xmlity::Deserializer<'de>,
                {
                    let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                        deserializer,
                    )?;
                    let value: ::std::string::String = text
                        .parse()
                        .map_err(::xmlity::de::Error::custom)?;
                    super::Variant0::try_from(value).map_err(::xmlity::de::Error::custom)
                }
                pub fn serialize<S>(
                    value: &super::Variant0,
                    serializer: S,
                ) -> ::core::result::Result<S::Ok, S::Error>
                where
                    S: ::xmlity::Serializer,
                {
                    let value: ::std::string::String = ::core::clone::Clone::clone(value)
                        .into();
                    ::xmlity::Serialize::serialize(
                        ::std::string::String::as_str(
                            &::std::string::ToString::to_string(&value),
                        ),
                        serializer,
                    )
                }
            }
            #[derive(::core::fmt::Debug)]
            pub enum Variant0ParseError {
                NonExistent { value: ::std::string::String },
            }
            impl ::core::fmt::Display for Variant0ParseError {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::result::Result<(), ::core::fmt::Error> {
                    match self {
                        Variant0ParseError::NonExistent { value } => {
                            write!(
                                f, "Value '{:?}' does not exist in the enumeration", value
                            )
                        }
                    }
                }
            }
            impl ::core::convert::TryFrom<::std::string::String> for Variant0 {
                type Error = Variant0ParseError;
                fn try_from(
                    value: ::std::string::String,
                ) -> ::core::result::Result<Self, Self::Error> {
                    match ::std::string::String::as_str(&value) {
                        "" => Ok(Variant0::Empty),
                        _ => {
                            Err(Variant0ParseError::NonExistent {
                                value,
                            })
                        }
                    }
                }
            }
            impl ::core::convert::From<Variant0> for ::std::string::String {
                fn from(value: Variant0) -> Self {
                    match value {
                        Variant0::Empty => ::std::string::String::from(""),
                    }
                }
            }
        }
        impl ::core::convert::From<String> for Lang {
            fn from(value: String) -> Self {
                Lang::Variant0(::std::boxed::Box::new(value))
            }
        }
        impl ::core::convert::From<variant_0_variants::Variant0> for Lang {
            fn from(value: variant_0_variants::Variant0) -> Self {
                Lang::Variant0_0(::std::boxed::Box::new(value))
            }
        }
        #[derive(
            ::core::fmt::Debug,
            ::xmlity::Serialize,
            ::xmlity::Deserialize,
            ::core::cmp::PartialEq,
            ::core::clone::Clone
        )]
        pub enum Lang {
            Variant0(::std::boxed::Box<String>),
            Variant0_0(::std::boxed::Box<variant_0_variants::Variant0>),
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::SerializeAttribute,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xattribute(name = "lang", namespace = "http://www.w3.org/XML/1998/namespace")]
    pub struct Lang(pub lang_items::Lang);
    impl ::core::convert::From<lang_items::Lang> for Lang {
        fn from(value: lang_items::Lang) -> Self {
            Lang(value)
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::SerializeAttribute,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xattribute(name = "space", namespace = "http://www.w3.org/XML/1998/namespace")]
    pub struct Space(pub ::std::string::String);
    impl ::core::convert::From<::std::string::String> for Space {
        fn from(value: ::std::string::String) -> Self {
            Space(value)
        }
    }
}
pub mod groups {}
