use core::fmt;
use std::{borrow::Cow, fmt::Display, str::FromStr};

use xmlity::{
    de::{self, DeserializeContext},
    types::string::FromStrVisitor,
    Deserialize, ExpandedName, LocalName, NoopDeSerializer, Prefix, Serialize, XmlNamespace,
};

use crate as xs;

#[derive(Debug, Clone, PartialEq, derive_more::Display, derive_more::Error, derive_more::From)]
pub enum Error {
    ParseInt(#[error(source)] std::num::ParseIntError),
    ParseFloat(#[error(source)] std::num::ParseFloatError),
    #[display("Unknown or invalid value: {value}")]
    UnknownOrInvalidValue {
        value: String,
    },
}

struct RefContext<'a, C: DeserializeContext> {
    ctx: &'a C,
}

impl<'a, C: DeserializeContext> RefContext<'a, C> {
    fn new(ctx: &'a C) -> Self {
        Self { ctx }
    }
}

impl<C: DeserializeContext> de::DeserializeContext for RefContext<'_, C> {
    fn default_namespace(&self) -> Option<XmlNamespace<'_>> {
        self.ctx.default_namespace()
    }

    fn resolve_prefix(&self, prefix: Prefix<'_>) -> Option<XmlNamespace<'_>> {
        self.ctx.resolve_prefix(prefix)
    }

    fn external_data<T>(&self) -> Option<&T>
    where
        T: core::any::Any,
    {
        self.ctx.external_data::<T>()
    }
}

struct SubStrDeserializer<'a, 'c, C: DeserializeContext + 'c, E: xmlity::de::Error> {
    bytes: &'a str,
    ctx: &'c C,
    _marker: std::marker::PhantomData<E>,
}

impl<'a, 'c, C: DeserializeContext + 'c, E: xmlity::de::Error> SubStrDeserializer<'a, 'c, C, E> {
    fn new(bytes: &'a str, ctx: &'c C) -> Self {
        Self {
            bytes,
            ctx,
            _marker: std::marker::PhantomData,
        }
    }
}

impl<'de, 'c, C: DeserializeContext, E: xmlity::de::Error> de::XmlText<'de>
    for SubStrDeserializer<'de, 'c, C, E>
{
    type DeserializeContext<'a>
        = RefContext<'c, C>
    where
        Self: 'a;

    fn into_bytes(self) -> Cow<'de, [u8]> {
        Cow::Borrowed(self.bytes.as_bytes())
    }

    fn as_bytes(&self) -> &[u8] {
        self.bytes.as_bytes()
    }

    fn into_string(self) -> Cow<'de, str> {
        Cow::Borrowed(self.bytes)
    }

    fn as_str(&self) -> &str {
        self.bytes
    }

    fn context(&self) -> Self::DeserializeContext<'_> {
        RefContext::new(self.ctx)
    }
}

impl<'de, 'c, C: DeserializeContext, E: xmlity::de::Error> de::Deserializer<'de>
    for SubStrDeserializer<'de, 'c, C, E>
{
    type Error = E;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_text(self)
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_text(self)
    }
}

enum OnceSeqSerializer<S: xmlity::ser::Serializer> {
    Unused(Option<S>),
    Used(S::Ok),
}

impl<S: xmlity::ser::Serializer> OnceSeqSerializer<S> {
    fn new(serializer: S) -> Self {
        OnceSeqSerializer::Unused(Some(serializer))
    }
}

impl<S> xmlity::ser::SerializeSeq for OnceSeqSerializer<S>
where
    S: xmlity::ser::Serializer,
{
    type Ok = S::Ok;

    type Error = S::Error;

    fn serialize_element<V: Serialize>(&mut self, v: &V) -> Result<(), Self::Error> {
        match self {
            OnceSeqSerializer::Unused(serializer) => {
                let result = v.serialize(serializer.take().expect("Should never be None"))?;
                *self = OnceSeqSerializer::Used(result);
                Ok(())
            }
            OnceSeqSerializer::Used(_) => Err(xmlity::ser::Error::custom(
                "Sequence can only be serialized once.",
            )),
        }
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        match self {
            OnceSeqSerializer::Unused(_) => {
                Err(xmlity::ser::Error::custom("Sequence was not serialized."))
            }
            OnceSeqSerializer::Used(result) => Ok(result),
        }
    }
}

struct SubStringSerializer<E: xmlity::ser::Error> {
    _marker: std::marker::PhantomData<E>,
}

