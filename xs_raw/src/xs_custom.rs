use core::fmt;
use std::{borrow::Cow, fmt::Display, str::FromStr};

use xmlity::{
    de::{self, NamespaceContext},
    types::string::FromStrVisitor,
    Deserialize, ExpandedName, LocalName, Prefix, Serialize, XmlNamespace,
};

use crate::xs;

#[derive(Debug, Clone, PartialEq, derive_more::Display, derive_more::Error, derive_more::From)]
pub enum Error {
    ParseIntError(#[error(source)] std::num::ParseIntError),
    ParseFloatError(#[error(source)] std::num::ParseFloatError),
    #[display("Unknown or invalid value: {value}")]
    UnknownOrInvalidValue {
        value: String,
    },
}

struct RefContext<'a, C: NamespaceContext> {
    ctx: &'a C,
}

impl<'a, C: NamespaceContext> RefContext<'a, C> {
    fn new(ctx: &'a C) -> Self {
        Self { ctx }
    }
}

impl<C: NamespaceContext> de::NamespaceContext for RefContext<'_, C> {
    fn default_namespace(&self) -> Option<XmlNamespace<'_>> {
        self.ctx.default_namespace()
    }

    fn resolve_prefix(&self, prefix: Prefix<'_>) -> Option<XmlNamespace<'_>> {
        self.ctx.resolve_prefix(prefix)
    }
}

struct SubStrDeserializer<'a, 'c, C: NamespaceContext + 'c, E: xmlity::de::Error> {
    bytes: &'a str,
    ctx: &'c C,
    _marker: std::marker::PhantomData<E>,
}

impl<'a, 'c, C: NamespaceContext + 'c, E: xmlity::de::Error> SubStrDeserializer<'a, 'c, C, E> {
    fn new(bytes: &'a str, ctx: &'c C) -> Self {
        Self {
            bytes,
            ctx,
            _marker: std::marker::PhantomData,
        }
    }
}

impl<'de, 'c, C: NamespaceContext, E: xmlity::de::Error> de::XmlText<'de>
    for SubStrDeserializer<'de, 'c, C, E>
{
    type NamespaceContext<'a>
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

    fn namespace_context(&self) -> Self::NamespaceContext<'_> {
        RefContext::new(self.ctx)
    }
}

