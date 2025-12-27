pub mod types {
    pub mod cdata_items {
        impl ::core::convert::From<::std::string::String> for Cdata {
            fn from(value: ::std::string::String) -> Self {
                Cdata(value)
            }
        }
        #[derive(
            ::core::fmt::Debug,
            ::xmlity::Serialize,
            ::xmlity::Deserialize,
            ::core::cmp::PartialEq,
            ::core::clone::Clone
        )]
        #[xvalue(with = cdata_with)]
        pub struct Cdata(pub ::std::string::String);
        pub mod cdata_with {
            pub fn deserialize<'de, D>(
                deserializer: D,
            ) -> ::core::result::Result<super::Cdata, D::Error>
            where
                D: ::xmlity::Deserializer<'de>,
            {
                let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                    deserializer,
                )?;
                let value: ::std::string::String = text
                    .parse()
                    .map_err(::xmlity::de::Error::custom)?;
                super::Cdata::try_from(value).map_err(::xmlity::de::Error::custom)
            }
            pub fn serialize<S>(
                value: &super::Cdata,
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
        #[derive(::core::fmt::Debug, ::core::cmp::PartialEq, ::core::clone::Clone)]
        pub enum CdataParseError {}
        impl ::core::convert::From<Cdata> for ::std::string::String {
            fn from(value: Cdata) -> Self {
                value.0
            }
        }
    }
    pub type Cdata = cdata_items::Cdata;
    pub mod character_items {
        impl ::core::convert::From<::std::string::String> for Character {
            fn from(value: ::std::string::String) -> Self {
                Character(value)
            }
        }
        #[derive(
            ::core::fmt::Debug,
            ::xmlity::Serialize,
            ::xmlity::Deserialize,
            ::core::cmp::PartialEq,
            ::core::clone::Clone
        )]
        #[xvalue(with = character_with)]
        pub struct Character(pub ::std::string::String);
        pub mod character_with {
            pub fn deserialize<'de, D>(
                deserializer: D,
            ) -> ::core::result::Result<super::Character, D::Error>
            where
                D: ::xmlity::Deserializer<'de>,
            {
                let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                    deserializer,
                )?;
                let value: ::std::string::String = text
                    .parse()
                    .map_err(::xmlity::de::Error::custom)?;
                super::Character::try_from(value).map_err(::xmlity::de::Error::custom)
            }
            pub fn serialize<S>(
                value: &super::Character,
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
        #[derive(::core::fmt::Debug, ::core::cmp::PartialEq, ::core::clone::Clone)]
        pub enum CharacterParseError {}
        impl ::core::convert::From<Character> for ::std::string::String {
            fn from(value: Character) -> Self {
                value.0
            }
        }
    }
    pub type Character = character_items::Character;
    pub mod charset_items {
        impl ::core::convert::From<::std::string::String> for Charset {
            fn from(value: ::std::string::String) -> Self {
                Charset(value)
            }
        }
        #[derive(
            ::core::fmt::Debug,
            ::xmlity::Serialize,
            ::xmlity::Deserialize,
            ::core::cmp::PartialEq,
            ::core::clone::Clone
        )]
        #[xvalue(with = charset_with)]
        pub struct Charset(pub ::std::string::String);
        pub mod charset_with {
            pub fn deserialize<'de, D>(
                deserializer: D,
            ) -> ::core::result::Result<super::Charset, D::Error>
            where
                D: ::xmlity::Deserializer<'de>,
            {
                let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                    deserializer,
                )?;
                let value: ::std::string::String = text
                    .parse()
                    .map_err(::xmlity::de::Error::custom)?;
                super::Charset::try_from(value).map_err(::xmlity::de::Error::custom)
            }
            pub fn serialize<S>(
                value: &super::Charset,
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
        #[derive(::core::fmt::Debug, ::core::cmp::PartialEq, ::core::clone::Clone)]
        pub enum CharsetParseError {}
        impl ::core::convert::From<Charset> for ::std::string::String {
            fn from(value: Charset) -> Self {
                value.0
            }
        }
    }
    pub type Charset = charset_items::Charset;
    pub type Charsets = ::xmlity_ns::List<
        ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Charset>,
    >;
    pub mod color_items {
        pub mod color_variants {
            impl ::core::convert::From<::std::string::String> for Variant0 {
                fn from(value: ::std::string::String) -> Self {
                    Variant0(value)
                }
            }
            #[derive(
                ::core::fmt::Debug,
                ::xmlity::Serialize,
                ::xmlity::Deserialize,
                ::core::cmp::PartialEq,
                ::core::clone::Clone
            )]
            #[xvalue(with = variant_0_with)]
            pub struct Variant0(pub ::std::string::String);
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
            #[derive(::core::fmt::Debug, ::core::cmp::PartialEq, ::core::clone::Clone)]
            pub enum Variant0ParseError {}
            impl ::core::convert::From<Variant0> for ::std::string::String {
                fn from(value: Variant0) -> Self {
                    value.0
                }
            }
        }
        impl ::core::convert::From<::xmlity_ns_xsd_types::NMToken> for Color {
            fn from(value: ::xmlity_ns_xsd_types::NMToken) -> Self {
                Color::Nmtoken(::std::boxed::Box::new(value))
            }
        }
        impl ::core::convert::From<color_variants::Variant0> for Color {
            fn from(value: color_variants::Variant0) -> Self {
                Color::Variant0(::std::boxed::Box::new(value))
            }
        }
        #[derive(
            ::core::fmt::Debug,
            ::xmlity::Serialize,
            ::xmlity::Deserialize,
            ::core::cmp::PartialEq,
            ::core::clone::Clone
        )]
        pub enum Color {
            Nmtoken(::std::boxed::Box<::xmlity_ns_xsd_types::NMToken>),
            Variant0(::std::boxed::Box<color_variants::Variant0>),
        }
    }
    pub type Color = color_items::Color;
    pub mod content_type_items {
        impl ::core::convert::From<::std::string::String> for ContentType {
            fn from(value: ::std::string::String) -> Self {
                ContentType(value)
            }
        }
        #[derive(
            ::core::fmt::Debug,
            ::xmlity::Serialize,
            ::xmlity::Deserialize,
            ::core::cmp::PartialEq,
            ::core::clone::Clone
        )]
        #[xvalue(with = content_type_with)]
        pub struct ContentType(pub ::std::string::String);
        pub mod content_type_with {
            pub fn deserialize<'de, D>(
                deserializer: D,
            ) -> ::core::result::Result<super::ContentType, D::Error>
            where
                D: ::xmlity::Deserializer<'de>,
            {
                let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                    deserializer,
                )?;
                let value: ::std::string::String = text
                    .parse()
                    .map_err(::xmlity::de::Error::custom)?;
                super::ContentType::try_from(value).map_err(::xmlity::de::Error::custom)
            }
            pub fn serialize<S>(
                value: &super::ContentType,
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
        #[derive(::core::fmt::Debug, ::core::cmp::PartialEq, ::core::clone::Clone)]
        pub enum ContentTypeParseError {}
        impl ::core::convert::From<ContentType> for ::std::string::String {
            fn from(value: ContentType) -> Self {
                value.0
            }
        }
    }
    pub type ContentType = content_type_items::ContentType;
    pub mod content_types_items {
        impl ::core::convert::From<::std::string::String> for ContentTypes {
            fn from(value: ::std::string::String) -> Self {
                ContentTypes(value)
            }
        }
        #[derive(
            ::core::fmt::Debug,
            ::xmlity::Serialize,
            ::xmlity::Deserialize,
            ::core::cmp::PartialEq,
            ::core::clone::Clone
        )]
        #[xvalue(with = content_types_with)]
        pub struct ContentTypes(pub ::std::string::String);
        pub mod content_types_with {
            pub fn deserialize<'de, D>(
                deserializer: D,
            ) -> ::core::result::Result<super::ContentTypes, D::Error>
            where
                D: ::xmlity::Deserializer<'de>,
            {
                let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                    deserializer,
                )?;
                let value: ::std::string::String = text
                    .parse()
                    .map_err(::xmlity::de::Error::custom)?;
                super::ContentTypes::try_from(value).map_err(::xmlity::de::Error::custom)
            }
            pub fn serialize<S>(
                value: &super::ContentTypes,
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
        #[derive(::core::fmt::Debug, ::core::cmp::PartialEq, ::core::clone::Clone)]
        pub enum ContentTypesParseError {}
        impl ::core::convert::From<ContentTypes> for ::std::string::String {
            fn from(value: ContentTypes) -> Self {
                value.0
            }
        }
    }
    pub type ContentTypes = content_types_items::ContentTypes;
    pub mod datetime_items {
        impl ::core::convert::From<::std::string::String> for Datetime {
            fn from(value: ::std::string::String) -> Self {
                Datetime(value)
            }
        }
        #[derive(
            ::core::fmt::Debug,
            ::xmlity::Serialize,
            ::xmlity::Deserialize,
            ::core::cmp::PartialEq,
            ::core::clone::Clone
        )]
        #[xvalue(with = datetime_with)]
        pub struct Datetime(pub ::std::string::String);
        pub mod datetime_with {
            pub fn deserialize<'de, D>(
                deserializer: D,
            ) -> ::core::result::Result<super::Datetime, D::Error>
            where
                D: ::xmlity::Deserializer<'de>,
            {
                let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                    deserializer,
                )?;
                let value: ::std::string::String = text
                    .parse()
                    .map_err(::xmlity::de::Error::custom)?;
                super::Datetime::try_from(value).map_err(::xmlity::de::Error::custom)
            }
            pub fn serialize<S>(
                value: &super::Datetime,
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
        #[derive(::core::fmt::Debug, ::core::cmp::PartialEq, ::core::clone::Clone)]
        pub enum DatetimeParseError {}
        impl ::core::convert::From<Datetime> for ::std::string::String {
            fn from(value: Datetime) -> Self {
                value.0
            }
        }
    }
    pub type Datetime = datetime_items::Datetime;
    pub mod fpi_items {
        impl ::core::convert::From<::std::string::String> for Fpi {
            fn from(value: ::std::string::String) -> Self {
                Fpi(value)
            }
        }
        #[derive(
            ::core::fmt::Debug,
            ::xmlity::Serialize,
            ::xmlity::Deserialize,
            ::core::cmp::PartialEq,
            ::core::clone::Clone
        )]
        #[xvalue(with = fpi_with)]
        pub struct Fpi(pub ::std::string::String);
        pub mod fpi_with {
            pub fn deserialize<'de, D>(
                deserializer: D,
            ) -> ::core::result::Result<super::Fpi, D::Error>
            where
                D: ::xmlity::Deserializer<'de>,
            {
                let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                    deserializer,
                )?;
                let value: ::std::string::String = text
                    .parse()
                    .map_err(::xmlity::de::Error::custom)?;
                super::Fpi::try_from(value).map_err(::xmlity::de::Error::custom)
            }
            pub fn serialize<S>(
                value: &super::Fpi,
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
        #[derive(::core::fmt::Debug, ::core::cmp::PartialEq, ::core::clone::Clone)]
        pub enum FpiParseError {}
        impl ::core::convert::From<Fpi> for ::std::string::String {
            fn from(value: Fpi) -> Self {
                value.0
            }
        }
    }
    pub type Fpi = fpi_items::Fpi;
    pub mod language_code_items {
        impl ::core::convert::From<::std::string::String> for LanguageCode {
            fn from(value: ::std::string::String) -> Self {
                LanguageCode(value)
            }
        }
        #[derive(
            ::core::fmt::Debug,
            ::xmlity::Serialize,
            ::xmlity::Deserialize,
            ::core::cmp::PartialEq,
            ::core::clone::Clone
        )]
        #[xvalue(with = language_code_with)]
        pub struct LanguageCode(pub ::std::string::String);
        pub mod language_code_with {
            pub fn deserialize<'de, D>(
                deserializer: D,
            ) -> ::core::result::Result<super::LanguageCode, D::Error>
            where
                D: ::xmlity::Deserializer<'de>,
            {
                let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                    deserializer,
                )?;
                let value: ::std::string::String = text
                    .parse()
                    .map_err(::xmlity::de::Error::custom)?;
                super::LanguageCode::try_from(value).map_err(::xmlity::de::Error::custom)
            }
            pub fn serialize<S>(
                value: &super::LanguageCode,
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
        #[derive(::core::fmt::Debug, ::core::cmp::PartialEq, ::core::clone::Clone)]
        pub enum LanguageCodeParseError {}
        impl ::core::convert::From<LanguageCode> for ::std::string::String {
            fn from(value: LanguageCode) -> Self {
                value.0
            }
        }
    }
    pub type LanguageCode = language_code_items::LanguageCode;
    pub mod length_items {
        pub mod length_variants {
            impl ::core::convert::From<::std::string::String> for Variant0 {
                fn from(value: ::std::string::String) -> Self {
                    Variant0(value)
                }
            }
            #[derive(
                ::core::fmt::Debug,
                ::xmlity::Serialize,
                ::xmlity::Deserialize,
                ::core::cmp::PartialEq,
                ::core::clone::Clone
            )]
            #[xvalue(with = variant_0_with)]
            pub struct Variant0(pub ::std::string::String);
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
            #[derive(::core::fmt::Debug, ::core::cmp::PartialEq, ::core::clone::Clone)]
            pub enum Variant0ParseError {}
            impl ::core::convert::From<Variant0> for ::std::string::String {
                fn from(value: Variant0) -> Self {
                    value.0
                }
            }
        }
        impl ::core::convert::From<usize> for Length {
            fn from(value: usize) -> Self {
                Length::NonNegativeInteger(::std::boxed::Box::new(value))
            }
        }
        impl ::core::convert::From<length_variants::Variant0> for Length {
            fn from(value: length_variants::Variant0) -> Self {
                Length::Variant0(::std::boxed::Box::new(value))
            }
        }
        #[derive(
            ::core::fmt::Debug,
            ::xmlity::Serialize,
            ::xmlity::Deserialize,
            ::core::cmp::PartialEq,
            ::core::clone::Clone
        )]
        pub enum Length {
            NonNegativeInteger(::std::boxed::Box<usize>),
            Variant0(::std::boxed::Box<length_variants::Variant0>),
        }
    }
    pub type Length = length_items::Length;
    pub type LinkTypes = ::xmlity_ns::List<::xmlity_ns_xsd_types::NMToken>;
    pub mod media_desc_items {
        impl ::core::convert::From<::std::string::String> for MediaDesc {
            fn from(value: ::std::string::String) -> Self {
                MediaDesc(value)
            }
        }
        #[derive(
            ::core::fmt::Debug,
            ::xmlity::Serialize,
            ::xmlity::Deserialize,
            ::core::cmp::PartialEq,
            ::core::clone::Clone
        )]
        #[xvalue(with = media_desc_with)]
        pub struct MediaDesc(pub ::std::string::String);
        pub mod media_desc_with {
            pub fn deserialize<'de, D>(
                deserializer: D,
            ) -> ::core::result::Result<super::MediaDesc, D::Error>
            where
                D: ::xmlity::Deserializer<'de>,
            {
                let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                    deserializer,
                )?;
                let value: ::std::string::String = text
                    .parse()
                    .map_err(::xmlity::de::Error::custom)?;
                super::MediaDesc::try_from(value).map_err(::xmlity::de::Error::custom)
            }
            pub fn serialize<S>(
                value: &super::MediaDesc,
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
        #[derive(::core::fmt::Debug, ::core::cmp::PartialEq, ::core::clone::Clone)]
        pub enum MediaDescParseError {}
        impl ::core::convert::From<MediaDesc> for ::std::string::String {
            fn from(value: MediaDesc) -> Self {
                value.0
            }
        }
    }
    pub type MediaDesc = media_desc_items::MediaDesc;
    pub mod multi_length_items {
        pub mod multi_length_variants {
            impl ::core::convert::From<::std::string::String> for Variant0 {
                fn from(value: ::std::string::String) -> Self {
                    Variant0(value)
                }
            }
            #[derive(
                ::core::fmt::Debug,
                ::xmlity::Serialize,
                ::xmlity::Deserialize,
                ::core::cmp::PartialEq,
                ::core::clone::Clone
            )]
            #[xvalue(with = variant_0_with)]
            pub struct Variant0(pub ::std::string::String);
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
            #[derive(::core::fmt::Debug, ::core::cmp::PartialEq, ::core::clone::Clone)]
            pub enum Variant0ParseError {}
            impl ::core::convert::From<Variant0> for ::std::string::String {
                fn from(value: Variant0) -> Self {
                    value.0
                }
            }
        }
        impl ::core::convert::From<crate::xhtml_schema::xhtml_datatypes::types::Length>
        for MultiLength {
            fn from(value: crate::xhtml_schema::xhtml_datatypes::types::Length) -> Self {
                MultiLength::Length(::std::boxed::Box::new(value))
            }
        }
        impl ::core::convert::From<multi_length_variants::Variant0> for MultiLength {
            fn from(value: multi_length_variants::Variant0) -> Self {
                MultiLength::Variant0(::std::boxed::Box::new(value))
            }
        }
        #[derive(
            ::core::fmt::Debug,
            ::xmlity::Serialize,
            ::xmlity::Deserialize,
            ::core::cmp::PartialEq,
            ::core::clone::Clone
        )]
        pub enum MultiLength {
            Length(
                ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Length>,
            ),
            Variant0(::std::boxed::Box<multi_length_variants::Variant0>),
        }
    }
    pub type MultiLength = multi_length_items::MultiLength;
    pub mod multi_lengths_items {
        impl ::core::convert::From<::std::string::String> for MultiLengths {
            fn from(value: ::std::string::String) -> Self {
                MultiLengths(value)
            }
        }
        #[derive(
            ::core::fmt::Debug,
            ::xmlity::Serialize,
            ::xmlity::Deserialize,
            ::core::cmp::PartialEq,
            ::core::clone::Clone
        )]
        #[xvalue(with = multi_lengths_with)]
        pub struct MultiLengths(pub ::std::string::String);
        pub mod multi_lengths_with {
            pub fn deserialize<'de, D>(
                deserializer: D,
            ) -> ::core::result::Result<super::MultiLengths, D::Error>
            where
                D: ::xmlity::Deserializer<'de>,
            {
                let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                    deserializer,
                )?;
                let value: ::std::string::String = text
                    .parse()
                    .map_err(::xmlity::de::Error::custom)?;
                super::MultiLengths::try_from(value).map_err(::xmlity::de::Error::custom)
            }
            pub fn serialize<S>(
                value: &super::MultiLengths,
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
        #[derive(::core::fmt::Debug, ::core::cmp::PartialEq, ::core::clone::Clone)]
        pub enum MultiLengthsParseError {}
        impl ::core::convert::From<MultiLengths> for ::std::string::String {
            fn from(value: MultiLengths) -> Self {
                value.0
            }
        }
    }
    pub type MultiLengths = multi_lengths_items::MultiLengths;
    pub mod number_items {
        impl ::core::convert::From<usize> for Number {
            fn from(value: usize) -> Self {
                Number(value)
            }
        }
        #[derive(
            ::core::fmt::Debug,
            ::xmlity::Serialize,
            ::xmlity::Deserialize,
            ::core::cmp::PartialEq,
            ::core::clone::Clone
        )]
        #[xvalue(with = number_with)]
        #[repr(transparent)]
        pub struct Number(pub usize);
        pub mod number_with {
            pub fn deserialize<'de, D>(
                deserializer: D,
            ) -> ::core::result::Result<super::Number, D::Error>
            where
                D: ::xmlity::Deserializer<'de>,
            {
                let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                    deserializer,
                )?;
                if text.is_empty() {
                    return super::Number::try_from(0usize)
                        .map_err(::xmlity::de::Error::custom);
                }
                let value: usize = text.parse().map_err(::xmlity::de::Error::custom)?;
                super::Number::try_from(value).map_err(::xmlity::de::Error::custom)
            }
            pub fn serialize<S>(
                value: &super::Number,
                serializer: S,
            ) -> ::core::result::Result<S::Ok, S::Error>
            where
                S: ::xmlity::Serializer,
            {
                let value: usize = ::core::clone::Clone::clone(value).into();
                ::xmlity::Serialize::serialize(
                    ::std::string::String::as_str(
                        &::std::string::ToString::to_string(&value),
                    ),
                    serializer,
                )
            }
        }
        #[derive(::core::fmt::Debug, ::core::cmp::PartialEq, ::core::clone::Clone)]
        pub enum NumberParseError {}
        impl ::core::convert::From<Number> for usize {
            fn from(value: Number) -> Self {
                value.0
            }
        }
    }
    pub type Number = number_items::Number;
    pub mod pixels_items {
        impl ::core::convert::From<usize> for Pixels {
            fn from(value: usize) -> Self {
                Pixels(value)
            }
        }
        #[derive(
            ::core::fmt::Debug,
            ::xmlity::Serialize,
            ::xmlity::Deserialize,
            ::core::cmp::PartialEq,
            ::core::clone::Clone
        )]
        #[xvalue(with = pixels_with)]
        #[repr(transparent)]
        pub struct Pixels(pub usize);
        pub mod pixels_with {
            pub fn deserialize<'de, D>(
                deserializer: D,
            ) -> ::core::result::Result<super::Pixels, D::Error>
            where
                D: ::xmlity::Deserializer<'de>,
            {
                let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                    deserializer,
                )?;
                if text.is_empty() {
                    return super::Pixels::try_from(0usize)
                        .map_err(::xmlity::de::Error::custom);
                }
                let value: usize = text.parse().map_err(::xmlity::de::Error::custom)?;
                super::Pixels::try_from(value).map_err(::xmlity::de::Error::custom)
            }
            pub fn serialize<S>(
                value: &super::Pixels,
                serializer: S,
            ) -> ::core::result::Result<S::Ok, S::Error>
            where
                S: ::xmlity::Serializer,
            {
                let value: usize = ::core::clone::Clone::clone(value).into();
                ::xmlity::Serialize::serialize(
                    ::std::string::String::as_str(
                        &::std::string::ToString::to_string(&value),
                    ),
                    serializer,
                )
            }
        }
        #[derive(::core::fmt::Debug, ::core::cmp::PartialEq, ::core::clone::Clone)]
        pub enum PixelsParseError {}
        impl ::core::convert::From<Pixels> for usize {
            fn from(value: Pixels) -> Self {
                value.0
            }
        }
    }
    pub type Pixels = pixels_items::Pixels;
    pub mod script_items {
        impl ::core::convert::From<::std::string::String> for Script {
            fn from(value: ::std::string::String) -> Self {
                Script(value)
            }
        }
        #[derive(
            ::core::fmt::Debug,
            ::xmlity::Serialize,
            ::xmlity::Deserialize,
            ::core::cmp::PartialEq,
            ::core::clone::Clone
        )]
        #[xvalue(with = script_with)]
        pub struct Script(pub ::std::string::String);
        pub mod script_with {
            pub fn deserialize<'de, D>(
                deserializer: D,
            ) -> ::core::result::Result<super::Script, D::Error>
            where
                D: ::xmlity::Deserializer<'de>,
            {
                let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                    deserializer,
                )?;
                let value: ::std::string::String = text
                    .parse()
                    .map_err(::xmlity::de::Error::custom)?;
                super::Script::try_from(value).map_err(::xmlity::de::Error::custom)
            }
            pub fn serialize<S>(
                value: &super::Script,
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
        #[derive(::core::fmt::Debug, ::core::cmp::PartialEq, ::core::clone::Clone)]
        pub enum ScriptParseError {}
        impl ::core::convert::From<Script> for ::std::string::String {
            fn from(value: Script) -> Self {
                value.0
            }
        }
    }
    pub type Script = script_items::Script;
    pub mod text_items {
        impl ::core::convert::From<::std::string::String> for Text {
            fn from(value: ::std::string::String) -> Self {
                Text(value)
            }
        }
        #[derive(
            ::core::fmt::Debug,
            ::xmlity::Serialize,
            ::xmlity::Deserialize,
            ::core::cmp::PartialEq,
            ::core::clone::Clone
        )]
        #[xvalue(with = text_with)]
        pub struct Text(pub ::std::string::String);
        pub mod text_with {
            pub fn deserialize<'de, D>(
                deserializer: D,
            ) -> ::core::result::Result<super::Text, D::Error>
            where
                D: ::xmlity::Deserializer<'de>,
            {
                let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                    deserializer,
                )?;
                let value: ::std::string::String = text
                    .parse()
                    .map_err(::xmlity::de::Error::custom)?;
                super::Text::try_from(value).map_err(::xmlity::de::Error::custom)
            }
            pub fn serialize<S>(
                value: &super::Text,
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
        #[derive(::core::fmt::Debug, ::core::cmp::PartialEq, ::core::clone::Clone)]
        pub enum TextParseError {}
        impl ::core::convert::From<Text> for ::std::string::String {
            fn from(value: Text) -> Self {
                value.0
            }
        }
    }
    pub type Text = text_items::Text;
    pub mod uri_items {
        impl ::core::convert::From<::std::string::String> for Uri {
            fn from(value: ::std::string::String) -> Self {
                Uri(value)
            }
        }
        #[derive(
            ::core::fmt::Debug,
            ::xmlity::Serialize,
            ::xmlity::Deserialize,
            ::core::cmp::PartialEq,
            ::core::clone::Clone
        )]
        #[xvalue(with = uri_with)]
        pub struct Uri(pub ::std::string::String);
        pub mod uri_with {
            pub fn deserialize<'de, D>(
                deserializer: D,
            ) -> ::core::result::Result<super::Uri, D::Error>
            where
                D: ::xmlity::Deserializer<'de>,
            {
                let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                    deserializer,
                )?;
                let value: ::std::string::String = text
                    .parse()
                    .map_err(::xmlity::de::Error::custom)?;
                super::Uri::try_from(value).map_err(::xmlity::de::Error::custom)
            }
            pub fn serialize<S>(
                value: &super::Uri,
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
        #[derive(::core::fmt::Debug, ::core::cmp::PartialEq, ::core::clone::Clone)]
        pub enum UriParseError {}
        impl ::core::convert::From<Uri> for ::std::string::String {
            fn from(value: Uri) -> Self {
                value.0
            }
        }
    }
    pub type Uri = uri_items::Uri;
    pub type Uris = ::xmlity_ns::List<String>;
}
