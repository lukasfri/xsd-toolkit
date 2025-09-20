//! # Binds for XMLity adapter of `xsd_types`
//!
//! TODO: Docs.

use syn::parse_quote;
use xmlity::{ExpandedName, LocalName, XmlNamespace};

// Note: These binds are designed to work with the xsd-codegen-xmlity system
// They reference the wrapper types from buf_types that have proper XSD serialization/deserialization
use xsd_codegen_xmlity::misc::TypeReference;
use xsd_codegen_xmlity::{BoundType, TypeType};

macro_rules! xs_name {
    ($local_name:expr) => {
        ExpandedName::new(
            LocalName::new_dangerous($local_name),
            Some(XmlNamespace::XS),
        )
    };
}

macro_rules! xs_bind {
    ($local_name:expr, $($tt:tt)*) => {
        (xs_name!($local_name), BoundType {
          ty: TypeReference::new_static(parse_quote!($($tt)*)),
          ty_type: TypeType::Simple,
          serialize_with: None,
          deserialize_with: None,
        })
    };
}

/// Iterator for primitive XSD types using wrapper types.
/// Provides binds for basic numeric and boolean types.
pub struct PrimitiveXsdTypes;

impl IntoIterator for PrimitiveXsdTypes {
    type Item = (ExpandedName<'static>, BoundType);
    type IntoIter = std::array::IntoIter<Self::Item, 8>;

    fn into_iter(self) -> Self::IntoIter {
        [
            // Primitive numeric types
            xs_bind!("boolean", ::xmlity_ns_xsd_types::Boolean),
            xs_bind!("decimal", ::xmlity_ns_xsd_types::Decimal),
            xs_bind!("float", ::xmlity_ns_xsd_types::Float),
            xs_bind!("double", ::xmlity_ns_xsd_types::Double),
            xs_bind!("integer", ::xmlity_ns_xsd_types::Integer),
            xs_bind!("negativeInteger", ::xmlity_ns_xsd_types::NegativeInteger),
            xs_bind!(
                "nonNegativeInteger",
                ::xmlity_ns_xsd_types::NonNegativeInteger
            ),
            xs_bind!(
                "nonPositiveInteger",
                ::xmlity_ns_xsd_types::NonPositiveInteger
            ),
        ]
        .into_iter()
    }
}

/// Iterator for positive integer XSD types using wrapper types.
/// Provides binds for positive integer types.
pub struct PositiveIntegerXsdTypes;

impl IntoIterator for PositiveIntegerXsdTypes {
    type Item = (ExpandedName<'static>, BoundType);
    type IntoIter = std::array::IntoIter<Self::Item, 1>;

    fn into_iter(self) -> Self::IntoIter {
        [xs_bind!(
            "positiveInteger",
            ::xmlity_ns_xsd_types::PositiveInteger
        )]
        .into_iter()
    }
}

/// Iterator for time-related XSD types using wrapper types.
/// Provides binds for date, time, duration, and related temporal types.
pub struct TimeXsdTypes;

impl IntoIterator for TimeXsdTypes {
    type Item = (ExpandedName<'static>, BoundType);
    type IntoIter = std::array::IntoIter<Self::Item, 12>;

    fn into_iter(self) -> Self::IntoIter {
        [
            xs_bind!("duration", ::xmlity_ns_xsd_types::Duration),
            xs_bind!("dateTime", ::xmlity_ns_xsd_types::DateTime),
            xs_bind!("dateTimeStamp", ::xmlity_ns_xsd_types::DateTimeStamp),
            xs_bind!("time", ::xmlity_ns_xsd_types::Time),
            xs_bind!("date", ::xmlity_ns_xsd_types::Date),
            xs_bind!("gYearMonth", ::xmlity_ns_xsd_types::GYearMonth),
            xs_bind!("gYear", ::xmlity_ns_xsd_types::GYear),
            xs_bind!("gMonthDay", ::xmlity_ns_xsd_types::GMonthDay),
            xs_bind!("gMonth", ::xmlity_ns_xsd_types::GMonth),
            xs_bind!("gDay", ::xmlity_ns_xsd_types::GDay),
            xs_bind!("dayTimeDuration", ::xmlity_ns_xsd_types::DayTimeDuration),
            xs_bind!(
                "yearMonthDuration",
                ::xmlity_ns_xsd_types::YearMonthDuration
            ),
        ]
        .into_iter()
    }
}

/// Iterator for string-based XSD types using wrapper types.
/// Provides binds for various string derivations like NCName, Token, Language, etc.
pub struct StringXsdTypes;

impl IntoIterator for StringXsdTypes {
    type Item = (ExpandedName<'static>, BoundType);
    type IntoIter = std::array::IntoIter<Self::Item, 8>;

    fn into_iter(self) -> Self::IntoIter {
        [
            xs_bind!("normalizedString", ::xmlity_ns_xsd_types::NormalizedString),
            xs_bind!("token", ::xmlity_ns_xsd_types::Token),
            xs_bind!("language", ::xmlity_ns_xsd_types::Language),
            xs_bind!("NMTOKEN", ::xmlity_ns_xsd_types::NMToken),
            xs_bind!("Name", ::xmlity_ns_xsd_types::Name),
            xs_bind!("NCName", ::xmlity_ns_xsd_types::NCName),
            xs_bind!("ID", ::xmlity_ns_xsd_types::Id),
            xs_bind!("IDREF", ::xmlity_ns_xsd_types::IdRef),
        ]
        .into_iter()
    }
}

/// Iterator that combines all XSD type bindings using wrapper types.
/// This provides a comprehensive set of type bindings for XSD code generation
/// using the strongly-typed wrapper types from the buf_types module.
pub struct XsdTypesWrappers;

impl IntoIterator for XsdTypesWrappers {
    type Item = (ExpandedName<'static>, BoundType);
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        None.into_iter()
            .chain(PrimitiveXsdTypes)
            .chain(PositiveIntegerXsdTypes)
            .chain(TimeXsdTypes)
            .chain(StringXsdTypes)
            .collect::<Vec<_>>()
            .into_iter()
    }
}