impl<'de, 'c, C: NamespaceContext, E: xmlity::de::Error> de::Deserializer<'de>
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
                    let ctx = value.namespace_context();

                    let mut qname_parts = value.as_str().split(":");
                    let first_part = qname_parts.next().expect("Always has at least one part.");
                    let last_part = qname_parts.next();

                    let expanded_name = match last_part {
                        Some(last_part) => {
                            let prefix = Prefix::new(first_part).unwrap();
                            let local_name = LocalName::new(last_part).unwrap().into_owned();

                            let namespace = ctx.resolve_prefix(prefix).unwrap().into_owned();

                            ExpandedName::new(local_name, Some(namespace))
                        }
                        None => {
                            let local_name = LocalName::new(first_part).unwrap().into_owned();

                            let default_namespace =
                                ctx.default_namespace().map(XmlNamespace::into_owned);

                            return Ok(QName(ExpandedName::new(local_name, default_namespace)));
                        }
                    };

                    Ok(QName(expanded_name))
                }
            }

            reader.deserialize_any(QNameVisitor)
        }
    }

    impl fmt::Display for QName {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            //TODO
            write!(f, "{}", self.0)
        }
    }

    impl Serialize for QName {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: xmlity::Serializer,
        {
            //TODO: Serialize with prefix in the future.
            serializer.serialize_text(&self.0.to_string())
        }
    }

    /// Represents the maximum occurrence of types or elements
    #[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
    pub enum MaxOccursValue {
        /// The occurrence is unbounded.
        Unbounded,

        /// The occurrence is bound to the specified limit.
        Bounded(usize),
    }

    impl Default for MaxOccursValue {
        fn default() -> Self {
            Self::Bounded(1)
        }
    }

    impl FromStr for MaxOccursValue {
        type Err = Error;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s {
                "unbounded" => Ok(Self::Unbounded),
                _ => Ok(Self::Bounded(usize::from_str(s)?)),
            }
        }
    }

    impl Display for MaxOccursValue {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Self::Unbounded => write!(f, "unbounded"),
                Self::Bounded(n) => write!(f, "{n}"),
            }
        }
    }

    impl_to_string_serialize!(MaxOccursValue);
    impl_from_str_deserialize!(MaxOccursValue);

    #[derive(Debug, Clone, Eq, PartialEq)]
    pub enum FullDerivationSetType {
        All,
        TypeDerivationControlList(TypeDerivationControlList),
    }
    impl FromStr for FullDerivationSetType {
        type Err = Error;

        fn from_str(bytes: &str) -> Result<Self, Self::Err> {
            match bytes {
                "#all" => Ok(Self::All),
                bytes => Ok(Self::TypeDerivationControlList(
                    TypeDerivationControlList::from_str(bytes)?,
                )),
            }
        }
    }

    impl Display for FullDerivationSetType {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::All => write!(f, "#all"),
                Self::TypeDerivationControlList(tdctl) => write!(f, "{tdctl}"),
            }
        }
    }

    impl_from_str_deserialize!(FullDerivationSetType);
    impl_to_string_serialize!(FullDerivationSetType);

    #[derive(Debug, Clone, Eq, PartialEq, Default)]
    pub struct TypeDerivationControlList(pub Vec<TypeDerivationControlType>);
    impl FromStr for TypeDerivationControlList {
        type Err = Error;

        fn from_str(bytes: &str) -> Result<Self, Self::Err> {
            Ok(Self(
                bytes
                    .split([' ', '|', ',', ';'])
                    .map(TypeDerivationControlType::from_str)
                    .collect::<Result<Vec<_>, _>>()?,
            ))
        }
    }

    impl Display for TypeDerivationControlList {
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

    impl_from_str_deserialize!(TypeDerivationControlList);
    impl_to_string_serialize!(TypeDerivationControlList);

    #[derive(Debug, Clone, Eq, PartialEq)]
    pub enum BlockSetType {
        All,
        BlockSetItemList(BlockSetItemList),
    }
    impl FromStr for BlockSetType {
        type Err = Error;

        fn from_str(bytes: &str) -> Result<Self, Self::Err> {
            match bytes {
                "#all" => Ok(Self::All),
                bytes => Ok(Self::BlockSetItemList(BlockSetItemList::from_str(bytes)?)),
            }
        }
    }

    impl Display for BlockSetType {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::All => write!(f, "#all"),
                Self::BlockSetItemList(list) => write!(f, "{}", list),
            }
        }
    }

    impl_from_str_deserialize!(BlockSetType);
    impl_to_string_serialize!(BlockSetType);

    #[derive(Debug, Clone, Eq, PartialEq, Default)]
    pub struct BlockSetItemList(pub Vec<BlockSetItemType>);
    impl FromStr for BlockSetItemList {
        type Err = Error;

        fn from_str(bytes: &str) -> Result<Self, Self::Err> {
            Ok(Self(
                bytes
                    .split([' ', '|', ',', ';'])
                    .map(BlockSetItemType::from_str)
                    .collect::<Result<Vec<_>, _>>()?,
            ))
        }
    }

    impl Display for BlockSetItemList {
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

    impl_from_str_deserialize!(BlockSetItemList);
    impl_to_string_serialize!(BlockSetItemList);

    #[derive(Debug, Clone, Eq, PartialEq)]
    pub enum FormChoiceType {
        Qualified,
        Unqualified,
    }

    impl FromStr for FormChoiceType {
        type Err = Error;

        fn from_str(bytes: &str) -> Result<Self, Self::Err> {
            match bytes {
                "qualified" => Ok(Self::Qualified),
                "unqualified" => Ok(Self::Unqualified),
                bytes => Err(Error::UnknownOrInvalidValue {
                    value: bytes.to_string(),
                }),
            }
        }
    }

    impl Display for FormChoiceType {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "{}",
                match self {
                    Self::Qualified => "qualified",
                    Self::Unqualified => "unqualified",
                }
            )
        }
    }

    impl_from_str_deserialize!(FormChoiceType);
    impl_to_string_serialize!(FormChoiceType);

    #[derive(Debug, Clone, Eq, PartialEq)]
    pub enum XpathDefaultNamespaceType {
        String(String),
        DefaultNamespace,
        TargetNamespace,
        Local,
    }
    impl FromStr for XpathDefaultNamespaceType {
        type Err = Error;

        fn from_str(bytes: &str) -> Result<Self, Self::Err> {
            match bytes {
                "##defaultNamespace" => Ok(Self::DefaultNamespace),
                "##targetNamespace" => Ok(Self::TargetNamespace),
                "##local" => Ok(Self::Local),
                bytes => Ok(Self::String(bytes.to_string())),
            }
        }
    }

    impl Display for XpathDefaultNamespaceType {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "{}",
                match self {
                    Self::String(s) => s,
                    Self::DefaultNamespace => "##defaultNamespace",
                    Self::TargetNamespace => "##targetNamespace",
                    Self::Local => "##local",
                }
            )
        }
    }

    impl_from_str_deserialize!(XpathDefaultNamespaceType);
    impl_to_string_serialize!(XpathDefaultNamespaceType);

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

    #[derive(Debug, Clone, Eq, PartialEq)]
    pub enum TypeDerivationControlType {
        Extension,
        Restriction,
        List,
        Union,
    }
    impl FromStr for TypeDerivationControlType {
        type Err = Error;

        fn from_str(bytes: &str) -> Result<Self, Self::Err> {
            match bytes {
                "extension" => Ok(Self::Extension),
                "restriction" => Ok(Self::Restriction),
                "list" => Ok(Self::List),
                "union" => Ok(Self::Union),
                bytes => Err(Error::UnknownOrInvalidValue {
                    value: bytes.to_string(),
                }),
            }
        }
    }

    impl Display for TypeDerivationControlType {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Extension => write!(f, "extension"),
                Self::Restriction => write!(f, "restriction"),
                Self::List => write!(f, "list"),
                Self::Union => write!(f, "union"),
            }
        }
    }

    impl_from_str_deserialize!(TypeDerivationControlType);
    impl_to_string_serialize!(TypeDerivationControlType);

    #[derive(Debug, Clone, Eq, PartialEq)]
    pub enum BlockSetItemType {
        Extension,
        Restriction,
        Substitution,
    }
    impl FromStr for BlockSetItemType {
        type Err = Error;

        fn from_str(bytes: &str) -> Result<Self, Self::Err> {
            match bytes {
                "extension" => Ok(Self::Extension),
                "restriction" => Ok(Self::Restriction),
                "substitution" => Ok(Self::Substitution),
                bytes => Err(Error::UnknownOrInvalidValue {
                    value: bytes.to_string(),
                }),
            }
        }
    }

    impl Display for BlockSetItemType {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Extension => write!(f, "extension"),
                Self::Restriction => write!(f, "restriction"),
                Self::Substitution => write!(f, "substitution"),
            }
        }
    }

    impl_from_str_deserialize!(BlockSetItemType);
    impl_to_string_serialize!(BlockSetItemType);

    #[derive(Debug, Clone, Eq, PartialEq)]
    pub enum DefaultOpenContentModeType {
        Interleave,
        Suffix,
    }
    impl FromStr for DefaultOpenContentModeType {
        type Err = Error;

        fn from_str(bytes: &str) -> Result<Self, Self::Err> {
            match bytes {
                "interleave" => Ok(Self::Interleave),
                "suffix" => Ok(Self::Suffix),
                bytes => Err(Error::UnknownOrInvalidValue {
                    value: bytes.to_string(),
                }),
            }
        }
    }

    impl Display for DefaultOpenContentModeType {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Interleave => write!(f, "interleave"),
                Self::Suffix => write!(f, "suffix"),
            }
        }
    }

    impl_from_str_deserialize!(DefaultOpenContentModeType);
    impl_to_string_serialize!(DefaultOpenContentModeType);

    #[derive(Debug, Clone, Eq, PartialEq)]
    pub enum SimpleDerivationSetType {
        All,
        SimpleDerivationSetItemList(SimpleDerivationSetItemList),
    }
    impl FromStr for SimpleDerivationSetType {
        type Err = Error;

        fn from_str(bytes: &str) -> Result<Self, Self::Err> {
            match bytes {
                "#all" => Ok(Self::All),
                bytes => Ok(Self::SimpleDerivationSetItemList(
                    SimpleDerivationSetItemList::from_str(bytes)?,
                )),
            }
        }
    }

    impl Display for SimpleDerivationSetType {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::All => f.write_str("#all"),
                Self::SimpleDerivationSetItemList(list) => list.fmt(f),
            }
        }
    }

    impl_from_str_deserialize!(SimpleDerivationSetType);
    impl_to_string_serialize!(SimpleDerivationSetType);

    #[derive(Debug, Clone, Eq, PartialEq)]
    pub enum DerivationSetType {
        All,
        ReducedDerivationControlList(ReducedDerivationControlList),
    }
    impl FromStr for DerivationSetType {
        type Err = Error;

        fn from_str(bytes: &str) -> Result<Self, Self::Err> {
            match bytes {
                "#all" => Ok(Self::All),
                bytes => Ok(Self::ReducedDerivationControlList(
                    ReducedDerivationControlList::from_str(bytes)?,
                )),
            }
        }
    }

    impl fmt::Display for DerivationSetType {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Self::All => write!(f, "#all"),
                Self::ReducedDerivationControlList(inner) => write!(f, "{}", inner),
            }
        }
    }

    impl_from_str_deserialize!(DerivationSetType);
    impl_to_string_serialize!(DerivationSetType);

    #[derive(Debug, Clone, Eq, PartialEq, Default)]
    pub struct ElementSubstitutionGroupType(pub Vec<QName>);

    impl Display for ElementSubstitutionGroupType {
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

    impl<'de> Deserialize<'de> for ElementSubstitutionGroupType {
        fn deserialize<D: xmlity::Deserializer<'de>>(reader: D) -> Result<Self, D::Error> {
            struct MyVisitor;

            impl<'de> xmlity::de::Visitor<'de> for MyVisitor {
                type Value = ElementSubstitutionGroupType;
                fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                    formatter.write_str("a ElementSubstitutionGroupType")
                }

                fn visit_text<E, V>(self, value: V) -> Result<Self::Value, E>
                where
                    E: xmlity::de::Error,
                    V: xmlity::de::XmlText<'de>,
                {
                    value
                        .as_str()
                        .split([' ', '|', ',', ';'])
                        .map(|s| {
                            QName::deserialize(SubStrDeserializer::<_, E>::new(
                                s,
                                &value.namespace_context(),
                            ))
                        })
                        .collect::<Result<Vec<_>, _>>()
                        .map(ElementSubstitutionGroupType)
                        .map_err(E::custom)
                }
            }

            reader.deserialize_any(MyVisitor)
        }
    }

    impl_to_string_serialize!(ElementSubstitutionGroupType);

    #[derive(Debug, Clone, Eq, PartialEq)]
    pub enum AttributeUseType {
        Prohibited,
        Optional,
        Required,
    }
    impl FromStr for AttributeUseType {
        type Err = Error;

        fn from_str(bytes: &str) -> Result<Self, Self::Err> {
            match bytes {
                "prohibited" => Ok(Self::Prohibited),
                "optional" => Ok(Self::Optional),
                "required" => Ok(Self::Required),
                bytes => Err(Error::UnknownOrInvalidValue {
                    value: bytes.to_string(),
                }),
            }
        }
    }

    impl Display for AttributeUseType {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Prohibited => write!(f, "prohibited"),
                Self::Optional => write!(f, "optional"),
                Self::Required => write!(f, "required"),
            }
        }
    }

    impl_from_str_deserialize!(AttributeUseType);
    impl_to_string_serialize!(AttributeUseType);

    #[derive(Debug, Clone, Eq, PartialEq)]
    pub enum NamespaceListType {
        Any,
        Other,
        BasicNamespaceList(BasicNamespaceListType),
    }
    impl FromStr for NamespaceListType {
        type Err = Error;

        fn from_str(bytes: &str) -> Result<Self, Self::Err> {
            match bytes {
                "##any" => Ok(Self::Any),
                "##other" => Ok(Self::Other),
                bytes => Ok(Self::BasicNamespaceList(BasicNamespaceListType::from_str(
                    bytes,
                )?)),
            }
        }
    }

    impl Display for NamespaceListType {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Any => write!(f, "##any"),
                Self::Other => write!(f, "##other"),
                Self::BasicNamespaceList(list) => write!(f, "{}", list),
            }
        }
    }

    impl_from_str_deserialize!(NamespaceListType);
    impl_to_string_serialize!(NamespaceListType);

    #[derive(Debug, Clone, Eq, PartialEq, Default)]
    pub struct BasicNamespaceListType(pub Vec<BasicNamespaceListItemType>);
    impl FromStr for BasicNamespaceListType {
        type Err = Error;

        fn from_str(bytes: &str) -> Result<Self, Self::Err> {
            Ok(Self(
                bytes
                    .split([' ', '|', ',', ';'])
                    .map(BasicNamespaceListItemType::from_str)
                    .collect::<Result<Vec<_>, _>>()?,
            ))
        }
    }

    impl Display for BasicNamespaceListType {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "{}",
                self.0
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
                    .join(" ")
            )
        }
    }

    impl_from_str_deserialize!(BasicNamespaceListType);
    impl_to_string_serialize!(BasicNamespaceListType);

    #[derive(Debug, Clone, Eq, PartialEq)]
    pub enum ProcessContentsType {
        Skip,
        Lax,
        Strict,
    }
    impl FromStr for ProcessContentsType {
        type Err = Error;

        fn from_str(bytes: &str) -> Result<Self, Self::Err> {
            match bytes {
                "skip" => Ok(Self::Skip),
                "lax" => Ok(Self::Lax),
                "strict" => Ok(Self::Strict),
                bytes => Err(Error::UnknownOrInvalidValue {
                    value: bytes.to_string(),
                }),
            }
        }
    }
    impl Display for ProcessContentsType {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "{}",
                match self {
                    Self::Skip => "skip",
                    Self::Lax => "lax",
                    Self::Strict => "strict",
                }
            )
        }
    }

    impl_from_str_deserialize!(ProcessContentsType);
    impl_to_string_serialize!(ProcessContentsType);

    #[derive(Debug, Clone, Eq, PartialEq, Default)]
    pub struct SimpleDerivationSetItemList(pub Vec<SimpleDerivationSetItemType>);
    impl FromStr for SimpleDerivationSetItemList {
        type Err = Error;

        fn from_str(bytes: &str) -> Result<Self, Self::Err> {
            Ok(Self(
                bytes
                    .split([' ', '|', ',', ';'])
                    .map(SimpleDerivationSetItemType::from_str)
                    .collect::<Result<Vec<_>, _>>()?,
            ))
        }
    }

    impl Display for SimpleDerivationSetItemList {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "{}",
                self.0
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
                    .join(" ")
            )
        }
    }

    impl_from_str_deserialize!(SimpleDerivationSetItemList);
    impl_to_string_serialize!(SimpleDerivationSetItemList);

    #[derive(Debug, Clone, Eq, PartialEq, Default)]
    pub struct ReducedDerivationControlList(pub Vec<ReducedDerivationControlType>);
    impl FromStr for ReducedDerivationControlList {
        type Err = Error;

        fn from_str(bytes: &str) -> Result<Self, Self::Err> {
            Ok(Self(
                bytes
                    .split([' ', '|', ',', ';'])
                    .map(ReducedDerivationControlType::from_str)
                    .collect::<Result<Vec<_>, _>>()?,
            ))
        }
    }

    impl Display for ReducedDerivationControlList {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "{}",
                self.0
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(" ")
            )
        }
    }

    impl_from_str_deserialize!(ReducedDerivationControlList);
    impl_to_string_serialize!(ReducedDerivationControlList);

    #[derive(Debug, Clone, Eq, PartialEq)]
    pub enum OpenContentModeType {
        None,
        Interleave,
        Suffix,
    }
    impl FromStr for OpenContentModeType {
        type Err = Error;

        fn from_str(bytes: &str) -> Result<Self, Self::Err> {
            match bytes {
                "none" => Ok(Self::None),
                "interleave" => Ok(Self::Interleave),
                "suffix" => Ok(Self::Suffix),
                bytes => Err(Error::UnknownOrInvalidValue {
                    value: bytes.to_string(),
                }),
            }
        }
    }

    impl Display for OpenContentModeType {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                OpenContentModeType::None => write!(f, "none"),
                OpenContentModeType::Interleave => write!(f, "interleave"),
                OpenContentModeType::Suffix => write!(f, "suffix"),
            }
        }
    }

    impl_from_str_deserialize!(OpenContentModeType);
    impl_to_string_serialize!(OpenContentModeType);

    #[derive(Debug, Clone, Eq, PartialEq, Default)]
    pub struct QnameListAType(pub Vec<QnameListAItemType>);

    impl Display for QnameListAType {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "{}",
                self.0
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(" ")
            )
        }
    }

    impl<'de> Deserialize<'de> for QnameListAType {
        fn deserialize<D: xmlity::Deserializer<'de>>(reader: D) -> Result<Self, D::Error> {
            struct MyVisitor;

            impl<'de> xmlity::de::Visitor<'de> for MyVisitor {
                type Value = QnameListAType;
                fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                    formatter.write_str("a QnameListAType")
                }

                fn visit_text<E, V>(self, value: V) -> Result<Self::Value, E>
                where
                    E: xmlity::de::Error,
                    V: xmlity::de::XmlText<'de>,
                {
                    value
                        .as_str()
                        .split([' ', '|', ',', ';'])
                        .map(|s| {
                            QnameListAItemType::deserialize(SubStrDeserializer::new(
                                s,
                                &value.namespace_context(),
                            ))
                        })
                        .collect::<Result<Vec<_>, _>>()
                        .map(QnameListAType)
                }
            }

            reader.deserialize_any(MyVisitor)
        }
    }
    impl_to_string_serialize!(QnameListAType);

    #[derive(Debug, Clone, Eq, PartialEq, Default)]
    pub struct QnameListType(pub Vec<QnameListItemType>);

    impl Display for QnameListType {
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

    impl<'de> Deserialize<'de> for QnameListType {
        fn deserialize<D: xmlity::Deserializer<'de>>(reader: D) -> Result<Self, D::Error> {
            struct MyVisitor;

            impl<'de> xmlity::de::Visitor<'de> for MyVisitor {
                type Value = QnameListType;
                fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                    formatter.write_str("a QnameListType")
                }

                fn visit_text<E, V>(self, value: V) -> Result<Self::Value, E>
                where
                    E: xmlity::de::Error,
                    V: xmlity::de::XmlText<'de>,
                {
                    value
                        .as_str()
                        .split([' ', '|', ',', ';'])
                        .map(|s| {
                            QnameListItemType::deserialize(SubStrDeserializer::new(
                                s,
                                &value.namespace_context(),
                            ))
                        })
                        .collect::<Result<Vec<_>, _>>()
                        .map(QnameListType)
                }
            }

            reader.deserialize_any(MyVisitor)
        }
    }
    impl_to_string_serialize!(QnameListType);

    #[derive(Debug, Clone, Eq, PartialEq)]
    pub enum BasicNamespaceListItemType {
        String(String),
        TargetNamespace,
        Local,
    }
    impl FromStr for BasicNamespaceListItemType {
        type Err = Error;

        fn from_str(bytes: &str) -> Result<Self, Self::Err> {
            match bytes {
                "##targetNamespace" => Ok(Self::TargetNamespace),
                "##local" => Ok(Self::Local),
                bytes => Ok(Self::String(bytes.to_string())),
            }
        }
    }

    impl Display for BasicNamespaceListItemType {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::String(string) => write!(f, "{}", string),
                Self::TargetNamespace => write!(f, "##targetNamespace"),
                Self::Local => write!(f, "##local"),
            }
        }
    }

    impl_from_str_deserialize!(BasicNamespaceListItemType);
    impl_to_string_serialize!(BasicNamespaceListItemType);

    #[derive(Debug, Clone, Eq, PartialEq)]
    pub enum SimpleDerivationSetItemType {
        List,
        Union,
        Restriction,
        Extension,
    }
    impl FromStr for SimpleDerivationSetItemType {
        type Err = Error;

        fn from_str(bytes: &str) -> Result<Self, Self::Err> {
            match bytes {
                "list" => Ok(Self::List),
                "union" => Ok(Self::Union),
                "restriction" => Ok(Self::Restriction),
                "extension" => Ok(Self::Extension),
                bytes => Err(Error::UnknownOrInvalidValue {
                    value: bytes.to_string(),
                }),
            }
        }
    }

    impl Display for SimpleDerivationSetItemType {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::List => write!(f, "list"),
                Self::Union => write!(f, "union"),
                Self::Restriction => write!(f, "restriction"),
                Self::Extension => write!(f, "extension"),
            }
        }
    }

    impl_from_str_deserialize!(SimpleDerivationSetItemType);
    impl_to_string_serialize!(SimpleDerivationSetItemType);

    #[derive(Debug, Clone, Eq, PartialEq)]
    pub enum ReducedDerivationControlType {
        Extension,
        Restriction,
    }
    impl FromStr for ReducedDerivationControlType {
        type Err = Error;

        fn from_str(bytes: &str) -> Result<Self, Self::Err> {
            match bytes {
                "extension" => Ok(Self::Extension),
                "restriction" => Ok(Self::Restriction),
                bytes => Err(Error::UnknownOrInvalidValue {
                    value: bytes.to_string(),
                }),
            }
        }
    }

    impl Display for ReducedDerivationControlType {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Extension => write!(f, "extension"),
                Self::Restriction => write!(f, "restriction"),
            }
        }
    }

    impl_from_str_deserialize!(ReducedDerivationControlType);
    impl_to_string_serialize!(ReducedDerivationControlType);

    #[derive(Debug, Clone, Eq, PartialEq)]
    pub enum QnameListAItemType {
        Qname(QName),
        Defined,
    }

    impl Display for QnameListAItemType {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Qname(qname) => write!(f, "{qname}"),
                Self::Defined => write!(f, "##defined"),
            }
        }
    }

    impl<'de> Deserialize<'de> for QnameListAItemType {
        fn deserialize<D: xmlity::Deserializer<'de>>(reader: D) -> Result<Self, D::Error> {
            struct MyVisitor;

            impl<'de> xmlity::de::Visitor<'de> for MyVisitor {
                type Value = QnameListAItemType;
                fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                    formatter.write_str("a QnameListAItemType")
                }

                fn visit_text<E, V>(self, value: V) -> Result<Self::Value, E>
                where
                    E: xmlity::de::Error,
                    V: xmlity::de::XmlText<'de>,
                {
                    match value.as_str() {
                        "##defined" => Ok(QnameListAItemType::Defined),
                        s => QName::deserialize(SubStrDeserializer::new(
                            s,
                            &value.namespace_context(),
                        ))
                        .map(QnameListAItemType::Qname),
                    }
                }
            }

            reader.deserialize_any(MyVisitor)
        }
    }
    impl_to_string_serialize!(QnameListAItemType);

    #[derive(Debug, Clone, Eq, PartialEq)]
    pub enum QnameListItemType {
        Qname(QName),
        Defined,
        DefinedSibling,
    }

    impl Display for QnameListItemType {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Defined => write!(f, "##defined"),
                Self::DefinedSibling => write!(f, "##definedSibling"),
                Self::Qname(n) => write!(f, "{n}"),
            }
        }
    }

    impl<'de> Deserialize<'de> for QnameListItemType {
        fn deserialize<D: xmlity::Deserializer<'de>>(reader: D) -> Result<Self, D::Error> {
            struct MyVisitor;

            impl<'de> xmlity::de::Visitor<'de> for MyVisitor {
                type Value = QnameListItemType;
                fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                    formatter.write_str("a QnameListItemType")
                }

                fn visit_text<E, V>(self, value: V) -> Result<Self::Value, E>
                where
                    E: xmlity::de::Error,
                    V: xmlity::de::XmlText<'de>,
                {
                    match value.as_str() {
                        "##defined" => Ok(QnameListItemType::Defined),
                        "##definedSibling" => Ok(QnameListItemType::DefinedSibling),
                        s => QName::deserialize(SubStrDeserializer::<_, E>::new(
                            s,
                            &value.namespace_context(),
                        ))
                        .map(QnameListItemType::Qname),
                    }
                }
            }

            reader.deserialize_any(MyVisitor)
        }
    }
    impl_to_string_serialize!(QnameListItemType);
}
pub mod elements {
    use super::*;

    #[derive(Debug, Serialize, Deserialize, PartialEq, derive_more::From)]
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