impl<E: xmlity::ser::Error> SubStringSerializer<E> {
    fn new() -> Self {
        Self {
            _marker: std::marker::PhantomData,
        }
    }
}

impl<E> xmlity::ser::Serializer for SubStringSerializer<E>
where
    E: xmlity::ser::Error,
{
    type Ok = Option<String>;

    type Error = E;

    type SerializeElement = NoopDeSerializer<Option<String>, E>;

    type SerializeSeq = OnceSeqSerializer<SubStringSerializer<E>>;

    fn serialize_text<S: AsRef<str>>(self, text: S) -> Result<Self::Ok, Self::Error> {
        Ok(Some(text.as_ref().to_owned()))
    }

    fn serialize_cdata<S: AsRef<str>>(self, text: S) -> Result<Self::Ok, Self::Error> {
        Ok(Some(text.as_ref().to_owned()))
    }

    fn serialize_element(
        self,
        _name: &'_ ExpandedName<'_>,
    ) -> Result<Self::SerializeElement, Self::Error> {
        Err(E::custom(
            "SubStringSerializer only supports text serialization",
        ))
    }

    fn serialize_seq(self) -> Result<Self::SerializeSeq, Self::Error> {
        Ok(OnceSeqSerializer::new(self))
    }

    fn serialize_decl<S: AsRef<str>>(
        self,
        _version: S,
        _encoding: Option<S>,
        _standalone: Option<S>,
    ) -> Result<Self::Ok, Self::Error> {
        Err(E::custom(
            "SubStringSerializer only supports text serialization",
        ))
    }

    fn serialize_pi<S: AsRef<[u8]>>(
        self,
        _target: S,
        _content: S,
    ) -> Result<Self::Ok, Self::Error> {
        Err(E::custom(
            "SubStringSerializer only supports text serialization",
        ))
    }

    fn serialize_comment<S: AsRef<[u8]>>(self, _text: S) -> Result<Self::Ok, Self::Error> {
        Err(E::custom(
            "SubStringSerializer only supports text serialization",
        ))
    }

    fn serialize_doctype<S: AsRef<[u8]>>(self, _text: S) -> Result<Self::Ok, Self::Error> {
        Err(E::custom(
            "SubStringSerializer only supports text serialization",
        ))
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Ok(None)
    }
}

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
    use xmlity::DeserializeOwned;

    use super::*;

    #[derive(Debug, Clone, Eq, PartialEq, Default)]
    pub struct List<T>(pub Vec<T>);

    impl<T: Display> Display for List<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "{}",
                self.0
                    .iter()
                    .map(|a| a.to_string())
                    .collect::<Vec<_>>()
                    .join(" ")
            )
        }
    }

    impl<'de, T: DeserializeOwned> Deserialize<'de> for List<T> {
        fn deserialize<D: xmlity::Deserializer<'de>>(reader: D) -> Result<Self, D::Error> {
            struct ListVisitor<T> {
                _marker: std::marker::PhantomData<T>,
            }

            impl<'de, T: DeserializeOwned> xmlity::de::Visitor<'de> for ListVisitor<T> {
                type Value = List<T>;
                fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                    formatter.write_str("a List of items")
                }

                fn visit_text<E, V>(self, value: V) -> Result<Self::Value, E>
                where
                    E: xmlity::de::Error,
                    V: xmlity::de::XmlText<'de>,
                {
                    value
                        .as_str()
                        .split([' ', '|', ',', ';'])
                        .filter(|v| !v.is_empty())
                        .map(|s| T::deserialize(SubStrDeserializer::new(s, &value.context())))
                        .collect::<Result<Vec<_>, _>>()
                        .map(List)
                }
            }

            reader.deserialize_any(ListVisitor {
                _marker: std::marker::PhantomData,
            })
        }
    }

    impl<T: Serialize> Serialize for List<T> {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: xmlity::Serializer,
        {
            let parts = self
                .0
                .iter()
                .map(|item| item.serialize(SubStringSerializer::new()))
                .collect::<Result<Vec<_>, _>>()?;

            let list_string = parts
                .into_iter()
                .flatten()
                .fold(String::new(), |mut acc, item| {
                    if !acc.is_empty() {
                        acc.push(' ');
                    }
                    acc.push_str(&item);
                    acc
                });

            serializer.serialize_text(&list_string)
        }
    }

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
