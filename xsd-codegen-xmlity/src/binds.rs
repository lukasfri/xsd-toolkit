use syn::parse_quote;
use xmlity::{ExpandedName, LocalName, XmlNamespace};

use crate::{BoundType, TypeType};

macro_rules! xs_name {
    ($local_name:expr) => {
        ExpandedName::new(
            LocalName::new_dangerous($local_name),
            Some(XmlNamespace::XS),
        )
    };
}

macro_rules! xs_bind {
    ($local_name:expr, default_with = $default_with:expr, $($tt:tt)*) => {
        (xs_name!($local_name), BoundType {
          ty: crate::TypeReference::new_static(parse_quote!($($tt)*)),
          ty_type:
          TypeType::Simple,
          serialize_with: None,
          deserialize_with: None,
          default_with: $default_with,
        })
    };
    ($local_name:expr, $($tt:tt)*) => {
        xs_bind!($local_name, default_with = None, $($tt)*)
    };
}

pub struct PrimitiveStdXsdTypes;

impl IntoIterator for PrimitiveStdXsdTypes {
    type Item = (ExpandedName<'static>, BoundType);
    type IntoIter = std::array::IntoIter<Self::Item, 18>;

    fn into_iter(self) -> Self::IntoIter {
        [
            // Primitives
            xs_bind!(
                "string",
                default_with = Some(parse_quote!(String::new)),
                String
            ),
            xs_bind!(
                "boolean",
                default_with = Some(parse_quote!(std::default::Default::default)),
                bool
            ),
            xs_bind!(
                "decimal",
                default_with = Some(parse_quote!(std::default::Default::default)),
                f64
            ),
            xs_bind!(
                "float",
                default_with = Some(parse_quote!(std::default::Default::default)),
                f32
            ),
            xs_bind!(
                "double",
                default_with = Some(parse_quote!(std::default::Default::default)),
                f64
            ),
            xs_bind!(
                "long",
                default_with = Some(parse_quote!(std::default::Default::default)),
                i64
            ),
            xs_bind!(
                "int",
                default_with = Some(parse_quote!(std::default::Default::default)),
                i32
            ),
            xs_bind!(
                "integer",
                default_with = Some(parse_quote!(std::default::Default::default)),
                i32
            ),
            xs_bind!(
                "short",
                default_with = Some(parse_quote!(std::default::Default::default)),
                i16
            ),
            xs_bind!(
                "byte",
                default_with = Some(parse_quote!(std::default::Default::default)),
                i8
            ),
            xs_bind!(
                "negativeInteger",
                default_with = Some(parse_quote!(std::default::Default::default)),
                isize
            ),
            xs_bind!(
                "nonPositiveInteger",
                default_with = Some(parse_quote!(std::default::Default::default)),
                isize
            ),
            xs_bind!(
                "unsignedLong",
                default_with = Some(parse_quote!(std::default::Default::default)),
                u64
            ),
            xs_bind!(
                "unsignedInt",
                default_with = Some(parse_quote!(std::default::Default::default)),
                u32
            ),
            xs_bind!(
                "unsignedShort",
                default_with = Some(parse_quote!(std::default::Default::default)),
                u16
            ),
            xs_bind!(
                "unsignedByte",
                default_with = Some(parse_quote!(std::default::Default::default)),
                u8
            ),
            xs_bind!(
                "positiveInteger",
                default_with = Some(parse_quote!(std::default::Default::default)),
                usize
            ),
            xs_bind!(
                "nonNegativeInteger",
                default_with = Some(parse_quote!(std::default::Default::default)),
                usize
            ),
        ]
        .into_iter()
    }
}

pub struct NonZeroStdXsdTypes;

impl IntoIterator for NonZeroStdXsdTypes {
    type Item = (ExpandedName<'static>, BoundType);
    type IntoIter = std::array::IntoIter<Self::Item, 1>;

    fn into_iter(self) -> Self::IntoIter {
        [xs_bind!("positiveInteger", ::core::num::NonZeroUsize)].into_iter()
    }
}

pub struct TimeStdXsdTypes;

impl IntoIterator for TimeStdXsdTypes {
    type Item = (ExpandedName<'static>, BoundType);
    type IntoIter = std::array::IntoIter<Self::Item, 9>;

    fn into_iter(self) -> Self::IntoIter {
        [
            xs_bind!("duration", String),
            xs_bind!("dateTime", String),
            xs_bind!("time", String),
            xs_bind!("date", String),
            xs_bind!("gYearMonth", String),
            xs_bind!("gYear", String),
            xs_bind!("gMonthDay", String),
            xs_bind!("gMonth", String),
            xs_bind!("gDay", String),
        ]
        .into_iter()
    }
}

pub struct TimeTimeXsdTypes;

impl IntoIterator for TimeTimeXsdTypes {
    type Item = (ExpandedName<'static>, BoundType);
    type IntoIter = std::array::IntoIter<Self::Item, 9>;

    fn into_iter(self) -> Self::IntoIter {
        [
            xs_bind!("duration", ::time::Duration),
            xs_bind!(
                "dateTime",
                (::time::PrimitiveDateTime, Option<::time::UtcOffset>)
            ),
            xs_bind!("time", ::time::Time),
            xs_bind!("date", ::time::Date),
            xs_bind!("gYearMonth", (i32, ::time::Month)),
            xs_bind!("gYear", i32),
            xs_bind!("gMonthDay", (::time::Month, u8)),
            xs_bind!("gMonth", ::time::Month),
            xs_bind!("gDay", u8),
        ]
        .into_iter()
    }
}

pub struct TimeChronoXsdTypes;

impl IntoIterator for TimeChronoXsdTypes {
    type Item = (ExpandedName<'static>, BoundType);
    type IntoIter = std::array::IntoIter<Self::Item, 9>;

    fn into_iter(self) -> Self::IntoIter {
        [
            xs_bind!("duration", ::chrono::TimeDelta),
            xs_bind!(
                "dateTime",
                (::chrono::NaiveDateTime, Option<::chrono::FixedOffset>)
            ),
            xs_bind!("time", ::chrono::NaiveTime),
            xs_bind!("date", ::chrono::NaiveDate),
            xs_bind!("gYearMonth", (i32, u8)),
            xs_bind!("gYear", i32),
            xs_bind!("gMonthDay", (u8, u8)),
            xs_bind!("gMonth", u8),
            xs_bind!("gDay", u8),
        ]
        .into_iter()
    }
}

pub struct BinaryStdXsdTypes;

impl IntoIterator for BinaryStdXsdTypes {
    type Item = (ExpandedName<'static>, BoundType);
    type IntoIter = std::array::IntoIter<Self::Item, 2>;

    fn into_iter(self) -> Self::IntoIter {
        [
            xs_bind!("hexBinary", String),
            xs_bind!("base64Binary", String),
        ]
        .into_iter()
    }
}

pub struct UrlStdXsdTypes;

impl IntoIterator for UrlStdXsdTypes {
    type Item = (ExpandedName<'static>, BoundType);
    type IntoIter = std::array::IntoIter<Self::Item, 3>;

    fn into_iter(self) -> Self::IntoIter {
        [
            xs_bind!("anyURI", String),
            xs_bind!("QName", String),
            xs_bind!("NOTATION", String),
        ]
        .into_iter()
    }
}

pub struct OtherStdXsdTypes;

impl IntoIterator for OtherStdXsdTypes {
    type Item = (ExpandedName<'static>, BoundType);
    type IntoIter = std::array::IntoIter<Self::Item, 9>;

    fn into_iter(self) -> Self::IntoIter {
        [
            xs_bind!("anySimpleType", String),
            xs_bind!("normalizedString", String),
            xs_bind!("token", String),
            xs_bind!("language", String),
            xs_bind!("NMTOKEN", String),
            xs_bind!("Name", String),
            xs_bind!("NCName", String),
            xs_bind!("ID", String),
            xs_bind!("IDREF", String),
        ]
        .into_iter()
    }
}

pub struct ListStdXsdTypes;

impl IntoIterator for ListStdXsdTypes {
    type Item = (ExpandedName<'static>, BoundType);
    type IntoIter = std::array::IntoIter<Self::Item, 4>;

    fn into_iter(self) -> Self::IntoIter {
        [
            xs_bind!("NMTOKENS", String),
            xs_bind!("IDREFS", String),
            xs_bind!("ENTITY", String),
            xs_bind!("ENTITIES", String),
        ]
        .into_iter()
    }
}

pub struct StdXsdTypes;

impl IntoIterator for StdXsdTypes {
    type Item = (ExpandedName<'static>, BoundType);
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        None.into_iter()
            .chain(PrimitiveStdXsdTypes)
            .chain(NonZeroStdXsdTypes)
            .chain(TimeStdXsdTypes)
            .chain(BinaryStdXsdTypes)
            .chain(UrlStdXsdTypes)
            .chain(OtherStdXsdTypes)
            .chain(ListStdXsdTypes)
            .collect::<Vec<_>>()
            .into_iter()
    }
}
