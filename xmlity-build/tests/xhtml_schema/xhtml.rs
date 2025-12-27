pub mod types {
    pub mod xhtml_coords_datatype_items {
        impl ::core::convert::From<::std::string::String> for XhtmlCoordsDatatype {
            fn from(value: ::std::string::String) -> Self {
                XhtmlCoordsDatatype(value)
            }
        }
        #[derive(
            ::core::fmt::Debug,
            ::xmlity::Serialize,
            ::xmlity::Deserialize,
            ::core::cmp::PartialEq,
            ::core::clone::Clone
        )]
        #[xvalue(with = xhtml_coords_datatype_with)]
        pub struct XhtmlCoordsDatatype(pub ::std::string::String);
        pub mod xhtml_coords_datatype_with {
            pub fn deserialize<'de, D>(
                deserializer: D,
            ) -> ::core::result::Result<super::XhtmlCoordsDatatype, D::Error>
            where
                D: ::xmlity::Deserializer<'de>,
            {
                let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                    deserializer,
                )?;
                let value: ::std::string::String = text
                    .parse()
                    .map_err(::xmlity::de::Error::custom)?;
                super::XhtmlCoordsDatatype::try_from(value)
                    .map_err(::xmlity::de::Error::custom)
            }
            pub fn serialize<S>(
                value: &super::XhtmlCoordsDatatype,
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
        pub enum XhtmlCoordsDatatypeParseError {}
        impl ::core::convert::From<XhtmlCoordsDatatype> for ::std::string::String {
            fn from(value: XhtmlCoordsDatatype) -> Self {
                value.0
            }
        }
    }
    pub type XhtmlCoordsDatatype = xhtml_coords_datatype_items::XhtmlCoordsDatatype;
    pub mod xhtml_input_type_class_items {
        #[derive(
            ::core::fmt::Debug,
            ::core::clone::Clone,
            ::core::marker::Copy,
            ::xmlity::Serialize,
            ::xmlity::Deserialize,
            ::core::cmp::PartialEq
        )]
        #[xvalue(with = xhtml_input_type_class_with)]
        pub enum XhtmlInputTypeClass {
            Text,
            Password,
            Checkbox,
            Radio,
            Submit,
            Reset,
            Hidden,
            Image,
            Button,
            File,
        }
        pub mod xhtml_input_type_class_with {
            pub fn deserialize<'de, D>(
                deserializer: D,
            ) -> ::core::result::Result<super::XhtmlInputTypeClass, D::Error>
            where
                D: ::xmlity::Deserializer<'de>,
            {
                let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                    deserializer,
                )?;
                let value: ::std::string::String = text
                    .parse()
                    .map_err(::xmlity::de::Error::custom)?;
                super::XhtmlInputTypeClass::try_from(value)
                    .map_err(::xmlity::de::Error::custom)
            }
            pub fn serialize<S>(
                value: &super::XhtmlInputTypeClass,
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
        pub enum XhtmlInputTypeClassParseError {
            NonExistent { value: ::std::string::String },
        }
        impl ::core::fmt::Display for XhtmlInputTypeClassParseError {
            fn fmt(
                &self,
                f: &mut ::core::fmt::Formatter<'_>,
            ) -> ::core::result::Result<(), ::core::fmt::Error> {
                match self {
                    XhtmlInputTypeClassParseError::NonExistent { value } => {
                        write!(f, "Value '{value:?}' does not exist in the enumeration")
                    }
                }
            }
        }
        impl ::core::convert::TryFrom<::std::string::String> for XhtmlInputTypeClass {
            type Error = XhtmlInputTypeClassParseError;
            fn try_from(
                value: ::std::string::String,
            ) -> ::core::result::Result<Self, Self::Error> {
                match ::std::string::String::as_str(&value) {
                    "text" => Ok(XhtmlInputTypeClass::Text),
                    "password" => Ok(XhtmlInputTypeClass::Password),
                    "checkbox" => Ok(XhtmlInputTypeClass::Checkbox),
                    "radio" => Ok(XhtmlInputTypeClass::Radio),
                    "submit" => Ok(XhtmlInputTypeClass::Submit),
                    "reset" => Ok(XhtmlInputTypeClass::Reset),
                    "hidden" => Ok(XhtmlInputTypeClass::Hidden),
                    "image" => Ok(XhtmlInputTypeClass::Image),
                    "button" => Ok(XhtmlInputTypeClass::Button),
                    "file" => Ok(XhtmlInputTypeClass::File),
                    _ => {
                        Err(XhtmlInputTypeClassParseError::NonExistent {
                            value,
                        })
                    }
                }
            }
        }
        impl ::core::convert::From<XhtmlInputTypeClass> for ::std::string::String {
            fn from(value: XhtmlInputTypeClass) -> Self {
                match value {
                    XhtmlInputTypeClass::Text => ::std::string::String::from("text"),
                    XhtmlInputTypeClass::Password => {
                        ::std::string::String::from("password")
                    }
                    XhtmlInputTypeClass::Checkbox => {
                        ::std::string::String::from("checkbox")
                    }
                    XhtmlInputTypeClass::Radio => ::std::string::String::from("radio"),
                    XhtmlInputTypeClass::Submit => ::std::string::String::from("submit"),
                    XhtmlInputTypeClass::Reset => ::std::string::String::from("reset"),
                    XhtmlInputTypeClass::Hidden => ::std::string::String::from("hidden"),
                    XhtmlInputTypeClass::Image => ::std::string::String::from("image"),
                    XhtmlInputTypeClass::Button => ::std::string::String::from("button"),
                    XhtmlInputTypeClass::File => ::std::string::String::from("file"),
                }
            }
        }
    }
    pub type XhtmlInputTypeClass = xhtml_input_type_class_items::XhtmlInputTypeClass;
    pub mod xhtml_shape_datatype_items {
        #[derive(
            ::core::fmt::Debug,
            ::core::clone::Clone,
            ::core::marker::Copy,
            ::xmlity::Serialize,
            ::xmlity::Deserialize,
            ::core::cmp::PartialEq
        )]
        #[xvalue(with = xhtml_shape_datatype_with)]
        pub enum XhtmlShapeDatatype {
            Rect,
            Circle,
            Poly,
            Default,
        }
        pub mod xhtml_shape_datatype_with {
            pub fn deserialize<'de, D>(
                deserializer: D,
            ) -> ::core::result::Result<super::XhtmlShapeDatatype, D::Error>
            where
                D: ::xmlity::Deserializer<'de>,
            {
                let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                    deserializer,
                )?;
                let value: ::std::string::String = text
                    .parse()
                    .map_err(::xmlity::de::Error::custom)?;
                super::XhtmlShapeDatatype::try_from(value)
                    .map_err(::xmlity::de::Error::custom)
            }
            pub fn serialize<S>(
                value: &super::XhtmlShapeDatatype,
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
        pub enum XhtmlShapeDatatypeParseError {
            NonExistent { value: ::std::string::String },
        }
        impl ::core::fmt::Display for XhtmlShapeDatatypeParseError {
            fn fmt(
                &self,
                f: &mut ::core::fmt::Formatter<'_>,
            ) -> ::core::result::Result<(), ::core::fmt::Error> {
                match self {
                    XhtmlShapeDatatypeParseError::NonExistent { value } => {
                        write!(f, "Value '{value:?}' does not exist in the enumeration")
                    }
                }
            }
        }
        impl ::core::convert::TryFrom<::std::string::String> for XhtmlShapeDatatype {
            type Error = XhtmlShapeDatatypeParseError;
            fn try_from(
                value: ::std::string::String,
            ) -> ::core::result::Result<Self, Self::Error> {
                match ::std::string::String::as_str(&value) {
                    "rect" => Ok(XhtmlShapeDatatype::Rect),
                    "circle" => Ok(XhtmlShapeDatatype::Circle),
                    "poly" => Ok(XhtmlShapeDatatype::Poly),
                    "default" => Ok(XhtmlShapeDatatype::Default),
                    _ => {
                        Err(XhtmlShapeDatatypeParseError::NonExistent {
                            value,
                        })
                    }
                }
            }
        }
        impl ::core::convert::From<XhtmlShapeDatatype> for ::std::string::String {
            fn from(value: XhtmlShapeDatatype) -> Self {
                match value {
                    XhtmlShapeDatatype::Rect => ::std::string::String::from("rect"),
                    XhtmlShapeDatatype::Circle => ::std::string::String::from("circle"),
                    XhtmlShapeDatatype::Poly => ::std::string::String::from("poly"),
                    XhtmlShapeDatatype::Default => ::std::string::String::from("default"),
                }
            }
        }
    }
    pub type XhtmlShapeDatatype = xhtml_shape_datatype_items::XhtmlShapeDatatype;
    pub mod xhtml_inl_pres_type_items {
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
                        write!(f, "Value '{value:?}' does not exist in the enumeration")
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
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub struct XhtmlInlPresType {
        #[xattribute(name = "id", optional)]
        pub id: ::core::option::Option<::xmlity_ns_xsd_types::Id>,
        #[xattribute(name = "class", optional)]
        pub class: ::core::option::Option<String>,
        #[xattribute(name = "title", optional)]
        pub title: ::core::option::Option<String>,
        #[xattribute(deferred = true, optional)]
        pub lang: ::core::option::Option<
            ::std::boxed::Box<xmlity_ns_xml::attributes::Lang>,
        >,
        #[xattribute(name = "dir", optional)]
        pub dir: ::core::option::Option<xhtml_inl_pres_type_items::DirValue>,
        #[xattribute(name = "style", optional)]
        pub style: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Cdata>,
        >,
        pub xhtml_inl_pres_content: ::std::boxed::Box<
            crate::xhtml_schema::xhtml::groups::XhtmlInlPresContent,
        >,
    }
    pub mod xhtml_a_type_items {
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
                        write!(f, "Value '{value:?}' does not exist in the enumeration")
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
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub struct XhtmlAType {
        #[xattribute(name = "id", optional)]
        pub id: ::core::option::Option<::xmlity_ns_xsd_types::Id>,
        #[xattribute(name = "class", optional)]
        pub class: ::core::option::Option<String>,
        #[xattribute(name = "title", optional)]
        pub title: ::core::option::Option<String>,
        #[xattribute(deferred = true, optional)]
        pub lang: ::core::option::Option<
            ::std::boxed::Box<xmlity_ns_xml::attributes::Lang>,
        >,
        #[xattribute(name = "dir", optional)]
        pub dir: ::core::option::Option<xhtml_a_type_items::DirValue>,
        #[xattribute(name = "style", optional)]
        pub style: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Cdata>,
        >,
        #[xattribute(name = "href", optional)]
        pub href: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Uri>,
        >,
        #[xattribute(name = "charset", optional)]
        pub charset: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Charset>,
        >,
        #[xattribute(name = "type", optional)]
        pub type_: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::ContentType>,
        >,
        #[xattribute(name = "hreflang", optional)]
        pub hreflang: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::LanguageCode>,
        >,
        #[xattribute(name = "rel", optional)]
        pub rel: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::LinkTypes>,
        >,
        #[xattribute(name = "rev", optional)]
        pub rev: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::LinkTypes>,
        >,
        #[xattribute(name = "accesskey", optional)]
        pub accesskey: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Character>,
        >,
        #[xattribute(name = "tabindex", optional)]
        pub tabindex: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Number>,
        >,
        #[xattribute(name = "shape", optional)]
        pub shape: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml::types::XhtmlShapeDatatype>,
        >,
        #[xattribute(name = "coords", optional)]
        pub coords: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml::types::XhtmlCoordsDatatype>,
        >,
        #[xattribute(name = "onfocus", optional)]
        pub onfocus: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Script>,
        >,
        #[xattribute(name = "onblur", optional)]
        pub onblur: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Script>,
        >,
        pub xhtml_a_content: ::std::boxed::Box<
            crate::xhtml_schema::xhtml::groups::XhtmlAContent,
        >,
    }
    pub mod xhtml_abbr_type_items {
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
                        write!(f, "Value '{value:?}' does not exist in the enumeration")
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
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub struct XhtmlAbbrType {
        #[xattribute(name = "id", optional)]
        pub id: ::core::option::Option<::xmlity_ns_xsd_types::Id>,
        #[xattribute(name = "class", optional)]
        pub class: ::core::option::Option<String>,
        #[xattribute(name = "title", optional)]
        pub title: ::core::option::Option<String>,
        #[xattribute(deferred = true, optional)]
        pub lang: ::core::option::Option<
            ::std::boxed::Box<xmlity_ns_xml::attributes::Lang>,
        >,
        #[xattribute(name = "dir", optional)]
        pub dir: ::core::option::Option<xhtml_abbr_type_items::DirValue>,
        #[xattribute(name = "style", optional)]
        pub style: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Cdata>,
        >,
        pub xhtml_abbr_content: ::std::boxed::Box<
            crate::xhtml_schema::xhtml::groups::XhtmlAbbrContent,
        >,
    }
    pub mod xhtml_acronym_type_items {
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
                        write!(f, "Value '{value:?}' does not exist in the enumeration")
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
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub struct XhtmlAcronymType {
        #[xattribute(name = "id", optional)]
        pub id: ::core::option::Option<::xmlity_ns_xsd_types::Id>,
        #[xattribute(name = "class", optional)]
        pub class: ::core::option::Option<String>,
        #[xattribute(name = "title", optional)]
        pub title: ::core::option::Option<String>,
        #[xattribute(deferred = true, optional)]
        pub lang: ::core::option::Option<
            ::std::boxed::Box<xmlity_ns_xml::attributes::Lang>,
        >,
        #[xattribute(name = "dir", optional)]
        pub dir: ::core::option::Option<xhtml_acronym_type_items::DirValue>,
        #[xattribute(name = "style", optional)]
        pub style: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Cdata>,
        >,
        pub xhtml_acronym_content: ::std::boxed::Box<
            crate::xhtml_schema::xhtml::groups::XhtmlAcronymContent,
        >,
    }
    pub mod xhtml_address_type_items {
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
                        write!(f, "Value '{value:?}' does not exist in the enumeration")
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
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub struct XhtmlAddressType {
        #[xattribute(name = "id", optional)]
        pub id: ::core::option::Option<::xmlity_ns_xsd_types::Id>,
        #[xattribute(name = "class", optional)]
        pub class: ::core::option::Option<String>,
        #[xattribute(name = "title", optional)]
        pub title: ::core::option::Option<String>,
        #[xattribute(deferred = true, optional)]
        pub lang: ::core::option::Option<
            ::std::boxed::Box<xmlity_ns_xml::attributes::Lang>,
        >,
        #[xattribute(name = "dir", optional)]
        pub dir: ::core::option::Option<xhtml_address_type_items::DirValue>,
        #[xattribute(name = "style", optional)]
        pub style: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Cdata>,
        >,
        pub xhtml_address_content: ::std::boxed::Box<
            crate::xhtml_schema::xhtml::groups::XhtmlAddressContent,
        >,
    }
    pub mod xhtml_area_type_items {
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
                        write!(f, "Value '{value:?}' does not exist in the enumeration")
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
                        write!(f, "Value '{value:?}' does not exist in the enumeration")
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
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub struct XhtmlAreaType {
        #[xattribute(name = "id", optional)]
        pub id: ::core::option::Option<::xmlity_ns_xsd_types::Id>,
        #[xattribute(name = "class", optional)]
        pub class: ::core::option::Option<String>,
        #[xattribute(name = "title", optional)]
        pub title: ::core::option::Option<String>,
        #[xattribute(deferred = true, optional)]
        pub lang: ::core::option::Option<
            ::std::boxed::Box<xmlity_ns_xml::attributes::Lang>,
        >,
        #[xattribute(name = "dir", optional)]
        pub dir: ::core::option::Option<xhtml_area_type_items::DirValue>,
        #[xattribute(name = "style", optional)]
        pub style: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Cdata>,
        >,
        #[xattribute(name = "href", optional)]
        pub href: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Uri>,
        >,
        #[xattribute(name = "shape", optional)]
        pub shape: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml::types::XhtmlShapeDatatype>,
        >,
        #[xattribute(name = "coords", optional)]
        pub coords: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml::types::XhtmlCoordsDatatype>,
        >,
        #[xattribute(name = "nohref", optional)]
        pub nohref: ::core::option::Option<xhtml_area_type_items::NohrefValue>,
        #[xattribute(name = "alt")]
        pub alt: ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Text>,
        #[xattribute(name = "tabindex", optional)]
        pub tabindex: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Number>,
        >,
        #[xattribute(name = "accesskey", optional)]
        pub accesskey: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Character>,
        >,
        #[xattribute(name = "onfocus", optional)]
        pub onfocus: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Script>,
        >,
        #[xattribute(name = "onblur", optional)]
        pub onblur: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Script>,
        >,
        pub xhtml_area_content: ::std::boxed::Box<
            crate::xhtml_schema::xhtml::groups::XhtmlAreaContent,
        >,
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub struct XhtmlBaseType {
        #[xattribute(name = "href")]
        pub href: ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Uri>,
        pub xhtml_base_content: ::std::boxed::Box<
            crate::xhtml_schema::xhtml::groups::XhtmlBaseContent,
        >,
    }
    pub mod xhtml_bdo_type_items {
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
                        write!(f, "Value '{value:?}' does not exist in the enumeration")
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
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub struct XhtmlBdoType {
        #[xattribute(name = "id", optional)]
        pub id: ::core::option::Option<::xmlity_ns_xsd_types::Id>,
        #[xattribute(name = "class", optional)]
        pub class: ::core::option::Option<String>,
        #[xattribute(name = "title", optional)]
        pub title: ::core::option::Option<String>,
        #[xattribute(name = "dir")]
        pub dir: xhtml_bdo_type_items::DirValue,
        pub xhtml_bdo_content: ::std::boxed::Box<
            crate::xhtml_schema::xhtml::groups::XhtmlBdoContent,
        >,
    }
    pub mod xhtml_blockquote_type_items {
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
                        write!(f, "Value '{value:?}' does not exist in the enumeration")
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
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub struct XhtmlBlockquoteType {
        #[xattribute(name = "id", optional)]
        pub id: ::core::option::Option<::xmlity_ns_xsd_types::Id>,
        #[xattribute(name = "class", optional)]
        pub class: ::core::option::Option<String>,
        #[xattribute(name = "title", optional)]
        pub title: ::core::option::Option<String>,
        #[xattribute(deferred = true, optional)]
        pub lang: ::core::option::Option<
            ::std::boxed::Box<xmlity_ns_xml::attributes::Lang>,
        >,
        #[xattribute(name = "dir", optional)]
        pub dir: ::core::option::Option<xhtml_blockquote_type_items::DirValue>,
        #[xattribute(name = "style", optional)]
        pub style: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Cdata>,
        >,
        #[xattribute(name = "cite", optional)]
        pub cite: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Uri>,
        >,
        pub xhtml_blockquote_content: ::std::boxed::Box<
            crate::xhtml_schema::xhtml::groups::XhtmlBlockquoteContent,
        >,
    }
    pub mod xhtml_body_type_items {
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
                        write!(f, "Value '{value:?}' does not exist in the enumeration")
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
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub struct XhtmlBodyType {
        #[xattribute(name = "id", optional)]
        pub id: ::core::option::Option<::xmlity_ns_xsd_types::Id>,
        #[xattribute(name = "class", optional)]
        pub class: ::core::option::Option<String>,
        #[xattribute(name = "title", optional)]
        pub title: ::core::option::Option<String>,
        #[xattribute(deferred = true, optional)]
        pub lang: ::core::option::Option<
            ::std::boxed::Box<xmlity_ns_xml::attributes::Lang>,
        >,
        #[xattribute(name = "dir", optional)]
        pub dir: ::core::option::Option<xhtml_body_type_items::DirValue>,
        #[xattribute(name = "style", optional)]
        pub style: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Cdata>,
        >,
        #[xattribute(name = "onload", optional)]
        pub onload: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Script>,
        >,
        #[xattribute(name = "onunload", optional)]
        pub onunload: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Script>,
        >,
        pub xhtml_body_content: ::std::boxed::Box<
            crate::xhtml_schema::xhtml::groups::XhtmlBodyContent,
        >,
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub struct XhtmlBrType {
        #[xattribute(name = "id", optional)]
        pub id: ::core::option::Option<::xmlity_ns_xsd_types::Id>,
        #[xattribute(name = "class", optional)]
        pub class: ::core::option::Option<String>,
        #[xattribute(name = "title", optional)]
        pub title: ::core::option::Option<String>,
        pub xhtml_br_content: ::std::boxed::Box<
            crate::xhtml_schema::xhtml::groups::XhtmlBrContent,
        >,
    }
    pub mod xhtml_button_type_items {
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
                        write!(f, "Value '{value:?}' does not exist in the enumeration")
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
                        write!(f, "Value '{value:?}' does not exist in the enumeration")
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
                super::DisabledValue::try_from(value)
                    .map_err(::xmlity::de::Error::custom)
            }
            pub fn serialize<S>(
                value: &super::DisabledValue,
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
                        write!(f, "Value '{value:?}' does not exist in the enumeration")
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
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub struct XhtmlButtonType {
        #[xattribute(name = "id", optional)]
        pub id: ::core::option::Option<::xmlity_ns_xsd_types::Id>,
        #[xattribute(name = "class", optional)]
        pub class: ::core::option::Option<String>,
        #[xattribute(name = "title", optional)]
        pub title: ::core::option::Option<String>,
        #[xattribute(deferred = true, optional)]
        pub lang: ::core::option::Option<
            ::std::boxed::Box<xmlity_ns_xml::attributes::Lang>,
        >,
        #[xattribute(name = "dir", optional)]
        pub dir: ::core::option::Option<xhtml_button_type_items::DirValue>,
        #[xattribute(name = "style", optional)]
        pub style: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Cdata>,
        >,
        #[xattribute(name = "name", optional)]
        pub name: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Cdata>,
        >,
        #[xattribute(name = "value", optional)]
        pub value: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Cdata>,
        >,
        #[xattribute(name = "type", optional)]
        pub type_: ::core::option::Option<xhtml_button_type_items::TypeValue>,
        #[xattribute(name = "disabled", optional)]
        pub disabled: ::core::option::Option<xhtml_button_type_items::DisabledValue>,
        #[xattribute(name = "tabindex", optional)]
        pub tabindex: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Number>,
        >,
        #[xattribute(name = "accesskey", optional)]
        pub accesskey: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Character>,
        >,
        #[xattribute(name = "onfocus", optional)]
        pub onfocus: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Script>,
        >,
        #[xattribute(name = "onblur", optional)]
        pub onblur: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Script>,
        >,
        pub xhtml_button_content: ::std::boxed::Box<
            crate::xhtml_schema::xhtml::groups::XhtmlButtonContent,
        >,
    }
    pub mod xhtml_caption_type_items {
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
                        write!(f, "Value '{value:?}' does not exist in the enumeration")
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
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub struct XhtmlCaptionType {
        #[xattribute(name = "id", optional)]
        pub id: ::core::option::Option<::xmlity_ns_xsd_types::Id>,
        #[xattribute(name = "class", optional)]
        pub class: ::core::option::Option<String>,
        #[xattribute(name = "title", optional)]
        pub title: ::core::option::Option<String>,
        #[xattribute(deferred = true, optional)]
        pub lang: ::core::option::Option<
            ::std::boxed::Box<xmlity_ns_xml::attributes::Lang>,
        >,
        #[xattribute(name = "dir", optional)]
        pub dir: ::core::option::Option<xhtml_caption_type_items::DirValue>,
        #[xattribute(name = "style", optional)]
        pub style: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Cdata>,
        >,
        pub xhtml_caption_content: ::std::boxed::Box<
            crate::xhtml_schema::xhtml::groups::XhtmlCaptionContent,
        >,
    }
    pub mod xhtml_cite_type_items {
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
                        write!(f, "Value '{value:?}' does not exist in the enumeration")
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
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub struct XhtmlCiteType {
        #[xattribute(name = "id", optional)]
        pub id: ::core::option::Option<::xmlity_ns_xsd_types::Id>,
        #[xattribute(name = "class", optional)]
        pub class: ::core::option::Option<String>,
        #[xattribute(name = "title", optional)]
        pub title: ::core::option::Option<String>,
        #[xattribute(deferred = true, optional)]
        pub lang: ::core::option::Option<
            ::std::boxed::Box<xmlity_ns_xml::attributes::Lang>,
        >,
        #[xattribute(name = "dir", optional)]
        pub dir: ::core::option::Option<xhtml_cite_type_items::DirValue>,
        #[xattribute(name = "style", optional)]
        pub style: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Cdata>,
        >,
        pub xhtml_cite_content: ::std::boxed::Box<
            crate::xhtml_schema::xhtml::groups::XhtmlCiteContent,
        >,
    }
    pub mod xhtml_code_type_items {
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
                        write!(f, "Value '{value:?}' does not exist in the enumeration")
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
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub struct XhtmlCodeType {
        #[xattribute(name = "id", optional)]
        pub id: ::core::option::Option<::xmlity_ns_xsd_types::Id>,
        #[xattribute(name = "class", optional)]
        pub class: ::core::option::Option<String>,
        #[xattribute(name = "title", optional)]
        pub title: ::core::option::Option<String>,
        #[xattribute(deferred = true, optional)]
        pub lang: ::core::option::Option<
            ::std::boxed::Box<xmlity_ns_xml::attributes::Lang>,
        >,
        #[xattribute(name = "dir", optional)]
        pub dir: ::core::option::Option<xhtml_code_type_items::DirValue>,
        #[xattribute(name = "style", optional)]
        pub style: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Cdata>,
        >,
        pub xhtml_code_content: ::std::boxed::Box<
            crate::xhtml_schema::xhtml::groups::XhtmlCodeContent,
        >,
    }
    pub mod xhtml_col_type_items {
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
                        write!(f, "Value '{value:?}' does not exist in the enumeration")
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
                        write!(f, "Value '{value:?}' does not exist in the enumeration")
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
                        write!(f, "Value '{value:?}' does not exist in the enumeration")
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
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub struct XhtmlColType {
        #[xattribute(name = "id", optional)]
        pub id: ::core::option::Option<::xmlity_ns_xsd_types::Id>,
        #[xattribute(name = "class", optional)]
        pub class: ::core::option::Option<String>,
        #[xattribute(name = "title", optional)]
        pub title: ::core::option::Option<String>,
        #[xattribute(deferred = true, optional)]
        pub lang: ::core::option::Option<
            ::std::boxed::Box<xmlity_ns_xml::attributes::Lang>,
        >,
        #[xattribute(name = "dir", optional)]
        pub dir: ::core::option::Option<xhtml_col_type_items::DirValue>,
        #[xattribute(name = "style", optional)]
        pub style: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Cdata>,
        >,
        #[xattribute(name = "span", optional)]
        pub span: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Number>,
        >,
        #[xattribute(name = "width", optional)]
        pub width: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::MultiLength>,
        >,
        #[xattribute(name = "align", optional)]
        pub align: ::core::option::Option<xhtml_col_type_items::AlignValue>,
        #[xattribute(name = "char", optional)]
        pub char: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Character>,
        >,
        #[xattribute(name = "charoff", optional)]
        pub charoff: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Length>,
        >,
        #[xattribute(name = "valign", optional)]
        pub valign: ::core::option::Option<xhtml_col_type_items::ValignValue>,
        pub xhtml_col_content: ::std::boxed::Box<
            crate::xhtml_schema::xhtml::groups::XhtmlColContent,
        >,
    }
    pub mod xhtml_colgroup_type_items {
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
                        write!(f, "Value '{value:?}' does not exist in the enumeration")
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
                        write!(f, "Value '{value:?}' does not exist in the enumeration")
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
                        write!(f, "Value '{value:?}' does not exist in the enumeration")
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
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub struct XhtmlColgroupType {
        #[xattribute(name = "id", optional)]
        pub id: ::core::option::Option<::xmlity_ns_xsd_types::Id>,
        #[xattribute(name = "class", optional)]
        pub class: ::core::option::Option<String>,
        #[xattribute(name = "title", optional)]
        pub title: ::core::option::Option<String>,
        #[xattribute(deferred = true, optional)]
        pub lang: ::core::option::Option<
            ::std::boxed::Box<xmlity_ns_xml::attributes::Lang>,
        >,
        #[xattribute(name = "dir", optional)]
        pub dir: ::core::option::Option<xhtml_colgroup_type_items::DirValue>,
        #[xattribute(name = "style", optional)]
        pub style: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Cdata>,
        >,
        #[xattribute(name = "span", optional)]
        pub span: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Number>,
        >,
        #[xattribute(name = "width", optional)]
        pub width: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::MultiLength>,
        >,
        #[xattribute(name = "align", optional)]
        pub align: ::core::option::Option<xhtml_colgroup_type_items::AlignValue>,
        #[xattribute(name = "char", optional)]
        pub char: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Character>,
        >,
        #[xattribute(name = "charoff", optional)]
        pub charoff: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Length>,
        >,
        #[xattribute(name = "valign", optional)]
        pub valign: ::core::option::Option<xhtml_colgroup_type_items::ValignValue>,
        pub xhtml_colgroup_content: ::std::boxed::Box<
            crate::xhtml_schema::xhtml::groups::XhtmlColgroupContent,
        >,
    }
    pub mod xhtml_dd_type_items {
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
                        write!(f, "Value '{value:?}' does not exist in the enumeration")
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
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub struct XhtmlDdType {
        #[xattribute(name = "id", optional)]
        pub id: ::core::option::Option<::xmlity_ns_xsd_types::Id>,
        #[xattribute(name = "class", optional)]
        pub class: ::core::option::Option<String>,
        #[xattribute(name = "title", optional)]
        pub title: ::core::option::Option<String>,
        #[xattribute(deferred = true, optional)]
        pub lang: ::core::option::Option<
            ::std::boxed::Box<xmlity_ns_xml::attributes::Lang>,
        >,
        #[xattribute(name = "dir", optional)]
        pub dir: ::core::option::Option<xhtml_dd_type_items::DirValue>,
        #[xattribute(name = "style", optional)]
        pub style: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Cdata>,
        >,
        pub xhtml_dd_content: ::std::boxed::Box<
            crate::xhtml_schema::xhtml::groups::XhtmlDdContent,
        >,
    }
    pub mod xhtml_dfn_type_items {
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
                        write!(f, "Value '{value:?}' does not exist in the enumeration")
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
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub struct XhtmlDfnType {
        #[xattribute(name = "id", optional)]
        pub id: ::core::option::Option<::xmlity_ns_xsd_types::Id>,
        #[xattribute(name = "class", optional)]
        pub class: ::core::option::Option<String>,
        #[xattribute(name = "title", optional)]
        pub title: ::core::option::Option<String>,
        #[xattribute(deferred = true, optional)]
        pub lang: ::core::option::Option<
            ::std::boxed::Box<xmlity_ns_xml::attributes::Lang>,
        >,
        #[xattribute(name = "dir", optional)]
        pub dir: ::core::option::Option<xhtml_dfn_type_items::DirValue>,
        #[xattribute(name = "style", optional)]
        pub style: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Cdata>,
        >,
        pub xhtml_dfn_content: ::std::boxed::Box<
            crate::xhtml_schema::xhtml::groups::XhtmlDfnContent,
        >,
    }
    pub mod xhtml_div_type_items {
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
                        write!(f, "Value '{value:?}' does not exist in the enumeration")
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
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub struct XhtmlDivType {
        #[xattribute(name = "id", optional)]
        pub id: ::core::option::Option<::xmlity_ns_xsd_types::Id>,
        #[xattribute(name = "class", optional)]
        pub class: ::core::option::Option<String>,
        #[xattribute(name = "title", optional)]
        pub title: ::core::option::Option<String>,
        #[xattribute(deferred = true, optional)]
        pub lang: ::core::option::Option<
            ::std::boxed::Box<xmlity_ns_xml::attributes::Lang>,
        >,
        #[xattribute(name = "dir", optional)]
        pub dir: ::core::option::Option<xhtml_div_type_items::DirValue>,
        #[xattribute(name = "style", optional)]
        pub style: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Cdata>,
        >,
        pub xhtml_div_content: ::std::boxed::Box<
            crate::xhtml_schema::xhtml::groups::XhtmlDivContent,
        >,
    }
    pub mod xhtml_dl_type_items {
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
                        write!(f, "Value '{value:?}' does not exist in the enumeration")
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
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub struct XhtmlDlType {
        #[xattribute(name = "id", optional)]
        pub id: ::core::option::Option<::xmlity_ns_xsd_types::Id>,
        #[xattribute(name = "class", optional)]
        pub class: ::core::option::Option<String>,
        #[xattribute(name = "title", optional)]
        pub title: ::core::option::Option<String>,
        #[xattribute(deferred = true, optional)]
        pub lang: ::core::option::Option<
            ::std::boxed::Box<xmlity_ns_xml::attributes::Lang>,
        >,
        #[xattribute(name = "dir", optional)]
        pub dir: ::core::option::Option<xhtml_dl_type_items::DirValue>,
        #[xattribute(name = "style", optional)]
        pub style: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Cdata>,
        >,
        pub xhtml_dl_content: ::std::boxed::Box<
            crate::xhtml_schema::xhtml::groups::XhtmlDlContent,
        >,
    }
    pub mod xhtml_dt_type_items {
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
                        write!(f, "Value '{value:?}' does not exist in the enumeration")
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
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub struct XhtmlDtType {
        #[xattribute(name = "id", optional)]
        pub id: ::core::option::Option<::xmlity_ns_xsd_types::Id>,
        #[xattribute(name = "class", optional)]
        pub class: ::core::option::Option<String>,
        #[xattribute(name = "title", optional)]
        pub title: ::core::option::Option<String>,
        #[xattribute(deferred = true, optional)]
        pub lang: ::core::option::Option<
            ::std::boxed::Box<xmlity_ns_xml::attributes::Lang>,
        >,
        #[xattribute(name = "dir", optional)]
        pub dir: ::core::option::Option<xhtml_dt_type_items::DirValue>,
        #[xattribute(name = "style", optional)]
        pub style: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Cdata>,
        >,
        pub xhtml_dt_content: ::std::boxed::Box<
            crate::xhtml_schema::xhtml::groups::XhtmlDtContent,
        >,
    }
    pub mod xhtml_edit_type_items {
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
                        write!(f, "Value '{value:?}' does not exist in the enumeration")
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
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub struct XhtmlEditType {
        #[xattribute(name = "id", optional)]
        pub id: ::core::option::Option<::xmlity_ns_xsd_types::Id>,
        #[xattribute(name = "class", optional)]
        pub class: ::core::option::Option<String>,
        #[xattribute(name = "title", optional)]
        pub title: ::core::option::Option<String>,
        #[xattribute(deferred = true, optional)]
        pub lang: ::core::option::Option<
            ::std::boxed::Box<xmlity_ns_xml::attributes::Lang>,
        >,
        #[xattribute(name = "dir", optional)]
        pub dir: ::core::option::Option<xhtml_edit_type_items::DirValue>,
        #[xattribute(name = "style", optional)]
        pub style: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Cdata>,
        >,
        #[xattribute(name = "cite", optional)]
        pub cite: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Uri>,
        >,
        #[xattribute(name = "datetime", optional)]
        pub datetime: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Datetime>,
        >,
        pub xhtml_edit_content: ::std::boxed::Box<
            crate::xhtml_schema::xhtml::groups::XhtmlEditContent,
        >,
    }
    pub mod xhtml_em_type_items {
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
                        write!(f, "Value '{value:?}' does not exist in the enumeration")
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
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub struct XhtmlEmType {
        #[xattribute(name = "id", optional)]
        pub id: ::core::option::Option<::xmlity_ns_xsd_types::Id>,
        #[xattribute(name = "class", optional)]
        pub class: ::core::option::Option<String>,
        #[xattribute(name = "title", optional)]
        pub title: ::core::option::Option<String>,
        #[xattribute(deferred = true, optional)]
        pub lang: ::core::option::Option<
            ::std::boxed::Box<xmlity_ns_xml::attributes::Lang>,
        >,
        #[xattribute(name = "dir", optional)]
        pub dir: ::core::option::Option<xhtml_em_type_items::DirValue>,
        #[xattribute(name = "style", optional)]
        pub style: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Cdata>,
        >,
        pub xhtml_em_content: ::std::boxed::Box<
            crate::xhtml_schema::xhtml::groups::XhtmlEmContent,
        >,
    }
    pub mod xhtml_fieldset_type_items {
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
                        write!(f, "Value '{value:?}' does not exist in the enumeration")
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
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub struct XhtmlFieldsetType {
        #[xattribute(name = "id", optional)]
        pub id: ::core::option::Option<::xmlity_ns_xsd_types::Id>,
        #[xattribute(name = "class", optional)]
        pub class: ::core::option::Option<String>,
        #[xattribute(name = "title", optional)]
        pub title: ::core::option::Option<String>,
        #[xattribute(deferred = true, optional)]
        pub lang: ::core::option::Option<
            ::std::boxed::Box<xmlity_ns_xml::attributes::Lang>,
        >,
        #[xattribute(name = "dir", optional)]
        pub dir: ::core::option::Option<xhtml_fieldset_type_items::DirValue>,
        #[xattribute(name = "style", optional)]
        pub style: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Cdata>,
        >,
        pub xhtml_fieldset_content: ::std::boxed::Box<
            crate::xhtml_schema::xhtml::groups::XhtmlFieldsetContent,
        >,
    }
    pub mod xhtml_form_type_items {
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
                        write!(f, "Value '{value:?}' does not exist in the enumeration")
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
                        write!(f, "Value '{value:?}' does not exist in the enumeration")
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
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub struct XhtmlFormType {
        #[xattribute(name = "id", optional)]
        pub id: ::core::option::Option<::xmlity_ns_xsd_types::Id>,
        #[xattribute(name = "class", optional)]
        pub class: ::core::option::Option<String>,
        #[xattribute(name = "title", optional)]
        pub title: ::core::option::Option<String>,
        #[xattribute(deferred = true, optional)]
        pub lang: ::core::option::Option<
            ::std::boxed::Box<xmlity_ns_xml::attributes::Lang>,
        >,
        #[xattribute(name = "dir", optional)]
        pub dir: ::core::option::Option<xhtml_form_type_items::DirValue>,
        #[xattribute(name = "style", optional)]
        pub style: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Cdata>,
        >,
        #[xattribute(name = "action")]
        pub action: ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Uri>,
        #[xattribute(name = "method", optional)]
        pub method: ::core::option::Option<xhtml_form_type_items::MethodValue>,
        #[xattribute(name = "enctype", optional)]
        pub enctype: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::ContentType>,
        >,
        #[xattribute(name = "accept-charset", optional)]
        pub accept_charset: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Charsets>,
        >,
        #[xattribute(name = "accept", optional)]
        pub accept: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::ContentTypes>,
        >,
        #[xattribute(name = "onsubmit", optional)]
        pub onsubmit: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Script>,
        >,
        #[xattribute(name = "onreset", optional)]
        pub onreset: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Script>,
        >,
        pub xhtml_form_content: ::std::boxed::Box<
            crate::xhtml_schema::xhtml::groups::XhtmlFormContent,
        >,
    }
    pub mod xhtml_h1_type_items {
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
                        write!(f, "Value '{value:?}' does not exist in the enumeration")
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
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub struct XhtmlH1Type {
        #[xattribute(name = "id", optional)]
        pub id: ::core::option::Option<::xmlity_ns_xsd_types::Id>,
        #[xattribute(name = "class", optional)]
        pub class: ::core::option::Option<String>,
        #[xattribute(name = "title", optional)]
        pub title: ::core::option::Option<String>,
        #[xattribute(deferred = true, optional)]
        pub lang: ::core::option::Option<
            ::std::boxed::Box<xmlity_ns_xml::attributes::Lang>,
        >,
        #[xattribute(name = "dir", optional)]
        pub dir: ::core::option::Option<xhtml_h1_type_items::DirValue>,
        #[xattribute(name = "style", optional)]
        pub style: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Cdata>,
        >,
        pub xhtml_h1_content: ::std::boxed::Box<
            crate::xhtml_schema::xhtml::groups::XhtmlH1Content,
        >,
    }
    pub mod xhtml_h2_type_items {
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
                        write!(f, "Value '{value:?}' does not exist in the enumeration")
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
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub struct XhtmlH2Type {
        #[xattribute(name = "id", optional)]
        pub id: ::core::option::Option<::xmlity_ns_xsd_types::Id>,
        #[xattribute(name = "class", optional)]
        pub class: ::core::option::Option<String>,
        #[xattribute(name = "title", optional)]
        pub title: ::core::option::Option<String>,
        #[xattribute(deferred = true, optional)]
        pub lang: ::core::option::Option<
            ::std::boxed::Box<xmlity_ns_xml::attributes::Lang>,
        >,
        #[xattribute(name = "dir", optional)]
        pub dir: ::core::option::Option<xhtml_h2_type_items::DirValue>,
        #[xattribute(name = "style", optional)]
        pub style: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Cdata>,
        >,
        pub xhtml_h2_content: ::std::boxed::Box<
            crate::xhtml_schema::xhtml::groups::XhtmlH2Content,
        >,
    }
    pub mod xhtml_h3_type_items {
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
                        write!(f, "Value '{value:?}' does not exist in the enumeration")
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
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub struct XhtmlH3Type {
        #[xattribute(name = "id", optional)]
        pub id: ::core::option::Option<::xmlity_ns_xsd_types::Id>,
        #[xattribute(name = "class", optional)]
        pub class: ::core::option::Option<String>,
        #[xattribute(name = "title", optional)]
        pub title: ::core::option::Option<String>,
        #[xattribute(deferred = true, optional)]
        pub lang: ::core::option::Option<
            ::std::boxed::Box<xmlity_ns_xml::attributes::Lang>,
        >,
        #[xattribute(name = "dir", optional)]
        pub dir: ::core::option::Option<xhtml_h3_type_items::DirValue>,
        #[xattribute(name = "style", optional)]
        pub style: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Cdata>,
        >,
        pub xhtml_h3_content: ::std::boxed::Box<
            crate::xhtml_schema::xhtml::groups::XhtmlH3Content,
        >,
    }
    pub mod xhtml_h4_type_items {
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
                        write!(f, "Value '{value:?}' does not exist in the enumeration")
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
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub struct XhtmlH4Type {
        #[xattribute(name = "id", optional)]
        pub id: ::core::option::Option<::xmlity_ns_xsd_types::Id>,
        #[xattribute(name = "class", optional)]
        pub class: ::core::option::Option<String>,
        #[xattribute(name = "title", optional)]
        pub title: ::core::option::Option<String>,
        #[xattribute(deferred = true, optional)]
        pub lang: ::core::option::Option<
            ::std::boxed::Box<xmlity_ns_xml::attributes::Lang>,
        >,
        #[xattribute(name = "dir", optional)]
        pub dir: ::core::option::Option<xhtml_h4_type_items::DirValue>,
        #[xattribute(name = "style", optional)]
        pub style: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Cdata>,
        >,
        pub xhtml_h4_content: ::std::boxed::Box<
            crate::xhtml_schema::xhtml::groups::XhtmlH4Content,
        >,
    }
    pub mod xhtml_h5_type_items {
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
                        write!(f, "Value '{value:?}' does not exist in the enumeration")
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
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub struct XhtmlH5Type {
        #[xattribute(name = "id", optional)]
        pub id: ::core::option::Option<::xmlity_ns_xsd_types::Id>,
        #[xattribute(name = "class", optional)]
        pub class: ::core::option::Option<String>,
        #[xattribute(name = "title", optional)]
        pub title: ::core::option::Option<String>,
        #[xattribute(deferred = true, optional)]
        pub lang: ::core::option::Option<
            ::std::boxed::Box<xmlity_ns_xml::attributes::Lang>,
        >,
        #[xattribute(name = "dir", optional)]
        pub dir: ::core::option::Option<xhtml_h5_type_items::DirValue>,
        #[xattribute(name = "style", optional)]
        pub style: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Cdata>,
        >,
        pub xhtml_h5_content: ::std::boxed::Box<
            crate::xhtml_schema::xhtml::groups::XhtmlH5Content,
        >,
    }
    pub mod xhtml_h6_type_items {
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
                        write!(f, "Value '{value:?}' does not exist in the enumeration")
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
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub struct XhtmlH6Type {
        #[xattribute(name = "id", optional)]
        pub id: ::core::option::Option<::xmlity_ns_xsd_types::Id>,
        #[xattribute(name = "class", optional)]
        pub class: ::core::option::Option<String>,
        #[xattribute(name = "title", optional)]
        pub title: ::core::option::Option<String>,
        #[xattribute(deferred = true, optional)]
        pub lang: ::core::option::Option<
            ::std::boxed::Box<xmlity_ns_xml::attributes::Lang>,
        >,
        #[xattribute(name = "dir", optional)]
        pub dir: ::core::option::Option<xhtml_h6_type_items::DirValue>,
        #[xattribute(name = "style", optional)]
        pub style: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Cdata>,
        >,
        pub xhtml_h6_content: ::std::boxed::Box<
            crate::xhtml_schema::xhtml::groups::XhtmlH6Content,
        >,
    }
    pub mod xhtml_head_type_items {
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
                        write!(f, "Value '{value:?}' does not exist in the enumeration")
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
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub struct XhtmlHeadType {
        #[xattribute(name = "profile", optional)]
        pub profile: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Uri>,
        >,
        #[xattribute(deferred = true, optional)]
        pub lang: ::core::option::Option<
            ::std::boxed::Box<xmlity_ns_xml::attributes::Lang>,
        >,
        #[xattribute(name = "dir", optional)]
        pub dir: ::core::option::Option<xhtml_head_type_items::DirValue>,
        pub xhtml_head_content: ::std::boxed::Box<
            crate::xhtml_schema::xhtml::groups::XhtmlHeadContent,
        >,
    }
    pub mod xhtml_heading_type_items {
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
                        write!(f, "Value '{value:?}' does not exist in the enumeration")
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
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub struct XhtmlHeadingType {
        #[xattribute(name = "id", optional)]
        pub id: ::core::option::Option<::xmlity_ns_xsd_types::Id>,
        #[xattribute(name = "class", optional)]
        pub class: ::core::option::Option<String>,
        #[xattribute(name = "title", optional)]
        pub title: ::core::option::Option<String>,
        #[xattribute(deferred = true, optional)]
        pub lang: ::core::option::Option<
            ::std::boxed::Box<xmlity_ns_xml::attributes::Lang>,
        >,
        #[xattribute(name = "dir", optional)]
        pub dir: ::core::option::Option<xhtml_heading_type_items::DirValue>,
        #[xattribute(name = "style", optional)]
        pub style: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Cdata>,
        >,
        #[xvalue(default)]
        pub xhtml_inline_mix: ::std::vec::Vec<
            crate::xhtml_schema::xhtml::groups::XhtmlInlineMix,
        >,
    }
    pub mod xhtml_hr_type_items {
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
                        write!(f, "Value '{value:?}' does not exist in the enumeration")
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
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub struct XhtmlHrType {
        #[xattribute(name = "id", optional)]
        pub id: ::core::option::Option<::xmlity_ns_xsd_types::Id>,
        #[xattribute(name = "class", optional)]
        pub class: ::core::option::Option<String>,
        #[xattribute(name = "title", optional)]
        pub title: ::core::option::Option<String>,
        #[xattribute(deferred = true, optional)]
        pub lang: ::core::option::Option<
            ::std::boxed::Box<xmlity_ns_xml::attributes::Lang>,
        >,
        #[xattribute(name = "dir", optional)]
        pub dir: ::core::option::Option<xhtml_hr_type_items::DirValue>,
        #[xattribute(name = "style", optional)]
        pub style: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Cdata>,
        >,
        pub xhtml_hr_content: ::std::boxed::Box<
            crate::xhtml_schema::xhtml::groups::XhtmlHrContent,
        >,
    }
    pub mod xhtml_html_type_items {
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
                        write!(f, "Value '{value:?}' does not exist in the enumeration")
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
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub struct XhtmlHtmlType {
        #[xattribute(name = "version", optional)]
        pub version: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Fpi>,
        >,
        #[xattribute(deferred = true, optional)]
        pub lang: ::core::option::Option<
            ::std::boxed::Box<xmlity_ns_xml::attributes::Lang>,
        >,
        #[xattribute(name = "dir", optional)]
        pub dir: ::core::option::Option<xhtml_html_type_items::DirValue>,
        pub xhtml_html_content: ::std::boxed::Box<
            crate::xhtml_schema::xhtml::groups::XhtmlHtmlContent,
        >,
    }
    pub mod xhtml_img_type_items {
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
                        write!(f, "Value '{value:?}' does not exist in the enumeration")
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
                        write!(f, "Value '{value:?}' does not exist in the enumeration")
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
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub struct XhtmlImgType {
        #[xattribute(name = "id", optional)]
        pub id: ::core::option::Option<::xmlity_ns_xsd_types::Id>,
        #[xattribute(name = "class", optional)]
        pub class: ::core::option::Option<String>,
        #[xattribute(name = "title", optional)]
        pub title: ::core::option::Option<String>,
        #[xattribute(deferred = true, optional)]
        pub lang: ::core::option::Option<
            ::std::boxed::Box<xmlity_ns_xml::attributes::Lang>,
        >,
        #[xattribute(name = "dir", optional)]
        pub dir: ::core::option::Option<xhtml_img_type_items::DirValue>,
        #[xattribute(name = "style", optional)]
        pub style: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Cdata>,
        >,
        #[xattribute(name = "src")]
        pub src: ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Uri>,
        #[xattribute(name = "alt")]
        pub alt: ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Text>,
        #[xattribute(name = "longdesc", optional)]
        pub longdesc: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Uri>,
        >,
        #[xattribute(name = "height", optional)]
        pub height: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Length>,
        >,
        #[xattribute(name = "width", optional)]
        pub width: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Length>,
        >,
        #[xattribute(name = "usemap", optional)]
        pub usemap: ::core::option::Option<::xmlity_ns_xsd_types::IdRef>,
        #[xattribute(name = "ismap", optional)]
        pub ismap: ::core::option::Option<xhtml_img_type_items::IsmapValue>,
        pub xhtml_img_content: ::std::boxed::Box<
            crate::xhtml_schema::xhtml::groups::XhtmlImgContent,
        >,
    }
    pub mod xhtml_input_type_items {
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
                        write!(f, "Value '{value:?}' does not exist in the enumeration")
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
                        write!(f, "Value '{value:?}' does not exist in the enumeration")
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
                super::DisabledValue::try_from(value)
                    .map_err(::xmlity::de::Error::custom)
            }
            pub fn serialize<S>(
                value: &super::DisabledValue,
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
                        write!(f, "Value '{value:?}' does not exist in the enumeration")
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
                super::ReadonlyValue::try_from(value)
                    .map_err(::xmlity::de::Error::custom)
            }
            pub fn serialize<S>(
                value: &super::ReadonlyValue,
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
                        write!(f, "Value '{value:?}' does not exist in the enumeration")
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
                        write!(f, "Value '{value:?}' does not exist in the enumeration")
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
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub struct XhtmlInputType {
        #[xattribute(name = "id", optional)]
        pub id: ::core::option::Option<::xmlity_ns_xsd_types::Id>,
        #[xattribute(name = "class", optional)]
        pub class: ::core::option::Option<String>,
        #[xattribute(name = "title", optional)]
        pub title: ::core::option::Option<String>,
        #[xattribute(deferred = true, optional)]
        pub lang: ::core::option::Option<
            ::std::boxed::Box<xmlity_ns_xml::attributes::Lang>,
        >,
        #[xattribute(name = "dir", optional)]
        pub dir: ::core::option::Option<xhtml_input_type_items::DirValue>,
        #[xattribute(name = "style", optional)]
        pub style: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Cdata>,
        >,
        #[xattribute(name = "type", optional)]
        pub type_: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml::types::XhtmlInputTypeClass>,
        >,
        #[xattribute(name = "name", optional)]
        pub name: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Cdata>,
        >,
        #[xattribute(name = "value", optional)]
        pub value: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Cdata>,
        >,
        #[xattribute(name = "checked", optional)]
        pub checked: ::core::option::Option<xhtml_input_type_items::CheckedValue>,
        #[xattribute(name = "disabled", optional)]
        pub disabled: ::core::option::Option<xhtml_input_type_items::DisabledValue>,
        #[xattribute(name = "readonly", optional)]
        pub readonly: ::core::option::Option<xhtml_input_type_items::ReadonlyValue>,
        #[xattribute(name = "size", optional)]
        pub size: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Number>,
        >,
        #[xattribute(name = "maxlength", optional)]
        pub maxlength: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Number>,
        >,
        #[xattribute(name = "src", optional)]
        pub src: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Uri>,
        >,
        #[xattribute(name = "alt", optional)]
        pub alt: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Text>,
        >,
        #[xattribute(name = "tabindex", optional)]
        pub tabindex: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Number>,
        >,
        #[xattribute(name = "accesskey", optional)]
        pub accesskey: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Character>,
        >,
        #[xattribute(name = "accept", optional)]
        pub accept: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::ContentTypes>,
        >,
        #[xattribute(name = "usemap", optional)]
        pub usemap: ::core::option::Option<::xmlity_ns_xsd_types::IdRef>,
        #[xattribute(name = "ismap", optional)]
        pub ismap: ::core::option::Option<xhtml_input_type_items::IsmapValue>,
        #[xattribute(name = "onfocus", optional)]
        pub onfocus: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Script>,
        >,
        #[xattribute(name = "onblur", optional)]
        pub onblur: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Script>,
        >,
        #[xattribute(name = "onselect", optional)]
        pub onselect: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Script>,
        >,
        #[xattribute(name = "onchange", optional)]
        pub onchange: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Script>,
        >,
        pub xhtml_input_content: ::std::boxed::Box<
            crate::xhtml_schema::xhtml::groups::XhtmlInputContent,
        >,
    }
    pub mod xhtml_kbd_type_items {
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
                        write!(f, "Value '{value:?}' does not exist in the enumeration")
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
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub struct XhtmlKbdType {
        #[xattribute(name = "id", optional)]
        pub id: ::core::option::Option<::xmlity_ns_xsd_types::Id>,
        #[xattribute(name = "class", optional)]
        pub class: ::core::option::Option<String>,
        #[xattribute(name = "title", optional)]
        pub title: ::core::option::Option<String>,
        #[xattribute(deferred = true, optional)]
        pub lang: ::core::option::Option<
            ::std::boxed::Box<xmlity_ns_xml::attributes::Lang>,
        >,
        #[xattribute(name = "dir", optional)]
        pub dir: ::core::option::Option<xhtml_kbd_type_items::DirValue>,
        #[xattribute(name = "style", optional)]
        pub style: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Cdata>,
        >,
        pub xhtml_kbd_content: ::std::boxed::Box<
            crate::xhtml_schema::xhtml::groups::XhtmlKbdContent,
        >,
    }
    pub mod xhtml_label_type_items {
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
                        write!(f, "Value '{value:?}' does not exist in the enumeration")
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
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub struct XhtmlLabelType {
        #[xattribute(name = "id", optional)]
        pub id: ::core::option::Option<::xmlity_ns_xsd_types::Id>,
        #[xattribute(name = "class", optional)]
        pub class: ::core::option::Option<String>,
        #[xattribute(name = "title", optional)]
        pub title: ::core::option::Option<String>,
        #[xattribute(deferred = true, optional)]
        pub lang: ::core::option::Option<
            ::std::boxed::Box<xmlity_ns_xml::attributes::Lang>,
        >,
        #[xattribute(name = "dir", optional)]
        pub dir: ::core::option::Option<xhtml_label_type_items::DirValue>,
        #[xattribute(name = "style", optional)]
        pub style: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Cdata>,
        >,
        #[xattribute(name = "for", optional)]
        pub for_: ::core::option::Option<::xmlity_ns_xsd_types::IdRef>,
        #[xattribute(name = "accesskey", optional)]
        pub accesskey: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Character>,
        >,
        #[xattribute(name = "onfocus", optional)]
        pub onfocus: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Script>,
        >,
        #[xattribute(name = "onblur", optional)]
        pub onblur: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Script>,
        >,
        pub xhtml_label_content: ::std::boxed::Box<
            crate::xhtml_schema::xhtml::groups::XhtmlLabelContent,
        >,
    }
    pub mod xhtml_legend_type_items {
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
                        write!(f, "Value '{value:?}' does not exist in the enumeration")
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
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub struct XhtmlLegendType {
        #[xattribute(name = "id", optional)]
        pub id: ::core::option::Option<::xmlity_ns_xsd_types::Id>,
        #[xattribute(name = "class", optional)]
        pub class: ::core::option::Option<String>,
        #[xattribute(name = "title", optional)]
        pub title: ::core::option::Option<String>,
        #[xattribute(deferred = true, optional)]
        pub lang: ::core::option::Option<
            ::std::boxed::Box<xmlity_ns_xml::attributes::Lang>,
        >,
        #[xattribute(name = "dir", optional)]
        pub dir: ::core::option::Option<xhtml_legend_type_items::DirValue>,
        #[xattribute(name = "style", optional)]
        pub style: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Cdata>,
        >,
        #[xattribute(name = "accesskey", optional)]
        pub accesskey: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Character>,
        >,
        pub xhtml_legend_content: ::std::boxed::Box<
            crate::xhtml_schema::xhtml::groups::XhtmlLegendContent,
        >,
    }
    pub mod xhtml_li_type_items {
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
                        write!(f, "Value '{value:?}' does not exist in the enumeration")
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
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub struct XhtmlLiType {
        #[xattribute(name = "id", optional)]
        pub id: ::core::option::Option<::xmlity_ns_xsd_types::Id>,
        #[xattribute(name = "class", optional)]
        pub class: ::core::option::Option<String>,
        #[xattribute(name = "title", optional)]
        pub title: ::core::option::Option<String>,
        #[xattribute(deferred = true, optional)]
        pub lang: ::core::option::Option<
            ::std::boxed::Box<xmlity_ns_xml::attributes::Lang>,
        >,
        #[xattribute(name = "dir", optional)]
        pub dir: ::core::option::Option<xhtml_li_type_items::DirValue>,
        #[xattribute(name = "style", optional)]
        pub style: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Cdata>,
        >,
        pub xhtml_li_content: ::std::boxed::Box<
            crate::xhtml_schema::xhtml::groups::XhtmlLiContent,
        >,
    }
    pub mod xhtml_link_type_items {
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
                        write!(f, "Value '{value:?}' does not exist in the enumeration")
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
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub struct XhtmlLinkType {
        #[xattribute(name = "id", optional)]
        pub id: ::core::option::Option<::xmlity_ns_xsd_types::Id>,
        #[xattribute(name = "class", optional)]
        pub class: ::core::option::Option<String>,
        #[xattribute(name = "title", optional)]
        pub title: ::core::option::Option<String>,
        #[xattribute(deferred = true, optional)]
        pub lang: ::core::option::Option<
            ::std::boxed::Box<xmlity_ns_xml::attributes::Lang>,
        >,
        #[xattribute(name = "dir", optional)]
        pub dir: ::core::option::Option<xhtml_link_type_items::DirValue>,
        #[xattribute(name = "style", optional)]
        pub style: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Cdata>,
        >,
        #[xattribute(name = "charset", optional)]
        pub charset: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Charset>,
        >,
        #[xattribute(name = "href", optional)]
        pub href: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Uri>,
        >,
        #[xattribute(name = "hreflang", optional)]
        pub hreflang: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::LanguageCode>,
        >,
        #[xattribute(name = "type", optional)]
        pub type_: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::ContentType>,
        >,
        #[xattribute(name = "rel", optional)]
        pub rel: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::LinkTypes>,
        >,
        #[xattribute(name = "rev", optional)]
        pub rev: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::LinkTypes>,
        >,
        #[xattribute(name = "media", optional)]
        pub media: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::MediaDesc>,
        >,
        pub xhtml_link_content: ::std::boxed::Box<
            crate::xhtml_schema::xhtml::groups::XhtmlLinkContent,
        >,
    }
    pub mod xhtml_map_type_items {
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
                        write!(f, "Value '{value:?}' does not exist in the enumeration")
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
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub struct XhtmlMapType {
        #[xattribute(name = "id")]
        pub id: ::xmlity_ns_xsd_types::Id,
        #[xattribute(name = "class", optional)]
        pub class: ::core::option::Option<String>,
        #[xattribute(name = "title", optional)]
        pub title: ::core::option::Option<String>,
        #[xattribute(deferred = true, optional)]
        pub lang: ::core::option::Option<
            ::std::boxed::Box<xmlity_ns_xml::attributes::Lang>,
        >,
        #[xattribute(name = "dir", optional)]
        pub dir: ::core::option::Option<xhtml_map_type_items::DirValue>,
        pub xhtml_map_content: ::std::boxed::Box<
            crate::xhtml_schema::xhtml::groups::XhtmlMapContent,
        >,
    }
    pub mod xhtml_meta_type_items {
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
                        write!(f, "Value '{value:?}' does not exist in the enumeration")
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
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub struct XhtmlMetaType {
        #[xattribute(deferred = true, optional)]
        pub lang: ::core::option::Option<
            ::std::boxed::Box<xmlity_ns_xml::attributes::Lang>,
        >,
        #[xattribute(name = "dir", optional)]
        pub dir: ::core::option::Option<xhtml_meta_type_items::DirValue>,
        #[xattribute(name = "http-equiv", optional)]
        pub http_equiv: ::core::option::Option<::xmlity_ns_xsd_types::NMToken>,
        #[xattribute(name = "name", optional)]
        pub name: ::core::option::Option<::xmlity_ns_xsd_types::NMToken>,
        #[xattribute(name = "content")]
        pub content: ::std::boxed::Box<
            crate::xhtml_schema::xhtml_datatypes::types::Cdata,
        >,
        #[xattribute(name = "scheme", optional)]
        pub scheme: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Cdata>,
        >,
        pub xhtml_meta_content: ::std::boxed::Box<
            crate::xhtml_schema::xhtml::groups::XhtmlMetaContent,
        >,
    }
    pub mod xhtml_noscript_type_items {
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
                        write!(f, "Value '{value:?}' does not exist in the enumeration")
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
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub struct XhtmlNoscriptType {
        #[xattribute(name = "id", optional)]
        pub id: ::core::option::Option<::xmlity_ns_xsd_types::Id>,
        #[xattribute(name = "class", optional)]
        pub class: ::core::option::Option<String>,
        #[xattribute(name = "title", optional)]
        pub title: ::core::option::Option<String>,
        #[xattribute(deferred = true, optional)]
        pub lang: ::core::option::Option<
            ::std::boxed::Box<xmlity_ns_xml::attributes::Lang>,
        >,
        #[xattribute(name = "dir", optional)]
        pub dir: ::core::option::Option<xhtml_noscript_type_items::DirValue>,
        #[xattribute(name = "style", optional)]
        pub style: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Cdata>,
        >,
        pub xhtml_noscript_content: ::std::boxed::Box<
            crate::xhtml_schema::xhtml::groups::XhtmlNoscriptContent,
        >,
    }
    pub mod xhtml_object_type_items {
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
                        write!(f, "Value '{value:?}' does not exist in the enumeration")
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
                        write!(f, "Value '{value:?}' does not exist in the enumeration")
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
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub struct XhtmlObjectType {
        #[xattribute(name = "id", optional)]
        pub id: ::core::option::Option<::xmlity_ns_xsd_types::Id>,
        #[xattribute(name = "class", optional)]
        pub class: ::core::option::Option<String>,
        #[xattribute(name = "title", optional)]
        pub title: ::core::option::Option<String>,
        #[xattribute(deferred = true, optional)]
        pub lang: ::core::option::Option<
            ::std::boxed::Box<xmlity_ns_xml::attributes::Lang>,
        >,
        #[xattribute(name = "dir", optional)]
        pub dir: ::core::option::Option<xhtml_object_type_items::DirValue>,
        #[xattribute(name = "style", optional)]
        pub style: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Cdata>,
        >,
        #[xattribute(name = "declare", optional)]
        pub declare: ::core::option::Option<xhtml_object_type_items::DeclareValue>,
        #[xattribute(name = "classid", optional)]
        pub classid: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Uri>,
        >,
        #[xattribute(name = "codebase", optional)]
        pub codebase: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Uri>,
        >,
        #[xattribute(name = "data", optional)]
        pub data: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Uri>,
        >,
        #[xattribute(name = "type", optional)]
        pub type_: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::ContentType>,
        >,
        #[xattribute(name = "codetype", optional)]
        pub codetype: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::ContentType>,
        >,
        #[xattribute(name = "archive", optional)]
        pub archive: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Uris>,
        >,
        #[xattribute(name = "standby", optional)]
        pub standby: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Text>,
        >,
        #[xattribute(name = "height", optional)]
        pub height: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Length>,
        >,
        #[xattribute(name = "width", optional)]
        pub width: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Length>,
        >,
        #[xattribute(name = "name", optional)]
        pub name: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Cdata>,
        >,
        #[xattribute(name = "tabindex", optional)]
        pub tabindex: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Number>,
        >,
        #[xattribute(name = "usemap", optional)]
        pub usemap: ::core::option::Option<::xmlity_ns_xsd_types::IdRef>,
        pub xhtml_object_content: ::std::boxed::Box<
            crate::xhtml_schema::xhtml::groups::XhtmlObjectContent,
        >,
    }
    pub mod xhtml_ol_type_items {
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
                        write!(f, "Value '{value:?}' does not exist in the enumeration")
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
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub struct XhtmlOlType {
        #[xattribute(name = "id", optional)]
        pub id: ::core::option::Option<::xmlity_ns_xsd_types::Id>,
        #[xattribute(name = "class", optional)]
        pub class: ::core::option::Option<String>,
        #[xattribute(name = "title", optional)]
        pub title: ::core::option::Option<String>,
        #[xattribute(deferred = true, optional)]
        pub lang: ::core::option::Option<
            ::std::boxed::Box<xmlity_ns_xml::attributes::Lang>,
        >,
        #[xattribute(name = "dir", optional)]
        pub dir: ::core::option::Option<xhtml_ol_type_items::DirValue>,
        #[xattribute(name = "style", optional)]
        pub style: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Cdata>,
        >,
        pub xhtml_ol_content: ::std::boxed::Box<
            crate::xhtml_schema::xhtml::groups::XhtmlOlContent,
        >,
    }
    pub mod xhtml_optgroup_type_items {
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
                        write!(f, "Value '{value:?}' does not exist in the enumeration")
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
                super::DisabledValue::try_from(value)
                    .map_err(::xmlity::de::Error::custom)
            }
            pub fn serialize<S>(
                value: &super::DisabledValue,
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
                        write!(f, "Value '{value:?}' does not exist in the enumeration")
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
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub struct XhtmlOptgroupType {
        #[xattribute(name = "id", optional)]
        pub id: ::core::option::Option<::xmlity_ns_xsd_types::Id>,
        #[xattribute(name = "class", optional)]
        pub class: ::core::option::Option<String>,
        #[xattribute(name = "title", optional)]
        pub title: ::core::option::Option<String>,
        #[xattribute(deferred = true, optional)]
        pub lang: ::core::option::Option<
            ::std::boxed::Box<xmlity_ns_xml::attributes::Lang>,
        >,
        #[xattribute(name = "dir", optional)]
        pub dir: ::core::option::Option<xhtml_optgroup_type_items::DirValue>,
        #[xattribute(name = "style", optional)]
        pub style: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Cdata>,
        >,
        #[xattribute(name = "disabled", optional)]
        pub disabled: ::core::option::Option<xhtml_optgroup_type_items::DisabledValue>,
        #[xattribute(name = "label")]
        pub label: ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Text>,
        pub xhtml_optgroup_content: ::std::boxed::Box<
            crate::xhtml_schema::xhtml::groups::XhtmlOptgroupContent,
        >,
    }
    pub mod xhtml_option_type_items {
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
                        write!(f, "Value '{value:?}' does not exist in the enumeration")
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
                super::SelectedValue::try_from(value)
                    .map_err(::xmlity::de::Error::custom)
            }
            pub fn serialize<S>(
                value: &super::SelectedValue,
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
                        write!(f, "Value '{value:?}' does not exist in the enumeration")
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
                super::DisabledValue::try_from(value)
                    .map_err(::xmlity::de::Error::custom)
            }
            pub fn serialize<S>(
                value: &super::DisabledValue,
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
                        write!(f, "Value '{value:?}' does not exist in the enumeration")
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
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub struct XhtmlOptionType {
        #[xattribute(name = "id", optional)]
        pub id: ::core::option::Option<::xmlity_ns_xsd_types::Id>,
        #[xattribute(name = "class", optional)]
        pub class: ::core::option::Option<String>,
        #[xattribute(name = "title", optional)]
        pub title: ::core::option::Option<String>,
        #[xattribute(deferred = true, optional)]
        pub lang: ::core::option::Option<
            ::std::boxed::Box<xmlity_ns_xml::attributes::Lang>,
        >,
        #[xattribute(name = "dir", optional)]
        pub dir: ::core::option::Option<xhtml_option_type_items::DirValue>,
        #[xattribute(name = "style", optional)]
        pub style: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Cdata>,
        >,
        #[xattribute(name = "selected", optional)]
        pub selected: ::core::option::Option<xhtml_option_type_items::SelectedValue>,
        #[xattribute(name = "disabled", optional)]
        pub disabled: ::core::option::Option<xhtml_option_type_items::DisabledValue>,
        #[xattribute(name = "label", optional)]
        pub label: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Text>,
        >,
        #[xattribute(name = "value", optional)]
        pub value: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Cdata>,
        >,
        pub xhtml_option_content: ::std::boxed::Box<
            crate::xhtml_schema::xhtml::groups::XhtmlOptionContent,
        >,
    }
    pub mod xhtml_p_type_items {
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
                        write!(f, "Value '{value:?}' does not exist in the enumeration")
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
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub struct XhtmlPType {
        #[xattribute(name = "id", optional)]
        pub id: ::core::option::Option<::xmlity_ns_xsd_types::Id>,
        #[xattribute(name = "class", optional)]
        pub class: ::core::option::Option<String>,
        #[xattribute(name = "title", optional)]
        pub title: ::core::option::Option<String>,
        #[xattribute(deferred = true, optional)]
        pub lang: ::core::option::Option<
            ::std::boxed::Box<xmlity_ns_xml::attributes::Lang>,
        >,
        #[xattribute(name = "dir", optional)]
        pub dir: ::core::option::Option<xhtml_p_type_items::DirValue>,
        #[xattribute(name = "style", optional)]
        pub style: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Cdata>,
        >,
        pub xhtml_p_content: ::std::boxed::Box<
            crate::xhtml_schema::xhtml::groups::XhtmlPContent,
        >,
    }
    pub mod xhtml_param_type_items {
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
                super::ValuetypeValue::try_from(value)
                    .map_err(::xmlity::de::Error::custom)
            }
            pub fn serialize<S>(
                value: &super::ValuetypeValue,
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
                        write!(f, "Value '{value:?}' does not exist in the enumeration")
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
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub struct XhtmlParamType {
        #[xattribute(name = "id", optional)]
        pub id: ::core::option::Option<::xmlity_ns_xsd_types::Id>,
        #[xattribute(name = "name")]
        pub name: ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Cdata>,
        #[xattribute(name = "value", optional)]
        pub value: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Cdata>,
        >,
        #[xattribute(name = "valuetype", optional)]
        pub valuetype: ::core::option::Option<xhtml_param_type_items::ValuetypeValue>,
        #[xattribute(name = "type", optional)]
        pub type_: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::ContentType>,
        >,
        pub xhtml_param_content: ::std::boxed::Box<
            crate::xhtml_schema::xhtml::groups::XhtmlParamContent,
        >,
    }
    pub mod xhtml_pre_type_items {
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
                        write!(f, "Value '{value:?}' does not exist in the enumeration")
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
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub struct XhtmlPreType {
        #[xattribute(deferred = true, optional)]
        pub space: ::core::option::Option<
            ::std::boxed::Box<xmlity_ns_xml::attributes::Space>,
        >,
        #[xattribute(name = "id", optional)]
        pub id: ::core::option::Option<::xmlity_ns_xsd_types::Id>,
        #[xattribute(name = "class", optional)]
        pub class: ::core::option::Option<String>,
        #[xattribute(name = "title", optional)]
        pub title: ::core::option::Option<String>,
        #[xattribute(deferred = true, optional)]
        pub lang: ::core::option::Option<
            ::std::boxed::Box<xmlity_ns_xml::attributes::Lang>,
        >,
        #[xattribute(name = "dir", optional)]
        pub dir: ::core::option::Option<xhtml_pre_type_items::DirValue>,
        #[xattribute(name = "style", optional)]
        pub style: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Cdata>,
        >,
        pub xhtml_pre_content: ::std::boxed::Box<
            crate::xhtml_schema::xhtml::groups::XhtmlPreContent,
        >,
    }
    pub mod xhtml_q_type_items {
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
                        write!(f, "Value '{value:?}' does not exist in the enumeration")
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
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub struct XhtmlQType {
        #[xattribute(name = "id", optional)]
        pub id: ::core::option::Option<::xmlity_ns_xsd_types::Id>,
        #[xattribute(name = "class", optional)]
        pub class: ::core::option::Option<String>,
        #[xattribute(name = "title", optional)]
        pub title: ::core::option::Option<String>,
        #[xattribute(deferred = true, optional)]
        pub lang: ::core::option::Option<
            ::std::boxed::Box<xmlity_ns_xml::attributes::Lang>,
        >,
        #[xattribute(name = "dir", optional)]
        pub dir: ::core::option::Option<xhtml_q_type_items::DirValue>,
        #[xattribute(name = "style", optional)]
        pub style: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Cdata>,
        >,
        #[xattribute(name = "cite", optional)]
        pub cite: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Uri>,
        >,
        pub xhtml_q_content: ::std::boxed::Box<
            crate::xhtml_schema::xhtml::groups::XhtmlQContent,
        >,
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub struct XhtmlRbType;
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub struct XhtmlRpType;
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub struct XhtmlRtType;
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub struct XhtmlRubyType {
        pub xhtml_ruby_content_simple: ::std::boxed::Box<
            crate::xhtml_schema::xhtml::groups::XhtmlRubyContentSimple,
        >,
    }
    pub mod xhtml_samp_type_items {
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
                        write!(f, "Value '{value:?}' does not exist in the enumeration")
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
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub struct XhtmlSampType {
        #[xattribute(name = "id", optional)]
        pub id: ::core::option::Option<::xmlity_ns_xsd_types::Id>,
        #[xattribute(name = "class", optional)]
        pub class: ::core::option::Option<String>,
        #[xattribute(name = "title", optional)]
        pub title: ::core::option::Option<String>,
        #[xattribute(deferred = true, optional)]
        pub lang: ::core::option::Option<
            ::std::boxed::Box<xmlity_ns_xml::attributes::Lang>,
        >,
        #[xattribute(name = "dir", optional)]
        pub dir: ::core::option::Option<xhtml_samp_type_items::DirValue>,
        #[xattribute(name = "style", optional)]
        pub style: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Cdata>,
        >,
        pub xhtml_samp_content: ::std::boxed::Box<
            crate::xhtml_schema::xhtml::groups::XhtmlSampContent,
        >,
    }
    pub mod xhtml_script_type_items {
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
                        write!(f, "Value '{value:?}' does not exist in the enumeration")
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
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub struct XhtmlScriptType {
        #[xattribute(name = "charset", optional)]
        pub charset: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Charset>,
        >,
        #[xattribute(name = "type")]
        pub type_: ::std::boxed::Box<
            crate::xhtml_schema::xhtml_datatypes::types::ContentType,
        >,
        #[xattribute(name = "src", optional)]
        pub src: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Uri>,
        >,
        #[xattribute(name = "defer", optional)]
        pub defer: ::core::option::Option<xhtml_script_type_items::DeferValue>,
        pub xhtml_script_content: ::std::boxed::Box<
            crate::xhtml_schema::xhtml::groups::XhtmlScriptContent,
        >,
    }
    pub mod xhtml_select_type_items {
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
                        write!(f, "Value '{value:?}' does not exist in the enumeration")
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
                super::MultipleValue::try_from(value)
                    .map_err(::xmlity::de::Error::custom)
            }
            pub fn serialize<S>(
                value: &super::MultipleValue,
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
                        write!(f, "Value '{value:?}' does not exist in the enumeration")
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
                super::DisabledValue::try_from(value)
                    .map_err(::xmlity::de::Error::custom)
            }
            pub fn serialize<S>(
                value: &super::DisabledValue,
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
                        write!(f, "Value '{value:?}' does not exist in the enumeration")
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
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub struct XhtmlSelectType {
        #[xattribute(name = "id", optional)]
        pub id: ::core::option::Option<::xmlity_ns_xsd_types::Id>,
        #[xattribute(name = "class", optional)]
        pub class: ::core::option::Option<String>,
        #[xattribute(name = "title", optional)]
        pub title: ::core::option::Option<String>,
        #[xattribute(deferred = true, optional)]
        pub lang: ::core::option::Option<
            ::std::boxed::Box<xmlity_ns_xml::attributes::Lang>,
        >,
        #[xattribute(name = "dir", optional)]
        pub dir: ::core::option::Option<xhtml_select_type_items::DirValue>,
        #[xattribute(name = "style", optional)]
        pub style: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Cdata>,
        >,
        #[xattribute(name = "name", optional)]
        pub name: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Cdata>,
        >,
        #[xattribute(name = "size", optional)]
        pub size: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Number>,
        >,
        #[xattribute(name = "multiple", optional)]
        pub multiple: ::core::option::Option<xhtml_select_type_items::MultipleValue>,
        #[xattribute(name = "disabled", optional)]
        pub disabled: ::core::option::Option<xhtml_select_type_items::DisabledValue>,
        #[xattribute(name = "tabindex", optional)]
        pub tabindex: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Number>,
        >,
        #[xattribute(name = "onfocus", optional)]
        pub onfocus: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Script>,
        >,
        #[xattribute(name = "onblur", optional)]
        pub onblur: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Script>,
        >,
        #[xattribute(name = "onchange", optional)]
        pub onchange: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Script>,
        >,
        pub xhtml_select_content: ::std::boxed::Box<
            crate::xhtml_schema::xhtml::groups::XhtmlSelectContent,
        >,
    }
    pub mod xhtml_span_type_items {
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
                        write!(f, "Value '{value:?}' does not exist in the enumeration")
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
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub struct XhtmlSpanType {
        #[xattribute(name = "id", optional)]
        pub id: ::core::option::Option<::xmlity_ns_xsd_types::Id>,
        #[xattribute(name = "class", optional)]
        pub class: ::core::option::Option<String>,
        #[xattribute(name = "title", optional)]
        pub title: ::core::option::Option<String>,
        #[xattribute(deferred = true, optional)]
        pub lang: ::core::option::Option<
            ::std::boxed::Box<xmlity_ns_xml::attributes::Lang>,
        >,
        #[xattribute(name = "dir", optional)]
        pub dir: ::core::option::Option<xhtml_span_type_items::DirValue>,
        #[xattribute(name = "style", optional)]
        pub style: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Cdata>,
        >,
        pub xhtml_span_content: ::std::boxed::Box<
            crate::xhtml_schema::xhtml::groups::XhtmlSpanContent,
        >,
    }
    pub mod xhtml_strong_type_items {
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
                        write!(f, "Value '{value:?}' does not exist in the enumeration")
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
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub struct XhtmlStrongType {
        #[xattribute(name = "id", optional)]
        pub id: ::core::option::Option<::xmlity_ns_xsd_types::Id>,
        #[xattribute(name = "class", optional)]
        pub class: ::core::option::Option<String>,
        #[xattribute(name = "title", optional)]
        pub title: ::core::option::Option<String>,
        #[xattribute(deferred = true, optional)]
        pub lang: ::core::option::Option<
            ::std::boxed::Box<xmlity_ns_xml::attributes::Lang>,
        >,
        #[xattribute(name = "dir", optional)]
        pub dir: ::core::option::Option<xhtml_strong_type_items::DirValue>,
        #[xattribute(name = "style", optional)]
        pub style: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Cdata>,
        >,
        pub xhtml_strong_content: ::std::boxed::Box<
            crate::xhtml_schema::xhtml::groups::XhtmlStrongContent,
        >,
    }
    pub mod xhtml_style_type_items {
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
                        write!(f, "Value '{value:?}' does not exist in the enumeration")
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
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub struct XhtmlStyleType {
        #[xattribute(name = "title", optional)]
        pub title: ::core::option::Option<String>,
        #[xattribute(deferred = true, optional)]
        pub lang: ::core::option::Option<
            ::std::boxed::Box<xmlity_ns_xml::attributes::Lang>,
        >,
        #[xattribute(name = "dir", optional)]
        pub dir: ::core::option::Option<xhtml_style_type_items::DirValue>,
        #[xattribute(name = "type")]
        pub type_: ::std::boxed::Box<
            crate::xhtml_schema::xhtml_datatypes::types::ContentType,
        >,
        #[xattribute(name = "media", optional)]
        pub media: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::MediaDesc>,
        >,
        pub xhtml_style_content: ::std::boxed::Box<
            crate::xhtml_schema::xhtml::groups::XhtmlStyleContent,
        >,
    }
    pub mod xhtml_table_type_items {
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
                        write!(f, "Value '{value:?}' does not exist in the enumeration")
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
        #[xvalue(with = frame_value_with)]
        pub enum FrameValue {
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
        pub mod frame_value_with {
            pub fn deserialize<'de, D>(
                deserializer: D,
            ) -> ::core::result::Result<super::FrameValue, D::Error>
            where
                D: ::xmlity::Deserializer<'de>,
            {
                let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                    deserializer,
                )?;
                let value: ::std::string::String = text
                    .parse()
                    .map_err(::xmlity::de::Error::custom)?;
                super::FrameValue::try_from(value).map_err(::xmlity::de::Error::custom)
            }
            pub fn serialize<S>(
                value: &super::FrameValue,
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
        pub enum FrameValueParseError {
            NonExistent { value: ::std::string::String },
        }
        impl ::core::fmt::Display for FrameValueParseError {
            fn fmt(
                &self,
                f: &mut ::core::fmt::Formatter<'_>,
            ) -> ::core::result::Result<(), ::core::fmt::Error> {
                match self {
                    FrameValueParseError::NonExistent { value } => {
                        write!(f, "Value '{value:?}' does not exist in the enumeration")
                    }
                }
            }
        }
        impl ::core::convert::TryFrom<::std::string::String> for FrameValue {
            type Error = FrameValueParseError;
            fn try_from(
                value: ::std::string::String,
            ) -> ::core::result::Result<Self, Self::Error> {
                match ::std::string::String::as_str(&value) {
                    "void" => Ok(FrameValue::Void),
                    "above" => Ok(FrameValue::Above),
                    "below" => Ok(FrameValue::Below),
                    "hsides" => Ok(FrameValue::Hsides),
                    "lhs" => Ok(FrameValue::Lhs),
                    "rhs" => Ok(FrameValue::Rhs),
                    "vsides" => Ok(FrameValue::Vsides),
                    "box" => Ok(FrameValue::Box),
                    "border" => Ok(FrameValue::Border),
                    _ => {
                        Err(FrameValueParseError::NonExistent {
                            value,
                        })
                    }
                }
            }
        }
        impl ::core::convert::From<FrameValue> for ::std::string::String {
            fn from(value: FrameValue) -> Self {
                match value {
                    FrameValue::Void => ::std::string::String::from("void"),
                    FrameValue::Above => ::std::string::String::from("above"),
                    FrameValue::Below => ::std::string::String::from("below"),
                    FrameValue::Hsides => ::std::string::String::from("hsides"),
                    FrameValue::Lhs => ::std::string::String::from("lhs"),
                    FrameValue::Rhs => ::std::string::String::from("rhs"),
                    FrameValue::Vsides => ::std::string::String::from("vsides"),
                    FrameValue::Box => ::std::string::String::from("box"),
                    FrameValue::Border => ::std::string::String::from("border"),
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
        #[xvalue(with = rules_value_with)]
        pub enum RulesValue {
            None,
            Groups,
            Rows,
            Cols,
            All,
        }
        pub mod rules_value_with {
            pub fn deserialize<'de, D>(
                deserializer: D,
            ) -> ::core::result::Result<super::RulesValue, D::Error>
            where
                D: ::xmlity::Deserializer<'de>,
            {
                let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                    deserializer,
                )?;
                let value: ::std::string::String = text
                    .parse()
                    .map_err(::xmlity::de::Error::custom)?;
                super::RulesValue::try_from(value).map_err(::xmlity::de::Error::custom)
            }
            pub fn serialize<S>(
                value: &super::RulesValue,
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
        pub enum RulesValueParseError {
            NonExistent { value: ::std::string::String },
        }
        impl ::core::fmt::Display for RulesValueParseError {
            fn fmt(
                &self,
                f: &mut ::core::fmt::Formatter<'_>,
            ) -> ::core::result::Result<(), ::core::fmt::Error> {
                match self {
                    RulesValueParseError::NonExistent { value } => {
                        write!(f, "Value '{value:?}' does not exist in the enumeration")
                    }
                }
            }
        }
        impl ::core::convert::TryFrom<::std::string::String> for RulesValue {
            type Error = RulesValueParseError;
            fn try_from(
                value: ::std::string::String,
            ) -> ::core::result::Result<Self, Self::Error> {
                match ::std::string::String::as_str(&value) {
                    "none" => Ok(RulesValue::None),
                    "groups" => Ok(RulesValue::Groups),
                    "rows" => Ok(RulesValue::Rows),
                    "cols" => Ok(RulesValue::Cols),
                    "all" => Ok(RulesValue::All),
                    _ => {
                        Err(RulesValueParseError::NonExistent {
                            value,
                        })
                    }
                }
            }
        }
        impl ::core::convert::From<RulesValue> for ::std::string::String {
            fn from(value: RulesValue) -> Self {
                match value {
                    RulesValue::None => ::std::string::String::from("none"),
                    RulesValue::Groups => ::std::string::String::from("groups"),
                    RulesValue::Rows => ::std::string::String::from("rows"),
                    RulesValue::Cols => ::std::string::String::from("cols"),
                    RulesValue::All => ::std::string::String::from("all"),
                }
            }
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub struct XhtmlTableType {
        #[xattribute(name = "id", optional)]
        pub id: ::core::option::Option<::xmlity_ns_xsd_types::Id>,
        #[xattribute(name = "class", optional)]
        pub class: ::core::option::Option<String>,
        #[xattribute(name = "title", optional)]
        pub title: ::core::option::Option<String>,
        #[xattribute(deferred = true, optional)]
        pub lang: ::core::option::Option<
            ::std::boxed::Box<xmlity_ns_xml::attributes::Lang>,
        >,
        #[xattribute(name = "dir", optional)]
        pub dir: ::core::option::Option<xhtml_table_type_items::DirValue>,
        #[xattribute(name = "style", optional)]
        pub style: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Cdata>,
        >,
        #[xattribute(name = "summary", optional)]
        pub summary: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Text>,
        >,
        #[xattribute(name = "width", optional)]
        pub width: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Length>,
        >,
        #[xattribute(name = "border", optional)]
        pub border: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Pixels>,
        >,
        #[xattribute(name = "frame", optional)]
        pub frame: ::core::option::Option<xhtml_table_type_items::FrameValue>,
        #[xattribute(name = "rules", optional)]
        pub rules: ::core::option::Option<xhtml_table_type_items::RulesValue>,
        #[xattribute(name = "cellspacing", optional)]
        pub cellspacing: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Length>,
        >,
        #[xattribute(name = "cellpadding", optional)]
        pub cellpadding: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Length>,
        >,
        pub xhtml_table_content: ::std::boxed::Box<
            crate::xhtml_schema::xhtml::groups::XhtmlTableContent,
        >,
    }
    pub mod xhtml_tbody_type_items {
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
                        write!(f, "Value '{value:?}' does not exist in the enumeration")
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
                        write!(f, "Value '{value:?}' does not exist in the enumeration")
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
                        write!(f, "Value '{value:?}' does not exist in the enumeration")
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
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub struct XhtmlTbodyType {
        #[xattribute(name = "id", optional)]
        pub id: ::core::option::Option<::xmlity_ns_xsd_types::Id>,
        #[xattribute(name = "class", optional)]
        pub class: ::core::option::Option<String>,
        #[xattribute(name = "title", optional)]
        pub title: ::core::option::Option<String>,
        #[xattribute(deferred = true, optional)]
        pub lang: ::core::option::Option<
            ::std::boxed::Box<xmlity_ns_xml::attributes::Lang>,
        >,
        #[xattribute(name = "dir", optional)]
        pub dir: ::core::option::Option<xhtml_tbody_type_items::DirValue>,
        #[xattribute(name = "style", optional)]
        pub style: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Cdata>,
        >,
        #[xattribute(name = "align", optional)]
        pub align: ::core::option::Option<xhtml_tbody_type_items::AlignValue>,
        #[xattribute(name = "char", optional)]
        pub char: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Character>,
        >,
        #[xattribute(name = "charoff", optional)]
        pub charoff: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Length>,
        >,
        #[xattribute(name = "valign", optional)]
        pub valign: ::core::option::Option<xhtml_tbody_type_items::ValignValue>,
        pub xhtml_tbody_content: ::std::boxed::Box<
            crate::xhtml_schema::xhtml::groups::XhtmlTbodyContent,
        >,
    }
    pub mod xhtml_td_type_items {
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
                        write!(f, "Value '{value:?}' does not exist in the enumeration")
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
        #[xvalue(with = scope_value_with)]
        pub enum ScopeValue {
            Row,
            Col,
            Rowgroup,
            Colgroup,
        }
        pub mod scope_value_with {
            pub fn deserialize<'de, D>(
                deserializer: D,
            ) -> ::core::result::Result<super::ScopeValue, D::Error>
            where
                D: ::xmlity::Deserializer<'de>,
            {
                let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                    deserializer,
                )?;
                let value: ::std::string::String = text
                    .parse()
                    .map_err(::xmlity::de::Error::custom)?;
                super::ScopeValue::try_from(value).map_err(::xmlity::de::Error::custom)
            }
            pub fn serialize<S>(
                value: &super::ScopeValue,
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
        pub enum ScopeValueParseError {
            NonExistent { value: ::std::string::String },
        }
        impl ::core::fmt::Display for ScopeValueParseError {
            fn fmt(
                &self,
                f: &mut ::core::fmt::Formatter<'_>,
            ) -> ::core::result::Result<(), ::core::fmt::Error> {
                match self {
                    ScopeValueParseError::NonExistent { value } => {
                        write!(f, "Value '{value:?}' does not exist in the enumeration")
                    }
                }
            }
        }
        impl ::core::convert::TryFrom<::std::string::String> for ScopeValue {
            type Error = ScopeValueParseError;
            fn try_from(
                value: ::std::string::String,
            ) -> ::core::result::Result<Self, Self::Error> {
                match ::std::string::String::as_str(&value) {
                    "row" => Ok(ScopeValue::Row),
                    "col" => Ok(ScopeValue::Col),
                    "rowgroup" => Ok(ScopeValue::Rowgroup),
                    "colgroup" => Ok(ScopeValue::Colgroup),
                    _ => {
                        Err(ScopeValueParseError::NonExistent {
                            value,
                        })
                    }
                }
            }
        }
        impl ::core::convert::From<ScopeValue> for ::std::string::String {
            fn from(value: ScopeValue) -> Self {
                match value {
                    ScopeValue::Row => ::std::string::String::from("row"),
                    ScopeValue::Col => ::std::string::String::from("col"),
                    ScopeValue::Rowgroup => ::std::string::String::from("rowgroup"),
                    ScopeValue::Colgroup => ::std::string::String::from("colgroup"),
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
                        write!(f, "Value '{value:?}' does not exist in the enumeration")
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
                        write!(f, "Value '{value:?}' does not exist in the enumeration")
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
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub struct XhtmlTdType {
        #[xattribute(name = "id", optional)]
        pub id: ::core::option::Option<::xmlity_ns_xsd_types::Id>,
        #[xattribute(name = "class", optional)]
        pub class: ::core::option::Option<String>,
        #[xattribute(name = "title", optional)]
        pub title: ::core::option::Option<String>,
        #[xattribute(deferred = true, optional)]
        pub lang: ::core::option::Option<
            ::std::boxed::Box<xmlity_ns_xml::attributes::Lang>,
        >,
        #[xattribute(name = "dir", optional)]
        pub dir: ::core::option::Option<xhtml_td_type_items::DirValue>,
        #[xattribute(name = "style", optional)]
        pub style: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Cdata>,
        >,
        #[xattribute(name = "abbr", optional)]
        pub abbr: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Text>,
        >,
        #[xattribute(name = "axis", optional)]
        pub axis: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Cdata>,
        >,
        #[xattribute(name = "headers", optional)]
        pub headers: ::core::option::Option<String>,
        #[xattribute(name = "scope", optional)]
        pub scope: ::core::option::Option<xhtml_td_type_items::ScopeValue>,
        #[xattribute(name = "rowspan", optional)]
        pub rowspan: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Number>,
        >,
        #[xattribute(name = "colspan", optional)]
        pub colspan: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Number>,
        >,
        #[xattribute(name = "align", optional)]
        pub align: ::core::option::Option<xhtml_td_type_items::AlignValue>,
        #[xattribute(name = "char", optional)]
        pub char: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Character>,
        >,
        #[xattribute(name = "charoff", optional)]
        pub charoff: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Length>,
        >,
        #[xattribute(name = "valign", optional)]
        pub valign: ::core::option::Option<xhtml_td_type_items::ValignValue>,
        pub xhtml_td_content: ::std::boxed::Box<
            crate::xhtml_schema::xhtml::groups::XhtmlTdContent,
        >,
    }
    pub mod xhtml_textarea_type_items {
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
                        write!(f, "Value '{value:?}' does not exist in the enumeration")
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
                super::DisabledValue::try_from(value)
                    .map_err(::xmlity::de::Error::custom)
            }
            pub fn serialize<S>(
                value: &super::DisabledValue,
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
                        write!(f, "Value '{value:?}' does not exist in the enumeration")
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
                super::ReadonlyValue::try_from(value)
                    .map_err(::xmlity::de::Error::custom)
            }
            pub fn serialize<S>(
                value: &super::ReadonlyValue,
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
                        write!(f, "Value '{value:?}' does not exist in the enumeration")
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
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub struct XhtmlTextareaType {
        #[xattribute(name = "id", optional)]
        pub id: ::core::option::Option<::xmlity_ns_xsd_types::Id>,
        #[xattribute(name = "class", optional)]
        pub class: ::core::option::Option<String>,
        #[xattribute(name = "title", optional)]
        pub title: ::core::option::Option<String>,
        #[xattribute(deferred = true, optional)]
        pub lang: ::core::option::Option<
            ::std::boxed::Box<xmlity_ns_xml::attributes::Lang>,
        >,
        #[xattribute(name = "dir", optional)]
        pub dir: ::core::option::Option<xhtml_textarea_type_items::DirValue>,
        #[xattribute(name = "style", optional)]
        pub style: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Cdata>,
        >,
        #[xattribute(name = "name", optional)]
        pub name: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Cdata>,
        >,
        #[xattribute(name = "rows")]
        pub rows: ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Number>,
        #[xattribute(name = "cols")]
        pub cols: ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Number>,
        #[xattribute(name = "disabled", optional)]
        pub disabled: ::core::option::Option<xhtml_textarea_type_items::DisabledValue>,
        #[xattribute(name = "readonly", optional)]
        pub readonly: ::core::option::Option<xhtml_textarea_type_items::ReadonlyValue>,
        #[xattribute(name = "tabindex", optional)]
        pub tabindex: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Number>,
        >,
        #[xattribute(name = "accesskey", optional)]
        pub accesskey: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Character>,
        >,
        #[xattribute(name = "onfocus", optional)]
        pub onfocus: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Script>,
        >,
        #[xattribute(name = "onblur", optional)]
        pub onblur: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Script>,
        >,
        #[xattribute(name = "onselect", optional)]
        pub onselect: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Script>,
        >,
        #[xattribute(name = "onchange", optional)]
        pub onchange: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Script>,
        >,
        pub xhtml_textarea_content: ::std::boxed::Box<
            crate::xhtml_schema::xhtml::groups::XhtmlTextareaContent,
        >,
    }
    pub mod xhtml_tfoot_type_items {
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
                        write!(f, "Value '{value:?}' does not exist in the enumeration")
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
                        write!(f, "Value '{value:?}' does not exist in the enumeration")
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
                        write!(f, "Value '{value:?}' does not exist in the enumeration")
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
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub struct XhtmlTfootType {
        #[xattribute(name = "id", optional)]
        pub id: ::core::option::Option<::xmlity_ns_xsd_types::Id>,
        #[xattribute(name = "class", optional)]
        pub class: ::core::option::Option<String>,
        #[xattribute(name = "title", optional)]
        pub title: ::core::option::Option<String>,
        #[xattribute(deferred = true, optional)]
        pub lang: ::core::option::Option<
            ::std::boxed::Box<xmlity_ns_xml::attributes::Lang>,
        >,
        #[xattribute(name = "dir", optional)]
        pub dir: ::core::option::Option<xhtml_tfoot_type_items::DirValue>,
        #[xattribute(name = "style", optional)]
        pub style: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Cdata>,
        >,
        #[xattribute(name = "align", optional)]
        pub align: ::core::option::Option<xhtml_tfoot_type_items::AlignValue>,
        #[xattribute(name = "char", optional)]
        pub char: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Character>,
        >,
        #[xattribute(name = "charoff", optional)]
        pub charoff: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Length>,
        >,
        #[xattribute(name = "valign", optional)]
        pub valign: ::core::option::Option<xhtml_tfoot_type_items::ValignValue>,
        pub xhtml_tfoot_content: ::std::boxed::Box<
            crate::xhtml_schema::xhtml::groups::XhtmlTfootContent,
        >,
    }
    pub mod xhtml_th_type_items {
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
                        write!(f, "Value '{value:?}' does not exist in the enumeration")
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
        #[xvalue(with = scope_value_with)]
        pub enum ScopeValue {
            Row,
            Col,
            Rowgroup,
            Colgroup,
        }
        pub mod scope_value_with {
            pub fn deserialize<'de, D>(
                deserializer: D,
            ) -> ::core::result::Result<super::ScopeValue, D::Error>
            where
                D: ::xmlity::Deserializer<'de>,
            {
                let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                    deserializer,
                )?;
                let value: ::std::string::String = text
                    .parse()
                    .map_err(::xmlity::de::Error::custom)?;
                super::ScopeValue::try_from(value).map_err(::xmlity::de::Error::custom)
            }
            pub fn serialize<S>(
                value: &super::ScopeValue,
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
        pub enum ScopeValueParseError {
            NonExistent { value: ::std::string::String },
        }
        impl ::core::fmt::Display for ScopeValueParseError {
            fn fmt(
                &self,
                f: &mut ::core::fmt::Formatter<'_>,
            ) -> ::core::result::Result<(), ::core::fmt::Error> {
                match self {
                    ScopeValueParseError::NonExistent { value } => {
                        write!(f, "Value '{value:?}' does not exist in the enumeration")
                    }
                }
            }
        }
        impl ::core::convert::TryFrom<::std::string::String> for ScopeValue {
            type Error = ScopeValueParseError;
            fn try_from(
                value: ::std::string::String,
            ) -> ::core::result::Result<Self, Self::Error> {
                match ::std::string::String::as_str(&value) {
                    "row" => Ok(ScopeValue::Row),
                    "col" => Ok(ScopeValue::Col),
                    "rowgroup" => Ok(ScopeValue::Rowgroup),
                    "colgroup" => Ok(ScopeValue::Colgroup),
                    _ => {
                        Err(ScopeValueParseError::NonExistent {
                            value,
                        })
                    }
                }
            }
        }
        impl ::core::convert::From<ScopeValue> for ::std::string::String {
            fn from(value: ScopeValue) -> Self {
                match value {
                    ScopeValue::Row => ::std::string::String::from("row"),
                    ScopeValue::Col => ::std::string::String::from("col"),
                    ScopeValue::Rowgroup => ::std::string::String::from("rowgroup"),
                    ScopeValue::Colgroup => ::std::string::String::from("colgroup"),
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
                        write!(f, "Value '{value:?}' does not exist in the enumeration")
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
                        write!(f, "Value '{value:?}' does not exist in the enumeration")
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
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub struct XhtmlThType {
        #[xattribute(name = "id", optional)]
        pub id: ::core::option::Option<::xmlity_ns_xsd_types::Id>,
        #[xattribute(name = "class", optional)]
        pub class: ::core::option::Option<String>,
        #[xattribute(name = "title", optional)]
        pub title: ::core::option::Option<String>,
        #[xattribute(deferred = true, optional)]
        pub lang: ::core::option::Option<
            ::std::boxed::Box<xmlity_ns_xml::attributes::Lang>,
        >,
        #[xattribute(name = "dir", optional)]
        pub dir: ::core::option::Option<xhtml_th_type_items::DirValue>,
        #[xattribute(name = "style", optional)]
        pub style: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Cdata>,
        >,
        #[xattribute(name = "abbr", optional)]
        pub abbr: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Text>,
        >,
        #[xattribute(name = "axis", optional)]
        pub axis: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Cdata>,
        >,
        #[xattribute(name = "headers", optional)]
        pub headers: ::core::option::Option<String>,
        #[xattribute(name = "scope", optional)]
        pub scope: ::core::option::Option<xhtml_th_type_items::ScopeValue>,
        #[xattribute(name = "rowspan", optional)]
        pub rowspan: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Number>,
        >,
        #[xattribute(name = "colspan", optional)]
        pub colspan: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Number>,
        >,
        #[xattribute(name = "align", optional)]
        pub align: ::core::option::Option<xhtml_th_type_items::AlignValue>,
        #[xattribute(name = "char", optional)]
        pub char: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Character>,
        >,
        #[xattribute(name = "charoff", optional)]
        pub charoff: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Length>,
        >,
        #[xattribute(name = "valign", optional)]
        pub valign: ::core::option::Option<xhtml_th_type_items::ValignValue>,
        pub xhtml_th_content: ::std::boxed::Box<
            crate::xhtml_schema::xhtml::groups::XhtmlThContent,
        >,
    }
    pub mod xhtml_thead_type_items {
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
                        write!(f, "Value '{value:?}' does not exist in the enumeration")
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
                        write!(f, "Value '{value:?}' does not exist in the enumeration")
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
                        write!(f, "Value '{value:?}' does not exist in the enumeration")
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
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub struct XhtmlTheadType {
        #[xattribute(name = "id", optional)]
        pub id: ::core::option::Option<::xmlity_ns_xsd_types::Id>,
        #[xattribute(name = "class", optional)]
        pub class: ::core::option::Option<String>,
        #[xattribute(name = "title", optional)]
        pub title: ::core::option::Option<String>,
        #[xattribute(deferred = true, optional)]
        pub lang: ::core::option::Option<
            ::std::boxed::Box<xmlity_ns_xml::attributes::Lang>,
        >,
        #[xattribute(name = "dir", optional)]
        pub dir: ::core::option::Option<xhtml_thead_type_items::DirValue>,
        #[xattribute(name = "style", optional)]
        pub style: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Cdata>,
        >,
        #[xattribute(name = "align", optional)]
        pub align: ::core::option::Option<xhtml_thead_type_items::AlignValue>,
        #[xattribute(name = "char", optional)]
        pub char: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Character>,
        >,
        #[xattribute(name = "charoff", optional)]
        pub charoff: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Length>,
        >,
        #[xattribute(name = "valign", optional)]
        pub valign: ::core::option::Option<xhtml_thead_type_items::ValignValue>,
        pub xhtml_thead_content: ::std::boxed::Box<
            crate::xhtml_schema::xhtml::groups::XhtmlTheadContent,
        >,
    }
    pub mod xhtml_title_type_items {
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
                        write!(f, "Value '{value:?}' does not exist in the enumeration")
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
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub struct XhtmlTitleType {
        #[xattribute(deferred = true, optional)]
        pub lang: ::core::option::Option<
            ::std::boxed::Box<xmlity_ns_xml::attributes::Lang>,
        >,
        #[xattribute(name = "dir", optional)]
        pub dir: ::core::option::Option<xhtml_title_type_items::DirValue>,
        pub xhtml_title_content: ::std::boxed::Box<
            crate::xhtml_schema::xhtml::groups::XhtmlTitleContent,
        >,
    }
    pub mod xhtml_tr_type_items {
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
                        write!(f, "Value '{value:?}' does not exist in the enumeration")
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
                        write!(f, "Value '{value:?}' does not exist in the enumeration")
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
                        write!(f, "Value '{value:?}' does not exist in the enumeration")
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
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub struct XhtmlTrType {
        #[xattribute(name = "id", optional)]
        pub id: ::core::option::Option<::xmlity_ns_xsd_types::Id>,
        #[xattribute(name = "class", optional)]
        pub class: ::core::option::Option<String>,
        #[xattribute(name = "title", optional)]
        pub title: ::core::option::Option<String>,
        #[xattribute(deferred = true, optional)]
        pub lang: ::core::option::Option<
            ::std::boxed::Box<xmlity_ns_xml::attributes::Lang>,
        >,
        #[xattribute(name = "dir", optional)]
        pub dir: ::core::option::Option<xhtml_tr_type_items::DirValue>,
        #[xattribute(name = "style", optional)]
        pub style: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Cdata>,
        >,
        #[xattribute(name = "align", optional)]
        pub align: ::core::option::Option<xhtml_tr_type_items::AlignValue>,
        #[xattribute(name = "char", optional)]
        pub char: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Character>,
        >,
        #[xattribute(name = "charoff", optional)]
        pub charoff: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Length>,
        >,
        #[xattribute(name = "valign", optional)]
        pub valign: ::core::option::Option<xhtml_tr_type_items::ValignValue>,
        pub xhtml_tr_content: ::std::boxed::Box<
            crate::xhtml_schema::xhtml::groups::XhtmlTrContent,
        >,
    }
    pub mod xhtml_ul_type_items {
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
                        write!(f, "Value '{value:?}' does not exist in the enumeration")
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
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub struct XhtmlUlType {
        #[xattribute(name = "id", optional)]
        pub id: ::core::option::Option<::xmlity_ns_xsd_types::Id>,
        #[xattribute(name = "class", optional)]
        pub class: ::core::option::Option<String>,
        #[xattribute(name = "title", optional)]
        pub title: ::core::option::Option<String>,
        #[xattribute(deferred = true, optional)]
        pub lang: ::core::option::Option<
            ::std::boxed::Box<xmlity_ns_xml::attributes::Lang>,
        >,
        #[xattribute(name = "dir", optional)]
        pub dir: ::core::option::Option<xhtml_ul_type_items::DirValue>,
        #[xattribute(name = "style", optional)]
        pub style: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Cdata>,
        >,
        pub xhtml_ul_content: ::std::boxed::Box<
            crate::xhtml_schema::xhtml::groups::XhtmlUlContent,
        >,
    }
    pub mod xhtml_var_type_items {
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
                        write!(f, "Value '{value:?}' does not exist in the enumeration")
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
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub struct XhtmlVarType {
        #[xattribute(name = "id", optional)]
        pub id: ::core::option::Option<::xmlity_ns_xsd_types::Id>,
        #[xattribute(name = "class", optional)]
        pub class: ::core::option::Option<String>,
        #[xattribute(name = "title", optional)]
        pub title: ::core::option::Option<String>,
        #[xattribute(deferred = true, optional)]
        pub lang: ::core::option::Option<
            ::std::boxed::Box<xmlity_ns_xml::attributes::Lang>,
        >,
        #[xattribute(name = "dir", optional)]
        pub dir: ::core::option::Option<xhtml_var_type_items::DirValue>,
        #[xattribute(name = "style", optional)]
        pub style: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Cdata>,
        >,
        pub xhtml_var_content: ::std::boxed::Box<
            crate::xhtml_schema::xhtml::groups::XhtmlVarContent,
        >,
    }
}
pub mod attributes {
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::SerializeAttribute,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xattribute(name = "class", namespace = "http://www.w3.org/1999/xhtml")]
    pub struct Class(pub String);
    impl ::core::convert::From<String> for Class {
        fn from(value: String) -> Self {
            Class(value)
        }
    }
    pub mod dir_items {
        #[derive(
            ::core::fmt::Debug,
            ::core::clone::Clone,
            ::core::marker::Copy,
            ::xmlity::Serialize,
            ::xmlity::Deserialize,
            ::core::cmp::PartialEq
        )]
        #[xvalue(with = dir_with)]
        pub enum Dir {
            Ltr,
            Rtl,
        }
        pub mod dir_with {
            pub fn deserialize<'de, D>(
                deserializer: D,
            ) -> ::core::result::Result<super::Dir, D::Error>
            where
                D: ::xmlity::Deserializer<'de>,
            {
                let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                    deserializer,
                )?;
                let value: ::std::string::String = text
                    .parse()
                    .map_err(::xmlity::de::Error::custom)?;
                super::Dir::try_from(value).map_err(::xmlity::de::Error::custom)
            }
            pub fn serialize<S>(
                value: &super::Dir,
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
        pub enum DirParseError {
            NonExistent { value: ::std::string::String },
        }
        impl ::core::fmt::Display for DirParseError {
            fn fmt(
                &self,
                f: &mut ::core::fmt::Formatter<'_>,
            ) -> ::core::result::Result<(), ::core::fmt::Error> {
                match self {
                    DirParseError::NonExistent { value } => {
                        write!(f, "Value '{value:?}' does not exist in the enumeration")
                    }
                }
            }
        }
        impl ::core::convert::TryFrom<::std::string::String> for Dir {
            type Error = DirParseError;
            fn try_from(
                value: ::std::string::String,
            ) -> ::core::result::Result<Self, Self::Error> {
                match ::std::string::String::as_str(&value) {
                    "ltr" => Ok(Dir::Ltr),
                    "rtl" => Ok(Dir::Rtl),
                    _ => {
                        Err(DirParseError::NonExistent {
                            value,
                        })
                    }
                }
            }
        }
        impl ::core::convert::From<Dir> for ::std::string::String {
            fn from(value: Dir) -> Self {
                match value {
                    Dir::Ltr => ::std::string::String::from("ltr"),
                    Dir::Rtl => ::std::string::String::from("rtl"),
                }
            }
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::SerializeAttribute,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xattribute(name = "dir", namespace = "http://www.w3.org/1999/xhtml")]
    pub struct Dir(pub dir_items::Dir);
    impl ::core::convert::From<dir_items::Dir> for Dir {
        fn from(value: dir_items::Dir) -> Self {
            Dir(value)
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::SerializeAttribute,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xattribute(name = "id", namespace = "http://www.w3.org/1999/xhtml")]
    pub struct Id(pub ::xmlity_ns_xsd_types::Id);
    impl ::core::convert::From<::xmlity_ns_xsd_types::Id> for Id {
        fn from(value: ::xmlity_ns_xsd_types::Id) -> Self {
            Id(value)
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::SerializeAttribute,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xattribute(name = "onclick", namespace = "http://www.w3.org/1999/xhtml")]
    pub struct Onclick(
        pub ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Script>,
    );
    impl ::core::convert::From<crate::xhtml_schema::xhtml_datatypes::types::Script>
    for Onclick {
        fn from(value: crate::xhtml_schema::xhtml_datatypes::types::Script) -> Self {
            Onclick(::std::boxed::Box::new(value))
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::SerializeAttribute,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xattribute(name = "ondblclick", namespace = "http://www.w3.org/1999/xhtml")]
    pub struct Ondblclick(
        pub ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Script>,
    );
    impl ::core::convert::From<crate::xhtml_schema::xhtml_datatypes::types::Script>
    for Ondblclick {
        fn from(value: crate::xhtml_schema::xhtml_datatypes::types::Script) -> Self {
            Ondblclick(::std::boxed::Box::new(value))
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::SerializeAttribute,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xattribute(name = "onkeydown", namespace = "http://www.w3.org/1999/xhtml")]
    pub struct Onkeydown(
        pub ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Script>,
    );
    impl ::core::convert::From<crate::xhtml_schema::xhtml_datatypes::types::Script>
    for Onkeydown {
        fn from(value: crate::xhtml_schema::xhtml_datatypes::types::Script) -> Self {
            Onkeydown(::std::boxed::Box::new(value))
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::SerializeAttribute,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xattribute(name = "onkeypress", namespace = "http://www.w3.org/1999/xhtml")]
    pub struct Onkeypress(
        pub ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Script>,
    );
    impl ::core::convert::From<crate::xhtml_schema::xhtml_datatypes::types::Script>
    for Onkeypress {
        fn from(value: crate::xhtml_schema::xhtml_datatypes::types::Script) -> Self {
            Onkeypress(::std::boxed::Box::new(value))
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::SerializeAttribute,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xattribute(name = "onkeyup", namespace = "http://www.w3.org/1999/xhtml")]
    pub struct Onkeyup(
        pub ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Script>,
    );
    impl ::core::convert::From<crate::xhtml_schema::xhtml_datatypes::types::Script>
    for Onkeyup {
        fn from(value: crate::xhtml_schema::xhtml_datatypes::types::Script) -> Self {
            Onkeyup(::std::boxed::Box::new(value))
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::SerializeAttribute,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xattribute(name = "onmousedown", namespace = "http://www.w3.org/1999/xhtml")]
    pub struct Onmousedown(
        pub ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Script>,
    );
    impl ::core::convert::From<crate::xhtml_schema::xhtml_datatypes::types::Script>
    for Onmousedown {
        fn from(value: crate::xhtml_schema::xhtml_datatypes::types::Script) -> Self {
            Onmousedown(::std::boxed::Box::new(value))
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::SerializeAttribute,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xattribute(name = "onmousemove", namespace = "http://www.w3.org/1999/xhtml")]
    pub struct Onmousemove(
        pub ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Script>,
    );
    impl ::core::convert::From<crate::xhtml_schema::xhtml_datatypes::types::Script>
    for Onmousemove {
        fn from(value: crate::xhtml_schema::xhtml_datatypes::types::Script) -> Self {
            Onmousemove(::std::boxed::Box::new(value))
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::SerializeAttribute,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xattribute(name = "onmouseout", namespace = "http://www.w3.org/1999/xhtml")]
    pub struct Onmouseout(
        pub ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Script>,
    );
    impl ::core::convert::From<crate::xhtml_schema::xhtml_datatypes::types::Script>
    for Onmouseout {
        fn from(value: crate::xhtml_schema::xhtml_datatypes::types::Script) -> Self {
            Onmouseout(::std::boxed::Box::new(value))
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::SerializeAttribute,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xattribute(name = "onmouseover", namespace = "http://www.w3.org/1999/xhtml")]
    pub struct Onmouseover(
        pub ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Script>,
    );
    impl ::core::convert::From<crate::xhtml_schema::xhtml_datatypes::types::Script>
    for Onmouseover {
        fn from(value: crate::xhtml_schema::xhtml_datatypes::types::Script) -> Self {
            Onmouseover(::std::boxed::Box::new(value))
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::SerializeAttribute,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xattribute(name = "onmouseup", namespace = "http://www.w3.org/1999/xhtml")]
    pub struct Onmouseup(
        pub ::std::boxed::Box<crate::xhtml_schema::xhtml_datatypes::types::Script>,
    );
    impl ::core::convert::From<crate::xhtml_schema::xhtml_datatypes::types::Script>
    for Onmouseup {
        fn from(value: crate::xhtml_schema::xhtml_datatypes::types::Script) -> Self {
            Onmouseup(::std::boxed::Box::new(value))
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::SerializeAttribute,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xattribute(name = "title", namespace = "http://www.w3.org/1999/xhtml")]
    pub struct Title(pub String);
    impl ::core::convert::From<String> for Title {
        fn from(value: String) -> Self {
            Title(value)
        }
    }
}
pub mod groups {
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xvalue(order = "strict")]
    pub struct XhtmlAnchorClass {
        #[xelement(name = "a", namespace = "http://www.w3.org/1999/xhtml", group)]
        pub a: ::std::boxed::Box<crate::xhtml_schema::xhtml::types::XhtmlAType>,
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub enum XhtmlBlkNoFormMix {
        XhtmlHeadingClass(
            ::std::boxed::Box<crate::xhtml_schema::xhtml::groups::XhtmlHeadingClass>,
        ),
        XhtmlListClass(
            ::std::boxed::Box<crate::xhtml_schema::xhtml::groups::XhtmlListClass>,
        ),
        XhtmlBlkStructClass(
            ::std::boxed::Box<crate::xhtml_schema::xhtml::groups::XhtmlBlkStructClass>,
        ),
        XhtmlBlkPhrasClass(
            ::std::boxed::Box<crate::xhtml_schema::xhtml::groups::XhtmlBlkPhrasClass>,
        ),
        XhtmlBlkPresClass(
            ::std::boxed::Box<crate::xhtml_schema::xhtml::groups::XhtmlBlkPresClass>,
        ),
        XhtmlTableClass(
            ::std::boxed::Box<crate::xhtml_schema::xhtml::groups::XhtmlTableClass>,
        ),
        XhtmlBlockExtra(
            ::std::boxed::Box<crate::xhtml_schema::xhtml::groups::XhtmlBlockExtra>,
        ),
        XhtmlMiscClass(
            ::std::boxed::Box<crate::xhtml_schema::xhtml::groups::XhtmlMiscClass>,
        ),
    }
    impl ::core::convert::From<crate::xhtml_schema::xhtml::groups::XhtmlHeadingClass>
    for XhtmlBlkNoFormMix {
        fn from(value: crate::xhtml_schema::xhtml::groups::XhtmlHeadingClass) -> Self {
            XhtmlBlkNoFormMix::XhtmlHeadingClass(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml_schema::xhtml::groups::XhtmlListClass>
    for XhtmlBlkNoFormMix {
        fn from(value: crate::xhtml_schema::xhtml::groups::XhtmlListClass) -> Self {
            XhtmlBlkNoFormMix::XhtmlListClass(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml_schema::xhtml::groups::XhtmlBlkStructClass>
    for XhtmlBlkNoFormMix {
        fn from(value: crate::xhtml_schema::xhtml::groups::XhtmlBlkStructClass) -> Self {
            XhtmlBlkNoFormMix::XhtmlBlkStructClass(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml_schema::xhtml::groups::XhtmlBlkPhrasClass>
    for XhtmlBlkNoFormMix {
        fn from(value: crate::xhtml_schema::xhtml::groups::XhtmlBlkPhrasClass) -> Self {
            XhtmlBlkNoFormMix::XhtmlBlkPhrasClass(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml_schema::xhtml::groups::XhtmlBlkPresClass>
    for XhtmlBlkNoFormMix {
        fn from(value: crate::xhtml_schema::xhtml::groups::XhtmlBlkPresClass) -> Self {
            XhtmlBlkNoFormMix::XhtmlBlkPresClass(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml_schema::xhtml::groups::XhtmlTableClass>
    for XhtmlBlkNoFormMix {
        fn from(value: crate::xhtml_schema::xhtml::groups::XhtmlTableClass) -> Self {
            XhtmlBlkNoFormMix::XhtmlTableClass(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml_schema::xhtml::groups::XhtmlBlockExtra>
    for XhtmlBlkNoFormMix {
        fn from(value: crate::xhtml_schema::xhtml::groups::XhtmlBlockExtra) -> Self {
            XhtmlBlkNoFormMix::XhtmlBlockExtra(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml_schema::xhtml::groups::XhtmlMiscClass>
    for XhtmlBlkNoFormMix {
        fn from(value: crate::xhtml_schema::xhtml::groups::XhtmlMiscClass) -> Self {
            XhtmlBlkNoFormMix::XhtmlMiscClass(::std::boxed::Box::new(value))
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub enum XhtmlBlkPhrasClass {
        #[xelement(
            name = "pre",
            namespace = "http://www.w3.org/1999/xhtml",
            allow_unknown_attributes = "any"
        )]
        Pre(
            #[xgroup]
            ::std::boxed::Box<crate::xhtml_schema::xhtml::types::XhtmlPreType>,
        ),
        #[xelement(
            name = "blockquote",
            namespace = "http://www.w3.org/1999/xhtml",
            allow_unknown_attributes = "any"
        )]
        Blockquote(
            #[xgroup]
            ::std::boxed::Box<crate::xhtml_schema::xhtml::types::XhtmlBlockquoteType>,
        ),
        #[xelement(
            name = "address",
            namespace = "http://www.w3.org/1999/xhtml",
            allow_unknown_attributes = "any"
        )]
        Address(
            #[xgroup]
            ::std::boxed::Box<crate::xhtml_schema::xhtml::types::XhtmlAddressType>,
        ),
    }
    impl ::core::convert::From<crate::xhtml_schema::xhtml::types::XhtmlPreType>
    for XhtmlBlkPhrasClass {
        fn from(value: crate::xhtml_schema::xhtml::types::XhtmlPreType) -> Self {
            XhtmlBlkPhrasClass::Pre(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml_schema::xhtml::types::XhtmlBlockquoteType>
    for XhtmlBlkPhrasClass {
        fn from(value: crate::xhtml_schema::xhtml::types::XhtmlBlockquoteType) -> Self {
            XhtmlBlkPhrasClass::Blockquote(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml_schema::xhtml::types::XhtmlAddressType>
    for XhtmlBlkPhrasClass {
        fn from(value: crate::xhtml_schema::xhtml::types::XhtmlAddressType) -> Self {
            XhtmlBlkPhrasClass::Address(::std::boxed::Box::new(value))
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
    pub struct XhtmlBlkPresClass {
        #[xelement(name = "hr", namespace = "http://www.w3.org/1999/xhtml", group)]
        pub hr: ::std::boxed::Box<crate::xhtml_schema::xhtml::types::XhtmlHrType>,
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub enum XhtmlBlkSpecialClass {
        XhtmlTableClass(
            ::std::boxed::Box<crate::xhtml_schema::xhtml::groups::XhtmlTableClass>,
        ),
        XhtmlFormClass(
            ::std::boxed::Box<crate::xhtml_schema::xhtml::groups::XhtmlFormClass>,
        ),
        XhtmlFieldsetClass(
            ::std::boxed::Box<crate::xhtml_schema::xhtml::groups::XhtmlFieldsetClass>,
        ),
    }
    impl ::core::convert::From<crate::xhtml_schema::xhtml::groups::XhtmlTableClass>
    for XhtmlBlkSpecialClass {
        fn from(value: crate::xhtml_schema::xhtml::groups::XhtmlTableClass) -> Self {
            XhtmlBlkSpecialClass::XhtmlTableClass(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml_schema::xhtml::groups::XhtmlFormClass>
    for XhtmlBlkSpecialClass {
        fn from(value: crate::xhtml_schema::xhtml::groups::XhtmlFormClass) -> Self {
            XhtmlBlkSpecialClass::XhtmlFormClass(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml_schema::xhtml::groups::XhtmlFieldsetClass>
    for XhtmlBlkSpecialClass {
        fn from(value: crate::xhtml_schema::xhtml::groups::XhtmlFieldsetClass) -> Self {
            XhtmlBlkSpecialClass::XhtmlFieldsetClass(::std::boxed::Box::new(value))
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub enum XhtmlBlkStructClass {
        #[xelement(
            name = "p",
            namespace = "http://www.w3.org/1999/xhtml",
            allow_unknown_attributes = "any"
        )]
        P(#[xgroup] ::std::boxed::Box<crate::xhtml_schema::xhtml::types::XhtmlPType>),
        #[xelement(
            name = "div",
            namespace = "http://www.w3.org/1999/xhtml",
            allow_unknown_attributes = "any"
        )]
        Div(
            #[xgroup]
            ::std::boxed::Box<crate::xhtml_schema::xhtml::types::XhtmlDivType>,
        ),
    }
    impl ::core::convert::From<crate::xhtml_schema::xhtml::types::XhtmlPType>
    for XhtmlBlkStructClass {
        fn from(value: crate::xhtml_schema::xhtml::types::XhtmlPType) -> Self {
            XhtmlBlkStructClass::P(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml_schema::xhtml::types::XhtmlDivType>
    for XhtmlBlkStructClass {
        fn from(value: crate::xhtml_schema::xhtml::types::XhtmlDivType) -> Self {
            XhtmlBlkStructClass::Div(::std::boxed::Box::new(value))
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub enum XhtmlBlockClass {
        XhtmlBlkStructClass(
            ::std::boxed::Box<crate::xhtml_schema::xhtml::groups::XhtmlBlkStructClass>,
        ),
        XhtmlBlkPhrasClass(
            ::std::boxed::Box<crate::xhtml_schema::xhtml::groups::XhtmlBlkPhrasClass>,
        ),
        XhtmlBlkPresClass(
            ::std::boxed::Box<crate::xhtml_schema::xhtml::groups::XhtmlBlkPresClass>,
        ),
        XhtmlBlkSpecialClass(
            ::std::boxed::Box<crate::xhtml_schema::xhtml::groups::XhtmlBlkSpecialClass>,
        ),
        XhtmlBlockExtra(
            ::std::boxed::Box<crate::xhtml_schema::xhtml::groups::XhtmlBlockExtra>,
        ),
    }
    impl ::core::convert::From<crate::xhtml_schema::xhtml::groups::XhtmlBlkStructClass>
    for XhtmlBlockClass {
        fn from(value: crate::xhtml_schema::xhtml::groups::XhtmlBlkStructClass) -> Self {
            XhtmlBlockClass::XhtmlBlkStructClass(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml_schema::xhtml::groups::XhtmlBlkPhrasClass>
    for XhtmlBlockClass {
        fn from(value: crate::xhtml_schema::xhtml::groups::XhtmlBlkPhrasClass) -> Self {
            XhtmlBlockClass::XhtmlBlkPhrasClass(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml_schema::xhtml::groups::XhtmlBlkPresClass>
    for XhtmlBlockClass {
        fn from(value: crate::xhtml_schema::xhtml::groups::XhtmlBlkPresClass) -> Self {
            XhtmlBlockClass::XhtmlBlkPresClass(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml_schema::xhtml::groups::XhtmlBlkSpecialClass>
    for XhtmlBlockClass {
        fn from(
            value: crate::xhtml_schema::xhtml::groups::XhtmlBlkSpecialClass,
        ) -> Self {
            XhtmlBlockClass::XhtmlBlkSpecialClass(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml_schema::xhtml::groups::XhtmlBlockExtra>
    for XhtmlBlockClass {
        fn from(value: crate::xhtml_schema::xhtml::groups::XhtmlBlockExtra) -> Self {
            XhtmlBlockClass::XhtmlBlockExtra(::std::boxed::Box::new(value))
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
    pub struct XhtmlBlockExtra;
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub enum XhtmlBlockMix {
        XhtmlHeadingClass(
            ::std::boxed::Box<crate::xhtml_schema::xhtml::groups::XhtmlHeadingClass>,
        ),
        XhtmlListClass(
            ::std::boxed::Box<crate::xhtml_schema::xhtml::groups::XhtmlListClass>,
        ),
        XhtmlBlockClass(
            ::std::boxed::Box<crate::xhtml_schema::xhtml::groups::XhtmlBlockClass>,
        ),
        XhtmlMiscClass(
            ::std::boxed::Box<crate::xhtml_schema::xhtml::groups::XhtmlMiscClass>,
        ),
    }
    impl ::core::convert::From<crate::xhtml_schema::xhtml::groups::XhtmlHeadingClass>
    for XhtmlBlockMix {
        fn from(value: crate::xhtml_schema::xhtml::groups::XhtmlHeadingClass) -> Self {
            XhtmlBlockMix::XhtmlHeadingClass(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml_schema::xhtml::groups::XhtmlListClass>
    for XhtmlBlockMix {
        fn from(value: crate::xhtml_schema::xhtml::groups::XhtmlListClass) -> Self {
            XhtmlBlockMix::XhtmlListClass(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml_schema::xhtml::groups::XhtmlBlockClass>
    for XhtmlBlockMix {
        fn from(value: crate::xhtml_schema::xhtml::groups::XhtmlBlockClass) -> Self {
            XhtmlBlockMix::XhtmlBlockClass(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml_schema::xhtml::groups::XhtmlMiscClass>
    for XhtmlBlockMix {
        fn from(value: crate::xhtml_schema::xhtml::groups::XhtmlMiscClass) -> Self {
            XhtmlBlockMix::XhtmlMiscClass(::std::boxed::Box::new(value))
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub enum XhtmlEditClass {
        #[xelement(
            name = "ins",
            namespace = "http://www.w3.org/1999/xhtml",
            allow_unknown_attributes = "any"
        )]
        Ins(
            #[xgroup]
            ::std::boxed::Box<crate::xhtml_schema::xhtml::types::XhtmlEditType>,
        ),
        #[xelement(
            name = "del",
            namespace = "http://www.w3.org/1999/xhtml",
            allow_unknown_attributes = "any"
        )]
        Del(
            #[xgroup]
            ::std::boxed::Box<crate::xhtml_schema::xhtml::types::XhtmlEditType>,
        ),
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub enum XhtmlFieldsetClass {
        #[xelement(
            name = "fieldset",
            namespace = "http://www.w3.org/1999/xhtml",
            allow_unknown_attributes = "any"
        )]
        Fieldset(
            #[xgroup]
            ::std::boxed::Box<crate::xhtml_schema::xhtml::types::XhtmlFieldsetType>,
        ),
    }
    impl ::core::convert::From<crate::xhtml_schema::xhtml::types::XhtmlFieldsetType>
    for XhtmlFieldsetClass {
        fn from(value: crate::xhtml_schema::xhtml::types::XhtmlFieldsetType) -> Self {
            XhtmlFieldsetClass::Fieldset(::std::boxed::Box::new(value))
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub enum XhtmlFlowMix {
        XhtmlHeadingClass(
            ::std::boxed::Box<crate::xhtml_schema::xhtml::groups::XhtmlHeadingClass>,
        ),
        XhtmlListClass(
            ::std::boxed::Box<crate::xhtml_schema::xhtml::groups::XhtmlListClass>,
        ),
        XhtmlBlockClass(
            ::std::boxed::Box<crate::xhtml_schema::xhtml::groups::XhtmlBlockClass>,
        ),
        XhtmlInlineClass(
            ::std::boxed::Box<crate::xhtml_schema::xhtml::groups::XhtmlInlineClass>,
        ),
        XhtmlMiscClass(
            ::std::boxed::Box<crate::xhtml_schema::xhtml::groups::XhtmlMiscClass>,
        ),
    }
    impl ::core::convert::From<crate::xhtml_schema::xhtml::groups::XhtmlHeadingClass>
    for XhtmlFlowMix {
        fn from(value: crate::xhtml_schema::xhtml::groups::XhtmlHeadingClass) -> Self {
            XhtmlFlowMix::XhtmlHeadingClass(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml_schema::xhtml::groups::XhtmlListClass>
    for XhtmlFlowMix {
        fn from(value: crate::xhtml_schema::xhtml::groups::XhtmlListClass) -> Self {
            XhtmlFlowMix::XhtmlListClass(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml_schema::xhtml::groups::XhtmlBlockClass>
    for XhtmlFlowMix {
        fn from(value: crate::xhtml_schema::xhtml::groups::XhtmlBlockClass) -> Self {
            XhtmlFlowMix::XhtmlBlockClass(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml_schema::xhtml::groups::XhtmlInlineClass>
    for XhtmlFlowMix {
        fn from(value: crate::xhtml_schema::xhtml::groups::XhtmlInlineClass) -> Self {
            XhtmlFlowMix::XhtmlInlineClass(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml_schema::xhtml::groups::XhtmlMiscClass>
    for XhtmlFlowMix {
        fn from(value: crate::xhtml_schema::xhtml::groups::XhtmlMiscClass) -> Self {
            XhtmlFlowMix::XhtmlMiscClass(::std::boxed::Box::new(value))
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub enum XhtmlFormClass {
        #[xelement(
            name = "form",
            namespace = "http://www.w3.org/1999/xhtml",
            allow_unknown_attributes = "any"
        )]
        Form(
            #[xgroup]
            ::std::boxed::Box<crate::xhtml_schema::xhtml::types::XhtmlFormType>,
        ),
    }
    impl ::core::convert::From<crate::xhtml_schema::xhtml::types::XhtmlFormType>
    for XhtmlFormClass {
        fn from(value: crate::xhtml_schema::xhtml::types::XhtmlFormType) -> Self {
            XhtmlFormClass::Form(::std::boxed::Box::new(value))
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
    pub struct XhtmlHeadExtra;
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub enum XhtmlHeadOptsMix {
        #[xelement(
            name = "script",
            namespace = "http://www.w3.org/1999/xhtml",
            allow_unknown_attributes = "any"
        )]
        Script(
            #[xgroup]
            ::std::boxed::Box<crate::xhtml_schema::xhtml::types::XhtmlScriptType>,
        ),
        #[xelement(
            name = "style",
            namespace = "http://www.w3.org/1999/xhtml",
            allow_unknown_attributes = "any"
        )]
        Style(
            #[xgroup]
            ::std::boxed::Box<crate::xhtml_schema::xhtml::types::XhtmlStyleType>,
        ),
        #[xelement(
            name = "meta",
            namespace = "http://www.w3.org/1999/xhtml",
            allow_unknown_attributes = "any"
        )]
        Meta(
            #[xgroup]
            ::std::boxed::Box<crate::xhtml_schema::xhtml::types::XhtmlMetaType>,
        ),
        #[xelement(
            name = "link",
            namespace = "http://www.w3.org/1999/xhtml",
            allow_unknown_attributes = "any"
        )]
        Link(
            #[xgroup]
            ::std::boxed::Box<crate::xhtml_schema::xhtml::types::XhtmlLinkType>,
        ),
        #[xelement(
            name = "object",
            namespace = "http://www.w3.org/1999/xhtml",
            allow_unknown_attributes = "any"
        )]
        Object(
            #[xgroup]
            ::std::boxed::Box<crate::xhtml_schema::xhtml::types::XhtmlObjectType>,
        ),
        XhtmlHeadExtra(
            ::std::boxed::Box<crate::xhtml_schema::xhtml::groups::XhtmlHeadExtra>,
        ),
    }
    impl ::core::convert::From<crate::xhtml_schema::xhtml::types::XhtmlScriptType>
    for XhtmlHeadOptsMix {
        fn from(value: crate::xhtml_schema::xhtml::types::XhtmlScriptType) -> Self {
            XhtmlHeadOptsMix::Script(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml_schema::xhtml::types::XhtmlStyleType>
    for XhtmlHeadOptsMix {
        fn from(value: crate::xhtml_schema::xhtml::types::XhtmlStyleType) -> Self {
            XhtmlHeadOptsMix::Style(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml_schema::xhtml::types::XhtmlMetaType>
    for XhtmlHeadOptsMix {
        fn from(value: crate::xhtml_schema::xhtml::types::XhtmlMetaType) -> Self {
            XhtmlHeadOptsMix::Meta(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml_schema::xhtml::types::XhtmlLinkType>
    for XhtmlHeadOptsMix {
        fn from(value: crate::xhtml_schema::xhtml::types::XhtmlLinkType) -> Self {
            XhtmlHeadOptsMix::Link(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml_schema::xhtml::types::XhtmlObjectType>
    for XhtmlHeadOptsMix {
        fn from(value: crate::xhtml_schema::xhtml::types::XhtmlObjectType) -> Self {
            XhtmlHeadOptsMix::Object(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml_schema::xhtml::groups::XhtmlHeadExtra>
    for XhtmlHeadOptsMix {
        fn from(value: crate::xhtml_schema::xhtml::groups::XhtmlHeadExtra) -> Self {
            XhtmlHeadOptsMix::XhtmlHeadExtra(::std::boxed::Box::new(value))
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub enum XhtmlHeadingClass {
        #[xelement(
            name = "h1",
            namespace = "http://www.w3.org/1999/xhtml",
            allow_unknown_attributes = "any"
        )]
        H1(#[xgroup] ::std::boxed::Box<crate::xhtml_schema::xhtml::types::XhtmlH1Type>),
        #[xelement(
            name = "h2",
            namespace = "http://www.w3.org/1999/xhtml",
            allow_unknown_attributes = "any"
        )]
        H2(#[xgroup] ::std::boxed::Box<crate::xhtml_schema::xhtml::types::XhtmlH2Type>),
        #[xelement(
            name = "h3",
            namespace = "http://www.w3.org/1999/xhtml",
            allow_unknown_attributes = "any"
        )]
        H3(#[xgroup] ::std::boxed::Box<crate::xhtml_schema::xhtml::types::XhtmlH3Type>),
        #[xelement(
            name = "h4",
            namespace = "http://www.w3.org/1999/xhtml",
            allow_unknown_attributes = "any"
        )]
        H4(#[xgroup] ::std::boxed::Box<crate::xhtml_schema::xhtml::types::XhtmlH4Type>),
        #[xelement(
            name = "h5",
            namespace = "http://www.w3.org/1999/xhtml",
            allow_unknown_attributes = "any"
        )]
        H5(#[xgroup] ::std::boxed::Box<crate::xhtml_schema::xhtml::types::XhtmlH5Type>),
        #[xelement(
            name = "h6",
            namespace = "http://www.w3.org/1999/xhtml",
            allow_unknown_attributes = "any"
        )]
        H6(#[xgroup] ::std::boxed::Box<crate::xhtml_schema::xhtml::types::XhtmlH6Type>),
    }
    impl ::core::convert::From<crate::xhtml_schema::xhtml::types::XhtmlH1Type>
    for XhtmlHeadingClass {
        fn from(value: crate::xhtml_schema::xhtml::types::XhtmlH1Type) -> Self {
            XhtmlHeadingClass::H1(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml_schema::xhtml::types::XhtmlH2Type>
    for XhtmlHeadingClass {
        fn from(value: crate::xhtml_schema::xhtml::types::XhtmlH2Type) -> Self {
            XhtmlHeadingClass::H2(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml_schema::xhtml::types::XhtmlH3Type>
    for XhtmlHeadingClass {
        fn from(value: crate::xhtml_schema::xhtml::types::XhtmlH3Type) -> Self {
            XhtmlHeadingClass::H3(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml_schema::xhtml::types::XhtmlH4Type>
    for XhtmlHeadingClass {
        fn from(value: crate::xhtml_schema::xhtml::types::XhtmlH4Type) -> Self {
            XhtmlHeadingClass::H4(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml_schema::xhtml::types::XhtmlH5Type>
    for XhtmlHeadingClass {
        fn from(value: crate::xhtml_schema::xhtml::types::XhtmlH5Type) -> Self {
            XhtmlHeadingClass::H5(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml_schema::xhtml::types::XhtmlH6Type>
    for XhtmlHeadingClass {
        fn from(value: crate::xhtml_schema::xhtml::types::XhtmlH6Type) -> Self {
            XhtmlHeadingClass::H6(::std::boxed::Box::new(value))
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
    pub struct XhtmlI18NClass {
        #[xelement(name = "bdo", namespace = "http://www.w3.org/1999/xhtml", group)]
        pub bdo: ::std::boxed::Box<crate::xhtml_schema::xhtml::types::XhtmlBdoType>,
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub enum XhtmlInlFormClass {
        #[xelement(
            name = "input",
            namespace = "http://www.w3.org/1999/xhtml",
            allow_unknown_attributes = "any"
        )]
        Input(
            #[xgroup]
            ::std::boxed::Box<crate::xhtml_schema::xhtml::types::XhtmlInputType>,
        ),
        #[xelement(
            name = "select",
            namespace = "http://www.w3.org/1999/xhtml",
            allow_unknown_attributes = "any"
        )]
        Select(
            #[xgroup]
            ::std::boxed::Box<crate::xhtml_schema::xhtml::types::XhtmlSelectType>,
        ),
        #[xelement(
            name = "textarea",
            namespace = "http://www.w3.org/1999/xhtml",
            allow_unknown_attributes = "any"
        )]
        Textarea(
            #[xgroup]
            ::std::boxed::Box<crate::xhtml_schema::xhtml::types::XhtmlTextareaType>,
        ),
        #[xelement(
            name = "label",
            namespace = "http://www.w3.org/1999/xhtml",
            allow_unknown_attributes = "any"
        )]
        Label(
            #[xgroup]
            ::std::boxed::Box<crate::xhtml_schema::xhtml::types::XhtmlLabelType>,
        ),
        #[xelement(
            name = "button",
            namespace = "http://www.w3.org/1999/xhtml",
            allow_unknown_attributes = "any"
        )]
        Button(
            #[xgroup]
            ::std::boxed::Box<crate::xhtml_schema::xhtml::types::XhtmlButtonType>,
        ),
    }
    impl ::core::convert::From<crate::xhtml_schema::xhtml::types::XhtmlInputType>
    for XhtmlInlFormClass {
        fn from(value: crate::xhtml_schema::xhtml::types::XhtmlInputType) -> Self {
            XhtmlInlFormClass::Input(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml_schema::xhtml::types::XhtmlSelectType>
    for XhtmlInlFormClass {
        fn from(value: crate::xhtml_schema::xhtml::types::XhtmlSelectType) -> Self {
            XhtmlInlFormClass::Select(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml_schema::xhtml::types::XhtmlTextareaType>
    for XhtmlInlFormClass {
        fn from(value: crate::xhtml_schema::xhtml::types::XhtmlTextareaType) -> Self {
            XhtmlInlFormClass::Textarea(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml_schema::xhtml::types::XhtmlLabelType>
    for XhtmlInlFormClass {
        fn from(value: crate::xhtml_schema::xhtml::types::XhtmlLabelType) -> Self {
            XhtmlInlFormClass::Label(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml_schema::xhtml::types::XhtmlButtonType>
    for XhtmlInlFormClass {
        fn from(value: crate::xhtml_schema::xhtml::types::XhtmlButtonType) -> Self {
            XhtmlInlFormClass::Button(::std::boxed::Box::new(value))
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub enum XhtmlInlNoAnchorClass {
        XhtmlInlStructClass(
            ::std::boxed::Box<crate::xhtml_schema::xhtml::groups::XhtmlInlStructClass>,
        ),
        XhtmlInlPhrasClass(
            ::std::boxed::Box<crate::xhtml_schema::xhtml::groups::XhtmlInlPhrasClass>,
        ),
        XhtmlInlPresClass(
            ::std::boxed::Box<crate::xhtml_schema::xhtml::groups::XhtmlInlPresClass>,
        ),
        XhtmlI18NClass(
            ::std::boxed::Box<crate::xhtml_schema::xhtml::groups::XhtmlI18NClass>,
        ),
        XhtmlInlSpecialClass(
            ::std::boxed::Box<crate::xhtml_schema::xhtml::groups::XhtmlInlSpecialClass>,
        ),
        XhtmlInlFormClass(
            ::std::boxed::Box<crate::xhtml_schema::xhtml::groups::XhtmlInlFormClass>,
        ),
        XhtmlRubyClass(
            ::std::boxed::Box<crate::xhtml_schema::xhtml::groups::XhtmlRubyClass>,
        ),
        XhtmlInlineExtra(
            ::std::boxed::Box<crate::xhtml_schema::xhtml::groups::XhtmlInlineExtra>,
        ),
    }
    impl ::core::convert::From<crate::xhtml_schema::xhtml::groups::XhtmlInlStructClass>
    for XhtmlInlNoAnchorClass {
        fn from(value: crate::xhtml_schema::xhtml::groups::XhtmlInlStructClass) -> Self {
            XhtmlInlNoAnchorClass::XhtmlInlStructClass(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml_schema::xhtml::groups::XhtmlInlPhrasClass>
    for XhtmlInlNoAnchorClass {
        fn from(value: crate::xhtml_schema::xhtml::groups::XhtmlInlPhrasClass) -> Self {
            XhtmlInlNoAnchorClass::XhtmlInlPhrasClass(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml_schema::xhtml::groups::XhtmlInlPresClass>
    for XhtmlInlNoAnchorClass {
        fn from(value: crate::xhtml_schema::xhtml::groups::XhtmlInlPresClass) -> Self {
            XhtmlInlNoAnchorClass::XhtmlInlPresClass(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml_schema::xhtml::groups::XhtmlI18NClass>
    for XhtmlInlNoAnchorClass {
        fn from(value: crate::xhtml_schema::xhtml::groups::XhtmlI18NClass) -> Self {
            XhtmlInlNoAnchorClass::XhtmlI18NClass(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml_schema::xhtml::groups::XhtmlInlSpecialClass>
    for XhtmlInlNoAnchorClass {
        fn from(
            value: crate::xhtml_schema::xhtml::groups::XhtmlInlSpecialClass,
        ) -> Self {
            XhtmlInlNoAnchorClass::XhtmlInlSpecialClass(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml_schema::xhtml::groups::XhtmlInlFormClass>
    for XhtmlInlNoAnchorClass {
        fn from(value: crate::xhtml_schema::xhtml::groups::XhtmlInlFormClass) -> Self {
            XhtmlInlNoAnchorClass::XhtmlInlFormClass(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml_schema::xhtml::groups::XhtmlRubyClass>
    for XhtmlInlNoAnchorClass {
        fn from(value: crate::xhtml_schema::xhtml::groups::XhtmlRubyClass) -> Self {
            XhtmlInlNoAnchorClass::XhtmlRubyClass(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml_schema::xhtml::groups::XhtmlInlineExtra>
    for XhtmlInlNoAnchorClass {
        fn from(value: crate::xhtml_schema::xhtml::groups::XhtmlInlineExtra) -> Self {
            XhtmlInlNoAnchorClass::XhtmlInlineExtra(::std::boxed::Box::new(value))
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub enum XhtmlInlNoAnchorMix {
        XhtmlInlNoAnchorClass(
            ::std::boxed::Box<crate::xhtml_schema::xhtml::groups::XhtmlInlNoAnchorClass>,
        ),
        XhtmlMiscClass(
            ::std::boxed::Box<crate::xhtml_schema::xhtml::groups::XhtmlMiscClass>,
        ),
    }
    impl ::core::convert::From<crate::xhtml_schema::xhtml::groups::XhtmlInlNoAnchorClass>
    for XhtmlInlNoAnchorMix {
        fn from(
            value: crate::xhtml_schema::xhtml::groups::XhtmlInlNoAnchorClass,
        ) -> Self {
            XhtmlInlNoAnchorMix::XhtmlInlNoAnchorClass(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml_schema::xhtml::groups::XhtmlMiscClass>
    for XhtmlInlNoAnchorMix {
        fn from(value: crate::xhtml_schema::xhtml::groups::XhtmlMiscClass) -> Self {
            XhtmlInlNoAnchorMix::XhtmlMiscClass(::std::boxed::Box::new(value))
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub enum XhtmlInlNoRubyClass {
        XhtmlInlStructClass(
            ::std::boxed::Box<crate::xhtml_schema::xhtml::groups::XhtmlInlStructClass>,
        ),
        XhtmlInlPhrasClass(
            ::std::boxed::Box<crate::xhtml_schema::xhtml::groups::XhtmlInlPhrasClass>,
        ),
        XhtmlInlPresClass(
            ::std::boxed::Box<crate::xhtml_schema::xhtml::groups::XhtmlInlPresClass>,
        ),
        XhtmlI18NClass(
            ::std::boxed::Box<crate::xhtml_schema::xhtml::groups::XhtmlI18NClass>,
        ),
        XhtmlAnchorClass(
            ::std::boxed::Box<crate::xhtml_schema::xhtml::groups::XhtmlAnchorClass>,
        ),
        XhtmlInlSpecialClass(
            ::std::boxed::Box<crate::xhtml_schema::xhtml::groups::XhtmlInlSpecialClass>,
        ),
        XhtmlInlFormClass(
            ::std::boxed::Box<crate::xhtml_schema::xhtml::groups::XhtmlInlFormClass>,
        ),
        XhtmlInlineExtra(
            ::std::boxed::Box<crate::xhtml_schema::xhtml::groups::XhtmlInlineExtra>,
        ),
    }
    impl ::core::convert::From<crate::xhtml_schema::xhtml::groups::XhtmlInlStructClass>
    for XhtmlInlNoRubyClass {
        fn from(value: crate::xhtml_schema::xhtml::groups::XhtmlInlStructClass) -> Self {
            XhtmlInlNoRubyClass::XhtmlInlStructClass(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml_schema::xhtml::groups::XhtmlInlPhrasClass>
    for XhtmlInlNoRubyClass {
        fn from(value: crate::xhtml_schema::xhtml::groups::XhtmlInlPhrasClass) -> Self {
            XhtmlInlNoRubyClass::XhtmlInlPhrasClass(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml_schema::xhtml::groups::XhtmlInlPresClass>
    for XhtmlInlNoRubyClass {
        fn from(value: crate::xhtml_schema::xhtml::groups::XhtmlInlPresClass) -> Self {
            XhtmlInlNoRubyClass::XhtmlInlPresClass(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml_schema::xhtml::groups::XhtmlI18NClass>
    for XhtmlInlNoRubyClass {
        fn from(value: crate::xhtml_schema::xhtml::groups::XhtmlI18NClass) -> Self {
            XhtmlInlNoRubyClass::XhtmlI18NClass(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml_schema::xhtml::groups::XhtmlAnchorClass>
    for XhtmlInlNoRubyClass {
        fn from(value: crate::xhtml_schema::xhtml::groups::XhtmlAnchorClass) -> Self {
            XhtmlInlNoRubyClass::XhtmlAnchorClass(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml_schema::xhtml::groups::XhtmlInlSpecialClass>
    for XhtmlInlNoRubyClass {
        fn from(
            value: crate::xhtml_schema::xhtml::groups::XhtmlInlSpecialClass,
        ) -> Self {
            XhtmlInlNoRubyClass::XhtmlInlSpecialClass(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml_schema::xhtml::groups::XhtmlInlFormClass>
    for XhtmlInlNoRubyClass {
        fn from(value: crate::xhtml_schema::xhtml::groups::XhtmlInlFormClass) -> Self {
            XhtmlInlNoRubyClass::XhtmlInlFormClass(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml_schema::xhtml::groups::XhtmlInlineExtra>
    for XhtmlInlNoRubyClass {
        fn from(value: crate::xhtml_schema::xhtml::groups::XhtmlInlineExtra) -> Self {
            XhtmlInlNoRubyClass::XhtmlInlineExtra(::std::boxed::Box::new(value))
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub enum XhtmlInlNoRubyMix {
        XhtmlInlNoRubyClass(
            ::std::boxed::Box<crate::xhtml_schema::xhtml::groups::XhtmlInlNoRubyClass>,
        ),
        XhtmlMiscClass(
            ::std::boxed::Box<crate::xhtml_schema::xhtml::groups::XhtmlMiscClass>,
        ),
    }
    impl ::core::convert::From<crate::xhtml_schema::xhtml::groups::XhtmlInlNoRubyClass>
    for XhtmlInlNoRubyMix {
        fn from(value: crate::xhtml_schema::xhtml::groups::XhtmlInlNoRubyClass) -> Self {
            XhtmlInlNoRubyMix::XhtmlInlNoRubyClass(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml_schema::xhtml::groups::XhtmlMiscClass>
    for XhtmlInlNoRubyMix {
        fn from(value: crate::xhtml_schema::xhtml::groups::XhtmlMiscClass) -> Self {
            XhtmlInlNoRubyMix::XhtmlMiscClass(::std::boxed::Box::new(value))
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub enum XhtmlInlPhrasClass {
        #[xelement(
            name = "em",
            namespace = "http://www.w3.org/1999/xhtml",
            allow_unknown_attributes = "any"
        )]
        Em(#[xgroup] ::std::boxed::Box<crate::xhtml_schema::xhtml::types::XhtmlEmType>),
        #[xelement(
            name = "strong",
            namespace = "http://www.w3.org/1999/xhtml",
            allow_unknown_attributes = "any"
        )]
        Strong(
            #[xgroup]
            ::std::boxed::Box<crate::xhtml_schema::xhtml::types::XhtmlStrongType>,
        ),
        #[xelement(
            name = "dfn",
            namespace = "http://www.w3.org/1999/xhtml",
            allow_unknown_attributes = "any"
        )]
        Dfn(
            #[xgroup]
            ::std::boxed::Box<crate::xhtml_schema::xhtml::types::XhtmlDfnType>,
        ),
        #[xelement(
            name = "code",
            namespace = "http://www.w3.org/1999/xhtml",
            allow_unknown_attributes = "any"
        )]
        Code(
            #[xgroup]
            ::std::boxed::Box<crate::xhtml_schema::xhtml::types::XhtmlCodeType>,
        ),
        #[xelement(
            name = "samp",
            namespace = "http://www.w3.org/1999/xhtml",
            allow_unknown_attributes = "any"
        )]
        Samp(
            #[xgroup]
            ::std::boxed::Box<crate::xhtml_schema::xhtml::types::XhtmlSampType>,
        ),
        #[xelement(
            name = "kbd",
            namespace = "http://www.w3.org/1999/xhtml",
            allow_unknown_attributes = "any"
        )]
        Kbd(
            #[xgroup]
            ::std::boxed::Box<crate::xhtml_schema::xhtml::types::XhtmlKbdType>,
        ),
        #[xelement(
            name = "var",
            namespace = "http://www.w3.org/1999/xhtml",
            allow_unknown_attributes = "any"
        )]
        Var(
            #[xgroup]
            ::std::boxed::Box<crate::xhtml_schema::xhtml::types::XhtmlVarType>,
        ),
        #[xelement(
            name = "cite",
            namespace = "http://www.w3.org/1999/xhtml",
            allow_unknown_attributes = "any"
        )]
        Cite(
            #[xgroup]
            ::std::boxed::Box<crate::xhtml_schema::xhtml::types::XhtmlCiteType>,
        ),
        #[xelement(
            name = "abbr",
            namespace = "http://www.w3.org/1999/xhtml",
            allow_unknown_attributes = "any"
        )]
        Abbr(
            #[xgroup]
            ::std::boxed::Box<crate::xhtml_schema::xhtml::types::XhtmlAbbrType>,
        ),
        #[xelement(
            name = "acronym",
            namespace = "http://www.w3.org/1999/xhtml",
            allow_unknown_attributes = "any"
        )]
        Acronym(
            #[xgroup]
            ::std::boxed::Box<crate::xhtml_schema::xhtml::types::XhtmlAcronymType>,
        ),
        #[xelement(
            name = "q",
            namespace = "http://www.w3.org/1999/xhtml",
            allow_unknown_attributes = "any"
        )]
        Q(#[xgroup] ::std::boxed::Box<crate::xhtml_schema::xhtml::types::XhtmlQType>),
    }
    impl ::core::convert::From<crate::xhtml_schema::xhtml::types::XhtmlEmType>
    for XhtmlInlPhrasClass {
        fn from(value: crate::xhtml_schema::xhtml::types::XhtmlEmType) -> Self {
            XhtmlInlPhrasClass::Em(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml_schema::xhtml::types::XhtmlStrongType>
    for XhtmlInlPhrasClass {
        fn from(value: crate::xhtml_schema::xhtml::types::XhtmlStrongType) -> Self {
            XhtmlInlPhrasClass::Strong(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml_schema::xhtml::types::XhtmlDfnType>
    for XhtmlInlPhrasClass {
        fn from(value: crate::xhtml_schema::xhtml::types::XhtmlDfnType) -> Self {
            XhtmlInlPhrasClass::Dfn(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml_schema::xhtml::types::XhtmlCodeType>
    for XhtmlInlPhrasClass {
        fn from(value: crate::xhtml_schema::xhtml::types::XhtmlCodeType) -> Self {
            XhtmlInlPhrasClass::Code(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml_schema::xhtml::types::XhtmlSampType>
    for XhtmlInlPhrasClass {
        fn from(value: crate::xhtml_schema::xhtml::types::XhtmlSampType) -> Self {
            XhtmlInlPhrasClass::Samp(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml_schema::xhtml::types::XhtmlKbdType>
    for XhtmlInlPhrasClass {
        fn from(value: crate::xhtml_schema::xhtml::types::XhtmlKbdType) -> Self {
            XhtmlInlPhrasClass::Kbd(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml_schema::xhtml::types::XhtmlVarType>
    for XhtmlInlPhrasClass {
        fn from(value: crate::xhtml_schema::xhtml::types::XhtmlVarType) -> Self {
            XhtmlInlPhrasClass::Var(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml_schema::xhtml::types::XhtmlCiteType>
    for XhtmlInlPhrasClass {
        fn from(value: crate::xhtml_schema::xhtml::types::XhtmlCiteType) -> Self {
            XhtmlInlPhrasClass::Cite(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml_schema::xhtml::types::XhtmlAbbrType>
    for XhtmlInlPhrasClass {
        fn from(value: crate::xhtml_schema::xhtml::types::XhtmlAbbrType) -> Self {
            XhtmlInlPhrasClass::Abbr(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml_schema::xhtml::types::XhtmlAcronymType>
    for XhtmlInlPhrasClass {
        fn from(value: crate::xhtml_schema::xhtml::types::XhtmlAcronymType) -> Self {
            XhtmlInlPhrasClass::Acronym(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml_schema::xhtml::types::XhtmlQType>
    for XhtmlInlPhrasClass {
        fn from(value: crate::xhtml_schema::xhtml::types::XhtmlQType) -> Self {
            XhtmlInlPhrasClass::Q(::std::boxed::Box::new(value))
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub enum XhtmlInlPresClass {
        #[xelement(
            name = "tt",
            namespace = "http://www.w3.org/1999/xhtml",
            allow_unknown_attributes = "any"
        )]
        Tt(
            #[xgroup]
            ::std::boxed::Box<crate::xhtml_schema::xhtml::types::XhtmlInlPresType>,
        ),
        #[xelement(
            name = "i",
            namespace = "http://www.w3.org/1999/xhtml",
            allow_unknown_attributes = "any"
        )]
        I(
            #[xgroup]
            ::std::boxed::Box<crate::xhtml_schema::xhtml::types::XhtmlInlPresType>,
        ),
        #[xelement(
            name = "b",
            namespace = "http://www.w3.org/1999/xhtml",
            allow_unknown_attributes = "any"
        )]
        B(
            #[xgroup]
            ::std::boxed::Box<crate::xhtml_schema::xhtml::types::XhtmlInlPresType>,
        ),
        #[xelement(
            name = "big",
            namespace = "http://www.w3.org/1999/xhtml",
            allow_unknown_attributes = "any"
        )]
        Big(
            #[xgroup]
            ::std::boxed::Box<crate::xhtml_schema::xhtml::types::XhtmlInlPresType>,
        ),
        #[xelement(
            name = "small",
            namespace = "http://www.w3.org/1999/xhtml",
            allow_unknown_attributes = "any"
        )]
        Small(
            #[xgroup]
            ::std::boxed::Box<crate::xhtml_schema::xhtml::types::XhtmlInlPresType>,
        ),
        #[xelement(
            name = "sub",
            namespace = "http://www.w3.org/1999/xhtml",
            allow_unknown_attributes = "any"
        )]
        Sub(
            #[xgroup]
            ::std::boxed::Box<crate::xhtml_schema::xhtml::types::XhtmlInlPresType>,
        ),
        #[xelement(
            name = "sup",
            namespace = "http://www.w3.org/1999/xhtml",
            allow_unknown_attributes = "any"
        )]
        Sup(
            #[xgroup]
            ::std::boxed::Box<crate::xhtml_schema::xhtml::types::XhtmlInlPresType>,
        ),
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xvalue(order = "strict")]
    pub struct XhtmlInlPresContent {
        #[xvalue(default)]
        pub xhtml_inline_mix: ::std::vec::Vec<
            crate::xhtml_schema::xhtml::groups::XhtmlInlineMix,
        >,
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub enum XhtmlInlSpecialClass {
        #[xelement(
            name = "img",
            namespace = "http://www.w3.org/1999/xhtml",
            allow_unknown_attributes = "any"
        )]
        Img(
            #[xgroup]
            ::std::boxed::Box<crate::xhtml_schema::xhtml::types::XhtmlImgType>,
        ),
        #[xelement(
            name = "map",
            namespace = "http://www.w3.org/1999/xhtml",
            allow_unknown_attributes = "any"
        )]
        Map(
            #[xgroup]
            ::std::boxed::Box<crate::xhtml_schema::xhtml::types::XhtmlMapType>,
        ),
        #[xelement(
            name = "object",
            namespace = "http://www.w3.org/1999/xhtml",
            allow_unknown_attributes = "any"
        )]
        Object(
            #[xgroup]
            ::std::boxed::Box<crate::xhtml_schema::xhtml::types::XhtmlObjectType>,
        ),
    }
    impl ::core::convert::From<crate::xhtml_schema::xhtml::types::XhtmlImgType>
    for XhtmlInlSpecialClass {
        fn from(value: crate::xhtml_schema::xhtml::types::XhtmlImgType) -> Self {
            XhtmlInlSpecialClass::Img(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml_schema::xhtml::types::XhtmlMapType>
    for XhtmlInlSpecialClass {
        fn from(value: crate::xhtml_schema::xhtml::types::XhtmlMapType) -> Self {
            XhtmlInlSpecialClass::Map(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml_schema::xhtml::types::XhtmlObjectType>
    for XhtmlInlSpecialClass {
        fn from(value: crate::xhtml_schema::xhtml::types::XhtmlObjectType) -> Self {
            XhtmlInlSpecialClass::Object(::std::boxed::Box::new(value))
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub enum XhtmlInlStructClass {
        #[xelement(
            name = "br",
            namespace = "http://www.w3.org/1999/xhtml",
            allow_unknown_attributes = "any"
        )]
        Br(#[xgroup] ::std::boxed::Box<crate::xhtml_schema::xhtml::types::XhtmlBrType>),
        #[xelement(
            name = "span",
            namespace = "http://www.w3.org/1999/xhtml",
            allow_unknown_attributes = "any"
        )]
        Span(
            #[xgroup]
            ::std::boxed::Box<crate::xhtml_schema::xhtml::types::XhtmlSpanType>,
        ),
    }
    impl ::core::convert::From<crate::xhtml_schema::xhtml::types::XhtmlBrType>
    for XhtmlInlStructClass {
        fn from(value: crate::xhtml_schema::xhtml::types::XhtmlBrType) -> Self {
            XhtmlInlStructClass::Br(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml_schema::xhtml::types::XhtmlSpanType>
    for XhtmlInlStructClass {
        fn from(value: crate::xhtml_schema::xhtml::types::XhtmlSpanType) -> Self {
            XhtmlInlStructClass::Span(::std::boxed::Box::new(value))
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub enum XhtmlInlineClass {
        XhtmlInlStructClass(
            ::std::boxed::Box<crate::xhtml_schema::xhtml::groups::XhtmlInlStructClass>,
        ),
        XhtmlInlPhrasClass(
            ::std::boxed::Box<crate::xhtml_schema::xhtml::groups::XhtmlInlPhrasClass>,
        ),
        XhtmlInlPresClass(
            ::std::boxed::Box<crate::xhtml_schema::xhtml::groups::XhtmlInlPresClass>,
        ),
        XhtmlI18NClass(
            ::std::boxed::Box<crate::xhtml_schema::xhtml::groups::XhtmlI18NClass>,
        ),
        XhtmlAnchorClass(
            ::std::boxed::Box<crate::xhtml_schema::xhtml::groups::XhtmlAnchorClass>,
        ),
        XhtmlInlSpecialClass(
            ::std::boxed::Box<crate::xhtml_schema::xhtml::groups::XhtmlInlSpecialClass>,
        ),
        XhtmlInlFormClass(
            ::std::boxed::Box<crate::xhtml_schema::xhtml::groups::XhtmlInlFormClass>,
        ),
        XhtmlRubyClass(
            ::std::boxed::Box<crate::xhtml_schema::xhtml::groups::XhtmlRubyClass>,
        ),
        XhtmlInlineExtra(
            ::std::boxed::Box<crate::xhtml_schema::xhtml::groups::XhtmlInlineExtra>,
        ),
    }
    impl ::core::convert::From<crate::xhtml_schema::xhtml::groups::XhtmlInlStructClass>
    for XhtmlInlineClass {
        fn from(value: crate::xhtml_schema::xhtml::groups::XhtmlInlStructClass) -> Self {
            XhtmlInlineClass::XhtmlInlStructClass(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml_schema::xhtml::groups::XhtmlInlPhrasClass>
    for XhtmlInlineClass {
        fn from(value: crate::xhtml_schema::xhtml::groups::XhtmlInlPhrasClass) -> Self {
            XhtmlInlineClass::XhtmlInlPhrasClass(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml_schema::xhtml::groups::XhtmlInlPresClass>
    for XhtmlInlineClass {
        fn from(value: crate::xhtml_schema::xhtml::groups::XhtmlInlPresClass) -> Self {
            XhtmlInlineClass::XhtmlInlPresClass(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml_schema::xhtml::groups::XhtmlI18NClass>
    for XhtmlInlineClass {
        fn from(value: crate::xhtml_schema::xhtml::groups::XhtmlI18NClass) -> Self {
            XhtmlInlineClass::XhtmlI18NClass(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml_schema::xhtml::groups::XhtmlAnchorClass>
    for XhtmlInlineClass {
        fn from(value: crate::xhtml_schema::xhtml::groups::XhtmlAnchorClass) -> Self {
            XhtmlInlineClass::XhtmlAnchorClass(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml_schema::xhtml::groups::XhtmlInlSpecialClass>
    for XhtmlInlineClass {
        fn from(
            value: crate::xhtml_schema::xhtml::groups::XhtmlInlSpecialClass,
        ) -> Self {
            XhtmlInlineClass::XhtmlInlSpecialClass(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml_schema::xhtml::groups::XhtmlInlFormClass>
    for XhtmlInlineClass {
        fn from(value: crate::xhtml_schema::xhtml::groups::XhtmlInlFormClass) -> Self {
            XhtmlInlineClass::XhtmlInlFormClass(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml_schema::xhtml::groups::XhtmlRubyClass>
    for XhtmlInlineClass {
        fn from(value: crate::xhtml_schema::xhtml::groups::XhtmlRubyClass) -> Self {
            XhtmlInlineClass::XhtmlRubyClass(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml_schema::xhtml::groups::XhtmlInlineExtra>
    for XhtmlInlineClass {
        fn from(value: crate::xhtml_schema::xhtml::groups::XhtmlInlineExtra) -> Self {
            XhtmlInlineClass::XhtmlInlineExtra(::std::boxed::Box::new(value))
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
    pub struct XhtmlInlineExtra;
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub enum XhtmlInlineMix {
        XhtmlInlineClass(
            ::std::boxed::Box<crate::xhtml_schema::xhtml::groups::XhtmlInlineClass>,
        ),
        XhtmlMiscClass(
            ::std::boxed::Box<crate::xhtml_schema::xhtml::groups::XhtmlMiscClass>,
        ),
    }
    impl ::core::convert::From<crate::xhtml_schema::xhtml::groups::XhtmlInlineClass>
    for XhtmlInlineMix {
        fn from(value: crate::xhtml_schema::xhtml::groups::XhtmlInlineClass) -> Self {
            XhtmlInlineMix::XhtmlInlineClass(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml_schema::xhtml::groups::XhtmlMiscClass>
    for XhtmlInlineMix {
        fn from(value: crate::xhtml_schema::xhtml::groups::XhtmlMiscClass) -> Self {
            XhtmlInlineMix::XhtmlMiscClass(::std::boxed::Box::new(value))
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub enum XhtmlInlinePreMix {
        XhtmlInlStructClass(
            ::std::boxed::Box<crate::xhtml_schema::xhtml::groups::XhtmlInlStructClass>,
        ),
        XhtmlInlPhrasClass(
            ::std::boxed::Box<crate::xhtml_schema::xhtml::groups::XhtmlInlPhrasClass>,
        ),
        #[xelement(
            name = "tt",
            namespace = "http://www.w3.org/1999/xhtml",
            allow_unknown_attributes = "any"
        )]
        Tt(
            #[xgroup]
            ::std::boxed::Box<crate::xhtml_schema::xhtml::types::XhtmlInlPresType>,
        ),
        #[xelement(
            name = "i",
            namespace = "http://www.w3.org/1999/xhtml",
            allow_unknown_attributes = "any"
        )]
        I(
            #[xgroup]
            ::std::boxed::Box<crate::xhtml_schema::xhtml::types::XhtmlInlPresType>,
        ),
        #[xelement(
            name = "b",
            namespace = "http://www.w3.org/1999/xhtml",
            allow_unknown_attributes = "any"
        )]
        B(
            #[xgroup]
            ::std::boxed::Box<crate::xhtml_schema::xhtml::types::XhtmlInlPresType>,
        ),
        XhtmlI18NClass(
            ::std::boxed::Box<crate::xhtml_schema::xhtml::groups::XhtmlI18NClass>,
        ),
        XhtmlAnchorClass(
            ::std::boxed::Box<crate::xhtml_schema::xhtml::groups::XhtmlAnchorClass>,
        ),
        XhtmlMiscClass(
            ::std::boxed::Box<crate::xhtml_schema::xhtml::groups::XhtmlMiscClass>,
        ),
        #[xelement(
            name = "map",
            namespace = "http://www.w3.org/1999/xhtml",
            allow_unknown_attributes = "any"
        )]
        Map(
            #[xgroup]
            ::std::boxed::Box<crate::xhtml_schema::xhtml::types::XhtmlMapType>,
        ),
        XhtmlInlineExtra(
            ::std::boxed::Box<crate::xhtml_schema::xhtml::groups::XhtmlInlineExtra>,
        ),
    }
    impl ::core::convert::From<crate::xhtml_schema::xhtml::groups::XhtmlInlStructClass>
    for XhtmlInlinePreMix {
        fn from(value: crate::xhtml_schema::xhtml::groups::XhtmlInlStructClass) -> Self {
            XhtmlInlinePreMix::XhtmlInlStructClass(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml_schema::xhtml::groups::XhtmlInlPhrasClass>
    for XhtmlInlinePreMix {
        fn from(value: crate::xhtml_schema::xhtml::groups::XhtmlInlPhrasClass) -> Self {
            XhtmlInlinePreMix::XhtmlInlPhrasClass(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml_schema::xhtml::groups::XhtmlI18NClass>
    for XhtmlInlinePreMix {
        fn from(value: crate::xhtml_schema::xhtml::groups::XhtmlI18NClass) -> Self {
            XhtmlInlinePreMix::XhtmlI18NClass(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml_schema::xhtml::groups::XhtmlAnchorClass>
    for XhtmlInlinePreMix {
        fn from(value: crate::xhtml_schema::xhtml::groups::XhtmlAnchorClass) -> Self {
            XhtmlInlinePreMix::XhtmlAnchorClass(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml_schema::xhtml::groups::XhtmlMiscClass>
    for XhtmlInlinePreMix {
        fn from(value: crate::xhtml_schema::xhtml::groups::XhtmlMiscClass) -> Self {
            XhtmlInlinePreMix::XhtmlMiscClass(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml_schema::xhtml::types::XhtmlMapType>
    for XhtmlInlinePreMix {
        fn from(value: crate::xhtml_schema::xhtml::types::XhtmlMapType) -> Self {
            XhtmlInlinePreMix::Map(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml_schema::xhtml::groups::XhtmlInlineExtra>
    for XhtmlInlinePreMix {
        fn from(value: crate::xhtml_schema::xhtml::groups::XhtmlInlineExtra) -> Self {
            XhtmlInlinePreMix::XhtmlInlineExtra(::std::boxed::Box::new(value))
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub enum XhtmlListClass {
        #[xelement(
            name = "ul",
            namespace = "http://www.w3.org/1999/xhtml",
            allow_unknown_attributes = "any"
        )]
        Ul(#[xgroup] ::std::boxed::Box<crate::xhtml_schema::xhtml::types::XhtmlUlType>),
        #[xelement(
            name = "ol",
            namespace = "http://www.w3.org/1999/xhtml",
            allow_unknown_attributes = "any"
        )]
        Ol(#[xgroup] ::std::boxed::Box<crate::xhtml_schema::xhtml::types::XhtmlOlType>),
        #[xelement(
            name = "dl",
            namespace = "http://www.w3.org/1999/xhtml",
            allow_unknown_attributes = "any"
        )]
        Dl(#[xgroup] ::std::boxed::Box<crate::xhtml_schema::xhtml::types::XhtmlDlType>),
    }
    impl ::core::convert::From<crate::xhtml_schema::xhtml::types::XhtmlUlType>
    for XhtmlListClass {
        fn from(value: crate::xhtml_schema::xhtml::types::XhtmlUlType) -> Self {
            XhtmlListClass::Ul(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml_schema::xhtml::types::XhtmlOlType>
    for XhtmlListClass {
        fn from(value: crate::xhtml_schema::xhtml::types::XhtmlOlType) -> Self {
            XhtmlListClass::Ol(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml_schema::xhtml::types::XhtmlDlType>
    for XhtmlListClass {
        fn from(value: crate::xhtml_schema::xhtml::types::XhtmlDlType) -> Self {
            XhtmlListClass::Dl(::std::boxed::Box::new(value))
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub enum XhtmlMiscClass {
        XhtmlEditClass(
            ::std::boxed::Box<crate::xhtml_schema::xhtml::groups::XhtmlEditClass>,
        ),
        XhtmlScriptClass(
            ::std::boxed::Box<crate::xhtml_schema::xhtml::groups::XhtmlScriptClass>,
        ),
        XhtmlMiscExtra(
            ::std::boxed::Box<crate::xhtml_schema::xhtml::groups::XhtmlMiscExtra>,
        ),
    }
    impl ::core::convert::From<crate::xhtml_schema::xhtml::groups::XhtmlEditClass>
    for XhtmlMiscClass {
        fn from(value: crate::xhtml_schema::xhtml::groups::XhtmlEditClass) -> Self {
            XhtmlMiscClass::XhtmlEditClass(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml_schema::xhtml::groups::XhtmlScriptClass>
    for XhtmlMiscClass {
        fn from(value: crate::xhtml_schema::xhtml::groups::XhtmlScriptClass) -> Self {
            XhtmlMiscClass::XhtmlScriptClass(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml_schema::xhtml::groups::XhtmlMiscExtra>
    for XhtmlMiscClass {
        fn from(value: crate::xhtml_schema::xhtml::groups::XhtmlMiscExtra) -> Self {
            XhtmlMiscClass::XhtmlMiscExtra(::std::boxed::Box::new(value))
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
    pub struct XhtmlMiscExtra;
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xvalue(order = "strict")]
    pub struct XhtmlRubyClass {
        #[xelement(name = "ruby", namespace = "http://www.w3.org/1999/xhtml", group)]
        pub ruby: ::std::boxed::Box<crate::xhtml_schema::xhtml::types::XhtmlRubyType>,
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub enum XhtmlScriptClass {
        #[xelement(
            name = "script",
            namespace = "http://www.w3.org/1999/xhtml",
            allow_unknown_attributes = "any"
        )]
        Script(
            #[xgroup]
            ::std::boxed::Box<crate::xhtml_schema::xhtml::types::XhtmlScriptType>,
        ),
        #[xelement(
            name = "noscript",
            namespace = "http://www.w3.org/1999/xhtml",
            allow_unknown_attributes = "any"
        )]
        Noscript(
            #[xgroup]
            ::std::boxed::Box<crate::xhtml_schema::xhtml::types::XhtmlNoscriptType>,
        ),
    }
    impl ::core::convert::From<crate::xhtml_schema::xhtml::types::XhtmlScriptType>
    for XhtmlScriptClass {
        fn from(value: crate::xhtml_schema::xhtml::types::XhtmlScriptType) -> Self {
            XhtmlScriptClass::Script(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xhtml_schema::xhtml::types::XhtmlNoscriptType>
    for XhtmlScriptClass {
        fn from(value: crate::xhtml_schema::xhtml::types::XhtmlNoscriptType) -> Self {
            XhtmlScriptClass::Noscript(::std::boxed::Box::new(value))
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub enum XhtmlTableClass {
        #[xelement(
            name = "table",
            namespace = "http://www.w3.org/1999/xhtml",
            allow_unknown_attributes = "any"
        )]
        Table(
            #[xgroup]
            ::std::boxed::Box<crate::xhtml_schema::xhtml::types::XhtmlTableType>,
        ),
    }
    impl ::core::convert::From<crate::xhtml_schema::xhtml::types::XhtmlTableType>
    for XhtmlTableClass {
        fn from(value: crate::xhtml_schema::xhtml::types::XhtmlTableType) -> Self {
            XhtmlTableClass::Table(::std::boxed::Box::new(value))
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
    pub struct XhtmlAContent {
        #[xvalue(default)]
        pub xhtml_inl_no_anchor_mix: ::std::vec::Vec<
            crate::xhtml_schema::xhtml::groups::XhtmlInlNoAnchorMix,
        >,
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xvalue(order = "strict")]
    pub struct XhtmlAbbrContent {
        #[xvalue(default)]
        pub xhtml_inline_mix: ::std::vec::Vec<
            crate::xhtml_schema::xhtml::groups::XhtmlInlineMix,
        >,
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xvalue(order = "strict")]
    pub struct XhtmlAcronymContent {
        #[xvalue(default)]
        pub xhtml_inline_mix: ::std::vec::Vec<
            crate::xhtml_schema::xhtml::groups::XhtmlInlineMix,
        >,
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xvalue(order = "strict")]
    pub struct XhtmlAddressContent {
        #[xvalue(default)]
        pub xhtml_inline_mix: ::std::vec::Vec<
            crate::xhtml_schema::xhtml::groups::XhtmlInlineMix,
        >,
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xvalue(order = "strict")]
    pub struct XhtmlAreaContent;
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xvalue(order = "strict")]
    pub struct XhtmlBaseContent;
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xvalue(order = "strict")]
    pub struct XhtmlBdoContent {
        #[xvalue(default)]
        pub xhtml_inline_mix: ::std::vec::Vec<
            crate::xhtml_schema::xhtml::groups::XhtmlInlineMix,
        >,
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xvalue(order = "strict")]
    pub struct XhtmlBlockquoteContent {
        #[xvalue(default)]
        pub xhtml_block_mix: ::std::vec::Vec<
            crate::xhtml_schema::xhtml::groups::XhtmlBlockMix,
        >,
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xvalue(order = "strict")]
    pub struct XhtmlBodyContent {
        #[xvalue(default)]
        pub xhtml_block_mix: ::std::vec::Vec<
            crate::xhtml_schema::xhtml::groups::XhtmlBlockMix,
        >,
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xvalue(order = "strict")]
    pub struct XhtmlBrContent;
    pub mod xhtml_button_content_items {
        impl ::core::convert::From<crate::xhtml_schema::xhtml::groups::XhtmlBlkNoFormMix>
        for Xhtml {
            fn from(
                value: crate::xhtml_schema::xhtml::groups::XhtmlBlkNoFormMix,
            ) -> Self {
                Xhtml::XhtmlBlkNoFormMix(::std::boxed::Box::new(value))
            }
        }
        impl ::core::convert::From<
            crate::xhtml_schema::xhtml::groups::XhtmlInlStructClass,
        > for Xhtml {
            fn from(
                value: crate::xhtml_schema::xhtml::groups::XhtmlInlStructClass,
            ) -> Self {
                Xhtml::XhtmlInlStructClass(::std::boxed::Box::new(value))
            }
        }
        impl ::core::convert::From<
            crate::xhtml_schema::xhtml::groups::XhtmlInlPhrasClass,
        > for Xhtml {
            fn from(
                value: crate::xhtml_schema::xhtml::groups::XhtmlInlPhrasClass,
            ) -> Self {
                Xhtml::XhtmlInlPhrasClass(::std::boxed::Box::new(value))
            }
        }
        impl ::core::convert::From<crate::xhtml_schema::xhtml::groups::XhtmlInlPresClass>
        for Xhtml {
            fn from(
                value: crate::xhtml_schema::xhtml::groups::XhtmlInlPresClass,
            ) -> Self {
                Xhtml::XhtmlInlPresClass(::std::boxed::Box::new(value))
            }
        }
        impl ::core::convert::From<crate::xhtml_schema::xhtml::groups::XhtmlI18NClass>
        for Xhtml {
            fn from(value: crate::xhtml_schema::xhtml::groups::XhtmlI18NClass) -> Self {
                Xhtml::XhtmlI18NClass(::std::boxed::Box::new(value))
            }
        }
        impl ::core::convert::From<
            crate::xhtml_schema::xhtml::groups::XhtmlInlSpecialClass,
        > for Xhtml {
            fn from(
                value: crate::xhtml_schema::xhtml::groups::XhtmlInlSpecialClass,
            ) -> Self {
                Xhtml::XhtmlInlSpecialClass(::std::boxed::Box::new(value))
            }
        }
        impl ::core::convert::From<crate::xhtml_schema::xhtml::groups::XhtmlInlineExtra>
        for Xhtml {
            fn from(
                value: crate::xhtml_schema::xhtml::groups::XhtmlInlineExtra,
            ) -> Self {
                Xhtml::XhtmlInlineExtra(::std::boxed::Box::new(value))
            }
        }
        #[derive(
            ::core::fmt::Debug,
            ::xmlity::Serialize,
            ::xmlity::Deserialize,
            ::core::cmp::PartialEq,
            ::core::clone::Clone
        )]
        pub enum Xhtml {
            XhtmlBlkNoFormMix(
                ::std::boxed::Box<crate::xhtml_schema::xhtml::groups::XhtmlBlkNoFormMix>,
            ),
            XhtmlInlStructClass(
                ::std::boxed::Box<
                    crate::xhtml_schema::xhtml::groups::XhtmlInlStructClass,
                >,
            ),
            XhtmlInlPhrasClass(
                ::std::boxed::Box<crate::xhtml_schema::xhtml::groups::XhtmlInlPhrasClass>,
            ),
            XhtmlInlPresClass(
                ::std::boxed::Box<crate::xhtml_schema::xhtml::groups::XhtmlInlPresClass>,
            ),
            XhtmlI18NClass(
                ::std::boxed::Box<crate::xhtml_schema::xhtml::groups::XhtmlI18NClass>,
            ),
            XhtmlInlSpecialClass(
                ::std::boxed::Box<
                    crate::xhtml_schema::xhtml::groups::XhtmlInlSpecialClass,
                >,
            ),
            XhtmlInlineExtra(
                ::std::boxed::Box<crate::xhtml_schema::xhtml::groups::XhtmlInlineExtra>,
            ),
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
    pub struct XhtmlButtonContent {
        #[xvalue(default)]
        pub xhtml: ::std::vec::Vec<xhtml_button_content_items::Xhtml>,
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xvalue(order = "strict")]
    pub struct XhtmlCaptionContent {
        #[xvalue(default)]
        pub xhtml_inline_mix: ::std::vec::Vec<
            crate::xhtml_schema::xhtml::groups::XhtmlInlineMix,
        >,
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xvalue(order = "strict")]
    pub struct XhtmlCiteContent {
        #[xvalue(default)]
        pub xhtml_inline_mix: ::std::vec::Vec<
            crate::xhtml_schema::xhtml::groups::XhtmlInlineMix,
        >,
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xvalue(order = "strict")]
    pub struct XhtmlCodeContent {
        #[xvalue(default)]
        pub xhtml_inline_mix: ::std::vec::Vec<
            crate::xhtml_schema::xhtml::groups::XhtmlInlineMix,
        >,
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xvalue(order = "strict")]
    pub struct XhtmlColContent;
    pub mod xhtml_colgroup_content_items {
        impl ::core::convert::From<crate::xhtml_schema::xhtml::types::XhtmlColType>
        for Col {
            fn from(value: crate::xhtml_schema::xhtml::types::XhtmlColType) -> Self {
                Col(::std::boxed::Box::new(value))
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
        pub struct Col(
            #[xgroup]
            pub ::std::boxed::Box<crate::xhtml_schema::xhtml::types::XhtmlColType>,
        );
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xvalue(order = "strict")]
    pub struct XhtmlColgroupContent {
        #[xvalue(default)]
        pub col: ::std::vec::Vec<xhtml_colgroup_content_items::Col>,
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xvalue(order = "strict")]
    pub struct XhtmlDdContent {
        #[xvalue(default)]
        pub xhtml_flow_mix: ::std::vec::Vec<
            crate::xhtml_schema::xhtml::groups::XhtmlFlowMix,
        >,
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xvalue(order = "strict")]
    pub struct XhtmlDfnContent {
        #[xvalue(default)]
        pub xhtml_inline_mix: ::std::vec::Vec<
            crate::xhtml_schema::xhtml::groups::XhtmlInlineMix,
        >,
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xvalue(order = "strict")]
    pub struct XhtmlDivContent {
        #[xvalue(default)]
        pub xhtml_flow_mix: ::std::vec::Vec<
            crate::xhtml_schema::xhtml::groups::XhtmlFlowMix,
        >,
    }
    pub mod xhtml_dl_content_items {
        impl ::core::convert::From<crate::xhtml_schema::xhtml::types::XhtmlDtType>
        for Child0 {
            fn from(value: crate::xhtml_schema::xhtml::types::XhtmlDtType) -> Self {
                Child0::Dt(::std::boxed::Box::new(value))
            }
        }
        impl ::core::convert::From<crate::xhtml_schema::xhtml::types::XhtmlDdType>
        for Child0 {
            fn from(value: crate::xhtml_schema::xhtml::types::XhtmlDdType) -> Self {
                Child0::Dd(::std::boxed::Box::new(value))
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
            #[xelement(
                name = "dt",
                namespace = "http://www.w3.org/1999/xhtml",
                allow_unknown_attributes = "any"
            )]
            Dt(
                #[xgroup]
                ::std::boxed::Box<crate::xhtml_schema::xhtml::types::XhtmlDtType>,
            ),
            #[xelement(
                name = "dd",
                namespace = "http://www.w3.org/1999/xhtml",
                allow_unknown_attributes = "any"
            )]
            Dd(
                #[xgroup]
                ::std::boxed::Box<crate::xhtml_schema::xhtml::types::XhtmlDdType>,
            ),
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
    pub struct XhtmlDlContent {
        #[xvalue(default)]
        pub child_0: ::std::vec::Vec<xhtml_dl_content_items::Child0>,
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xvalue(order = "strict")]
    pub struct XhtmlDtContent {
        #[xvalue(default)]
        pub xhtml_inline_mix: ::std::vec::Vec<
            crate::xhtml_schema::xhtml::groups::XhtmlInlineMix,
        >,
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xvalue(order = "strict")]
    pub struct XhtmlEditContent {
        #[xvalue(default)]
        pub xhtml_flow_mix: ::std::vec::Vec<
            crate::xhtml_schema::xhtml::groups::XhtmlFlowMix,
        >,
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xvalue(order = "strict")]
    pub struct XhtmlEmContent {
        #[xvalue(default)]
        pub xhtml_inline_mix: ::std::vec::Vec<
            crate::xhtml_schema::xhtml::groups::XhtmlInlineMix,
        >,
    }
    pub mod xhtml_fieldset_content_items {
        impl ::core::convert::From<crate::xhtml_schema::xhtml::types::XhtmlLegendType>
        for Child0 {
            fn from(value: crate::xhtml_schema::xhtml::types::XhtmlLegendType) -> Self {
                Child0::Legend(::std::boxed::Box::new(value))
            }
        }
        impl ::core::convert::From<crate::xhtml_schema::xhtml::groups::XhtmlFlowMix>
        for Child0 {
            fn from(value: crate::xhtml_schema::xhtml::groups::XhtmlFlowMix) -> Self {
                Child0::XhtmlFlowMix(::std::boxed::Box::new(value))
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
            #[xelement(
                name = "legend",
                namespace = "http://www.w3.org/1999/xhtml",
                allow_unknown_attributes = "any"
            )]
            Legend(
                #[xgroup]
                ::std::boxed::Box<crate::xhtml_schema::xhtml::types::XhtmlLegendType>,
            ),
            XhtmlFlowMix(
                ::std::boxed::Box<crate::xhtml_schema::xhtml::groups::XhtmlFlowMix>,
            ),
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
    pub struct XhtmlFieldsetContent {
        #[xvalue(default)]
        pub child_0: ::std::vec::Vec<xhtml_fieldset_content_items::Child0>,
    }
    pub mod xhtml_form_content_items {
        impl ::core::convert::From<crate::xhtml_schema::xhtml::groups::XhtmlBlkNoFormMix>
        for Child0 {
            fn from(
                value: crate::xhtml_schema::xhtml::groups::XhtmlBlkNoFormMix,
            ) -> Self {
                Child0::XhtmlBlkNoFormMix(::std::boxed::Box::new(value))
            }
        }
        impl ::core::convert::From<crate::xhtml_schema::xhtml::types::XhtmlFieldsetType>
        for Child0 {
            fn from(
                value: crate::xhtml_schema::xhtml::types::XhtmlFieldsetType,
            ) -> Self {
                Child0::Fieldset(::std::boxed::Box::new(value))
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
            XhtmlBlkNoFormMix(
                ::std::boxed::Box<crate::xhtml_schema::xhtml::groups::XhtmlBlkNoFormMix>,
            ),
            #[xelement(
                name = "fieldset",
                namespace = "http://www.w3.org/1999/xhtml",
                allow_unknown_attributes = "any"
            )]
            Fieldset(
                #[xgroup]
                ::std::boxed::Box<crate::xhtml_schema::xhtml::types::XhtmlFieldsetType>,
            ),
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
    pub struct XhtmlFormContent {
        #[xvalue(default)]
        pub child_0: ::std::vec::Vec<xhtml_form_content_items::Child0>,
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xvalue(order = "strict")]
    pub struct XhtmlH1Content {
        #[xvalue(default)]
        pub xhtml_inline_mix: ::std::vec::Vec<
            crate::xhtml_schema::xhtml::groups::XhtmlInlineMix,
        >,
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xvalue(order = "strict")]
    pub struct XhtmlH2Content {
        #[xvalue(default)]
        pub xhtml_inline_mix: ::std::vec::Vec<
            crate::xhtml_schema::xhtml::groups::XhtmlInlineMix,
        >,
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xvalue(order = "strict")]
    pub struct XhtmlH3Content {
        #[xvalue(default)]
        pub xhtml_inline_mix: ::std::vec::Vec<
            crate::xhtml_schema::xhtml::groups::XhtmlInlineMix,
        >,
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xvalue(order = "strict")]
    pub struct XhtmlH4Content {
        #[xvalue(default)]
        pub xhtml_inline_mix: ::std::vec::Vec<
            crate::xhtml_schema::xhtml::groups::XhtmlInlineMix,
        >,
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xvalue(order = "strict")]
    pub struct XhtmlH5Content {
        #[xvalue(default)]
        pub xhtml_inline_mix: ::std::vec::Vec<
            crate::xhtml_schema::xhtml::groups::XhtmlInlineMix,
        >,
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xvalue(order = "strict")]
    pub struct XhtmlH6Content {
        #[xvalue(default)]
        pub xhtml_inline_mix: ::std::vec::Vec<
            crate::xhtml_schema::xhtml::groups::XhtmlInlineMix,
        >,
    }
    pub mod xhtml_head_content_items {
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
                    #[xelement(
                        name = "base",
                        namespace = "http://www.w3.org/1999/xhtml",
                        group
                    )]
                    pub base: ::std::boxed::Box<
                        crate::xhtml_schema::xhtml::types::XhtmlBaseType,
                    >,
                    #[xvalue(default)]
                    pub xhtml_head_opts_mix: ::std::vec::Vec<
                        crate::xhtml_schema::xhtml::groups::XhtmlHeadOptsMix,
                    >,
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
                #[xelement(
                    name = "title",
                    namespace = "http://www.w3.org/1999/xhtml",
                    group
                )]
                pub title: ::std::boxed::Box<
                    crate::xhtml_schema::xhtml::types::XhtmlTitleType,
                >,
                #[xvalue(default)]
                pub xhtml_head_opts_mix: ::std::vec::Vec<
                    crate::xhtml_schema::xhtml::groups::XhtmlHeadOptsMix,
                >,
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
                #[xelement(
                    name = "base",
                    namespace = "http://www.w3.org/1999/xhtml",
                    group
                )]
                pub base: ::std::boxed::Box<
                    crate::xhtml_schema::xhtml::types::XhtmlBaseType,
                >,
                #[xvalue(default)]
                pub xhtml_head_opts_mix: ::std::vec::Vec<
                    crate::xhtml_schema::xhtml::groups::XhtmlHeadOptsMix,
                >,
                #[xelement(
                    name = "title",
                    namespace = "http://www.w3.org/1999/xhtml",
                    group
                )]
                pub title: ::std::boxed::Box<
                    crate::xhtml_schema::xhtml::types::XhtmlTitleType,
                >,
                #[xvalue(default)]
                pub xhtml_head_opts_mix_0: ::std::vec::Vec<
                    crate::xhtml_schema::xhtml::groups::XhtmlHeadOptsMix,
                >,
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
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xvalue(order = "strict")]
    pub struct XhtmlHeadContent {
        #[xvalue(default)]
        pub xhtml_head_opts_mix: ::std::vec::Vec<
            crate::xhtml_schema::xhtml::groups::XhtmlHeadOptsMix,
        >,
        pub variant: xhtml_head_content_items::Variant,
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xvalue(order = "strict")]
    pub struct XhtmlHrContent;
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xvalue(order = "strict")]
    pub struct XhtmlHtmlContent {
        #[xelement(name = "head", namespace = "http://www.w3.org/1999/xhtml", group)]
        pub head: ::std::boxed::Box<crate::xhtml_schema::xhtml::types::XhtmlHeadType>,
        #[xelement(name = "body", namespace = "http://www.w3.org/1999/xhtml", group)]
        pub body: ::std::boxed::Box<crate::xhtml_schema::xhtml::types::XhtmlBodyType>,
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xvalue(order = "strict")]
    pub struct XhtmlImgContent;
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xvalue(order = "strict")]
    pub struct XhtmlInputContent;
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xvalue(order = "strict")]
    pub struct XhtmlKbdContent {
        #[xvalue(default)]
        pub xhtml_inline_mix: ::std::vec::Vec<
            crate::xhtml_schema::xhtml::groups::XhtmlInlineMix,
        >,
    }
    pub mod xhtml_label_content_items {
        impl ::core::convert::From<crate::xhtml_schema::xhtml::types::XhtmlInputType>
        for Child0 {
            fn from(value: crate::xhtml_schema::xhtml::types::XhtmlInputType) -> Self {
                Child0::Input(::std::boxed::Box::new(value))
            }
        }
        impl ::core::convert::From<crate::xhtml_schema::xhtml::types::XhtmlSelectType>
        for Child0 {
            fn from(value: crate::xhtml_schema::xhtml::types::XhtmlSelectType) -> Self {
                Child0::Select(::std::boxed::Box::new(value))
            }
        }
        impl ::core::convert::From<crate::xhtml_schema::xhtml::types::XhtmlTextareaType>
        for Child0 {
            fn from(
                value: crate::xhtml_schema::xhtml::types::XhtmlTextareaType,
            ) -> Self {
                Child0::Textarea(::std::boxed::Box::new(value))
            }
        }
        impl ::core::convert::From<crate::xhtml_schema::xhtml::types::XhtmlButtonType>
        for Child0 {
            fn from(value: crate::xhtml_schema::xhtml::types::XhtmlButtonType) -> Self {
                Child0::Button(::std::boxed::Box::new(value))
            }
        }
        impl ::core::convert::From<
            crate::xhtml_schema::xhtml::groups::XhtmlInlStructClass,
        > for Child0 {
            fn from(
                value: crate::xhtml_schema::xhtml::groups::XhtmlInlStructClass,
            ) -> Self {
                Child0::XhtmlInlStructClass(::std::boxed::Box::new(value))
            }
        }
        impl ::core::convert::From<
            crate::xhtml_schema::xhtml::groups::XhtmlInlPhrasClass,
        > for Child0 {
            fn from(
                value: crate::xhtml_schema::xhtml::groups::XhtmlInlPhrasClass,
            ) -> Self {
                Child0::XhtmlInlPhrasClass(::std::boxed::Box::new(value))
            }
        }
        impl ::core::convert::From<crate::xhtml_schema::xhtml::groups::XhtmlI18NClass>
        for Child0 {
            fn from(value: crate::xhtml_schema::xhtml::groups::XhtmlI18NClass) -> Self {
                Child0::XhtmlI18NClass(::std::boxed::Box::new(value))
            }
        }
        impl ::core::convert::From<crate::xhtml_schema::xhtml::groups::XhtmlInlPresClass>
        for Child0 {
            fn from(
                value: crate::xhtml_schema::xhtml::groups::XhtmlInlPresClass,
            ) -> Self {
                Child0::XhtmlInlPresClass(::std::boxed::Box::new(value))
            }
        }
        impl ::core::convert::From<
            crate::xhtml_schema::xhtml::groups::XhtmlInlSpecialClass,
        > for Child0 {
            fn from(
                value: crate::xhtml_schema::xhtml::groups::XhtmlInlSpecialClass,
            ) -> Self {
                Child0::XhtmlInlSpecialClass(::std::boxed::Box::new(value))
            }
        }
        impl ::core::convert::From<crate::xhtml_schema::xhtml::groups::XhtmlInlineExtra>
        for Child0 {
            fn from(
                value: crate::xhtml_schema::xhtml::groups::XhtmlInlineExtra,
            ) -> Self {
                Child0::XhtmlInlineExtra(::std::boxed::Box::new(value))
            }
        }
        impl ::core::convert::From<crate::xhtml_schema::xhtml::groups::XhtmlMiscClass>
        for Child0 {
            fn from(value: crate::xhtml_schema::xhtml::groups::XhtmlMiscClass) -> Self {
                Child0::XhtmlMiscClass(::std::boxed::Box::new(value))
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
            #[xelement(
                name = "input",
                namespace = "http://www.w3.org/1999/xhtml",
                allow_unknown_attributes = "any"
            )]
            Input(
                #[xgroup]
                ::std::boxed::Box<crate::xhtml_schema::xhtml::types::XhtmlInputType>,
            ),
            #[xelement(
                name = "select",
                namespace = "http://www.w3.org/1999/xhtml",
                allow_unknown_attributes = "any"
            )]
            Select(
                #[xgroup]
                ::std::boxed::Box<crate::xhtml_schema::xhtml::types::XhtmlSelectType>,
            ),
            #[xelement(
                name = "textarea",
                namespace = "http://www.w3.org/1999/xhtml",
                allow_unknown_attributes = "any"
            )]
            Textarea(
                #[xgroup]
                ::std::boxed::Box<crate::xhtml_schema::xhtml::types::XhtmlTextareaType>,
            ),
            #[xelement(
                name = "button",
                namespace = "http://www.w3.org/1999/xhtml",
                allow_unknown_attributes = "any"
            )]
            Button(
                #[xgroup]
                ::std::boxed::Box<crate::xhtml_schema::xhtml::types::XhtmlButtonType>,
            ),
            XhtmlInlStructClass(
                ::std::boxed::Box<
                    crate::xhtml_schema::xhtml::groups::XhtmlInlStructClass,
                >,
            ),
            XhtmlInlPhrasClass(
                ::std::boxed::Box<crate::xhtml_schema::xhtml::groups::XhtmlInlPhrasClass>,
            ),
            XhtmlI18NClass(
                ::std::boxed::Box<crate::xhtml_schema::xhtml::groups::XhtmlI18NClass>,
            ),
            XhtmlInlPresClass(
                ::std::boxed::Box<crate::xhtml_schema::xhtml::groups::XhtmlInlPresClass>,
            ),
            XhtmlInlSpecialClass(
                ::std::boxed::Box<
                    crate::xhtml_schema::xhtml::groups::XhtmlInlSpecialClass,
                >,
            ),
            XhtmlInlineExtra(
                ::std::boxed::Box<crate::xhtml_schema::xhtml::groups::XhtmlInlineExtra>,
            ),
            XhtmlMiscClass(
                ::std::boxed::Box<crate::xhtml_schema::xhtml::groups::XhtmlMiscClass>,
            ),
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
    pub struct XhtmlLabelContent {
        #[xvalue(default)]
        pub child_0: ::std::vec::Vec<xhtml_label_content_items::Child0>,
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xvalue(order = "strict")]
    pub struct XhtmlLegendContent {
        #[xvalue(default)]
        pub xhtml_inline_mix: ::std::vec::Vec<
            crate::xhtml_schema::xhtml::groups::XhtmlInlineMix,
        >,
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xvalue(order = "strict")]
    pub struct XhtmlLiContent {
        #[xvalue(default)]
        pub xhtml_flow_mix: ::std::vec::Vec<
            crate::xhtml_schema::xhtml::groups::XhtmlFlowMix,
        >,
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xvalue(order = "strict")]
    pub struct XhtmlLinkContent;
    pub mod xhtml_map_content_items {
        impl ::core::convert::From<crate::xhtml_schema::xhtml::groups::XhtmlBlockMix>
        for Child0 {
            fn from(value: crate::xhtml_schema::xhtml::groups::XhtmlBlockMix) -> Self {
                Child0::XhtmlBlockMix(::std::boxed::Box::new(value))
            }
        }
        impl ::core::convert::From<crate::xhtml_schema::xhtml::types::XhtmlAreaType>
        for Child0 {
            fn from(value: crate::xhtml_schema::xhtml::types::XhtmlAreaType) -> Self {
                Child0::Area(::std::boxed::Box::new(value))
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
            XhtmlBlockMix(
                ::std::boxed::Box<crate::xhtml_schema::xhtml::groups::XhtmlBlockMix>,
            ),
            #[xelement(
                name = "area",
                namespace = "http://www.w3.org/1999/xhtml",
                allow_unknown_attributes = "any"
            )]
            Area(
                #[xgroup]
                ::std::boxed::Box<crate::xhtml_schema::xhtml::types::XhtmlAreaType>,
            ),
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
    pub struct XhtmlMapContent {
        #[xvalue(default)]
        pub child_0: ::std::vec::Vec<xhtml_map_content_items::Child0>,
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xvalue(order = "strict")]
    pub struct XhtmlMetaContent;
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xvalue(order = "strict")]
    pub struct XhtmlNoscriptContent {
        #[xvalue(default)]
        pub xhtml_block_mix: ::std::vec::Vec<
            crate::xhtml_schema::xhtml::groups::XhtmlBlockMix,
        >,
    }
    pub mod xhtml_object_content_items {
        impl ::core::convert::From<crate::xhtml_schema::xhtml::types::XhtmlParamType>
        for Child0 {
            fn from(value: crate::xhtml_schema::xhtml::types::XhtmlParamType) -> Self {
                Child0::Param(::std::boxed::Box::new(value))
            }
        }
        impl ::core::convert::From<crate::xhtml_schema::xhtml::groups::XhtmlFlowMix>
        for Child0 {
            fn from(value: crate::xhtml_schema::xhtml::groups::XhtmlFlowMix) -> Self {
                Child0::XhtmlFlowMix(::std::boxed::Box::new(value))
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
            #[xelement(
                name = "param",
                namespace = "http://www.w3.org/1999/xhtml",
                allow_unknown_attributes = "any"
            )]
            Param(
                #[xgroup]
                ::std::boxed::Box<crate::xhtml_schema::xhtml::types::XhtmlParamType>,
            ),
            XhtmlFlowMix(
                ::std::boxed::Box<crate::xhtml_schema::xhtml::groups::XhtmlFlowMix>,
            ),
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
    pub struct XhtmlObjectContent {
        #[xvalue(default)]
        pub child_0: ::std::vec::Vec<xhtml_object_content_items::Child0>,
    }
    pub mod xhtml_ol_content_items {
        impl ::core::convert::From<crate::xhtml_schema::xhtml::types::XhtmlLiType>
        for Li {
            fn from(value: crate::xhtml_schema::xhtml::types::XhtmlLiType) -> Self {
                Li(::std::boxed::Box::new(value))
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
        pub struct Li(
            #[xgroup]
            pub ::std::boxed::Box<crate::xhtml_schema::xhtml::types::XhtmlLiType>,
        );
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xvalue(order = "strict")]
    pub struct XhtmlOlContent {
        #[xvalue(default)]
        pub li: ::std::vec::Vec<xhtml_ol_content_items::Li>,
    }
    pub mod xhtml_optgroup_content_items {
        impl ::core::convert::From<crate::xhtml_schema::xhtml::types::XhtmlOptionType>
        for Option {
            fn from(value: crate::xhtml_schema::xhtml::types::XhtmlOptionType) -> Self {
                Option(::std::boxed::Box::new(value))
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
        pub struct Option(
            #[xgroup]
            pub ::std::boxed::Box<crate::xhtml_schema::xhtml::types::XhtmlOptionType>,
        );
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xvalue(order = "strict")]
    pub struct XhtmlOptgroupContent {
        #[xvalue(default)]
        pub option: ::std::vec::Vec<xhtml_optgroup_content_items::Option>,
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xvalue(order = "strict")]
    pub struct XhtmlOptionContent;
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xvalue(order = "strict")]
    pub struct XhtmlPContent {
        #[xvalue(default)]
        pub xhtml_inline_mix: ::std::vec::Vec<
            crate::xhtml_schema::xhtml::groups::XhtmlInlineMix,
        >,
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xvalue(order = "strict")]
    pub struct XhtmlParamContent;
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xvalue(order = "strict")]
    pub struct XhtmlPreContent {
        #[xvalue(default)]
        pub xhtml_inline_pre_mix: ::std::vec::Vec<
            crate::xhtml_schema::xhtml::groups::XhtmlInlinePreMix,
        >,
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xvalue(order = "strict")]
    pub struct XhtmlQContent {
        #[xvalue(default)]
        pub xhtml_inline_mix: ::std::vec::Vec<
            crate::xhtml_schema::xhtml::groups::XhtmlInlineMix,
        >,
    }
    pub mod xhtml_ruby_content_simple_items {
        pub mod child_1_variants {
            #[derive(
                ::core::fmt::Debug,
                ::xmlity::Serialize,
                ::xmlity::Deserialize,
                ::core::cmp::PartialEq,
                ::core::clone::Clone
            )]
            #[xvalue(order = "strict")]
            pub struct Variant1 {
                #[xelement(
                    name = "rp",
                    namespace = "http://www.w3.org/1999/xhtml",
                    group
                )]
                pub rp: ::std::boxed::Box<
                    crate::xhtml_schema::xhtml::types::XhtmlRpType,
                >,
                #[xelement(
                    name = "rt",
                    namespace = "http://www.w3.org/1999/xhtml",
                    group
                )]
                pub rt: ::std::boxed::Box<
                    crate::xhtml_schema::xhtml::types::XhtmlRtType,
                >,
                #[xelement(
                    name = "rp",
                    namespace = "http://www.w3.org/1999/xhtml",
                    group
                )]
                pub rp_0: ::std::boxed::Box<
                    crate::xhtml_schema::xhtml::types::XhtmlRpType,
                >,
            }
        }
        impl ::core::convert::From<crate::xhtml_schema::xhtml::types::XhtmlRtType>
        for Child1 {
            fn from(value: crate::xhtml_schema::xhtml::types::XhtmlRtType) -> Self {
                Child1::Rt(::std::boxed::Box::new(value))
            }
        }
        impl ::core::convert::From<child_1_variants::Variant1> for Child1 {
            fn from(value: child_1_variants::Variant1) -> Self {
                Child1::Variant1(::std::boxed::Box::new(value))
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
            #[xelement(
                name = "rt",
                namespace = "http://www.w3.org/1999/xhtml",
                allow_unknown_attributes = "any"
            )]
            Rt(
                #[xgroup]
                ::std::boxed::Box<crate::xhtml_schema::xhtml::types::XhtmlRtType>,
            ),
            Variant1(::std::boxed::Box<child_1_variants::Variant1>),
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
    pub struct XhtmlRubyContentSimple {
        #[xelement(name = "rb", namespace = "http://www.w3.org/1999/xhtml", group)]
        pub rb: ::std::boxed::Box<crate::xhtml_schema::xhtml::types::XhtmlRbType>,
        pub child_1: xhtml_ruby_content_simple_items::Child1,
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xvalue(order = "strict")]
    pub struct XhtmlSampContent {
        #[xvalue(default)]
        pub xhtml_inline_mix: ::std::vec::Vec<
            crate::xhtml_schema::xhtml::groups::XhtmlInlineMix,
        >,
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xvalue(order = "strict")]
    pub struct XhtmlScriptContent;
    pub mod xhtml_select_content_items {
        impl ::core::convert::From<crate::xhtml_schema::xhtml::types::XhtmlOptgroupType>
        for Child0 {
            fn from(
                value: crate::xhtml_schema::xhtml::types::XhtmlOptgroupType,
            ) -> Self {
                Child0::Optgroup(::std::boxed::Box::new(value))
            }
        }
        impl ::core::convert::From<crate::xhtml_schema::xhtml::types::XhtmlOptionType>
        for Child0 {
            fn from(value: crate::xhtml_schema::xhtml::types::XhtmlOptionType) -> Self {
                Child0::Option(::std::boxed::Box::new(value))
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
            #[xelement(
                name = "optgroup",
                namespace = "http://www.w3.org/1999/xhtml",
                allow_unknown_attributes = "any"
            )]
            Optgroup(
                #[xgroup]
                ::std::boxed::Box<crate::xhtml_schema::xhtml::types::XhtmlOptgroupType>,
            ),
            #[xelement(
                name = "option",
                namespace = "http://www.w3.org/1999/xhtml",
                allow_unknown_attributes = "any"
            )]
            Option(
                #[xgroup]
                ::std::boxed::Box<crate::xhtml_schema::xhtml::types::XhtmlOptionType>,
            ),
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
    pub struct XhtmlSelectContent {
        #[xvalue(default)]
        pub child_0: ::std::vec::Vec<xhtml_select_content_items::Child0>,
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xvalue(order = "strict")]
    pub struct XhtmlSpanContent {
        #[xvalue(default)]
        pub xhtml_inline_mix: ::std::vec::Vec<
            crate::xhtml_schema::xhtml::groups::XhtmlInlineMix,
        >,
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xvalue(order = "strict")]
    pub struct XhtmlStrongContent {
        #[xvalue(default)]
        pub xhtml_inline_mix: ::std::vec::Vec<
            crate::xhtml_schema::xhtml::groups::XhtmlInlineMix,
        >,
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xvalue(order = "strict")]
    pub struct XhtmlStyleContent;
    pub mod xhtml_table_content_items {
        pub mod child_1_variants {
            impl ::core::convert::From<crate::xhtml_schema::xhtml::types::XhtmlColType>
            for Col {
                fn from(value: crate::xhtml_schema::xhtml::types::XhtmlColType) -> Self {
                    Col(::std::boxed::Box::new(value))
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
            pub struct Col(
                #[xgroup]
                pub ::std::boxed::Box<crate::xhtml_schema::xhtml::types::XhtmlColType>,
            );
            impl ::core::convert::From<
                crate::xhtml_schema::xhtml::types::XhtmlColgroupType,
            > for Colgroup {
                fn from(
                    value: crate::xhtml_schema::xhtml::types::XhtmlColgroupType,
                ) -> Self {
                    Colgroup(::std::boxed::Box::new(value))
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
                allow_unknown_attributes = "any"
            )]
            pub struct Colgroup(
                #[xgroup]
                pub ::std::boxed::Box<
                    crate::xhtml_schema::xhtml::types::XhtmlColgroupType,
                >,
            );
        }
        impl ::core::convert::From<::std::vec::Vec<child_1_variants::Col>> for Child1 {
            fn from(value: ::std::vec::Vec<child_1_variants::Col>) -> Self {
                Child1::Col(value)
            }
        }
        impl ::core::convert::From<::std::vec::Vec<child_1_variants::Colgroup>> for Child1 {
            fn from(value: ::std::vec::Vec<child_1_variants::Colgroup>) -> Self {
                Child1::Colgroup(value)
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
            Col(#[xvalue(default)] ::std::vec::Vec<child_1_variants::Col>),
            Colgroup(#[xvalue(default)] ::std::vec::Vec<child_1_variants::Colgroup>),
        }
        pub mod child_2_variants {
            pub mod variant_0_items {
                impl ::core::convert::From<
                    crate::xhtml_schema::xhtml::types::XhtmlTbodyType,
                > for Tbody {
                    fn from(
                        value: crate::xhtml_schema::xhtml::types::XhtmlTbodyType,
                    ) -> Self {
                        Tbody(::std::boxed::Box::new(value))
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
                    allow_unknown_attributes = "any"
                )]
                pub struct Tbody(
                    #[xgroup]
                    pub ::std::boxed::Box<
                        crate::xhtml_schema::xhtml::types::XhtmlTbodyType,
                    >,
                );
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
                #[xelement(
                    name = "thead",
                    namespace = "http://www.w3.org/1999/xhtml",
                    group,
                    optional
                )]
                pub thead: ::core::option::Option<
                    ::std::boxed::Box<crate::xhtml_schema::xhtml::types::XhtmlTheadType>,
                >,
                #[xelement(
                    name = "tfoot",
                    namespace = "http://www.w3.org/1999/xhtml",
                    group,
                    optional
                )]
                pub tfoot: ::core::option::Option<
                    ::std::boxed::Box<crate::xhtml_schema::xhtml::types::XhtmlTfootType>,
                >,
                #[xvalue(default)]
                pub tbody: ::std::vec::Vec<variant_0_items::Tbody>,
            }
            pub mod variant_1_items {
                impl ::core::convert::From<
                    crate::xhtml_schema::xhtml::types::XhtmlTrType,
                > for Tr {
                    fn from(
                        value: crate::xhtml_schema::xhtml::types::XhtmlTrType,
                    ) -> Self {
                        Tr(::std::boxed::Box::new(value))
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
                pub struct Tr(
                    #[xgroup]
                    pub ::std::boxed::Box<crate::xhtml_schema::xhtml::types::XhtmlTrType>,
                );
            }
            #[derive(
                ::core::fmt::Debug,
                ::xmlity::Serialize,
                ::xmlity::Deserialize,
                ::core::cmp::PartialEq,
                ::core::clone::Clone
            )]
            #[xvalue(order = "strict")]
            pub struct Tr {
                #[xvalue(default)]
                pub tr: ::std::vec::Vec<variant_1_items::Tr>,
            }
        }
        impl ::core::convert::From<child_2_variants::Variant0> for Child2 {
            fn from(value: child_2_variants::Variant0) -> Self {
                Child2::Variant0(::std::boxed::Box::new(value))
            }
        }
        impl ::core::convert::From<child_2_variants::Tr> for Child2 {
            fn from(value: child_2_variants::Tr) -> Self {
                Child2::Tr(::std::boxed::Box::new(value))
            }
        }
        #[derive(
            ::core::fmt::Debug,
            ::xmlity::Serialize,
            ::xmlity::Deserialize,
            ::core::cmp::PartialEq,
            ::core::clone::Clone
        )]
        pub enum Child2 {
            Variant0(::std::boxed::Box<child_2_variants::Variant0>),
            Tr(::std::boxed::Box<child_2_variants::Tr>),
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
    pub struct XhtmlTableContent {
        #[xelement(
            name = "caption",
            namespace = "http://www.w3.org/1999/xhtml",
            group,
            optional
        )]
        pub caption: ::core::option::Option<
            ::std::boxed::Box<crate::xhtml_schema::xhtml::types::XhtmlCaptionType>,
        >,
        pub child_1: xhtml_table_content_items::Child1,
        pub child_2: xhtml_table_content_items::Child2,
    }
    pub mod xhtml_tbody_content_items {
        impl ::core::convert::From<crate::xhtml_schema::xhtml::types::XhtmlTrType>
        for Tr {
            fn from(value: crate::xhtml_schema::xhtml::types::XhtmlTrType) -> Self {
                Tr(::std::boxed::Box::new(value))
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
        pub struct Tr(
            #[xgroup]
            pub ::std::boxed::Box<crate::xhtml_schema::xhtml::types::XhtmlTrType>,
        );
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xvalue(order = "strict")]
    pub struct XhtmlTbodyContent {
        #[xvalue(default)]
        pub tr: ::std::vec::Vec<xhtml_tbody_content_items::Tr>,
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xvalue(order = "strict")]
    pub struct XhtmlTdContent {
        #[xvalue(default)]
        pub xhtml_flow_mix: ::std::vec::Vec<
            crate::xhtml_schema::xhtml::groups::XhtmlFlowMix,
        >,
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xvalue(order = "strict")]
    pub struct XhtmlTextareaContent;
    pub mod xhtml_tfoot_content_items {
        impl ::core::convert::From<crate::xhtml_schema::xhtml::types::XhtmlTrType>
        for Tr {
            fn from(value: crate::xhtml_schema::xhtml::types::XhtmlTrType) -> Self {
                Tr(::std::boxed::Box::new(value))
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
        pub struct Tr(
            #[xgroup]
            pub ::std::boxed::Box<crate::xhtml_schema::xhtml::types::XhtmlTrType>,
        );
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xvalue(order = "strict")]
    pub struct XhtmlTfootContent {
        #[xvalue(default)]
        pub tr: ::std::vec::Vec<xhtml_tfoot_content_items::Tr>,
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xvalue(order = "strict")]
    pub struct XhtmlThContent {
        #[xvalue(default)]
        pub xhtml_flow_mix: ::std::vec::Vec<
            crate::xhtml_schema::xhtml::groups::XhtmlFlowMix,
        >,
    }
    pub mod xhtml_thead_content_items {
        impl ::core::convert::From<crate::xhtml_schema::xhtml::types::XhtmlTrType>
        for Tr {
            fn from(value: crate::xhtml_schema::xhtml::types::XhtmlTrType) -> Self {
                Tr(::std::boxed::Box::new(value))
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
        pub struct Tr(
            #[xgroup]
            pub ::std::boxed::Box<crate::xhtml_schema::xhtml::types::XhtmlTrType>,
        );
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xvalue(order = "strict")]
    pub struct XhtmlTheadContent {
        #[xvalue(default)]
        pub tr: ::std::vec::Vec<xhtml_thead_content_items::Tr>,
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xvalue(order = "strict")]
    pub struct XhtmlTitleContent;
    pub mod xhtml_tr_content_items {
        impl ::core::convert::From<crate::xhtml_schema::xhtml::types::XhtmlThType>
        for Child0 {
            fn from(value: crate::xhtml_schema::xhtml::types::XhtmlThType) -> Self {
                Child0::Th(::std::boxed::Box::new(value))
            }
        }
        impl ::core::convert::From<crate::xhtml_schema::xhtml::types::XhtmlTdType>
        for Child0 {
            fn from(value: crate::xhtml_schema::xhtml::types::XhtmlTdType) -> Self {
                Child0::Td(::std::boxed::Box::new(value))
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
            #[xelement(
                name = "th",
                namespace = "http://www.w3.org/1999/xhtml",
                allow_unknown_attributes = "any"
            )]
            Th(
                #[xgroup]
                ::std::boxed::Box<crate::xhtml_schema::xhtml::types::XhtmlThType>,
            ),
            #[xelement(
                name = "td",
                namespace = "http://www.w3.org/1999/xhtml",
                allow_unknown_attributes = "any"
            )]
            Td(
                #[xgroup]
                ::std::boxed::Box<crate::xhtml_schema::xhtml::types::XhtmlTdType>,
            ),
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
    pub struct XhtmlTrContent {
        #[xvalue(default)]
        pub child_0: ::std::vec::Vec<xhtml_tr_content_items::Child0>,
    }
    pub mod xhtml_ul_content_items {
        impl ::core::convert::From<crate::xhtml_schema::xhtml::types::XhtmlLiType>
        for Li {
            fn from(value: crate::xhtml_schema::xhtml::types::XhtmlLiType) -> Self {
                Li(::std::boxed::Box::new(value))
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
        pub struct Li(
            #[xgroup]
            pub ::std::boxed::Box<crate::xhtml_schema::xhtml::types::XhtmlLiType>,
        );
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xvalue(order = "strict")]
    pub struct XhtmlUlContent {
        #[xvalue(default)]
        pub li: ::std::vec::Vec<xhtml_ul_content_items::Li>,
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xvalue(order = "strict")]
    pub struct XhtmlVarContent {
        #[xvalue(default)]
        pub xhtml_inline_mix: ::std::vec::Vec<
            crate::xhtml_schema::xhtml::groups::XhtmlInlineMix,
        >,
    }
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
pub enum Html {
    #[xelement(
        name = "html",
        namespace = "http://www.w3.org/1999/xhtml",
        allow_unknown_attributes = "any",
        allow_unknown_children = "none"
    )]
    Html(#[xgroup] ::std::boxed::Box<crate::xhtml_schema::xhtml::types::XhtmlHtmlType>),
    Dynamic(::xmlity_ns::SubstitutionGroup<crate::xhtml_schema::xhtml::Html>),
}
impl ::core::convert::From<crate::xhtml_schema::xhtml::types::XhtmlHtmlType> for Html {
    fn from(value: crate::xhtml_schema::xhtml::types::XhtmlHtmlType) -> Self {
        Html::Html(::std::boxed::Box::new(value))
    }
}
impl ::core::convert::From<
    ::xmlity_ns::SubstitutionGroup<crate::xhtml_schema::xhtml::Html>,
> for Html {
    fn from(
        value: ::xmlity_ns::SubstitutionGroup<crate::xhtml_schema::xhtml::Html>,
    ) -> Self {
        Html::Dynamic(value)
    }
}
