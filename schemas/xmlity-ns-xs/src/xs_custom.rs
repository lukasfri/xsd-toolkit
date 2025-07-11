use core::fmt;
use std::{fmt::Display, str::FromStr};

use xmlity::{
    de::DeserializeContext, types::string::FromStrVisitor, Deserialize, ExpandedName, LocalName,
    Prefix, Serialize, XmlNamespace,
};

use crate as xs;

macro_rules! impl_from_str_deserialize {
    ($ty:ty) => {
        impl<'de> Deserialize<'de> for $ty {
            fn deserialize<D: xmlity::Deserializer<'de>>(reader: D) -> Result<Self, D::Error> {
                reader.deserialize_any(FromStrVisitor::default())
            }
        }
    };
    ($($ty:ty),+) => ($(
        impl_from_str_deserialize!($ty);
    )+);
}

macro_rules! impl_to_string_serialize  {
    ($ty:ty) => {
        impl Serialize for $ty {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: xmlity::Serializer,
            {
                serializer.serialize_text(&self.to_string())
            }
        }
    };
    ($($ty:ty),+) => ($(
        impl_to_string_serialize!($ty);
    )+);
}

pub mod types {

    use super::*;

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct QName(pub ExpandedName<'static>);

    impl<'de> Deserialize<'de> for QName {
        fn deserialize<D: xmlity::Deserializer<'de>>(reader: D) -> Result<Self, D::Error> {
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
                    let ctx = value.context();

                    let mut qname_parts = value.as_str().split(":");
                    let first_part = qname_parts.next().expect("Always has at least one part.");
                    let last_part = qname_parts.next();

                    let expanded_name = match last_part {
                        Some(last_part) => {
                            let local_name = LocalName::new(last_part).unwrap().into_owned();

                            let prefix = Prefix::new(first_part).unwrap();
                            let namespace = ctx.resolve_prefix(prefix).unwrap().into_owned();

                            ExpandedName::new(local_name, Some(namespace))
                        }
                        None => {
                            let local_name = LocalName::new(first_part).unwrap().into_owned();

                            let default_namespace =
                                ctx.default_namespace().map(XmlNamespace::into_owned);

                            ExpandedName::new(local_name, default_namespace)
                        }
                    };

                    Ok(QName(expanded_name))
                }
            }

            reader.deserialize_any(QNameVisitor)
        }
    }

    impl Serialize for QName {
        fn serialize<S>(&self, _serializer: S) -> Result<S::Ok, S::Error>
        where
            S: xmlity::Serializer,
        {
            todo!()
        }
    }

    #[derive(Debug, Clone, Eq, PartialEq)]
    pub struct TargetNamespace(pub XmlNamespace<'static>);

    impl Display for TargetNamespace {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    impl FromStr for TargetNamespace {
        type Err = <XmlNamespace<'static> as FromStr>::Err;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Ok(Self(XmlNamespace::from_str(s)?))
        }
    }

    impl_to_string_serialize!(TargetNamespace);
    impl_from_str_deserialize!(TargetNamespace);
}
pub mod elements {
    use super::*;

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, derive_more::From)]
    pub enum Facet {
        MinExclusive(xs::MinExclusive),
        MinInclusive(xs::MinInclusive),
        MaxExclusive(xs::MaxExclusive),
        MaxInclusive(xs::MaxInclusive),
        TotalDigits(xs::TotalDigits),
        FractionDigits(xs::FractionDigits),
        Length(xs::Length),
        MinLength(xs::MinLength),
        MaxLength(xs::MaxLength),
        Enumeration(xs::Enumeration),
        WhiteSpace(xs::WhiteSpace),
        Pattern(xs::Pattern),
        Assertion(xs::Assertion),
        ExplicitTimezone(xs::ExplicitTimezone),
    }
}
