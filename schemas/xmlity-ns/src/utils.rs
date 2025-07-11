use std::borrow::Cow;

use xmlity::{
    de::{self, DeserializeContext},
    ser, ExpandedName, NoopDeSerializer, Prefix, Serialize, Serializer, XmlNamespace,
};

#[derive(Debug, Clone, PartialEq, derive_more::Display, derive_more::Error, derive_more::From)]
pub enum Error {
    ParseInt(#[error(source)] std::num::ParseIntError),
    ParseFloat(#[error(source)] std::num::ParseFloatError),
    #[display("Unknown or invalid value: {value}")]
    UnknownOrInvalidValue {
        value: String,
    },
}

pub struct RefContext<'a, C: DeserializeContext> {
    ctx: &'a C,
}

impl<'a, C: DeserializeContext> RefContext<'a, C> {
    pub fn new(ctx: &'a C) -> Self {
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

pub struct SubStrDeserializer<'a, 'c, C: DeserializeContext + 'c, E: de::Error> {
    bytes: &'a str,
    ctx: &'c C,
    _marker: std::marker::PhantomData<E>,
}

impl<'a, 'c, C: DeserializeContext + 'c, E: de::Error> SubStrDeserializer<'a, 'c, C, E> {
    pub fn new(bytes: &'a str, ctx: &'c C) -> Self {
        Self {
            bytes,
            ctx,
            _marker: std::marker::PhantomData,
        }
    }
}

impl<'de, 'c, C: DeserializeContext, E: de::Error> de::XmlText<'de>
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

impl<'de, 'c, C: DeserializeContext, E: de::Error> de::Deserializer<'de>
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

pub enum OnceSeqSerializer<S: Serializer> {
    Unused(Option<S>),
    Used(S::Ok),
}

impl<S: Serializer> OnceSeqSerializer<S> {
    fn new(serializer: S) -> Self {
        OnceSeqSerializer::Unused(Some(serializer))
    }
}

impl<S> ser::SerializeSeq for OnceSeqSerializer<S>
where
    S: Serializer,
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
            OnceSeqSerializer::Used(_) => {
                Err(ser::Error::custom("Sequence can only be serialized once."))
            }
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

pub struct SubStringSerializer<E: xmlity::ser::Error> {
    _marker: std::marker::PhantomData<E>,
}

impl<E: xmlity::ser::Error> SubStringSerializer<E> {
    pub fn new() -> Self {
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
