pub mod types {
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
    pub mod charsets_items {
        impl ::core::convert::From<::std::string::String> for Charsets {
            fn from(value: ::std::string::String) -> Self {
                Charsets(value)
            }
        }
        #[derive(
            ::core::fmt::Debug,
            ::xmlity::Serialize,
            ::xmlity::Deserialize,
            ::core::cmp::PartialEq,
            ::core::clone::Clone
        )]
        #[xvalue(with = charsets_with)]
        pub struct Charsets(pub ::std::string::String);
        pub mod charsets_with {
            pub fn deserialize<'de, D>(
                deserializer: D,
            ) -> ::core::result::Result<super::Charsets, D::Error>
            where
                D: ::xmlity::Deserializer<'de>,
            {
                let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                    deserializer,
                )?;
                let value: ::std::string::String = text
                    .parse()
                    .map_err(::xmlity::de::Error::custom)?;
                super::Charsets::try_from(value).map_err(::xmlity::de::Error::custom)
            }
            pub fn serialize<S>(
                value: &super::Charsets,
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
        pub enum CharsetsParseError {}
        impl ::core::convert::From<Charsets> for ::std::string::String {
            fn from(value: Charsets) -> Self {
                value.0
            }
        }
    }
    pub type Charsets = charsets_items::Charsets;
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
    pub mod coords_items {
        impl ::core::convert::From<::std::string::String> for Coords {
            fn from(value: ::std::string::String) -> Self {
                Coords(value)
            }
        }
        #[derive(
            ::core::fmt::Debug,
            ::xmlity::Serialize,
            ::xmlity::Deserialize,
            ::core::cmp::PartialEq,
            ::core::clone::Clone
        )]
        #[xvalue(with = coords_with)]
        pub struct Coords(pub ::std::string::String);
        pub mod coords_with {
            pub fn deserialize<'de, D>(
                deserializer: D,
            ) -> ::core::result::Result<super::Coords, D::Error>
            where
                D: ::xmlity::Deserializer<'de>,
            {
                let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                    deserializer,
                )?;
                let value: ::std::string::String = text
                    .parse()
                    .map_err(::xmlity::de::Error::custom)?;
                super::Coords::try_from(value).map_err(::xmlity::de::Error::custom)
            }
            pub fn serialize<S>(
                value: &super::Coords,
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
        pub enum CoordsParseError {}
        impl ::core::convert::From<Coords> for ::std::string::String {
            fn from(value: Coords) -> Self {
                value.0
            }
        }
    }
    pub type Coords = coords_items::Coords;
    pub type Datetime = ::std::string::String;
    pub mod input_type_items {
        #[derive(
            ::core::fmt::Debug,
            ::core::clone::Clone,
            ::core::marker::Copy,
            ::xmlity::Serialize,
            ::xmlity::Deserialize,
            ::core::cmp::PartialEq
        )]
        #[xvalue(with = input_type_with)]
        pub enum InputType {
            Text,
            Password,
            Checkbox,
            Radio,
            Submit,
            Reset,
            File,
            Hidden,
            Image,
            Button,
        }
        pub mod input_type_with {
            pub fn deserialize<'de, D>(
                deserializer: D,
            ) -> ::core::result::Result<super::InputType, D::Error>
            where
                D: ::xmlity::Deserializer<'de>,
            {
                let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                    deserializer,
                )?;
                let value: ::std::string::String = text
                    .parse()
                    .map_err(::xmlity::de::Error::custom)?;
                super::InputType::try_from(value).map_err(::xmlity::de::Error::custom)
            }
            pub fn serialize<S>(
                value: &super::InputType,
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
        pub enum InputTypeParseError {
            NonExistent { value: ::std::string::String },
        }
        impl ::core::fmt::Display for InputTypeParseError {
            fn fmt(
                &self,
                f: &mut ::core::fmt::Formatter<'_>,
            ) -> ::core::result::Result<(), ::core::fmt::Error> {
                match self {
                    InputTypeParseError::NonExistent { value } => {
                        write!(
                            f, "Value '{:?}' does not exist in the enumeration", value
                        )
                    }
                }
            }
        }
        impl ::core::convert::TryFrom<::std::string::String> for InputType {
            type Error = InputTypeParseError;
            fn try_from(
                value: ::std::string::String,
            ) -> ::core::result::Result<Self, Self::Error> {
                match ::std::string::String::as_str(&value) {
                    "text" => Ok(InputType::Text),
                    "password" => Ok(InputType::Password),
                    "checkbox" => Ok(InputType::Checkbox),
                    "radio" => Ok(InputType::Radio),
                    "submit" => Ok(InputType::Submit),
                    "reset" => Ok(InputType::Reset),
                    "file" => Ok(InputType::File),
                    "hidden" => Ok(InputType::Hidden),
                    "image" => Ok(InputType::Image),
                    "button" => Ok(InputType::Button),
                    _ => {
                        Err(InputTypeParseError::NonExistent {
                            value,
                        })
                    }
                }
            }
        }
        impl ::core::convert::From<InputType> for ::std::string::String {
            fn from(value: InputType) -> Self {
                match value {
                    InputType::Text => ::std::string::String::from("text"),
                    InputType::Password => ::std::string::String::from("password"),
                    InputType::Checkbox => ::std::string::String::from("checkbox"),
                    InputType::Radio => ::std::string::String::from("radio"),
                    InputType::Submit => ::std::string::String::from("submit"),
                    InputType::Reset => ::std::string::String::from("reset"),
                    InputType::File => ::std::string::String::from("file"),
                    InputType::Hidden => ::std::string::String::from("hidden"),
                    InputType::Image => ::std::string::String::from("image"),
                    InputType::Button => ::std::string::String::from("button"),
                }
            }
        }
    }
    pub type InputType = input_type_items::InputType;
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
        impl ::core::convert::From<::std::string::String> for Length {
            fn from(value: ::std::string::String) -> Self {
                Length(value)
            }
        }
        #[derive(
            ::core::fmt::Debug,
            ::xmlity::Serialize,
            ::xmlity::Deserialize,
            ::core::cmp::PartialEq,
            ::core::clone::Clone
        )]
        #[xvalue(with = length_with)]
        pub struct Length(pub ::std::string::String);
        pub mod length_with {
            pub fn deserialize<'de, D>(
                deserializer: D,
            ) -> ::core::result::Result<super::Length, D::Error>
            where
                D: ::xmlity::Deserializer<'de>,
            {
                let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                    deserializer,
                )?;
                let value: ::std::string::String = text
                    .parse()
                    .map_err(::xmlity::de::Error::custom)?;
                super::Length::try_from(value).map_err(::xmlity::de::Error::custom)
            }
            pub fn serialize<S>(
                value: &super::Length,
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
        pub enum LengthParseError {}
        impl ::core::convert::From<Length> for ::std::string::String {
            fn from(value: Length) -> Self {
                value.0
            }
        }
    }
    pub type Length = length_items::Length;
    pub mod link_types_items {
        impl ::core::convert::From<::std::string::String> for LinkTypes {
            fn from(value: ::std::string::String) -> Self {
                LinkTypes(value)
            }
        }
        #[derive(
            ::core::fmt::Debug,
            ::xmlity::Serialize,
            ::xmlity::Deserialize,
            ::core::cmp::PartialEq,
            ::core::clone::Clone
        )]
        #[xvalue(with = link_types_with)]
        pub struct LinkTypes(pub ::std::string::String);
        pub mod link_types_with {
            pub fn deserialize<'de, D>(
                deserializer: D,
            ) -> ::core::result::Result<super::LinkTypes, D::Error>
            where
                D: ::xmlity::Deserializer<'de>,
            {
                let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                    deserializer,
                )?;
                let value: ::std::string::String = text
                    .parse()
                    .map_err(::xmlity::de::Error::custom)?;
                super::LinkTypes::try_from(value).map_err(::xmlity::de::Error::custom)
            }
            pub fn serialize<S>(
                value: &super::LinkTypes,
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
        pub enum LinkTypesParseError {}
        impl ::core::convert::From<LinkTypes> for ::std::string::String {
            fn from(value: LinkTypes) -> Self {
                value.0
            }
        }
    }
    pub type LinkTypes = link_types_items::LinkTypes;
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
        impl ::core::convert::From<::std::string::String> for MultiLength {
            fn from(value: ::std::string::String) -> Self {
                MultiLength(value)
            }
        }
        #[derive(
            ::core::fmt::Debug,
            ::xmlity::Serialize,
            ::xmlity::Deserialize,
            ::core::cmp::PartialEq,
            ::core::clone::Clone
        )]
        #[xvalue(with = multi_length_with)]
        pub struct MultiLength(pub ::std::string::String);
        pub mod multi_length_with {
            pub fn deserialize<'de, D>(
                deserializer: D,
            ) -> ::core::result::Result<super::MultiLength, D::Error>
            where
                D: ::xmlity::Deserializer<'de>,
            {
                let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                    deserializer,
                )?;
                let value: ::std::string::String = text
                    .parse()
                    .map_err(::xmlity::de::Error::custom)?;
                super::MultiLength::try_from(value).map_err(::xmlity::de::Error::custom)
            }
            pub fn serialize<S>(
                value: &super::MultiLength,
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
        pub enum MultiLengthParseError {}
        impl ::core::convert::From<MultiLength> for ::std::string::String {
            fn from(value: MultiLength) -> Self {
                value.0
            }
        }
    }
    pub type MultiLength = multi_length_items::MultiLength;
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
    pub mod scope_items {
        #[derive(
            ::core::fmt::Debug,
            ::core::clone::Clone,
            ::core::marker::Copy,
            ::xmlity::Serialize,
            ::xmlity::Deserialize,
            ::core::cmp::PartialEq
        )]
        #[xvalue(with = scope_with)]
        pub enum Scope {
            Row,
            Col,
            Rowgroup,
            Colgroup,
        }
        pub mod scope_with {
            pub fn deserialize<'de, D>(
                deserializer: D,
            ) -> ::core::result::Result<super::Scope, D::Error>
            where
                D: ::xmlity::Deserializer<'de>,
            {
                let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                    deserializer,
                )?;
                let value: ::std::string::String = text
                    .parse()
                    .map_err(::xmlity::de::Error::custom)?;
                super::Scope::try_from(value).map_err(::xmlity::de::Error::custom)
            }
            pub fn serialize<S>(
                value: &super::Scope,
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
        pub enum ScopeParseError {
            NonExistent { value: ::std::string::String },
        }
        impl ::core::fmt::Display for ScopeParseError {
            fn fmt(
                &self,
                f: &mut ::core::fmt::Formatter<'_>,
            ) -> ::core::result::Result<(), ::core::fmt::Error> {
                match self {
                    ScopeParseError::NonExistent { value } => {
                        write!(
                            f, "Value '{:?}' does not exist in the enumeration", value
                        )
                    }
                }
            }
        }
        impl ::core::convert::TryFrom<::std::string::String> for Scope {
            type Error = ScopeParseError;
            fn try_from(
                value: ::std::string::String,
            ) -> ::core::result::Result<Self, Self::Error> {
                match ::std::string::String::as_str(&value) {
                    "row" => Ok(Scope::Row),
                    "col" => Ok(Scope::Col),
                    "rowgroup" => Ok(Scope::Rowgroup),
                    "colgroup" => Ok(Scope::Colgroup),
                    _ => {
                        Err(ScopeParseError::NonExistent {
                            value,
                        })
                    }
                }
            }
        }
        impl ::core::convert::From<Scope> for ::std::string::String {
            fn from(value: Scope) -> Self {
                match value {
                    Scope::Row => ::std::string::String::from("row"),
                    Scope::Col => ::std::string::String::from("col"),
                    Scope::Rowgroup => ::std::string::String::from("rowgroup"),
                    Scope::Colgroup => ::std::string::String::from("colgroup"),
                }
            }
        }
    }
    pub type Scope = scope_items::Scope;
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
    pub mod shape_items {
        #[derive(
            ::core::fmt::Debug,
            ::core::clone::Clone,
            ::core::marker::Copy,
            ::xmlity::Serialize,
            ::xmlity::Deserialize,
            ::core::cmp::PartialEq
        )]
        #[xvalue(with = shape_with)]
        pub enum Shape {
            Rect,
            Circle,
            Poly,
            Default,
        }
        pub mod shape_with {
            pub fn deserialize<'de, D>(
                deserializer: D,
            ) -> ::core::result::Result<super::Shape, D::Error>
            where
                D: ::xmlity::Deserializer<'de>,
            {
                let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                    deserializer,
                )?;
                let value: ::std::string::String = text
                    .parse()
                    .map_err(::xmlity::de::Error::custom)?;
                super::Shape::try_from(value).map_err(::xmlity::de::Error::custom)
            }
            pub fn serialize<S>(
                value: &super::Shape,
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
        pub enum ShapeParseError {
            NonExistent { value: ::std::string::String },
        }
        impl ::core::fmt::Display for ShapeParseError {
            fn fmt(
                &self,
                f: &mut ::core::fmt::Formatter<'_>,
            ) -> ::core::result::Result<(), ::core::fmt::Error> {
                match self {
                    ShapeParseError::NonExistent { value } => {
                        write!(
                            f, "Value '{:?}' does not exist in the enumeration", value
                        )
                    }
                }
            }
        }
        impl ::core::convert::TryFrom<::std::string::String> for Shape {
            type Error = ShapeParseError;
            fn try_from(
                value: ::std::string::String,
            ) -> ::core::result::Result<Self, Self::Error> {
                match ::std::string::String::as_str(&value) {
                    "rect" => Ok(Shape::Rect),
                    "circle" => Ok(Shape::Circle),
                    "poly" => Ok(Shape::Poly),
                    "default" => Ok(Shape::Default),
                    _ => {
                        Err(ShapeParseError::NonExistent {
                            value,
                        })
                    }
                }
            }
        }
        impl ::core::convert::From<Shape> for ::std::string::String {
            fn from(value: Shape) -> Self {
                match value {
                    Shape::Rect => ::std::string::String::from("rect"),
                    Shape::Circle => ::std::string::String::from("circle"),
                    Shape::Poly => ::std::string::String::from("poly"),
                    Shape::Default => ::std::string::String::from("default"),
                }
            }
        }
    }
    pub type Shape = shape_items::Shape;
    pub mod style_sheet_items {
        impl ::core::convert::From<::std::string::String> for StyleSheet {
            fn from(value: ::std::string::String) -> Self {
                StyleSheet(value)
            }
        }
        #[derive(
            ::core::fmt::Debug,
            ::xmlity::Serialize,
            ::xmlity::Deserialize,
            ::core::cmp::PartialEq,
            ::core::clone::Clone
        )]
        #[xvalue(with = style_sheet_with)]
        pub struct StyleSheet(pub ::std::string::String);
        pub mod style_sheet_with {
            pub fn deserialize<'de, D>(
                deserializer: D,
            ) -> ::core::result::Result<super::StyleSheet, D::Error>
            where
                D: ::xmlity::Deserializer<'de>,
            {
                let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                    deserializer,
                )?;
                let value: ::std::string::String = text
                    .parse()
                    .map_err(::xmlity::de::Error::custom)?;
                super::StyleSheet::try_from(value).map_err(::xmlity::de::Error::custom)
            }
            pub fn serialize<S>(
                value: &super::StyleSheet,
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
        pub enum StyleSheetParseError {}
        impl ::core::convert::From<StyleSheet> for ::std::string::String {
            fn from(value: StyleSheet) -> Self {
                value.0
            }
        }
    }
    pub type StyleSheet = style_sheet_items::StyleSheet;
    pub mod tframe_items {
        #[derive(
            ::core::fmt::Debug,
            ::core::clone::Clone,
            ::core::marker::Copy,
            ::xmlity::Serialize,
            ::xmlity::Deserialize,
            ::core::cmp::PartialEq
        )]
        #[xvalue(with = tframe_with)]
        pub enum Tframe {
            Void,
            Above,
            Below,
            Hsides,
            Lhs,
            Rhs,
            Vsides,
            Box,
            Border,
        }
        pub mod tframe_with {
            pub fn deserialize<'de, D>(
                deserializer: D,
            ) -> ::core::result::Result<super::Tframe, D::Error>
            where
                D: ::xmlity::Deserializer<'de>,
            {
                let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                    deserializer,
                )?;
                let value: ::std::string::String = text
                    .parse()
                    .map_err(::xmlity::de::Error::custom)?;
                super::Tframe::try_from(value).map_err(::xmlity::de::Error::custom)
            }
            pub fn serialize<S>(
                value: &super::Tframe,
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
        pub enum TframeParseError {
            NonExistent { value: ::std::string::String },
        }
        impl ::core::fmt::Display for TframeParseError {
            fn fmt(
                &self,
                f: &mut ::core::fmt::Formatter<'_>,
            ) -> ::core::result::Result<(), ::core::fmt::Error> {
                match self {
                    TframeParseError::NonExistent { value } => {
                        write!(
                            f, "Value '{:?}' does not exist in the enumeration", value
                        )
                    }
                }
            }
        }
        impl ::core::convert::TryFrom<::std::string::String> for Tframe {
            type Error = TframeParseError;
            fn try_from(
                value: ::std::string::String,
            ) -> ::core::result::Result<Self, Self::Error> {
                match ::std::string::String::as_str(&value) {
                    "void" => Ok(Tframe::Void),
                    "above" => Ok(Tframe::Above),
                    "below" => Ok(Tframe::Below),
                    "hsides" => Ok(Tframe::Hsides),
                    "lhs" => Ok(Tframe::Lhs),
                    "rhs" => Ok(Tframe::Rhs),
                    "vsides" => Ok(Tframe::Vsides),
                    "box" => Ok(Tframe::Box),
                    "border" => Ok(Tframe::Border),
                    _ => {
                        Err(TframeParseError::NonExistent {
                            value,
                        })
                    }
                }
            }
        }
        impl ::core::convert::From<Tframe> for ::std::string::String {
            fn from(value: Tframe) -> Self {
                match value {
                    Tframe::Void => ::std::string::String::from("void"),
                    Tframe::Above => ::std::string::String::from("above"),
                    Tframe::Below => ::std::string::String::from("below"),
                    Tframe::Hsides => ::std::string::String::from("hsides"),
                    Tframe::Lhs => ::std::string::String::from("lhs"),
                    Tframe::Rhs => ::std::string::String::from("rhs"),
                    Tframe::Vsides => ::std::string::String::from("vsides"),
                    Tframe::Box => ::std::string::String::from("box"),
                    Tframe::Border => ::std::string::String::from("border"),
                }
            }
        }
    }
    pub type Tframe = tframe_items::Tframe;
    pub mod trules_items {
        #[derive(
            ::core::fmt::Debug,
            ::core::clone::Clone,
            ::core::marker::Copy,
            ::xmlity::Serialize,
            ::xmlity::Deserialize,
            ::core::cmp::PartialEq
        )]
        #[xvalue(with = trules_with)]
        pub enum Trules {
            None,
            Groups,
            Rows,
            Cols,
            All,
        }
        pub mod trules_with {
            pub fn deserialize<'de, D>(
                deserializer: D,
            ) -> ::core::result::Result<super::Trules, D::Error>
            where
                D: ::xmlity::Deserializer<'de>,
            {
                let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                    deserializer,
                )?;
                let value: ::std::string::String = text
                    .parse()
                    .map_err(::xmlity::de::Error::custom)?;
                super::Trules::try_from(value).map_err(::xmlity::de::Error::custom)
            }
            pub fn serialize<S>(
                value: &super::Trules,
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
        pub enum TrulesParseError {
            NonExistent { value: ::std::string::String },
        }
        impl ::core::fmt::Display for TrulesParseError {
            fn fmt(
                &self,
                f: &mut ::core::fmt::Formatter<'_>,
            ) -> ::core::result::Result<(), ::core::fmt::Error> {
                match self {
                    TrulesParseError::NonExistent { value } => {
                        write!(
                            f, "Value '{:?}' does not exist in the enumeration", value
                        )
                    }
                }
            }
        }
        impl ::core::convert::TryFrom<::std::string::String> for Trules {
            type Error = TrulesParseError;
            fn try_from(
                value: ::std::string::String,
            ) -> ::core::result::Result<Self, Self::Error> {
                match ::std::string::String::as_str(&value) {
                    "none" => Ok(Trules::None),
                    "groups" => Ok(Trules::Groups),
                    "rows" => Ok(Trules::Rows),
                    "cols" => Ok(Trules::Cols),
                    "all" => Ok(Trules::All),
                    _ => {
                        Err(TrulesParseError::NonExistent {
                            value,
                        })
                    }
                }
            }
        }
        impl ::core::convert::From<Trules> for ::std::string::String {
            fn from(value: Trules) -> Self {
                match value {
                    Trules::None => ::std::string::String::from("none"),
                    Trules::Groups => ::std::string::String::from("groups"),
                    Trules::Rows => ::std::string::String::from("rows"),
                    Trules::Cols => ::std::string::String::from("cols"),
                    Trules::All => ::std::string::String::from("all"),
                }
            }
        }
    }
    pub type Trules = trules_items::Trules;
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
    pub type Uri = ::std::string::String;
    pub mod uri_list_items {
        impl ::core::convert::From<::std::string::String> for UriList {
            fn from(value: ::std::string::String) -> Self {
                UriList(value)
            }
        }
        #[derive(
            ::core::fmt::Debug,
            ::xmlity::Serialize,
            ::xmlity::Deserialize,
            ::core::cmp::PartialEq,
            ::core::clone::Clone
        )]
        #[xvalue(with = uri_list_with)]
        pub struct UriList(pub ::std::string::String);
        pub mod uri_list_with {
            pub fn deserialize<'de, D>(
                deserializer: D,
            ) -> ::core::result::Result<super::UriList, D::Error>
            where
                D: ::xmlity::Deserializer<'de>,
            {
                let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                    deserializer,
                )?;
                let value: ::std::string::String = text
                    .parse()
                    .map_err(::xmlity::de::Error::custom)?;
                super::UriList::try_from(value).map_err(::xmlity::de::Error::custom)
            }
            pub fn serialize<S>(
                value: &super::UriList,
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
        pub enum UriListParseError {}
        impl ::core::convert::From<UriList> for ::std::string::String {
            fn from(value: UriList) -> Self {
                value.0
            }
        }
    }
    pub type UriList = uri_list_items::UriList;
    pub type TabindexNumber = ::std::string::String;
    pub mod block_items {
        impl ::core::convert::From<crate::xhtml::groups::Block> for Block {
            fn from(value: crate::xhtml::groups::Block) -> Self {
                Block::Block(::std::boxed::Box::new(value))
            }
        }
        impl ::core::convert::From<crate::xhtml::Form> for Block {
            fn from(value: crate::xhtml::Form) -> Self {
                Block::Form(::std::boxed::Box::new(value))
            }
        }
        impl ::core::convert::From<crate::xhtml::groups::Misc> for Block {
            fn from(value: crate::xhtml::groups::Misc) -> Self {
                Block::Misc(::std::boxed::Box::new(value))
            }
        }
        #[derive(
            ::core::fmt::Debug,
            ::xmlity::Serialize,
            ::xmlity::Deserialize,
            ::core::cmp::PartialEq,
            ::core::clone::Clone
        )]
        pub enum Block {
            Block(::std::boxed::Box<crate::xhtml::groups::Block>),
            Form(::std::boxed::Box<crate::xhtml::Form>),
            Misc(::std::boxed::Box<crate::xhtml::groups::Misc>),
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub struct Block {
        #[xvalue(default)]
        pub block: ::std::vec::Vec<block_items::Block>,
    }
    pub mod flow_items {
        impl ::core::convert::From<crate::xhtml::groups::Block> for Flow {
            fn from(value: crate::xhtml::groups::Block) -> Self {
                Flow::Block(::std::boxed::Box::new(value))
            }
        }
        impl ::core::convert::From<crate::xhtml::Form> for Flow {
            fn from(value: crate::xhtml::Form) -> Self {
                Flow::Form(::std::boxed::Box::new(value))
            }
        }
        impl ::core::convert::From<crate::xhtml::groups::Inline> for Flow {
            fn from(value: crate::xhtml::groups::Inline) -> Self {
                Flow::Inline(::std::boxed::Box::new(value))
            }
        }
        impl ::core::convert::From<crate::xhtml::groups::Misc> for Flow {
            fn from(value: crate::xhtml::groups::Misc) -> Self {
                Flow::Misc(::std::boxed::Box::new(value))
            }
        }
        #[derive(
            ::core::fmt::Debug,
            ::xmlity::Serialize,
            ::xmlity::Deserialize,
            ::core::cmp::PartialEq,
            ::core::clone::Clone
        )]
        pub enum Flow {
            Block(::std::boxed::Box<crate::xhtml::groups::Block>),
            Form(::std::boxed::Box<crate::xhtml::Form>),
            Inline(::std::boxed::Box<crate::xhtml::groups::Inline>),
            Misc(::std::boxed::Box<crate::xhtml::groups::Misc>),
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub struct Flow {
        #[xvalue(default)]
        pub flow: ::std::vec::Vec<flow_items::Flow>,
    }
    pub mod inline_items {
        impl ::core::convert::From<crate::xhtml::groups::Inline> for Inline {
            fn from(value: crate::xhtml::groups::Inline) -> Self {
                Inline::Inline(::std::boxed::Box::new(value))
            }
        }
        impl ::core::convert::From<crate::xhtml::groups::MiscInline> for Inline {
            fn from(value: crate::xhtml::groups::MiscInline) -> Self {
                Inline::MiscInline(::std::boxed::Box::new(value))
            }
        }
        #[derive(
            ::core::fmt::Debug,
            ::xmlity::Serialize,
            ::xmlity::Deserialize,
            ::core::cmp::PartialEq,
            ::core::clone::Clone
        )]
        pub enum Inline {
            Inline(::std::boxed::Box<crate::xhtml::groups::Inline>),
            MiscInline(::std::boxed::Box<crate::xhtml::groups::MiscInline>),
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub struct Inline {
        #[xvalue(default)]
        pub inline: ::std::vec::Vec<inline_items::Inline>,
    }
    pub mod a_content_items {
        impl ::core::convert::From<crate::xhtml::groups::Special> for Acontent {
            fn from(value: crate::xhtml::groups::Special) -> Self {
                Acontent::Special(::std::boxed::Box::new(value))
            }
        }
        impl ::core::convert::From<crate::xhtml::groups::Fontstyle> for Acontent {
            fn from(value: crate::xhtml::groups::Fontstyle) -> Self {
                Acontent::Fontstyle(::std::boxed::Box::new(value))
            }
        }
        impl ::core::convert::From<crate::xhtml::groups::Phrase> for Acontent {
            fn from(value: crate::xhtml::groups::Phrase) -> Self {
                Acontent::Phrase(::std::boxed::Box::new(value))
            }
        }
        impl ::core::convert::From<crate::xhtml::groups::InlineForms> for Acontent {
            fn from(value: crate::xhtml::groups::InlineForms) -> Self {
                Acontent::InlineForms(::std::boxed::Box::new(value))
            }
        }
        impl ::core::convert::From<crate::xhtml::groups::MiscInline> for Acontent {
            fn from(value: crate::xhtml::groups::MiscInline) -> Self {
                Acontent::MiscInline(::std::boxed::Box::new(value))
            }
        }
        #[derive(
            ::core::fmt::Debug,
            ::xmlity::Serialize,
            ::xmlity::Deserialize,
            ::core::cmp::PartialEq,
            ::core::clone::Clone
        )]
        pub enum Acontent {
            Special(::std::boxed::Box<crate::xhtml::groups::Special>),
            Fontstyle(::std::boxed::Box<crate::xhtml::groups::Fontstyle>),
            Phrase(::std::boxed::Box<crate::xhtml::groups::Phrase>),
            InlineForms(::std::boxed::Box<crate::xhtml::groups::InlineForms>),
            MiscInline(::std::boxed::Box<crate::xhtml::groups::MiscInline>),
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub struct AContent {
        #[xvalue(default)]
        pub acontent: ::std::vec::Vec<a_content_items::Acontent>,
    }
    pub mod button_content_items {
        impl ::core::convert::From<crate::xhtml::P> for ButtonContent {
            fn from(value: crate::xhtml::P) -> Self {
                ButtonContent::P(::std::boxed::Box::new(value))
            }
        }
        impl ::core::convert::From<crate::xhtml::groups::Heading> for ButtonContent {
            fn from(value: crate::xhtml::groups::Heading) -> Self {
                ButtonContent::Heading(::std::boxed::Box::new(value))
            }
        }
        impl ::core::convert::From<crate::xhtml::Div> for ButtonContent {
            fn from(value: crate::xhtml::Div) -> Self {
                ButtonContent::Div(::std::boxed::Box::new(value))
            }
        }
        impl ::core::convert::From<crate::xhtml::groups::Lists> for ButtonContent {
            fn from(value: crate::xhtml::groups::Lists) -> Self {
                ButtonContent::Lists(::std::boxed::Box::new(value))
            }
        }
        impl ::core::convert::From<crate::xhtml::groups::Blocktext> for ButtonContent {
            fn from(value: crate::xhtml::groups::Blocktext) -> Self {
                ButtonContent::Blocktext(::std::boxed::Box::new(value))
            }
        }
        impl ::core::convert::From<crate::xhtml::Table> for ButtonContent {
            fn from(value: crate::xhtml::Table) -> Self {
                ButtonContent::Table(::std::boxed::Box::new(value))
            }
        }
        impl ::core::convert::From<crate::xhtml::groups::Special> for ButtonContent {
            fn from(value: crate::xhtml::groups::Special) -> Self {
                ButtonContent::Special(::std::boxed::Box::new(value))
            }
        }
        impl ::core::convert::From<crate::xhtml::groups::Fontstyle> for ButtonContent {
            fn from(value: crate::xhtml::groups::Fontstyle) -> Self {
                ButtonContent::Fontstyle(::std::boxed::Box::new(value))
            }
        }
        impl ::core::convert::From<crate::xhtml::groups::Phrase> for ButtonContent {
            fn from(value: crate::xhtml::groups::Phrase) -> Self {
                ButtonContent::Phrase(::std::boxed::Box::new(value))
            }
        }
        impl ::core::convert::From<crate::xhtml::groups::Misc> for ButtonContent {
            fn from(value: crate::xhtml::groups::Misc) -> Self {
                ButtonContent::Misc(::std::boxed::Box::new(value))
            }
        }
        #[derive(
            ::core::fmt::Debug,
            ::xmlity::Serialize,
            ::xmlity::Deserialize,
            ::core::cmp::PartialEq,
            ::core::clone::Clone
        )]
        pub enum ButtonContent {
            P(::std::boxed::Box<crate::xhtml::P>),
            Heading(::std::boxed::Box<crate::xhtml::groups::Heading>),
            Div(::std::boxed::Box<crate::xhtml::Div>),
            Lists(::std::boxed::Box<crate::xhtml::groups::Lists>),
            Blocktext(::std::boxed::Box<crate::xhtml::groups::Blocktext>),
            Table(::std::boxed::Box<crate::xhtml::Table>),
            Special(::std::boxed::Box<crate::xhtml::groups::Special>),
            Fontstyle(::std::boxed::Box<crate::xhtml::groups::Fontstyle>),
            Phrase(::std::boxed::Box<crate::xhtml::groups::Phrase>),
            Misc(::std::boxed::Box<crate::xhtml::groups::Misc>),
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub struct ButtonContent {
        #[xvalue(default)]
        pub button_content: ::std::vec::Vec<button_content_items::ButtonContent>,
    }
    pub mod form_content_items {
        impl ::core::convert::From<crate::xhtml::groups::Block> for FormContent {
            fn from(value: crate::xhtml::groups::Block) -> Self {
                FormContent::Block(::std::boxed::Box::new(value))
            }
        }
        impl ::core::convert::From<crate::xhtml::groups::Misc> for FormContent {
            fn from(value: crate::xhtml::groups::Misc) -> Self {
                FormContent::Misc(::std::boxed::Box::new(value))
            }
        }
        #[derive(
            ::core::fmt::Debug,
            ::xmlity::Serialize,
            ::xmlity::Deserialize,
            ::core::cmp::PartialEq,
            ::core::clone::Clone
        )]
        pub enum FormContent {
            Block(::std::boxed::Box<crate::xhtml::groups::Block>),
            Misc(::std::boxed::Box<crate::xhtml::groups::Misc>),
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub struct FormContent {
        #[xvalue(default)]
        pub form_content: ::std::vec::Vec<form_content_items::FormContent>,
    }
    pub mod pre_content_items {
        impl ::core::convert::From<crate::xhtml::A> for PreContent {
            fn from(value: crate::xhtml::A) -> Self {
                PreContent::A(::std::boxed::Box::new(value))
            }
        }
        impl ::core::convert::From<crate::xhtml::groups::Fontstyle> for PreContent {
            fn from(value: crate::xhtml::groups::Fontstyle) -> Self {
                PreContent::Fontstyle(::std::boxed::Box::new(value))
            }
        }
        impl ::core::convert::From<crate::xhtml::groups::Phrase> for PreContent {
            fn from(value: crate::xhtml::groups::Phrase) -> Self {
                PreContent::Phrase(::std::boxed::Box::new(value))
            }
        }
        impl ::core::convert::From<crate::xhtml::groups::SpecialPre> for PreContent {
            fn from(value: crate::xhtml::groups::SpecialPre) -> Self {
                PreContent::SpecialPre(::std::boxed::Box::new(value))
            }
        }
        impl ::core::convert::From<crate::xhtml::groups::MiscInline> for PreContent {
            fn from(value: crate::xhtml::groups::MiscInline) -> Self {
                PreContent::MiscInline(::std::boxed::Box::new(value))
            }
        }
        impl ::core::convert::From<crate::xhtml::groups::InlineForms> for PreContent {
            fn from(value: crate::xhtml::groups::InlineForms) -> Self {
                PreContent::InlineForms(::std::boxed::Box::new(value))
            }
        }
        #[derive(
            ::core::fmt::Debug,
            ::xmlity::Serialize,
            ::xmlity::Deserialize,
            ::core::cmp::PartialEq,
            ::core::clone::Clone
        )]
        pub enum PreContent {
            A(::std::boxed::Box<crate::xhtml::A>),
            Fontstyle(::std::boxed::Box<crate::xhtml::groups::Fontstyle>),
            Phrase(::std::boxed::Box<crate::xhtml::groups::Phrase>),
            SpecialPre(::std::boxed::Box<crate::xhtml::groups::SpecialPre>),
            MiscInline(::std::boxed::Box<crate::xhtml::groups::MiscInline>),
            InlineForms(::std::boxed::Box<crate::xhtml::groups::InlineForms>),
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub struct PreContent {
        #[xvalue(default)]
        pub pre_content: ::std::vec::Vec<pre_content_items::PreContent>,
    }
}
pub mod attributes {}
pub mod groups {
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub enum Block {
        P(::std::boxed::Box<crate::xhtml::P>),
        Heading(::std::boxed::Box<crate::xhtml::groups::Heading>),
        Div(::std::boxed::Box<crate::xhtml::Div>),
        Lists(::std::boxed::Box<crate::xhtml::groups::Lists>),
        Blocktext(::std::boxed::Box<crate::xhtml::groups::Blocktext>),
        Fieldset(::std::boxed::Box<crate::xhtml::Fieldset>),
        Table(::std::boxed::Box<crate::xhtml::Table>),
    }
    impl ::core::convert::From<crate::xhtml::P> for Block {
        fn from(value: crate::xhtml::P) -> Self {
            Block::P(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml::groups::Heading> for Block {
        fn from(value: crate::xhtml::groups::Heading) -> Self {
            Block::Heading(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml::Div> for Block {
        fn from(value: crate::xhtml::Div) -> Self {
            Block::Div(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml::groups::Lists> for Block {
        fn from(value: crate::xhtml::groups::Lists) -> Self {
            Block::Lists(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml::groups::Blocktext> for Block {
        fn from(value: crate::xhtml::groups::Blocktext) -> Self {
            Block::Blocktext(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml::Fieldset> for Block {
        fn from(value: crate::xhtml::Fieldset) -> Self {
            Block::Fieldset(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml::Table> for Block {
        fn from(value: crate::xhtml::Table) -> Self {
            Block::Table(::std::boxed::Box::new(value))
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub enum Blocktext {
        Pre(::std::boxed::Box<crate::xhtml::Pre>),
        Hr(::std::boxed::Box<crate::xhtml::Hr>),
        Blockquote(::std::boxed::Box<crate::xhtml::Blockquote>),
        Address(::std::boxed::Box<crate::xhtml::Address>),
    }
    impl ::core::convert::From<crate::xhtml::Pre> for Blocktext {
        fn from(value: crate::xhtml::Pre) -> Self {
            Blocktext::Pre(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml::Hr> for Blocktext {
        fn from(value: crate::xhtml::Hr) -> Self {
            Blocktext::Hr(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml::Blockquote> for Blocktext {
        fn from(value: crate::xhtml::Blockquote) -> Self {
            Blocktext::Blockquote(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml::Address> for Blocktext {
        fn from(value: crate::xhtml::Address) -> Self {
            Blocktext::Address(::std::boxed::Box::new(value))
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub enum Fontstyle {
        Tt(::std::boxed::Box<crate::xhtml::Tt>),
        I(::std::boxed::Box<crate::xhtml::I>),
        B(::std::boxed::Box<crate::xhtml::B>),
        Big(::std::boxed::Box<crate::xhtml::Big>),
        Small(::std::boxed::Box<crate::xhtml::Small>),
    }
    impl ::core::convert::From<crate::xhtml::Tt> for Fontstyle {
        fn from(value: crate::xhtml::Tt) -> Self {
            Fontstyle::Tt(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml::I> for Fontstyle {
        fn from(value: crate::xhtml::I) -> Self {
            Fontstyle::I(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml::B> for Fontstyle {
        fn from(value: crate::xhtml::B) -> Self {
            Fontstyle::B(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml::Big> for Fontstyle {
        fn from(value: crate::xhtml::Big) -> Self {
            Fontstyle::Big(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml::Small> for Fontstyle {
        fn from(value: crate::xhtml::Small) -> Self {
            Fontstyle::Small(::std::boxed::Box::new(value))
        }
    }
    pub mod head_misc_items {
        impl ::core::convert::From<crate::xhtml::Script> for Child0 {
            fn from(value: crate::xhtml::Script) -> Self {
                Child0::Script(::std::boxed::Box::new(value))
            }
        }
        impl ::core::convert::From<crate::xhtml::Style> for Child0 {
            fn from(value: crate::xhtml::Style) -> Self {
                Child0::Style(::std::boxed::Box::new(value))
            }
        }
        impl ::core::convert::From<crate::xhtml::Meta> for Child0 {
            fn from(value: crate::xhtml::Meta) -> Self {
                Child0::Meta(::std::boxed::Box::new(value))
            }
        }
        impl ::core::convert::From<crate::xhtml::Link> for Child0 {
            fn from(value: crate::xhtml::Link) -> Self {
                Child0::Link(::std::boxed::Box::new(value))
            }
        }
        impl ::core::convert::From<crate::xhtml::Object> for Child0 {
            fn from(value: crate::xhtml::Object) -> Self {
                Child0::Object(::std::boxed::Box::new(value))
            }
        }
        #[derive(
            ::core::fmt::Debug,
            ::xmlity::Serialize,
            ::xmlity::Deserialize,
            ::core::cmp::PartialEq,
            ::core::clone::Clone
        )]
        pub enum Child0 {
            Script(::std::boxed::Box<crate::xhtml::Script>),
            Style(::std::boxed::Box<crate::xhtml::Style>),
            Meta(::std::boxed::Box<crate::xhtml::Meta>),
            Link(::std::boxed::Box<crate::xhtml::Link>),
            Object(::std::boxed::Box<crate::xhtml::Object>),
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xvalue(order = "strict")]
    pub struct HeadMisc {
        #[xvalue(default)]
        pub child_0: ::std::vec::Vec<head_misc_items::Child0>,
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub enum Heading {
        H1(::std::boxed::Box<crate::xhtml::H1>),
        H2(::std::boxed::Box<crate::xhtml::H2>),
        H3(::std::boxed::Box<crate::xhtml::H3>),
        H4(::std::boxed::Box<crate::xhtml::H4>),
        H5(::std::boxed::Box<crate::xhtml::H5>),
        H6(::std::boxed::Box<crate::xhtml::H6>),
    }
    impl ::core::convert::From<crate::xhtml::H1> for Heading {
        fn from(value: crate::xhtml::H1) -> Self {
            Heading::H1(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml::H2> for Heading {
        fn from(value: crate::xhtml::H2) -> Self {
            Heading::H2(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml::H3> for Heading {
        fn from(value: crate::xhtml::H3) -> Self {
            Heading::H3(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml::H4> for Heading {
        fn from(value: crate::xhtml::H4) -> Self {
            Heading::H4(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml::H5> for Heading {
        fn from(value: crate::xhtml::H5) -> Self {
            Heading::H5(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml::H6> for Heading {
        fn from(value: crate::xhtml::H6) -> Self {
            Heading::H6(::std::boxed::Box::new(value))
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub enum Inline {
        A(::std::boxed::Box<crate::xhtml::A>),
        Special(::std::boxed::Box<crate::xhtml::groups::Special>),
        Fontstyle(::std::boxed::Box<crate::xhtml::groups::Fontstyle>),
        Phrase(::std::boxed::Box<crate::xhtml::groups::Phrase>),
        InlineForms(::std::boxed::Box<crate::xhtml::groups::InlineForms>),
    }
    impl ::core::convert::From<crate::xhtml::A> for Inline {
        fn from(value: crate::xhtml::A) -> Self {
            Inline::A(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml::groups::Special> for Inline {
        fn from(value: crate::xhtml::groups::Special) -> Self {
            Inline::Special(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml::groups::Fontstyle> for Inline {
        fn from(value: crate::xhtml::groups::Fontstyle) -> Self {
            Inline::Fontstyle(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml::groups::Phrase> for Inline {
        fn from(value: crate::xhtml::groups::Phrase) -> Self {
            Inline::Phrase(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml::groups::InlineForms> for Inline {
        fn from(value: crate::xhtml::groups::InlineForms) -> Self {
            Inline::InlineForms(::std::boxed::Box::new(value))
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub enum InlineForms {
        Input(::std::boxed::Box<crate::xhtml::Input>),
        Select(::std::boxed::Box<crate::xhtml::Select>),
        Textarea(::std::boxed::Box<crate::xhtml::Textarea>),
        Label(::std::boxed::Box<crate::xhtml::Label>),
        Button(::std::boxed::Box<crate::xhtml::Button>),
    }
    impl ::core::convert::From<crate::xhtml::Input> for InlineForms {
        fn from(value: crate::xhtml::Input) -> Self {
            InlineForms::Input(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml::Select> for InlineForms {
        fn from(value: crate::xhtml::Select) -> Self {
            InlineForms::Select(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml::Textarea> for InlineForms {
        fn from(value: crate::xhtml::Textarea) -> Self {
            InlineForms::Textarea(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml::Label> for InlineForms {
        fn from(value: crate::xhtml::Label) -> Self {
            InlineForms::Label(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml::Button> for InlineForms {
        fn from(value: crate::xhtml::Button) -> Self {
            InlineForms::Button(::std::boxed::Box::new(value))
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub enum Lists {
        Ul(::std::boxed::Box<crate::xhtml::Ul>),
        Ol(::std::boxed::Box<crate::xhtml::Ol>),
        Dl(::std::boxed::Box<crate::xhtml::Dl>),
    }
    impl ::core::convert::From<crate::xhtml::Ul> for Lists {
        fn from(value: crate::xhtml::Ul) -> Self {
            Lists::Ul(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml::Ol> for Lists {
        fn from(value: crate::xhtml::Ol) -> Self {
            Lists::Ol(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml::Dl> for Lists {
        fn from(value: crate::xhtml::Dl) -> Self {
            Lists::Dl(::std::boxed::Box::new(value))
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub enum Misc {
        Noscript(::std::boxed::Box<crate::xhtml::Noscript>),
        MiscInline(::std::boxed::Box<crate::xhtml::groups::MiscInline>),
    }
    impl ::core::convert::From<crate::xhtml::Noscript> for Misc {
        fn from(value: crate::xhtml::Noscript) -> Self {
            Misc::Noscript(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml::groups::MiscInline> for Misc {
        fn from(value: crate::xhtml::groups::MiscInline) -> Self {
            Misc::MiscInline(::std::boxed::Box::new(value))
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub enum MiscInline {
        Ins(::std::boxed::Box<crate::xhtml::Ins>),
        Del(::std::boxed::Box<crate::xhtml::Del>),
        Script(::std::boxed::Box<crate::xhtml::Script>),
    }
    impl ::core::convert::From<crate::xhtml::Ins> for MiscInline {
        fn from(value: crate::xhtml::Ins) -> Self {
            MiscInline::Ins(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml::Del> for MiscInline {
        fn from(value: crate::xhtml::Del) -> Self {
            MiscInline::Del(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml::Script> for MiscInline {
        fn from(value: crate::xhtml::Script) -> Self {
            MiscInline::Script(::std::boxed::Box::new(value))
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub enum Phrase {
        Em(::std::boxed::Box<crate::xhtml::Em>),
        Strong(::std::boxed::Box<crate::xhtml::Strong>),
        Dfn(::std::boxed::Box<crate::xhtml::Dfn>),
        Code(::std::boxed::Box<crate::xhtml::Code>),
        Q(::std::boxed::Box<crate::xhtml::Q>),
        Samp(::std::boxed::Box<crate::xhtml::Samp>),
        Kbd(::std::boxed::Box<crate::xhtml::Kbd>),
        Var(::std::boxed::Box<crate::xhtml::Var>),
        Cite(::std::boxed::Box<crate::xhtml::Cite>),
        Abbr(::std::boxed::Box<crate::xhtml::Abbr>),
        Acronym(::std::boxed::Box<crate::xhtml::Acronym>),
        Sub(::std::boxed::Box<crate::xhtml::Sub>),
        Sup(::std::boxed::Box<crate::xhtml::Sup>),
    }
    impl ::core::convert::From<crate::xhtml::Em> for Phrase {
        fn from(value: crate::xhtml::Em) -> Self {
            Phrase::Em(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml::Strong> for Phrase {
        fn from(value: crate::xhtml::Strong) -> Self {
            Phrase::Strong(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml::Dfn> for Phrase {
        fn from(value: crate::xhtml::Dfn) -> Self {
            Phrase::Dfn(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml::Code> for Phrase {
        fn from(value: crate::xhtml::Code) -> Self {
            Phrase::Code(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml::Q> for Phrase {
        fn from(value: crate::xhtml::Q) -> Self {
            Phrase::Q(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml::Samp> for Phrase {
        fn from(value: crate::xhtml::Samp) -> Self {
            Phrase::Samp(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml::Kbd> for Phrase {
        fn from(value: crate::xhtml::Kbd) -> Self {
            Phrase::Kbd(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml::Var> for Phrase {
        fn from(value: crate::xhtml::Var) -> Self {
            Phrase::Var(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml::Cite> for Phrase {
        fn from(value: crate::xhtml::Cite) -> Self {
            Phrase::Cite(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml::Abbr> for Phrase {
        fn from(value: crate::xhtml::Abbr) -> Self {
            Phrase::Abbr(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml::Acronym> for Phrase {
        fn from(value: crate::xhtml::Acronym) -> Self {
            Phrase::Acronym(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml::Sub> for Phrase {
        fn from(value: crate::xhtml::Sub) -> Self {
            Phrase::Sub(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml::Sup> for Phrase {
        fn from(value: crate::xhtml::Sup) -> Self {
            Phrase::Sup(::std::boxed::Box::new(value))
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub enum Special {
        SpecialPre(::std::boxed::Box<crate::xhtml::groups::SpecialPre>),
        Object(::std::boxed::Box<crate::xhtml::Object>),
        Img(::std::boxed::Box<crate::xhtml::Img>),
    }
    impl ::core::convert::From<crate::xhtml::groups::SpecialPre> for Special {
        fn from(value: crate::xhtml::groups::SpecialPre) -> Self {
            Special::SpecialPre(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml::Object> for Special {
        fn from(value: crate::xhtml::Object) -> Self {
            Special::Object(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml::Img> for Special {
        fn from(value: crate::xhtml::Img) -> Self {
            Special::Img(::std::boxed::Box::new(value))
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub enum SpecialPre {
        Br(::std::boxed::Box<crate::xhtml::Br>),
        Span(::std::boxed::Box<crate::xhtml::Span>),
        Bdo(::std::boxed::Box<crate::xhtml::Bdo>),
        Map(::std::boxed::Box<crate::xhtml::Map>),
    }
    impl ::core::convert::From<crate::xhtml::Br> for SpecialPre {
        fn from(value: crate::xhtml::Br) -> Self {
            SpecialPre::Br(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml::Span> for SpecialPre {
        fn from(value: crate::xhtml::Span) -> Self {
            SpecialPre::Span(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml::Bdo> for SpecialPre {
        fn from(value: crate::xhtml::Bdo) -> Self {
            SpecialPre::Bdo(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml::Map> for SpecialPre {
        fn from(value: crate::xhtml::Map) -> Self {
            SpecialPre::Map(::std::boxed::Box::new(value))
        }
    }
}
pub mod a_items {
    impl ::core::convert::From<crate::xhtml::groups::Special> for A {
        fn from(value: crate::xhtml::groups::Special) -> Self {
            A::Special(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml::groups::Fontstyle> for A {
        fn from(value: crate::xhtml::groups::Fontstyle) -> Self {
            A::Fontstyle(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml::groups::Phrase> for A {
        fn from(value: crate::xhtml::groups::Phrase) -> Self {
            A::Phrase(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml::groups::InlineForms> for A {
        fn from(value: crate::xhtml::groups::InlineForms) -> Self {
            A::InlineForms(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml::groups::MiscInline> for A {
        fn from(value: crate::xhtml::groups::MiscInline) -> Self {
            A::MiscInline(::std::boxed::Box::new(value))
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub enum A {
        Special(::std::boxed::Box<crate::xhtml::groups::Special>),
        Fontstyle(::std::boxed::Box<crate::xhtml::groups::Fontstyle>),
        Phrase(::std::boxed::Box<crate::xhtml::groups::Phrase>),
        InlineForms(::std::boxed::Box<crate::xhtml::groups::InlineForms>),
        MiscInline(::std::boxed::Box<crate::xhtml::groups::MiscInline>),
    }
    #[derive(
        ::core::fmt::Debug,
        ::core::clone::Clone,
        ::core::marker::Copy,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq
    )]
    #[xvalue(with = dir_value_with)]
    pub enum DirValue {
        Ltr,
        Rtl,
    }
    pub mod dir_value_with {
        pub fn deserialize<'de, D>(
            deserializer: D,
        ) -> ::core::result::Result<super::DirValue, D::Error>
        where
            D: ::xmlity::Deserializer<'de>,
        {
            let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                deserializer,
            )?;
            let value: ::std::string::String = text
                .parse()
                .map_err(::xmlity::de::Error::custom)?;
            super::DirValue::try_from(value).map_err(::xmlity::de::Error::custom)
        }
        pub fn serialize<S>(
            value: &super::DirValue,
            serializer: S,
        ) -> ::core::result::Result<S::Ok, S::Error>
        where
            S: ::xmlity::Serializer,
        {
            let value: ::std::string::String = ::core::clone::Clone::clone(value).into();
            ::xmlity::Serialize::serialize(
                ::std::string::String::as_str(
                    &::std::string::ToString::to_string(&value),
                ),
                serializer,
            )
        }
    }
    #[derive(::core::fmt::Debug)]
    pub enum DirValueParseError {
        NonExistent { value: ::std::string::String },
    }
    impl ::core::fmt::Display for DirValueParseError {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::result::Result<(), ::core::fmt::Error> {
            match self {
                DirValueParseError::NonExistent { value } => {
                    write!(f, "Value '{:?}' does not exist in the enumeration", value)
                }
            }
        }
    }
    impl ::core::convert::TryFrom<::std::string::String> for DirValue {
        type Error = DirValueParseError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::core::result::Result<Self, Self::Error> {
            match ::std::string::String::as_str(&value) {
                "ltr" => Ok(DirValue::Ltr),
                "rtl" => Ok(DirValue::Rtl),
                _ => {
                    Err(DirValueParseError::NonExistent {
                        value,
                    })
                }
            }
        }
    }
    impl ::core::convert::From<DirValue> for ::std::string::String {
        fn from(value: DirValue) -> Self {
            match value {
                DirValue::Ltr => ::std::string::String::from("ltr"),
                DirValue::Rtl => ::std::string::String::from("rtl"),
            }
        }
    }
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
#[xelement(
    name = "a",
    namespace = "http://www.w3.org/1999/xhtml",
    allow_unknown_attributes = "any"
)]
pub struct A {
    #[xattribute(name = "id", optional, default)]
    pub id: ::core::option::Option<String>,
    #[xattribute(name = "class", optional, default)]
    pub class: ::core::option::Option<String>,
    #[xattribute(name = "style", optional, default)]
    pub style: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::StyleSheet>,
    >,
    #[xattribute(name = "title", optional, default)]
    pub title: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Text>>,
    #[xattribute(name = "lang", optional, default)]
    pub lang: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::LanguageCode>,
    >,
    #[xattribute(deferred = true, optional, default)]
    pub attribute_5: ::core::option::Option<crate::xml::Lang>,
    #[xattribute(name = "dir", optional, default)]
    pub dir: ::core::option::Option<a_items::DirValue>,
    #[xattribute(name = "onclick", optional, default)]
    pub onclick: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xattribute(name = "ondblclick", optional, default)]
    pub ondblclick: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmousedown", optional, default)]
    pub onmousedown: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseup", optional, default)]
    pub onmouseup: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseover", optional, default)]
    pub onmouseover: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmousemove", optional, default)]
    pub onmousemove: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseout", optional, default)]
    pub onmouseout: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeypress", optional, default)]
    pub onkeypress: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeydown", optional, default)]
    pub onkeydown: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeyup", optional, default)]
    pub onkeyup: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xattribute(name = "accesskey", optional, default)]
    pub accesskey: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Character>,
    >,
    #[xattribute(name = "tabindex", optional, default)]
    pub tabindex: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::TabindexNumber>,
    >,
    #[xattribute(name = "onfocus", optional, default)]
    pub onfocus: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xattribute(name = "onblur", optional, default)]
    pub onblur: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xattribute(name = "charset", optional, default)]
    pub charset: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Charset>>,
    #[xattribute(name = "type", optional, default)]
    pub type_: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::ContentType>,
    >,
    #[xattribute(name = "name", optional, default)]
    pub name: ::core::option::Option<String>,
    #[xattribute(name = "href", optional, default)]
    pub href: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Uri>>,
    #[xattribute(name = "hreflang", optional, default)]
    pub hreflang: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::LanguageCode>,
    >,
    #[xattribute(name = "rel", optional, default)]
    pub rel: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::LinkTypes>>,
    #[xattribute(name = "rev", optional, default)]
    pub rev: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::LinkTypes>>,
    #[xattribute(name = "shape", optional, default)]
    pub shape: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Shape>>,
    #[xattribute(name = "coords", optional, default)]
    pub coords: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Coords>>,
    #[xvalue(default)]
    pub a: ::std::vec::Vec<a_items::A>,
}
pub mod abbr_items {
    impl ::core::convert::From<crate::xhtml::groups::Inline> for Inline {
        fn from(value: crate::xhtml::groups::Inline) -> Self {
            Inline::Inline(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml::groups::MiscInline> for Inline {
        fn from(value: crate::xhtml::groups::MiscInline) -> Self {
            Inline::MiscInline(::std::boxed::Box::new(value))
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub enum Inline {
        Inline(::std::boxed::Box<crate::xhtml::groups::Inline>),
        MiscInline(::std::boxed::Box<crate::xhtml::groups::MiscInline>),
    }
    #[derive(
        ::core::fmt::Debug,
        ::core::clone::Clone,
        ::core::marker::Copy,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq
    )]
    #[xvalue(with = dir_value_with)]
    pub enum DirValue {
        Ltr,
        Rtl,
    }
    pub mod dir_value_with {
        pub fn deserialize<'de, D>(
            deserializer: D,
        ) -> ::core::result::Result<super::DirValue, D::Error>
        where
            D: ::xmlity::Deserializer<'de>,
        {
            let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                deserializer,
            )?;
            let value: ::std::string::String = text
                .parse()
                .map_err(::xmlity::de::Error::custom)?;
            super::DirValue::try_from(value).map_err(::xmlity::de::Error::custom)
        }
        pub fn serialize<S>(
            value: &super::DirValue,
            serializer: S,
        ) -> ::core::result::Result<S::Ok, S::Error>
        where
            S: ::xmlity::Serializer,
        {
            let value: ::std::string::String = ::core::clone::Clone::clone(value).into();
            ::xmlity::Serialize::serialize(
                ::std::string::String::as_str(
                    &::std::string::ToString::to_string(&value),
                ),
                serializer,
            )
        }
    }
    #[derive(::core::fmt::Debug)]
    pub enum DirValueParseError {
        NonExistent { value: ::std::string::String },
    }
    impl ::core::fmt::Display for DirValueParseError {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::result::Result<(), ::core::fmt::Error> {
            match self {
                DirValueParseError::NonExistent { value } => {
                    write!(f, "Value '{:?}' does not exist in the enumeration", value)
                }
            }
        }
    }
    impl ::core::convert::TryFrom<::std::string::String> for DirValue {
        type Error = DirValueParseError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::core::result::Result<Self, Self::Error> {
            match ::std::string::String::as_str(&value) {
                "ltr" => Ok(DirValue::Ltr),
                "rtl" => Ok(DirValue::Rtl),
                _ => {
                    Err(DirValueParseError::NonExistent {
                        value,
                    })
                }
            }
        }
    }
    impl ::core::convert::From<DirValue> for ::std::string::String {
        fn from(value: DirValue) -> Self {
            match value {
                DirValue::Ltr => ::std::string::String::from("ltr"),
                DirValue::Rtl => ::std::string::String::from("rtl"),
            }
        }
    }
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
#[xelement(
    name = "abbr",
    namespace = "http://www.w3.org/1999/xhtml",
    allow_unknown_attributes = "any"
)]
pub struct Abbr {
    #[xattribute(name = "id", optional, default)]
    pub id: ::core::option::Option<String>,
    #[xattribute(name = "class", optional, default)]
    pub class: ::core::option::Option<String>,
    #[xattribute(name = "style", optional, default)]
    pub style: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::StyleSheet>,
    >,
    #[xattribute(name = "title", optional, default)]
    pub title: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Text>>,
    #[xattribute(name = "lang", optional, default)]
    pub lang: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::LanguageCode>,
    >,
    #[xattribute(deferred = true, optional, default)]
    pub attribute_5: ::core::option::Option<crate::xml::Lang>,
    #[xattribute(name = "dir", optional, default)]
    pub dir: ::core::option::Option<abbr_items::DirValue>,
    #[xattribute(name = "onclick", optional, default)]
    pub onclick: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xattribute(name = "ondblclick", optional, default)]
    pub ondblclick: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmousedown", optional, default)]
    pub onmousedown: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseup", optional, default)]
    pub onmouseup: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseover", optional, default)]
    pub onmouseover: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmousemove", optional, default)]
    pub onmousemove: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseout", optional, default)]
    pub onmouseout: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeypress", optional, default)]
    pub onkeypress: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeydown", optional, default)]
    pub onkeydown: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeyup", optional, default)]
    pub onkeyup: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xvalue(default)]
    pub inline: ::std::vec::Vec<abbr_items::Inline>,
}
pub mod acronym_items {
    impl ::core::convert::From<crate::xhtml::groups::Inline> for Inline {
        fn from(value: crate::xhtml::groups::Inline) -> Self {
            Inline::Inline(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml::groups::MiscInline> for Inline {
        fn from(value: crate::xhtml::groups::MiscInline) -> Self {
            Inline::MiscInline(::std::boxed::Box::new(value))
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub enum Inline {
        Inline(::std::boxed::Box<crate::xhtml::groups::Inline>),
        MiscInline(::std::boxed::Box<crate::xhtml::groups::MiscInline>),
    }
    #[derive(
        ::core::fmt::Debug,
        ::core::clone::Clone,
        ::core::marker::Copy,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq
    )]
    #[xvalue(with = dir_value_with)]
    pub enum DirValue {
        Ltr,
        Rtl,
    }
    pub mod dir_value_with {
        pub fn deserialize<'de, D>(
            deserializer: D,
        ) -> ::core::result::Result<super::DirValue, D::Error>
        where
            D: ::xmlity::Deserializer<'de>,
        {
            let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                deserializer,
            )?;
            let value: ::std::string::String = text
                .parse()
                .map_err(::xmlity::de::Error::custom)?;
            super::DirValue::try_from(value).map_err(::xmlity::de::Error::custom)
        }
        pub fn serialize<S>(
            value: &super::DirValue,
            serializer: S,
        ) -> ::core::result::Result<S::Ok, S::Error>
        where
            S: ::xmlity::Serializer,
        {
            let value: ::std::string::String = ::core::clone::Clone::clone(value).into();
            ::xmlity::Serialize::serialize(
                ::std::string::String::as_str(
                    &::std::string::ToString::to_string(&value),
                ),
                serializer,
            )
        }
    }
    #[derive(::core::fmt::Debug)]
    pub enum DirValueParseError {
        NonExistent { value: ::std::string::String },
    }
    impl ::core::fmt::Display for DirValueParseError {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::result::Result<(), ::core::fmt::Error> {
            match self {
                DirValueParseError::NonExistent { value } => {
                    write!(f, "Value '{:?}' does not exist in the enumeration", value)
                }
            }
        }
    }
    impl ::core::convert::TryFrom<::std::string::String> for DirValue {
        type Error = DirValueParseError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::core::result::Result<Self, Self::Error> {
            match ::std::string::String::as_str(&value) {
                "ltr" => Ok(DirValue::Ltr),
                "rtl" => Ok(DirValue::Rtl),
                _ => {
                    Err(DirValueParseError::NonExistent {
                        value,
                    })
                }
            }
        }
    }
    impl ::core::convert::From<DirValue> for ::std::string::String {
        fn from(value: DirValue) -> Self {
            match value {
                DirValue::Ltr => ::std::string::String::from("ltr"),
                DirValue::Rtl => ::std::string::String::from("rtl"),
            }
        }
    }
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
#[xelement(
    name = "acronym",
    namespace = "http://www.w3.org/1999/xhtml",
    allow_unknown_attributes = "any"
)]
pub struct Acronym {
    #[xattribute(name = "id", optional, default)]
    pub id: ::core::option::Option<String>,
    #[xattribute(name = "class", optional, default)]
    pub class: ::core::option::Option<String>,
    #[xattribute(name = "style", optional, default)]
    pub style: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::StyleSheet>,
    >,
    #[xattribute(name = "title", optional, default)]
    pub title: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Text>>,
    #[xattribute(name = "lang", optional, default)]
    pub lang: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::LanguageCode>,
    >,
    #[xattribute(deferred = true, optional, default)]
    pub attribute_5: ::core::option::Option<crate::xml::Lang>,
    #[xattribute(name = "dir", optional, default)]
    pub dir: ::core::option::Option<acronym_items::DirValue>,
    #[xattribute(name = "onclick", optional, default)]
    pub onclick: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xattribute(name = "ondblclick", optional, default)]
    pub ondblclick: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmousedown", optional, default)]
    pub onmousedown: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseup", optional, default)]
    pub onmouseup: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseover", optional, default)]
    pub onmouseover: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmousemove", optional, default)]
    pub onmousemove: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseout", optional, default)]
    pub onmouseout: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeypress", optional, default)]
    pub onkeypress: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeydown", optional, default)]
    pub onkeydown: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeyup", optional, default)]
    pub onkeyup: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xvalue(default)]
    pub inline: ::std::vec::Vec<acronym_items::Inline>,
}
pub mod address_items {
    impl ::core::convert::From<crate::xhtml::groups::Inline> for Inline {
        fn from(value: crate::xhtml::groups::Inline) -> Self {
            Inline::Inline(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml::groups::MiscInline> for Inline {
        fn from(value: crate::xhtml::groups::MiscInline) -> Self {
            Inline::MiscInline(::std::boxed::Box::new(value))
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub enum Inline {
        Inline(::std::boxed::Box<crate::xhtml::groups::Inline>),
        MiscInline(::std::boxed::Box<crate::xhtml::groups::MiscInline>),
    }
    #[derive(
        ::core::fmt::Debug,
        ::core::clone::Clone,
        ::core::marker::Copy,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq
    )]
    #[xvalue(with = dir_value_with)]
    pub enum DirValue {
        Ltr,
        Rtl,
    }
    pub mod dir_value_with {
        pub fn deserialize<'de, D>(
            deserializer: D,
        ) -> ::core::result::Result<super::DirValue, D::Error>
        where
            D: ::xmlity::Deserializer<'de>,
        {
            let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                deserializer,
            )?;
            let value: ::std::string::String = text
                .parse()
                .map_err(::xmlity::de::Error::custom)?;
            super::DirValue::try_from(value).map_err(::xmlity::de::Error::custom)
        }
        pub fn serialize<S>(
            value: &super::DirValue,
            serializer: S,
        ) -> ::core::result::Result<S::Ok, S::Error>
        where
            S: ::xmlity::Serializer,
        {
            let value: ::std::string::String = ::core::clone::Clone::clone(value).into();
            ::xmlity::Serialize::serialize(
                ::std::string::String::as_str(
                    &::std::string::ToString::to_string(&value),
                ),
                serializer,
            )
        }
    }
    #[derive(::core::fmt::Debug)]
    pub enum DirValueParseError {
        NonExistent { value: ::std::string::String },
    }
    impl ::core::fmt::Display for DirValueParseError {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::result::Result<(), ::core::fmt::Error> {
            match self {
                DirValueParseError::NonExistent { value } => {
                    write!(f, "Value '{:?}' does not exist in the enumeration", value)
                }
            }
        }
    }
    impl ::core::convert::TryFrom<::std::string::String> for DirValue {
        type Error = DirValueParseError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::core::result::Result<Self, Self::Error> {
            match ::std::string::String::as_str(&value) {
                "ltr" => Ok(DirValue::Ltr),
                "rtl" => Ok(DirValue::Rtl),
                _ => {
                    Err(DirValueParseError::NonExistent {
                        value,
                    })
                }
            }
        }
    }
    impl ::core::convert::From<DirValue> for ::std::string::String {
        fn from(value: DirValue) -> Self {
            match value {
                DirValue::Ltr => ::std::string::String::from("ltr"),
                DirValue::Rtl => ::std::string::String::from("rtl"),
            }
        }
    }
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
#[xelement(
    name = "address",
    namespace = "http://www.w3.org/1999/xhtml",
    allow_unknown_attributes = "any"
)]
pub struct Address {
    #[xattribute(name = "id", optional, default)]
    pub id: ::core::option::Option<String>,
    #[xattribute(name = "class", optional, default)]
    pub class: ::core::option::Option<String>,
    #[xattribute(name = "style", optional, default)]
    pub style: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::StyleSheet>,
    >,
    #[xattribute(name = "title", optional, default)]
    pub title: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Text>>,
    #[xattribute(name = "lang", optional, default)]
    pub lang: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::LanguageCode>,
    >,
    #[xattribute(deferred = true, optional, default)]
    pub attribute_5: ::core::option::Option<crate::xml::Lang>,
    #[xattribute(name = "dir", optional, default)]
    pub dir: ::core::option::Option<address_items::DirValue>,
    #[xattribute(name = "onclick", optional, default)]
    pub onclick: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xattribute(name = "ondblclick", optional, default)]
    pub ondblclick: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmousedown", optional, default)]
    pub onmousedown: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseup", optional, default)]
    pub onmouseup: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseover", optional, default)]
    pub onmouseover: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmousemove", optional, default)]
    pub onmousemove: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseout", optional, default)]
    pub onmouseout: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeypress", optional, default)]
    pub onkeypress: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeydown", optional, default)]
    pub onkeydown: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeyup", optional, default)]
    pub onkeyup: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xvalue(default)]
    pub inline: ::std::vec::Vec<address_items::Inline>,
}
pub mod area_items {
    #[derive(
        ::core::fmt::Debug,
        ::core::clone::Clone,
        ::core::marker::Copy,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq
    )]
    #[xvalue(with = dir_value_with)]
    pub enum DirValue {
        Ltr,
        Rtl,
    }
    pub mod dir_value_with {
        pub fn deserialize<'de, D>(
            deserializer: D,
        ) -> ::core::result::Result<super::DirValue, D::Error>
        where
            D: ::xmlity::Deserializer<'de>,
        {
            let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                deserializer,
            )?;
            let value: ::std::string::String = text
                .parse()
                .map_err(::xmlity::de::Error::custom)?;
            super::DirValue::try_from(value).map_err(::xmlity::de::Error::custom)
        }
        pub fn serialize<S>(
            value: &super::DirValue,
            serializer: S,
        ) -> ::core::result::Result<S::Ok, S::Error>
        where
            S: ::xmlity::Serializer,
        {
            let value: ::std::string::String = ::core::clone::Clone::clone(value).into();
            ::xmlity::Serialize::serialize(
                ::std::string::String::as_str(
                    &::std::string::ToString::to_string(&value),
                ),
                serializer,
            )
        }
    }
    #[derive(::core::fmt::Debug)]
    pub enum DirValueParseError {
        NonExistent { value: ::std::string::String },
    }
    impl ::core::fmt::Display for DirValueParseError {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::result::Result<(), ::core::fmt::Error> {
            match self {
                DirValueParseError::NonExistent { value } => {
                    write!(f, "Value '{:?}' does not exist in the enumeration", value)
                }
            }
        }
    }
    impl ::core::convert::TryFrom<::std::string::String> for DirValue {
        type Error = DirValueParseError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::core::result::Result<Self, Self::Error> {
            match ::std::string::String::as_str(&value) {
                "ltr" => Ok(DirValue::Ltr),
                "rtl" => Ok(DirValue::Rtl),
                _ => {
                    Err(DirValueParseError::NonExistent {
                        value,
                    })
                }
            }
        }
    }
    impl ::core::convert::From<DirValue> for ::std::string::String {
        fn from(value: DirValue) -> Self {
            match value {
                DirValue::Ltr => ::std::string::String::from("ltr"),
                DirValue::Rtl => ::std::string::String::from("rtl"),
            }
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::core::clone::Clone,
        ::core::marker::Copy,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq
    )]
    #[xvalue(with = nohref_value_with)]
    pub enum NohrefValue {
        Nohref,
    }
    pub mod nohref_value_with {
        pub fn deserialize<'de, D>(
            deserializer: D,
        ) -> ::core::result::Result<super::NohrefValue, D::Error>
        where
            D: ::xmlity::Deserializer<'de>,
        {
            let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                deserializer,
            )?;
            let value: ::std::string::String = text
                .parse()
                .map_err(::xmlity::de::Error::custom)?;
            super::NohrefValue::try_from(value).map_err(::xmlity::de::Error::custom)
        }
        pub fn serialize<S>(
            value: &super::NohrefValue,
            serializer: S,
        ) -> ::core::result::Result<S::Ok, S::Error>
        where
            S: ::xmlity::Serializer,
        {
            let value: ::std::string::String = ::core::clone::Clone::clone(value).into();
            ::xmlity::Serialize::serialize(
                ::std::string::String::as_str(
                    &::std::string::ToString::to_string(&value),
                ),
                serializer,
            )
        }
    }
    #[derive(::core::fmt::Debug)]
    pub enum NohrefValueParseError {
        NonExistent { value: ::std::string::String },
    }
    impl ::core::fmt::Display for NohrefValueParseError {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::result::Result<(), ::core::fmt::Error> {
            match self {
                NohrefValueParseError::NonExistent { value } => {
                    write!(f, "Value '{:?}' does not exist in the enumeration", value)
                }
            }
        }
    }
    impl ::core::convert::TryFrom<::std::string::String> for NohrefValue {
        type Error = NohrefValueParseError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::core::result::Result<Self, Self::Error> {
            match ::std::string::String::as_str(&value) {
                "nohref" => Ok(NohrefValue::Nohref),
                _ => {
                    Err(NohrefValueParseError::NonExistent {
                        value,
                    })
                }
            }
        }
    }
    impl ::core::convert::From<NohrefValue> for ::std::string::String {
        fn from(value: NohrefValue) -> Self {
            match value {
                NohrefValue::Nohref => ::std::string::String::from("nohref"),
            }
        }
    }
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
#[xelement(
    name = "area",
    namespace = "http://www.w3.org/1999/xhtml",
    allow_unknown_attributes = "any"
)]
pub struct Area {
    #[xattribute(name = "id", optional, default)]
    pub id: ::core::option::Option<String>,
    #[xattribute(name = "class", optional, default)]
    pub class: ::core::option::Option<String>,
    #[xattribute(name = "style", optional, default)]
    pub style: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::StyleSheet>,
    >,
    #[xattribute(name = "title", optional, default)]
    pub title: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Text>>,
    #[xattribute(name = "lang", optional, default)]
    pub lang: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::LanguageCode>,
    >,
    #[xattribute(deferred = true, optional, default)]
    pub attribute_5: ::core::option::Option<crate::xml::Lang>,
    #[xattribute(name = "dir", optional, default)]
    pub dir: ::core::option::Option<area_items::DirValue>,
    #[xattribute(name = "onclick", optional, default)]
    pub onclick: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xattribute(name = "ondblclick", optional, default)]
    pub ondblclick: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmousedown", optional, default)]
    pub onmousedown: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseup", optional, default)]
    pub onmouseup: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseover", optional, default)]
    pub onmouseover: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmousemove", optional, default)]
    pub onmousemove: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseout", optional, default)]
    pub onmouseout: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeypress", optional, default)]
    pub onkeypress: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeydown", optional, default)]
    pub onkeydown: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeyup", optional, default)]
    pub onkeyup: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xattribute(name = "accesskey", optional, default)]
    pub accesskey: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Character>,
    >,
    #[xattribute(name = "tabindex", optional, default)]
    pub tabindex: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::TabindexNumber>,
    >,
    #[xattribute(name = "onfocus", optional, default)]
    pub onfocus: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xattribute(name = "onblur", optional, default)]
    pub onblur: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xattribute(name = "shape", optional, default)]
    pub shape: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Shape>>,
    #[xattribute(name = "coords", optional, default)]
    pub coords: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Coords>>,
    #[xattribute(name = "href", optional, default)]
    pub href: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Uri>>,
    #[xattribute(name = "nohref", optional, default)]
    pub nohref: ::core::option::Option<area_items::NohrefValue>,
    #[xattribute(name = "alt")]
    pub alt: ::std::boxed::Box<crate::xhtml::types::Text>,
}
pub mod b_items {
    impl ::core::convert::From<crate::xhtml::groups::Inline> for Inline {
        fn from(value: crate::xhtml::groups::Inline) -> Self {
            Inline::Inline(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml::groups::MiscInline> for Inline {
        fn from(value: crate::xhtml::groups::MiscInline) -> Self {
            Inline::MiscInline(::std::boxed::Box::new(value))
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub enum Inline {
        Inline(::std::boxed::Box<crate::xhtml::groups::Inline>),
        MiscInline(::std::boxed::Box<crate::xhtml::groups::MiscInline>),
    }
    #[derive(
        ::core::fmt::Debug,
        ::core::clone::Clone,
        ::core::marker::Copy,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq
    )]
    #[xvalue(with = dir_value_with)]
    pub enum DirValue {
        Ltr,
        Rtl,
    }
    pub mod dir_value_with {
        pub fn deserialize<'de, D>(
            deserializer: D,
        ) -> ::core::result::Result<super::DirValue, D::Error>
        where
            D: ::xmlity::Deserializer<'de>,
        {
            let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                deserializer,
            )?;
            let value: ::std::string::String = text
                .parse()
                .map_err(::xmlity::de::Error::custom)?;
            super::DirValue::try_from(value).map_err(::xmlity::de::Error::custom)
        }
        pub fn serialize<S>(
            value: &super::DirValue,
            serializer: S,
        ) -> ::core::result::Result<S::Ok, S::Error>
        where
            S: ::xmlity::Serializer,
        {
            let value: ::std::string::String = ::core::clone::Clone::clone(value).into();
            ::xmlity::Serialize::serialize(
                ::std::string::String::as_str(
                    &::std::string::ToString::to_string(&value),
                ),
                serializer,
            )
        }
    }
    #[derive(::core::fmt::Debug)]
    pub enum DirValueParseError {
        NonExistent { value: ::std::string::String },
    }
    impl ::core::fmt::Display for DirValueParseError {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::result::Result<(), ::core::fmt::Error> {
            match self {
                DirValueParseError::NonExistent { value } => {
                    write!(f, "Value '{:?}' does not exist in the enumeration", value)
                }
            }
        }
    }
    impl ::core::convert::TryFrom<::std::string::String> for DirValue {
        type Error = DirValueParseError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::core::result::Result<Self, Self::Error> {
            match ::std::string::String::as_str(&value) {
                "ltr" => Ok(DirValue::Ltr),
                "rtl" => Ok(DirValue::Rtl),
                _ => {
                    Err(DirValueParseError::NonExistent {
                        value,
                    })
                }
            }
        }
    }
    impl ::core::convert::From<DirValue> for ::std::string::String {
        fn from(value: DirValue) -> Self {
            match value {
                DirValue::Ltr => ::std::string::String::from("ltr"),
                DirValue::Rtl => ::std::string::String::from("rtl"),
            }
        }
    }
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
#[xelement(
    name = "b",
    namespace = "http://www.w3.org/1999/xhtml",
    allow_unknown_attributes = "any"
)]
pub struct B {
    #[xattribute(name = "id", optional, default)]
    pub id: ::core::option::Option<String>,
    #[xattribute(name = "class", optional, default)]
    pub class: ::core::option::Option<String>,
    #[xattribute(name = "style", optional, default)]
    pub style: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::StyleSheet>,
    >,
    #[xattribute(name = "title", optional, default)]
    pub title: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Text>>,
    #[xattribute(name = "lang", optional, default)]
    pub lang: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::LanguageCode>,
    >,
    #[xattribute(deferred = true, optional, default)]
    pub attribute_5: ::core::option::Option<crate::xml::Lang>,
    #[xattribute(name = "dir", optional, default)]
    pub dir: ::core::option::Option<b_items::DirValue>,
    #[xattribute(name = "onclick", optional, default)]
    pub onclick: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xattribute(name = "ondblclick", optional, default)]
    pub ondblclick: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmousedown", optional, default)]
    pub onmousedown: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseup", optional, default)]
    pub onmouseup: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseover", optional, default)]
    pub onmouseover: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmousemove", optional, default)]
    pub onmousemove: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseout", optional, default)]
    pub onmouseout: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeypress", optional, default)]
    pub onkeypress: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeydown", optional, default)]
    pub onkeydown: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeyup", optional, default)]
    pub onkeyup: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xvalue(default)]
    pub inline: ::std::vec::Vec<b_items::Inline>,
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
#[xelement(
    name = "base",
    namespace = "http://www.w3.org/1999/xhtml",
    allow_unknown_attributes = "any"
)]
pub struct Base {
    #[xattribute(name = "href")]
    pub href: ::std::boxed::Box<crate::xhtml::types::Uri>,
    #[xattribute(name = "id", optional, default)]
    pub id: ::core::option::Option<String>,
}
pub mod bdo_items {
    impl ::core::convert::From<crate::xhtml::groups::Inline> for Inline {
        fn from(value: crate::xhtml::groups::Inline) -> Self {
            Inline::Inline(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml::groups::MiscInline> for Inline {
        fn from(value: crate::xhtml::groups::MiscInline) -> Self {
            Inline::MiscInline(::std::boxed::Box::new(value))
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub enum Inline {
        Inline(::std::boxed::Box<crate::xhtml::groups::Inline>),
        MiscInline(::std::boxed::Box<crate::xhtml::groups::MiscInline>),
    }
    #[derive(
        ::core::fmt::Debug,
        ::core::clone::Clone,
        ::core::marker::Copy,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq
    )]
    #[xvalue(with = dir_value_with)]
    pub enum DirValue {
        Ltr,
        Rtl,
    }
    pub mod dir_value_with {
        pub fn deserialize<'de, D>(
            deserializer: D,
        ) -> ::core::result::Result<super::DirValue, D::Error>
        where
            D: ::xmlity::Deserializer<'de>,
        {
            let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                deserializer,
            )?;
            let value: ::std::string::String = text
                .parse()
                .map_err(::xmlity::de::Error::custom)?;
            super::DirValue::try_from(value).map_err(::xmlity::de::Error::custom)
        }
        pub fn serialize<S>(
            value: &super::DirValue,
            serializer: S,
        ) -> ::core::result::Result<S::Ok, S::Error>
        where
            S: ::xmlity::Serializer,
        {
            let value: ::std::string::String = ::core::clone::Clone::clone(value).into();
            ::xmlity::Serialize::serialize(
                ::std::string::String::as_str(
                    &::std::string::ToString::to_string(&value),
                ),
                serializer,
            )
        }
    }
    #[derive(::core::fmt::Debug)]
    pub enum DirValueParseError {
        NonExistent { value: ::std::string::String },
    }
    impl ::core::fmt::Display for DirValueParseError {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::result::Result<(), ::core::fmt::Error> {
            match self {
                DirValueParseError::NonExistent { value } => {
                    write!(f, "Value '{:?}' does not exist in the enumeration", value)
                }
            }
        }
    }
    impl ::core::convert::TryFrom<::std::string::String> for DirValue {
        type Error = DirValueParseError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::core::result::Result<Self, Self::Error> {
            match ::std::string::String::as_str(&value) {
                "ltr" => Ok(DirValue::Ltr),
                "rtl" => Ok(DirValue::Rtl),
                _ => {
                    Err(DirValueParseError::NonExistent {
                        value,
                    })
                }
            }
        }
    }
    impl ::core::convert::From<DirValue> for ::std::string::String {
        fn from(value: DirValue) -> Self {
            match value {
                DirValue::Ltr => ::std::string::String::from("ltr"),
                DirValue::Rtl => ::std::string::String::from("rtl"),
            }
        }
    }
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
#[xelement(
    name = "bdo",
    namespace = "http://www.w3.org/1999/xhtml",
    allow_unknown_attributes = "any"
)]
pub struct Bdo {
    #[xattribute(name = "id", optional, default)]
    pub id: ::core::option::Option<String>,
    #[xattribute(name = "class", optional, default)]
    pub class: ::core::option::Option<String>,
    #[xattribute(name = "style", optional, default)]
    pub style: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::StyleSheet>,
    >,
    #[xattribute(name = "title", optional, default)]
    pub title: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Text>>,
    #[xattribute(name = "onclick", optional, default)]
    pub onclick: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xattribute(name = "ondblclick", optional, default)]
    pub ondblclick: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmousedown", optional, default)]
    pub onmousedown: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseup", optional, default)]
    pub onmouseup: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseover", optional, default)]
    pub onmouseover: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmousemove", optional, default)]
    pub onmousemove: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseout", optional, default)]
    pub onmouseout: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeypress", optional, default)]
    pub onkeypress: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeydown", optional, default)]
    pub onkeydown: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeyup", optional, default)]
    pub onkeyup: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xattribute(name = "lang", optional, default)]
    pub lang: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::LanguageCode>,
    >,
    #[xattribute(deferred = true, optional, default)]
    pub attribute_15: ::core::option::Option<crate::xml::Lang>,
    #[xattribute(name = "dir")]
    pub dir: bdo_items::DirValue,
    #[xvalue(default)]
    pub inline: ::std::vec::Vec<bdo_items::Inline>,
}
pub mod big_items {
    impl ::core::convert::From<crate::xhtml::groups::Inline> for Inline {
        fn from(value: crate::xhtml::groups::Inline) -> Self {
            Inline::Inline(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml::groups::MiscInline> for Inline {
        fn from(value: crate::xhtml::groups::MiscInline) -> Self {
            Inline::MiscInline(::std::boxed::Box::new(value))
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub enum Inline {
        Inline(::std::boxed::Box<crate::xhtml::groups::Inline>),
        MiscInline(::std::boxed::Box<crate::xhtml::groups::MiscInline>),
    }
    #[derive(
        ::core::fmt::Debug,
        ::core::clone::Clone,
        ::core::marker::Copy,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq
    )]
    #[xvalue(with = dir_value_with)]
    pub enum DirValue {
        Ltr,
        Rtl,
    }
    pub mod dir_value_with {
        pub fn deserialize<'de, D>(
            deserializer: D,
        ) -> ::core::result::Result<super::DirValue, D::Error>
        where
            D: ::xmlity::Deserializer<'de>,
        {
            let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                deserializer,
            )?;
            let value: ::std::string::String = text
                .parse()
                .map_err(::xmlity::de::Error::custom)?;
            super::DirValue::try_from(value).map_err(::xmlity::de::Error::custom)
        }
        pub fn serialize<S>(
            value: &super::DirValue,
            serializer: S,
        ) -> ::core::result::Result<S::Ok, S::Error>
        where
            S: ::xmlity::Serializer,
        {
            let value: ::std::string::String = ::core::clone::Clone::clone(value).into();
            ::xmlity::Serialize::serialize(
                ::std::string::String::as_str(
                    &::std::string::ToString::to_string(&value),
                ),
                serializer,
            )
        }
    }
    #[derive(::core::fmt::Debug)]
    pub enum DirValueParseError {
        NonExistent { value: ::std::string::String },
    }
    impl ::core::fmt::Display for DirValueParseError {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::result::Result<(), ::core::fmt::Error> {
            match self {
                DirValueParseError::NonExistent { value } => {
                    write!(f, "Value '{:?}' does not exist in the enumeration", value)
                }
            }
        }
    }
    impl ::core::convert::TryFrom<::std::string::String> for DirValue {
        type Error = DirValueParseError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::core::result::Result<Self, Self::Error> {
            match ::std::string::String::as_str(&value) {
                "ltr" => Ok(DirValue::Ltr),
                "rtl" => Ok(DirValue::Rtl),
                _ => {
                    Err(DirValueParseError::NonExistent {
                        value,
                    })
                }
            }
        }
    }
    impl ::core::convert::From<DirValue> for ::std::string::String {
        fn from(value: DirValue) -> Self {
            match value {
                DirValue::Ltr => ::std::string::String::from("ltr"),
                DirValue::Rtl => ::std::string::String::from("rtl"),
            }
        }
    }
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
#[xelement(
    name = "big",
    namespace = "http://www.w3.org/1999/xhtml",
    allow_unknown_attributes = "any"
)]
pub struct Big {
    #[xattribute(name = "id", optional, default)]
    pub id: ::core::option::Option<String>,
    #[xattribute(name = "class", optional, default)]
    pub class: ::core::option::Option<String>,
    #[xattribute(name = "style", optional, default)]
    pub style: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::StyleSheet>,
    >,
    #[xattribute(name = "title", optional, default)]
    pub title: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Text>>,
    #[xattribute(name = "lang", optional, default)]
    pub lang: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::LanguageCode>,
    >,
    #[xattribute(deferred = true, optional, default)]
    pub attribute_5: ::core::option::Option<crate::xml::Lang>,
    #[xattribute(name = "dir", optional, default)]
    pub dir: ::core::option::Option<big_items::DirValue>,
    #[xattribute(name = "onclick", optional, default)]
    pub onclick: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xattribute(name = "ondblclick", optional, default)]
    pub ondblclick: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmousedown", optional, default)]
    pub onmousedown: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseup", optional, default)]
    pub onmouseup: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseover", optional, default)]
    pub onmouseover: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmousemove", optional, default)]
    pub onmousemove: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseout", optional, default)]
    pub onmouseout: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeypress", optional, default)]
    pub onkeypress: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeydown", optional, default)]
    pub onkeydown: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeyup", optional, default)]
    pub onkeyup: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xvalue(default)]
    pub inline: ::std::vec::Vec<big_items::Inline>,
}
pub mod blockquote_items {
    impl ::core::convert::From<crate::xhtml::groups::Block> for Blockquote {
        fn from(value: crate::xhtml::groups::Block) -> Self {
            Blockquote::Block(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml::Form> for Blockquote {
        fn from(value: crate::xhtml::Form) -> Self {
            Blockquote::Form(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml::groups::Misc> for Blockquote {
        fn from(value: crate::xhtml::groups::Misc) -> Self {
            Blockquote::Misc(::std::boxed::Box::new(value))
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub enum Blockquote {
        Block(::std::boxed::Box<crate::xhtml::groups::Block>),
        Form(::std::boxed::Box<crate::xhtml::Form>),
        Misc(::std::boxed::Box<crate::xhtml::groups::Misc>),
    }
    #[derive(
        ::core::fmt::Debug,
        ::core::clone::Clone,
        ::core::marker::Copy,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq
    )]
    #[xvalue(with = dir_value_with)]
    pub enum DirValue {
        Ltr,
        Rtl,
    }
    pub mod dir_value_with {
        pub fn deserialize<'de, D>(
            deserializer: D,
        ) -> ::core::result::Result<super::DirValue, D::Error>
        where
            D: ::xmlity::Deserializer<'de>,
        {
            let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                deserializer,
            )?;
            let value: ::std::string::String = text
                .parse()
                .map_err(::xmlity::de::Error::custom)?;
            super::DirValue::try_from(value).map_err(::xmlity::de::Error::custom)
        }
        pub fn serialize<S>(
            value: &super::DirValue,
            serializer: S,
        ) -> ::core::result::Result<S::Ok, S::Error>
        where
            S: ::xmlity::Serializer,
        {
            let value: ::std::string::String = ::core::clone::Clone::clone(value).into();
            ::xmlity::Serialize::serialize(
                ::std::string::String::as_str(
                    &::std::string::ToString::to_string(&value),
                ),
                serializer,
            )
        }
    }
    #[derive(::core::fmt::Debug)]
    pub enum DirValueParseError {
        NonExistent { value: ::std::string::String },
    }
    impl ::core::fmt::Display for DirValueParseError {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::result::Result<(), ::core::fmt::Error> {
            match self {
                DirValueParseError::NonExistent { value } => {
                    write!(f, "Value '{:?}' does not exist in the enumeration", value)
                }
            }
        }
    }
    impl ::core::convert::TryFrom<::std::string::String> for DirValue {
        type Error = DirValueParseError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::core::result::Result<Self, Self::Error> {
            match ::std::string::String::as_str(&value) {
                "ltr" => Ok(DirValue::Ltr),
                "rtl" => Ok(DirValue::Rtl),
                _ => {
                    Err(DirValueParseError::NonExistent {
                        value,
                    })
                }
            }
        }
    }
    impl ::core::convert::From<DirValue> for ::std::string::String {
        fn from(value: DirValue) -> Self {
            match value {
                DirValue::Ltr => ::std::string::String::from("ltr"),
                DirValue::Rtl => ::std::string::String::from("rtl"),
            }
        }
    }
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
#[xelement(
    name = "blockquote",
    namespace = "http://www.w3.org/1999/xhtml",
    allow_unknown_attributes = "any"
)]
pub struct Blockquote {
    #[xattribute(name = "id", optional, default)]
    pub id: ::core::option::Option<String>,
    #[xattribute(name = "class", optional, default)]
    pub class: ::core::option::Option<String>,
    #[xattribute(name = "style", optional, default)]
    pub style: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::StyleSheet>,
    >,
    #[xattribute(name = "title", optional, default)]
    pub title: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Text>>,
    #[xattribute(name = "lang", optional, default)]
    pub lang: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::LanguageCode>,
    >,
    #[xattribute(deferred = true, optional, default)]
    pub attribute_5: ::core::option::Option<crate::xml::Lang>,
    #[xattribute(name = "dir", optional, default)]
    pub dir: ::core::option::Option<blockquote_items::DirValue>,
    #[xattribute(name = "onclick", optional, default)]
    pub onclick: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xattribute(name = "ondblclick", optional, default)]
    pub ondblclick: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmousedown", optional, default)]
    pub onmousedown: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseup", optional, default)]
    pub onmouseup: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseover", optional, default)]
    pub onmouseover: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmousemove", optional, default)]
    pub onmousemove: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseout", optional, default)]
    pub onmouseout: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeypress", optional, default)]
    pub onkeypress: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeydown", optional, default)]
    pub onkeydown: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeyup", optional, default)]
    pub onkeyup: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xattribute(name = "cite", optional, default)]
    pub cite: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Uri>>,
    #[xvalue(default)]
    pub blockquote: ::std::vec::Vec<blockquote_items::Blockquote>,
}
pub mod body_items {
    impl ::core::convert::From<crate::xhtml::groups::Block> for Body {
        fn from(value: crate::xhtml::groups::Block) -> Self {
            Body::Block(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml::Form> for Body {
        fn from(value: crate::xhtml::Form) -> Self {
            Body::Form(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml::groups::Misc> for Body {
        fn from(value: crate::xhtml::groups::Misc) -> Self {
            Body::Misc(::std::boxed::Box::new(value))
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub enum Body {
        Block(::std::boxed::Box<crate::xhtml::groups::Block>),
        Form(::std::boxed::Box<crate::xhtml::Form>),
        Misc(::std::boxed::Box<crate::xhtml::groups::Misc>),
    }
    #[derive(
        ::core::fmt::Debug,
        ::core::clone::Clone,
        ::core::marker::Copy,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq
    )]
    #[xvalue(with = dir_value_with)]
    pub enum DirValue {
        Ltr,
        Rtl,
    }
    pub mod dir_value_with {
        pub fn deserialize<'de, D>(
            deserializer: D,
        ) -> ::core::result::Result<super::DirValue, D::Error>
        where
            D: ::xmlity::Deserializer<'de>,
        {
            let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                deserializer,
            )?;
            let value: ::std::string::String = text
                .parse()
                .map_err(::xmlity::de::Error::custom)?;
            super::DirValue::try_from(value).map_err(::xmlity::de::Error::custom)
        }
        pub fn serialize<S>(
            value: &super::DirValue,
            serializer: S,
        ) -> ::core::result::Result<S::Ok, S::Error>
        where
            S: ::xmlity::Serializer,
        {
            let value: ::std::string::String = ::core::clone::Clone::clone(value).into();
            ::xmlity::Serialize::serialize(
                ::std::string::String::as_str(
                    &::std::string::ToString::to_string(&value),
                ),
                serializer,
            )
        }
    }
    #[derive(::core::fmt::Debug)]
    pub enum DirValueParseError {
        NonExistent { value: ::std::string::String },
    }
    impl ::core::fmt::Display for DirValueParseError {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::result::Result<(), ::core::fmt::Error> {
            match self {
                DirValueParseError::NonExistent { value } => {
                    write!(f, "Value '{:?}' does not exist in the enumeration", value)
                }
            }
        }
    }
    impl ::core::convert::TryFrom<::std::string::String> for DirValue {
        type Error = DirValueParseError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::core::result::Result<Self, Self::Error> {
            match ::std::string::String::as_str(&value) {
                "ltr" => Ok(DirValue::Ltr),
                "rtl" => Ok(DirValue::Rtl),
                _ => {
                    Err(DirValueParseError::NonExistent {
                        value,
                    })
                }
            }
        }
    }
    impl ::core::convert::From<DirValue> for ::std::string::String {
        fn from(value: DirValue) -> Self {
            match value {
                DirValue::Ltr => ::std::string::String::from("ltr"),
                DirValue::Rtl => ::std::string::String::from("rtl"),
            }
        }
    }
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
#[xelement(
    name = "body",
    namespace = "http://www.w3.org/1999/xhtml",
    allow_unknown_attributes = "any"
)]
pub struct Body {
    #[xattribute(name = "id", optional, default)]
    pub id: ::core::option::Option<String>,
    #[xattribute(name = "class", optional, default)]
    pub class: ::core::option::Option<String>,
    #[xattribute(name = "style", optional, default)]
    pub style: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::StyleSheet>,
    >,
    #[xattribute(name = "title", optional, default)]
    pub title: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Text>>,
    #[xattribute(name = "lang", optional, default)]
    pub lang: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::LanguageCode>,
    >,
    #[xattribute(deferred = true, optional, default)]
    pub attribute_5: ::core::option::Option<crate::xml::Lang>,
    #[xattribute(name = "dir", optional, default)]
    pub dir: ::core::option::Option<body_items::DirValue>,
    #[xattribute(name = "onclick", optional, default)]
    pub onclick: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xattribute(name = "ondblclick", optional, default)]
    pub ondblclick: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmousedown", optional, default)]
    pub onmousedown: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseup", optional, default)]
    pub onmouseup: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseover", optional, default)]
    pub onmouseover: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmousemove", optional, default)]
    pub onmousemove: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseout", optional, default)]
    pub onmouseout: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeypress", optional, default)]
    pub onkeypress: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeydown", optional, default)]
    pub onkeydown: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeyup", optional, default)]
    pub onkeyup: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xattribute(name = "onload", optional, default)]
    pub onload: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xattribute(name = "onunload", optional, default)]
    pub onunload: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xvalue(default)]
    pub body: ::std::vec::Vec<body_items::Body>,
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
#[xelement(
    name = "br",
    namespace = "http://www.w3.org/1999/xhtml",
    allow_unknown_attributes = "any"
)]
pub struct Br {
    #[xattribute(name = "id", optional, default)]
    pub id: ::core::option::Option<String>,
    #[xattribute(name = "class", optional, default)]
    pub class: ::core::option::Option<String>,
    #[xattribute(name = "style", optional, default)]
    pub style: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::StyleSheet>,
    >,
    #[xattribute(name = "title", optional, default)]
    pub title: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Text>>,
}
pub mod button_items {
    impl ::core::convert::From<crate::xhtml::P> for Button {
        fn from(value: crate::xhtml::P) -> Self {
            Button::P(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml::groups::Heading> for Button {
        fn from(value: crate::xhtml::groups::Heading) -> Self {
            Button::Heading(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml::Div> for Button {
        fn from(value: crate::xhtml::Div) -> Self {
            Button::Div(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml::groups::Lists> for Button {
        fn from(value: crate::xhtml::groups::Lists) -> Self {
            Button::Lists(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml::groups::Blocktext> for Button {
        fn from(value: crate::xhtml::groups::Blocktext) -> Self {
            Button::Blocktext(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml::Table> for Button {
        fn from(value: crate::xhtml::Table) -> Self {
            Button::Table(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml::groups::Special> for Button {
        fn from(value: crate::xhtml::groups::Special) -> Self {
            Button::Special(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml::groups::Fontstyle> for Button {
        fn from(value: crate::xhtml::groups::Fontstyle) -> Self {
            Button::Fontstyle(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml::groups::Phrase> for Button {
        fn from(value: crate::xhtml::groups::Phrase) -> Self {
            Button::Phrase(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml::groups::Misc> for Button {
        fn from(value: crate::xhtml::groups::Misc) -> Self {
            Button::Misc(::std::boxed::Box::new(value))
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub enum Button {
        P(::std::boxed::Box<crate::xhtml::P>),
        Heading(::std::boxed::Box<crate::xhtml::groups::Heading>),
        Div(::std::boxed::Box<crate::xhtml::Div>),
        Lists(::std::boxed::Box<crate::xhtml::groups::Lists>),
        Blocktext(::std::boxed::Box<crate::xhtml::groups::Blocktext>),
        Table(::std::boxed::Box<crate::xhtml::Table>),
        Special(::std::boxed::Box<crate::xhtml::groups::Special>),
        Fontstyle(::std::boxed::Box<crate::xhtml::groups::Fontstyle>),
        Phrase(::std::boxed::Box<crate::xhtml::groups::Phrase>),
        Misc(::std::boxed::Box<crate::xhtml::groups::Misc>),
    }
    #[derive(
        ::core::fmt::Debug,
        ::core::clone::Clone,
        ::core::marker::Copy,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq
    )]
    #[xvalue(with = dir_value_with)]
    pub enum DirValue {
        Ltr,
        Rtl,
    }
    pub mod dir_value_with {
        pub fn deserialize<'de, D>(
            deserializer: D,
        ) -> ::core::result::Result<super::DirValue, D::Error>
        where
            D: ::xmlity::Deserializer<'de>,
        {
            let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                deserializer,
            )?;
            let value: ::std::string::String = text
                .parse()
                .map_err(::xmlity::de::Error::custom)?;
            super::DirValue::try_from(value).map_err(::xmlity::de::Error::custom)
        }
        pub fn serialize<S>(
            value: &super::DirValue,
            serializer: S,
        ) -> ::core::result::Result<S::Ok, S::Error>
        where
            S: ::xmlity::Serializer,
        {
            let value: ::std::string::String = ::core::clone::Clone::clone(value).into();
            ::xmlity::Serialize::serialize(
                ::std::string::String::as_str(
                    &::std::string::ToString::to_string(&value),
                ),
                serializer,
            )
        }
    }
    #[derive(::core::fmt::Debug)]
    pub enum DirValueParseError {
        NonExistent { value: ::std::string::String },
    }
    impl ::core::fmt::Display for DirValueParseError {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::result::Result<(), ::core::fmt::Error> {
            match self {
                DirValueParseError::NonExistent { value } => {
                    write!(f, "Value '{:?}' does not exist in the enumeration", value)
                }
            }
        }
    }
    impl ::core::convert::TryFrom<::std::string::String> for DirValue {
        type Error = DirValueParseError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::core::result::Result<Self, Self::Error> {
            match ::std::string::String::as_str(&value) {
                "ltr" => Ok(DirValue::Ltr),
                "rtl" => Ok(DirValue::Rtl),
                _ => {
                    Err(DirValueParseError::NonExistent {
                        value,
                    })
                }
            }
        }
    }
    impl ::core::convert::From<DirValue> for ::std::string::String {
        fn from(value: DirValue) -> Self {
            match value {
                DirValue::Ltr => ::std::string::String::from("ltr"),
                DirValue::Rtl => ::std::string::String::from("rtl"),
            }
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::core::clone::Clone,
        ::core::marker::Copy,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq
    )]
    #[xvalue(with = type_value_with)]
    pub enum TypeValue {
        Button,
        Submit,
        Reset,
    }
    pub mod type_value_with {
        pub fn deserialize<'de, D>(
            deserializer: D,
        ) -> ::core::result::Result<super::TypeValue, D::Error>
        where
            D: ::xmlity::Deserializer<'de>,
        {
            let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                deserializer,
            )?;
            let value: ::std::string::String = text
                .parse()
                .map_err(::xmlity::de::Error::custom)?;
            super::TypeValue::try_from(value).map_err(::xmlity::de::Error::custom)
        }
        pub fn serialize<S>(
            value: &super::TypeValue,
            serializer: S,
        ) -> ::core::result::Result<S::Ok, S::Error>
        where
            S: ::xmlity::Serializer,
        {
            let value: ::std::string::String = ::core::clone::Clone::clone(value).into();
            ::xmlity::Serialize::serialize(
                ::std::string::String::as_str(
                    &::std::string::ToString::to_string(&value),
                ),
                serializer,
            )
        }
    }
    #[derive(::core::fmt::Debug)]
    pub enum TypeValueParseError {
        NonExistent { value: ::std::string::String },
    }
    impl ::core::fmt::Display for TypeValueParseError {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::result::Result<(), ::core::fmt::Error> {
            match self {
                TypeValueParseError::NonExistent { value } => {
                    write!(f, "Value '{:?}' does not exist in the enumeration", value)
                }
            }
        }
    }
    impl ::core::convert::TryFrom<::std::string::String> for TypeValue {
        type Error = TypeValueParseError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::core::result::Result<Self, Self::Error> {
            match ::std::string::String::as_str(&value) {
                "button" => Ok(TypeValue::Button),
                "submit" => Ok(TypeValue::Submit),
                "reset" => Ok(TypeValue::Reset),
                _ => {
                    Err(TypeValueParseError::NonExistent {
                        value,
                    })
                }
            }
        }
    }
    impl ::core::convert::From<TypeValue> for ::std::string::String {
        fn from(value: TypeValue) -> Self {
            match value {
                TypeValue::Button => ::std::string::String::from("button"),
                TypeValue::Submit => ::std::string::String::from("submit"),
                TypeValue::Reset => ::std::string::String::from("reset"),
            }
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::core::clone::Clone,
        ::core::marker::Copy,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq
    )]
    #[xvalue(with = disabled_value_with)]
    pub enum DisabledValue {
        Disabled,
    }
    pub mod disabled_value_with {
        pub fn deserialize<'de, D>(
            deserializer: D,
        ) -> ::core::result::Result<super::DisabledValue, D::Error>
        where
            D: ::xmlity::Deserializer<'de>,
        {
            let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                deserializer,
            )?;
            let value: ::std::string::String = text
                .parse()
                .map_err(::xmlity::de::Error::custom)?;
            super::DisabledValue::try_from(value).map_err(::xmlity::de::Error::custom)
        }
        pub fn serialize<S>(
            value: &super::DisabledValue,
            serializer: S,
        ) -> ::core::result::Result<S::Ok, S::Error>
        where
            S: ::xmlity::Serializer,
        {
            let value: ::std::string::String = ::core::clone::Clone::clone(value).into();
            ::xmlity::Serialize::serialize(
                ::std::string::String::as_str(
                    &::std::string::ToString::to_string(&value),
                ),
                serializer,
            )
        }
    }
    #[derive(::core::fmt::Debug)]
    pub enum DisabledValueParseError {
        NonExistent { value: ::std::string::String },
    }
    impl ::core::fmt::Display for DisabledValueParseError {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::result::Result<(), ::core::fmt::Error> {
            match self {
                DisabledValueParseError::NonExistent { value } => {
                    write!(f, "Value '{:?}' does not exist in the enumeration", value)
                }
            }
        }
    }
    impl ::core::convert::TryFrom<::std::string::String> for DisabledValue {
        type Error = DisabledValueParseError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::core::result::Result<Self, Self::Error> {
            match ::std::string::String::as_str(&value) {
                "disabled" => Ok(DisabledValue::Disabled),
                _ => {
                    Err(DisabledValueParseError::NonExistent {
                        value,
                    })
                }
            }
        }
    }
    impl ::core::convert::From<DisabledValue> for ::std::string::String {
        fn from(value: DisabledValue) -> Self {
            match value {
                DisabledValue::Disabled => ::std::string::String::from("disabled"),
            }
        }
    }
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
#[xelement(
    name = "button",
    namespace = "http://www.w3.org/1999/xhtml",
    allow_unknown_attributes = "any"
)]
pub struct Button {
    #[xattribute(name = "id", optional, default)]
    pub id: ::core::option::Option<String>,
    #[xattribute(name = "class", optional, default)]
    pub class: ::core::option::Option<String>,
    #[xattribute(name = "style", optional, default)]
    pub style: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::StyleSheet>,
    >,
    #[xattribute(name = "title", optional, default)]
    pub title: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Text>>,
    #[xattribute(name = "lang", optional, default)]
    pub lang: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::LanguageCode>,
    >,
    #[xattribute(deferred = true, optional, default)]
    pub attribute_5: ::core::option::Option<crate::xml::Lang>,
    #[xattribute(name = "dir", optional, default)]
    pub dir: ::core::option::Option<button_items::DirValue>,
    #[xattribute(name = "onclick", optional, default)]
    pub onclick: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xattribute(name = "ondblclick", optional, default)]
    pub ondblclick: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmousedown", optional, default)]
    pub onmousedown: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseup", optional, default)]
    pub onmouseup: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseover", optional, default)]
    pub onmouseover: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmousemove", optional, default)]
    pub onmousemove: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseout", optional, default)]
    pub onmouseout: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeypress", optional, default)]
    pub onkeypress: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeydown", optional, default)]
    pub onkeydown: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeyup", optional, default)]
    pub onkeyup: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xattribute(name = "accesskey", optional, default)]
    pub accesskey: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Character>,
    >,
    #[xattribute(name = "tabindex", optional, default)]
    pub tabindex: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::TabindexNumber>,
    >,
    #[xattribute(name = "onfocus", optional, default)]
    pub onfocus: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xattribute(name = "onblur", optional, default)]
    pub onblur: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xattribute(name = "name", optional, default)]
    pub name: ::core::option::Option<String>,
    #[xattribute(name = "value", optional, default)]
    pub value: ::core::option::Option<String>,
    #[xattribute(name = "type", optional, default)]
    pub type_: ::core::option::Option<button_items::TypeValue>,
    #[xattribute(name = "disabled", optional, default)]
    pub disabled: ::core::option::Option<button_items::DisabledValue>,
    #[xvalue(default)]
    pub button: ::std::vec::Vec<button_items::Button>,
}
pub mod caption_items {
    impl ::core::convert::From<crate::xhtml::groups::Inline> for Inline {
        fn from(value: crate::xhtml::groups::Inline) -> Self {
            Inline::Inline(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml::groups::MiscInline> for Inline {
        fn from(value: crate::xhtml::groups::MiscInline) -> Self {
            Inline::MiscInline(::std::boxed::Box::new(value))
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub enum Inline {
        Inline(::std::boxed::Box<crate::xhtml::groups::Inline>),
        MiscInline(::std::boxed::Box<crate::xhtml::groups::MiscInline>),
    }
    #[derive(
        ::core::fmt::Debug,
        ::core::clone::Clone,
        ::core::marker::Copy,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq
    )]
    #[xvalue(with = dir_value_with)]
    pub enum DirValue {
        Ltr,
        Rtl,
    }
    pub mod dir_value_with {
        pub fn deserialize<'de, D>(
            deserializer: D,
        ) -> ::core::result::Result<super::DirValue, D::Error>
        where
            D: ::xmlity::Deserializer<'de>,
        {
            let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                deserializer,
            )?;
            let value: ::std::string::String = text
                .parse()
                .map_err(::xmlity::de::Error::custom)?;
            super::DirValue::try_from(value).map_err(::xmlity::de::Error::custom)
        }
        pub fn serialize<S>(
            value: &super::DirValue,
            serializer: S,
        ) -> ::core::result::Result<S::Ok, S::Error>
        where
            S: ::xmlity::Serializer,
        {
            let value: ::std::string::String = ::core::clone::Clone::clone(value).into();
            ::xmlity::Serialize::serialize(
                ::std::string::String::as_str(
                    &::std::string::ToString::to_string(&value),
                ),
                serializer,
            )
        }
    }
    #[derive(::core::fmt::Debug)]
    pub enum DirValueParseError {
        NonExistent { value: ::std::string::String },
    }
    impl ::core::fmt::Display for DirValueParseError {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::result::Result<(), ::core::fmt::Error> {
            match self {
                DirValueParseError::NonExistent { value } => {
                    write!(f, "Value '{:?}' does not exist in the enumeration", value)
                }
            }
        }
    }
    impl ::core::convert::TryFrom<::std::string::String> for DirValue {
        type Error = DirValueParseError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::core::result::Result<Self, Self::Error> {
            match ::std::string::String::as_str(&value) {
                "ltr" => Ok(DirValue::Ltr),
                "rtl" => Ok(DirValue::Rtl),
                _ => {
                    Err(DirValueParseError::NonExistent {
                        value,
                    })
                }
            }
        }
    }
    impl ::core::convert::From<DirValue> for ::std::string::String {
        fn from(value: DirValue) -> Self {
            match value {
                DirValue::Ltr => ::std::string::String::from("ltr"),
                DirValue::Rtl => ::std::string::String::from("rtl"),
            }
        }
    }
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
#[xelement(
    name = "caption",
    namespace = "http://www.w3.org/1999/xhtml",
    allow_unknown_attributes = "any"
)]
pub struct Caption {
    #[xattribute(name = "id", optional, default)]
    pub id: ::core::option::Option<String>,
    #[xattribute(name = "class", optional, default)]
    pub class: ::core::option::Option<String>,
    #[xattribute(name = "style", optional, default)]
    pub style: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::StyleSheet>,
    >,
    #[xattribute(name = "title", optional, default)]
    pub title: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Text>>,
    #[xattribute(name = "lang", optional, default)]
    pub lang: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::LanguageCode>,
    >,
    #[xattribute(deferred = true, optional, default)]
    pub attribute_5: ::core::option::Option<crate::xml::Lang>,
    #[xattribute(name = "dir", optional, default)]
    pub dir: ::core::option::Option<caption_items::DirValue>,
    #[xattribute(name = "onclick", optional, default)]
    pub onclick: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xattribute(name = "ondblclick", optional, default)]
    pub ondblclick: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmousedown", optional, default)]
    pub onmousedown: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseup", optional, default)]
    pub onmouseup: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseover", optional, default)]
    pub onmouseover: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmousemove", optional, default)]
    pub onmousemove: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseout", optional, default)]
    pub onmouseout: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeypress", optional, default)]
    pub onkeypress: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeydown", optional, default)]
    pub onkeydown: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeyup", optional, default)]
    pub onkeyup: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xvalue(default)]
    pub inline: ::std::vec::Vec<caption_items::Inline>,
}
pub mod cite_items {
    impl ::core::convert::From<crate::xhtml::groups::Inline> for Inline {
        fn from(value: crate::xhtml::groups::Inline) -> Self {
            Inline::Inline(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml::groups::MiscInline> for Inline {
        fn from(value: crate::xhtml::groups::MiscInline) -> Self {
            Inline::MiscInline(::std::boxed::Box::new(value))
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub enum Inline {
        Inline(::std::boxed::Box<crate::xhtml::groups::Inline>),
        MiscInline(::std::boxed::Box<crate::xhtml::groups::MiscInline>),
    }
    #[derive(
        ::core::fmt::Debug,
        ::core::clone::Clone,
        ::core::marker::Copy,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq
    )]
    #[xvalue(with = dir_value_with)]
    pub enum DirValue {
        Ltr,
        Rtl,
    }
    pub mod dir_value_with {
        pub fn deserialize<'de, D>(
            deserializer: D,
        ) -> ::core::result::Result<super::DirValue, D::Error>
        where
            D: ::xmlity::Deserializer<'de>,
        {
            let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                deserializer,
            )?;
            let value: ::std::string::String = text
                .parse()
                .map_err(::xmlity::de::Error::custom)?;
            super::DirValue::try_from(value).map_err(::xmlity::de::Error::custom)
        }
        pub fn serialize<S>(
            value: &super::DirValue,
            serializer: S,
        ) -> ::core::result::Result<S::Ok, S::Error>
        where
            S: ::xmlity::Serializer,
        {
            let value: ::std::string::String = ::core::clone::Clone::clone(value).into();
            ::xmlity::Serialize::serialize(
                ::std::string::String::as_str(
                    &::std::string::ToString::to_string(&value),
                ),
                serializer,
            )
        }
    }
    #[derive(::core::fmt::Debug)]
    pub enum DirValueParseError {
        NonExistent { value: ::std::string::String },
    }
    impl ::core::fmt::Display for DirValueParseError {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::result::Result<(), ::core::fmt::Error> {
            match self {
                DirValueParseError::NonExistent { value } => {
                    write!(f, "Value '{:?}' does not exist in the enumeration", value)
                }
            }
        }
    }
    impl ::core::convert::TryFrom<::std::string::String> for DirValue {
        type Error = DirValueParseError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::core::result::Result<Self, Self::Error> {
            match ::std::string::String::as_str(&value) {
                "ltr" => Ok(DirValue::Ltr),
                "rtl" => Ok(DirValue::Rtl),
                _ => {
                    Err(DirValueParseError::NonExistent {
                        value,
                    })
                }
            }
        }
    }
    impl ::core::convert::From<DirValue> for ::std::string::String {
        fn from(value: DirValue) -> Self {
            match value {
                DirValue::Ltr => ::std::string::String::from("ltr"),
                DirValue::Rtl => ::std::string::String::from("rtl"),
            }
        }
    }
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
#[xelement(
    name = "cite",
    namespace = "http://www.w3.org/1999/xhtml",
    allow_unknown_attributes = "any"
)]
pub struct Cite {
    #[xattribute(name = "id", optional, default)]
    pub id: ::core::option::Option<String>,
    #[xattribute(name = "class", optional, default)]
    pub class: ::core::option::Option<String>,
    #[xattribute(name = "style", optional, default)]
    pub style: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::StyleSheet>,
    >,
    #[xattribute(name = "title", optional, default)]
    pub title: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Text>>,
    #[xattribute(name = "lang", optional, default)]
    pub lang: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::LanguageCode>,
    >,
    #[xattribute(deferred = true, optional, default)]
    pub attribute_5: ::core::option::Option<crate::xml::Lang>,
    #[xattribute(name = "dir", optional, default)]
    pub dir: ::core::option::Option<cite_items::DirValue>,
    #[xattribute(name = "onclick", optional, default)]
    pub onclick: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xattribute(name = "ondblclick", optional, default)]
    pub ondblclick: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmousedown", optional, default)]
    pub onmousedown: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseup", optional, default)]
    pub onmouseup: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseover", optional, default)]
    pub onmouseover: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmousemove", optional, default)]
    pub onmousemove: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseout", optional, default)]
    pub onmouseout: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeypress", optional, default)]
    pub onkeypress: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeydown", optional, default)]
    pub onkeydown: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeyup", optional, default)]
    pub onkeyup: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xvalue(default)]
    pub inline: ::std::vec::Vec<cite_items::Inline>,
}
pub mod code_items {
    impl ::core::convert::From<crate::xhtml::groups::Inline> for Inline {
        fn from(value: crate::xhtml::groups::Inline) -> Self {
            Inline::Inline(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml::groups::MiscInline> for Inline {
        fn from(value: crate::xhtml::groups::MiscInline) -> Self {
            Inline::MiscInline(::std::boxed::Box::new(value))
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub enum Inline {
        Inline(::std::boxed::Box<crate::xhtml::groups::Inline>),
        MiscInline(::std::boxed::Box<crate::xhtml::groups::MiscInline>),
    }
    #[derive(
        ::core::fmt::Debug,
        ::core::clone::Clone,
        ::core::marker::Copy,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq
    )]
    #[xvalue(with = dir_value_with)]
    pub enum DirValue {
        Ltr,
        Rtl,
    }
    pub mod dir_value_with {
        pub fn deserialize<'de, D>(
            deserializer: D,
        ) -> ::core::result::Result<super::DirValue, D::Error>
        where
            D: ::xmlity::Deserializer<'de>,
        {
            let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                deserializer,
            )?;
            let value: ::std::string::String = text
                .parse()
                .map_err(::xmlity::de::Error::custom)?;
            super::DirValue::try_from(value).map_err(::xmlity::de::Error::custom)
        }
        pub fn serialize<S>(
            value: &super::DirValue,
            serializer: S,
        ) -> ::core::result::Result<S::Ok, S::Error>
        where
            S: ::xmlity::Serializer,
        {
            let value: ::std::string::String = ::core::clone::Clone::clone(value).into();
            ::xmlity::Serialize::serialize(
                ::std::string::String::as_str(
                    &::std::string::ToString::to_string(&value),
                ),
                serializer,
            )
        }
    }
    #[derive(::core::fmt::Debug)]
    pub enum DirValueParseError {
        NonExistent { value: ::std::string::String },
    }
    impl ::core::fmt::Display for DirValueParseError {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::result::Result<(), ::core::fmt::Error> {
            match self {
                DirValueParseError::NonExistent { value } => {
                    write!(f, "Value '{:?}' does not exist in the enumeration", value)
                }
            }
        }
    }
    impl ::core::convert::TryFrom<::std::string::String> for DirValue {
        type Error = DirValueParseError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::core::result::Result<Self, Self::Error> {
            match ::std::string::String::as_str(&value) {
                "ltr" => Ok(DirValue::Ltr),
                "rtl" => Ok(DirValue::Rtl),
                _ => {
                    Err(DirValueParseError::NonExistent {
                        value,
                    })
                }
            }
        }
    }
    impl ::core::convert::From<DirValue> for ::std::string::String {
        fn from(value: DirValue) -> Self {
            match value {
                DirValue::Ltr => ::std::string::String::from("ltr"),
                DirValue::Rtl => ::std::string::String::from("rtl"),
            }
        }
    }
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
#[xelement(
    name = "code",
    namespace = "http://www.w3.org/1999/xhtml",
    allow_unknown_attributes = "any"
)]
pub struct Code {
    #[xattribute(name = "id", optional, default)]
    pub id: ::core::option::Option<String>,
    #[xattribute(name = "class", optional, default)]
    pub class: ::core::option::Option<String>,
    #[xattribute(name = "style", optional, default)]
    pub style: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::StyleSheet>,
    >,
    #[xattribute(name = "title", optional, default)]
    pub title: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Text>>,
    #[xattribute(name = "lang", optional, default)]
    pub lang: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::LanguageCode>,
    >,
    #[xattribute(deferred = true, optional, default)]
    pub attribute_5: ::core::option::Option<crate::xml::Lang>,
    #[xattribute(name = "dir", optional, default)]
    pub dir: ::core::option::Option<code_items::DirValue>,
    #[xattribute(name = "onclick", optional, default)]
    pub onclick: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xattribute(name = "ondblclick", optional, default)]
    pub ondblclick: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmousedown", optional, default)]
    pub onmousedown: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseup", optional, default)]
    pub onmouseup: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseover", optional, default)]
    pub onmouseover: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmousemove", optional, default)]
    pub onmousemove: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseout", optional, default)]
    pub onmouseout: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeypress", optional, default)]
    pub onkeypress: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeydown", optional, default)]
    pub onkeydown: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeyup", optional, default)]
    pub onkeyup: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xvalue(default)]
    pub inline: ::std::vec::Vec<code_items::Inline>,
}
pub mod col_items {
    #[derive(
        ::core::fmt::Debug,
        ::core::clone::Clone,
        ::core::marker::Copy,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq
    )]
    #[xvalue(with = dir_value_with)]
    pub enum DirValue {
        Ltr,
        Rtl,
    }
    pub mod dir_value_with {
        pub fn deserialize<'de, D>(
            deserializer: D,
        ) -> ::core::result::Result<super::DirValue, D::Error>
        where
            D: ::xmlity::Deserializer<'de>,
        {
            let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                deserializer,
            )?;
            let value: ::std::string::String = text
                .parse()
                .map_err(::xmlity::de::Error::custom)?;
            super::DirValue::try_from(value).map_err(::xmlity::de::Error::custom)
        }
        pub fn serialize<S>(
            value: &super::DirValue,
            serializer: S,
        ) -> ::core::result::Result<S::Ok, S::Error>
        where
            S: ::xmlity::Serializer,
        {
            let value: ::std::string::String = ::core::clone::Clone::clone(value).into();
            ::xmlity::Serialize::serialize(
                ::std::string::String::as_str(
                    &::std::string::ToString::to_string(&value),
                ),
                serializer,
            )
        }
    }
    #[derive(::core::fmt::Debug)]
    pub enum DirValueParseError {
        NonExistent { value: ::std::string::String },
    }
    impl ::core::fmt::Display for DirValueParseError {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::result::Result<(), ::core::fmt::Error> {
            match self {
                DirValueParseError::NonExistent { value } => {
                    write!(f, "Value '{:?}' does not exist in the enumeration", value)
                }
            }
        }
    }
    impl ::core::convert::TryFrom<::std::string::String> for DirValue {
        type Error = DirValueParseError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::core::result::Result<Self, Self::Error> {
            match ::std::string::String::as_str(&value) {
                "ltr" => Ok(DirValue::Ltr),
                "rtl" => Ok(DirValue::Rtl),
                _ => {
                    Err(DirValueParseError::NonExistent {
                        value,
                    })
                }
            }
        }
    }
    impl ::core::convert::From<DirValue> for ::std::string::String {
        fn from(value: DirValue) -> Self {
            match value {
                DirValue::Ltr => ::std::string::String::from("ltr"),
                DirValue::Rtl => ::std::string::String::from("rtl"),
            }
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::core::clone::Clone,
        ::core::marker::Copy,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq
    )]
    #[xvalue(with = align_value_with)]
    pub enum AlignValue {
        Left,
        Center,
        Right,
        Justify,
        Char,
    }
    pub mod align_value_with {
        pub fn deserialize<'de, D>(
            deserializer: D,
        ) -> ::core::result::Result<super::AlignValue, D::Error>
        where
            D: ::xmlity::Deserializer<'de>,
        {
            let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                deserializer,
            )?;
            let value: ::std::string::String = text
                .parse()
                .map_err(::xmlity::de::Error::custom)?;
            super::AlignValue::try_from(value).map_err(::xmlity::de::Error::custom)
        }
        pub fn serialize<S>(
            value: &super::AlignValue,
            serializer: S,
        ) -> ::core::result::Result<S::Ok, S::Error>
        where
            S: ::xmlity::Serializer,
        {
            let value: ::std::string::String = ::core::clone::Clone::clone(value).into();
            ::xmlity::Serialize::serialize(
                ::std::string::String::as_str(
                    &::std::string::ToString::to_string(&value),
                ),
                serializer,
            )
        }
    }
    #[derive(::core::fmt::Debug)]
    pub enum AlignValueParseError {
        NonExistent { value: ::std::string::String },
    }
    impl ::core::fmt::Display for AlignValueParseError {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::result::Result<(), ::core::fmt::Error> {
            match self {
                AlignValueParseError::NonExistent { value } => {
                    write!(f, "Value '{:?}' does not exist in the enumeration", value)
                }
            }
        }
    }
    impl ::core::convert::TryFrom<::std::string::String> for AlignValue {
        type Error = AlignValueParseError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::core::result::Result<Self, Self::Error> {
            match ::std::string::String::as_str(&value) {
                "left" => Ok(AlignValue::Left),
                "center" => Ok(AlignValue::Center),
                "right" => Ok(AlignValue::Right),
                "justify" => Ok(AlignValue::Justify),
                "char" => Ok(AlignValue::Char),
                _ => {
                    Err(AlignValueParseError::NonExistent {
                        value,
                    })
                }
            }
        }
    }
    impl ::core::convert::From<AlignValue> for ::std::string::String {
        fn from(value: AlignValue) -> Self {
            match value {
                AlignValue::Left => ::std::string::String::from("left"),
                AlignValue::Center => ::std::string::String::from("center"),
                AlignValue::Right => ::std::string::String::from("right"),
                AlignValue::Justify => ::std::string::String::from("justify"),
                AlignValue::Char => ::std::string::String::from("char"),
            }
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::core::clone::Clone,
        ::core::marker::Copy,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq
    )]
    #[xvalue(with = valign_value_with)]
    pub enum ValignValue {
        Top,
        Middle,
        Bottom,
        Baseline,
    }
    pub mod valign_value_with {
        pub fn deserialize<'de, D>(
            deserializer: D,
        ) -> ::core::result::Result<super::ValignValue, D::Error>
        where
            D: ::xmlity::Deserializer<'de>,
        {
            let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                deserializer,
            )?;
            let value: ::std::string::String = text
                .parse()
                .map_err(::xmlity::de::Error::custom)?;
            super::ValignValue::try_from(value).map_err(::xmlity::de::Error::custom)
        }
        pub fn serialize<S>(
            value: &super::ValignValue,
            serializer: S,
        ) -> ::core::result::Result<S::Ok, S::Error>
        where
            S: ::xmlity::Serializer,
        {
            let value: ::std::string::String = ::core::clone::Clone::clone(value).into();
            ::xmlity::Serialize::serialize(
                ::std::string::String::as_str(
                    &::std::string::ToString::to_string(&value),
                ),
                serializer,
            )
        }
    }
    #[derive(::core::fmt::Debug)]
    pub enum ValignValueParseError {
        NonExistent { value: ::std::string::String },
    }
    impl ::core::fmt::Display for ValignValueParseError {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::result::Result<(), ::core::fmt::Error> {
            match self {
                ValignValueParseError::NonExistent { value } => {
                    write!(f, "Value '{:?}' does not exist in the enumeration", value)
                }
            }
        }
    }
    impl ::core::convert::TryFrom<::std::string::String> for ValignValue {
        type Error = ValignValueParseError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::core::result::Result<Self, Self::Error> {
            match ::std::string::String::as_str(&value) {
                "top" => Ok(ValignValue::Top),
                "middle" => Ok(ValignValue::Middle),
                "bottom" => Ok(ValignValue::Bottom),
                "baseline" => Ok(ValignValue::Baseline),
                _ => {
                    Err(ValignValueParseError::NonExistent {
                        value,
                    })
                }
            }
        }
    }
    impl ::core::convert::From<ValignValue> for ::std::string::String {
        fn from(value: ValignValue) -> Self {
            match value {
                ValignValue::Top => ::std::string::String::from("top"),
                ValignValue::Middle => ::std::string::String::from("middle"),
                ValignValue::Bottom => ::std::string::String::from("bottom"),
                ValignValue::Baseline => ::std::string::String::from("baseline"),
            }
        }
    }
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
#[xelement(
    name = "col",
    namespace = "http://www.w3.org/1999/xhtml",
    allow_unknown_attributes = "any"
)]
pub struct Col {
    #[xattribute(name = "id", optional, default)]
    pub id: ::core::option::Option<String>,
    #[xattribute(name = "class", optional, default)]
    pub class: ::core::option::Option<String>,
    #[xattribute(name = "style", optional, default)]
    pub style: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::StyleSheet>,
    >,
    #[xattribute(name = "title", optional, default)]
    pub title: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Text>>,
    #[xattribute(name = "lang", optional, default)]
    pub lang: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::LanguageCode>,
    >,
    #[xattribute(deferred = true, optional, default)]
    pub attribute_5: ::core::option::Option<crate::xml::Lang>,
    #[xattribute(name = "dir", optional, default)]
    pub dir: ::core::option::Option<col_items::DirValue>,
    #[xattribute(name = "onclick", optional, default)]
    pub onclick: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xattribute(name = "ondblclick", optional, default)]
    pub ondblclick: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmousedown", optional, default)]
    pub onmousedown: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseup", optional, default)]
    pub onmouseup: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseover", optional, default)]
    pub onmouseover: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmousemove", optional, default)]
    pub onmousemove: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseout", optional, default)]
    pub onmouseout: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeypress", optional, default)]
    pub onkeypress: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeydown", optional, default)]
    pub onkeydown: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeyup", optional, default)]
    pub onkeyup: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xattribute(name = "span", optional, default)]
    pub span: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Number>>,
    #[xattribute(name = "width", optional, default)]
    pub width: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::MultiLength>,
    >,
    #[xattribute(name = "align", optional, default)]
    pub align: ::core::option::Option<col_items::AlignValue>,
    #[xattribute(name = "char", optional, default)]
    pub char: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Character>>,
    #[xattribute(name = "charoff", optional, default)]
    pub charoff: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Length>>,
    #[xattribute(name = "valign", optional, default)]
    pub valign: ::core::option::Option<col_items::ValignValue>,
}
pub mod colgroup_items {
    #[derive(
        ::core::fmt::Debug,
        ::core::clone::Clone,
        ::core::marker::Copy,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq
    )]
    #[xvalue(with = dir_value_with)]
    pub enum DirValue {
        Ltr,
        Rtl,
    }
    pub mod dir_value_with {
        pub fn deserialize<'de, D>(
            deserializer: D,
        ) -> ::core::result::Result<super::DirValue, D::Error>
        where
            D: ::xmlity::Deserializer<'de>,
        {
            let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                deserializer,
            )?;
            let value: ::std::string::String = text
                .parse()
                .map_err(::xmlity::de::Error::custom)?;
            super::DirValue::try_from(value).map_err(::xmlity::de::Error::custom)
        }
        pub fn serialize<S>(
            value: &super::DirValue,
            serializer: S,
        ) -> ::core::result::Result<S::Ok, S::Error>
        where
            S: ::xmlity::Serializer,
        {
            let value: ::std::string::String = ::core::clone::Clone::clone(value).into();
            ::xmlity::Serialize::serialize(
                ::std::string::String::as_str(
                    &::std::string::ToString::to_string(&value),
                ),
                serializer,
            )
        }
    }
    #[derive(::core::fmt::Debug)]
    pub enum DirValueParseError {
        NonExistent { value: ::std::string::String },
    }
    impl ::core::fmt::Display for DirValueParseError {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::result::Result<(), ::core::fmt::Error> {
            match self {
                DirValueParseError::NonExistent { value } => {
                    write!(f, "Value '{:?}' does not exist in the enumeration", value)
                }
            }
        }
    }
    impl ::core::convert::TryFrom<::std::string::String> for DirValue {
        type Error = DirValueParseError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::core::result::Result<Self, Self::Error> {
            match ::std::string::String::as_str(&value) {
                "ltr" => Ok(DirValue::Ltr),
                "rtl" => Ok(DirValue::Rtl),
                _ => {
                    Err(DirValueParseError::NonExistent {
                        value,
                    })
                }
            }
        }
    }
    impl ::core::convert::From<DirValue> for ::std::string::String {
        fn from(value: DirValue) -> Self {
            match value {
                DirValue::Ltr => ::std::string::String::from("ltr"),
                DirValue::Rtl => ::std::string::String::from("rtl"),
            }
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::core::clone::Clone,
        ::core::marker::Copy,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq
    )]
    #[xvalue(with = align_value_with)]
    pub enum AlignValue {
        Left,
        Center,
        Right,
        Justify,
        Char,
    }
    pub mod align_value_with {
        pub fn deserialize<'de, D>(
            deserializer: D,
        ) -> ::core::result::Result<super::AlignValue, D::Error>
        where
            D: ::xmlity::Deserializer<'de>,
        {
            let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                deserializer,
            )?;
            let value: ::std::string::String = text
                .parse()
                .map_err(::xmlity::de::Error::custom)?;
            super::AlignValue::try_from(value).map_err(::xmlity::de::Error::custom)
        }
        pub fn serialize<S>(
            value: &super::AlignValue,
            serializer: S,
        ) -> ::core::result::Result<S::Ok, S::Error>
        where
            S: ::xmlity::Serializer,
        {
            let value: ::std::string::String = ::core::clone::Clone::clone(value).into();
            ::xmlity::Serialize::serialize(
                ::std::string::String::as_str(
                    &::std::string::ToString::to_string(&value),
                ),
                serializer,
            )
        }
    }
    #[derive(::core::fmt::Debug)]
    pub enum AlignValueParseError {
        NonExistent { value: ::std::string::String },
    }
    impl ::core::fmt::Display for AlignValueParseError {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::result::Result<(), ::core::fmt::Error> {
            match self {
                AlignValueParseError::NonExistent { value } => {
                    write!(f, "Value '{:?}' does not exist in the enumeration", value)
                }
            }
        }
    }
    impl ::core::convert::TryFrom<::std::string::String> for AlignValue {
        type Error = AlignValueParseError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::core::result::Result<Self, Self::Error> {
            match ::std::string::String::as_str(&value) {
                "left" => Ok(AlignValue::Left),
                "center" => Ok(AlignValue::Center),
                "right" => Ok(AlignValue::Right),
                "justify" => Ok(AlignValue::Justify),
                "char" => Ok(AlignValue::Char),
                _ => {
                    Err(AlignValueParseError::NonExistent {
                        value,
                    })
                }
            }
        }
    }
    impl ::core::convert::From<AlignValue> for ::std::string::String {
        fn from(value: AlignValue) -> Self {
            match value {
                AlignValue::Left => ::std::string::String::from("left"),
                AlignValue::Center => ::std::string::String::from("center"),
                AlignValue::Right => ::std::string::String::from("right"),
                AlignValue::Justify => ::std::string::String::from("justify"),
                AlignValue::Char => ::std::string::String::from("char"),
            }
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::core::clone::Clone,
        ::core::marker::Copy,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq
    )]
    #[xvalue(with = valign_value_with)]
    pub enum ValignValue {
        Top,
        Middle,
        Bottom,
        Baseline,
    }
    pub mod valign_value_with {
        pub fn deserialize<'de, D>(
            deserializer: D,
        ) -> ::core::result::Result<super::ValignValue, D::Error>
        where
            D: ::xmlity::Deserializer<'de>,
        {
            let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                deserializer,
            )?;
            let value: ::std::string::String = text
                .parse()
                .map_err(::xmlity::de::Error::custom)?;
            super::ValignValue::try_from(value).map_err(::xmlity::de::Error::custom)
        }
        pub fn serialize<S>(
            value: &super::ValignValue,
            serializer: S,
        ) -> ::core::result::Result<S::Ok, S::Error>
        where
            S: ::xmlity::Serializer,
        {
            let value: ::std::string::String = ::core::clone::Clone::clone(value).into();
            ::xmlity::Serialize::serialize(
                ::std::string::String::as_str(
                    &::std::string::ToString::to_string(&value),
                ),
                serializer,
            )
        }
    }
    #[derive(::core::fmt::Debug)]
    pub enum ValignValueParseError {
        NonExistent { value: ::std::string::String },
    }
    impl ::core::fmt::Display for ValignValueParseError {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::result::Result<(), ::core::fmt::Error> {
            match self {
                ValignValueParseError::NonExistent { value } => {
                    write!(f, "Value '{:?}' does not exist in the enumeration", value)
                }
            }
        }
    }
    impl ::core::convert::TryFrom<::std::string::String> for ValignValue {
        type Error = ValignValueParseError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::core::result::Result<Self, Self::Error> {
            match ::std::string::String::as_str(&value) {
                "top" => Ok(ValignValue::Top),
                "middle" => Ok(ValignValue::Middle),
                "bottom" => Ok(ValignValue::Bottom),
                "baseline" => Ok(ValignValue::Baseline),
                _ => {
                    Err(ValignValueParseError::NonExistent {
                        value,
                    })
                }
            }
        }
    }
    impl ::core::convert::From<ValignValue> for ::std::string::String {
        fn from(value: ValignValue) -> Self {
            match value {
                ValignValue::Top => ::std::string::String::from("top"),
                ValignValue::Middle => ::std::string::String::from("middle"),
                ValignValue::Bottom => ::std::string::String::from("bottom"),
                ValignValue::Baseline => ::std::string::String::from("baseline"),
            }
        }
    }
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
#[xelement(
    name = "colgroup",
    namespace = "http://www.w3.org/1999/xhtml",
    allow_unknown_attributes = "any",
    children_order = "strict"
)]
pub struct Colgroup {
    #[xattribute(name = "id", optional, default)]
    pub id: ::core::option::Option<String>,
    #[xattribute(name = "class", optional, default)]
    pub class: ::core::option::Option<String>,
    #[xattribute(name = "style", optional, default)]
    pub style: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::StyleSheet>,
    >,
    #[xattribute(name = "title", optional, default)]
    pub title: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Text>>,
    #[xattribute(name = "lang", optional, default)]
    pub lang: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::LanguageCode>,
    >,
    #[xattribute(deferred = true, optional, default)]
    pub attribute_5: ::core::option::Option<crate::xml::Lang>,
    #[xattribute(name = "dir", optional, default)]
    pub dir: ::core::option::Option<colgroup_items::DirValue>,
    #[xattribute(name = "onclick", optional, default)]
    pub onclick: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xattribute(name = "ondblclick", optional, default)]
    pub ondblclick: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmousedown", optional, default)]
    pub onmousedown: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseup", optional, default)]
    pub onmouseup: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseover", optional, default)]
    pub onmouseover: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmousemove", optional, default)]
    pub onmousemove: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseout", optional, default)]
    pub onmouseout: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeypress", optional, default)]
    pub onkeypress: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeydown", optional, default)]
    pub onkeydown: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeyup", optional, default)]
    pub onkeyup: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xattribute(name = "span", optional, default)]
    pub span: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Number>>,
    #[xattribute(name = "width", optional, default)]
    pub width: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::MultiLength>,
    >,
    #[xattribute(name = "align", optional, default)]
    pub align: ::core::option::Option<colgroup_items::AlignValue>,
    #[xattribute(name = "char", optional, default)]
    pub char: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Character>>,
    #[xattribute(name = "charoff", optional, default)]
    pub charoff: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Length>>,
    #[xattribute(name = "valign", optional, default)]
    pub valign: ::core::option::Option<colgroup_items::ValignValue>,
    #[xvalue(default)]
    pub col: ::std::vec::Vec<crate::xhtml::Col>,
}
pub mod dd_items {
    impl ::core::convert::From<crate::xhtml::groups::Block> for Dd {
        fn from(value: crate::xhtml::groups::Block) -> Self {
            Dd::Block(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml::Form> for Dd {
        fn from(value: crate::xhtml::Form) -> Self {
            Dd::Form(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml::groups::Inline> for Dd {
        fn from(value: crate::xhtml::groups::Inline) -> Self {
            Dd::Inline(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml::groups::Misc> for Dd {
        fn from(value: crate::xhtml::groups::Misc) -> Self {
            Dd::Misc(::std::boxed::Box::new(value))
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub enum Dd {
        Block(::std::boxed::Box<crate::xhtml::groups::Block>),
        Form(::std::boxed::Box<crate::xhtml::Form>),
        Inline(::std::boxed::Box<crate::xhtml::groups::Inline>),
        Misc(::std::boxed::Box<crate::xhtml::groups::Misc>),
    }
    #[derive(
        ::core::fmt::Debug,
        ::core::clone::Clone,
        ::core::marker::Copy,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq
    )]
    #[xvalue(with = dir_value_with)]
    pub enum DirValue {
        Ltr,
        Rtl,
    }
    pub mod dir_value_with {
        pub fn deserialize<'de, D>(
            deserializer: D,
        ) -> ::core::result::Result<super::DirValue, D::Error>
        where
            D: ::xmlity::Deserializer<'de>,
        {
            let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                deserializer,
            )?;
            let value: ::std::string::String = text
                .parse()
                .map_err(::xmlity::de::Error::custom)?;
            super::DirValue::try_from(value).map_err(::xmlity::de::Error::custom)
        }
        pub fn serialize<S>(
            value: &super::DirValue,
            serializer: S,
        ) -> ::core::result::Result<S::Ok, S::Error>
        where
            S: ::xmlity::Serializer,
        {
            let value: ::std::string::String = ::core::clone::Clone::clone(value).into();
            ::xmlity::Serialize::serialize(
                ::std::string::String::as_str(
                    &::std::string::ToString::to_string(&value),
                ),
                serializer,
            )
        }
    }
    #[derive(::core::fmt::Debug)]
    pub enum DirValueParseError {
        NonExistent { value: ::std::string::String },
    }
    impl ::core::fmt::Display for DirValueParseError {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::result::Result<(), ::core::fmt::Error> {
            match self {
                DirValueParseError::NonExistent { value } => {
                    write!(f, "Value '{:?}' does not exist in the enumeration", value)
                }
            }
        }
    }
    impl ::core::convert::TryFrom<::std::string::String> for DirValue {
        type Error = DirValueParseError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::core::result::Result<Self, Self::Error> {
            match ::std::string::String::as_str(&value) {
                "ltr" => Ok(DirValue::Ltr),
                "rtl" => Ok(DirValue::Rtl),
                _ => {
                    Err(DirValueParseError::NonExistent {
                        value,
                    })
                }
            }
        }
    }
    impl ::core::convert::From<DirValue> for ::std::string::String {
        fn from(value: DirValue) -> Self {
            match value {
                DirValue::Ltr => ::std::string::String::from("ltr"),
                DirValue::Rtl => ::std::string::String::from("rtl"),
            }
        }
    }
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
#[xelement(
    name = "dd",
    namespace = "http://www.w3.org/1999/xhtml",
    allow_unknown_attributes = "any"
)]
pub struct Dd {
    #[xattribute(name = "id", optional, default)]
    pub id: ::core::option::Option<String>,
    #[xattribute(name = "class", optional, default)]
    pub class: ::core::option::Option<String>,
    #[xattribute(name = "style", optional, default)]
    pub style: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::StyleSheet>,
    >,
    #[xattribute(name = "title", optional, default)]
    pub title: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Text>>,
    #[xattribute(name = "lang", optional, default)]
    pub lang: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::LanguageCode>,
    >,
    #[xattribute(deferred = true, optional, default)]
    pub attribute_5: ::core::option::Option<crate::xml::Lang>,
    #[xattribute(name = "dir", optional, default)]
    pub dir: ::core::option::Option<dd_items::DirValue>,
    #[xattribute(name = "onclick", optional, default)]
    pub onclick: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xattribute(name = "ondblclick", optional, default)]
    pub ondblclick: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmousedown", optional, default)]
    pub onmousedown: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseup", optional, default)]
    pub onmouseup: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseover", optional, default)]
    pub onmouseover: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmousemove", optional, default)]
    pub onmousemove: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseout", optional, default)]
    pub onmouseout: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeypress", optional, default)]
    pub onkeypress: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeydown", optional, default)]
    pub onkeydown: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeyup", optional, default)]
    pub onkeyup: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xvalue(default)]
    pub dd: ::std::vec::Vec<dd_items::Dd>,
}
pub mod del_items {
    impl ::core::convert::From<crate::xhtml::groups::Block> for Del {
        fn from(value: crate::xhtml::groups::Block) -> Self {
            Del::Block(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml::Form> for Del {
        fn from(value: crate::xhtml::Form) -> Self {
            Del::Form(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml::groups::Inline> for Del {
        fn from(value: crate::xhtml::groups::Inline) -> Self {
            Del::Inline(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml::groups::Misc> for Del {
        fn from(value: crate::xhtml::groups::Misc) -> Self {
            Del::Misc(::std::boxed::Box::new(value))
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub enum Del {
        Block(::std::boxed::Box<crate::xhtml::groups::Block>),
        Form(::std::boxed::Box<crate::xhtml::Form>),
        Inline(::std::boxed::Box<crate::xhtml::groups::Inline>),
        Misc(::std::boxed::Box<crate::xhtml::groups::Misc>),
    }
    #[derive(
        ::core::fmt::Debug,
        ::core::clone::Clone,
        ::core::marker::Copy,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq
    )]
    #[xvalue(with = dir_value_with)]
    pub enum DirValue {
        Ltr,
        Rtl,
    }
    pub mod dir_value_with {
        pub fn deserialize<'de, D>(
            deserializer: D,
        ) -> ::core::result::Result<super::DirValue, D::Error>
        where
            D: ::xmlity::Deserializer<'de>,
        {
            let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                deserializer,
            )?;
            let value: ::std::string::String = text
                .parse()
                .map_err(::xmlity::de::Error::custom)?;
            super::DirValue::try_from(value).map_err(::xmlity::de::Error::custom)
        }
        pub fn serialize<S>(
            value: &super::DirValue,
            serializer: S,
        ) -> ::core::result::Result<S::Ok, S::Error>
        where
            S: ::xmlity::Serializer,
        {
            let value: ::std::string::String = ::core::clone::Clone::clone(value).into();
            ::xmlity::Serialize::serialize(
                ::std::string::String::as_str(
                    &::std::string::ToString::to_string(&value),
                ),
                serializer,
            )
        }
    }
    #[derive(::core::fmt::Debug)]
    pub enum DirValueParseError {
        NonExistent { value: ::std::string::String },
    }
    impl ::core::fmt::Display for DirValueParseError {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::result::Result<(), ::core::fmt::Error> {
            match self {
                DirValueParseError::NonExistent { value } => {
                    write!(f, "Value '{:?}' does not exist in the enumeration", value)
                }
            }
        }
    }
    impl ::core::convert::TryFrom<::std::string::String> for DirValue {
        type Error = DirValueParseError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::core::result::Result<Self, Self::Error> {
            match ::std::string::String::as_str(&value) {
                "ltr" => Ok(DirValue::Ltr),
                "rtl" => Ok(DirValue::Rtl),
                _ => {
                    Err(DirValueParseError::NonExistent {
                        value,
                    })
                }
            }
        }
    }
    impl ::core::convert::From<DirValue> for ::std::string::String {
        fn from(value: DirValue) -> Self {
            match value {
                DirValue::Ltr => ::std::string::String::from("ltr"),
                DirValue::Rtl => ::std::string::String::from("rtl"),
            }
        }
    }
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
#[xelement(
    name = "del",
    namespace = "http://www.w3.org/1999/xhtml",
    allow_unknown_attributes = "any"
)]
pub struct Del {
    #[xattribute(name = "id", optional, default)]
    pub id: ::core::option::Option<String>,
    #[xattribute(name = "class", optional, default)]
    pub class: ::core::option::Option<String>,
    #[xattribute(name = "style", optional, default)]
    pub style: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::StyleSheet>,
    >,
    #[xattribute(name = "title", optional, default)]
    pub title: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Text>>,
    #[xattribute(name = "lang", optional, default)]
    pub lang: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::LanguageCode>,
    >,
    #[xattribute(deferred = true, optional, default)]
    pub attribute_5: ::core::option::Option<crate::xml::Lang>,
    #[xattribute(name = "dir", optional, default)]
    pub dir: ::core::option::Option<del_items::DirValue>,
    #[xattribute(name = "onclick", optional, default)]
    pub onclick: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xattribute(name = "ondblclick", optional, default)]
    pub ondblclick: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmousedown", optional, default)]
    pub onmousedown: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseup", optional, default)]
    pub onmouseup: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseover", optional, default)]
    pub onmouseover: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmousemove", optional, default)]
    pub onmousemove: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseout", optional, default)]
    pub onmouseout: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeypress", optional, default)]
    pub onkeypress: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeydown", optional, default)]
    pub onkeydown: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeyup", optional, default)]
    pub onkeyup: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xattribute(name = "cite", optional, default)]
    pub cite: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Uri>>,
    #[xattribute(name = "datetime", optional, default)]
    pub datetime: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Datetime>,
    >,
    #[xvalue(default)]
    pub del: ::std::vec::Vec<del_items::Del>,
}
pub mod dfn_items {
    impl ::core::convert::From<crate::xhtml::groups::Inline> for Inline {
        fn from(value: crate::xhtml::groups::Inline) -> Self {
            Inline::Inline(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml::groups::MiscInline> for Inline {
        fn from(value: crate::xhtml::groups::MiscInline) -> Self {
            Inline::MiscInline(::std::boxed::Box::new(value))
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub enum Inline {
        Inline(::std::boxed::Box<crate::xhtml::groups::Inline>),
        MiscInline(::std::boxed::Box<crate::xhtml::groups::MiscInline>),
    }
    #[derive(
        ::core::fmt::Debug,
        ::core::clone::Clone,
        ::core::marker::Copy,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq
    )]
    #[xvalue(with = dir_value_with)]
    pub enum DirValue {
        Ltr,
        Rtl,
    }
    pub mod dir_value_with {
        pub fn deserialize<'de, D>(
            deserializer: D,
        ) -> ::core::result::Result<super::DirValue, D::Error>
        where
            D: ::xmlity::Deserializer<'de>,
        {
            let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                deserializer,
            )?;
            let value: ::std::string::String = text
                .parse()
                .map_err(::xmlity::de::Error::custom)?;
            super::DirValue::try_from(value).map_err(::xmlity::de::Error::custom)
        }
        pub fn serialize<S>(
            value: &super::DirValue,
            serializer: S,
        ) -> ::core::result::Result<S::Ok, S::Error>
        where
            S: ::xmlity::Serializer,
        {
            let value: ::std::string::String = ::core::clone::Clone::clone(value).into();
            ::xmlity::Serialize::serialize(
                ::std::string::String::as_str(
                    &::std::string::ToString::to_string(&value),
                ),
                serializer,
            )
        }
    }
    #[derive(::core::fmt::Debug)]
    pub enum DirValueParseError {
        NonExistent { value: ::std::string::String },
    }
    impl ::core::fmt::Display for DirValueParseError {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::result::Result<(), ::core::fmt::Error> {
            match self {
                DirValueParseError::NonExistent { value } => {
                    write!(f, "Value '{:?}' does not exist in the enumeration", value)
                }
            }
        }
    }
    impl ::core::convert::TryFrom<::std::string::String> for DirValue {
        type Error = DirValueParseError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::core::result::Result<Self, Self::Error> {
            match ::std::string::String::as_str(&value) {
                "ltr" => Ok(DirValue::Ltr),
                "rtl" => Ok(DirValue::Rtl),
                _ => {
                    Err(DirValueParseError::NonExistent {
                        value,
                    })
                }
            }
        }
    }
    impl ::core::convert::From<DirValue> for ::std::string::String {
        fn from(value: DirValue) -> Self {
            match value {
                DirValue::Ltr => ::std::string::String::from("ltr"),
                DirValue::Rtl => ::std::string::String::from("rtl"),
            }
        }
    }
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
#[xelement(
    name = "dfn",
    namespace = "http://www.w3.org/1999/xhtml",
    allow_unknown_attributes = "any"
)]
pub struct Dfn {
    #[xattribute(name = "id", optional, default)]
    pub id: ::core::option::Option<String>,
    #[xattribute(name = "class", optional, default)]
    pub class: ::core::option::Option<String>,
    #[xattribute(name = "style", optional, default)]
    pub style: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::StyleSheet>,
    >,
    #[xattribute(name = "title", optional, default)]
    pub title: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Text>>,
    #[xattribute(name = "lang", optional, default)]
    pub lang: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::LanguageCode>,
    >,
    #[xattribute(deferred = true, optional, default)]
    pub attribute_5: ::core::option::Option<crate::xml::Lang>,
    #[xattribute(name = "dir", optional, default)]
    pub dir: ::core::option::Option<dfn_items::DirValue>,
    #[xattribute(name = "onclick", optional, default)]
    pub onclick: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xattribute(name = "ondblclick", optional, default)]
    pub ondblclick: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmousedown", optional, default)]
    pub onmousedown: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseup", optional, default)]
    pub onmouseup: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseover", optional, default)]
    pub onmouseover: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmousemove", optional, default)]
    pub onmousemove: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseout", optional, default)]
    pub onmouseout: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeypress", optional, default)]
    pub onkeypress: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeydown", optional, default)]
    pub onkeydown: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeyup", optional, default)]
    pub onkeyup: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xvalue(default)]
    pub inline: ::std::vec::Vec<dfn_items::Inline>,
}
pub mod div_items {
    impl ::core::convert::From<crate::xhtml::groups::Block> for Div {
        fn from(value: crate::xhtml::groups::Block) -> Self {
            Div::Block(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml::Form> for Div {
        fn from(value: crate::xhtml::Form) -> Self {
            Div::Form(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml::groups::Inline> for Div {
        fn from(value: crate::xhtml::groups::Inline) -> Self {
            Div::Inline(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml::groups::Misc> for Div {
        fn from(value: crate::xhtml::groups::Misc) -> Self {
            Div::Misc(::std::boxed::Box::new(value))
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub enum Div {
        Block(::std::boxed::Box<crate::xhtml::groups::Block>),
        Form(::std::boxed::Box<crate::xhtml::Form>),
        Inline(::std::boxed::Box<crate::xhtml::groups::Inline>),
        Misc(::std::boxed::Box<crate::xhtml::groups::Misc>),
    }
    #[derive(
        ::core::fmt::Debug,
        ::core::clone::Clone,
        ::core::marker::Copy,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq
    )]
    #[xvalue(with = dir_value_with)]
    pub enum DirValue {
        Ltr,
        Rtl,
    }
    pub mod dir_value_with {
        pub fn deserialize<'de, D>(
            deserializer: D,
        ) -> ::core::result::Result<super::DirValue, D::Error>
        where
            D: ::xmlity::Deserializer<'de>,
        {
            let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                deserializer,
            )?;
            let value: ::std::string::String = text
                .parse()
                .map_err(::xmlity::de::Error::custom)?;
            super::DirValue::try_from(value).map_err(::xmlity::de::Error::custom)
        }
        pub fn serialize<S>(
            value: &super::DirValue,
            serializer: S,
        ) -> ::core::result::Result<S::Ok, S::Error>
        where
            S: ::xmlity::Serializer,
        {
            let value: ::std::string::String = ::core::clone::Clone::clone(value).into();
            ::xmlity::Serialize::serialize(
                ::std::string::String::as_str(
                    &::std::string::ToString::to_string(&value),
                ),
                serializer,
            )
        }
    }
    #[derive(::core::fmt::Debug)]
    pub enum DirValueParseError {
        NonExistent { value: ::std::string::String },
    }
    impl ::core::fmt::Display for DirValueParseError {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::result::Result<(), ::core::fmt::Error> {
            match self {
                DirValueParseError::NonExistent { value } => {
                    write!(f, "Value '{:?}' does not exist in the enumeration", value)
                }
            }
        }
    }
    impl ::core::convert::TryFrom<::std::string::String> for DirValue {
        type Error = DirValueParseError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::core::result::Result<Self, Self::Error> {
            match ::std::string::String::as_str(&value) {
                "ltr" => Ok(DirValue::Ltr),
                "rtl" => Ok(DirValue::Rtl),
                _ => {
                    Err(DirValueParseError::NonExistent {
                        value,
                    })
                }
            }
        }
    }
    impl ::core::convert::From<DirValue> for ::std::string::String {
        fn from(value: DirValue) -> Self {
            match value {
                DirValue::Ltr => ::std::string::String::from("ltr"),
                DirValue::Rtl => ::std::string::String::from("rtl"),
            }
        }
    }
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
#[xelement(
    name = "div",
    namespace = "http://www.w3.org/1999/xhtml",
    allow_unknown_attributes = "any"
)]
pub struct Div {
    #[xattribute(name = "id", optional, default)]
    pub id: ::core::option::Option<String>,
    #[xattribute(name = "class", optional, default)]
    pub class: ::core::option::Option<String>,
    #[xattribute(name = "style", optional, default)]
    pub style: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::StyleSheet>,
    >,
    #[xattribute(name = "title", optional, default)]
    pub title: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Text>>,
    #[xattribute(name = "lang", optional, default)]
    pub lang: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::LanguageCode>,
    >,
    #[xattribute(deferred = true, optional, default)]
    pub attribute_5: ::core::option::Option<crate::xml::Lang>,
    #[xattribute(name = "dir", optional, default)]
    pub dir: ::core::option::Option<div_items::DirValue>,
    #[xattribute(name = "onclick", optional, default)]
    pub onclick: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xattribute(name = "ondblclick", optional, default)]
    pub ondblclick: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmousedown", optional, default)]
    pub onmousedown: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseup", optional, default)]
    pub onmouseup: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseover", optional, default)]
    pub onmouseover: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmousemove", optional, default)]
    pub onmousemove: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseout", optional, default)]
    pub onmouseout: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeypress", optional, default)]
    pub onkeypress: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeydown", optional, default)]
    pub onkeydown: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeyup", optional, default)]
    pub onkeyup: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xvalue(default)]
    pub div: ::std::vec::Vec<div_items::Div>,
}
pub mod dl_items {
    impl ::core::convert::From<crate::xhtml::Dt> for Dl {
        fn from(value: crate::xhtml::Dt) -> Self {
            Dl::Dt(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml::Dd> for Dl {
        fn from(value: crate::xhtml::Dd) -> Self {
            Dl::Dd(::std::boxed::Box::new(value))
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub enum Dl {
        Dt(::std::boxed::Box<crate::xhtml::Dt>),
        Dd(::std::boxed::Box<crate::xhtml::Dd>),
    }
    #[derive(
        ::core::fmt::Debug,
        ::core::clone::Clone,
        ::core::marker::Copy,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq
    )]
    #[xvalue(with = dir_value_with)]
    pub enum DirValue {
        Ltr,
        Rtl,
    }
    pub mod dir_value_with {
        pub fn deserialize<'de, D>(
            deserializer: D,
        ) -> ::core::result::Result<super::DirValue, D::Error>
        where
            D: ::xmlity::Deserializer<'de>,
        {
            let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                deserializer,
            )?;
            let value: ::std::string::String = text
                .parse()
                .map_err(::xmlity::de::Error::custom)?;
            super::DirValue::try_from(value).map_err(::xmlity::de::Error::custom)
        }
        pub fn serialize<S>(
            value: &super::DirValue,
            serializer: S,
        ) -> ::core::result::Result<S::Ok, S::Error>
        where
            S: ::xmlity::Serializer,
        {
            let value: ::std::string::String = ::core::clone::Clone::clone(value).into();
            ::xmlity::Serialize::serialize(
                ::std::string::String::as_str(
                    &::std::string::ToString::to_string(&value),
                ),
                serializer,
            )
        }
    }
    #[derive(::core::fmt::Debug)]
    pub enum DirValueParseError {
        NonExistent { value: ::std::string::String },
    }
    impl ::core::fmt::Display for DirValueParseError {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::result::Result<(), ::core::fmt::Error> {
            match self {
                DirValueParseError::NonExistent { value } => {
                    write!(f, "Value '{:?}' does not exist in the enumeration", value)
                }
            }
        }
    }
    impl ::core::convert::TryFrom<::std::string::String> for DirValue {
        type Error = DirValueParseError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::core::result::Result<Self, Self::Error> {
            match ::std::string::String::as_str(&value) {
                "ltr" => Ok(DirValue::Ltr),
                "rtl" => Ok(DirValue::Rtl),
                _ => {
                    Err(DirValueParseError::NonExistent {
                        value,
                    })
                }
            }
        }
    }
    impl ::core::convert::From<DirValue> for ::std::string::String {
        fn from(value: DirValue) -> Self {
            match value {
                DirValue::Ltr => ::std::string::String::from("ltr"),
                DirValue::Rtl => ::std::string::String::from("rtl"),
            }
        }
    }
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
#[xelement(
    name = "dl",
    namespace = "http://www.w3.org/1999/xhtml",
    allow_unknown_attributes = "any"
)]
pub struct Dl {
    #[xattribute(name = "id", optional, default)]
    pub id: ::core::option::Option<String>,
    #[xattribute(name = "class", optional, default)]
    pub class: ::core::option::Option<String>,
    #[xattribute(name = "style", optional, default)]
    pub style: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::StyleSheet>,
    >,
    #[xattribute(name = "title", optional, default)]
    pub title: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Text>>,
    #[xattribute(name = "lang", optional, default)]
    pub lang: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::LanguageCode>,
    >,
    #[xattribute(deferred = true, optional, default)]
    pub attribute_5: ::core::option::Option<crate::xml::Lang>,
    #[xattribute(name = "dir", optional, default)]
    pub dir: ::core::option::Option<dl_items::DirValue>,
    #[xattribute(name = "onclick", optional, default)]
    pub onclick: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xattribute(name = "ondblclick", optional, default)]
    pub ondblclick: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmousedown", optional, default)]
    pub onmousedown: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseup", optional, default)]
    pub onmouseup: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseover", optional, default)]
    pub onmouseover: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmousemove", optional, default)]
    pub onmousemove: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseout", optional, default)]
    pub onmouseout: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeypress", optional, default)]
    pub onkeypress: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeydown", optional, default)]
    pub onkeydown: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeyup", optional, default)]
    pub onkeyup: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xvalue(default)]
    pub dl: ::std::vec::Vec<dl_items::Dl>,
}
pub mod dt_items {
    impl ::core::convert::From<crate::xhtml::groups::Inline> for Inline {
        fn from(value: crate::xhtml::groups::Inline) -> Self {
            Inline::Inline(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml::groups::MiscInline> for Inline {
        fn from(value: crate::xhtml::groups::MiscInline) -> Self {
            Inline::MiscInline(::std::boxed::Box::new(value))
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub enum Inline {
        Inline(::std::boxed::Box<crate::xhtml::groups::Inline>),
        MiscInline(::std::boxed::Box<crate::xhtml::groups::MiscInline>),
    }
    #[derive(
        ::core::fmt::Debug,
        ::core::clone::Clone,
        ::core::marker::Copy,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq
    )]
    #[xvalue(with = dir_value_with)]
    pub enum DirValue {
        Ltr,
        Rtl,
    }
    pub mod dir_value_with {
        pub fn deserialize<'de, D>(
            deserializer: D,
        ) -> ::core::result::Result<super::DirValue, D::Error>
        where
            D: ::xmlity::Deserializer<'de>,
        {
            let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                deserializer,
            )?;
            let value: ::std::string::String = text
                .parse()
                .map_err(::xmlity::de::Error::custom)?;
            super::DirValue::try_from(value).map_err(::xmlity::de::Error::custom)
        }
        pub fn serialize<S>(
            value: &super::DirValue,
            serializer: S,
        ) -> ::core::result::Result<S::Ok, S::Error>
        where
            S: ::xmlity::Serializer,
        {
            let value: ::std::string::String = ::core::clone::Clone::clone(value).into();
            ::xmlity::Serialize::serialize(
                ::std::string::String::as_str(
                    &::std::string::ToString::to_string(&value),
                ),
                serializer,
            )
        }
    }
    #[derive(::core::fmt::Debug)]
    pub enum DirValueParseError {
        NonExistent { value: ::std::string::String },
    }
    impl ::core::fmt::Display for DirValueParseError {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::result::Result<(), ::core::fmt::Error> {
            match self {
                DirValueParseError::NonExistent { value } => {
                    write!(f, "Value '{:?}' does not exist in the enumeration", value)
                }
            }
        }
    }
    impl ::core::convert::TryFrom<::std::string::String> for DirValue {
        type Error = DirValueParseError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::core::result::Result<Self, Self::Error> {
            match ::std::string::String::as_str(&value) {
                "ltr" => Ok(DirValue::Ltr),
                "rtl" => Ok(DirValue::Rtl),
                _ => {
                    Err(DirValueParseError::NonExistent {
                        value,
                    })
                }
            }
        }
    }
    impl ::core::convert::From<DirValue> for ::std::string::String {
        fn from(value: DirValue) -> Self {
            match value {
                DirValue::Ltr => ::std::string::String::from("ltr"),
                DirValue::Rtl => ::std::string::String::from("rtl"),
            }
        }
    }
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
#[xelement(
    name = "dt",
    namespace = "http://www.w3.org/1999/xhtml",
    allow_unknown_attributes = "any"
)]
pub struct Dt {
    #[xattribute(name = "id", optional, default)]
    pub id: ::core::option::Option<String>,
    #[xattribute(name = "class", optional, default)]
    pub class: ::core::option::Option<String>,
    #[xattribute(name = "style", optional, default)]
    pub style: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::StyleSheet>,
    >,
    #[xattribute(name = "title", optional, default)]
    pub title: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Text>>,
    #[xattribute(name = "lang", optional, default)]
    pub lang: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::LanguageCode>,
    >,
    #[xattribute(deferred = true, optional, default)]
    pub attribute_5: ::core::option::Option<crate::xml::Lang>,
    #[xattribute(name = "dir", optional, default)]
    pub dir: ::core::option::Option<dt_items::DirValue>,
    #[xattribute(name = "onclick", optional, default)]
    pub onclick: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xattribute(name = "ondblclick", optional, default)]
    pub ondblclick: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmousedown", optional, default)]
    pub onmousedown: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseup", optional, default)]
    pub onmouseup: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseover", optional, default)]
    pub onmouseover: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmousemove", optional, default)]
    pub onmousemove: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseout", optional, default)]
    pub onmouseout: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeypress", optional, default)]
    pub onkeypress: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeydown", optional, default)]
    pub onkeydown: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeyup", optional, default)]
    pub onkeyup: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xvalue(default)]
    pub inline: ::std::vec::Vec<dt_items::Inline>,
}
pub mod em_items {
    impl ::core::convert::From<crate::xhtml::groups::Inline> for Inline {
        fn from(value: crate::xhtml::groups::Inline) -> Self {
            Inline::Inline(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml::groups::MiscInline> for Inline {
        fn from(value: crate::xhtml::groups::MiscInline) -> Self {
            Inline::MiscInline(::std::boxed::Box::new(value))
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub enum Inline {
        Inline(::std::boxed::Box<crate::xhtml::groups::Inline>),
        MiscInline(::std::boxed::Box<crate::xhtml::groups::MiscInline>),
    }
    #[derive(
        ::core::fmt::Debug,
        ::core::clone::Clone,
        ::core::marker::Copy,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq
    )]
    #[xvalue(with = dir_value_with)]
    pub enum DirValue {
        Ltr,
        Rtl,
    }
    pub mod dir_value_with {
        pub fn deserialize<'de, D>(
            deserializer: D,
        ) -> ::core::result::Result<super::DirValue, D::Error>
        where
            D: ::xmlity::Deserializer<'de>,
        {
            let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                deserializer,
            )?;
            let value: ::std::string::String = text
                .parse()
                .map_err(::xmlity::de::Error::custom)?;
            super::DirValue::try_from(value).map_err(::xmlity::de::Error::custom)
        }
        pub fn serialize<S>(
            value: &super::DirValue,
            serializer: S,
        ) -> ::core::result::Result<S::Ok, S::Error>
        where
            S: ::xmlity::Serializer,
        {
            let value: ::std::string::String = ::core::clone::Clone::clone(value).into();
            ::xmlity::Serialize::serialize(
                ::std::string::String::as_str(
                    &::std::string::ToString::to_string(&value),
                ),
                serializer,
            )
        }
    }
    #[derive(::core::fmt::Debug)]
    pub enum DirValueParseError {
        NonExistent { value: ::std::string::String },
    }
    impl ::core::fmt::Display for DirValueParseError {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::result::Result<(), ::core::fmt::Error> {
            match self {
                DirValueParseError::NonExistent { value } => {
                    write!(f, "Value '{:?}' does not exist in the enumeration", value)
                }
            }
        }
    }
    impl ::core::convert::TryFrom<::std::string::String> for DirValue {
        type Error = DirValueParseError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::core::result::Result<Self, Self::Error> {
            match ::std::string::String::as_str(&value) {
                "ltr" => Ok(DirValue::Ltr),
                "rtl" => Ok(DirValue::Rtl),
                _ => {
                    Err(DirValueParseError::NonExistent {
                        value,
                    })
                }
            }
        }
    }
    impl ::core::convert::From<DirValue> for ::std::string::String {
        fn from(value: DirValue) -> Self {
            match value {
                DirValue::Ltr => ::std::string::String::from("ltr"),
                DirValue::Rtl => ::std::string::String::from("rtl"),
            }
        }
    }
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
#[xelement(
    name = "em",
    namespace = "http://www.w3.org/1999/xhtml",
    allow_unknown_attributes = "any"
)]
pub struct Em {
    #[xattribute(name = "id", optional, default)]
    pub id: ::core::option::Option<String>,
    #[xattribute(name = "class", optional, default)]
    pub class: ::core::option::Option<String>,
    #[xattribute(name = "style", optional, default)]
    pub style: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::StyleSheet>,
    >,
    #[xattribute(name = "title", optional, default)]
    pub title: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Text>>,
    #[xattribute(name = "lang", optional, default)]
    pub lang: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::LanguageCode>,
    >,
    #[xattribute(deferred = true, optional, default)]
    pub attribute_5: ::core::option::Option<crate::xml::Lang>,
    #[xattribute(name = "dir", optional, default)]
    pub dir: ::core::option::Option<em_items::DirValue>,
    #[xattribute(name = "onclick", optional, default)]
    pub onclick: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xattribute(name = "ondblclick", optional, default)]
    pub ondblclick: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmousedown", optional, default)]
    pub onmousedown: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseup", optional, default)]
    pub onmouseup: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseover", optional, default)]
    pub onmouseover: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmousemove", optional, default)]
    pub onmousemove: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseout", optional, default)]
    pub onmouseout: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeypress", optional, default)]
    pub onkeypress: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeydown", optional, default)]
    pub onkeydown: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeyup", optional, default)]
    pub onkeyup: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xvalue(default)]
    pub inline: ::std::vec::Vec<em_items::Inline>,
}
pub mod fieldset_items {
    impl ::core::convert::From<crate::xhtml::groups::Block> for Child1 {
        fn from(value: crate::xhtml::groups::Block) -> Self {
            Child1::Block(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml::Form> for Child1 {
        fn from(value: crate::xhtml::Form) -> Self {
            Child1::Form(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml::groups::Inline> for Child1 {
        fn from(value: crate::xhtml::groups::Inline) -> Self {
            Child1::Inline(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml::groups::Misc> for Child1 {
        fn from(value: crate::xhtml::groups::Misc) -> Self {
            Child1::Misc(::std::boxed::Box::new(value))
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub enum Child1 {
        Block(::std::boxed::Box<crate::xhtml::groups::Block>),
        Form(::std::boxed::Box<crate::xhtml::Form>),
        Inline(::std::boxed::Box<crate::xhtml::groups::Inline>),
        Misc(::std::boxed::Box<crate::xhtml::groups::Misc>),
    }
    #[derive(
        ::core::fmt::Debug,
        ::core::clone::Clone,
        ::core::marker::Copy,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq
    )]
    #[xvalue(with = dir_value_with)]
    pub enum DirValue {
        Ltr,
        Rtl,
    }
    pub mod dir_value_with {
        pub fn deserialize<'de, D>(
            deserializer: D,
        ) -> ::core::result::Result<super::DirValue, D::Error>
        where
            D: ::xmlity::Deserializer<'de>,
        {
            let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                deserializer,
            )?;
            let value: ::std::string::String = text
                .parse()
                .map_err(::xmlity::de::Error::custom)?;
            super::DirValue::try_from(value).map_err(::xmlity::de::Error::custom)
        }
        pub fn serialize<S>(
            value: &super::DirValue,
            serializer: S,
        ) -> ::core::result::Result<S::Ok, S::Error>
        where
            S: ::xmlity::Serializer,
        {
            let value: ::std::string::String = ::core::clone::Clone::clone(value).into();
            ::xmlity::Serialize::serialize(
                ::std::string::String::as_str(
                    &::std::string::ToString::to_string(&value),
                ),
                serializer,
            )
        }
    }
    #[derive(::core::fmt::Debug)]
    pub enum DirValueParseError {
        NonExistent { value: ::std::string::String },
    }
    impl ::core::fmt::Display for DirValueParseError {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::result::Result<(), ::core::fmt::Error> {
            match self {
                DirValueParseError::NonExistent { value } => {
                    write!(f, "Value '{:?}' does not exist in the enumeration", value)
                }
            }
        }
    }
    impl ::core::convert::TryFrom<::std::string::String> for DirValue {
        type Error = DirValueParseError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::core::result::Result<Self, Self::Error> {
            match ::std::string::String::as_str(&value) {
                "ltr" => Ok(DirValue::Ltr),
                "rtl" => Ok(DirValue::Rtl),
                _ => {
                    Err(DirValueParseError::NonExistent {
                        value,
                    })
                }
            }
        }
    }
    impl ::core::convert::From<DirValue> for ::std::string::String {
        fn from(value: DirValue) -> Self {
            match value {
                DirValue::Ltr => ::std::string::String::from("ltr"),
                DirValue::Rtl => ::std::string::String::from("rtl"),
            }
        }
    }
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
#[xelement(
    name = "fieldset",
    namespace = "http://www.w3.org/1999/xhtml",
    allow_unknown_attributes = "any",
    children_order = "strict"
)]
pub struct Fieldset {
    #[xattribute(name = "id", optional, default)]
    pub id: ::core::option::Option<String>,
    #[xattribute(name = "class", optional, default)]
    pub class: ::core::option::Option<String>,
    #[xattribute(name = "style", optional, default)]
    pub style: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::StyleSheet>,
    >,
    #[xattribute(name = "title", optional, default)]
    pub title: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Text>>,
    #[xattribute(name = "lang", optional, default)]
    pub lang: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::LanguageCode>,
    >,
    #[xattribute(deferred = true, optional, default)]
    pub attribute_5: ::core::option::Option<crate::xml::Lang>,
    #[xattribute(name = "dir", optional, default)]
    pub dir: ::core::option::Option<fieldset_items::DirValue>,
    #[xattribute(name = "onclick", optional, default)]
    pub onclick: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xattribute(name = "ondblclick", optional, default)]
    pub ondblclick: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmousedown", optional, default)]
    pub onmousedown: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseup", optional, default)]
    pub onmouseup: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseover", optional, default)]
    pub onmouseover: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmousemove", optional, default)]
    pub onmousemove: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseout", optional, default)]
    pub onmouseout: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeypress", optional, default)]
    pub onkeypress: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeydown", optional, default)]
    pub onkeydown: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeyup", optional, default)]
    pub onkeyup: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    pub legend: ::std::boxed::Box<crate::xhtml::Legend>,
    #[xvalue(default)]
    pub child_1: ::std::vec::Vec<fieldset_items::Child1>,
}
pub mod form_items {
    impl ::core::convert::From<crate::xhtml::groups::Block> for Form {
        fn from(value: crate::xhtml::groups::Block) -> Self {
            Form::Block(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml::groups::Misc> for Form {
        fn from(value: crate::xhtml::groups::Misc) -> Self {
            Form::Misc(::std::boxed::Box::new(value))
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub enum Form {
        Block(::std::boxed::Box<crate::xhtml::groups::Block>),
        Misc(::std::boxed::Box<crate::xhtml::groups::Misc>),
    }
    #[derive(
        ::core::fmt::Debug,
        ::core::clone::Clone,
        ::core::marker::Copy,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq
    )]
    #[xvalue(with = dir_value_with)]
    pub enum DirValue {
        Ltr,
        Rtl,
    }
    pub mod dir_value_with {
        pub fn deserialize<'de, D>(
            deserializer: D,
        ) -> ::core::result::Result<super::DirValue, D::Error>
        where
            D: ::xmlity::Deserializer<'de>,
        {
            let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                deserializer,
            )?;
            let value: ::std::string::String = text
                .parse()
                .map_err(::xmlity::de::Error::custom)?;
            super::DirValue::try_from(value).map_err(::xmlity::de::Error::custom)
        }
        pub fn serialize<S>(
            value: &super::DirValue,
            serializer: S,
        ) -> ::core::result::Result<S::Ok, S::Error>
        where
            S: ::xmlity::Serializer,
        {
            let value: ::std::string::String = ::core::clone::Clone::clone(value).into();
            ::xmlity::Serialize::serialize(
                ::std::string::String::as_str(
                    &::std::string::ToString::to_string(&value),
                ),
                serializer,
            )
        }
    }
    #[derive(::core::fmt::Debug)]
    pub enum DirValueParseError {
        NonExistent { value: ::std::string::String },
    }
    impl ::core::fmt::Display for DirValueParseError {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::result::Result<(), ::core::fmt::Error> {
            match self {
                DirValueParseError::NonExistent { value } => {
                    write!(f, "Value '{:?}' does not exist in the enumeration", value)
                }
            }
        }
    }
    impl ::core::convert::TryFrom<::std::string::String> for DirValue {
        type Error = DirValueParseError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::core::result::Result<Self, Self::Error> {
            match ::std::string::String::as_str(&value) {
                "ltr" => Ok(DirValue::Ltr),
                "rtl" => Ok(DirValue::Rtl),
                _ => {
                    Err(DirValueParseError::NonExistent {
                        value,
                    })
                }
            }
        }
    }
    impl ::core::convert::From<DirValue> for ::std::string::String {
        fn from(value: DirValue) -> Self {
            match value {
                DirValue::Ltr => ::std::string::String::from("ltr"),
                DirValue::Rtl => ::std::string::String::from("rtl"),
            }
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::core::clone::Clone,
        ::core::marker::Copy,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq
    )]
    #[xvalue(with = method_value_with)]
    pub enum MethodValue {
        Get,
        Post,
    }
    pub mod method_value_with {
        pub fn deserialize<'de, D>(
            deserializer: D,
        ) -> ::core::result::Result<super::MethodValue, D::Error>
        where
            D: ::xmlity::Deserializer<'de>,
        {
            let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                deserializer,
            )?;
            let value: ::std::string::String = text
                .parse()
                .map_err(::xmlity::de::Error::custom)?;
            super::MethodValue::try_from(value).map_err(::xmlity::de::Error::custom)
        }
        pub fn serialize<S>(
            value: &super::MethodValue,
            serializer: S,
        ) -> ::core::result::Result<S::Ok, S::Error>
        where
            S: ::xmlity::Serializer,
        {
            let value: ::std::string::String = ::core::clone::Clone::clone(value).into();
            ::xmlity::Serialize::serialize(
                ::std::string::String::as_str(
                    &::std::string::ToString::to_string(&value),
                ),
                serializer,
            )
        }
    }
    #[derive(::core::fmt::Debug)]
    pub enum MethodValueParseError {
        NonExistent { value: ::std::string::String },
    }
    impl ::core::fmt::Display for MethodValueParseError {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::result::Result<(), ::core::fmt::Error> {
            match self {
                MethodValueParseError::NonExistent { value } => {
                    write!(f, "Value '{:?}' does not exist in the enumeration", value)
                }
            }
        }
    }
    impl ::core::convert::TryFrom<::std::string::String> for MethodValue {
        type Error = MethodValueParseError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::core::result::Result<Self, Self::Error> {
            match ::std::string::String::as_str(&value) {
                "get" => Ok(MethodValue::Get),
                "post" => Ok(MethodValue::Post),
                _ => {
                    Err(MethodValueParseError::NonExistent {
                        value,
                    })
                }
            }
        }
    }
    impl ::core::convert::From<MethodValue> for ::std::string::String {
        fn from(value: MethodValue) -> Self {
            match value {
                MethodValue::Get => ::std::string::String::from("get"),
                MethodValue::Post => ::std::string::String::from("post"),
            }
        }
    }
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
#[xelement(
    name = "form",
    namespace = "http://www.w3.org/1999/xhtml",
    allow_unknown_attributes = "any"
)]
pub struct Form {
    #[xattribute(name = "id", optional, default)]
    pub id: ::core::option::Option<String>,
    #[xattribute(name = "class", optional, default)]
    pub class: ::core::option::Option<String>,
    #[xattribute(name = "style", optional, default)]
    pub style: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::StyleSheet>,
    >,
    #[xattribute(name = "title", optional, default)]
    pub title: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Text>>,
    #[xattribute(name = "lang", optional, default)]
    pub lang: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::LanguageCode>,
    >,
    #[xattribute(deferred = true, optional, default)]
    pub attribute_5: ::core::option::Option<crate::xml::Lang>,
    #[xattribute(name = "dir", optional, default)]
    pub dir: ::core::option::Option<form_items::DirValue>,
    #[xattribute(name = "onclick", optional, default)]
    pub onclick: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xattribute(name = "ondblclick", optional, default)]
    pub ondblclick: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmousedown", optional, default)]
    pub onmousedown: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseup", optional, default)]
    pub onmouseup: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseover", optional, default)]
    pub onmouseover: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmousemove", optional, default)]
    pub onmousemove: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseout", optional, default)]
    pub onmouseout: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeypress", optional, default)]
    pub onkeypress: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeydown", optional, default)]
    pub onkeydown: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeyup", optional, default)]
    pub onkeyup: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xattribute(name = "action")]
    pub action: ::std::boxed::Box<crate::xhtml::types::Uri>,
    #[xattribute(name = "method", optional, default)]
    pub method: ::core::option::Option<form_items::MethodValue>,
    #[xattribute(name = "enctype", optional, default)]
    pub enctype: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::ContentType>,
    >,
    #[xattribute(name = "onsubmit", optional, default)]
    pub onsubmit: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xattribute(name = "onreset", optional, default)]
    pub onreset: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xattribute(name = "accept", optional, default)]
    pub accept: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::ContentTypes>,
    >,
    #[xattribute(name = "accept-charset", optional, default)]
    pub accept_charset: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Charsets>,
    >,
    #[xvalue(default)]
    pub form: ::std::vec::Vec<form_items::Form>,
}
pub mod h1_items {
    impl ::core::convert::From<crate::xhtml::groups::Inline> for Inline {
        fn from(value: crate::xhtml::groups::Inline) -> Self {
            Inline::Inline(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml::groups::MiscInline> for Inline {
        fn from(value: crate::xhtml::groups::MiscInline) -> Self {
            Inline::MiscInline(::std::boxed::Box::new(value))
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub enum Inline {
        Inline(::std::boxed::Box<crate::xhtml::groups::Inline>),
        MiscInline(::std::boxed::Box<crate::xhtml::groups::MiscInline>),
    }
    #[derive(
        ::core::fmt::Debug,
        ::core::clone::Clone,
        ::core::marker::Copy,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq
    )]
    #[xvalue(with = dir_value_with)]
    pub enum DirValue {
        Ltr,
        Rtl,
    }
    pub mod dir_value_with {
        pub fn deserialize<'de, D>(
            deserializer: D,
        ) -> ::core::result::Result<super::DirValue, D::Error>
        where
            D: ::xmlity::Deserializer<'de>,
        {
            let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                deserializer,
            )?;
            let value: ::std::string::String = text
                .parse()
                .map_err(::xmlity::de::Error::custom)?;
            super::DirValue::try_from(value).map_err(::xmlity::de::Error::custom)
        }
        pub fn serialize<S>(
            value: &super::DirValue,
            serializer: S,
        ) -> ::core::result::Result<S::Ok, S::Error>
        where
            S: ::xmlity::Serializer,
        {
            let value: ::std::string::String = ::core::clone::Clone::clone(value).into();
            ::xmlity::Serialize::serialize(
                ::std::string::String::as_str(
                    &::std::string::ToString::to_string(&value),
                ),
                serializer,
            )
        }
    }
    #[derive(::core::fmt::Debug)]
    pub enum DirValueParseError {
        NonExistent { value: ::std::string::String },
    }
    impl ::core::fmt::Display for DirValueParseError {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::result::Result<(), ::core::fmt::Error> {
            match self {
                DirValueParseError::NonExistent { value } => {
                    write!(f, "Value '{:?}' does not exist in the enumeration", value)
                }
            }
        }
    }
    impl ::core::convert::TryFrom<::std::string::String> for DirValue {
        type Error = DirValueParseError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::core::result::Result<Self, Self::Error> {
            match ::std::string::String::as_str(&value) {
                "ltr" => Ok(DirValue::Ltr),
                "rtl" => Ok(DirValue::Rtl),
                _ => {
                    Err(DirValueParseError::NonExistent {
                        value,
                    })
                }
            }
        }
    }
    impl ::core::convert::From<DirValue> for ::std::string::String {
        fn from(value: DirValue) -> Self {
            match value {
                DirValue::Ltr => ::std::string::String::from("ltr"),
                DirValue::Rtl => ::std::string::String::from("rtl"),
            }
        }
    }
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
#[xelement(
    name = "h1",
    namespace = "http://www.w3.org/1999/xhtml",
    allow_unknown_attributes = "any"
)]
pub struct H1 {
    #[xattribute(name = "id", optional, default)]
    pub id: ::core::option::Option<String>,
    #[xattribute(name = "class", optional, default)]
    pub class: ::core::option::Option<String>,
    #[xattribute(name = "style", optional, default)]
    pub style: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::StyleSheet>,
    >,
    #[xattribute(name = "title", optional, default)]
    pub title: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Text>>,
    #[xattribute(name = "lang", optional, default)]
    pub lang: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::LanguageCode>,
    >,
    #[xattribute(deferred = true, optional, default)]
    pub attribute_5: ::core::option::Option<crate::xml::Lang>,
    #[xattribute(name = "dir", optional, default)]
    pub dir: ::core::option::Option<h1_items::DirValue>,
    #[xattribute(name = "onclick", optional, default)]
    pub onclick: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xattribute(name = "ondblclick", optional, default)]
    pub ondblclick: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmousedown", optional, default)]
    pub onmousedown: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseup", optional, default)]
    pub onmouseup: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseover", optional, default)]
    pub onmouseover: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmousemove", optional, default)]
    pub onmousemove: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseout", optional, default)]
    pub onmouseout: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeypress", optional, default)]
    pub onkeypress: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeydown", optional, default)]
    pub onkeydown: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeyup", optional, default)]
    pub onkeyup: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xvalue(default)]
    pub inline: ::std::vec::Vec<h1_items::Inline>,
}
pub mod h2_items {
    impl ::core::convert::From<crate::xhtml::groups::Inline> for Inline {
        fn from(value: crate::xhtml::groups::Inline) -> Self {
            Inline::Inline(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml::groups::MiscInline> for Inline {
        fn from(value: crate::xhtml::groups::MiscInline) -> Self {
            Inline::MiscInline(::std::boxed::Box::new(value))
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub enum Inline {
        Inline(::std::boxed::Box<crate::xhtml::groups::Inline>),
        MiscInline(::std::boxed::Box<crate::xhtml::groups::MiscInline>),
    }
    #[derive(
        ::core::fmt::Debug,
        ::core::clone::Clone,
        ::core::marker::Copy,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq
    )]
    #[xvalue(with = dir_value_with)]
    pub enum DirValue {
        Ltr,
        Rtl,
    }
    pub mod dir_value_with {
        pub fn deserialize<'de, D>(
            deserializer: D,
        ) -> ::core::result::Result<super::DirValue, D::Error>
        where
            D: ::xmlity::Deserializer<'de>,
        {
            let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                deserializer,
            )?;
            let value: ::std::string::String = text
                .parse()
                .map_err(::xmlity::de::Error::custom)?;
            super::DirValue::try_from(value).map_err(::xmlity::de::Error::custom)
        }
        pub fn serialize<S>(
            value: &super::DirValue,
            serializer: S,
        ) -> ::core::result::Result<S::Ok, S::Error>
        where
            S: ::xmlity::Serializer,
        {
            let value: ::std::string::String = ::core::clone::Clone::clone(value).into();
            ::xmlity::Serialize::serialize(
                ::std::string::String::as_str(
                    &::std::string::ToString::to_string(&value),
                ),
                serializer,
            )
        }
    }
    #[derive(::core::fmt::Debug)]
    pub enum DirValueParseError {
        NonExistent { value: ::std::string::String },
    }
    impl ::core::fmt::Display for DirValueParseError {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::result::Result<(), ::core::fmt::Error> {
            match self {
                DirValueParseError::NonExistent { value } => {
                    write!(f, "Value '{:?}' does not exist in the enumeration", value)
                }
            }
        }
    }
    impl ::core::convert::TryFrom<::std::string::String> for DirValue {
        type Error = DirValueParseError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::core::result::Result<Self, Self::Error> {
            match ::std::string::String::as_str(&value) {
                "ltr" => Ok(DirValue::Ltr),
                "rtl" => Ok(DirValue::Rtl),
                _ => {
                    Err(DirValueParseError::NonExistent {
                        value,
                    })
                }
            }
        }
    }
    impl ::core::convert::From<DirValue> for ::std::string::String {
        fn from(value: DirValue) -> Self {
            match value {
                DirValue::Ltr => ::std::string::String::from("ltr"),
                DirValue::Rtl => ::std::string::String::from("rtl"),
            }
        }
    }
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
#[xelement(
    name = "h2",
    namespace = "http://www.w3.org/1999/xhtml",
    allow_unknown_attributes = "any"
)]
pub struct H2 {
    #[xattribute(name = "id", optional, default)]
    pub id: ::core::option::Option<String>,
    #[xattribute(name = "class", optional, default)]
    pub class: ::core::option::Option<String>,
    #[xattribute(name = "style", optional, default)]
    pub style: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::StyleSheet>,
    >,
    #[xattribute(name = "title", optional, default)]
    pub title: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Text>>,
    #[xattribute(name = "lang", optional, default)]
    pub lang: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::LanguageCode>,
    >,
    #[xattribute(deferred = true, optional, default)]
    pub attribute_5: ::core::option::Option<crate::xml::Lang>,
    #[xattribute(name = "dir", optional, default)]
    pub dir: ::core::option::Option<h2_items::DirValue>,
    #[xattribute(name = "onclick", optional, default)]
    pub onclick: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xattribute(name = "ondblclick", optional, default)]
    pub ondblclick: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmousedown", optional, default)]
    pub onmousedown: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseup", optional, default)]
    pub onmouseup: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseover", optional, default)]
    pub onmouseover: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmousemove", optional, default)]
    pub onmousemove: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseout", optional, default)]
    pub onmouseout: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeypress", optional, default)]
    pub onkeypress: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeydown", optional, default)]
    pub onkeydown: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeyup", optional, default)]
    pub onkeyup: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xvalue(default)]
    pub inline: ::std::vec::Vec<h2_items::Inline>,
}
pub mod h3_items {
    impl ::core::convert::From<crate::xhtml::groups::Inline> for Inline {
        fn from(value: crate::xhtml::groups::Inline) -> Self {
            Inline::Inline(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml::groups::MiscInline> for Inline {
        fn from(value: crate::xhtml::groups::MiscInline) -> Self {
            Inline::MiscInline(::std::boxed::Box::new(value))
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub enum Inline {
        Inline(::std::boxed::Box<crate::xhtml::groups::Inline>),
        MiscInline(::std::boxed::Box<crate::xhtml::groups::MiscInline>),
    }
    #[derive(
        ::core::fmt::Debug,
        ::core::clone::Clone,
        ::core::marker::Copy,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq
    )]
    #[xvalue(with = dir_value_with)]
    pub enum DirValue {
        Ltr,
        Rtl,
    }
    pub mod dir_value_with {
        pub fn deserialize<'de, D>(
            deserializer: D,
        ) -> ::core::result::Result<super::DirValue, D::Error>
        where
            D: ::xmlity::Deserializer<'de>,
        {
            let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                deserializer,
            )?;
            let value: ::std::string::String = text
                .parse()
                .map_err(::xmlity::de::Error::custom)?;
            super::DirValue::try_from(value).map_err(::xmlity::de::Error::custom)
        }
        pub fn serialize<S>(
            value: &super::DirValue,
            serializer: S,
        ) -> ::core::result::Result<S::Ok, S::Error>
        where
            S: ::xmlity::Serializer,
        {
            let value: ::std::string::String = ::core::clone::Clone::clone(value).into();
            ::xmlity::Serialize::serialize(
                ::std::string::String::as_str(
                    &::std::string::ToString::to_string(&value),
                ),
                serializer,
            )
        }
    }
    #[derive(::core::fmt::Debug)]
    pub enum DirValueParseError {
        NonExistent { value: ::std::string::String },
    }
    impl ::core::fmt::Display for DirValueParseError {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::result::Result<(), ::core::fmt::Error> {
            match self {
                DirValueParseError::NonExistent { value } => {
                    write!(f, "Value '{:?}' does not exist in the enumeration", value)
                }
            }
        }
    }
    impl ::core::convert::TryFrom<::std::string::String> for DirValue {
        type Error = DirValueParseError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::core::result::Result<Self, Self::Error> {
            match ::std::string::String::as_str(&value) {
                "ltr" => Ok(DirValue::Ltr),
                "rtl" => Ok(DirValue::Rtl),
                _ => {
                    Err(DirValueParseError::NonExistent {
                        value,
                    })
                }
            }
        }
    }
    impl ::core::convert::From<DirValue> for ::std::string::String {
        fn from(value: DirValue) -> Self {
            match value {
                DirValue::Ltr => ::std::string::String::from("ltr"),
                DirValue::Rtl => ::std::string::String::from("rtl"),
            }
        }
    }
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
#[xelement(
    name = "h3",
    namespace = "http://www.w3.org/1999/xhtml",
    allow_unknown_attributes = "any"
)]
pub struct H3 {
    #[xattribute(name = "id", optional, default)]
    pub id: ::core::option::Option<String>,
    #[xattribute(name = "class", optional, default)]
    pub class: ::core::option::Option<String>,
    #[xattribute(name = "style", optional, default)]
    pub style: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::StyleSheet>,
    >,
    #[xattribute(name = "title", optional, default)]
    pub title: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Text>>,
    #[xattribute(name = "lang", optional, default)]
    pub lang: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::LanguageCode>,
    >,
    #[xattribute(deferred = true, optional, default)]
    pub attribute_5: ::core::option::Option<crate::xml::Lang>,
    #[xattribute(name = "dir", optional, default)]
    pub dir: ::core::option::Option<h3_items::DirValue>,
    #[xattribute(name = "onclick", optional, default)]
    pub onclick: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xattribute(name = "ondblclick", optional, default)]
    pub ondblclick: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmousedown", optional, default)]
    pub onmousedown: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseup", optional, default)]
    pub onmouseup: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseover", optional, default)]
    pub onmouseover: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmousemove", optional, default)]
    pub onmousemove: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseout", optional, default)]
    pub onmouseout: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeypress", optional, default)]
    pub onkeypress: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeydown", optional, default)]
    pub onkeydown: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeyup", optional, default)]
    pub onkeyup: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xvalue(default)]
    pub inline: ::std::vec::Vec<h3_items::Inline>,
}
pub mod h4_items {
    impl ::core::convert::From<crate::xhtml::groups::Inline> for Inline {
        fn from(value: crate::xhtml::groups::Inline) -> Self {
            Inline::Inline(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml::groups::MiscInline> for Inline {
        fn from(value: crate::xhtml::groups::MiscInline) -> Self {
            Inline::MiscInline(::std::boxed::Box::new(value))
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub enum Inline {
        Inline(::std::boxed::Box<crate::xhtml::groups::Inline>),
        MiscInline(::std::boxed::Box<crate::xhtml::groups::MiscInline>),
    }
    #[derive(
        ::core::fmt::Debug,
        ::core::clone::Clone,
        ::core::marker::Copy,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq
    )]
    #[xvalue(with = dir_value_with)]
    pub enum DirValue {
        Ltr,
        Rtl,
    }
    pub mod dir_value_with {
        pub fn deserialize<'de, D>(
            deserializer: D,
        ) -> ::core::result::Result<super::DirValue, D::Error>
        where
            D: ::xmlity::Deserializer<'de>,
        {
            let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                deserializer,
            )?;
            let value: ::std::string::String = text
                .parse()
                .map_err(::xmlity::de::Error::custom)?;
            super::DirValue::try_from(value).map_err(::xmlity::de::Error::custom)
        }
        pub fn serialize<S>(
            value: &super::DirValue,
            serializer: S,
        ) -> ::core::result::Result<S::Ok, S::Error>
        where
            S: ::xmlity::Serializer,
        {
            let value: ::std::string::String = ::core::clone::Clone::clone(value).into();
            ::xmlity::Serialize::serialize(
                ::std::string::String::as_str(
                    &::std::string::ToString::to_string(&value),
                ),
                serializer,
            )
        }
    }
    #[derive(::core::fmt::Debug)]
    pub enum DirValueParseError {
        NonExistent { value: ::std::string::String },
    }
    impl ::core::fmt::Display for DirValueParseError {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::result::Result<(), ::core::fmt::Error> {
            match self {
                DirValueParseError::NonExistent { value } => {
                    write!(f, "Value '{:?}' does not exist in the enumeration", value)
                }
            }
        }
    }
    impl ::core::convert::TryFrom<::std::string::String> for DirValue {
        type Error = DirValueParseError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::core::result::Result<Self, Self::Error> {
            match ::std::string::String::as_str(&value) {
                "ltr" => Ok(DirValue::Ltr),
                "rtl" => Ok(DirValue::Rtl),
                _ => {
                    Err(DirValueParseError::NonExistent {
                        value,
                    })
                }
            }
        }
    }
    impl ::core::convert::From<DirValue> for ::std::string::String {
        fn from(value: DirValue) -> Self {
            match value {
                DirValue::Ltr => ::std::string::String::from("ltr"),
                DirValue::Rtl => ::std::string::String::from("rtl"),
            }
        }
    }
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
#[xelement(
    name = "h4",
    namespace = "http://www.w3.org/1999/xhtml",
    allow_unknown_attributes = "any"
)]
pub struct H4 {
    #[xattribute(name = "id", optional, default)]
    pub id: ::core::option::Option<String>,
    #[xattribute(name = "class", optional, default)]
    pub class: ::core::option::Option<String>,
    #[xattribute(name = "style", optional, default)]
    pub style: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::StyleSheet>,
    >,
    #[xattribute(name = "title", optional, default)]
    pub title: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Text>>,
    #[xattribute(name = "lang", optional, default)]
    pub lang: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::LanguageCode>,
    >,
    #[xattribute(deferred = true, optional, default)]
    pub attribute_5: ::core::option::Option<crate::xml::Lang>,
    #[xattribute(name = "dir", optional, default)]
    pub dir: ::core::option::Option<h4_items::DirValue>,
    #[xattribute(name = "onclick", optional, default)]
    pub onclick: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xattribute(name = "ondblclick", optional, default)]
    pub ondblclick: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmousedown", optional, default)]
    pub onmousedown: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseup", optional, default)]
    pub onmouseup: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseover", optional, default)]
    pub onmouseover: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmousemove", optional, default)]
    pub onmousemove: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseout", optional, default)]
    pub onmouseout: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeypress", optional, default)]
    pub onkeypress: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeydown", optional, default)]
    pub onkeydown: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeyup", optional, default)]
    pub onkeyup: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xvalue(default)]
    pub inline: ::std::vec::Vec<h4_items::Inline>,
}
pub mod h5_items {
    impl ::core::convert::From<crate::xhtml::groups::Inline> for Inline {
        fn from(value: crate::xhtml::groups::Inline) -> Self {
            Inline::Inline(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml::groups::MiscInline> for Inline {
        fn from(value: crate::xhtml::groups::MiscInline) -> Self {
            Inline::MiscInline(::std::boxed::Box::new(value))
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub enum Inline {
        Inline(::std::boxed::Box<crate::xhtml::groups::Inline>),
        MiscInline(::std::boxed::Box<crate::xhtml::groups::MiscInline>),
    }
    #[derive(
        ::core::fmt::Debug,
        ::core::clone::Clone,
        ::core::marker::Copy,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq
    )]
    #[xvalue(with = dir_value_with)]
    pub enum DirValue {
        Ltr,
        Rtl,
    }
    pub mod dir_value_with {
        pub fn deserialize<'de, D>(
            deserializer: D,
        ) -> ::core::result::Result<super::DirValue, D::Error>
        where
            D: ::xmlity::Deserializer<'de>,
        {
            let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                deserializer,
            )?;
            let value: ::std::string::String = text
                .parse()
                .map_err(::xmlity::de::Error::custom)?;
            super::DirValue::try_from(value).map_err(::xmlity::de::Error::custom)
        }
        pub fn serialize<S>(
            value: &super::DirValue,
            serializer: S,
        ) -> ::core::result::Result<S::Ok, S::Error>
        where
            S: ::xmlity::Serializer,
        {
            let value: ::std::string::String = ::core::clone::Clone::clone(value).into();
            ::xmlity::Serialize::serialize(
                ::std::string::String::as_str(
                    &::std::string::ToString::to_string(&value),
                ),
                serializer,
            )
        }
    }
    #[derive(::core::fmt::Debug)]
    pub enum DirValueParseError {
        NonExistent { value: ::std::string::String },
    }
    impl ::core::fmt::Display for DirValueParseError {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::result::Result<(), ::core::fmt::Error> {
            match self {
                DirValueParseError::NonExistent { value } => {
                    write!(f, "Value '{:?}' does not exist in the enumeration", value)
                }
            }
        }
    }
    impl ::core::convert::TryFrom<::std::string::String> for DirValue {
        type Error = DirValueParseError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::core::result::Result<Self, Self::Error> {
            match ::std::string::String::as_str(&value) {
                "ltr" => Ok(DirValue::Ltr),
                "rtl" => Ok(DirValue::Rtl),
                _ => {
                    Err(DirValueParseError::NonExistent {
                        value,
                    })
                }
            }
        }
    }
    impl ::core::convert::From<DirValue> for ::std::string::String {
        fn from(value: DirValue) -> Self {
            match value {
                DirValue::Ltr => ::std::string::String::from("ltr"),
                DirValue::Rtl => ::std::string::String::from("rtl"),
            }
        }
    }
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
#[xelement(
    name = "h5",
    namespace = "http://www.w3.org/1999/xhtml",
    allow_unknown_attributes = "any"
)]
pub struct H5 {
    #[xattribute(name = "id", optional, default)]
    pub id: ::core::option::Option<String>,
    #[xattribute(name = "class", optional, default)]
    pub class: ::core::option::Option<String>,
    #[xattribute(name = "style", optional, default)]
    pub style: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::StyleSheet>,
    >,
    #[xattribute(name = "title", optional, default)]
    pub title: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Text>>,
    #[xattribute(name = "lang", optional, default)]
    pub lang: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::LanguageCode>,
    >,
    #[xattribute(deferred = true, optional, default)]
    pub attribute_5: ::core::option::Option<crate::xml::Lang>,
    #[xattribute(name = "dir", optional, default)]
    pub dir: ::core::option::Option<h5_items::DirValue>,
    #[xattribute(name = "onclick", optional, default)]
    pub onclick: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xattribute(name = "ondblclick", optional, default)]
    pub ondblclick: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmousedown", optional, default)]
    pub onmousedown: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseup", optional, default)]
    pub onmouseup: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseover", optional, default)]
    pub onmouseover: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmousemove", optional, default)]
    pub onmousemove: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseout", optional, default)]
    pub onmouseout: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeypress", optional, default)]
    pub onkeypress: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeydown", optional, default)]
    pub onkeydown: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeyup", optional, default)]
    pub onkeyup: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xvalue(default)]
    pub inline: ::std::vec::Vec<h5_items::Inline>,
}
pub mod h6_items {
    impl ::core::convert::From<crate::xhtml::groups::Inline> for Inline {
        fn from(value: crate::xhtml::groups::Inline) -> Self {
            Inline::Inline(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml::groups::MiscInline> for Inline {
        fn from(value: crate::xhtml::groups::MiscInline) -> Self {
            Inline::MiscInline(::std::boxed::Box::new(value))
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub enum Inline {
        Inline(::std::boxed::Box<crate::xhtml::groups::Inline>),
        MiscInline(::std::boxed::Box<crate::xhtml::groups::MiscInline>),
    }
    #[derive(
        ::core::fmt::Debug,
        ::core::clone::Clone,
        ::core::marker::Copy,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq
    )]
    #[xvalue(with = dir_value_with)]
    pub enum DirValue {
        Ltr,
        Rtl,
    }
    pub mod dir_value_with {
        pub fn deserialize<'de, D>(
            deserializer: D,
        ) -> ::core::result::Result<super::DirValue, D::Error>
        where
            D: ::xmlity::Deserializer<'de>,
        {
            let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                deserializer,
            )?;
            let value: ::std::string::String = text
                .parse()
                .map_err(::xmlity::de::Error::custom)?;
            super::DirValue::try_from(value).map_err(::xmlity::de::Error::custom)
        }
        pub fn serialize<S>(
            value: &super::DirValue,
            serializer: S,
        ) -> ::core::result::Result<S::Ok, S::Error>
        where
            S: ::xmlity::Serializer,
        {
            let value: ::std::string::String = ::core::clone::Clone::clone(value).into();
            ::xmlity::Serialize::serialize(
                ::std::string::String::as_str(
                    &::std::string::ToString::to_string(&value),
                ),
                serializer,
            )
        }
    }
    #[derive(::core::fmt::Debug)]
    pub enum DirValueParseError {
        NonExistent { value: ::std::string::String },
    }
    impl ::core::fmt::Display for DirValueParseError {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::result::Result<(), ::core::fmt::Error> {
            match self {
                DirValueParseError::NonExistent { value } => {
                    write!(f, "Value '{:?}' does not exist in the enumeration", value)
                }
            }
        }
    }
    impl ::core::convert::TryFrom<::std::string::String> for DirValue {
        type Error = DirValueParseError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::core::result::Result<Self, Self::Error> {
            match ::std::string::String::as_str(&value) {
                "ltr" => Ok(DirValue::Ltr),
                "rtl" => Ok(DirValue::Rtl),
                _ => {
                    Err(DirValueParseError::NonExistent {
                        value,
                    })
                }
            }
        }
    }
    impl ::core::convert::From<DirValue> for ::std::string::String {
        fn from(value: DirValue) -> Self {
            match value {
                DirValue::Ltr => ::std::string::String::from("ltr"),
                DirValue::Rtl => ::std::string::String::from("rtl"),
            }
        }
    }
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
#[xelement(
    name = "h6",
    namespace = "http://www.w3.org/1999/xhtml",
    allow_unknown_attributes = "any"
)]
pub struct H6 {
    #[xattribute(name = "id", optional, default)]
    pub id: ::core::option::Option<String>,
    #[xattribute(name = "class", optional, default)]
    pub class: ::core::option::Option<String>,
    #[xattribute(name = "style", optional, default)]
    pub style: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::StyleSheet>,
    >,
    #[xattribute(name = "title", optional, default)]
    pub title: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Text>>,
    #[xattribute(name = "lang", optional, default)]
    pub lang: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::LanguageCode>,
    >,
    #[xattribute(deferred = true, optional, default)]
    pub attribute_5: ::core::option::Option<crate::xml::Lang>,
    #[xattribute(name = "dir", optional, default)]
    pub dir: ::core::option::Option<h6_items::DirValue>,
    #[xattribute(name = "onclick", optional, default)]
    pub onclick: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xattribute(name = "ondblclick", optional, default)]
    pub ondblclick: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmousedown", optional, default)]
    pub onmousedown: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseup", optional, default)]
    pub onmouseup: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseover", optional, default)]
    pub onmouseover: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmousemove", optional, default)]
    pub onmousemove: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseout", optional, default)]
    pub onmouseout: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeypress", optional, default)]
    pub onkeypress: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeydown", optional, default)]
    pub onkeydown: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeyup", optional, default)]
    pub onkeyup: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xvalue(default)]
    pub inline: ::std::vec::Vec<h6_items::Inline>,
}
pub mod head_items {
    pub mod variant_variants {
        pub mod variant_0_items {
            #[derive(
                ::core::fmt::Debug,
                ::xmlity::Serialize,
                ::xmlity::Deserialize,
                ::core::cmp::PartialEq,
                ::core::clone::Clone
            )]
            #[xvalue(order = "strict")]
            pub struct Child2 {
                pub base: crate::xhtml::Base,
                pub head_misc: crate::xhtml::groups::HeadMisc,
            }
        }
        #[derive(
            ::core::fmt::Debug,
            ::xmlity::Serialize,
            ::xmlity::Deserialize,
            ::core::cmp::PartialEq,
            ::core::clone::Clone
        )]
        #[xvalue(order = "strict")]
        pub struct Variant0 {
            pub title: ::std::boxed::Box<crate::xhtml::Title>,
            pub head_misc: crate::xhtml::groups::HeadMisc,
            #[xvalue(default)]
            pub child_2: ::core::option::Option<variant_0_items::Child2>,
        }
        #[derive(
            ::core::fmt::Debug,
            ::xmlity::Serialize,
            ::xmlity::Deserialize,
            ::core::cmp::PartialEq,
            ::core::clone::Clone
        )]
        #[xvalue(order = "strict")]
        pub struct Variant1 {
            pub base: crate::xhtml::Base,
            pub head_misc: crate::xhtml::groups::HeadMisc,
            pub title: ::std::boxed::Box<crate::xhtml::Title>,
            pub head_misc_0: crate::xhtml::groups::HeadMisc,
        }
    }
    impl ::core::convert::From<variant_variants::Variant0> for Variant {
        fn from(value: variant_variants::Variant0) -> Self {
            Variant::Variant0(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<variant_variants::Variant1> for Variant {
        fn from(value: variant_variants::Variant1) -> Self {
            Variant::Variant1(::std::boxed::Box::new(value))
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub enum Variant {
        Variant0(::std::boxed::Box<variant_variants::Variant0>),
        Variant1(::std::boxed::Box<variant_variants::Variant1>),
    }
    #[derive(
        ::core::fmt::Debug,
        ::core::clone::Clone,
        ::core::marker::Copy,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq
    )]
    #[xvalue(with = dir_value_with)]
    pub enum DirValue {
        Ltr,
        Rtl,
    }
    pub mod dir_value_with {
        pub fn deserialize<'de, D>(
            deserializer: D,
        ) -> ::core::result::Result<super::DirValue, D::Error>
        where
            D: ::xmlity::Deserializer<'de>,
        {
            let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                deserializer,
            )?;
            let value: ::std::string::String = text
                .parse()
                .map_err(::xmlity::de::Error::custom)?;
            super::DirValue::try_from(value).map_err(::xmlity::de::Error::custom)
        }
        pub fn serialize<S>(
            value: &super::DirValue,
            serializer: S,
        ) -> ::core::result::Result<S::Ok, S::Error>
        where
            S: ::xmlity::Serializer,
        {
            let value: ::std::string::String = ::core::clone::Clone::clone(value).into();
            ::xmlity::Serialize::serialize(
                ::std::string::String::as_str(
                    &::std::string::ToString::to_string(&value),
                ),
                serializer,
            )
        }
    }
    #[derive(::core::fmt::Debug)]
    pub enum DirValueParseError {
        NonExistent { value: ::std::string::String },
    }
    impl ::core::fmt::Display for DirValueParseError {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::result::Result<(), ::core::fmt::Error> {
            match self {
                DirValueParseError::NonExistent { value } => {
                    write!(f, "Value '{:?}' does not exist in the enumeration", value)
                }
            }
        }
    }
    impl ::core::convert::TryFrom<::std::string::String> for DirValue {
        type Error = DirValueParseError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::core::result::Result<Self, Self::Error> {
            match ::std::string::String::as_str(&value) {
                "ltr" => Ok(DirValue::Ltr),
                "rtl" => Ok(DirValue::Rtl),
                _ => {
                    Err(DirValueParseError::NonExistent {
                        value,
                    })
                }
            }
        }
    }
    impl ::core::convert::From<DirValue> for ::std::string::String {
        fn from(value: DirValue) -> Self {
            match value {
                DirValue::Ltr => ::std::string::String::from("ltr"),
                DirValue::Rtl => ::std::string::String::from("rtl"),
            }
        }
    }
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
#[xelement(
    name = "head",
    namespace = "http://www.w3.org/1999/xhtml",
    allow_unknown_attributes = "any",
    children_order = "strict"
)]
pub struct Head {
    #[xattribute(name = "lang", optional, default)]
    pub lang: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::LanguageCode>,
    >,
    #[xattribute(deferred = true, optional, default)]
    pub attribute_1: ::core::option::Option<crate::xml::Lang>,
    #[xattribute(name = "dir", optional, default)]
    pub dir: ::core::option::Option<head_items::DirValue>,
    #[xattribute(name = "id", optional, default)]
    pub id: ::core::option::Option<String>,
    #[xattribute(name = "profile", optional, default)]
    pub profile: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Uri>>,
    pub head_misc: crate::xhtml::groups::HeadMisc,
    pub variant: head_items::Variant,
}
pub mod hr_items {
    #[derive(
        ::core::fmt::Debug,
        ::core::clone::Clone,
        ::core::marker::Copy,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq
    )]
    #[xvalue(with = dir_value_with)]
    pub enum DirValue {
        Ltr,
        Rtl,
    }
    pub mod dir_value_with {
        pub fn deserialize<'de, D>(
            deserializer: D,
        ) -> ::core::result::Result<super::DirValue, D::Error>
        where
            D: ::xmlity::Deserializer<'de>,
        {
            let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                deserializer,
            )?;
            let value: ::std::string::String = text
                .parse()
                .map_err(::xmlity::de::Error::custom)?;
            super::DirValue::try_from(value).map_err(::xmlity::de::Error::custom)
        }
        pub fn serialize<S>(
            value: &super::DirValue,
            serializer: S,
        ) -> ::core::result::Result<S::Ok, S::Error>
        where
            S: ::xmlity::Serializer,
        {
            let value: ::std::string::String = ::core::clone::Clone::clone(value).into();
            ::xmlity::Serialize::serialize(
                ::std::string::String::as_str(
                    &::std::string::ToString::to_string(&value),
                ),
                serializer,
            )
        }
    }
    #[derive(::core::fmt::Debug)]
    pub enum DirValueParseError {
        NonExistent { value: ::std::string::String },
    }
    impl ::core::fmt::Display for DirValueParseError {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::result::Result<(), ::core::fmt::Error> {
            match self {
                DirValueParseError::NonExistent { value } => {
                    write!(f, "Value '{:?}' does not exist in the enumeration", value)
                }
            }
        }
    }
    impl ::core::convert::TryFrom<::std::string::String> for DirValue {
        type Error = DirValueParseError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::core::result::Result<Self, Self::Error> {
            match ::std::string::String::as_str(&value) {
                "ltr" => Ok(DirValue::Ltr),
                "rtl" => Ok(DirValue::Rtl),
                _ => {
                    Err(DirValueParseError::NonExistent {
                        value,
                    })
                }
            }
        }
    }
    impl ::core::convert::From<DirValue> for ::std::string::String {
        fn from(value: DirValue) -> Self {
            match value {
                DirValue::Ltr => ::std::string::String::from("ltr"),
                DirValue::Rtl => ::std::string::String::from("rtl"),
            }
        }
    }
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
#[xelement(
    name = "hr",
    namespace = "http://www.w3.org/1999/xhtml",
    allow_unknown_attributes = "any"
)]
pub struct Hr {
    #[xattribute(name = "id", optional, default)]
    pub id: ::core::option::Option<String>,
    #[xattribute(name = "class", optional, default)]
    pub class: ::core::option::Option<String>,
    #[xattribute(name = "style", optional, default)]
    pub style: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::StyleSheet>,
    >,
    #[xattribute(name = "title", optional, default)]
    pub title: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Text>>,
    #[xattribute(name = "lang", optional, default)]
    pub lang: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::LanguageCode>,
    >,
    #[xattribute(deferred = true, optional, default)]
    pub attribute_5: ::core::option::Option<crate::xml::Lang>,
    #[xattribute(name = "dir", optional, default)]
    pub dir: ::core::option::Option<hr_items::DirValue>,
    #[xattribute(name = "onclick", optional, default)]
    pub onclick: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xattribute(name = "ondblclick", optional, default)]
    pub ondblclick: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmousedown", optional, default)]
    pub onmousedown: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseup", optional, default)]
    pub onmouseup: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseover", optional, default)]
    pub onmouseover: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmousemove", optional, default)]
    pub onmousemove: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseout", optional, default)]
    pub onmouseout: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeypress", optional, default)]
    pub onkeypress: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeydown", optional, default)]
    pub onkeydown: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeyup", optional, default)]
    pub onkeyup: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
}
pub mod html_items {
    #[derive(
        ::core::fmt::Debug,
        ::core::clone::Clone,
        ::core::marker::Copy,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq
    )]
    #[xvalue(with = dir_value_with)]
    pub enum DirValue {
        Ltr,
        Rtl,
    }
    pub mod dir_value_with {
        pub fn deserialize<'de, D>(
            deserializer: D,
        ) -> ::core::result::Result<super::DirValue, D::Error>
        where
            D: ::xmlity::Deserializer<'de>,
        {
            let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                deserializer,
            )?;
            let value: ::std::string::String = text
                .parse()
                .map_err(::xmlity::de::Error::custom)?;
            super::DirValue::try_from(value).map_err(::xmlity::de::Error::custom)
        }
        pub fn serialize<S>(
            value: &super::DirValue,
            serializer: S,
        ) -> ::core::result::Result<S::Ok, S::Error>
        where
            S: ::xmlity::Serializer,
        {
            let value: ::std::string::String = ::core::clone::Clone::clone(value).into();
            ::xmlity::Serialize::serialize(
                ::std::string::String::as_str(
                    &::std::string::ToString::to_string(&value),
                ),
                serializer,
            )
        }
    }
    #[derive(::core::fmt::Debug)]
    pub enum DirValueParseError {
        NonExistent { value: ::std::string::String },
    }
    impl ::core::fmt::Display for DirValueParseError {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::result::Result<(), ::core::fmt::Error> {
            match self {
                DirValueParseError::NonExistent { value } => {
                    write!(f, "Value '{:?}' does not exist in the enumeration", value)
                }
            }
        }
    }
    impl ::core::convert::TryFrom<::std::string::String> for DirValue {
        type Error = DirValueParseError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::core::result::Result<Self, Self::Error> {
            match ::std::string::String::as_str(&value) {
                "ltr" => Ok(DirValue::Ltr),
                "rtl" => Ok(DirValue::Rtl),
                _ => {
                    Err(DirValueParseError::NonExistent {
                        value,
                    })
                }
            }
        }
    }
    impl ::core::convert::From<DirValue> for ::std::string::String {
        fn from(value: DirValue) -> Self {
            match value {
                DirValue::Ltr => ::std::string::String::from("ltr"),
                DirValue::Rtl => ::std::string::String::from("rtl"),
            }
        }
    }
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
#[xelement(
    name = "html",
    namespace = "http://www.w3.org/1999/xhtml",
    allow_unknown_attributes = "any",
    children_order = "strict"
)]
pub struct Html {
    #[xattribute(name = "lang", optional, default)]
    pub lang: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::LanguageCode>,
    >,
    #[xattribute(deferred = true, optional, default)]
    pub attribute_1: ::core::option::Option<crate::xml::Lang>,
    #[xattribute(name = "dir", optional, default)]
    pub dir: ::core::option::Option<html_items::DirValue>,
    #[xattribute(name = "id", optional, default)]
    pub id: ::core::option::Option<String>,
    pub head: crate::xhtml::Head,
    pub body: crate::xhtml::Body,
}
pub mod i_items {
    impl ::core::convert::From<crate::xhtml::groups::Inline> for Inline {
        fn from(value: crate::xhtml::groups::Inline) -> Self {
            Inline::Inline(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml::groups::MiscInline> for Inline {
        fn from(value: crate::xhtml::groups::MiscInline) -> Self {
            Inline::MiscInline(::std::boxed::Box::new(value))
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub enum Inline {
        Inline(::std::boxed::Box<crate::xhtml::groups::Inline>),
        MiscInline(::std::boxed::Box<crate::xhtml::groups::MiscInline>),
    }
    #[derive(
        ::core::fmt::Debug,
        ::core::clone::Clone,
        ::core::marker::Copy,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq
    )]
    #[xvalue(with = dir_value_with)]
    pub enum DirValue {
        Ltr,
        Rtl,
    }
    pub mod dir_value_with {
        pub fn deserialize<'de, D>(
            deserializer: D,
        ) -> ::core::result::Result<super::DirValue, D::Error>
        where
            D: ::xmlity::Deserializer<'de>,
        {
            let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                deserializer,
            )?;
            let value: ::std::string::String = text
                .parse()
                .map_err(::xmlity::de::Error::custom)?;
            super::DirValue::try_from(value).map_err(::xmlity::de::Error::custom)
        }
        pub fn serialize<S>(
            value: &super::DirValue,
            serializer: S,
        ) -> ::core::result::Result<S::Ok, S::Error>
        where
            S: ::xmlity::Serializer,
        {
            let value: ::std::string::String = ::core::clone::Clone::clone(value).into();
            ::xmlity::Serialize::serialize(
                ::std::string::String::as_str(
                    &::std::string::ToString::to_string(&value),
                ),
                serializer,
            )
        }
    }
    #[derive(::core::fmt::Debug)]
    pub enum DirValueParseError {
        NonExistent { value: ::std::string::String },
    }
    impl ::core::fmt::Display for DirValueParseError {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::result::Result<(), ::core::fmt::Error> {
            match self {
                DirValueParseError::NonExistent { value } => {
                    write!(f, "Value '{:?}' does not exist in the enumeration", value)
                }
            }
        }
    }
    impl ::core::convert::TryFrom<::std::string::String> for DirValue {
        type Error = DirValueParseError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::core::result::Result<Self, Self::Error> {
            match ::std::string::String::as_str(&value) {
                "ltr" => Ok(DirValue::Ltr),
                "rtl" => Ok(DirValue::Rtl),
                _ => {
                    Err(DirValueParseError::NonExistent {
                        value,
                    })
                }
            }
        }
    }
    impl ::core::convert::From<DirValue> for ::std::string::String {
        fn from(value: DirValue) -> Self {
            match value {
                DirValue::Ltr => ::std::string::String::from("ltr"),
                DirValue::Rtl => ::std::string::String::from("rtl"),
            }
        }
    }
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
#[xelement(
    name = "i",
    namespace = "http://www.w3.org/1999/xhtml",
    allow_unknown_attributes = "any"
)]
pub struct I {
    #[xattribute(name = "id", optional, default)]
    pub id: ::core::option::Option<String>,
    #[xattribute(name = "class", optional, default)]
    pub class: ::core::option::Option<String>,
    #[xattribute(name = "style", optional, default)]
    pub style: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::StyleSheet>,
    >,
    #[xattribute(name = "title", optional, default)]
    pub title: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Text>>,
    #[xattribute(name = "lang", optional, default)]
    pub lang: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::LanguageCode>,
    >,
    #[xattribute(deferred = true, optional, default)]
    pub attribute_5: ::core::option::Option<crate::xml::Lang>,
    #[xattribute(name = "dir", optional, default)]
    pub dir: ::core::option::Option<i_items::DirValue>,
    #[xattribute(name = "onclick", optional, default)]
    pub onclick: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xattribute(name = "ondblclick", optional, default)]
    pub ondblclick: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmousedown", optional, default)]
    pub onmousedown: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseup", optional, default)]
    pub onmouseup: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseover", optional, default)]
    pub onmouseover: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmousemove", optional, default)]
    pub onmousemove: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseout", optional, default)]
    pub onmouseout: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeypress", optional, default)]
    pub onkeypress: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeydown", optional, default)]
    pub onkeydown: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeyup", optional, default)]
    pub onkeyup: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xvalue(default)]
    pub inline: ::std::vec::Vec<i_items::Inline>,
}
pub mod img_items {
    #[derive(
        ::core::fmt::Debug,
        ::core::clone::Clone,
        ::core::marker::Copy,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq
    )]
    #[xvalue(with = dir_value_with)]
    pub enum DirValue {
        Ltr,
        Rtl,
    }
    pub mod dir_value_with {
        pub fn deserialize<'de, D>(
            deserializer: D,
        ) -> ::core::result::Result<super::DirValue, D::Error>
        where
            D: ::xmlity::Deserializer<'de>,
        {
            let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                deserializer,
            )?;
            let value: ::std::string::String = text
                .parse()
                .map_err(::xmlity::de::Error::custom)?;
            super::DirValue::try_from(value).map_err(::xmlity::de::Error::custom)
        }
        pub fn serialize<S>(
            value: &super::DirValue,
            serializer: S,
        ) -> ::core::result::Result<S::Ok, S::Error>
        where
            S: ::xmlity::Serializer,
        {
            let value: ::std::string::String = ::core::clone::Clone::clone(value).into();
            ::xmlity::Serialize::serialize(
                ::std::string::String::as_str(
                    &::std::string::ToString::to_string(&value),
                ),
                serializer,
            )
        }
    }
    #[derive(::core::fmt::Debug)]
    pub enum DirValueParseError {
        NonExistent { value: ::std::string::String },
    }
    impl ::core::fmt::Display for DirValueParseError {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::result::Result<(), ::core::fmt::Error> {
            match self {
                DirValueParseError::NonExistent { value } => {
                    write!(f, "Value '{:?}' does not exist in the enumeration", value)
                }
            }
        }
    }
    impl ::core::convert::TryFrom<::std::string::String> for DirValue {
        type Error = DirValueParseError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::core::result::Result<Self, Self::Error> {
            match ::std::string::String::as_str(&value) {
                "ltr" => Ok(DirValue::Ltr),
                "rtl" => Ok(DirValue::Rtl),
                _ => {
                    Err(DirValueParseError::NonExistent {
                        value,
                    })
                }
            }
        }
    }
    impl ::core::convert::From<DirValue> for ::std::string::String {
        fn from(value: DirValue) -> Self {
            match value {
                DirValue::Ltr => ::std::string::String::from("ltr"),
                DirValue::Rtl => ::std::string::String::from("rtl"),
            }
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::core::clone::Clone,
        ::core::marker::Copy,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq
    )]
    #[xvalue(with = ismap_value_with)]
    pub enum IsmapValue {
        Ismap,
    }
    pub mod ismap_value_with {
        pub fn deserialize<'de, D>(
            deserializer: D,
        ) -> ::core::result::Result<super::IsmapValue, D::Error>
        where
            D: ::xmlity::Deserializer<'de>,
        {
            let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                deserializer,
            )?;
            let value: ::std::string::String = text
                .parse()
                .map_err(::xmlity::de::Error::custom)?;
            super::IsmapValue::try_from(value).map_err(::xmlity::de::Error::custom)
        }
        pub fn serialize<S>(
            value: &super::IsmapValue,
            serializer: S,
        ) -> ::core::result::Result<S::Ok, S::Error>
        where
            S: ::xmlity::Serializer,
        {
            let value: ::std::string::String = ::core::clone::Clone::clone(value).into();
            ::xmlity::Serialize::serialize(
                ::std::string::String::as_str(
                    &::std::string::ToString::to_string(&value),
                ),
                serializer,
            )
        }
    }
    #[derive(::core::fmt::Debug)]
    pub enum IsmapValueParseError {
        NonExistent { value: ::std::string::String },
    }
    impl ::core::fmt::Display for IsmapValueParseError {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::result::Result<(), ::core::fmt::Error> {
            match self {
                IsmapValueParseError::NonExistent { value } => {
                    write!(f, "Value '{:?}' does not exist in the enumeration", value)
                }
            }
        }
    }
    impl ::core::convert::TryFrom<::std::string::String> for IsmapValue {
        type Error = IsmapValueParseError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::core::result::Result<Self, Self::Error> {
            match ::std::string::String::as_str(&value) {
                "ismap" => Ok(IsmapValue::Ismap),
                _ => {
                    Err(IsmapValueParseError::NonExistent {
                        value,
                    })
                }
            }
        }
    }
    impl ::core::convert::From<IsmapValue> for ::std::string::String {
        fn from(value: IsmapValue) -> Self {
            match value {
                IsmapValue::Ismap => ::std::string::String::from("ismap"),
            }
        }
    }
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
#[xelement(
    name = "img",
    namespace = "http://www.w3.org/1999/xhtml",
    allow_unknown_attributes = "any"
)]
pub struct Img {
    #[xattribute(name = "id", optional, default)]
    pub id: ::core::option::Option<String>,
    #[xattribute(name = "class", optional, default)]
    pub class: ::core::option::Option<String>,
    #[xattribute(name = "style", optional, default)]
    pub style: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::StyleSheet>,
    >,
    #[xattribute(name = "title", optional, default)]
    pub title: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Text>>,
    #[xattribute(name = "lang", optional, default)]
    pub lang: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::LanguageCode>,
    >,
    #[xattribute(deferred = true, optional, default)]
    pub attribute_5: ::core::option::Option<crate::xml::Lang>,
    #[xattribute(name = "dir", optional, default)]
    pub dir: ::core::option::Option<img_items::DirValue>,
    #[xattribute(name = "onclick", optional, default)]
    pub onclick: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xattribute(name = "ondblclick", optional, default)]
    pub ondblclick: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmousedown", optional, default)]
    pub onmousedown: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseup", optional, default)]
    pub onmouseup: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseover", optional, default)]
    pub onmouseover: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmousemove", optional, default)]
    pub onmousemove: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseout", optional, default)]
    pub onmouseout: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeypress", optional, default)]
    pub onkeypress: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeydown", optional, default)]
    pub onkeydown: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeyup", optional, default)]
    pub onkeyup: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xattribute(name = "src")]
    pub src: ::std::boxed::Box<crate::xhtml::types::Uri>,
    #[xattribute(name = "alt")]
    pub alt: ::std::boxed::Box<crate::xhtml::types::Text>,
    #[xattribute(name = "longdesc", optional, default)]
    pub longdesc: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Uri>>,
    #[xattribute(name = "height", optional, default)]
    pub height: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Length>>,
    #[xattribute(name = "width", optional, default)]
    pub width: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Length>>,
    #[xattribute(name = "usemap", optional, default)]
    pub usemap: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Uri>>,
    #[xattribute(name = "ismap", optional, default)]
    pub ismap: ::core::option::Option<img_items::IsmapValue>,
}
pub mod input_items {
    #[derive(
        ::core::fmt::Debug,
        ::core::clone::Clone,
        ::core::marker::Copy,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq
    )]
    #[xvalue(with = dir_value_with)]
    pub enum DirValue {
        Ltr,
        Rtl,
    }
    pub mod dir_value_with {
        pub fn deserialize<'de, D>(
            deserializer: D,
        ) -> ::core::result::Result<super::DirValue, D::Error>
        where
            D: ::xmlity::Deserializer<'de>,
        {
            let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                deserializer,
            )?;
            let value: ::std::string::String = text
                .parse()
                .map_err(::xmlity::de::Error::custom)?;
            super::DirValue::try_from(value).map_err(::xmlity::de::Error::custom)
        }
        pub fn serialize<S>(
            value: &super::DirValue,
            serializer: S,
        ) -> ::core::result::Result<S::Ok, S::Error>
        where
            S: ::xmlity::Serializer,
        {
            let value: ::std::string::String = ::core::clone::Clone::clone(value).into();
            ::xmlity::Serialize::serialize(
                ::std::string::String::as_str(
                    &::std::string::ToString::to_string(&value),
                ),
                serializer,
            )
        }
    }
    #[derive(::core::fmt::Debug)]
    pub enum DirValueParseError {
        NonExistent { value: ::std::string::String },
    }
    impl ::core::fmt::Display for DirValueParseError {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::result::Result<(), ::core::fmt::Error> {
            match self {
                DirValueParseError::NonExistent { value } => {
                    write!(f, "Value '{:?}' does not exist in the enumeration", value)
                }
            }
        }
    }
    impl ::core::convert::TryFrom<::std::string::String> for DirValue {
        type Error = DirValueParseError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::core::result::Result<Self, Self::Error> {
            match ::std::string::String::as_str(&value) {
                "ltr" => Ok(DirValue::Ltr),
                "rtl" => Ok(DirValue::Rtl),
                _ => {
                    Err(DirValueParseError::NonExistent {
                        value,
                    })
                }
            }
        }
    }
    impl ::core::convert::From<DirValue> for ::std::string::String {
        fn from(value: DirValue) -> Self {
            match value {
                DirValue::Ltr => ::std::string::String::from("ltr"),
                DirValue::Rtl => ::std::string::String::from("rtl"),
            }
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::core::clone::Clone,
        ::core::marker::Copy,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq
    )]
    #[xvalue(with = checked_value_with)]
    pub enum CheckedValue {
        Checked,
    }
    pub mod checked_value_with {
        pub fn deserialize<'de, D>(
            deserializer: D,
        ) -> ::core::result::Result<super::CheckedValue, D::Error>
        where
            D: ::xmlity::Deserializer<'de>,
        {
            let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                deserializer,
            )?;
            let value: ::std::string::String = text
                .parse()
                .map_err(::xmlity::de::Error::custom)?;
            super::CheckedValue::try_from(value).map_err(::xmlity::de::Error::custom)
        }
        pub fn serialize<S>(
            value: &super::CheckedValue,
            serializer: S,
        ) -> ::core::result::Result<S::Ok, S::Error>
        where
            S: ::xmlity::Serializer,
        {
            let value: ::std::string::String = ::core::clone::Clone::clone(value).into();
            ::xmlity::Serialize::serialize(
                ::std::string::String::as_str(
                    &::std::string::ToString::to_string(&value),
                ),
                serializer,
            )
        }
    }
    #[derive(::core::fmt::Debug)]
    pub enum CheckedValueParseError {
        NonExistent { value: ::std::string::String },
    }
    impl ::core::fmt::Display for CheckedValueParseError {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::result::Result<(), ::core::fmt::Error> {
            match self {
                CheckedValueParseError::NonExistent { value } => {
                    write!(f, "Value '{:?}' does not exist in the enumeration", value)
                }
            }
        }
    }
    impl ::core::convert::TryFrom<::std::string::String> for CheckedValue {
        type Error = CheckedValueParseError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::core::result::Result<Self, Self::Error> {
            match ::std::string::String::as_str(&value) {
                "checked" => Ok(CheckedValue::Checked),
                _ => {
                    Err(CheckedValueParseError::NonExistent {
                        value,
                    })
                }
            }
        }
    }
    impl ::core::convert::From<CheckedValue> for ::std::string::String {
        fn from(value: CheckedValue) -> Self {
            match value {
                CheckedValue::Checked => ::std::string::String::from("checked"),
            }
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::core::clone::Clone,
        ::core::marker::Copy,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq
    )]
    #[xvalue(with = disabled_value_with)]
    pub enum DisabledValue {
        Disabled,
    }
    pub mod disabled_value_with {
        pub fn deserialize<'de, D>(
            deserializer: D,
        ) -> ::core::result::Result<super::DisabledValue, D::Error>
        where
            D: ::xmlity::Deserializer<'de>,
        {
            let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                deserializer,
            )?;
            let value: ::std::string::String = text
                .parse()
                .map_err(::xmlity::de::Error::custom)?;
            super::DisabledValue::try_from(value).map_err(::xmlity::de::Error::custom)
        }
        pub fn serialize<S>(
            value: &super::DisabledValue,
            serializer: S,
        ) -> ::core::result::Result<S::Ok, S::Error>
        where
            S: ::xmlity::Serializer,
        {
            let value: ::std::string::String = ::core::clone::Clone::clone(value).into();
            ::xmlity::Serialize::serialize(
                ::std::string::String::as_str(
                    &::std::string::ToString::to_string(&value),
                ),
                serializer,
            )
        }
    }
    #[derive(::core::fmt::Debug)]
    pub enum DisabledValueParseError {
        NonExistent { value: ::std::string::String },
    }
    impl ::core::fmt::Display for DisabledValueParseError {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::result::Result<(), ::core::fmt::Error> {
            match self {
                DisabledValueParseError::NonExistent { value } => {
                    write!(f, "Value '{:?}' does not exist in the enumeration", value)
                }
            }
        }
    }
    impl ::core::convert::TryFrom<::std::string::String> for DisabledValue {
        type Error = DisabledValueParseError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::core::result::Result<Self, Self::Error> {
            match ::std::string::String::as_str(&value) {
                "disabled" => Ok(DisabledValue::Disabled),
                _ => {
                    Err(DisabledValueParseError::NonExistent {
                        value,
                    })
                }
            }
        }
    }
    impl ::core::convert::From<DisabledValue> for ::std::string::String {
        fn from(value: DisabledValue) -> Self {
            match value {
                DisabledValue::Disabled => ::std::string::String::from("disabled"),
            }
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::core::clone::Clone,
        ::core::marker::Copy,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq
    )]
    #[xvalue(with = readonly_value_with)]
    pub enum ReadonlyValue {
        Readonly,
    }
    pub mod readonly_value_with {
        pub fn deserialize<'de, D>(
            deserializer: D,
        ) -> ::core::result::Result<super::ReadonlyValue, D::Error>
        where
            D: ::xmlity::Deserializer<'de>,
        {
            let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                deserializer,
            )?;
            let value: ::std::string::String = text
                .parse()
                .map_err(::xmlity::de::Error::custom)?;
            super::ReadonlyValue::try_from(value).map_err(::xmlity::de::Error::custom)
        }
        pub fn serialize<S>(
            value: &super::ReadonlyValue,
            serializer: S,
        ) -> ::core::result::Result<S::Ok, S::Error>
        where
            S: ::xmlity::Serializer,
        {
            let value: ::std::string::String = ::core::clone::Clone::clone(value).into();
            ::xmlity::Serialize::serialize(
                ::std::string::String::as_str(
                    &::std::string::ToString::to_string(&value),
                ),
                serializer,
            )
        }
    }
    #[derive(::core::fmt::Debug)]
    pub enum ReadonlyValueParseError {
        NonExistent { value: ::std::string::String },
    }
    impl ::core::fmt::Display for ReadonlyValueParseError {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::result::Result<(), ::core::fmt::Error> {
            match self {
                ReadonlyValueParseError::NonExistent { value } => {
                    write!(f, "Value '{:?}' does not exist in the enumeration", value)
                }
            }
        }
    }
    impl ::core::convert::TryFrom<::std::string::String> for ReadonlyValue {
        type Error = ReadonlyValueParseError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::core::result::Result<Self, Self::Error> {
            match ::std::string::String::as_str(&value) {
                "readonly" => Ok(ReadonlyValue::Readonly),
                _ => {
                    Err(ReadonlyValueParseError::NonExistent {
                        value,
                    })
                }
            }
        }
    }
    impl ::core::convert::From<ReadonlyValue> for ::std::string::String {
        fn from(value: ReadonlyValue) -> Self {
            match value {
                ReadonlyValue::Readonly => ::std::string::String::from("readonly"),
            }
        }
    }
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
#[xelement(
    name = "input",
    namespace = "http://www.w3.org/1999/xhtml",
    allow_unknown_attributes = "any"
)]
pub struct Input {
    #[xattribute(name = "id", optional, default)]
    pub id: ::core::option::Option<String>,
    #[xattribute(name = "class", optional, default)]
    pub class: ::core::option::Option<String>,
    #[xattribute(name = "style", optional, default)]
    pub style: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::StyleSheet>,
    >,
    #[xattribute(name = "title", optional, default)]
    pub title: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Text>>,
    #[xattribute(name = "lang", optional, default)]
    pub lang: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::LanguageCode>,
    >,
    #[xattribute(deferred = true, optional, default)]
    pub attribute_5: ::core::option::Option<crate::xml::Lang>,
    #[xattribute(name = "dir", optional, default)]
    pub dir: ::core::option::Option<input_items::DirValue>,
    #[xattribute(name = "onclick", optional, default)]
    pub onclick: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xattribute(name = "ondblclick", optional, default)]
    pub ondblclick: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmousedown", optional, default)]
    pub onmousedown: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseup", optional, default)]
    pub onmouseup: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseover", optional, default)]
    pub onmouseover: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmousemove", optional, default)]
    pub onmousemove: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseout", optional, default)]
    pub onmouseout: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeypress", optional, default)]
    pub onkeypress: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeydown", optional, default)]
    pub onkeydown: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeyup", optional, default)]
    pub onkeyup: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xattribute(name = "accesskey", optional, default)]
    pub accesskey: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Character>,
    >,
    #[xattribute(name = "tabindex", optional, default)]
    pub tabindex: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::TabindexNumber>,
    >,
    #[xattribute(name = "onfocus", optional, default)]
    pub onfocus: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xattribute(name = "onblur", optional, default)]
    pub onblur: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xattribute(name = "type", optional, default)]
    pub type_: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::InputType>>,
    #[xattribute(name = "name", optional, default)]
    pub name: ::core::option::Option<String>,
    #[xattribute(name = "value", optional, default)]
    pub value: ::core::option::Option<String>,
    #[xattribute(name = "checked", optional, default)]
    pub checked: ::core::option::Option<input_items::CheckedValue>,
    #[xattribute(name = "disabled", optional, default)]
    pub disabled: ::core::option::Option<input_items::DisabledValue>,
    #[xattribute(name = "readonly", optional, default)]
    pub readonly: ::core::option::Option<input_items::ReadonlyValue>,
    #[xattribute(name = "size", optional, default)]
    pub size: ::core::option::Option<String>,
    #[xattribute(name = "maxlength", optional, default)]
    pub maxlength: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Number>,
    >,
    #[xattribute(name = "src", optional, default)]
    pub src: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Uri>>,
    #[xattribute(name = "alt", optional, default)]
    pub alt: ::core::option::Option<String>,
    #[xattribute(name = "usemap", optional, default)]
    pub usemap: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Uri>>,
    #[xattribute(name = "onselect", optional, default)]
    pub onselect: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xattribute(name = "onchange", optional, default)]
    pub onchange: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xattribute(name = "accept", optional, default)]
    pub accept: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::ContentTypes>,
    >,
}
pub mod ins_items {
    impl ::core::convert::From<crate::xhtml::groups::Block> for Ins {
        fn from(value: crate::xhtml::groups::Block) -> Self {
            Ins::Block(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml::Form> for Ins {
        fn from(value: crate::xhtml::Form) -> Self {
            Ins::Form(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml::groups::Inline> for Ins {
        fn from(value: crate::xhtml::groups::Inline) -> Self {
            Ins::Inline(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml::groups::Misc> for Ins {
        fn from(value: crate::xhtml::groups::Misc) -> Self {
            Ins::Misc(::std::boxed::Box::new(value))
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub enum Ins {
        Block(::std::boxed::Box<crate::xhtml::groups::Block>),
        Form(::std::boxed::Box<crate::xhtml::Form>),
        Inline(::std::boxed::Box<crate::xhtml::groups::Inline>),
        Misc(::std::boxed::Box<crate::xhtml::groups::Misc>),
    }
    #[derive(
        ::core::fmt::Debug,
        ::core::clone::Clone,
        ::core::marker::Copy,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq
    )]
    #[xvalue(with = dir_value_with)]
    pub enum DirValue {
        Ltr,
        Rtl,
    }
    pub mod dir_value_with {
        pub fn deserialize<'de, D>(
            deserializer: D,
        ) -> ::core::result::Result<super::DirValue, D::Error>
        where
            D: ::xmlity::Deserializer<'de>,
        {
            let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                deserializer,
            )?;
            let value: ::std::string::String = text
                .parse()
                .map_err(::xmlity::de::Error::custom)?;
            super::DirValue::try_from(value).map_err(::xmlity::de::Error::custom)
        }
        pub fn serialize<S>(
            value: &super::DirValue,
            serializer: S,
        ) -> ::core::result::Result<S::Ok, S::Error>
        where
            S: ::xmlity::Serializer,
        {
            let value: ::std::string::String = ::core::clone::Clone::clone(value).into();
            ::xmlity::Serialize::serialize(
                ::std::string::String::as_str(
                    &::std::string::ToString::to_string(&value),
                ),
                serializer,
            )
        }
    }
    #[derive(::core::fmt::Debug)]
    pub enum DirValueParseError {
        NonExistent { value: ::std::string::String },
    }
    impl ::core::fmt::Display for DirValueParseError {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::result::Result<(), ::core::fmt::Error> {
            match self {
                DirValueParseError::NonExistent { value } => {
                    write!(f, "Value '{:?}' does not exist in the enumeration", value)
                }
            }
        }
    }
    impl ::core::convert::TryFrom<::std::string::String> for DirValue {
        type Error = DirValueParseError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::core::result::Result<Self, Self::Error> {
            match ::std::string::String::as_str(&value) {
                "ltr" => Ok(DirValue::Ltr),
                "rtl" => Ok(DirValue::Rtl),
                _ => {
                    Err(DirValueParseError::NonExistent {
                        value,
                    })
                }
            }
        }
    }
    impl ::core::convert::From<DirValue> for ::std::string::String {
        fn from(value: DirValue) -> Self {
            match value {
                DirValue::Ltr => ::std::string::String::from("ltr"),
                DirValue::Rtl => ::std::string::String::from("rtl"),
            }
        }
    }
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
#[xelement(
    name = "ins",
    namespace = "http://www.w3.org/1999/xhtml",
    allow_unknown_attributes = "any"
)]
pub struct Ins {
    #[xattribute(name = "id", optional, default)]
    pub id: ::core::option::Option<String>,
    #[xattribute(name = "class", optional, default)]
    pub class: ::core::option::Option<String>,
    #[xattribute(name = "style", optional, default)]
    pub style: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::StyleSheet>,
    >,
    #[xattribute(name = "title", optional, default)]
    pub title: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Text>>,
    #[xattribute(name = "lang", optional, default)]
    pub lang: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::LanguageCode>,
    >,
    #[xattribute(deferred = true, optional, default)]
    pub attribute_5: ::core::option::Option<crate::xml::Lang>,
    #[xattribute(name = "dir", optional, default)]
    pub dir: ::core::option::Option<ins_items::DirValue>,
    #[xattribute(name = "onclick", optional, default)]
    pub onclick: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xattribute(name = "ondblclick", optional, default)]
    pub ondblclick: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmousedown", optional, default)]
    pub onmousedown: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseup", optional, default)]
    pub onmouseup: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseover", optional, default)]
    pub onmouseover: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmousemove", optional, default)]
    pub onmousemove: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseout", optional, default)]
    pub onmouseout: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeypress", optional, default)]
    pub onkeypress: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeydown", optional, default)]
    pub onkeydown: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeyup", optional, default)]
    pub onkeyup: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xattribute(name = "cite", optional, default)]
    pub cite: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Uri>>,
    #[xattribute(name = "datetime", optional, default)]
    pub datetime: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Datetime>,
    >,
    #[xvalue(default)]
    pub ins: ::std::vec::Vec<ins_items::Ins>,
}
pub mod kbd_items {
    impl ::core::convert::From<crate::xhtml::groups::Inline> for Inline {
        fn from(value: crate::xhtml::groups::Inline) -> Self {
            Inline::Inline(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml::groups::MiscInline> for Inline {
        fn from(value: crate::xhtml::groups::MiscInline) -> Self {
            Inline::MiscInline(::std::boxed::Box::new(value))
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub enum Inline {
        Inline(::std::boxed::Box<crate::xhtml::groups::Inline>),
        MiscInline(::std::boxed::Box<crate::xhtml::groups::MiscInline>),
    }
    #[derive(
        ::core::fmt::Debug,
        ::core::clone::Clone,
        ::core::marker::Copy,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq
    )]
    #[xvalue(with = dir_value_with)]
    pub enum DirValue {
        Ltr,
        Rtl,
    }
    pub mod dir_value_with {
        pub fn deserialize<'de, D>(
            deserializer: D,
        ) -> ::core::result::Result<super::DirValue, D::Error>
        where
            D: ::xmlity::Deserializer<'de>,
        {
            let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                deserializer,
            )?;
            let value: ::std::string::String = text
                .parse()
                .map_err(::xmlity::de::Error::custom)?;
            super::DirValue::try_from(value).map_err(::xmlity::de::Error::custom)
        }
        pub fn serialize<S>(
            value: &super::DirValue,
            serializer: S,
        ) -> ::core::result::Result<S::Ok, S::Error>
        where
            S: ::xmlity::Serializer,
        {
            let value: ::std::string::String = ::core::clone::Clone::clone(value).into();
            ::xmlity::Serialize::serialize(
                ::std::string::String::as_str(
                    &::std::string::ToString::to_string(&value),
                ),
                serializer,
            )
        }
    }
    #[derive(::core::fmt::Debug)]
    pub enum DirValueParseError {
        NonExistent { value: ::std::string::String },
    }
    impl ::core::fmt::Display for DirValueParseError {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::result::Result<(), ::core::fmt::Error> {
            match self {
                DirValueParseError::NonExistent { value } => {
                    write!(f, "Value '{:?}' does not exist in the enumeration", value)
                }
            }
        }
    }
    impl ::core::convert::TryFrom<::std::string::String> for DirValue {
        type Error = DirValueParseError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::core::result::Result<Self, Self::Error> {
            match ::std::string::String::as_str(&value) {
                "ltr" => Ok(DirValue::Ltr),
                "rtl" => Ok(DirValue::Rtl),
                _ => {
                    Err(DirValueParseError::NonExistent {
                        value,
                    })
                }
            }
        }
    }
    impl ::core::convert::From<DirValue> for ::std::string::String {
        fn from(value: DirValue) -> Self {
            match value {
                DirValue::Ltr => ::std::string::String::from("ltr"),
                DirValue::Rtl => ::std::string::String::from("rtl"),
            }
        }
    }
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
#[xelement(
    name = "kbd",
    namespace = "http://www.w3.org/1999/xhtml",
    allow_unknown_attributes = "any"
)]
pub struct Kbd {
    #[xattribute(name = "id", optional, default)]
    pub id: ::core::option::Option<String>,
    #[xattribute(name = "class", optional, default)]
    pub class: ::core::option::Option<String>,
    #[xattribute(name = "style", optional, default)]
    pub style: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::StyleSheet>,
    >,
    #[xattribute(name = "title", optional, default)]
    pub title: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Text>>,
    #[xattribute(name = "lang", optional, default)]
    pub lang: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::LanguageCode>,
    >,
    #[xattribute(deferred = true, optional, default)]
    pub attribute_5: ::core::option::Option<crate::xml::Lang>,
    #[xattribute(name = "dir", optional, default)]
    pub dir: ::core::option::Option<kbd_items::DirValue>,
    #[xattribute(name = "onclick", optional, default)]
    pub onclick: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xattribute(name = "ondblclick", optional, default)]
    pub ondblclick: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmousedown", optional, default)]
    pub onmousedown: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseup", optional, default)]
    pub onmouseup: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseover", optional, default)]
    pub onmouseover: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmousemove", optional, default)]
    pub onmousemove: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseout", optional, default)]
    pub onmouseout: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeypress", optional, default)]
    pub onkeypress: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeydown", optional, default)]
    pub onkeydown: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeyup", optional, default)]
    pub onkeyup: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xvalue(default)]
    pub inline: ::std::vec::Vec<kbd_items::Inline>,
}
pub mod label_items {
    impl ::core::convert::From<crate::xhtml::groups::Inline> for Inline {
        fn from(value: crate::xhtml::groups::Inline) -> Self {
            Inline::Inline(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml::groups::MiscInline> for Inline {
        fn from(value: crate::xhtml::groups::MiscInline) -> Self {
            Inline::MiscInline(::std::boxed::Box::new(value))
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub enum Inline {
        Inline(::std::boxed::Box<crate::xhtml::groups::Inline>),
        MiscInline(::std::boxed::Box<crate::xhtml::groups::MiscInline>),
    }
    #[derive(
        ::core::fmt::Debug,
        ::core::clone::Clone,
        ::core::marker::Copy,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq
    )]
    #[xvalue(with = dir_value_with)]
    pub enum DirValue {
        Ltr,
        Rtl,
    }
    pub mod dir_value_with {
        pub fn deserialize<'de, D>(
            deserializer: D,
        ) -> ::core::result::Result<super::DirValue, D::Error>
        where
            D: ::xmlity::Deserializer<'de>,
        {
            let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                deserializer,
            )?;
            let value: ::std::string::String = text
                .parse()
                .map_err(::xmlity::de::Error::custom)?;
            super::DirValue::try_from(value).map_err(::xmlity::de::Error::custom)
        }
        pub fn serialize<S>(
            value: &super::DirValue,
            serializer: S,
        ) -> ::core::result::Result<S::Ok, S::Error>
        where
            S: ::xmlity::Serializer,
        {
            let value: ::std::string::String = ::core::clone::Clone::clone(value).into();
            ::xmlity::Serialize::serialize(
                ::std::string::String::as_str(
                    &::std::string::ToString::to_string(&value),
                ),
                serializer,
            )
        }
    }
    #[derive(::core::fmt::Debug)]
    pub enum DirValueParseError {
        NonExistent { value: ::std::string::String },
    }
    impl ::core::fmt::Display for DirValueParseError {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::result::Result<(), ::core::fmt::Error> {
            match self {
                DirValueParseError::NonExistent { value } => {
                    write!(f, "Value '{:?}' does not exist in the enumeration", value)
                }
            }
        }
    }
    impl ::core::convert::TryFrom<::std::string::String> for DirValue {
        type Error = DirValueParseError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::core::result::Result<Self, Self::Error> {
            match ::std::string::String::as_str(&value) {
                "ltr" => Ok(DirValue::Ltr),
                "rtl" => Ok(DirValue::Rtl),
                _ => {
                    Err(DirValueParseError::NonExistent {
                        value,
                    })
                }
            }
        }
    }
    impl ::core::convert::From<DirValue> for ::std::string::String {
        fn from(value: DirValue) -> Self {
            match value {
                DirValue::Ltr => ::std::string::String::from("ltr"),
                DirValue::Rtl => ::std::string::String::from("rtl"),
            }
        }
    }
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
#[xelement(
    name = "label",
    namespace = "http://www.w3.org/1999/xhtml",
    allow_unknown_attributes = "any"
)]
pub struct Label {
    #[xattribute(name = "id", optional, default)]
    pub id: ::core::option::Option<String>,
    #[xattribute(name = "class", optional, default)]
    pub class: ::core::option::Option<String>,
    #[xattribute(name = "style", optional, default)]
    pub style: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::StyleSheet>,
    >,
    #[xattribute(name = "title", optional, default)]
    pub title: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Text>>,
    #[xattribute(name = "lang", optional, default)]
    pub lang: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::LanguageCode>,
    >,
    #[xattribute(deferred = true, optional, default)]
    pub attribute_5: ::core::option::Option<crate::xml::Lang>,
    #[xattribute(name = "dir", optional, default)]
    pub dir: ::core::option::Option<label_items::DirValue>,
    #[xattribute(name = "onclick", optional, default)]
    pub onclick: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xattribute(name = "ondblclick", optional, default)]
    pub ondblclick: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmousedown", optional, default)]
    pub onmousedown: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseup", optional, default)]
    pub onmouseup: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseover", optional, default)]
    pub onmouseover: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmousemove", optional, default)]
    pub onmousemove: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseout", optional, default)]
    pub onmouseout: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeypress", optional, default)]
    pub onkeypress: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeydown", optional, default)]
    pub onkeydown: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeyup", optional, default)]
    pub onkeyup: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xattribute(name = "for", optional, default)]
    pub for_: ::core::option::Option<String>,
    #[xattribute(name = "accesskey", optional, default)]
    pub accesskey: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Character>,
    >,
    #[xattribute(name = "onfocus", optional, default)]
    pub onfocus: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xattribute(name = "onblur", optional, default)]
    pub onblur: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xvalue(default)]
    pub inline: ::std::vec::Vec<label_items::Inline>,
}
pub mod legend_items {
    impl ::core::convert::From<crate::xhtml::groups::Inline> for Inline {
        fn from(value: crate::xhtml::groups::Inline) -> Self {
            Inline::Inline(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml::groups::MiscInline> for Inline {
        fn from(value: crate::xhtml::groups::MiscInline) -> Self {
            Inline::MiscInline(::std::boxed::Box::new(value))
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub enum Inline {
        Inline(::std::boxed::Box<crate::xhtml::groups::Inline>),
        MiscInline(::std::boxed::Box<crate::xhtml::groups::MiscInline>),
    }
    #[derive(
        ::core::fmt::Debug,
        ::core::clone::Clone,
        ::core::marker::Copy,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq
    )]
    #[xvalue(with = dir_value_with)]
    pub enum DirValue {
        Ltr,
        Rtl,
    }
    pub mod dir_value_with {
        pub fn deserialize<'de, D>(
            deserializer: D,
        ) -> ::core::result::Result<super::DirValue, D::Error>
        where
            D: ::xmlity::Deserializer<'de>,
        {
            let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                deserializer,
            )?;
            let value: ::std::string::String = text
                .parse()
                .map_err(::xmlity::de::Error::custom)?;
            super::DirValue::try_from(value).map_err(::xmlity::de::Error::custom)
        }
        pub fn serialize<S>(
            value: &super::DirValue,
            serializer: S,
        ) -> ::core::result::Result<S::Ok, S::Error>
        where
            S: ::xmlity::Serializer,
        {
            let value: ::std::string::String = ::core::clone::Clone::clone(value).into();
            ::xmlity::Serialize::serialize(
                ::std::string::String::as_str(
                    &::std::string::ToString::to_string(&value),
                ),
                serializer,
            )
        }
    }
    #[derive(::core::fmt::Debug)]
    pub enum DirValueParseError {
        NonExistent { value: ::std::string::String },
    }
    impl ::core::fmt::Display for DirValueParseError {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::result::Result<(), ::core::fmt::Error> {
            match self {
                DirValueParseError::NonExistent { value } => {
                    write!(f, "Value '{:?}' does not exist in the enumeration", value)
                }
            }
        }
    }
    impl ::core::convert::TryFrom<::std::string::String> for DirValue {
        type Error = DirValueParseError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::core::result::Result<Self, Self::Error> {
            match ::std::string::String::as_str(&value) {
                "ltr" => Ok(DirValue::Ltr),
                "rtl" => Ok(DirValue::Rtl),
                _ => {
                    Err(DirValueParseError::NonExistent {
                        value,
                    })
                }
            }
        }
    }
    impl ::core::convert::From<DirValue> for ::std::string::String {
        fn from(value: DirValue) -> Self {
            match value {
                DirValue::Ltr => ::std::string::String::from("ltr"),
                DirValue::Rtl => ::std::string::String::from("rtl"),
            }
        }
    }
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
#[xelement(
    name = "legend",
    namespace = "http://www.w3.org/1999/xhtml",
    allow_unknown_attributes = "any"
)]
pub struct Legend {
    #[xattribute(name = "id", optional, default)]
    pub id: ::core::option::Option<String>,
    #[xattribute(name = "class", optional, default)]
    pub class: ::core::option::Option<String>,
    #[xattribute(name = "style", optional, default)]
    pub style: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::StyleSheet>,
    >,
    #[xattribute(name = "title", optional, default)]
    pub title: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Text>>,
    #[xattribute(name = "lang", optional, default)]
    pub lang: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::LanguageCode>,
    >,
    #[xattribute(deferred = true, optional, default)]
    pub attribute_5: ::core::option::Option<crate::xml::Lang>,
    #[xattribute(name = "dir", optional, default)]
    pub dir: ::core::option::Option<legend_items::DirValue>,
    #[xattribute(name = "onclick", optional, default)]
    pub onclick: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xattribute(name = "ondblclick", optional, default)]
    pub ondblclick: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmousedown", optional, default)]
    pub onmousedown: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseup", optional, default)]
    pub onmouseup: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseover", optional, default)]
    pub onmouseover: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmousemove", optional, default)]
    pub onmousemove: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseout", optional, default)]
    pub onmouseout: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeypress", optional, default)]
    pub onkeypress: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeydown", optional, default)]
    pub onkeydown: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeyup", optional, default)]
    pub onkeyup: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xattribute(name = "accesskey", optional, default)]
    pub accesskey: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Character>,
    >,
    #[xvalue(default)]
    pub inline: ::std::vec::Vec<legend_items::Inline>,
}
pub mod li_items {
    impl ::core::convert::From<crate::xhtml::groups::Block> for Li {
        fn from(value: crate::xhtml::groups::Block) -> Self {
            Li::Block(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml::Form> for Li {
        fn from(value: crate::xhtml::Form) -> Self {
            Li::Form(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml::groups::Inline> for Li {
        fn from(value: crate::xhtml::groups::Inline) -> Self {
            Li::Inline(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml::groups::Misc> for Li {
        fn from(value: crate::xhtml::groups::Misc) -> Self {
            Li::Misc(::std::boxed::Box::new(value))
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub enum Li {
        Block(::std::boxed::Box<crate::xhtml::groups::Block>),
        Form(::std::boxed::Box<crate::xhtml::Form>),
        Inline(::std::boxed::Box<crate::xhtml::groups::Inline>),
        Misc(::std::boxed::Box<crate::xhtml::groups::Misc>),
    }
    #[derive(
        ::core::fmt::Debug,
        ::core::clone::Clone,
        ::core::marker::Copy,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq
    )]
    #[xvalue(with = dir_value_with)]
    pub enum DirValue {
        Ltr,
        Rtl,
    }
    pub mod dir_value_with {
        pub fn deserialize<'de, D>(
            deserializer: D,
        ) -> ::core::result::Result<super::DirValue, D::Error>
        where
            D: ::xmlity::Deserializer<'de>,
        {
            let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                deserializer,
            )?;
            let value: ::std::string::String = text
                .parse()
                .map_err(::xmlity::de::Error::custom)?;
            super::DirValue::try_from(value).map_err(::xmlity::de::Error::custom)
        }
        pub fn serialize<S>(
            value: &super::DirValue,
            serializer: S,
        ) -> ::core::result::Result<S::Ok, S::Error>
        where
            S: ::xmlity::Serializer,
        {
            let value: ::std::string::String = ::core::clone::Clone::clone(value).into();
            ::xmlity::Serialize::serialize(
                ::std::string::String::as_str(
                    &::std::string::ToString::to_string(&value),
                ),
                serializer,
            )
        }
    }
    #[derive(::core::fmt::Debug)]
    pub enum DirValueParseError {
        NonExistent { value: ::std::string::String },
    }
    impl ::core::fmt::Display for DirValueParseError {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::result::Result<(), ::core::fmt::Error> {
            match self {
                DirValueParseError::NonExistent { value } => {
                    write!(f, "Value '{:?}' does not exist in the enumeration", value)
                }
            }
        }
    }
    impl ::core::convert::TryFrom<::std::string::String> for DirValue {
        type Error = DirValueParseError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::core::result::Result<Self, Self::Error> {
            match ::std::string::String::as_str(&value) {
                "ltr" => Ok(DirValue::Ltr),
                "rtl" => Ok(DirValue::Rtl),
                _ => {
                    Err(DirValueParseError::NonExistent {
                        value,
                    })
                }
            }
        }
    }
    impl ::core::convert::From<DirValue> for ::std::string::String {
        fn from(value: DirValue) -> Self {
            match value {
                DirValue::Ltr => ::std::string::String::from("ltr"),
                DirValue::Rtl => ::std::string::String::from("rtl"),
            }
        }
    }
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
#[xelement(
    name = "li",
    namespace = "http://www.w3.org/1999/xhtml",
    allow_unknown_attributes = "any"
)]
pub struct Li {
    #[xattribute(name = "id", optional, default)]
    pub id: ::core::option::Option<String>,
    #[xattribute(name = "class", optional, default)]
    pub class: ::core::option::Option<String>,
    #[xattribute(name = "style", optional, default)]
    pub style: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::StyleSheet>,
    >,
    #[xattribute(name = "title", optional, default)]
    pub title: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Text>>,
    #[xattribute(name = "lang", optional, default)]
    pub lang: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::LanguageCode>,
    >,
    #[xattribute(deferred = true, optional, default)]
    pub attribute_5: ::core::option::Option<crate::xml::Lang>,
    #[xattribute(name = "dir", optional, default)]
    pub dir: ::core::option::Option<li_items::DirValue>,
    #[xattribute(name = "onclick", optional, default)]
    pub onclick: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xattribute(name = "ondblclick", optional, default)]
    pub ondblclick: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmousedown", optional, default)]
    pub onmousedown: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseup", optional, default)]
    pub onmouseup: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseover", optional, default)]
    pub onmouseover: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmousemove", optional, default)]
    pub onmousemove: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseout", optional, default)]
    pub onmouseout: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeypress", optional, default)]
    pub onkeypress: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeydown", optional, default)]
    pub onkeydown: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeyup", optional, default)]
    pub onkeyup: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xvalue(default)]
    pub li: ::std::vec::Vec<li_items::Li>,
}
pub mod link_items {
    #[derive(
        ::core::fmt::Debug,
        ::core::clone::Clone,
        ::core::marker::Copy,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq
    )]
    #[xvalue(with = dir_value_with)]
    pub enum DirValue {
        Ltr,
        Rtl,
    }
    pub mod dir_value_with {
        pub fn deserialize<'de, D>(
            deserializer: D,
        ) -> ::core::result::Result<super::DirValue, D::Error>
        where
            D: ::xmlity::Deserializer<'de>,
        {
            let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                deserializer,
            )?;
            let value: ::std::string::String = text
                .parse()
                .map_err(::xmlity::de::Error::custom)?;
            super::DirValue::try_from(value).map_err(::xmlity::de::Error::custom)
        }
        pub fn serialize<S>(
            value: &super::DirValue,
            serializer: S,
        ) -> ::core::result::Result<S::Ok, S::Error>
        where
            S: ::xmlity::Serializer,
        {
            let value: ::std::string::String = ::core::clone::Clone::clone(value).into();
            ::xmlity::Serialize::serialize(
                ::std::string::String::as_str(
                    &::std::string::ToString::to_string(&value),
                ),
                serializer,
            )
        }
    }
    #[derive(::core::fmt::Debug)]
    pub enum DirValueParseError {
        NonExistent { value: ::std::string::String },
    }
    impl ::core::fmt::Display for DirValueParseError {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::result::Result<(), ::core::fmt::Error> {
            match self {
                DirValueParseError::NonExistent { value } => {
                    write!(f, "Value '{:?}' does not exist in the enumeration", value)
                }
            }
        }
    }
    impl ::core::convert::TryFrom<::std::string::String> for DirValue {
        type Error = DirValueParseError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::core::result::Result<Self, Self::Error> {
            match ::std::string::String::as_str(&value) {
                "ltr" => Ok(DirValue::Ltr),
                "rtl" => Ok(DirValue::Rtl),
                _ => {
                    Err(DirValueParseError::NonExistent {
                        value,
                    })
                }
            }
        }
    }
    impl ::core::convert::From<DirValue> for ::std::string::String {
        fn from(value: DirValue) -> Self {
            match value {
                DirValue::Ltr => ::std::string::String::from("ltr"),
                DirValue::Rtl => ::std::string::String::from("rtl"),
            }
        }
    }
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
#[xelement(
    name = "link",
    namespace = "http://www.w3.org/1999/xhtml",
    allow_unknown_attributes = "any"
)]
pub struct Link {
    #[xattribute(name = "id", optional, default)]
    pub id: ::core::option::Option<String>,
    #[xattribute(name = "class", optional, default)]
    pub class: ::core::option::Option<String>,
    #[xattribute(name = "style", optional, default)]
    pub style: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::StyleSheet>,
    >,
    #[xattribute(name = "title", optional, default)]
    pub title: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Text>>,
    #[xattribute(name = "lang", optional, default)]
    pub lang: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::LanguageCode>,
    >,
    #[xattribute(deferred = true, optional, default)]
    pub attribute_5: ::core::option::Option<crate::xml::Lang>,
    #[xattribute(name = "dir", optional, default)]
    pub dir: ::core::option::Option<link_items::DirValue>,
    #[xattribute(name = "onclick", optional, default)]
    pub onclick: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xattribute(name = "ondblclick", optional, default)]
    pub ondblclick: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmousedown", optional, default)]
    pub onmousedown: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseup", optional, default)]
    pub onmouseup: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseover", optional, default)]
    pub onmouseover: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmousemove", optional, default)]
    pub onmousemove: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseout", optional, default)]
    pub onmouseout: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeypress", optional, default)]
    pub onkeypress: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeydown", optional, default)]
    pub onkeydown: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeyup", optional, default)]
    pub onkeyup: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xattribute(name = "charset", optional, default)]
    pub charset: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Charset>>,
    #[xattribute(name = "href", optional, default)]
    pub href: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Uri>>,
    #[xattribute(name = "hreflang", optional, default)]
    pub hreflang: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::LanguageCode>,
    >,
    #[xattribute(name = "type", optional, default)]
    pub type_: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::ContentType>,
    >,
    #[xattribute(name = "rel", optional, default)]
    pub rel: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::LinkTypes>>,
    #[xattribute(name = "rev", optional, default)]
    pub rev: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::LinkTypes>>,
    #[xattribute(name = "media", optional, default)]
    pub media: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::MediaDesc>>,
}
pub mod map_items {
    pub mod map_variants {
        impl ::core::convert::From<crate::xhtml::groups::Block> for Variant0 {
            fn from(value: crate::xhtml::groups::Block) -> Self {
                Variant0::Block(::std::boxed::Box::new(value))
            }
        }
        impl ::core::convert::From<crate::xhtml::Form> for Variant0 {
            fn from(value: crate::xhtml::Form) -> Self {
                Variant0::Form(::std::boxed::Box::new(value))
            }
        }
        impl ::core::convert::From<crate::xhtml::groups::Misc> for Variant0 {
            fn from(value: crate::xhtml::groups::Misc) -> Self {
                Variant0::Misc(::std::boxed::Box::new(value))
            }
        }
        #[derive(
            ::core::fmt::Debug,
            ::xmlity::Serialize,
            ::xmlity::Deserialize,
            ::core::cmp::PartialEq,
            ::core::clone::Clone
        )]
        pub enum Variant0 {
            Block(::std::boxed::Box<crate::xhtml::groups::Block>),
            Form(::std::boxed::Box<crate::xhtml::Form>),
            Misc(::std::boxed::Box<crate::xhtml::groups::Misc>),
        }
    }
    impl ::core::convert::From<::std::vec::Vec<map_variants::Variant0>> for Map {
        fn from(value: ::std::vec::Vec<map_variants::Variant0>) -> Self {
            Map::Variant0(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<::std::vec::Vec<crate::xhtml::Area>> for Map {
        fn from(value: ::std::vec::Vec<crate::xhtml::Area>) -> Self {
            Map::Area(::std::boxed::Box::new(value))
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub enum Map {
        Variant0(
            #[xvalue(default)]
            ::std::boxed::Box<::std::vec::Vec<map_variants::Variant0>>,
        ),
        Area(#[xvalue(default)] ::std::boxed::Box<::std::vec::Vec<crate::xhtml::Area>>),
    }
    #[derive(
        ::core::fmt::Debug,
        ::core::clone::Clone,
        ::core::marker::Copy,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq
    )]
    #[xvalue(with = dir_value_with)]
    pub enum DirValue {
        Ltr,
        Rtl,
    }
    pub mod dir_value_with {
        pub fn deserialize<'de, D>(
            deserializer: D,
        ) -> ::core::result::Result<super::DirValue, D::Error>
        where
            D: ::xmlity::Deserializer<'de>,
        {
            let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                deserializer,
            )?;
            let value: ::std::string::String = text
                .parse()
                .map_err(::xmlity::de::Error::custom)?;
            super::DirValue::try_from(value).map_err(::xmlity::de::Error::custom)
        }
        pub fn serialize<S>(
            value: &super::DirValue,
            serializer: S,
        ) -> ::core::result::Result<S::Ok, S::Error>
        where
            S: ::xmlity::Serializer,
        {
            let value: ::std::string::String = ::core::clone::Clone::clone(value).into();
            ::xmlity::Serialize::serialize(
                ::std::string::String::as_str(
                    &::std::string::ToString::to_string(&value),
                ),
                serializer,
            )
        }
    }
    #[derive(::core::fmt::Debug)]
    pub enum DirValueParseError {
        NonExistent { value: ::std::string::String },
    }
    impl ::core::fmt::Display for DirValueParseError {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::result::Result<(), ::core::fmt::Error> {
            match self {
                DirValueParseError::NonExistent { value } => {
                    write!(f, "Value '{:?}' does not exist in the enumeration", value)
                }
            }
        }
    }
    impl ::core::convert::TryFrom<::std::string::String> for DirValue {
        type Error = DirValueParseError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::core::result::Result<Self, Self::Error> {
            match ::std::string::String::as_str(&value) {
                "ltr" => Ok(DirValue::Ltr),
                "rtl" => Ok(DirValue::Rtl),
                _ => {
                    Err(DirValueParseError::NonExistent {
                        value,
                    })
                }
            }
        }
    }
    impl ::core::convert::From<DirValue> for ::std::string::String {
        fn from(value: DirValue) -> Self {
            match value {
                DirValue::Ltr => ::std::string::String::from("ltr"),
                DirValue::Rtl => ::std::string::String::from("rtl"),
            }
        }
    }
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
#[xelement(
    name = "map",
    namespace = "http://www.w3.org/1999/xhtml",
    allow_unknown_attributes = "any"
)]
pub struct Map {
    #[xattribute(name = "lang", optional, default)]
    pub lang: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::LanguageCode>,
    >,
    #[xattribute(deferred = true, optional, default)]
    pub attribute_1: ::core::option::Option<crate::xml::Lang>,
    #[xattribute(name = "dir", optional, default)]
    pub dir: ::core::option::Option<map_items::DirValue>,
    #[xattribute(name = "onclick", optional, default)]
    pub onclick: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xattribute(name = "ondblclick", optional, default)]
    pub ondblclick: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmousedown", optional, default)]
    pub onmousedown: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseup", optional, default)]
    pub onmouseup: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseover", optional, default)]
    pub onmouseover: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmousemove", optional, default)]
    pub onmousemove: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseout", optional, default)]
    pub onmouseout: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeypress", optional, default)]
    pub onkeypress: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeydown", optional, default)]
    pub onkeydown: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeyup", optional, default)]
    pub onkeyup: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xattribute(name = "id")]
    pub id: String,
    #[xattribute(name = "class", optional, default)]
    pub class: ::core::option::Option<String>,
    #[xattribute(name = "style", optional, default)]
    pub style: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::StyleSheet>,
    >,
    #[xattribute(name = "title", optional, default)]
    pub title: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Text>>,
    #[xattribute(name = "name", optional, default)]
    pub name: ::core::option::Option<String>,
    pub map: map_items::Map,
}
pub mod meta_items {
    #[derive(
        ::core::fmt::Debug,
        ::core::clone::Clone,
        ::core::marker::Copy,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq
    )]
    #[xvalue(with = dir_value_with)]
    pub enum DirValue {
        Ltr,
        Rtl,
    }
    pub mod dir_value_with {
        pub fn deserialize<'de, D>(
            deserializer: D,
        ) -> ::core::result::Result<super::DirValue, D::Error>
        where
            D: ::xmlity::Deserializer<'de>,
        {
            let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                deserializer,
            )?;
            let value: ::std::string::String = text
                .parse()
                .map_err(::xmlity::de::Error::custom)?;
            super::DirValue::try_from(value).map_err(::xmlity::de::Error::custom)
        }
        pub fn serialize<S>(
            value: &super::DirValue,
            serializer: S,
        ) -> ::core::result::Result<S::Ok, S::Error>
        where
            S: ::xmlity::Serializer,
        {
            let value: ::std::string::String = ::core::clone::Clone::clone(value).into();
            ::xmlity::Serialize::serialize(
                ::std::string::String::as_str(
                    &::std::string::ToString::to_string(&value),
                ),
                serializer,
            )
        }
    }
    #[derive(::core::fmt::Debug)]
    pub enum DirValueParseError {
        NonExistent { value: ::std::string::String },
    }
    impl ::core::fmt::Display for DirValueParseError {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::result::Result<(), ::core::fmt::Error> {
            match self {
                DirValueParseError::NonExistent { value } => {
                    write!(f, "Value '{:?}' does not exist in the enumeration", value)
                }
            }
        }
    }
    impl ::core::convert::TryFrom<::std::string::String> for DirValue {
        type Error = DirValueParseError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::core::result::Result<Self, Self::Error> {
            match ::std::string::String::as_str(&value) {
                "ltr" => Ok(DirValue::Ltr),
                "rtl" => Ok(DirValue::Rtl),
                _ => {
                    Err(DirValueParseError::NonExistent {
                        value,
                    })
                }
            }
        }
    }
    impl ::core::convert::From<DirValue> for ::std::string::String {
        fn from(value: DirValue) -> Self {
            match value {
                DirValue::Ltr => ::std::string::String::from("ltr"),
                DirValue::Rtl => ::std::string::String::from("rtl"),
            }
        }
    }
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
#[xelement(
    name = "meta",
    namespace = "http://www.w3.org/1999/xhtml",
    allow_unknown_attributes = "any"
)]
pub struct Meta {
    #[xattribute(name = "lang", optional, default)]
    pub lang: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::LanguageCode>,
    >,
    #[xattribute(deferred = true, optional, default)]
    pub attribute_1: ::core::option::Option<crate::xml::Lang>,
    #[xattribute(name = "dir", optional, default)]
    pub dir: ::core::option::Option<meta_items::DirValue>,
    #[xattribute(name = "id", optional, default)]
    pub id: ::core::option::Option<String>,
    #[xattribute(name = "http-equiv", optional, default)]
    pub http_equiv: ::core::option::Option<String>,
    #[xattribute(name = "name", optional, default)]
    pub name: ::core::option::Option<String>,
    #[xattribute(name = "content")]
    pub content: String,
    #[xattribute(name = "scheme", optional, default)]
    pub scheme: ::core::option::Option<String>,
}
pub mod noscript_items {
    impl ::core::convert::From<crate::xhtml::groups::Block> for Noscript {
        fn from(value: crate::xhtml::groups::Block) -> Self {
            Noscript::Block(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml::Form> for Noscript {
        fn from(value: crate::xhtml::Form) -> Self {
            Noscript::Form(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml::groups::Misc> for Noscript {
        fn from(value: crate::xhtml::groups::Misc) -> Self {
            Noscript::Misc(::std::boxed::Box::new(value))
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub enum Noscript {
        Block(::std::boxed::Box<crate::xhtml::groups::Block>),
        Form(::std::boxed::Box<crate::xhtml::Form>),
        Misc(::std::boxed::Box<crate::xhtml::groups::Misc>),
    }
    #[derive(
        ::core::fmt::Debug,
        ::core::clone::Clone,
        ::core::marker::Copy,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq
    )]
    #[xvalue(with = dir_value_with)]
    pub enum DirValue {
        Ltr,
        Rtl,
    }
    pub mod dir_value_with {
        pub fn deserialize<'de, D>(
            deserializer: D,
        ) -> ::core::result::Result<super::DirValue, D::Error>
        where
            D: ::xmlity::Deserializer<'de>,
        {
            let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                deserializer,
            )?;
            let value: ::std::string::String = text
                .parse()
                .map_err(::xmlity::de::Error::custom)?;
            super::DirValue::try_from(value).map_err(::xmlity::de::Error::custom)
        }
        pub fn serialize<S>(
            value: &super::DirValue,
            serializer: S,
        ) -> ::core::result::Result<S::Ok, S::Error>
        where
            S: ::xmlity::Serializer,
        {
            let value: ::std::string::String = ::core::clone::Clone::clone(value).into();
            ::xmlity::Serialize::serialize(
                ::std::string::String::as_str(
                    &::std::string::ToString::to_string(&value),
                ),
                serializer,
            )
        }
    }
    #[derive(::core::fmt::Debug)]
    pub enum DirValueParseError {
        NonExistent { value: ::std::string::String },
    }
    impl ::core::fmt::Display for DirValueParseError {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::result::Result<(), ::core::fmt::Error> {
            match self {
                DirValueParseError::NonExistent { value } => {
                    write!(f, "Value '{:?}' does not exist in the enumeration", value)
                }
            }
        }
    }
    impl ::core::convert::TryFrom<::std::string::String> for DirValue {
        type Error = DirValueParseError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::core::result::Result<Self, Self::Error> {
            match ::std::string::String::as_str(&value) {
                "ltr" => Ok(DirValue::Ltr),
                "rtl" => Ok(DirValue::Rtl),
                _ => {
                    Err(DirValueParseError::NonExistent {
                        value,
                    })
                }
            }
        }
    }
    impl ::core::convert::From<DirValue> for ::std::string::String {
        fn from(value: DirValue) -> Self {
            match value {
                DirValue::Ltr => ::std::string::String::from("ltr"),
                DirValue::Rtl => ::std::string::String::from("rtl"),
            }
        }
    }
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
#[xelement(
    name = "noscript",
    namespace = "http://www.w3.org/1999/xhtml",
    allow_unknown_attributes = "any"
)]
pub struct Noscript {
    #[xattribute(name = "id", optional, default)]
    pub id: ::core::option::Option<String>,
    #[xattribute(name = "class", optional, default)]
    pub class: ::core::option::Option<String>,
    #[xattribute(name = "style", optional, default)]
    pub style: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::StyleSheet>,
    >,
    #[xattribute(name = "title", optional, default)]
    pub title: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Text>>,
    #[xattribute(name = "lang", optional, default)]
    pub lang: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::LanguageCode>,
    >,
    #[xattribute(deferred = true, optional, default)]
    pub attribute_5: ::core::option::Option<crate::xml::Lang>,
    #[xattribute(name = "dir", optional, default)]
    pub dir: ::core::option::Option<noscript_items::DirValue>,
    #[xattribute(name = "onclick", optional, default)]
    pub onclick: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xattribute(name = "ondblclick", optional, default)]
    pub ondblclick: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmousedown", optional, default)]
    pub onmousedown: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseup", optional, default)]
    pub onmouseup: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseover", optional, default)]
    pub onmouseover: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmousemove", optional, default)]
    pub onmousemove: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseout", optional, default)]
    pub onmouseout: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeypress", optional, default)]
    pub onkeypress: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeydown", optional, default)]
    pub onkeydown: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeyup", optional, default)]
    pub onkeyup: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xvalue(default)]
    pub noscript: ::std::vec::Vec<noscript_items::Noscript>,
}
pub mod object_items {
    impl ::core::convert::From<crate::xhtml::Param> for Object {
        fn from(value: crate::xhtml::Param) -> Self {
            Object::Param(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml::groups::Block> for Object {
        fn from(value: crate::xhtml::groups::Block) -> Self {
            Object::Block(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml::Form> for Object {
        fn from(value: crate::xhtml::Form) -> Self {
            Object::Form(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml::groups::Inline> for Object {
        fn from(value: crate::xhtml::groups::Inline) -> Self {
            Object::Inline(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml::groups::Misc> for Object {
        fn from(value: crate::xhtml::groups::Misc) -> Self {
            Object::Misc(::std::boxed::Box::new(value))
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub enum Object {
        Param(::std::boxed::Box<crate::xhtml::Param>),
        Block(::std::boxed::Box<crate::xhtml::groups::Block>),
        Form(::std::boxed::Box<crate::xhtml::Form>),
        Inline(::std::boxed::Box<crate::xhtml::groups::Inline>),
        Misc(::std::boxed::Box<crate::xhtml::groups::Misc>),
    }
    #[derive(
        ::core::fmt::Debug,
        ::core::clone::Clone,
        ::core::marker::Copy,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq
    )]
    #[xvalue(with = dir_value_with)]
    pub enum DirValue {
        Ltr,
        Rtl,
    }
    pub mod dir_value_with {
        pub fn deserialize<'de, D>(
            deserializer: D,
        ) -> ::core::result::Result<super::DirValue, D::Error>
        where
            D: ::xmlity::Deserializer<'de>,
        {
            let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                deserializer,
            )?;
            let value: ::std::string::String = text
                .parse()
                .map_err(::xmlity::de::Error::custom)?;
            super::DirValue::try_from(value).map_err(::xmlity::de::Error::custom)
        }
        pub fn serialize<S>(
            value: &super::DirValue,
            serializer: S,
        ) -> ::core::result::Result<S::Ok, S::Error>
        where
            S: ::xmlity::Serializer,
        {
            let value: ::std::string::String = ::core::clone::Clone::clone(value).into();
            ::xmlity::Serialize::serialize(
                ::std::string::String::as_str(
                    &::std::string::ToString::to_string(&value),
                ),
                serializer,
            )
        }
    }
    #[derive(::core::fmt::Debug)]
    pub enum DirValueParseError {
        NonExistent { value: ::std::string::String },
    }
    impl ::core::fmt::Display for DirValueParseError {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::result::Result<(), ::core::fmt::Error> {
            match self {
                DirValueParseError::NonExistent { value } => {
                    write!(f, "Value '{:?}' does not exist in the enumeration", value)
                }
            }
        }
    }
    impl ::core::convert::TryFrom<::std::string::String> for DirValue {
        type Error = DirValueParseError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::core::result::Result<Self, Self::Error> {
            match ::std::string::String::as_str(&value) {
                "ltr" => Ok(DirValue::Ltr),
                "rtl" => Ok(DirValue::Rtl),
                _ => {
                    Err(DirValueParseError::NonExistent {
                        value,
                    })
                }
            }
        }
    }
    impl ::core::convert::From<DirValue> for ::std::string::String {
        fn from(value: DirValue) -> Self {
            match value {
                DirValue::Ltr => ::std::string::String::from("ltr"),
                DirValue::Rtl => ::std::string::String::from("rtl"),
            }
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::core::clone::Clone,
        ::core::marker::Copy,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq
    )]
    #[xvalue(with = declare_value_with)]
    pub enum DeclareValue {
        Declare,
    }
    pub mod declare_value_with {
        pub fn deserialize<'de, D>(
            deserializer: D,
        ) -> ::core::result::Result<super::DeclareValue, D::Error>
        where
            D: ::xmlity::Deserializer<'de>,
        {
            let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                deserializer,
            )?;
            let value: ::std::string::String = text
                .parse()
                .map_err(::xmlity::de::Error::custom)?;
            super::DeclareValue::try_from(value).map_err(::xmlity::de::Error::custom)
        }
        pub fn serialize<S>(
            value: &super::DeclareValue,
            serializer: S,
        ) -> ::core::result::Result<S::Ok, S::Error>
        where
            S: ::xmlity::Serializer,
        {
            let value: ::std::string::String = ::core::clone::Clone::clone(value).into();
            ::xmlity::Serialize::serialize(
                ::std::string::String::as_str(
                    &::std::string::ToString::to_string(&value),
                ),
                serializer,
            )
        }
    }
    #[derive(::core::fmt::Debug)]
    pub enum DeclareValueParseError {
        NonExistent { value: ::std::string::String },
    }
    impl ::core::fmt::Display for DeclareValueParseError {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::result::Result<(), ::core::fmt::Error> {
            match self {
                DeclareValueParseError::NonExistent { value } => {
                    write!(f, "Value '{:?}' does not exist in the enumeration", value)
                }
            }
        }
    }
    impl ::core::convert::TryFrom<::std::string::String> for DeclareValue {
        type Error = DeclareValueParseError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::core::result::Result<Self, Self::Error> {
            match ::std::string::String::as_str(&value) {
                "declare" => Ok(DeclareValue::Declare),
                _ => {
                    Err(DeclareValueParseError::NonExistent {
                        value,
                    })
                }
            }
        }
    }
    impl ::core::convert::From<DeclareValue> for ::std::string::String {
        fn from(value: DeclareValue) -> Self {
            match value {
                DeclareValue::Declare => ::std::string::String::from("declare"),
            }
        }
    }
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
#[xelement(
    name = "object",
    namespace = "http://www.w3.org/1999/xhtml",
    allow_unknown_attributes = "any"
)]
pub struct Object {
    #[xattribute(name = "id", optional, default)]
    pub id: ::core::option::Option<String>,
    #[xattribute(name = "class", optional, default)]
    pub class: ::core::option::Option<String>,
    #[xattribute(name = "style", optional, default)]
    pub style: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::StyleSheet>,
    >,
    #[xattribute(name = "title", optional, default)]
    pub title: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Text>>,
    #[xattribute(name = "lang", optional, default)]
    pub lang: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::LanguageCode>,
    >,
    #[xattribute(deferred = true, optional, default)]
    pub attribute_5: ::core::option::Option<crate::xml::Lang>,
    #[xattribute(name = "dir", optional, default)]
    pub dir: ::core::option::Option<object_items::DirValue>,
    #[xattribute(name = "onclick", optional, default)]
    pub onclick: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xattribute(name = "ondblclick", optional, default)]
    pub ondblclick: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmousedown", optional, default)]
    pub onmousedown: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseup", optional, default)]
    pub onmouseup: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseover", optional, default)]
    pub onmouseover: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmousemove", optional, default)]
    pub onmousemove: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseout", optional, default)]
    pub onmouseout: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeypress", optional, default)]
    pub onkeypress: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeydown", optional, default)]
    pub onkeydown: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeyup", optional, default)]
    pub onkeyup: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xattribute(name = "declare", optional, default)]
    pub declare: ::core::option::Option<object_items::DeclareValue>,
    #[xattribute(name = "classid", optional, default)]
    pub classid: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Uri>>,
    #[xattribute(name = "codebase", optional, default)]
    pub codebase: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Uri>>,
    #[xattribute(name = "data", optional, default)]
    pub data: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Uri>>,
    #[xattribute(name = "type", optional, default)]
    pub type_: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::ContentType>,
    >,
    #[xattribute(name = "codetype", optional, default)]
    pub codetype: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::ContentType>,
    >,
    #[xattribute(name = "archive", optional, default)]
    pub archive: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::UriList>>,
    #[xattribute(name = "standby", optional, default)]
    pub standby: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Text>>,
    #[xattribute(name = "height", optional, default)]
    pub height: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Length>>,
    #[xattribute(name = "width", optional, default)]
    pub width: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Length>>,
    #[xattribute(name = "usemap", optional, default)]
    pub usemap: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Uri>>,
    #[xattribute(name = "name", optional, default)]
    pub name: ::core::option::Option<String>,
    #[xattribute(name = "tabindex", optional, default)]
    pub tabindex: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::TabindexNumber>,
    >,
    #[xvalue(default)]
    pub object: ::std::vec::Vec<object_items::Object>,
}
pub mod ol_items {
    #[derive(
        ::core::fmt::Debug,
        ::core::clone::Clone,
        ::core::marker::Copy,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq
    )]
    #[xvalue(with = dir_value_with)]
    pub enum DirValue {
        Ltr,
        Rtl,
    }
    pub mod dir_value_with {
        pub fn deserialize<'de, D>(
            deserializer: D,
        ) -> ::core::result::Result<super::DirValue, D::Error>
        where
            D: ::xmlity::Deserializer<'de>,
        {
            let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                deserializer,
            )?;
            let value: ::std::string::String = text
                .parse()
                .map_err(::xmlity::de::Error::custom)?;
            super::DirValue::try_from(value).map_err(::xmlity::de::Error::custom)
        }
        pub fn serialize<S>(
            value: &super::DirValue,
            serializer: S,
        ) -> ::core::result::Result<S::Ok, S::Error>
        where
            S: ::xmlity::Serializer,
        {
            let value: ::std::string::String = ::core::clone::Clone::clone(value).into();
            ::xmlity::Serialize::serialize(
                ::std::string::String::as_str(
                    &::std::string::ToString::to_string(&value),
                ),
                serializer,
            )
        }
    }
    #[derive(::core::fmt::Debug)]
    pub enum DirValueParseError {
        NonExistent { value: ::std::string::String },
    }
    impl ::core::fmt::Display for DirValueParseError {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::result::Result<(), ::core::fmt::Error> {
            match self {
                DirValueParseError::NonExistent { value } => {
                    write!(f, "Value '{:?}' does not exist in the enumeration", value)
                }
            }
        }
    }
    impl ::core::convert::TryFrom<::std::string::String> for DirValue {
        type Error = DirValueParseError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::core::result::Result<Self, Self::Error> {
            match ::std::string::String::as_str(&value) {
                "ltr" => Ok(DirValue::Ltr),
                "rtl" => Ok(DirValue::Rtl),
                _ => {
                    Err(DirValueParseError::NonExistent {
                        value,
                    })
                }
            }
        }
    }
    impl ::core::convert::From<DirValue> for ::std::string::String {
        fn from(value: DirValue) -> Self {
            match value {
                DirValue::Ltr => ::std::string::String::from("ltr"),
                DirValue::Rtl => ::std::string::String::from("rtl"),
            }
        }
    }
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
#[xelement(
    name = "ol",
    namespace = "http://www.w3.org/1999/xhtml",
    allow_unknown_attributes = "any",
    children_order = "strict"
)]
pub struct Ol {
    #[xattribute(name = "id", optional, default)]
    pub id: ::core::option::Option<String>,
    #[xattribute(name = "class", optional, default)]
    pub class: ::core::option::Option<String>,
    #[xattribute(name = "style", optional, default)]
    pub style: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::StyleSheet>,
    >,
    #[xattribute(name = "title", optional, default)]
    pub title: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Text>>,
    #[xattribute(name = "lang", optional, default)]
    pub lang: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::LanguageCode>,
    >,
    #[xattribute(deferred = true, optional, default)]
    pub attribute_5: ::core::option::Option<crate::xml::Lang>,
    #[xattribute(name = "dir", optional, default)]
    pub dir: ::core::option::Option<ol_items::DirValue>,
    #[xattribute(name = "onclick", optional, default)]
    pub onclick: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xattribute(name = "ondblclick", optional, default)]
    pub ondblclick: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmousedown", optional, default)]
    pub onmousedown: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseup", optional, default)]
    pub onmouseup: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseover", optional, default)]
    pub onmouseover: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmousemove", optional, default)]
    pub onmousemove: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseout", optional, default)]
    pub onmouseout: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeypress", optional, default)]
    pub onkeypress: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeydown", optional, default)]
    pub onkeydown: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeyup", optional, default)]
    pub onkeyup: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xvalue(default)]
    pub li: ::std::vec::Vec<crate::xhtml::Li>,
}
pub mod optgroup_items {
    #[derive(
        ::core::fmt::Debug,
        ::core::clone::Clone,
        ::core::marker::Copy,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq
    )]
    #[xvalue(with = dir_value_with)]
    pub enum DirValue {
        Ltr,
        Rtl,
    }
    pub mod dir_value_with {
        pub fn deserialize<'de, D>(
            deserializer: D,
        ) -> ::core::result::Result<super::DirValue, D::Error>
        where
            D: ::xmlity::Deserializer<'de>,
        {
            let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                deserializer,
            )?;
            let value: ::std::string::String = text
                .parse()
                .map_err(::xmlity::de::Error::custom)?;
            super::DirValue::try_from(value).map_err(::xmlity::de::Error::custom)
        }
        pub fn serialize<S>(
            value: &super::DirValue,
            serializer: S,
        ) -> ::core::result::Result<S::Ok, S::Error>
        where
            S: ::xmlity::Serializer,
        {
            let value: ::std::string::String = ::core::clone::Clone::clone(value).into();
            ::xmlity::Serialize::serialize(
                ::std::string::String::as_str(
                    &::std::string::ToString::to_string(&value),
                ),
                serializer,
            )
        }
    }
    #[derive(::core::fmt::Debug)]
    pub enum DirValueParseError {
        NonExistent { value: ::std::string::String },
    }
    impl ::core::fmt::Display for DirValueParseError {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::result::Result<(), ::core::fmt::Error> {
            match self {
                DirValueParseError::NonExistent { value } => {
                    write!(f, "Value '{:?}' does not exist in the enumeration", value)
                }
            }
        }
    }
    impl ::core::convert::TryFrom<::std::string::String> for DirValue {
        type Error = DirValueParseError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::core::result::Result<Self, Self::Error> {
            match ::std::string::String::as_str(&value) {
                "ltr" => Ok(DirValue::Ltr),
                "rtl" => Ok(DirValue::Rtl),
                _ => {
                    Err(DirValueParseError::NonExistent {
                        value,
                    })
                }
            }
        }
    }
    impl ::core::convert::From<DirValue> for ::std::string::String {
        fn from(value: DirValue) -> Self {
            match value {
                DirValue::Ltr => ::std::string::String::from("ltr"),
                DirValue::Rtl => ::std::string::String::from("rtl"),
            }
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::core::clone::Clone,
        ::core::marker::Copy,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq
    )]
    #[xvalue(with = disabled_value_with)]
    pub enum DisabledValue {
        Disabled,
    }
    pub mod disabled_value_with {
        pub fn deserialize<'de, D>(
            deserializer: D,
        ) -> ::core::result::Result<super::DisabledValue, D::Error>
        where
            D: ::xmlity::Deserializer<'de>,
        {
            let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                deserializer,
            )?;
            let value: ::std::string::String = text
                .parse()
                .map_err(::xmlity::de::Error::custom)?;
            super::DisabledValue::try_from(value).map_err(::xmlity::de::Error::custom)
        }
        pub fn serialize<S>(
            value: &super::DisabledValue,
            serializer: S,
        ) -> ::core::result::Result<S::Ok, S::Error>
        where
            S: ::xmlity::Serializer,
        {
            let value: ::std::string::String = ::core::clone::Clone::clone(value).into();
            ::xmlity::Serialize::serialize(
                ::std::string::String::as_str(
                    &::std::string::ToString::to_string(&value),
                ),
                serializer,
            )
        }
    }
    #[derive(::core::fmt::Debug)]
    pub enum DisabledValueParseError {
        NonExistent { value: ::std::string::String },
    }
    impl ::core::fmt::Display for DisabledValueParseError {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::result::Result<(), ::core::fmt::Error> {
            match self {
                DisabledValueParseError::NonExistent { value } => {
                    write!(f, "Value '{:?}' does not exist in the enumeration", value)
                }
            }
        }
    }
    impl ::core::convert::TryFrom<::std::string::String> for DisabledValue {
        type Error = DisabledValueParseError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::core::result::Result<Self, Self::Error> {
            match ::std::string::String::as_str(&value) {
                "disabled" => Ok(DisabledValue::Disabled),
                _ => {
                    Err(DisabledValueParseError::NonExistent {
                        value,
                    })
                }
            }
        }
    }
    impl ::core::convert::From<DisabledValue> for ::std::string::String {
        fn from(value: DisabledValue) -> Self {
            match value {
                DisabledValue::Disabled => ::std::string::String::from("disabled"),
            }
        }
    }
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
#[xelement(
    name = "optgroup",
    namespace = "http://www.w3.org/1999/xhtml",
    allow_unknown_attributes = "any",
    children_order = "strict"
)]
pub struct Optgroup {
    #[xattribute(name = "id", optional, default)]
    pub id: ::core::option::Option<String>,
    #[xattribute(name = "class", optional, default)]
    pub class: ::core::option::Option<String>,
    #[xattribute(name = "style", optional, default)]
    pub style: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::StyleSheet>,
    >,
    #[xattribute(name = "title", optional, default)]
    pub title: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Text>>,
    #[xattribute(name = "lang", optional, default)]
    pub lang: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::LanguageCode>,
    >,
    #[xattribute(deferred = true, optional, default)]
    pub attribute_5: ::core::option::Option<crate::xml::Lang>,
    #[xattribute(name = "dir", optional, default)]
    pub dir: ::core::option::Option<optgroup_items::DirValue>,
    #[xattribute(name = "onclick", optional, default)]
    pub onclick: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xattribute(name = "ondblclick", optional, default)]
    pub ondblclick: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmousedown", optional, default)]
    pub onmousedown: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseup", optional, default)]
    pub onmouseup: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseover", optional, default)]
    pub onmouseover: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmousemove", optional, default)]
    pub onmousemove: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseout", optional, default)]
    pub onmouseout: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeypress", optional, default)]
    pub onkeypress: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeydown", optional, default)]
    pub onkeydown: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeyup", optional, default)]
    pub onkeyup: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xattribute(name = "disabled", optional, default)]
    pub disabled: ::core::option::Option<optgroup_items::DisabledValue>,
    #[xattribute(name = "label")]
    pub label: ::std::boxed::Box<crate::xhtml::types::Text>,
    #[xvalue(default)]
    pub option: ::std::vec::Vec<crate::xhtml::Option>,
}
pub mod option_items {
    #[derive(
        ::core::fmt::Debug,
        ::core::clone::Clone,
        ::core::marker::Copy,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq
    )]
    #[xvalue(with = dir_value_with)]
    pub enum DirValue {
        Ltr,
        Rtl,
    }
    pub mod dir_value_with {
        pub fn deserialize<'de, D>(
            deserializer: D,
        ) -> ::core::result::Result<super::DirValue, D::Error>
        where
            D: ::xmlity::Deserializer<'de>,
        {
            let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                deserializer,
            )?;
            let value: ::std::string::String = text
                .parse()
                .map_err(::xmlity::de::Error::custom)?;
            super::DirValue::try_from(value).map_err(::xmlity::de::Error::custom)
        }
        pub fn serialize<S>(
            value: &super::DirValue,
            serializer: S,
        ) -> ::core::result::Result<S::Ok, S::Error>
        where
            S: ::xmlity::Serializer,
        {
            let value: ::std::string::String = ::core::clone::Clone::clone(value).into();
            ::xmlity::Serialize::serialize(
                ::std::string::String::as_str(
                    &::std::string::ToString::to_string(&value),
                ),
                serializer,
            )
        }
    }
    #[derive(::core::fmt::Debug)]
    pub enum DirValueParseError {
        NonExistent { value: ::std::string::String },
    }
    impl ::core::fmt::Display for DirValueParseError {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::result::Result<(), ::core::fmt::Error> {
            match self {
                DirValueParseError::NonExistent { value } => {
                    write!(f, "Value '{:?}' does not exist in the enumeration", value)
                }
            }
        }
    }
    impl ::core::convert::TryFrom<::std::string::String> for DirValue {
        type Error = DirValueParseError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::core::result::Result<Self, Self::Error> {
            match ::std::string::String::as_str(&value) {
                "ltr" => Ok(DirValue::Ltr),
                "rtl" => Ok(DirValue::Rtl),
                _ => {
                    Err(DirValueParseError::NonExistent {
                        value,
                    })
                }
            }
        }
    }
    impl ::core::convert::From<DirValue> for ::std::string::String {
        fn from(value: DirValue) -> Self {
            match value {
                DirValue::Ltr => ::std::string::String::from("ltr"),
                DirValue::Rtl => ::std::string::String::from("rtl"),
            }
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::core::clone::Clone,
        ::core::marker::Copy,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq
    )]
    #[xvalue(with = selected_value_with)]
    pub enum SelectedValue {
        Selected,
    }
    pub mod selected_value_with {
        pub fn deserialize<'de, D>(
            deserializer: D,
        ) -> ::core::result::Result<super::SelectedValue, D::Error>
        where
            D: ::xmlity::Deserializer<'de>,
        {
            let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                deserializer,
            )?;
            let value: ::std::string::String = text
                .parse()
                .map_err(::xmlity::de::Error::custom)?;
            super::SelectedValue::try_from(value).map_err(::xmlity::de::Error::custom)
        }
        pub fn serialize<S>(
            value: &super::SelectedValue,
            serializer: S,
        ) -> ::core::result::Result<S::Ok, S::Error>
        where
            S: ::xmlity::Serializer,
        {
            let value: ::std::string::String = ::core::clone::Clone::clone(value).into();
            ::xmlity::Serialize::serialize(
                ::std::string::String::as_str(
                    &::std::string::ToString::to_string(&value),
                ),
                serializer,
            )
        }
    }
    #[derive(::core::fmt::Debug)]
    pub enum SelectedValueParseError {
        NonExistent { value: ::std::string::String },
    }
    impl ::core::fmt::Display for SelectedValueParseError {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::result::Result<(), ::core::fmt::Error> {
            match self {
                SelectedValueParseError::NonExistent { value } => {
                    write!(f, "Value '{:?}' does not exist in the enumeration", value)
                }
            }
        }
    }
    impl ::core::convert::TryFrom<::std::string::String> for SelectedValue {
        type Error = SelectedValueParseError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::core::result::Result<Self, Self::Error> {
            match ::std::string::String::as_str(&value) {
                "selected" => Ok(SelectedValue::Selected),
                _ => {
                    Err(SelectedValueParseError::NonExistent {
                        value,
                    })
                }
            }
        }
    }
    impl ::core::convert::From<SelectedValue> for ::std::string::String {
        fn from(value: SelectedValue) -> Self {
            match value {
                SelectedValue::Selected => ::std::string::String::from("selected"),
            }
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::core::clone::Clone,
        ::core::marker::Copy,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq
    )]
    #[xvalue(with = disabled_value_with)]
    pub enum DisabledValue {
        Disabled,
    }
    pub mod disabled_value_with {
        pub fn deserialize<'de, D>(
            deserializer: D,
        ) -> ::core::result::Result<super::DisabledValue, D::Error>
        where
            D: ::xmlity::Deserializer<'de>,
        {
            let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                deserializer,
            )?;
            let value: ::std::string::String = text
                .parse()
                .map_err(::xmlity::de::Error::custom)?;
            super::DisabledValue::try_from(value).map_err(::xmlity::de::Error::custom)
        }
        pub fn serialize<S>(
            value: &super::DisabledValue,
            serializer: S,
        ) -> ::core::result::Result<S::Ok, S::Error>
        where
            S: ::xmlity::Serializer,
        {
            let value: ::std::string::String = ::core::clone::Clone::clone(value).into();
            ::xmlity::Serialize::serialize(
                ::std::string::String::as_str(
                    &::std::string::ToString::to_string(&value),
                ),
                serializer,
            )
        }
    }
    #[derive(::core::fmt::Debug)]
    pub enum DisabledValueParseError {
        NonExistent { value: ::std::string::String },
    }
    impl ::core::fmt::Display for DisabledValueParseError {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::result::Result<(), ::core::fmt::Error> {
            match self {
                DisabledValueParseError::NonExistent { value } => {
                    write!(f, "Value '{:?}' does not exist in the enumeration", value)
                }
            }
        }
    }
    impl ::core::convert::TryFrom<::std::string::String> for DisabledValue {
        type Error = DisabledValueParseError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::core::result::Result<Self, Self::Error> {
            match ::std::string::String::as_str(&value) {
                "disabled" => Ok(DisabledValue::Disabled),
                _ => {
                    Err(DisabledValueParseError::NonExistent {
                        value,
                    })
                }
            }
        }
    }
    impl ::core::convert::From<DisabledValue> for ::std::string::String {
        fn from(value: DisabledValue) -> Self {
            match value {
                DisabledValue::Disabled => ::std::string::String::from("disabled"),
            }
        }
    }
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
#[xelement(
    name = "option",
    namespace = "http://www.w3.org/1999/xhtml",
    allow_unknown_attributes = "any"
)]
pub struct Option {
    #[xattribute(name = "id", optional, default)]
    pub id: ::core::option::Option<String>,
    #[xattribute(name = "class", optional, default)]
    pub class: ::core::option::Option<String>,
    #[xattribute(name = "style", optional, default)]
    pub style: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::StyleSheet>,
    >,
    #[xattribute(name = "title", optional, default)]
    pub title: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Text>>,
    #[xattribute(name = "lang", optional, default)]
    pub lang: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::LanguageCode>,
    >,
    #[xattribute(deferred = true, optional, default)]
    pub attribute_5: ::core::option::Option<crate::xml::Lang>,
    #[xattribute(name = "dir", optional, default)]
    pub dir: ::core::option::Option<option_items::DirValue>,
    #[xattribute(name = "onclick", optional, default)]
    pub onclick: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xattribute(name = "ondblclick", optional, default)]
    pub ondblclick: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmousedown", optional, default)]
    pub onmousedown: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseup", optional, default)]
    pub onmouseup: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseover", optional, default)]
    pub onmouseover: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmousemove", optional, default)]
    pub onmousemove: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseout", optional, default)]
    pub onmouseout: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeypress", optional, default)]
    pub onkeypress: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeydown", optional, default)]
    pub onkeydown: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeyup", optional, default)]
    pub onkeyup: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xattribute(name = "selected", optional, default)]
    pub selected: ::core::option::Option<option_items::SelectedValue>,
    #[xattribute(name = "disabled", optional, default)]
    pub disabled: ::core::option::Option<option_items::DisabledValue>,
    #[xattribute(name = "label", optional, default)]
    pub label: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Text>>,
    #[xattribute(name = "value", optional, default)]
    pub value: ::core::option::Option<String>,
}
pub mod p_items {
    impl ::core::convert::From<crate::xhtml::groups::Inline> for Inline {
        fn from(value: crate::xhtml::groups::Inline) -> Self {
            Inline::Inline(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml::groups::MiscInline> for Inline {
        fn from(value: crate::xhtml::groups::MiscInline) -> Self {
            Inline::MiscInline(::std::boxed::Box::new(value))
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub enum Inline {
        Inline(::std::boxed::Box<crate::xhtml::groups::Inline>),
        MiscInline(::std::boxed::Box<crate::xhtml::groups::MiscInline>),
    }
    #[derive(
        ::core::fmt::Debug,
        ::core::clone::Clone,
        ::core::marker::Copy,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq
    )]
    #[xvalue(with = dir_value_with)]
    pub enum DirValue {
        Ltr,
        Rtl,
    }
    pub mod dir_value_with {
        pub fn deserialize<'de, D>(
            deserializer: D,
        ) -> ::core::result::Result<super::DirValue, D::Error>
        where
            D: ::xmlity::Deserializer<'de>,
        {
            let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                deserializer,
            )?;
            let value: ::std::string::String = text
                .parse()
                .map_err(::xmlity::de::Error::custom)?;
            super::DirValue::try_from(value).map_err(::xmlity::de::Error::custom)
        }
        pub fn serialize<S>(
            value: &super::DirValue,
            serializer: S,
        ) -> ::core::result::Result<S::Ok, S::Error>
        where
            S: ::xmlity::Serializer,
        {
            let value: ::std::string::String = ::core::clone::Clone::clone(value).into();
            ::xmlity::Serialize::serialize(
                ::std::string::String::as_str(
                    &::std::string::ToString::to_string(&value),
                ),
                serializer,
            )
        }
    }
    #[derive(::core::fmt::Debug)]
    pub enum DirValueParseError {
        NonExistent { value: ::std::string::String },
    }
    impl ::core::fmt::Display for DirValueParseError {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::result::Result<(), ::core::fmt::Error> {
            match self {
                DirValueParseError::NonExistent { value } => {
                    write!(f, "Value '{:?}' does not exist in the enumeration", value)
                }
            }
        }
    }
    impl ::core::convert::TryFrom<::std::string::String> for DirValue {
        type Error = DirValueParseError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::core::result::Result<Self, Self::Error> {
            match ::std::string::String::as_str(&value) {
                "ltr" => Ok(DirValue::Ltr),
                "rtl" => Ok(DirValue::Rtl),
                _ => {
                    Err(DirValueParseError::NonExistent {
                        value,
                    })
                }
            }
        }
    }
    impl ::core::convert::From<DirValue> for ::std::string::String {
        fn from(value: DirValue) -> Self {
            match value {
                DirValue::Ltr => ::std::string::String::from("ltr"),
                DirValue::Rtl => ::std::string::String::from("rtl"),
            }
        }
    }
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
#[xelement(
    name = "p",
    namespace = "http://www.w3.org/1999/xhtml",
    allow_unknown_attributes = "any"
)]
pub struct P {
    #[xattribute(name = "id", optional, default)]
    pub id: ::core::option::Option<String>,
    #[xattribute(name = "class", optional, default)]
    pub class: ::core::option::Option<String>,
    #[xattribute(name = "style", optional, default)]
    pub style: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::StyleSheet>,
    >,
    #[xattribute(name = "title", optional, default)]
    pub title: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Text>>,
    #[xattribute(name = "lang", optional, default)]
    pub lang: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::LanguageCode>,
    >,
    #[xattribute(deferred = true, optional, default)]
    pub attribute_5: ::core::option::Option<crate::xml::Lang>,
    #[xattribute(name = "dir", optional, default)]
    pub dir: ::core::option::Option<p_items::DirValue>,
    #[xattribute(name = "onclick", optional, default)]
    pub onclick: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xattribute(name = "ondblclick", optional, default)]
    pub ondblclick: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmousedown", optional, default)]
    pub onmousedown: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseup", optional, default)]
    pub onmouseup: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseover", optional, default)]
    pub onmouseover: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmousemove", optional, default)]
    pub onmousemove: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseout", optional, default)]
    pub onmouseout: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeypress", optional, default)]
    pub onkeypress: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeydown", optional, default)]
    pub onkeydown: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeyup", optional, default)]
    pub onkeyup: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xvalue(default)]
    pub inline: ::std::vec::Vec<p_items::Inline>,
}
pub mod param_items {
    #[derive(
        ::core::fmt::Debug,
        ::core::clone::Clone,
        ::core::marker::Copy,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq
    )]
    #[xvalue(with = valuetype_value_with)]
    pub enum ValuetypeValue {
        Data,
        Ref,
        Object,
    }
    pub mod valuetype_value_with {
        pub fn deserialize<'de, D>(
            deserializer: D,
        ) -> ::core::result::Result<super::ValuetypeValue, D::Error>
        where
            D: ::xmlity::Deserializer<'de>,
        {
            let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                deserializer,
            )?;
            let value: ::std::string::String = text
                .parse()
                .map_err(::xmlity::de::Error::custom)?;
            super::ValuetypeValue::try_from(value).map_err(::xmlity::de::Error::custom)
        }
        pub fn serialize<S>(
            value: &super::ValuetypeValue,
            serializer: S,
        ) -> ::core::result::Result<S::Ok, S::Error>
        where
            S: ::xmlity::Serializer,
        {
            let value: ::std::string::String = ::core::clone::Clone::clone(value).into();
            ::xmlity::Serialize::serialize(
                ::std::string::String::as_str(
                    &::std::string::ToString::to_string(&value),
                ),
                serializer,
            )
        }
    }
    #[derive(::core::fmt::Debug)]
    pub enum ValuetypeValueParseError {
        NonExistent { value: ::std::string::String },
    }
    impl ::core::fmt::Display for ValuetypeValueParseError {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::result::Result<(), ::core::fmt::Error> {
            match self {
                ValuetypeValueParseError::NonExistent { value } => {
                    write!(f, "Value '{:?}' does not exist in the enumeration", value)
                }
            }
        }
    }
    impl ::core::convert::TryFrom<::std::string::String> for ValuetypeValue {
        type Error = ValuetypeValueParseError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::core::result::Result<Self, Self::Error> {
            match ::std::string::String::as_str(&value) {
                "data" => Ok(ValuetypeValue::Data),
                "ref" => Ok(ValuetypeValue::Ref),
                "object" => Ok(ValuetypeValue::Object),
                _ => {
                    Err(ValuetypeValueParseError::NonExistent {
                        value,
                    })
                }
            }
        }
    }
    impl ::core::convert::From<ValuetypeValue> for ::std::string::String {
        fn from(value: ValuetypeValue) -> Self {
            match value {
                ValuetypeValue::Data => ::std::string::String::from("data"),
                ValuetypeValue::Ref => ::std::string::String::from("ref"),
                ValuetypeValue::Object => ::std::string::String::from("object"),
            }
        }
    }
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
#[xelement(
    name = "param",
    namespace = "http://www.w3.org/1999/xhtml",
    allow_unknown_attributes = "any"
)]
pub struct Param {
    #[xattribute(name = "id", optional, default)]
    pub id: ::core::option::Option<String>,
    #[xattribute(name = "name", optional, default)]
    pub name: ::core::option::Option<String>,
    #[xattribute(name = "value", optional, default)]
    pub value: ::core::option::Option<String>,
    #[xattribute(name = "valuetype", optional, default)]
    pub valuetype: ::core::option::Option<param_items::ValuetypeValue>,
    #[xattribute(name = "type", optional, default)]
    pub type_: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::ContentType>,
    >,
}
pub mod pre_items {
    impl ::core::convert::From<crate::xhtml::A> for Pre {
        fn from(value: crate::xhtml::A) -> Self {
            Pre::A(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml::groups::Fontstyle> for Pre {
        fn from(value: crate::xhtml::groups::Fontstyle) -> Self {
            Pre::Fontstyle(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml::groups::Phrase> for Pre {
        fn from(value: crate::xhtml::groups::Phrase) -> Self {
            Pre::Phrase(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml::groups::SpecialPre> for Pre {
        fn from(value: crate::xhtml::groups::SpecialPre) -> Self {
            Pre::SpecialPre(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml::groups::MiscInline> for Pre {
        fn from(value: crate::xhtml::groups::MiscInline) -> Self {
            Pre::MiscInline(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml::groups::InlineForms> for Pre {
        fn from(value: crate::xhtml::groups::InlineForms) -> Self {
            Pre::InlineForms(::std::boxed::Box::new(value))
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub enum Pre {
        A(::std::boxed::Box<crate::xhtml::A>),
        Fontstyle(::std::boxed::Box<crate::xhtml::groups::Fontstyle>),
        Phrase(::std::boxed::Box<crate::xhtml::groups::Phrase>),
        SpecialPre(::std::boxed::Box<crate::xhtml::groups::SpecialPre>),
        MiscInline(::std::boxed::Box<crate::xhtml::groups::MiscInline>),
        InlineForms(::std::boxed::Box<crate::xhtml::groups::InlineForms>),
    }
    #[derive(
        ::core::fmt::Debug,
        ::core::clone::Clone,
        ::core::marker::Copy,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq
    )]
    #[xvalue(with = dir_value_with)]
    pub enum DirValue {
        Ltr,
        Rtl,
    }
    pub mod dir_value_with {
        pub fn deserialize<'de, D>(
            deserializer: D,
        ) -> ::core::result::Result<super::DirValue, D::Error>
        where
            D: ::xmlity::Deserializer<'de>,
        {
            let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                deserializer,
            )?;
            let value: ::std::string::String = text
                .parse()
                .map_err(::xmlity::de::Error::custom)?;
            super::DirValue::try_from(value).map_err(::xmlity::de::Error::custom)
        }
        pub fn serialize<S>(
            value: &super::DirValue,
            serializer: S,
        ) -> ::core::result::Result<S::Ok, S::Error>
        where
            S: ::xmlity::Serializer,
        {
            let value: ::std::string::String = ::core::clone::Clone::clone(value).into();
            ::xmlity::Serialize::serialize(
                ::std::string::String::as_str(
                    &::std::string::ToString::to_string(&value),
                ),
                serializer,
            )
        }
    }
    #[derive(::core::fmt::Debug)]
    pub enum DirValueParseError {
        NonExistent { value: ::std::string::String },
    }
    impl ::core::fmt::Display for DirValueParseError {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::result::Result<(), ::core::fmt::Error> {
            match self {
                DirValueParseError::NonExistent { value } => {
                    write!(f, "Value '{:?}' does not exist in the enumeration", value)
                }
            }
        }
    }
    impl ::core::convert::TryFrom<::std::string::String> for DirValue {
        type Error = DirValueParseError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::core::result::Result<Self, Self::Error> {
            match ::std::string::String::as_str(&value) {
                "ltr" => Ok(DirValue::Ltr),
                "rtl" => Ok(DirValue::Rtl),
                _ => {
                    Err(DirValueParseError::NonExistent {
                        value,
                    })
                }
            }
        }
    }
    impl ::core::convert::From<DirValue> for ::std::string::String {
        fn from(value: DirValue) -> Self {
            match value {
                DirValue::Ltr => ::std::string::String::from("ltr"),
                DirValue::Rtl => ::std::string::String::from("rtl"),
            }
        }
    }
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
#[xelement(
    name = "pre",
    namespace = "http://www.w3.org/1999/xhtml",
    allow_unknown_attributes = "any"
)]
pub struct Pre {
    #[xattribute(name = "id", optional, default)]
    pub id: ::core::option::Option<String>,
    #[xattribute(name = "class", optional, default)]
    pub class: ::core::option::Option<String>,
    #[xattribute(name = "style", optional, default)]
    pub style: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::StyleSheet>,
    >,
    #[xattribute(name = "title", optional, default)]
    pub title: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Text>>,
    #[xattribute(name = "lang", optional, default)]
    pub lang: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::LanguageCode>,
    >,
    #[xattribute(deferred = true, optional, default)]
    pub attribute_5: ::core::option::Option<crate::xml::Lang>,
    #[xattribute(name = "dir", optional, default)]
    pub dir: ::core::option::Option<pre_items::DirValue>,
    #[xattribute(name = "onclick", optional, default)]
    pub onclick: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xattribute(name = "ondblclick", optional, default)]
    pub ondblclick: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmousedown", optional, default)]
    pub onmousedown: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseup", optional, default)]
    pub onmouseup: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseover", optional, default)]
    pub onmouseover: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmousemove", optional, default)]
    pub onmousemove: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseout", optional, default)]
    pub onmouseout: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeypress", optional, default)]
    pub onkeypress: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeydown", optional, default)]
    pub onkeydown: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeyup", optional, default)]
    pub onkeyup: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xattribute(deferred = true, optional, default)]
    pub attribute_17: ::core::option::Option<crate::xml::Space>,
    #[xvalue(default)]
    pub pre: ::std::vec::Vec<pre_items::Pre>,
}
pub mod q_items {
    impl ::core::convert::From<crate::xhtml::groups::Inline> for Inline {
        fn from(value: crate::xhtml::groups::Inline) -> Self {
            Inline::Inline(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml::groups::MiscInline> for Inline {
        fn from(value: crate::xhtml::groups::MiscInline) -> Self {
            Inline::MiscInline(::std::boxed::Box::new(value))
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub enum Inline {
        Inline(::std::boxed::Box<crate::xhtml::groups::Inline>),
        MiscInline(::std::boxed::Box<crate::xhtml::groups::MiscInline>),
    }
    #[derive(
        ::core::fmt::Debug,
        ::core::clone::Clone,
        ::core::marker::Copy,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq
    )]
    #[xvalue(with = dir_value_with)]
    pub enum DirValue {
        Ltr,
        Rtl,
    }
    pub mod dir_value_with {
        pub fn deserialize<'de, D>(
            deserializer: D,
        ) -> ::core::result::Result<super::DirValue, D::Error>
        where
            D: ::xmlity::Deserializer<'de>,
        {
            let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                deserializer,
            )?;
            let value: ::std::string::String = text
                .parse()
                .map_err(::xmlity::de::Error::custom)?;
            super::DirValue::try_from(value).map_err(::xmlity::de::Error::custom)
        }
        pub fn serialize<S>(
            value: &super::DirValue,
            serializer: S,
        ) -> ::core::result::Result<S::Ok, S::Error>
        where
            S: ::xmlity::Serializer,
        {
            let value: ::std::string::String = ::core::clone::Clone::clone(value).into();
            ::xmlity::Serialize::serialize(
                ::std::string::String::as_str(
                    &::std::string::ToString::to_string(&value),
                ),
                serializer,
            )
        }
    }
    #[derive(::core::fmt::Debug)]
    pub enum DirValueParseError {
        NonExistent { value: ::std::string::String },
    }
    impl ::core::fmt::Display for DirValueParseError {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::result::Result<(), ::core::fmt::Error> {
            match self {
                DirValueParseError::NonExistent { value } => {
                    write!(f, "Value '{:?}' does not exist in the enumeration", value)
                }
            }
        }
    }
    impl ::core::convert::TryFrom<::std::string::String> for DirValue {
        type Error = DirValueParseError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::core::result::Result<Self, Self::Error> {
            match ::std::string::String::as_str(&value) {
                "ltr" => Ok(DirValue::Ltr),
                "rtl" => Ok(DirValue::Rtl),
                _ => {
                    Err(DirValueParseError::NonExistent {
                        value,
                    })
                }
            }
        }
    }
    impl ::core::convert::From<DirValue> for ::std::string::String {
        fn from(value: DirValue) -> Self {
            match value {
                DirValue::Ltr => ::std::string::String::from("ltr"),
                DirValue::Rtl => ::std::string::String::from("rtl"),
            }
        }
    }
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
#[xelement(
    name = "q",
    namespace = "http://www.w3.org/1999/xhtml",
    allow_unknown_attributes = "any"
)]
pub struct Q {
    #[xattribute(name = "id", optional, default)]
    pub id: ::core::option::Option<String>,
    #[xattribute(name = "class", optional, default)]
    pub class: ::core::option::Option<String>,
    #[xattribute(name = "style", optional, default)]
    pub style: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::StyleSheet>,
    >,
    #[xattribute(name = "title", optional, default)]
    pub title: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Text>>,
    #[xattribute(name = "lang", optional, default)]
    pub lang: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::LanguageCode>,
    >,
    #[xattribute(deferred = true, optional, default)]
    pub attribute_5: ::core::option::Option<crate::xml::Lang>,
    #[xattribute(name = "dir", optional, default)]
    pub dir: ::core::option::Option<q_items::DirValue>,
    #[xattribute(name = "onclick", optional, default)]
    pub onclick: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xattribute(name = "ondblclick", optional, default)]
    pub ondblclick: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmousedown", optional, default)]
    pub onmousedown: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseup", optional, default)]
    pub onmouseup: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseover", optional, default)]
    pub onmouseover: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmousemove", optional, default)]
    pub onmousemove: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseout", optional, default)]
    pub onmouseout: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeypress", optional, default)]
    pub onkeypress: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeydown", optional, default)]
    pub onkeydown: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeyup", optional, default)]
    pub onkeyup: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xattribute(name = "cite", optional, default)]
    pub cite: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Uri>>,
    #[xvalue(default)]
    pub inline: ::std::vec::Vec<q_items::Inline>,
}
pub mod samp_items {
    impl ::core::convert::From<crate::xhtml::groups::Inline> for Inline {
        fn from(value: crate::xhtml::groups::Inline) -> Self {
            Inline::Inline(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml::groups::MiscInline> for Inline {
        fn from(value: crate::xhtml::groups::MiscInline) -> Self {
            Inline::MiscInline(::std::boxed::Box::new(value))
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub enum Inline {
        Inline(::std::boxed::Box<crate::xhtml::groups::Inline>),
        MiscInline(::std::boxed::Box<crate::xhtml::groups::MiscInline>),
    }
    #[derive(
        ::core::fmt::Debug,
        ::core::clone::Clone,
        ::core::marker::Copy,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq
    )]
    #[xvalue(with = dir_value_with)]
    pub enum DirValue {
        Ltr,
        Rtl,
    }
    pub mod dir_value_with {
        pub fn deserialize<'de, D>(
            deserializer: D,
        ) -> ::core::result::Result<super::DirValue, D::Error>
        where
            D: ::xmlity::Deserializer<'de>,
        {
            let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                deserializer,
            )?;
            let value: ::std::string::String = text
                .parse()
                .map_err(::xmlity::de::Error::custom)?;
            super::DirValue::try_from(value).map_err(::xmlity::de::Error::custom)
        }
        pub fn serialize<S>(
            value: &super::DirValue,
            serializer: S,
        ) -> ::core::result::Result<S::Ok, S::Error>
        where
            S: ::xmlity::Serializer,
        {
            let value: ::std::string::String = ::core::clone::Clone::clone(value).into();
            ::xmlity::Serialize::serialize(
                ::std::string::String::as_str(
                    &::std::string::ToString::to_string(&value),
                ),
                serializer,
            )
        }
    }
    #[derive(::core::fmt::Debug)]
    pub enum DirValueParseError {
        NonExistent { value: ::std::string::String },
    }
    impl ::core::fmt::Display for DirValueParseError {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::result::Result<(), ::core::fmt::Error> {
            match self {
                DirValueParseError::NonExistent { value } => {
                    write!(f, "Value '{:?}' does not exist in the enumeration", value)
                }
            }
        }
    }
    impl ::core::convert::TryFrom<::std::string::String> for DirValue {
        type Error = DirValueParseError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::core::result::Result<Self, Self::Error> {
            match ::std::string::String::as_str(&value) {
                "ltr" => Ok(DirValue::Ltr),
                "rtl" => Ok(DirValue::Rtl),
                _ => {
                    Err(DirValueParseError::NonExistent {
                        value,
                    })
                }
            }
        }
    }
    impl ::core::convert::From<DirValue> for ::std::string::String {
        fn from(value: DirValue) -> Self {
            match value {
                DirValue::Ltr => ::std::string::String::from("ltr"),
                DirValue::Rtl => ::std::string::String::from("rtl"),
            }
        }
    }
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
#[xelement(
    name = "samp",
    namespace = "http://www.w3.org/1999/xhtml",
    allow_unknown_attributes = "any"
)]
pub struct Samp {
    #[xattribute(name = "id", optional, default)]
    pub id: ::core::option::Option<String>,
    #[xattribute(name = "class", optional, default)]
    pub class: ::core::option::Option<String>,
    #[xattribute(name = "style", optional, default)]
    pub style: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::StyleSheet>,
    >,
    #[xattribute(name = "title", optional, default)]
    pub title: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Text>>,
    #[xattribute(name = "lang", optional, default)]
    pub lang: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::LanguageCode>,
    >,
    #[xattribute(deferred = true, optional, default)]
    pub attribute_5: ::core::option::Option<crate::xml::Lang>,
    #[xattribute(name = "dir", optional, default)]
    pub dir: ::core::option::Option<samp_items::DirValue>,
    #[xattribute(name = "onclick", optional, default)]
    pub onclick: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xattribute(name = "ondblclick", optional, default)]
    pub ondblclick: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmousedown", optional, default)]
    pub onmousedown: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseup", optional, default)]
    pub onmouseup: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseover", optional, default)]
    pub onmouseover: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmousemove", optional, default)]
    pub onmousemove: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseout", optional, default)]
    pub onmouseout: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeypress", optional, default)]
    pub onkeypress: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeydown", optional, default)]
    pub onkeydown: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeyup", optional, default)]
    pub onkeyup: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xvalue(default)]
    pub inline: ::std::vec::Vec<samp_items::Inline>,
}
pub mod script_items {
    #[derive(
        ::core::fmt::Debug,
        ::core::clone::Clone,
        ::core::marker::Copy,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq
    )]
    #[xvalue(with = defer_value_with)]
    pub enum DeferValue {
        Defer,
    }
    pub mod defer_value_with {
        pub fn deserialize<'de, D>(
            deserializer: D,
        ) -> ::core::result::Result<super::DeferValue, D::Error>
        where
            D: ::xmlity::Deserializer<'de>,
        {
            let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                deserializer,
            )?;
            let value: ::std::string::String = text
                .parse()
                .map_err(::xmlity::de::Error::custom)?;
            super::DeferValue::try_from(value).map_err(::xmlity::de::Error::custom)
        }
        pub fn serialize<S>(
            value: &super::DeferValue,
            serializer: S,
        ) -> ::core::result::Result<S::Ok, S::Error>
        where
            S: ::xmlity::Serializer,
        {
            let value: ::std::string::String = ::core::clone::Clone::clone(value).into();
            ::xmlity::Serialize::serialize(
                ::std::string::String::as_str(
                    &::std::string::ToString::to_string(&value),
                ),
                serializer,
            )
        }
    }
    #[derive(::core::fmt::Debug)]
    pub enum DeferValueParseError {
        NonExistent { value: ::std::string::String },
    }
    impl ::core::fmt::Display for DeferValueParseError {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::result::Result<(), ::core::fmt::Error> {
            match self {
                DeferValueParseError::NonExistent { value } => {
                    write!(f, "Value '{:?}' does not exist in the enumeration", value)
                }
            }
        }
    }
    impl ::core::convert::TryFrom<::std::string::String> for DeferValue {
        type Error = DeferValueParseError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::core::result::Result<Self, Self::Error> {
            match ::std::string::String::as_str(&value) {
                "defer" => Ok(DeferValue::Defer),
                _ => {
                    Err(DeferValueParseError::NonExistent {
                        value,
                    })
                }
            }
        }
    }
    impl ::core::convert::From<DeferValue> for ::std::string::String {
        fn from(value: DeferValue) -> Self {
            match value {
                DeferValue::Defer => ::std::string::String::from("defer"),
            }
        }
    }
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
#[xelement(
    name = "script",
    namespace = "http://www.w3.org/1999/xhtml",
    allow_unknown_attributes = "any"
)]
pub struct Script {
    #[xattribute(name = "id", optional, default)]
    pub id: ::core::option::Option<String>,
    #[xattribute(name = "charset", optional, default)]
    pub charset: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Charset>>,
    #[xattribute(name = "type")]
    pub type_: ::std::boxed::Box<crate::xhtml::types::ContentType>,
    #[xattribute(name = "src", optional, default)]
    pub src: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Uri>>,
    #[xattribute(name = "defer", optional, default)]
    pub defer: ::core::option::Option<script_items::DeferValue>,
    #[xattribute(deferred = true, optional, default)]
    pub attribute_5: ::core::option::Option<crate::xml::Space>,
}
pub mod select_items {
    impl ::core::convert::From<crate::xhtml::Optgroup> for Select {
        fn from(value: crate::xhtml::Optgroup) -> Self {
            Select::Optgroup(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml::Option> for Select {
        fn from(value: crate::xhtml::Option) -> Self {
            Select::Option(::std::boxed::Box::new(value))
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub enum Select {
        Optgroup(::std::boxed::Box<crate::xhtml::Optgroup>),
        Option(::std::boxed::Box<crate::xhtml::Option>),
    }
    #[derive(
        ::core::fmt::Debug,
        ::core::clone::Clone,
        ::core::marker::Copy,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq
    )]
    #[xvalue(with = dir_value_with)]
    pub enum DirValue {
        Ltr,
        Rtl,
    }
    pub mod dir_value_with {
        pub fn deserialize<'de, D>(
            deserializer: D,
        ) -> ::core::result::Result<super::DirValue, D::Error>
        where
            D: ::xmlity::Deserializer<'de>,
        {
            let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                deserializer,
            )?;
            let value: ::std::string::String = text
                .parse()
                .map_err(::xmlity::de::Error::custom)?;
            super::DirValue::try_from(value).map_err(::xmlity::de::Error::custom)
        }
        pub fn serialize<S>(
            value: &super::DirValue,
            serializer: S,
        ) -> ::core::result::Result<S::Ok, S::Error>
        where
            S: ::xmlity::Serializer,
        {
            let value: ::std::string::String = ::core::clone::Clone::clone(value).into();
            ::xmlity::Serialize::serialize(
                ::std::string::String::as_str(
                    &::std::string::ToString::to_string(&value),
                ),
                serializer,
            )
        }
    }
    #[derive(::core::fmt::Debug)]
    pub enum DirValueParseError {
        NonExistent { value: ::std::string::String },
    }
    impl ::core::fmt::Display for DirValueParseError {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::result::Result<(), ::core::fmt::Error> {
            match self {
                DirValueParseError::NonExistent { value } => {
                    write!(f, "Value '{:?}' does not exist in the enumeration", value)
                }
            }
        }
    }
    impl ::core::convert::TryFrom<::std::string::String> for DirValue {
        type Error = DirValueParseError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::core::result::Result<Self, Self::Error> {
            match ::std::string::String::as_str(&value) {
                "ltr" => Ok(DirValue::Ltr),
                "rtl" => Ok(DirValue::Rtl),
                _ => {
                    Err(DirValueParseError::NonExistent {
                        value,
                    })
                }
            }
        }
    }
    impl ::core::convert::From<DirValue> for ::std::string::String {
        fn from(value: DirValue) -> Self {
            match value {
                DirValue::Ltr => ::std::string::String::from("ltr"),
                DirValue::Rtl => ::std::string::String::from("rtl"),
            }
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::core::clone::Clone,
        ::core::marker::Copy,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq
    )]
    #[xvalue(with = multiple_value_with)]
    pub enum MultipleValue {
        Multiple,
    }
    pub mod multiple_value_with {
        pub fn deserialize<'de, D>(
            deserializer: D,
        ) -> ::core::result::Result<super::MultipleValue, D::Error>
        where
            D: ::xmlity::Deserializer<'de>,
        {
            let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                deserializer,
            )?;
            let value: ::std::string::String = text
                .parse()
                .map_err(::xmlity::de::Error::custom)?;
            super::MultipleValue::try_from(value).map_err(::xmlity::de::Error::custom)
        }
        pub fn serialize<S>(
            value: &super::MultipleValue,
            serializer: S,
        ) -> ::core::result::Result<S::Ok, S::Error>
        where
            S: ::xmlity::Serializer,
        {
            let value: ::std::string::String = ::core::clone::Clone::clone(value).into();
            ::xmlity::Serialize::serialize(
                ::std::string::String::as_str(
                    &::std::string::ToString::to_string(&value),
                ),
                serializer,
            )
        }
    }
    #[derive(::core::fmt::Debug)]
    pub enum MultipleValueParseError {
        NonExistent { value: ::std::string::String },
    }
    impl ::core::fmt::Display for MultipleValueParseError {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::result::Result<(), ::core::fmt::Error> {
            match self {
                MultipleValueParseError::NonExistent { value } => {
                    write!(f, "Value '{:?}' does not exist in the enumeration", value)
                }
            }
        }
    }
    impl ::core::convert::TryFrom<::std::string::String> for MultipleValue {
        type Error = MultipleValueParseError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::core::result::Result<Self, Self::Error> {
            match ::std::string::String::as_str(&value) {
                "multiple" => Ok(MultipleValue::Multiple),
                _ => {
                    Err(MultipleValueParseError::NonExistent {
                        value,
                    })
                }
            }
        }
    }
    impl ::core::convert::From<MultipleValue> for ::std::string::String {
        fn from(value: MultipleValue) -> Self {
            match value {
                MultipleValue::Multiple => ::std::string::String::from("multiple"),
            }
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::core::clone::Clone,
        ::core::marker::Copy,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq
    )]
    #[xvalue(with = disabled_value_with)]
    pub enum DisabledValue {
        Disabled,
    }
    pub mod disabled_value_with {
        pub fn deserialize<'de, D>(
            deserializer: D,
        ) -> ::core::result::Result<super::DisabledValue, D::Error>
        where
            D: ::xmlity::Deserializer<'de>,
        {
            let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                deserializer,
            )?;
            let value: ::std::string::String = text
                .parse()
                .map_err(::xmlity::de::Error::custom)?;
            super::DisabledValue::try_from(value).map_err(::xmlity::de::Error::custom)
        }
        pub fn serialize<S>(
            value: &super::DisabledValue,
            serializer: S,
        ) -> ::core::result::Result<S::Ok, S::Error>
        where
            S: ::xmlity::Serializer,
        {
            let value: ::std::string::String = ::core::clone::Clone::clone(value).into();
            ::xmlity::Serialize::serialize(
                ::std::string::String::as_str(
                    &::std::string::ToString::to_string(&value),
                ),
                serializer,
            )
        }
    }
    #[derive(::core::fmt::Debug)]
    pub enum DisabledValueParseError {
        NonExistent { value: ::std::string::String },
    }
    impl ::core::fmt::Display for DisabledValueParseError {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::result::Result<(), ::core::fmt::Error> {
            match self {
                DisabledValueParseError::NonExistent { value } => {
                    write!(f, "Value '{:?}' does not exist in the enumeration", value)
                }
            }
        }
    }
    impl ::core::convert::TryFrom<::std::string::String> for DisabledValue {
        type Error = DisabledValueParseError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::core::result::Result<Self, Self::Error> {
            match ::std::string::String::as_str(&value) {
                "disabled" => Ok(DisabledValue::Disabled),
                _ => {
                    Err(DisabledValueParseError::NonExistent {
                        value,
                    })
                }
            }
        }
    }
    impl ::core::convert::From<DisabledValue> for ::std::string::String {
        fn from(value: DisabledValue) -> Self {
            match value {
                DisabledValue::Disabled => ::std::string::String::from("disabled"),
            }
        }
    }
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
#[xelement(
    name = "select",
    namespace = "http://www.w3.org/1999/xhtml",
    allow_unknown_attributes = "any"
)]
pub struct Select {
    #[xattribute(name = "id", optional, default)]
    pub id: ::core::option::Option<String>,
    #[xattribute(name = "class", optional, default)]
    pub class: ::core::option::Option<String>,
    #[xattribute(name = "style", optional, default)]
    pub style: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::StyleSheet>,
    >,
    #[xattribute(name = "title", optional, default)]
    pub title: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Text>>,
    #[xattribute(name = "lang", optional, default)]
    pub lang: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::LanguageCode>,
    >,
    #[xattribute(deferred = true, optional, default)]
    pub attribute_5: ::core::option::Option<crate::xml::Lang>,
    #[xattribute(name = "dir", optional, default)]
    pub dir: ::core::option::Option<select_items::DirValue>,
    #[xattribute(name = "onclick", optional, default)]
    pub onclick: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xattribute(name = "ondblclick", optional, default)]
    pub ondblclick: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmousedown", optional, default)]
    pub onmousedown: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseup", optional, default)]
    pub onmouseup: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseover", optional, default)]
    pub onmouseover: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmousemove", optional, default)]
    pub onmousemove: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseout", optional, default)]
    pub onmouseout: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeypress", optional, default)]
    pub onkeypress: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeydown", optional, default)]
    pub onkeydown: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeyup", optional, default)]
    pub onkeyup: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xattribute(name = "name", optional, default)]
    pub name: ::core::option::Option<String>,
    #[xattribute(name = "size", optional, default)]
    pub size: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Number>>,
    #[xattribute(name = "multiple", optional, default)]
    pub multiple: ::core::option::Option<select_items::MultipleValue>,
    #[xattribute(name = "disabled", optional, default)]
    pub disabled: ::core::option::Option<select_items::DisabledValue>,
    #[xattribute(name = "tabindex", optional, default)]
    pub tabindex: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::TabindexNumber>,
    >,
    #[xattribute(name = "onfocus", optional, default)]
    pub onfocus: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xattribute(name = "onblur", optional, default)]
    pub onblur: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xattribute(name = "onchange", optional, default)]
    pub onchange: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xvalue(default)]
    pub select: ::std::vec::Vec<select_items::Select>,
}
pub mod small_items {
    impl ::core::convert::From<crate::xhtml::groups::Inline> for Inline {
        fn from(value: crate::xhtml::groups::Inline) -> Self {
            Inline::Inline(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml::groups::MiscInline> for Inline {
        fn from(value: crate::xhtml::groups::MiscInline) -> Self {
            Inline::MiscInline(::std::boxed::Box::new(value))
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub enum Inline {
        Inline(::std::boxed::Box<crate::xhtml::groups::Inline>),
        MiscInline(::std::boxed::Box<crate::xhtml::groups::MiscInline>),
    }
    #[derive(
        ::core::fmt::Debug,
        ::core::clone::Clone,
        ::core::marker::Copy,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq
    )]
    #[xvalue(with = dir_value_with)]
    pub enum DirValue {
        Ltr,
        Rtl,
    }
    pub mod dir_value_with {
        pub fn deserialize<'de, D>(
            deserializer: D,
        ) -> ::core::result::Result<super::DirValue, D::Error>
        where
            D: ::xmlity::Deserializer<'de>,
        {
            let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                deserializer,
            )?;
            let value: ::std::string::String = text
                .parse()
                .map_err(::xmlity::de::Error::custom)?;
            super::DirValue::try_from(value).map_err(::xmlity::de::Error::custom)
        }
        pub fn serialize<S>(
            value: &super::DirValue,
            serializer: S,
        ) -> ::core::result::Result<S::Ok, S::Error>
        where
            S: ::xmlity::Serializer,
        {
            let value: ::std::string::String = ::core::clone::Clone::clone(value).into();
            ::xmlity::Serialize::serialize(
                ::std::string::String::as_str(
                    &::std::string::ToString::to_string(&value),
                ),
                serializer,
            )
        }
    }
    #[derive(::core::fmt::Debug)]
    pub enum DirValueParseError {
        NonExistent { value: ::std::string::String },
    }
    impl ::core::fmt::Display for DirValueParseError {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::result::Result<(), ::core::fmt::Error> {
            match self {
                DirValueParseError::NonExistent { value } => {
                    write!(f, "Value '{:?}' does not exist in the enumeration", value)
                }
            }
        }
    }
    impl ::core::convert::TryFrom<::std::string::String> for DirValue {
        type Error = DirValueParseError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::core::result::Result<Self, Self::Error> {
            match ::std::string::String::as_str(&value) {
                "ltr" => Ok(DirValue::Ltr),
                "rtl" => Ok(DirValue::Rtl),
                _ => {
                    Err(DirValueParseError::NonExistent {
                        value,
                    })
                }
            }
        }
    }
    impl ::core::convert::From<DirValue> for ::std::string::String {
        fn from(value: DirValue) -> Self {
            match value {
                DirValue::Ltr => ::std::string::String::from("ltr"),
                DirValue::Rtl => ::std::string::String::from("rtl"),
            }
        }
    }
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
#[xelement(
    name = "small",
    namespace = "http://www.w3.org/1999/xhtml",
    allow_unknown_attributes = "any"
)]
pub struct Small {
    #[xattribute(name = "id", optional, default)]
    pub id: ::core::option::Option<String>,
    #[xattribute(name = "class", optional, default)]
    pub class: ::core::option::Option<String>,
    #[xattribute(name = "style", optional, default)]
    pub style: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::StyleSheet>,
    >,
    #[xattribute(name = "title", optional, default)]
    pub title: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Text>>,
    #[xattribute(name = "lang", optional, default)]
    pub lang: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::LanguageCode>,
    >,
    #[xattribute(deferred = true, optional, default)]
    pub attribute_5: ::core::option::Option<crate::xml::Lang>,
    #[xattribute(name = "dir", optional, default)]
    pub dir: ::core::option::Option<small_items::DirValue>,
    #[xattribute(name = "onclick", optional, default)]
    pub onclick: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xattribute(name = "ondblclick", optional, default)]
    pub ondblclick: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmousedown", optional, default)]
    pub onmousedown: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseup", optional, default)]
    pub onmouseup: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseover", optional, default)]
    pub onmouseover: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmousemove", optional, default)]
    pub onmousemove: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseout", optional, default)]
    pub onmouseout: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeypress", optional, default)]
    pub onkeypress: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeydown", optional, default)]
    pub onkeydown: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeyup", optional, default)]
    pub onkeyup: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xvalue(default)]
    pub inline: ::std::vec::Vec<small_items::Inline>,
}
pub mod span_items {
    impl ::core::convert::From<crate::xhtml::groups::Inline> for Inline {
        fn from(value: crate::xhtml::groups::Inline) -> Self {
            Inline::Inline(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml::groups::MiscInline> for Inline {
        fn from(value: crate::xhtml::groups::MiscInline) -> Self {
            Inline::MiscInline(::std::boxed::Box::new(value))
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub enum Inline {
        Inline(::std::boxed::Box<crate::xhtml::groups::Inline>),
        MiscInline(::std::boxed::Box<crate::xhtml::groups::MiscInline>),
    }
    #[derive(
        ::core::fmt::Debug,
        ::core::clone::Clone,
        ::core::marker::Copy,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq
    )]
    #[xvalue(with = dir_value_with)]
    pub enum DirValue {
        Ltr,
        Rtl,
    }
    pub mod dir_value_with {
        pub fn deserialize<'de, D>(
            deserializer: D,
        ) -> ::core::result::Result<super::DirValue, D::Error>
        where
            D: ::xmlity::Deserializer<'de>,
        {
            let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                deserializer,
            )?;
            let value: ::std::string::String = text
                .parse()
                .map_err(::xmlity::de::Error::custom)?;
            super::DirValue::try_from(value).map_err(::xmlity::de::Error::custom)
        }
        pub fn serialize<S>(
            value: &super::DirValue,
            serializer: S,
        ) -> ::core::result::Result<S::Ok, S::Error>
        where
            S: ::xmlity::Serializer,
        {
            let value: ::std::string::String = ::core::clone::Clone::clone(value).into();
            ::xmlity::Serialize::serialize(
                ::std::string::String::as_str(
                    &::std::string::ToString::to_string(&value),
                ),
                serializer,
            )
        }
    }
    #[derive(::core::fmt::Debug)]
    pub enum DirValueParseError {
        NonExistent { value: ::std::string::String },
    }
    impl ::core::fmt::Display for DirValueParseError {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::result::Result<(), ::core::fmt::Error> {
            match self {
                DirValueParseError::NonExistent { value } => {
                    write!(f, "Value '{:?}' does not exist in the enumeration", value)
                }
            }
        }
    }
    impl ::core::convert::TryFrom<::std::string::String> for DirValue {
        type Error = DirValueParseError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::core::result::Result<Self, Self::Error> {
            match ::std::string::String::as_str(&value) {
                "ltr" => Ok(DirValue::Ltr),
                "rtl" => Ok(DirValue::Rtl),
                _ => {
                    Err(DirValueParseError::NonExistent {
                        value,
                    })
                }
            }
        }
    }
    impl ::core::convert::From<DirValue> for ::std::string::String {
        fn from(value: DirValue) -> Self {
            match value {
                DirValue::Ltr => ::std::string::String::from("ltr"),
                DirValue::Rtl => ::std::string::String::from("rtl"),
            }
        }
    }
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
#[xelement(
    name = "span",
    namespace = "http://www.w3.org/1999/xhtml",
    allow_unknown_attributes = "any"
)]
pub struct Span {
    #[xattribute(name = "id", optional, default)]
    pub id: ::core::option::Option<String>,
    #[xattribute(name = "class", optional, default)]
    pub class: ::core::option::Option<String>,
    #[xattribute(name = "style", optional, default)]
    pub style: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::StyleSheet>,
    >,
    #[xattribute(name = "title", optional, default)]
    pub title: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Text>>,
    #[xattribute(name = "lang", optional, default)]
    pub lang: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::LanguageCode>,
    >,
    #[xattribute(deferred = true, optional, default)]
    pub attribute_5: ::core::option::Option<crate::xml::Lang>,
    #[xattribute(name = "dir", optional, default)]
    pub dir: ::core::option::Option<span_items::DirValue>,
    #[xattribute(name = "onclick", optional, default)]
    pub onclick: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xattribute(name = "ondblclick", optional, default)]
    pub ondblclick: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmousedown", optional, default)]
    pub onmousedown: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseup", optional, default)]
    pub onmouseup: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseover", optional, default)]
    pub onmouseover: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmousemove", optional, default)]
    pub onmousemove: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseout", optional, default)]
    pub onmouseout: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeypress", optional, default)]
    pub onkeypress: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeydown", optional, default)]
    pub onkeydown: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeyup", optional, default)]
    pub onkeyup: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xvalue(default)]
    pub inline: ::std::vec::Vec<span_items::Inline>,
}
pub mod strong_items {
    impl ::core::convert::From<crate::xhtml::groups::Inline> for Inline {
        fn from(value: crate::xhtml::groups::Inline) -> Self {
            Inline::Inline(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml::groups::MiscInline> for Inline {
        fn from(value: crate::xhtml::groups::MiscInline) -> Self {
            Inline::MiscInline(::std::boxed::Box::new(value))
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub enum Inline {
        Inline(::std::boxed::Box<crate::xhtml::groups::Inline>),
        MiscInline(::std::boxed::Box<crate::xhtml::groups::MiscInline>),
    }
    #[derive(
        ::core::fmt::Debug,
        ::core::clone::Clone,
        ::core::marker::Copy,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq
    )]
    #[xvalue(with = dir_value_with)]
    pub enum DirValue {
        Ltr,
        Rtl,
    }
    pub mod dir_value_with {
        pub fn deserialize<'de, D>(
            deserializer: D,
        ) -> ::core::result::Result<super::DirValue, D::Error>
        where
            D: ::xmlity::Deserializer<'de>,
        {
            let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                deserializer,
            )?;
            let value: ::std::string::String = text
                .parse()
                .map_err(::xmlity::de::Error::custom)?;
            super::DirValue::try_from(value).map_err(::xmlity::de::Error::custom)
        }
        pub fn serialize<S>(
            value: &super::DirValue,
            serializer: S,
        ) -> ::core::result::Result<S::Ok, S::Error>
        where
            S: ::xmlity::Serializer,
        {
            let value: ::std::string::String = ::core::clone::Clone::clone(value).into();
            ::xmlity::Serialize::serialize(
                ::std::string::String::as_str(
                    &::std::string::ToString::to_string(&value),
                ),
                serializer,
            )
        }
    }
    #[derive(::core::fmt::Debug)]
    pub enum DirValueParseError {
        NonExistent { value: ::std::string::String },
    }
    impl ::core::fmt::Display for DirValueParseError {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::result::Result<(), ::core::fmt::Error> {
            match self {
                DirValueParseError::NonExistent { value } => {
                    write!(f, "Value '{:?}' does not exist in the enumeration", value)
                }
            }
        }
    }
    impl ::core::convert::TryFrom<::std::string::String> for DirValue {
        type Error = DirValueParseError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::core::result::Result<Self, Self::Error> {
            match ::std::string::String::as_str(&value) {
                "ltr" => Ok(DirValue::Ltr),
                "rtl" => Ok(DirValue::Rtl),
                _ => {
                    Err(DirValueParseError::NonExistent {
                        value,
                    })
                }
            }
        }
    }
    impl ::core::convert::From<DirValue> for ::std::string::String {
        fn from(value: DirValue) -> Self {
            match value {
                DirValue::Ltr => ::std::string::String::from("ltr"),
                DirValue::Rtl => ::std::string::String::from("rtl"),
            }
        }
    }
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
#[xelement(
    name = "strong",
    namespace = "http://www.w3.org/1999/xhtml",
    allow_unknown_attributes = "any"
)]
pub struct Strong {
    #[xattribute(name = "id", optional, default)]
    pub id: ::core::option::Option<String>,
    #[xattribute(name = "class", optional, default)]
    pub class: ::core::option::Option<String>,
    #[xattribute(name = "style", optional, default)]
    pub style: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::StyleSheet>,
    >,
    #[xattribute(name = "title", optional, default)]
    pub title: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Text>>,
    #[xattribute(name = "lang", optional, default)]
    pub lang: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::LanguageCode>,
    >,
    #[xattribute(deferred = true, optional, default)]
    pub attribute_5: ::core::option::Option<crate::xml::Lang>,
    #[xattribute(name = "dir", optional, default)]
    pub dir: ::core::option::Option<strong_items::DirValue>,
    #[xattribute(name = "onclick", optional, default)]
    pub onclick: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xattribute(name = "ondblclick", optional, default)]
    pub ondblclick: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmousedown", optional, default)]
    pub onmousedown: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseup", optional, default)]
    pub onmouseup: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseover", optional, default)]
    pub onmouseover: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmousemove", optional, default)]
    pub onmousemove: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseout", optional, default)]
    pub onmouseout: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeypress", optional, default)]
    pub onkeypress: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeydown", optional, default)]
    pub onkeydown: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeyup", optional, default)]
    pub onkeyup: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xvalue(default)]
    pub inline: ::std::vec::Vec<strong_items::Inline>,
}
pub mod style_items {
    #[derive(
        ::core::fmt::Debug,
        ::core::clone::Clone,
        ::core::marker::Copy,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq
    )]
    #[xvalue(with = dir_value_with)]
    pub enum DirValue {
        Ltr,
        Rtl,
    }
    pub mod dir_value_with {
        pub fn deserialize<'de, D>(
            deserializer: D,
        ) -> ::core::result::Result<super::DirValue, D::Error>
        where
            D: ::xmlity::Deserializer<'de>,
        {
            let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                deserializer,
            )?;
            let value: ::std::string::String = text
                .parse()
                .map_err(::xmlity::de::Error::custom)?;
            super::DirValue::try_from(value).map_err(::xmlity::de::Error::custom)
        }
        pub fn serialize<S>(
            value: &super::DirValue,
            serializer: S,
        ) -> ::core::result::Result<S::Ok, S::Error>
        where
            S: ::xmlity::Serializer,
        {
            let value: ::std::string::String = ::core::clone::Clone::clone(value).into();
            ::xmlity::Serialize::serialize(
                ::std::string::String::as_str(
                    &::std::string::ToString::to_string(&value),
                ),
                serializer,
            )
        }
    }
    #[derive(::core::fmt::Debug)]
    pub enum DirValueParseError {
        NonExistent { value: ::std::string::String },
    }
    impl ::core::fmt::Display for DirValueParseError {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::result::Result<(), ::core::fmt::Error> {
            match self {
                DirValueParseError::NonExistent { value } => {
                    write!(f, "Value '{:?}' does not exist in the enumeration", value)
                }
            }
        }
    }
    impl ::core::convert::TryFrom<::std::string::String> for DirValue {
        type Error = DirValueParseError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::core::result::Result<Self, Self::Error> {
            match ::std::string::String::as_str(&value) {
                "ltr" => Ok(DirValue::Ltr),
                "rtl" => Ok(DirValue::Rtl),
                _ => {
                    Err(DirValueParseError::NonExistent {
                        value,
                    })
                }
            }
        }
    }
    impl ::core::convert::From<DirValue> for ::std::string::String {
        fn from(value: DirValue) -> Self {
            match value {
                DirValue::Ltr => ::std::string::String::from("ltr"),
                DirValue::Rtl => ::std::string::String::from("rtl"),
            }
        }
    }
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
#[xelement(
    name = "style",
    namespace = "http://www.w3.org/1999/xhtml",
    allow_unknown_attributes = "any"
)]
pub struct Style {
    #[xattribute(name = "lang", optional, default)]
    pub lang: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::LanguageCode>,
    >,
    #[xattribute(deferred = true, optional, default)]
    pub attribute_1: ::core::option::Option<crate::xml::Lang>,
    #[xattribute(name = "dir", optional, default)]
    pub dir: ::core::option::Option<style_items::DirValue>,
    #[xattribute(name = "id", optional, default)]
    pub id: ::core::option::Option<String>,
    #[xattribute(name = "type")]
    pub type_: ::std::boxed::Box<crate::xhtml::types::ContentType>,
    #[xattribute(name = "media", optional, default)]
    pub media: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::MediaDesc>>,
    #[xattribute(name = "title", optional, default)]
    pub title: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Text>>,
    #[xattribute(deferred = true, optional, default)]
    pub attribute_7: ::core::option::Option<crate::xml::Space>,
}
pub mod sub_items {
    impl ::core::convert::From<crate::xhtml::groups::Inline> for Inline {
        fn from(value: crate::xhtml::groups::Inline) -> Self {
            Inline::Inline(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml::groups::MiscInline> for Inline {
        fn from(value: crate::xhtml::groups::MiscInline) -> Self {
            Inline::MiscInline(::std::boxed::Box::new(value))
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub enum Inline {
        Inline(::std::boxed::Box<crate::xhtml::groups::Inline>),
        MiscInline(::std::boxed::Box<crate::xhtml::groups::MiscInline>),
    }
    #[derive(
        ::core::fmt::Debug,
        ::core::clone::Clone,
        ::core::marker::Copy,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq
    )]
    #[xvalue(with = dir_value_with)]
    pub enum DirValue {
        Ltr,
        Rtl,
    }
    pub mod dir_value_with {
        pub fn deserialize<'de, D>(
            deserializer: D,
        ) -> ::core::result::Result<super::DirValue, D::Error>
        where
            D: ::xmlity::Deserializer<'de>,
        {
            let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                deserializer,
            )?;
            let value: ::std::string::String = text
                .parse()
                .map_err(::xmlity::de::Error::custom)?;
            super::DirValue::try_from(value).map_err(::xmlity::de::Error::custom)
        }
        pub fn serialize<S>(
            value: &super::DirValue,
            serializer: S,
        ) -> ::core::result::Result<S::Ok, S::Error>
        where
            S: ::xmlity::Serializer,
        {
            let value: ::std::string::String = ::core::clone::Clone::clone(value).into();
            ::xmlity::Serialize::serialize(
                ::std::string::String::as_str(
                    &::std::string::ToString::to_string(&value),
                ),
                serializer,
            )
        }
    }
    #[derive(::core::fmt::Debug)]
    pub enum DirValueParseError {
        NonExistent { value: ::std::string::String },
    }
    impl ::core::fmt::Display for DirValueParseError {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::result::Result<(), ::core::fmt::Error> {
            match self {
                DirValueParseError::NonExistent { value } => {
                    write!(f, "Value '{:?}' does not exist in the enumeration", value)
                }
            }
        }
    }
    impl ::core::convert::TryFrom<::std::string::String> for DirValue {
        type Error = DirValueParseError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::core::result::Result<Self, Self::Error> {
            match ::std::string::String::as_str(&value) {
                "ltr" => Ok(DirValue::Ltr),
                "rtl" => Ok(DirValue::Rtl),
                _ => {
                    Err(DirValueParseError::NonExistent {
                        value,
                    })
                }
            }
        }
    }
    impl ::core::convert::From<DirValue> for ::std::string::String {
        fn from(value: DirValue) -> Self {
            match value {
                DirValue::Ltr => ::std::string::String::from("ltr"),
                DirValue::Rtl => ::std::string::String::from("rtl"),
            }
        }
    }
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
#[xelement(
    name = "sub",
    namespace = "http://www.w3.org/1999/xhtml",
    allow_unknown_attributes = "any"
)]
pub struct Sub {
    #[xattribute(name = "id", optional, default)]
    pub id: ::core::option::Option<String>,
    #[xattribute(name = "class", optional, default)]
    pub class: ::core::option::Option<String>,
    #[xattribute(name = "style", optional, default)]
    pub style: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::StyleSheet>,
    >,
    #[xattribute(name = "title", optional, default)]
    pub title: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Text>>,
    #[xattribute(name = "lang", optional, default)]
    pub lang: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::LanguageCode>,
    >,
    #[xattribute(deferred = true, optional, default)]
    pub attribute_5: ::core::option::Option<crate::xml::Lang>,
    #[xattribute(name = "dir", optional, default)]
    pub dir: ::core::option::Option<sub_items::DirValue>,
    #[xattribute(name = "onclick", optional, default)]
    pub onclick: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xattribute(name = "ondblclick", optional, default)]
    pub ondblclick: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmousedown", optional, default)]
    pub onmousedown: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseup", optional, default)]
    pub onmouseup: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseover", optional, default)]
    pub onmouseover: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmousemove", optional, default)]
    pub onmousemove: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseout", optional, default)]
    pub onmouseout: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeypress", optional, default)]
    pub onkeypress: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeydown", optional, default)]
    pub onkeydown: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeyup", optional, default)]
    pub onkeyup: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xvalue(default)]
    pub inline: ::std::vec::Vec<sub_items::Inline>,
}
pub mod sup_items {
    impl ::core::convert::From<crate::xhtml::groups::Inline> for Inline {
        fn from(value: crate::xhtml::groups::Inline) -> Self {
            Inline::Inline(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml::groups::MiscInline> for Inline {
        fn from(value: crate::xhtml::groups::MiscInline) -> Self {
            Inline::MiscInline(::std::boxed::Box::new(value))
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub enum Inline {
        Inline(::std::boxed::Box<crate::xhtml::groups::Inline>),
        MiscInline(::std::boxed::Box<crate::xhtml::groups::MiscInline>),
    }
    #[derive(
        ::core::fmt::Debug,
        ::core::clone::Clone,
        ::core::marker::Copy,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq
    )]
    #[xvalue(with = dir_value_with)]
    pub enum DirValue {
        Ltr,
        Rtl,
    }
    pub mod dir_value_with {
        pub fn deserialize<'de, D>(
            deserializer: D,
        ) -> ::core::result::Result<super::DirValue, D::Error>
        where
            D: ::xmlity::Deserializer<'de>,
        {
            let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                deserializer,
            )?;
            let value: ::std::string::String = text
                .parse()
                .map_err(::xmlity::de::Error::custom)?;
            super::DirValue::try_from(value).map_err(::xmlity::de::Error::custom)
        }
        pub fn serialize<S>(
            value: &super::DirValue,
            serializer: S,
        ) -> ::core::result::Result<S::Ok, S::Error>
        where
            S: ::xmlity::Serializer,
        {
            let value: ::std::string::String = ::core::clone::Clone::clone(value).into();
            ::xmlity::Serialize::serialize(
                ::std::string::String::as_str(
                    &::std::string::ToString::to_string(&value),
                ),
                serializer,
            )
        }
    }
    #[derive(::core::fmt::Debug)]
    pub enum DirValueParseError {
        NonExistent { value: ::std::string::String },
    }
    impl ::core::fmt::Display for DirValueParseError {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::result::Result<(), ::core::fmt::Error> {
            match self {
                DirValueParseError::NonExistent { value } => {
                    write!(f, "Value '{:?}' does not exist in the enumeration", value)
                }
            }
        }
    }
    impl ::core::convert::TryFrom<::std::string::String> for DirValue {
        type Error = DirValueParseError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::core::result::Result<Self, Self::Error> {
            match ::std::string::String::as_str(&value) {
                "ltr" => Ok(DirValue::Ltr),
                "rtl" => Ok(DirValue::Rtl),
                _ => {
                    Err(DirValueParseError::NonExistent {
                        value,
                    })
                }
            }
        }
    }
    impl ::core::convert::From<DirValue> for ::std::string::String {
        fn from(value: DirValue) -> Self {
            match value {
                DirValue::Ltr => ::std::string::String::from("ltr"),
                DirValue::Rtl => ::std::string::String::from("rtl"),
            }
        }
    }
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
#[xelement(
    name = "sup",
    namespace = "http://www.w3.org/1999/xhtml",
    allow_unknown_attributes = "any"
)]
pub struct Sup {
    #[xattribute(name = "id", optional, default)]
    pub id: ::core::option::Option<String>,
    #[xattribute(name = "class", optional, default)]
    pub class: ::core::option::Option<String>,
    #[xattribute(name = "style", optional, default)]
    pub style: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::StyleSheet>,
    >,
    #[xattribute(name = "title", optional, default)]
    pub title: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Text>>,
    #[xattribute(name = "lang", optional, default)]
    pub lang: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::LanguageCode>,
    >,
    #[xattribute(deferred = true, optional, default)]
    pub attribute_5: ::core::option::Option<crate::xml::Lang>,
    #[xattribute(name = "dir", optional, default)]
    pub dir: ::core::option::Option<sup_items::DirValue>,
    #[xattribute(name = "onclick", optional, default)]
    pub onclick: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xattribute(name = "ondblclick", optional, default)]
    pub ondblclick: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmousedown", optional, default)]
    pub onmousedown: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseup", optional, default)]
    pub onmouseup: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseover", optional, default)]
    pub onmouseover: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmousemove", optional, default)]
    pub onmousemove: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseout", optional, default)]
    pub onmouseout: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeypress", optional, default)]
    pub onkeypress: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeydown", optional, default)]
    pub onkeydown: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeyup", optional, default)]
    pub onkeyup: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xvalue(default)]
    pub inline: ::std::vec::Vec<sup_items::Inline>,
}
pub mod table_items {
    impl ::core::convert::From<::std::vec::Vec<crate::xhtml::Col>> for Child1 {
        fn from(value: ::std::vec::Vec<crate::xhtml::Col>) -> Self {
            Child1::Col(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<::std::vec::Vec<crate::xhtml::Colgroup>> for Child1 {
        fn from(value: ::std::vec::Vec<crate::xhtml::Colgroup>) -> Self {
            Child1::Colgroup(::std::boxed::Box::new(value))
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub enum Child1 {
        Col(#[xvalue(default)] ::std::boxed::Box<::std::vec::Vec<crate::xhtml::Col>>),
        Colgroup(
            #[xvalue(default)]
            ::std::boxed::Box<::std::vec::Vec<crate::xhtml::Colgroup>>,
        ),
    }
    impl ::core::convert::From<::std::vec::Vec<crate::xhtml::Tbody>> for Child4 {
        fn from(value: ::std::vec::Vec<crate::xhtml::Tbody>) -> Self {
            Child4::Tbody(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<::std::vec::Vec<crate::xhtml::Tr>> for Child4 {
        fn from(value: ::std::vec::Vec<crate::xhtml::Tr>) -> Self {
            Child4::Tr(::std::boxed::Box::new(value))
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub enum Child4 {
        Tbody(
            #[xvalue(default)]
            ::std::boxed::Box<::std::vec::Vec<crate::xhtml::Tbody>>,
        ),
        Tr(#[xvalue(default)] ::std::boxed::Box<::std::vec::Vec<crate::xhtml::Tr>>),
    }
    #[derive(
        ::core::fmt::Debug,
        ::core::clone::Clone,
        ::core::marker::Copy,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq
    )]
    #[xvalue(with = dir_value_with)]
    pub enum DirValue {
        Ltr,
        Rtl,
    }
    pub mod dir_value_with {
        pub fn deserialize<'de, D>(
            deserializer: D,
        ) -> ::core::result::Result<super::DirValue, D::Error>
        where
            D: ::xmlity::Deserializer<'de>,
        {
            let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                deserializer,
            )?;
            let value: ::std::string::String = text
                .parse()
                .map_err(::xmlity::de::Error::custom)?;
            super::DirValue::try_from(value).map_err(::xmlity::de::Error::custom)
        }
        pub fn serialize<S>(
            value: &super::DirValue,
            serializer: S,
        ) -> ::core::result::Result<S::Ok, S::Error>
        where
            S: ::xmlity::Serializer,
        {
            let value: ::std::string::String = ::core::clone::Clone::clone(value).into();
            ::xmlity::Serialize::serialize(
                ::std::string::String::as_str(
                    &::std::string::ToString::to_string(&value),
                ),
                serializer,
            )
        }
    }
    #[derive(::core::fmt::Debug)]
    pub enum DirValueParseError {
        NonExistent { value: ::std::string::String },
    }
    impl ::core::fmt::Display for DirValueParseError {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::result::Result<(), ::core::fmt::Error> {
            match self {
                DirValueParseError::NonExistent { value } => {
                    write!(f, "Value '{:?}' does not exist in the enumeration", value)
                }
            }
        }
    }
    impl ::core::convert::TryFrom<::std::string::String> for DirValue {
        type Error = DirValueParseError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::core::result::Result<Self, Self::Error> {
            match ::std::string::String::as_str(&value) {
                "ltr" => Ok(DirValue::Ltr),
                "rtl" => Ok(DirValue::Rtl),
                _ => {
                    Err(DirValueParseError::NonExistent {
                        value,
                    })
                }
            }
        }
    }
    impl ::core::convert::From<DirValue> for ::std::string::String {
        fn from(value: DirValue) -> Self {
            match value {
                DirValue::Ltr => ::std::string::String::from("ltr"),
                DirValue::Rtl => ::std::string::String::from("rtl"),
            }
        }
    }
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
#[xelement(
    name = "table",
    namespace = "http://www.w3.org/1999/xhtml",
    allow_unknown_attributes = "any",
    children_order = "strict"
)]
pub struct Table {
    #[xattribute(name = "id", optional, default)]
    pub id: ::core::option::Option<String>,
    #[xattribute(name = "class", optional, default)]
    pub class: ::core::option::Option<String>,
    #[xattribute(name = "style", optional, default)]
    pub style: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::StyleSheet>,
    >,
    #[xattribute(name = "title", optional, default)]
    pub title: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Text>>,
    #[xattribute(name = "lang", optional, default)]
    pub lang: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::LanguageCode>,
    >,
    #[xattribute(deferred = true, optional, default)]
    pub attribute_5: ::core::option::Option<crate::xml::Lang>,
    #[xattribute(name = "dir", optional, default)]
    pub dir: ::core::option::Option<table_items::DirValue>,
    #[xattribute(name = "onclick", optional, default)]
    pub onclick: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xattribute(name = "ondblclick", optional, default)]
    pub ondblclick: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmousedown", optional, default)]
    pub onmousedown: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseup", optional, default)]
    pub onmouseup: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseover", optional, default)]
    pub onmouseover: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmousemove", optional, default)]
    pub onmousemove: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseout", optional, default)]
    pub onmouseout: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeypress", optional, default)]
    pub onkeypress: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeydown", optional, default)]
    pub onkeydown: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeyup", optional, default)]
    pub onkeyup: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xattribute(name = "summary", optional, default)]
    pub summary: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Text>>,
    #[xattribute(name = "width", optional, default)]
    pub width: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Length>>,
    #[xattribute(name = "border", optional, default)]
    pub border: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Pixels>>,
    #[xattribute(name = "frame", optional, default)]
    pub frame: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Tframe>>,
    #[xattribute(name = "rules", optional, default)]
    pub rules: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Trules>>,
    #[xattribute(name = "cellspacing", optional, default)]
    pub cellspacing: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Length>,
    >,
    #[xattribute(name = "cellpadding", optional, default)]
    pub cellpadding: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Length>,
    >,
    #[xvalue(default)]
    pub caption: ::core::option::Option<crate::xhtml::Caption>,
    pub child_1: table_items::Child1,
    #[xvalue(default)]
    pub thead: ::core::option::Option<::std::boxed::Box<crate::xhtml::Thead>>,
    #[xvalue(default)]
    pub tfoot: ::core::option::Option<::std::boxed::Box<crate::xhtml::Tfoot>>,
    pub child_4: table_items::Child4,
}
pub mod tbody_items {
    #[derive(
        ::core::fmt::Debug,
        ::core::clone::Clone,
        ::core::marker::Copy,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq
    )]
    #[xvalue(with = dir_value_with)]
    pub enum DirValue {
        Ltr,
        Rtl,
    }
    pub mod dir_value_with {
        pub fn deserialize<'de, D>(
            deserializer: D,
        ) -> ::core::result::Result<super::DirValue, D::Error>
        where
            D: ::xmlity::Deserializer<'de>,
        {
            let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                deserializer,
            )?;
            let value: ::std::string::String = text
                .parse()
                .map_err(::xmlity::de::Error::custom)?;
            super::DirValue::try_from(value).map_err(::xmlity::de::Error::custom)
        }
        pub fn serialize<S>(
            value: &super::DirValue,
            serializer: S,
        ) -> ::core::result::Result<S::Ok, S::Error>
        where
            S: ::xmlity::Serializer,
        {
            let value: ::std::string::String = ::core::clone::Clone::clone(value).into();
            ::xmlity::Serialize::serialize(
                ::std::string::String::as_str(
                    &::std::string::ToString::to_string(&value),
                ),
                serializer,
            )
        }
    }
    #[derive(::core::fmt::Debug)]
    pub enum DirValueParseError {
        NonExistent { value: ::std::string::String },
    }
    impl ::core::fmt::Display for DirValueParseError {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::result::Result<(), ::core::fmt::Error> {
            match self {
                DirValueParseError::NonExistent { value } => {
                    write!(f, "Value '{:?}' does not exist in the enumeration", value)
                }
            }
        }
    }
    impl ::core::convert::TryFrom<::std::string::String> for DirValue {
        type Error = DirValueParseError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::core::result::Result<Self, Self::Error> {
            match ::std::string::String::as_str(&value) {
                "ltr" => Ok(DirValue::Ltr),
                "rtl" => Ok(DirValue::Rtl),
                _ => {
                    Err(DirValueParseError::NonExistent {
                        value,
                    })
                }
            }
        }
    }
    impl ::core::convert::From<DirValue> for ::std::string::String {
        fn from(value: DirValue) -> Self {
            match value {
                DirValue::Ltr => ::std::string::String::from("ltr"),
                DirValue::Rtl => ::std::string::String::from("rtl"),
            }
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::core::clone::Clone,
        ::core::marker::Copy,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq
    )]
    #[xvalue(with = align_value_with)]
    pub enum AlignValue {
        Left,
        Center,
        Right,
        Justify,
        Char,
    }
    pub mod align_value_with {
        pub fn deserialize<'de, D>(
            deserializer: D,
        ) -> ::core::result::Result<super::AlignValue, D::Error>
        where
            D: ::xmlity::Deserializer<'de>,
        {
            let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                deserializer,
            )?;
            let value: ::std::string::String = text
                .parse()
                .map_err(::xmlity::de::Error::custom)?;
            super::AlignValue::try_from(value).map_err(::xmlity::de::Error::custom)
        }
        pub fn serialize<S>(
            value: &super::AlignValue,
            serializer: S,
        ) -> ::core::result::Result<S::Ok, S::Error>
        where
            S: ::xmlity::Serializer,
        {
            let value: ::std::string::String = ::core::clone::Clone::clone(value).into();
            ::xmlity::Serialize::serialize(
                ::std::string::String::as_str(
                    &::std::string::ToString::to_string(&value),
                ),
                serializer,
            )
        }
    }
    #[derive(::core::fmt::Debug)]
    pub enum AlignValueParseError {
        NonExistent { value: ::std::string::String },
    }
    impl ::core::fmt::Display for AlignValueParseError {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::result::Result<(), ::core::fmt::Error> {
            match self {
                AlignValueParseError::NonExistent { value } => {
                    write!(f, "Value '{:?}' does not exist in the enumeration", value)
                }
            }
        }
    }
    impl ::core::convert::TryFrom<::std::string::String> for AlignValue {
        type Error = AlignValueParseError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::core::result::Result<Self, Self::Error> {
            match ::std::string::String::as_str(&value) {
                "left" => Ok(AlignValue::Left),
                "center" => Ok(AlignValue::Center),
                "right" => Ok(AlignValue::Right),
                "justify" => Ok(AlignValue::Justify),
                "char" => Ok(AlignValue::Char),
                _ => {
                    Err(AlignValueParseError::NonExistent {
                        value,
                    })
                }
            }
        }
    }
    impl ::core::convert::From<AlignValue> for ::std::string::String {
        fn from(value: AlignValue) -> Self {
            match value {
                AlignValue::Left => ::std::string::String::from("left"),
                AlignValue::Center => ::std::string::String::from("center"),
                AlignValue::Right => ::std::string::String::from("right"),
                AlignValue::Justify => ::std::string::String::from("justify"),
                AlignValue::Char => ::std::string::String::from("char"),
            }
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::core::clone::Clone,
        ::core::marker::Copy,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq
    )]
    #[xvalue(with = valign_value_with)]
    pub enum ValignValue {
        Top,
        Middle,
        Bottom,
        Baseline,
    }
    pub mod valign_value_with {
        pub fn deserialize<'de, D>(
            deserializer: D,
        ) -> ::core::result::Result<super::ValignValue, D::Error>
        where
            D: ::xmlity::Deserializer<'de>,
        {
            let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                deserializer,
            )?;
            let value: ::std::string::String = text
                .parse()
                .map_err(::xmlity::de::Error::custom)?;
            super::ValignValue::try_from(value).map_err(::xmlity::de::Error::custom)
        }
        pub fn serialize<S>(
            value: &super::ValignValue,
            serializer: S,
        ) -> ::core::result::Result<S::Ok, S::Error>
        where
            S: ::xmlity::Serializer,
        {
            let value: ::std::string::String = ::core::clone::Clone::clone(value).into();
            ::xmlity::Serialize::serialize(
                ::std::string::String::as_str(
                    &::std::string::ToString::to_string(&value),
                ),
                serializer,
            )
        }
    }
    #[derive(::core::fmt::Debug)]
    pub enum ValignValueParseError {
        NonExistent { value: ::std::string::String },
    }
    impl ::core::fmt::Display for ValignValueParseError {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::result::Result<(), ::core::fmt::Error> {
            match self {
                ValignValueParseError::NonExistent { value } => {
                    write!(f, "Value '{:?}' does not exist in the enumeration", value)
                }
            }
        }
    }
    impl ::core::convert::TryFrom<::std::string::String> for ValignValue {
        type Error = ValignValueParseError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::core::result::Result<Self, Self::Error> {
            match ::std::string::String::as_str(&value) {
                "top" => Ok(ValignValue::Top),
                "middle" => Ok(ValignValue::Middle),
                "bottom" => Ok(ValignValue::Bottom),
                "baseline" => Ok(ValignValue::Baseline),
                _ => {
                    Err(ValignValueParseError::NonExistent {
                        value,
                    })
                }
            }
        }
    }
    impl ::core::convert::From<ValignValue> for ::std::string::String {
        fn from(value: ValignValue) -> Self {
            match value {
                ValignValue::Top => ::std::string::String::from("top"),
                ValignValue::Middle => ::std::string::String::from("middle"),
                ValignValue::Bottom => ::std::string::String::from("bottom"),
                ValignValue::Baseline => ::std::string::String::from("baseline"),
            }
        }
    }
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
#[xelement(
    name = "tbody",
    namespace = "http://www.w3.org/1999/xhtml",
    allow_unknown_attributes = "any",
    children_order = "strict"
)]
pub struct Tbody {
    #[xattribute(name = "id", optional, default)]
    pub id: ::core::option::Option<String>,
    #[xattribute(name = "class", optional, default)]
    pub class: ::core::option::Option<String>,
    #[xattribute(name = "style", optional, default)]
    pub style: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::StyleSheet>,
    >,
    #[xattribute(name = "title", optional, default)]
    pub title: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Text>>,
    #[xattribute(name = "lang", optional, default)]
    pub lang: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::LanguageCode>,
    >,
    #[xattribute(deferred = true, optional, default)]
    pub attribute_5: ::core::option::Option<crate::xml::Lang>,
    #[xattribute(name = "dir", optional, default)]
    pub dir: ::core::option::Option<tbody_items::DirValue>,
    #[xattribute(name = "onclick", optional, default)]
    pub onclick: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xattribute(name = "ondblclick", optional, default)]
    pub ondblclick: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmousedown", optional, default)]
    pub onmousedown: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseup", optional, default)]
    pub onmouseup: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseover", optional, default)]
    pub onmouseover: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmousemove", optional, default)]
    pub onmousemove: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseout", optional, default)]
    pub onmouseout: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeypress", optional, default)]
    pub onkeypress: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeydown", optional, default)]
    pub onkeydown: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeyup", optional, default)]
    pub onkeyup: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xattribute(name = "align", optional, default)]
    pub align: ::core::option::Option<tbody_items::AlignValue>,
    #[xattribute(name = "char", optional, default)]
    pub char: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Character>>,
    #[xattribute(name = "charoff", optional, default)]
    pub charoff: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Length>>,
    #[xattribute(name = "valign", optional, default)]
    pub valign: ::core::option::Option<tbody_items::ValignValue>,
    #[xvalue(default)]
    pub tr: ::std::vec::Vec<crate::xhtml::Tr>,
}
pub mod td_items {
    impl ::core::convert::From<crate::xhtml::groups::Block> for Td {
        fn from(value: crate::xhtml::groups::Block) -> Self {
            Td::Block(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml::Form> for Td {
        fn from(value: crate::xhtml::Form) -> Self {
            Td::Form(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml::groups::Inline> for Td {
        fn from(value: crate::xhtml::groups::Inline) -> Self {
            Td::Inline(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml::groups::Misc> for Td {
        fn from(value: crate::xhtml::groups::Misc) -> Self {
            Td::Misc(::std::boxed::Box::new(value))
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub enum Td {
        Block(::std::boxed::Box<crate::xhtml::groups::Block>),
        Form(::std::boxed::Box<crate::xhtml::Form>),
        Inline(::std::boxed::Box<crate::xhtml::groups::Inline>),
        Misc(::std::boxed::Box<crate::xhtml::groups::Misc>),
    }
    #[derive(
        ::core::fmt::Debug,
        ::core::clone::Clone,
        ::core::marker::Copy,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq
    )]
    #[xvalue(with = dir_value_with)]
    pub enum DirValue {
        Ltr,
        Rtl,
    }
    pub mod dir_value_with {
        pub fn deserialize<'de, D>(
            deserializer: D,
        ) -> ::core::result::Result<super::DirValue, D::Error>
        where
            D: ::xmlity::Deserializer<'de>,
        {
            let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                deserializer,
            )?;
            let value: ::std::string::String = text
                .parse()
                .map_err(::xmlity::de::Error::custom)?;
            super::DirValue::try_from(value).map_err(::xmlity::de::Error::custom)
        }
        pub fn serialize<S>(
            value: &super::DirValue,
            serializer: S,
        ) -> ::core::result::Result<S::Ok, S::Error>
        where
            S: ::xmlity::Serializer,
        {
            let value: ::std::string::String = ::core::clone::Clone::clone(value).into();
            ::xmlity::Serialize::serialize(
                ::std::string::String::as_str(
                    &::std::string::ToString::to_string(&value),
                ),
                serializer,
            )
        }
    }
    #[derive(::core::fmt::Debug)]
    pub enum DirValueParseError {
        NonExistent { value: ::std::string::String },
    }
    impl ::core::fmt::Display for DirValueParseError {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::result::Result<(), ::core::fmt::Error> {
            match self {
                DirValueParseError::NonExistent { value } => {
                    write!(f, "Value '{:?}' does not exist in the enumeration", value)
                }
            }
        }
    }
    impl ::core::convert::TryFrom<::std::string::String> for DirValue {
        type Error = DirValueParseError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::core::result::Result<Self, Self::Error> {
            match ::std::string::String::as_str(&value) {
                "ltr" => Ok(DirValue::Ltr),
                "rtl" => Ok(DirValue::Rtl),
                _ => {
                    Err(DirValueParseError::NonExistent {
                        value,
                    })
                }
            }
        }
    }
    impl ::core::convert::From<DirValue> for ::std::string::String {
        fn from(value: DirValue) -> Self {
            match value {
                DirValue::Ltr => ::std::string::String::from("ltr"),
                DirValue::Rtl => ::std::string::String::from("rtl"),
            }
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::core::clone::Clone,
        ::core::marker::Copy,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq
    )]
    #[xvalue(with = align_value_with)]
    pub enum AlignValue {
        Left,
        Center,
        Right,
        Justify,
        Char,
    }
    pub mod align_value_with {
        pub fn deserialize<'de, D>(
            deserializer: D,
        ) -> ::core::result::Result<super::AlignValue, D::Error>
        where
            D: ::xmlity::Deserializer<'de>,
        {
            let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                deserializer,
            )?;
            let value: ::std::string::String = text
                .parse()
                .map_err(::xmlity::de::Error::custom)?;
            super::AlignValue::try_from(value).map_err(::xmlity::de::Error::custom)
        }
        pub fn serialize<S>(
            value: &super::AlignValue,
            serializer: S,
        ) -> ::core::result::Result<S::Ok, S::Error>
        where
            S: ::xmlity::Serializer,
        {
            let value: ::std::string::String = ::core::clone::Clone::clone(value).into();
            ::xmlity::Serialize::serialize(
                ::std::string::String::as_str(
                    &::std::string::ToString::to_string(&value),
                ),
                serializer,
            )
        }
    }
    #[derive(::core::fmt::Debug)]
    pub enum AlignValueParseError {
        NonExistent { value: ::std::string::String },
    }
    impl ::core::fmt::Display for AlignValueParseError {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::result::Result<(), ::core::fmt::Error> {
            match self {
                AlignValueParseError::NonExistent { value } => {
                    write!(f, "Value '{:?}' does not exist in the enumeration", value)
                }
            }
        }
    }
    impl ::core::convert::TryFrom<::std::string::String> for AlignValue {
        type Error = AlignValueParseError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::core::result::Result<Self, Self::Error> {
            match ::std::string::String::as_str(&value) {
                "left" => Ok(AlignValue::Left),
                "center" => Ok(AlignValue::Center),
                "right" => Ok(AlignValue::Right),
                "justify" => Ok(AlignValue::Justify),
                "char" => Ok(AlignValue::Char),
                _ => {
                    Err(AlignValueParseError::NonExistent {
                        value,
                    })
                }
            }
        }
    }
    impl ::core::convert::From<AlignValue> for ::std::string::String {
        fn from(value: AlignValue) -> Self {
            match value {
                AlignValue::Left => ::std::string::String::from("left"),
                AlignValue::Center => ::std::string::String::from("center"),
                AlignValue::Right => ::std::string::String::from("right"),
                AlignValue::Justify => ::std::string::String::from("justify"),
                AlignValue::Char => ::std::string::String::from("char"),
            }
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::core::clone::Clone,
        ::core::marker::Copy,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq
    )]
    #[xvalue(with = valign_value_with)]
    pub enum ValignValue {
        Top,
        Middle,
        Bottom,
        Baseline,
    }
    pub mod valign_value_with {
        pub fn deserialize<'de, D>(
            deserializer: D,
        ) -> ::core::result::Result<super::ValignValue, D::Error>
        where
            D: ::xmlity::Deserializer<'de>,
        {
            let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                deserializer,
            )?;
            let value: ::std::string::String = text
                .parse()
                .map_err(::xmlity::de::Error::custom)?;
            super::ValignValue::try_from(value).map_err(::xmlity::de::Error::custom)
        }
        pub fn serialize<S>(
            value: &super::ValignValue,
            serializer: S,
        ) -> ::core::result::Result<S::Ok, S::Error>
        where
            S: ::xmlity::Serializer,
        {
            let value: ::std::string::String = ::core::clone::Clone::clone(value).into();
            ::xmlity::Serialize::serialize(
                ::std::string::String::as_str(
                    &::std::string::ToString::to_string(&value),
                ),
                serializer,
            )
        }
    }
    #[derive(::core::fmt::Debug)]
    pub enum ValignValueParseError {
        NonExistent { value: ::std::string::String },
    }
    impl ::core::fmt::Display for ValignValueParseError {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::result::Result<(), ::core::fmt::Error> {
            match self {
                ValignValueParseError::NonExistent { value } => {
                    write!(f, "Value '{:?}' does not exist in the enumeration", value)
                }
            }
        }
    }
    impl ::core::convert::TryFrom<::std::string::String> for ValignValue {
        type Error = ValignValueParseError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::core::result::Result<Self, Self::Error> {
            match ::std::string::String::as_str(&value) {
                "top" => Ok(ValignValue::Top),
                "middle" => Ok(ValignValue::Middle),
                "bottom" => Ok(ValignValue::Bottom),
                "baseline" => Ok(ValignValue::Baseline),
                _ => {
                    Err(ValignValueParseError::NonExistent {
                        value,
                    })
                }
            }
        }
    }
    impl ::core::convert::From<ValignValue> for ::std::string::String {
        fn from(value: ValignValue) -> Self {
            match value {
                ValignValue::Top => ::std::string::String::from("top"),
                ValignValue::Middle => ::std::string::String::from("middle"),
                ValignValue::Bottom => ::std::string::String::from("bottom"),
                ValignValue::Baseline => ::std::string::String::from("baseline"),
            }
        }
    }
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
#[xelement(
    name = "td",
    namespace = "http://www.w3.org/1999/xhtml",
    allow_unknown_attributes = "any"
)]
pub struct Td {
    #[xattribute(name = "id", optional, default)]
    pub id: ::core::option::Option<String>,
    #[xattribute(name = "class", optional, default)]
    pub class: ::core::option::Option<String>,
    #[xattribute(name = "style", optional, default)]
    pub style: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::StyleSheet>,
    >,
    #[xattribute(name = "title", optional, default)]
    pub title: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Text>>,
    #[xattribute(name = "lang", optional, default)]
    pub lang: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::LanguageCode>,
    >,
    #[xattribute(deferred = true, optional, default)]
    pub attribute_5: ::core::option::Option<crate::xml::Lang>,
    #[xattribute(name = "dir", optional, default)]
    pub dir: ::core::option::Option<td_items::DirValue>,
    #[xattribute(name = "onclick", optional, default)]
    pub onclick: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xattribute(name = "ondblclick", optional, default)]
    pub ondblclick: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmousedown", optional, default)]
    pub onmousedown: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseup", optional, default)]
    pub onmouseup: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseover", optional, default)]
    pub onmouseover: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmousemove", optional, default)]
    pub onmousemove: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseout", optional, default)]
    pub onmouseout: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeypress", optional, default)]
    pub onkeypress: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeydown", optional, default)]
    pub onkeydown: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeyup", optional, default)]
    pub onkeyup: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xattribute(name = "abbr", optional, default)]
    pub abbr: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Text>>,
    #[xattribute(name = "axis", optional, default)]
    pub axis: ::core::option::Option<String>,
    #[xattribute(name = "headers", optional, default)]
    pub headers: ::core::option::Option<String>,
    #[xattribute(name = "scope", optional, default)]
    pub scope: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Scope>>,
    #[xattribute(name = "rowspan", optional, default)]
    pub rowspan: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Number>>,
    #[xattribute(name = "colspan", optional, default)]
    pub colspan: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Number>>,
    #[xattribute(name = "align", optional, default)]
    pub align: ::core::option::Option<td_items::AlignValue>,
    #[xattribute(name = "char", optional, default)]
    pub char: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Character>>,
    #[xattribute(name = "charoff", optional, default)]
    pub charoff: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Length>>,
    #[xattribute(name = "valign", optional, default)]
    pub valign: ::core::option::Option<td_items::ValignValue>,
    #[xvalue(default)]
    pub td: ::std::vec::Vec<td_items::Td>,
}
pub mod textarea_items {
    #[derive(
        ::core::fmt::Debug,
        ::core::clone::Clone,
        ::core::marker::Copy,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq
    )]
    #[xvalue(with = dir_value_with)]
    pub enum DirValue {
        Ltr,
        Rtl,
    }
    pub mod dir_value_with {
        pub fn deserialize<'de, D>(
            deserializer: D,
        ) -> ::core::result::Result<super::DirValue, D::Error>
        where
            D: ::xmlity::Deserializer<'de>,
        {
            let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                deserializer,
            )?;
            let value: ::std::string::String = text
                .parse()
                .map_err(::xmlity::de::Error::custom)?;
            super::DirValue::try_from(value).map_err(::xmlity::de::Error::custom)
        }
        pub fn serialize<S>(
            value: &super::DirValue,
            serializer: S,
        ) -> ::core::result::Result<S::Ok, S::Error>
        where
            S: ::xmlity::Serializer,
        {
            let value: ::std::string::String = ::core::clone::Clone::clone(value).into();
            ::xmlity::Serialize::serialize(
                ::std::string::String::as_str(
                    &::std::string::ToString::to_string(&value),
                ),
                serializer,
            )
        }
    }
    #[derive(::core::fmt::Debug)]
    pub enum DirValueParseError {
        NonExistent { value: ::std::string::String },
    }
    impl ::core::fmt::Display for DirValueParseError {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::result::Result<(), ::core::fmt::Error> {
            match self {
                DirValueParseError::NonExistent { value } => {
                    write!(f, "Value '{:?}' does not exist in the enumeration", value)
                }
            }
        }
    }
    impl ::core::convert::TryFrom<::std::string::String> for DirValue {
        type Error = DirValueParseError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::core::result::Result<Self, Self::Error> {
            match ::std::string::String::as_str(&value) {
                "ltr" => Ok(DirValue::Ltr),
                "rtl" => Ok(DirValue::Rtl),
                _ => {
                    Err(DirValueParseError::NonExistent {
                        value,
                    })
                }
            }
        }
    }
    impl ::core::convert::From<DirValue> for ::std::string::String {
        fn from(value: DirValue) -> Self {
            match value {
                DirValue::Ltr => ::std::string::String::from("ltr"),
                DirValue::Rtl => ::std::string::String::from("rtl"),
            }
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::core::clone::Clone,
        ::core::marker::Copy,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq
    )]
    #[xvalue(with = disabled_value_with)]
    pub enum DisabledValue {
        Disabled,
    }
    pub mod disabled_value_with {
        pub fn deserialize<'de, D>(
            deserializer: D,
        ) -> ::core::result::Result<super::DisabledValue, D::Error>
        where
            D: ::xmlity::Deserializer<'de>,
        {
            let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                deserializer,
            )?;
            let value: ::std::string::String = text
                .parse()
                .map_err(::xmlity::de::Error::custom)?;
            super::DisabledValue::try_from(value).map_err(::xmlity::de::Error::custom)
        }
        pub fn serialize<S>(
            value: &super::DisabledValue,
            serializer: S,
        ) -> ::core::result::Result<S::Ok, S::Error>
        where
            S: ::xmlity::Serializer,
        {
            let value: ::std::string::String = ::core::clone::Clone::clone(value).into();
            ::xmlity::Serialize::serialize(
                ::std::string::String::as_str(
                    &::std::string::ToString::to_string(&value),
                ),
                serializer,
            )
        }
    }
    #[derive(::core::fmt::Debug)]
    pub enum DisabledValueParseError {
        NonExistent { value: ::std::string::String },
    }
    impl ::core::fmt::Display for DisabledValueParseError {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::result::Result<(), ::core::fmt::Error> {
            match self {
                DisabledValueParseError::NonExistent { value } => {
                    write!(f, "Value '{:?}' does not exist in the enumeration", value)
                }
            }
        }
    }
    impl ::core::convert::TryFrom<::std::string::String> for DisabledValue {
        type Error = DisabledValueParseError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::core::result::Result<Self, Self::Error> {
            match ::std::string::String::as_str(&value) {
                "disabled" => Ok(DisabledValue::Disabled),
                _ => {
                    Err(DisabledValueParseError::NonExistent {
                        value,
                    })
                }
            }
        }
    }
    impl ::core::convert::From<DisabledValue> for ::std::string::String {
        fn from(value: DisabledValue) -> Self {
            match value {
                DisabledValue::Disabled => ::std::string::String::from("disabled"),
            }
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::core::clone::Clone,
        ::core::marker::Copy,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq
    )]
    #[xvalue(with = readonly_value_with)]
    pub enum ReadonlyValue {
        Readonly,
    }
    pub mod readonly_value_with {
        pub fn deserialize<'de, D>(
            deserializer: D,
        ) -> ::core::result::Result<super::ReadonlyValue, D::Error>
        where
            D: ::xmlity::Deserializer<'de>,
        {
            let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                deserializer,
            )?;
            let value: ::std::string::String = text
                .parse()
                .map_err(::xmlity::de::Error::custom)?;
            super::ReadonlyValue::try_from(value).map_err(::xmlity::de::Error::custom)
        }
        pub fn serialize<S>(
            value: &super::ReadonlyValue,
            serializer: S,
        ) -> ::core::result::Result<S::Ok, S::Error>
        where
            S: ::xmlity::Serializer,
        {
            let value: ::std::string::String = ::core::clone::Clone::clone(value).into();
            ::xmlity::Serialize::serialize(
                ::std::string::String::as_str(
                    &::std::string::ToString::to_string(&value),
                ),
                serializer,
            )
        }
    }
    #[derive(::core::fmt::Debug)]
    pub enum ReadonlyValueParseError {
        NonExistent { value: ::std::string::String },
    }
    impl ::core::fmt::Display for ReadonlyValueParseError {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::result::Result<(), ::core::fmt::Error> {
            match self {
                ReadonlyValueParseError::NonExistent { value } => {
                    write!(f, "Value '{:?}' does not exist in the enumeration", value)
                }
            }
        }
    }
    impl ::core::convert::TryFrom<::std::string::String> for ReadonlyValue {
        type Error = ReadonlyValueParseError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::core::result::Result<Self, Self::Error> {
            match ::std::string::String::as_str(&value) {
                "readonly" => Ok(ReadonlyValue::Readonly),
                _ => {
                    Err(ReadonlyValueParseError::NonExistent {
                        value,
                    })
                }
            }
        }
    }
    impl ::core::convert::From<ReadonlyValue> for ::std::string::String {
        fn from(value: ReadonlyValue) -> Self {
            match value {
                ReadonlyValue::Readonly => ::std::string::String::from("readonly"),
            }
        }
    }
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
#[xelement(
    name = "textarea",
    namespace = "http://www.w3.org/1999/xhtml",
    allow_unknown_attributes = "any"
)]
pub struct Textarea {
    #[xattribute(name = "id", optional, default)]
    pub id: ::core::option::Option<String>,
    #[xattribute(name = "class", optional, default)]
    pub class: ::core::option::Option<String>,
    #[xattribute(name = "style", optional, default)]
    pub style: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::StyleSheet>,
    >,
    #[xattribute(name = "title", optional, default)]
    pub title: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Text>>,
    #[xattribute(name = "lang", optional, default)]
    pub lang: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::LanguageCode>,
    >,
    #[xattribute(deferred = true, optional, default)]
    pub attribute_5: ::core::option::Option<crate::xml::Lang>,
    #[xattribute(name = "dir", optional, default)]
    pub dir: ::core::option::Option<textarea_items::DirValue>,
    #[xattribute(name = "onclick", optional, default)]
    pub onclick: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xattribute(name = "ondblclick", optional, default)]
    pub ondblclick: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmousedown", optional, default)]
    pub onmousedown: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseup", optional, default)]
    pub onmouseup: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseover", optional, default)]
    pub onmouseover: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmousemove", optional, default)]
    pub onmousemove: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseout", optional, default)]
    pub onmouseout: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeypress", optional, default)]
    pub onkeypress: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeydown", optional, default)]
    pub onkeydown: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeyup", optional, default)]
    pub onkeyup: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xattribute(name = "accesskey", optional, default)]
    pub accesskey: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Character>,
    >,
    #[xattribute(name = "tabindex", optional, default)]
    pub tabindex: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::TabindexNumber>,
    >,
    #[xattribute(name = "onfocus", optional, default)]
    pub onfocus: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xattribute(name = "onblur", optional, default)]
    pub onblur: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xattribute(name = "name", optional, default)]
    pub name: ::core::option::Option<String>,
    #[xattribute(name = "rows")]
    pub rows: ::std::boxed::Box<crate::xhtml::types::Number>,
    #[xattribute(name = "cols")]
    pub cols: ::std::boxed::Box<crate::xhtml::types::Number>,
    #[xattribute(name = "disabled", optional, default)]
    pub disabled: ::core::option::Option<textarea_items::DisabledValue>,
    #[xattribute(name = "readonly", optional, default)]
    pub readonly: ::core::option::Option<textarea_items::ReadonlyValue>,
    #[xattribute(name = "onselect", optional, default)]
    pub onselect: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xattribute(name = "onchange", optional, default)]
    pub onchange: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
}
pub mod tfoot_items {
    #[derive(
        ::core::fmt::Debug,
        ::core::clone::Clone,
        ::core::marker::Copy,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq
    )]
    #[xvalue(with = dir_value_with)]
    pub enum DirValue {
        Ltr,
        Rtl,
    }
    pub mod dir_value_with {
        pub fn deserialize<'de, D>(
            deserializer: D,
        ) -> ::core::result::Result<super::DirValue, D::Error>
        where
            D: ::xmlity::Deserializer<'de>,
        {
            let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                deserializer,
            )?;
            let value: ::std::string::String = text
                .parse()
                .map_err(::xmlity::de::Error::custom)?;
            super::DirValue::try_from(value).map_err(::xmlity::de::Error::custom)
        }
        pub fn serialize<S>(
            value: &super::DirValue,
            serializer: S,
        ) -> ::core::result::Result<S::Ok, S::Error>
        where
            S: ::xmlity::Serializer,
        {
            let value: ::std::string::String = ::core::clone::Clone::clone(value).into();
            ::xmlity::Serialize::serialize(
                ::std::string::String::as_str(
                    &::std::string::ToString::to_string(&value),
                ),
                serializer,
            )
        }
    }
    #[derive(::core::fmt::Debug)]
    pub enum DirValueParseError {
        NonExistent { value: ::std::string::String },
    }
    impl ::core::fmt::Display for DirValueParseError {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::result::Result<(), ::core::fmt::Error> {
            match self {
                DirValueParseError::NonExistent { value } => {
                    write!(f, "Value '{:?}' does not exist in the enumeration", value)
                }
            }
        }
    }
    impl ::core::convert::TryFrom<::std::string::String> for DirValue {
        type Error = DirValueParseError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::core::result::Result<Self, Self::Error> {
            match ::std::string::String::as_str(&value) {
                "ltr" => Ok(DirValue::Ltr),
                "rtl" => Ok(DirValue::Rtl),
                _ => {
                    Err(DirValueParseError::NonExistent {
                        value,
                    })
                }
            }
        }
    }
    impl ::core::convert::From<DirValue> for ::std::string::String {
        fn from(value: DirValue) -> Self {
            match value {
                DirValue::Ltr => ::std::string::String::from("ltr"),
                DirValue::Rtl => ::std::string::String::from("rtl"),
            }
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::core::clone::Clone,
        ::core::marker::Copy,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq
    )]
    #[xvalue(with = align_value_with)]
    pub enum AlignValue {
        Left,
        Center,
        Right,
        Justify,
        Char,
    }
    pub mod align_value_with {
        pub fn deserialize<'de, D>(
            deserializer: D,
        ) -> ::core::result::Result<super::AlignValue, D::Error>
        where
            D: ::xmlity::Deserializer<'de>,
        {
            let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                deserializer,
            )?;
            let value: ::std::string::String = text
                .parse()
                .map_err(::xmlity::de::Error::custom)?;
            super::AlignValue::try_from(value).map_err(::xmlity::de::Error::custom)
        }
        pub fn serialize<S>(
            value: &super::AlignValue,
            serializer: S,
        ) -> ::core::result::Result<S::Ok, S::Error>
        where
            S: ::xmlity::Serializer,
        {
            let value: ::std::string::String = ::core::clone::Clone::clone(value).into();
            ::xmlity::Serialize::serialize(
                ::std::string::String::as_str(
                    &::std::string::ToString::to_string(&value),
                ),
                serializer,
            )
        }
    }
    #[derive(::core::fmt::Debug)]
    pub enum AlignValueParseError {
        NonExistent { value: ::std::string::String },
    }
    impl ::core::fmt::Display for AlignValueParseError {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::result::Result<(), ::core::fmt::Error> {
            match self {
                AlignValueParseError::NonExistent { value } => {
                    write!(f, "Value '{:?}' does not exist in the enumeration", value)
                }
            }
        }
    }
    impl ::core::convert::TryFrom<::std::string::String> for AlignValue {
        type Error = AlignValueParseError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::core::result::Result<Self, Self::Error> {
            match ::std::string::String::as_str(&value) {
                "left" => Ok(AlignValue::Left),
                "center" => Ok(AlignValue::Center),
                "right" => Ok(AlignValue::Right),
                "justify" => Ok(AlignValue::Justify),
                "char" => Ok(AlignValue::Char),
                _ => {
                    Err(AlignValueParseError::NonExistent {
                        value,
                    })
                }
            }
        }
    }
    impl ::core::convert::From<AlignValue> for ::std::string::String {
        fn from(value: AlignValue) -> Self {
            match value {
                AlignValue::Left => ::std::string::String::from("left"),
                AlignValue::Center => ::std::string::String::from("center"),
                AlignValue::Right => ::std::string::String::from("right"),
                AlignValue::Justify => ::std::string::String::from("justify"),
                AlignValue::Char => ::std::string::String::from("char"),
            }
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::core::clone::Clone,
        ::core::marker::Copy,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq
    )]
    #[xvalue(with = valign_value_with)]
    pub enum ValignValue {
        Top,
        Middle,
        Bottom,
        Baseline,
    }
    pub mod valign_value_with {
        pub fn deserialize<'de, D>(
            deserializer: D,
        ) -> ::core::result::Result<super::ValignValue, D::Error>
        where
            D: ::xmlity::Deserializer<'de>,
        {
            let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                deserializer,
            )?;
            let value: ::std::string::String = text
                .parse()
                .map_err(::xmlity::de::Error::custom)?;
            super::ValignValue::try_from(value).map_err(::xmlity::de::Error::custom)
        }
        pub fn serialize<S>(
            value: &super::ValignValue,
            serializer: S,
        ) -> ::core::result::Result<S::Ok, S::Error>
        where
            S: ::xmlity::Serializer,
        {
            let value: ::std::string::String = ::core::clone::Clone::clone(value).into();
            ::xmlity::Serialize::serialize(
                ::std::string::String::as_str(
                    &::std::string::ToString::to_string(&value),
                ),
                serializer,
            )
        }
    }
    #[derive(::core::fmt::Debug)]
    pub enum ValignValueParseError {
        NonExistent { value: ::std::string::String },
    }
    impl ::core::fmt::Display for ValignValueParseError {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::result::Result<(), ::core::fmt::Error> {
            match self {
                ValignValueParseError::NonExistent { value } => {
                    write!(f, "Value '{:?}' does not exist in the enumeration", value)
                }
            }
        }
    }
    impl ::core::convert::TryFrom<::std::string::String> for ValignValue {
        type Error = ValignValueParseError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::core::result::Result<Self, Self::Error> {
            match ::std::string::String::as_str(&value) {
                "top" => Ok(ValignValue::Top),
                "middle" => Ok(ValignValue::Middle),
                "bottom" => Ok(ValignValue::Bottom),
                "baseline" => Ok(ValignValue::Baseline),
                _ => {
                    Err(ValignValueParseError::NonExistent {
                        value,
                    })
                }
            }
        }
    }
    impl ::core::convert::From<ValignValue> for ::std::string::String {
        fn from(value: ValignValue) -> Self {
            match value {
                ValignValue::Top => ::std::string::String::from("top"),
                ValignValue::Middle => ::std::string::String::from("middle"),
                ValignValue::Bottom => ::std::string::String::from("bottom"),
                ValignValue::Baseline => ::std::string::String::from("baseline"),
            }
        }
    }
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
#[xelement(
    name = "tfoot",
    namespace = "http://www.w3.org/1999/xhtml",
    allow_unknown_attributes = "any",
    children_order = "strict"
)]
pub struct Tfoot {
    #[xattribute(name = "id", optional, default)]
    pub id: ::core::option::Option<String>,
    #[xattribute(name = "class", optional, default)]
    pub class: ::core::option::Option<String>,
    #[xattribute(name = "style", optional, default)]
    pub style: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::StyleSheet>,
    >,
    #[xattribute(name = "title", optional, default)]
    pub title: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Text>>,
    #[xattribute(name = "lang", optional, default)]
    pub lang: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::LanguageCode>,
    >,
    #[xattribute(deferred = true, optional, default)]
    pub attribute_5: ::core::option::Option<crate::xml::Lang>,
    #[xattribute(name = "dir", optional, default)]
    pub dir: ::core::option::Option<tfoot_items::DirValue>,
    #[xattribute(name = "onclick", optional, default)]
    pub onclick: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xattribute(name = "ondblclick", optional, default)]
    pub ondblclick: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmousedown", optional, default)]
    pub onmousedown: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseup", optional, default)]
    pub onmouseup: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseover", optional, default)]
    pub onmouseover: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmousemove", optional, default)]
    pub onmousemove: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseout", optional, default)]
    pub onmouseout: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeypress", optional, default)]
    pub onkeypress: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeydown", optional, default)]
    pub onkeydown: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeyup", optional, default)]
    pub onkeyup: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xattribute(name = "align", optional, default)]
    pub align: ::core::option::Option<tfoot_items::AlignValue>,
    #[xattribute(name = "char", optional, default)]
    pub char: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Character>>,
    #[xattribute(name = "charoff", optional, default)]
    pub charoff: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Length>>,
    #[xattribute(name = "valign", optional, default)]
    pub valign: ::core::option::Option<tfoot_items::ValignValue>,
    #[xvalue(default)]
    pub tr: ::std::vec::Vec<crate::xhtml::Tr>,
}
pub mod th_items {
    impl ::core::convert::From<crate::xhtml::groups::Block> for Th {
        fn from(value: crate::xhtml::groups::Block) -> Self {
            Th::Block(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml::Form> for Th {
        fn from(value: crate::xhtml::Form) -> Self {
            Th::Form(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml::groups::Inline> for Th {
        fn from(value: crate::xhtml::groups::Inline) -> Self {
            Th::Inline(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml::groups::Misc> for Th {
        fn from(value: crate::xhtml::groups::Misc) -> Self {
            Th::Misc(::std::boxed::Box::new(value))
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub enum Th {
        Block(::std::boxed::Box<crate::xhtml::groups::Block>),
        Form(::std::boxed::Box<crate::xhtml::Form>),
        Inline(::std::boxed::Box<crate::xhtml::groups::Inline>),
        Misc(::std::boxed::Box<crate::xhtml::groups::Misc>),
    }
    #[derive(
        ::core::fmt::Debug,
        ::core::clone::Clone,
        ::core::marker::Copy,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq
    )]
    #[xvalue(with = dir_value_with)]
    pub enum DirValue {
        Ltr,
        Rtl,
    }
    pub mod dir_value_with {
        pub fn deserialize<'de, D>(
            deserializer: D,
        ) -> ::core::result::Result<super::DirValue, D::Error>
        where
            D: ::xmlity::Deserializer<'de>,
        {
            let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                deserializer,
            )?;
            let value: ::std::string::String = text
                .parse()
                .map_err(::xmlity::de::Error::custom)?;
            super::DirValue::try_from(value).map_err(::xmlity::de::Error::custom)
        }
        pub fn serialize<S>(
            value: &super::DirValue,
            serializer: S,
        ) -> ::core::result::Result<S::Ok, S::Error>
        where
            S: ::xmlity::Serializer,
        {
            let value: ::std::string::String = ::core::clone::Clone::clone(value).into();
            ::xmlity::Serialize::serialize(
                ::std::string::String::as_str(
                    &::std::string::ToString::to_string(&value),
                ),
                serializer,
            )
        }
    }
    #[derive(::core::fmt::Debug)]
    pub enum DirValueParseError {
        NonExistent { value: ::std::string::String },
    }
    impl ::core::fmt::Display for DirValueParseError {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::result::Result<(), ::core::fmt::Error> {
            match self {
                DirValueParseError::NonExistent { value } => {
                    write!(f, "Value '{:?}' does not exist in the enumeration", value)
                }
            }
        }
    }
    impl ::core::convert::TryFrom<::std::string::String> for DirValue {
        type Error = DirValueParseError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::core::result::Result<Self, Self::Error> {
            match ::std::string::String::as_str(&value) {
                "ltr" => Ok(DirValue::Ltr),
                "rtl" => Ok(DirValue::Rtl),
                _ => {
                    Err(DirValueParseError::NonExistent {
                        value,
                    })
                }
            }
        }
    }
    impl ::core::convert::From<DirValue> for ::std::string::String {
        fn from(value: DirValue) -> Self {
            match value {
                DirValue::Ltr => ::std::string::String::from("ltr"),
                DirValue::Rtl => ::std::string::String::from("rtl"),
            }
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::core::clone::Clone,
        ::core::marker::Copy,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq
    )]
    #[xvalue(with = align_value_with)]
    pub enum AlignValue {
        Left,
        Center,
        Right,
        Justify,
        Char,
    }
    pub mod align_value_with {
        pub fn deserialize<'de, D>(
            deserializer: D,
        ) -> ::core::result::Result<super::AlignValue, D::Error>
        where
            D: ::xmlity::Deserializer<'de>,
        {
            let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                deserializer,
            )?;
            let value: ::std::string::String = text
                .parse()
                .map_err(::xmlity::de::Error::custom)?;
            super::AlignValue::try_from(value).map_err(::xmlity::de::Error::custom)
        }
        pub fn serialize<S>(
            value: &super::AlignValue,
            serializer: S,
        ) -> ::core::result::Result<S::Ok, S::Error>
        where
            S: ::xmlity::Serializer,
        {
            let value: ::std::string::String = ::core::clone::Clone::clone(value).into();
            ::xmlity::Serialize::serialize(
                ::std::string::String::as_str(
                    &::std::string::ToString::to_string(&value),
                ),
                serializer,
            )
        }
    }
    #[derive(::core::fmt::Debug)]
    pub enum AlignValueParseError {
        NonExistent { value: ::std::string::String },
    }
    impl ::core::fmt::Display for AlignValueParseError {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::result::Result<(), ::core::fmt::Error> {
            match self {
                AlignValueParseError::NonExistent { value } => {
                    write!(f, "Value '{:?}' does not exist in the enumeration", value)
                }
            }
        }
    }
    impl ::core::convert::TryFrom<::std::string::String> for AlignValue {
        type Error = AlignValueParseError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::core::result::Result<Self, Self::Error> {
            match ::std::string::String::as_str(&value) {
                "left" => Ok(AlignValue::Left),
                "center" => Ok(AlignValue::Center),
                "right" => Ok(AlignValue::Right),
                "justify" => Ok(AlignValue::Justify),
                "char" => Ok(AlignValue::Char),
                _ => {
                    Err(AlignValueParseError::NonExistent {
                        value,
                    })
                }
            }
        }
    }
    impl ::core::convert::From<AlignValue> for ::std::string::String {
        fn from(value: AlignValue) -> Self {
            match value {
                AlignValue::Left => ::std::string::String::from("left"),
                AlignValue::Center => ::std::string::String::from("center"),
                AlignValue::Right => ::std::string::String::from("right"),
                AlignValue::Justify => ::std::string::String::from("justify"),
                AlignValue::Char => ::std::string::String::from("char"),
            }
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::core::clone::Clone,
        ::core::marker::Copy,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq
    )]
    #[xvalue(with = valign_value_with)]
    pub enum ValignValue {
        Top,
        Middle,
        Bottom,
        Baseline,
    }
    pub mod valign_value_with {
        pub fn deserialize<'de, D>(
            deserializer: D,
        ) -> ::core::result::Result<super::ValignValue, D::Error>
        where
            D: ::xmlity::Deserializer<'de>,
        {
            let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                deserializer,
            )?;
            let value: ::std::string::String = text
                .parse()
                .map_err(::xmlity::de::Error::custom)?;
            super::ValignValue::try_from(value).map_err(::xmlity::de::Error::custom)
        }
        pub fn serialize<S>(
            value: &super::ValignValue,
            serializer: S,
        ) -> ::core::result::Result<S::Ok, S::Error>
        where
            S: ::xmlity::Serializer,
        {
            let value: ::std::string::String = ::core::clone::Clone::clone(value).into();
            ::xmlity::Serialize::serialize(
                ::std::string::String::as_str(
                    &::std::string::ToString::to_string(&value),
                ),
                serializer,
            )
        }
    }
    #[derive(::core::fmt::Debug)]
    pub enum ValignValueParseError {
        NonExistent { value: ::std::string::String },
    }
    impl ::core::fmt::Display for ValignValueParseError {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::result::Result<(), ::core::fmt::Error> {
            match self {
                ValignValueParseError::NonExistent { value } => {
                    write!(f, "Value '{:?}' does not exist in the enumeration", value)
                }
            }
        }
    }
    impl ::core::convert::TryFrom<::std::string::String> for ValignValue {
        type Error = ValignValueParseError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::core::result::Result<Self, Self::Error> {
            match ::std::string::String::as_str(&value) {
                "top" => Ok(ValignValue::Top),
                "middle" => Ok(ValignValue::Middle),
                "bottom" => Ok(ValignValue::Bottom),
                "baseline" => Ok(ValignValue::Baseline),
                _ => {
                    Err(ValignValueParseError::NonExistent {
                        value,
                    })
                }
            }
        }
    }
    impl ::core::convert::From<ValignValue> for ::std::string::String {
        fn from(value: ValignValue) -> Self {
            match value {
                ValignValue::Top => ::std::string::String::from("top"),
                ValignValue::Middle => ::std::string::String::from("middle"),
                ValignValue::Bottom => ::std::string::String::from("bottom"),
                ValignValue::Baseline => ::std::string::String::from("baseline"),
            }
        }
    }
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
#[xelement(
    name = "th",
    namespace = "http://www.w3.org/1999/xhtml",
    allow_unknown_attributes = "any"
)]
pub struct Th {
    #[xattribute(name = "id", optional, default)]
    pub id: ::core::option::Option<String>,
    #[xattribute(name = "class", optional, default)]
    pub class: ::core::option::Option<String>,
    #[xattribute(name = "style", optional, default)]
    pub style: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::StyleSheet>,
    >,
    #[xattribute(name = "title", optional, default)]
    pub title: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Text>>,
    #[xattribute(name = "lang", optional, default)]
    pub lang: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::LanguageCode>,
    >,
    #[xattribute(deferred = true, optional, default)]
    pub attribute_5: ::core::option::Option<crate::xml::Lang>,
    #[xattribute(name = "dir", optional, default)]
    pub dir: ::core::option::Option<th_items::DirValue>,
    #[xattribute(name = "onclick", optional, default)]
    pub onclick: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xattribute(name = "ondblclick", optional, default)]
    pub ondblclick: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmousedown", optional, default)]
    pub onmousedown: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseup", optional, default)]
    pub onmouseup: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseover", optional, default)]
    pub onmouseover: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmousemove", optional, default)]
    pub onmousemove: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseout", optional, default)]
    pub onmouseout: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeypress", optional, default)]
    pub onkeypress: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeydown", optional, default)]
    pub onkeydown: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeyup", optional, default)]
    pub onkeyup: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xattribute(name = "abbr", optional, default)]
    pub abbr: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Text>>,
    #[xattribute(name = "axis", optional, default)]
    pub axis: ::core::option::Option<String>,
    #[xattribute(name = "headers", optional, default)]
    pub headers: ::core::option::Option<String>,
    #[xattribute(name = "scope", optional, default)]
    pub scope: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Scope>>,
    #[xattribute(name = "rowspan", optional, default)]
    pub rowspan: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Number>>,
    #[xattribute(name = "colspan", optional, default)]
    pub colspan: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Number>>,
    #[xattribute(name = "align", optional, default)]
    pub align: ::core::option::Option<th_items::AlignValue>,
    #[xattribute(name = "char", optional, default)]
    pub char: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Character>>,
    #[xattribute(name = "charoff", optional, default)]
    pub charoff: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Length>>,
    #[xattribute(name = "valign", optional, default)]
    pub valign: ::core::option::Option<th_items::ValignValue>,
    #[xvalue(default)]
    pub th: ::std::vec::Vec<th_items::Th>,
}
pub mod thead_items {
    #[derive(
        ::core::fmt::Debug,
        ::core::clone::Clone,
        ::core::marker::Copy,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq
    )]
    #[xvalue(with = dir_value_with)]
    pub enum DirValue {
        Ltr,
        Rtl,
    }
    pub mod dir_value_with {
        pub fn deserialize<'de, D>(
            deserializer: D,
        ) -> ::core::result::Result<super::DirValue, D::Error>
        where
            D: ::xmlity::Deserializer<'de>,
        {
            let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                deserializer,
            )?;
            let value: ::std::string::String = text
                .parse()
                .map_err(::xmlity::de::Error::custom)?;
            super::DirValue::try_from(value).map_err(::xmlity::de::Error::custom)
        }
        pub fn serialize<S>(
            value: &super::DirValue,
            serializer: S,
        ) -> ::core::result::Result<S::Ok, S::Error>
        where
            S: ::xmlity::Serializer,
        {
            let value: ::std::string::String = ::core::clone::Clone::clone(value).into();
            ::xmlity::Serialize::serialize(
                ::std::string::String::as_str(
                    &::std::string::ToString::to_string(&value),
                ),
                serializer,
            )
        }
    }
    #[derive(::core::fmt::Debug)]
    pub enum DirValueParseError {
        NonExistent { value: ::std::string::String },
    }
    impl ::core::fmt::Display for DirValueParseError {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::result::Result<(), ::core::fmt::Error> {
            match self {
                DirValueParseError::NonExistent { value } => {
                    write!(f, "Value '{:?}' does not exist in the enumeration", value)
                }
            }
        }
    }
    impl ::core::convert::TryFrom<::std::string::String> for DirValue {
        type Error = DirValueParseError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::core::result::Result<Self, Self::Error> {
            match ::std::string::String::as_str(&value) {
                "ltr" => Ok(DirValue::Ltr),
                "rtl" => Ok(DirValue::Rtl),
                _ => {
                    Err(DirValueParseError::NonExistent {
                        value,
                    })
                }
            }
        }
    }
    impl ::core::convert::From<DirValue> for ::std::string::String {
        fn from(value: DirValue) -> Self {
            match value {
                DirValue::Ltr => ::std::string::String::from("ltr"),
                DirValue::Rtl => ::std::string::String::from("rtl"),
            }
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::core::clone::Clone,
        ::core::marker::Copy,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq
    )]
    #[xvalue(with = align_value_with)]
    pub enum AlignValue {
        Left,
        Center,
        Right,
        Justify,
        Char,
    }
    pub mod align_value_with {
        pub fn deserialize<'de, D>(
            deserializer: D,
        ) -> ::core::result::Result<super::AlignValue, D::Error>
        where
            D: ::xmlity::Deserializer<'de>,
        {
            let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                deserializer,
            )?;
            let value: ::std::string::String = text
                .parse()
                .map_err(::xmlity::de::Error::custom)?;
            super::AlignValue::try_from(value).map_err(::xmlity::de::Error::custom)
        }
        pub fn serialize<S>(
            value: &super::AlignValue,
            serializer: S,
        ) -> ::core::result::Result<S::Ok, S::Error>
        where
            S: ::xmlity::Serializer,
        {
            let value: ::std::string::String = ::core::clone::Clone::clone(value).into();
            ::xmlity::Serialize::serialize(
                ::std::string::String::as_str(
                    &::std::string::ToString::to_string(&value),
                ),
                serializer,
            )
        }
    }
    #[derive(::core::fmt::Debug)]
    pub enum AlignValueParseError {
        NonExistent { value: ::std::string::String },
    }
    impl ::core::fmt::Display for AlignValueParseError {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::result::Result<(), ::core::fmt::Error> {
            match self {
                AlignValueParseError::NonExistent { value } => {
                    write!(f, "Value '{:?}' does not exist in the enumeration", value)
                }
            }
        }
    }
    impl ::core::convert::TryFrom<::std::string::String> for AlignValue {
        type Error = AlignValueParseError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::core::result::Result<Self, Self::Error> {
            match ::std::string::String::as_str(&value) {
                "left" => Ok(AlignValue::Left),
                "center" => Ok(AlignValue::Center),
                "right" => Ok(AlignValue::Right),
                "justify" => Ok(AlignValue::Justify),
                "char" => Ok(AlignValue::Char),
                _ => {
                    Err(AlignValueParseError::NonExistent {
                        value,
                    })
                }
            }
        }
    }
    impl ::core::convert::From<AlignValue> for ::std::string::String {
        fn from(value: AlignValue) -> Self {
            match value {
                AlignValue::Left => ::std::string::String::from("left"),
                AlignValue::Center => ::std::string::String::from("center"),
                AlignValue::Right => ::std::string::String::from("right"),
                AlignValue::Justify => ::std::string::String::from("justify"),
                AlignValue::Char => ::std::string::String::from("char"),
            }
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::core::clone::Clone,
        ::core::marker::Copy,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq
    )]
    #[xvalue(with = valign_value_with)]
    pub enum ValignValue {
        Top,
        Middle,
        Bottom,
        Baseline,
    }
    pub mod valign_value_with {
        pub fn deserialize<'de, D>(
            deserializer: D,
        ) -> ::core::result::Result<super::ValignValue, D::Error>
        where
            D: ::xmlity::Deserializer<'de>,
        {
            let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                deserializer,
            )?;
            let value: ::std::string::String = text
                .parse()
                .map_err(::xmlity::de::Error::custom)?;
            super::ValignValue::try_from(value).map_err(::xmlity::de::Error::custom)
        }
        pub fn serialize<S>(
            value: &super::ValignValue,
            serializer: S,
        ) -> ::core::result::Result<S::Ok, S::Error>
        where
            S: ::xmlity::Serializer,
        {
            let value: ::std::string::String = ::core::clone::Clone::clone(value).into();
            ::xmlity::Serialize::serialize(
                ::std::string::String::as_str(
                    &::std::string::ToString::to_string(&value),
                ),
                serializer,
            )
        }
    }
    #[derive(::core::fmt::Debug)]
    pub enum ValignValueParseError {
        NonExistent { value: ::std::string::String },
    }
    impl ::core::fmt::Display for ValignValueParseError {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::result::Result<(), ::core::fmt::Error> {
            match self {
                ValignValueParseError::NonExistent { value } => {
                    write!(f, "Value '{:?}' does not exist in the enumeration", value)
                }
            }
        }
    }
    impl ::core::convert::TryFrom<::std::string::String> for ValignValue {
        type Error = ValignValueParseError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::core::result::Result<Self, Self::Error> {
            match ::std::string::String::as_str(&value) {
                "top" => Ok(ValignValue::Top),
                "middle" => Ok(ValignValue::Middle),
                "bottom" => Ok(ValignValue::Bottom),
                "baseline" => Ok(ValignValue::Baseline),
                _ => {
                    Err(ValignValueParseError::NonExistent {
                        value,
                    })
                }
            }
        }
    }
    impl ::core::convert::From<ValignValue> for ::std::string::String {
        fn from(value: ValignValue) -> Self {
            match value {
                ValignValue::Top => ::std::string::String::from("top"),
                ValignValue::Middle => ::std::string::String::from("middle"),
                ValignValue::Bottom => ::std::string::String::from("bottom"),
                ValignValue::Baseline => ::std::string::String::from("baseline"),
            }
        }
    }
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
#[xelement(
    name = "thead",
    namespace = "http://www.w3.org/1999/xhtml",
    allow_unknown_attributes = "any",
    children_order = "strict"
)]
pub struct Thead {
    #[xattribute(name = "id", optional, default)]
    pub id: ::core::option::Option<String>,
    #[xattribute(name = "class", optional, default)]
    pub class: ::core::option::Option<String>,
    #[xattribute(name = "style", optional, default)]
    pub style: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::StyleSheet>,
    >,
    #[xattribute(name = "title", optional, default)]
    pub title: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Text>>,
    #[xattribute(name = "lang", optional, default)]
    pub lang: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::LanguageCode>,
    >,
    #[xattribute(deferred = true, optional, default)]
    pub attribute_5: ::core::option::Option<crate::xml::Lang>,
    #[xattribute(name = "dir", optional, default)]
    pub dir: ::core::option::Option<thead_items::DirValue>,
    #[xattribute(name = "onclick", optional, default)]
    pub onclick: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xattribute(name = "ondblclick", optional, default)]
    pub ondblclick: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmousedown", optional, default)]
    pub onmousedown: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseup", optional, default)]
    pub onmouseup: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseover", optional, default)]
    pub onmouseover: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmousemove", optional, default)]
    pub onmousemove: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseout", optional, default)]
    pub onmouseout: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeypress", optional, default)]
    pub onkeypress: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeydown", optional, default)]
    pub onkeydown: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeyup", optional, default)]
    pub onkeyup: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xattribute(name = "align", optional, default)]
    pub align: ::core::option::Option<thead_items::AlignValue>,
    #[xattribute(name = "char", optional, default)]
    pub char: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Character>>,
    #[xattribute(name = "charoff", optional, default)]
    pub charoff: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Length>>,
    #[xattribute(name = "valign", optional, default)]
    pub valign: ::core::option::Option<thead_items::ValignValue>,
    #[xvalue(default)]
    pub tr: ::std::vec::Vec<crate::xhtml::Tr>,
}
pub mod title_items {
    #[derive(
        ::core::fmt::Debug,
        ::core::clone::Clone,
        ::core::marker::Copy,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq
    )]
    #[xvalue(with = dir_value_with)]
    pub enum DirValue {
        Ltr,
        Rtl,
    }
    pub mod dir_value_with {
        pub fn deserialize<'de, D>(
            deserializer: D,
        ) -> ::core::result::Result<super::DirValue, D::Error>
        where
            D: ::xmlity::Deserializer<'de>,
        {
            let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                deserializer,
            )?;
            let value: ::std::string::String = text
                .parse()
                .map_err(::xmlity::de::Error::custom)?;
            super::DirValue::try_from(value).map_err(::xmlity::de::Error::custom)
        }
        pub fn serialize<S>(
            value: &super::DirValue,
            serializer: S,
        ) -> ::core::result::Result<S::Ok, S::Error>
        where
            S: ::xmlity::Serializer,
        {
            let value: ::std::string::String = ::core::clone::Clone::clone(value).into();
            ::xmlity::Serialize::serialize(
                ::std::string::String::as_str(
                    &::std::string::ToString::to_string(&value),
                ),
                serializer,
            )
        }
    }
    #[derive(::core::fmt::Debug)]
    pub enum DirValueParseError {
        NonExistent { value: ::std::string::String },
    }
    impl ::core::fmt::Display for DirValueParseError {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::result::Result<(), ::core::fmt::Error> {
            match self {
                DirValueParseError::NonExistent { value } => {
                    write!(f, "Value '{:?}' does not exist in the enumeration", value)
                }
            }
        }
    }
    impl ::core::convert::TryFrom<::std::string::String> for DirValue {
        type Error = DirValueParseError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::core::result::Result<Self, Self::Error> {
            match ::std::string::String::as_str(&value) {
                "ltr" => Ok(DirValue::Ltr),
                "rtl" => Ok(DirValue::Rtl),
                _ => {
                    Err(DirValueParseError::NonExistent {
                        value,
                    })
                }
            }
        }
    }
    impl ::core::convert::From<DirValue> for ::std::string::String {
        fn from(value: DirValue) -> Self {
            match value {
                DirValue::Ltr => ::std::string::String::from("ltr"),
                DirValue::Rtl => ::std::string::String::from("rtl"),
            }
        }
    }
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
#[xelement(
    name = "title",
    namespace = "http://www.w3.org/1999/xhtml",
    allow_unknown_attributes = "any"
)]
pub struct Title {
    #[xattribute(name = "lang", optional, default)]
    pub lang: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::LanguageCode>,
    >,
    #[xattribute(deferred = true, optional, default)]
    pub attribute_1: ::core::option::Option<crate::xml::Lang>,
    #[xattribute(name = "dir", optional, default)]
    pub dir: ::core::option::Option<title_items::DirValue>,
    #[xattribute(name = "id", optional, default)]
    pub id: ::core::option::Option<String>,
}
pub mod tr_items {
    impl ::core::convert::From<crate::xhtml::Th> for Tr {
        fn from(value: crate::xhtml::Th) -> Self {
            Tr::Th(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml::Td> for Tr {
        fn from(value: crate::xhtml::Td) -> Self {
            Tr::Td(::std::boxed::Box::new(value))
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub enum Tr {
        Th(::std::boxed::Box<crate::xhtml::Th>),
        Td(::std::boxed::Box<crate::xhtml::Td>),
    }
    #[derive(
        ::core::fmt::Debug,
        ::core::clone::Clone,
        ::core::marker::Copy,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq
    )]
    #[xvalue(with = dir_value_with)]
    pub enum DirValue {
        Ltr,
        Rtl,
    }
    pub mod dir_value_with {
        pub fn deserialize<'de, D>(
            deserializer: D,
        ) -> ::core::result::Result<super::DirValue, D::Error>
        where
            D: ::xmlity::Deserializer<'de>,
        {
            let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                deserializer,
            )?;
            let value: ::std::string::String = text
                .parse()
                .map_err(::xmlity::de::Error::custom)?;
            super::DirValue::try_from(value).map_err(::xmlity::de::Error::custom)
        }
        pub fn serialize<S>(
            value: &super::DirValue,
            serializer: S,
        ) -> ::core::result::Result<S::Ok, S::Error>
        where
            S: ::xmlity::Serializer,
        {
            let value: ::std::string::String = ::core::clone::Clone::clone(value).into();
            ::xmlity::Serialize::serialize(
                ::std::string::String::as_str(
                    &::std::string::ToString::to_string(&value),
                ),
                serializer,
            )
        }
    }
    #[derive(::core::fmt::Debug)]
    pub enum DirValueParseError {
        NonExistent { value: ::std::string::String },
    }
    impl ::core::fmt::Display for DirValueParseError {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::result::Result<(), ::core::fmt::Error> {
            match self {
                DirValueParseError::NonExistent { value } => {
                    write!(f, "Value '{:?}' does not exist in the enumeration", value)
                }
            }
        }
    }
    impl ::core::convert::TryFrom<::std::string::String> for DirValue {
        type Error = DirValueParseError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::core::result::Result<Self, Self::Error> {
            match ::std::string::String::as_str(&value) {
                "ltr" => Ok(DirValue::Ltr),
                "rtl" => Ok(DirValue::Rtl),
                _ => {
                    Err(DirValueParseError::NonExistent {
                        value,
                    })
                }
            }
        }
    }
    impl ::core::convert::From<DirValue> for ::std::string::String {
        fn from(value: DirValue) -> Self {
            match value {
                DirValue::Ltr => ::std::string::String::from("ltr"),
                DirValue::Rtl => ::std::string::String::from("rtl"),
            }
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::core::clone::Clone,
        ::core::marker::Copy,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq
    )]
    #[xvalue(with = align_value_with)]
    pub enum AlignValue {
        Left,
        Center,
        Right,
        Justify,
        Char,
    }
    pub mod align_value_with {
        pub fn deserialize<'de, D>(
            deserializer: D,
        ) -> ::core::result::Result<super::AlignValue, D::Error>
        where
            D: ::xmlity::Deserializer<'de>,
        {
            let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                deserializer,
            )?;
            let value: ::std::string::String = text
                .parse()
                .map_err(::xmlity::de::Error::custom)?;
            super::AlignValue::try_from(value).map_err(::xmlity::de::Error::custom)
        }
        pub fn serialize<S>(
            value: &super::AlignValue,
            serializer: S,
        ) -> ::core::result::Result<S::Ok, S::Error>
        where
            S: ::xmlity::Serializer,
        {
            let value: ::std::string::String = ::core::clone::Clone::clone(value).into();
            ::xmlity::Serialize::serialize(
                ::std::string::String::as_str(
                    &::std::string::ToString::to_string(&value),
                ),
                serializer,
            )
        }
    }
    #[derive(::core::fmt::Debug)]
    pub enum AlignValueParseError {
        NonExistent { value: ::std::string::String },
    }
    impl ::core::fmt::Display for AlignValueParseError {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::result::Result<(), ::core::fmt::Error> {
            match self {
                AlignValueParseError::NonExistent { value } => {
                    write!(f, "Value '{:?}' does not exist in the enumeration", value)
                }
            }
        }
    }
    impl ::core::convert::TryFrom<::std::string::String> for AlignValue {
        type Error = AlignValueParseError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::core::result::Result<Self, Self::Error> {
            match ::std::string::String::as_str(&value) {
                "left" => Ok(AlignValue::Left),
                "center" => Ok(AlignValue::Center),
                "right" => Ok(AlignValue::Right),
                "justify" => Ok(AlignValue::Justify),
                "char" => Ok(AlignValue::Char),
                _ => {
                    Err(AlignValueParseError::NonExistent {
                        value,
                    })
                }
            }
        }
    }
    impl ::core::convert::From<AlignValue> for ::std::string::String {
        fn from(value: AlignValue) -> Self {
            match value {
                AlignValue::Left => ::std::string::String::from("left"),
                AlignValue::Center => ::std::string::String::from("center"),
                AlignValue::Right => ::std::string::String::from("right"),
                AlignValue::Justify => ::std::string::String::from("justify"),
                AlignValue::Char => ::std::string::String::from("char"),
            }
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::core::clone::Clone,
        ::core::marker::Copy,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq
    )]
    #[xvalue(with = valign_value_with)]
    pub enum ValignValue {
        Top,
        Middle,
        Bottom,
        Baseline,
    }
    pub mod valign_value_with {
        pub fn deserialize<'de, D>(
            deserializer: D,
        ) -> ::core::result::Result<super::ValignValue, D::Error>
        where
            D: ::xmlity::Deserializer<'de>,
        {
            let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                deserializer,
            )?;
            let value: ::std::string::String = text
                .parse()
                .map_err(::xmlity::de::Error::custom)?;
            super::ValignValue::try_from(value).map_err(::xmlity::de::Error::custom)
        }
        pub fn serialize<S>(
            value: &super::ValignValue,
            serializer: S,
        ) -> ::core::result::Result<S::Ok, S::Error>
        where
            S: ::xmlity::Serializer,
        {
            let value: ::std::string::String = ::core::clone::Clone::clone(value).into();
            ::xmlity::Serialize::serialize(
                ::std::string::String::as_str(
                    &::std::string::ToString::to_string(&value),
                ),
                serializer,
            )
        }
    }
    #[derive(::core::fmt::Debug)]
    pub enum ValignValueParseError {
        NonExistent { value: ::std::string::String },
    }
    impl ::core::fmt::Display for ValignValueParseError {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::result::Result<(), ::core::fmt::Error> {
            match self {
                ValignValueParseError::NonExistent { value } => {
                    write!(f, "Value '{:?}' does not exist in the enumeration", value)
                }
            }
        }
    }
    impl ::core::convert::TryFrom<::std::string::String> for ValignValue {
        type Error = ValignValueParseError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::core::result::Result<Self, Self::Error> {
            match ::std::string::String::as_str(&value) {
                "top" => Ok(ValignValue::Top),
                "middle" => Ok(ValignValue::Middle),
                "bottom" => Ok(ValignValue::Bottom),
                "baseline" => Ok(ValignValue::Baseline),
                _ => {
                    Err(ValignValueParseError::NonExistent {
                        value,
                    })
                }
            }
        }
    }
    impl ::core::convert::From<ValignValue> for ::std::string::String {
        fn from(value: ValignValue) -> Self {
            match value {
                ValignValue::Top => ::std::string::String::from("top"),
                ValignValue::Middle => ::std::string::String::from("middle"),
                ValignValue::Bottom => ::std::string::String::from("bottom"),
                ValignValue::Baseline => ::std::string::String::from("baseline"),
            }
        }
    }
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
#[xelement(
    name = "tr",
    namespace = "http://www.w3.org/1999/xhtml",
    allow_unknown_attributes = "any"
)]
pub struct Tr {
    #[xattribute(name = "id", optional, default)]
    pub id: ::core::option::Option<String>,
    #[xattribute(name = "class", optional, default)]
    pub class: ::core::option::Option<String>,
    #[xattribute(name = "style", optional, default)]
    pub style: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::StyleSheet>,
    >,
    #[xattribute(name = "title", optional, default)]
    pub title: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Text>>,
    #[xattribute(name = "lang", optional, default)]
    pub lang: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::LanguageCode>,
    >,
    #[xattribute(deferred = true, optional, default)]
    pub attribute_5: ::core::option::Option<crate::xml::Lang>,
    #[xattribute(name = "dir", optional, default)]
    pub dir: ::core::option::Option<tr_items::DirValue>,
    #[xattribute(name = "onclick", optional, default)]
    pub onclick: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xattribute(name = "ondblclick", optional, default)]
    pub ondblclick: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmousedown", optional, default)]
    pub onmousedown: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseup", optional, default)]
    pub onmouseup: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseover", optional, default)]
    pub onmouseover: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmousemove", optional, default)]
    pub onmousemove: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseout", optional, default)]
    pub onmouseout: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeypress", optional, default)]
    pub onkeypress: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeydown", optional, default)]
    pub onkeydown: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeyup", optional, default)]
    pub onkeyup: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xattribute(name = "align", optional, default)]
    pub align: ::core::option::Option<tr_items::AlignValue>,
    #[xattribute(name = "char", optional, default)]
    pub char: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Character>>,
    #[xattribute(name = "charoff", optional, default)]
    pub charoff: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Length>>,
    #[xattribute(name = "valign", optional, default)]
    pub valign: ::core::option::Option<tr_items::ValignValue>,
    #[xvalue(default)]
    pub tr: ::std::vec::Vec<tr_items::Tr>,
}
pub mod tt_items {
    impl ::core::convert::From<crate::xhtml::groups::Inline> for Inline {
        fn from(value: crate::xhtml::groups::Inline) -> Self {
            Inline::Inline(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml::groups::MiscInline> for Inline {
        fn from(value: crate::xhtml::groups::MiscInline) -> Self {
            Inline::MiscInline(::std::boxed::Box::new(value))
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub enum Inline {
        Inline(::std::boxed::Box<crate::xhtml::groups::Inline>),
        MiscInline(::std::boxed::Box<crate::xhtml::groups::MiscInline>),
    }
    #[derive(
        ::core::fmt::Debug,
        ::core::clone::Clone,
        ::core::marker::Copy,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq
    )]
    #[xvalue(with = dir_value_with)]
    pub enum DirValue {
        Ltr,
        Rtl,
    }
    pub mod dir_value_with {
        pub fn deserialize<'de, D>(
            deserializer: D,
        ) -> ::core::result::Result<super::DirValue, D::Error>
        where
            D: ::xmlity::Deserializer<'de>,
        {
            let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                deserializer,
            )?;
            let value: ::std::string::String = text
                .parse()
                .map_err(::xmlity::de::Error::custom)?;
            super::DirValue::try_from(value).map_err(::xmlity::de::Error::custom)
        }
        pub fn serialize<S>(
            value: &super::DirValue,
            serializer: S,
        ) -> ::core::result::Result<S::Ok, S::Error>
        where
            S: ::xmlity::Serializer,
        {
            let value: ::std::string::String = ::core::clone::Clone::clone(value).into();
            ::xmlity::Serialize::serialize(
                ::std::string::String::as_str(
                    &::std::string::ToString::to_string(&value),
                ),
                serializer,
            )
        }
    }
    #[derive(::core::fmt::Debug)]
    pub enum DirValueParseError {
        NonExistent { value: ::std::string::String },
    }
    impl ::core::fmt::Display for DirValueParseError {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::result::Result<(), ::core::fmt::Error> {
            match self {
                DirValueParseError::NonExistent { value } => {
                    write!(f, "Value '{:?}' does not exist in the enumeration", value)
                }
            }
        }
    }
    impl ::core::convert::TryFrom<::std::string::String> for DirValue {
        type Error = DirValueParseError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::core::result::Result<Self, Self::Error> {
            match ::std::string::String::as_str(&value) {
                "ltr" => Ok(DirValue::Ltr),
                "rtl" => Ok(DirValue::Rtl),
                _ => {
                    Err(DirValueParseError::NonExistent {
                        value,
                    })
                }
            }
        }
    }
    impl ::core::convert::From<DirValue> for ::std::string::String {
        fn from(value: DirValue) -> Self {
            match value {
                DirValue::Ltr => ::std::string::String::from("ltr"),
                DirValue::Rtl => ::std::string::String::from("rtl"),
            }
        }
    }
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
#[xelement(
    name = "tt",
    namespace = "http://www.w3.org/1999/xhtml",
    allow_unknown_attributes = "any"
)]
pub struct Tt {
    #[xattribute(name = "id", optional, default)]
    pub id: ::core::option::Option<String>,
    #[xattribute(name = "class", optional, default)]
    pub class: ::core::option::Option<String>,
    #[xattribute(name = "style", optional, default)]
    pub style: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::StyleSheet>,
    >,
    #[xattribute(name = "title", optional, default)]
    pub title: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Text>>,
    #[xattribute(name = "lang", optional, default)]
    pub lang: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::LanguageCode>,
    >,
    #[xattribute(deferred = true, optional, default)]
    pub attribute_5: ::core::option::Option<crate::xml::Lang>,
    #[xattribute(name = "dir", optional, default)]
    pub dir: ::core::option::Option<tt_items::DirValue>,
    #[xattribute(name = "onclick", optional, default)]
    pub onclick: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xattribute(name = "ondblclick", optional, default)]
    pub ondblclick: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmousedown", optional, default)]
    pub onmousedown: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseup", optional, default)]
    pub onmouseup: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseover", optional, default)]
    pub onmouseover: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmousemove", optional, default)]
    pub onmousemove: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseout", optional, default)]
    pub onmouseout: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeypress", optional, default)]
    pub onkeypress: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeydown", optional, default)]
    pub onkeydown: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeyup", optional, default)]
    pub onkeyup: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xvalue(default)]
    pub inline: ::std::vec::Vec<tt_items::Inline>,
}
pub mod ul_items {
    #[derive(
        ::core::fmt::Debug,
        ::core::clone::Clone,
        ::core::marker::Copy,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq
    )]
    #[xvalue(with = dir_value_with)]
    pub enum DirValue {
        Ltr,
        Rtl,
    }
    pub mod dir_value_with {
        pub fn deserialize<'de, D>(
            deserializer: D,
        ) -> ::core::result::Result<super::DirValue, D::Error>
        where
            D: ::xmlity::Deserializer<'de>,
        {
            let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                deserializer,
            )?;
            let value: ::std::string::String = text
                .parse()
                .map_err(::xmlity::de::Error::custom)?;
            super::DirValue::try_from(value).map_err(::xmlity::de::Error::custom)
        }
        pub fn serialize<S>(
            value: &super::DirValue,
            serializer: S,
        ) -> ::core::result::Result<S::Ok, S::Error>
        where
            S: ::xmlity::Serializer,
        {
            let value: ::std::string::String = ::core::clone::Clone::clone(value).into();
            ::xmlity::Serialize::serialize(
                ::std::string::String::as_str(
                    &::std::string::ToString::to_string(&value),
                ),
                serializer,
            )
        }
    }
    #[derive(::core::fmt::Debug)]
    pub enum DirValueParseError {
        NonExistent { value: ::std::string::String },
    }
    impl ::core::fmt::Display for DirValueParseError {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::result::Result<(), ::core::fmt::Error> {
            match self {
                DirValueParseError::NonExistent { value } => {
                    write!(f, "Value '{:?}' does not exist in the enumeration", value)
                }
            }
        }
    }
    impl ::core::convert::TryFrom<::std::string::String> for DirValue {
        type Error = DirValueParseError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::core::result::Result<Self, Self::Error> {
            match ::std::string::String::as_str(&value) {
                "ltr" => Ok(DirValue::Ltr),
                "rtl" => Ok(DirValue::Rtl),
                _ => {
                    Err(DirValueParseError::NonExistent {
                        value,
                    })
                }
            }
        }
    }
    impl ::core::convert::From<DirValue> for ::std::string::String {
        fn from(value: DirValue) -> Self {
            match value {
                DirValue::Ltr => ::std::string::String::from("ltr"),
                DirValue::Rtl => ::std::string::String::from("rtl"),
            }
        }
    }
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
#[xelement(
    name = "ul",
    namespace = "http://www.w3.org/1999/xhtml",
    allow_unknown_attributes = "any",
    children_order = "strict"
)]
pub struct Ul {
    #[xattribute(name = "id", optional, default)]
    pub id: ::core::option::Option<String>,
    #[xattribute(name = "class", optional, default)]
    pub class: ::core::option::Option<String>,
    #[xattribute(name = "style", optional, default)]
    pub style: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::StyleSheet>,
    >,
    #[xattribute(name = "title", optional, default)]
    pub title: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Text>>,
    #[xattribute(name = "lang", optional, default)]
    pub lang: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::LanguageCode>,
    >,
    #[xattribute(deferred = true, optional, default)]
    pub attribute_5: ::core::option::Option<crate::xml::Lang>,
    #[xattribute(name = "dir", optional, default)]
    pub dir: ::core::option::Option<ul_items::DirValue>,
    #[xattribute(name = "onclick", optional, default)]
    pub onclick: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xattribute(name = "ondblclick", optional, default)]
    pub ondblclick: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmousedown", optional, default)]
    pub onmousedown: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseup", optional, default)]
    pub onmouseup: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseover", optional, default)]
    pub onmouseover: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmousemove", optional, default)]
    pub onmousemove: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseout", optional, default)]
    pub onmouseout: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeypress", optional, default)]
    pub onkeypress: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeydown", optional, default)]
    pub onkeydown: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeyup", optional, default)]
    pub onkeyup: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xvalue(default)]
    pub li: ::std::vec::Vec<crate::xhtml::Li>,
}
pub mod var_items {
    impl ::core::convert::From<crate::xhtml::groups::Inline> for Inline {
        fn from(value: crate::xhtml::groups::Inline) -> Self {
            Inline::Inline(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml::groups::MiscInline> for Inline {
        fn from(value: crate::xhtml::groups::MiscInline) -> Self {
            Inline::MiscInline(::std::boxed::Box::new(value))
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub enum Inline {
        Inline(::std::boxed::Box<crate::xhtml::groups::Inline>),
        MiscInline(::std::boxed::Box<crate::xhtml::groups::MiscInline>),
    }
    #[derive(
        ::core::fmt::Debug,
        ::core::clone::Clone,
        ::core::marker::Copy,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq
    )]
    #[xvalue(with = dir_value_with)]
    pub enum DirValue {
        Ltr,
        Rtl,
    }
    pub mod dir_value_with {
        pub fn deserialize<'de, D>(
            deserializer: D,
        ) -> ::core::result::Result<super::DirValue, D::Error>
        where
            D: ::xmlity::Deserializer<'de>,
        {
            let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                deserializer,
            )?;
            let value: ::std::string::String = text
                .parse()
                .map_err(::xmlity::de::Error::custom)?;
            super::DirValue::try_from(value).map_err(::xmlity::de::Error::custom)
        }
        pub fn serialize<S>(
            value: &super::DirValue,
            serializer: S,
        ) -> ::core::result::Result<S::Ok, S::Error>
        where
            S: ::xmlity::Serializer,
        {
            let value: ::std::string::String = ::core::clone::Clone::clone(value).into();
            ::xmlity::Serialize::serialize(
                ::std::string::String::as_str(
                    &::std::string::ToString::to_string(&value),
                ),
                serializer,
            )
        }
    }
    #[derive(::core::fmt::Debug)]
    pub enum DirValueParseError {
        NonExistent { value: ::std::string::String },
    }
    impl ::core::fmt::Display for DirValueParseError {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::result::Result<(), ::core::fmt::Error> {
            match self {
                DirValueParseError::NonExistent { value } => {
                    write!(f, "Value '{:?}' does not exist in the enumeration", value)
                }
            }
        }
    }
    impl ::core::convert::TryFrom<::std::string::String> for DirValue {
        type Error = DirValueParseError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::core::result::Result<Self, Self::Error> {
            match ::std::string::String::as_str(&value) {
                "ltr" => Ok(DirValue::Ltr),
                "rtl" => Ok(DirValue::Rtl),
                _ => {
                    Err(DirValueParseError::NonExistent {
                        value,
                    })
                }
            }
        }
    }
    impl ::core::convert::From<DirValue> for ::std::string::String {
        fn from(value: DirValue) -> Self {
            match value {
                DirValue::Ltr => ::std::string::String::from("ltr"),
                DirValue::Rtl => ::std::string::String::from("rtl"),
            }
        }
    }
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
#[xelement(
    name = "var",
    namespace = "http://www.w3.org/1999/xhtml",
    allow_unknown_attributes = "any"
)]
pub struct Var {
    #[xattribute(name = "id", optional, default)]
    pub id: ::core::option::Option<String>,
    #[xattribute(name = "class", optional, default)]
    pub class: ::core::option::Option<String>,
    #[xattribute(name = "style", optional, default)]
    pub style: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::StyleSheet>,
    >,
    #[xattribute(name = "title", optional, default)]
    pub title: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Text>>,
    #[xattribute(name = "lang", optional, default)]
    pub lang: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::LanguageCode>,
    >,
    #[xattribute(deferred = true, optional, default)]
    pub attribute_5: ::core::option::Option<crate::xml::Lang>,
    #[xattribute(name = "dir", optional, default)]
    pub dir: ::core::option::Option<var_items::DirValue>,
    #[xattribute(name = "onclick", optional, default)]
    pub onclick: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xattribute(name = "ondblclick", optional, default)]
    pub ondblclick: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmousedown", optional, default)]
    pub onmousedown: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseup", optional, default)]
    pub onmouseup: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseover", optional, default)]
    pub onmouseover: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmousemove", optional, default)]
    pub onmousemove: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onmouseout", optional, default)]
    pub onmouseout: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeypress", optional, default)]
    pub onkeypress: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeydown", optional, default)]
    pub onkeydown: ::core::option::Option<
        ::std::boxed::Box<crate::xhtml::types::Script>,
    >,
    #[xattribute(name = "onkeyup", optional, default)]
    pub onkeyup: ::core::option::Option<::std::boxed::Box<crate::xhtml::types::Script>>,
    #[xvalue(default)]
    pub inline: ::std::vec::Vec<var_items::Inline>,
}
