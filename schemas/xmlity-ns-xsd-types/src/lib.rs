//! # XMLity adapter of `xsd_types`
//!
//! TODO: Docs.

use derive_more::{Deref, DerefMut, From, Into};
use xmlity::{Deserialize, Serialize};
use xsd_types::ParseXsd;

macro_rules! impl_de_serialize_xsd_types {
    ($name:ident, $inner:ty) => {
        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D: xmlity::Deserializer<'de>>(reader: D) -> Result<Self, D::Error> {
                String::deserialize(reader).and_then(|s| {
                    <$inner>::parse_xsd(&s).map($name).map_err(|e| {
                        xmlity::de::Error::custom(match e {
                            xsd_types::ParseXsdError::InvalidLexicalForm(invalid) => {
                                format!("Invalid lexical form: {:?}", invalid)
                            }
                            xsd_types::ParseXsdError::InvalidValue(e) => {
                                format!("Error when parsing: {}", e)
                            }
                        })
                    })
                })
            }
        }

        impl Serialize for $name {
            fn serialize<S: xmlity::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                self.0.to_string().serialize(serializer)
            }
        }
    };
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deref, DerefMut, From, Into)]
pub struct Boolean(pub xsd_types::Boolean);

impl_de_serialize_xsd_types!(Boolean, xsd_types::Boolean);

#[derive(Debug, Clone, Deref, DerefMut, From, Into)]
pub struct Date(pub xsd_types::Date);

impl_de_serialize_xsd_types!(Date, xsd_types::Date);

#[derive(Debug, Clone, Deref, DerefMut, From, Into)]
pub struct DateTime(pub xsd_types::DateTime);

impl_de_serialize_xsd_types!(DateTime, xsd_types::DateTime);

#[derive(Debug, Clone, Deref, DerefMut, From, Into)]
pub struct DateTimeStamp(pub xsd_types::DateTimeStamp);

impl_de_serialize_xsd_types!(DateTimeStamp, xsd_types::DateTimeStamp);

#[derive(Debug, Clone, Deref, DerefMut, From, Into)]
pub struct DayTimeDuration(pub xsd_types::DayTimeDuration);

impl_de_serialize_xsd_types!(DayTimeDuration, xsd_types::DayTimeDuration);

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deref, DerefMut, From, Into)]
pub struct Decimal(pub xsd_types::Decimal);

impl_de_serialize_xsd_types!(Decimal, xsd_types::Decimal);

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deref, DerefMut, From, Into)]
pub struct Double(pub xsd_types::Double);

impl_de_serialize_xsd_types!(Double, xsd_types::Double);

#[derive(Debug, Clone, Deref, DerefMut, From, Into)]
pub struct Duration(pub xsd_types::Duration);

impl_de_serialize_xsd_types!(Duration, xsd_types::Duration);

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deref, DerefMut, From, Into)]
pub struct Float(pub xsd_types::Float);

impl_de_serialize_xsd_types!(Float, xsd_types::Float);

#[derive(Debug, Clone, Deref, DerefMut, From, Into)]
pub struct GDay(pub xsd_types::GDay);

impl_de_serialize_xsd_types!(GDay, xsd_types::GDay);

#[derive(Debug, Clone, Deref, DerefMut, From, Into)]
pub struct GMonth(pub xsd_types::GMonth);

impl_de_serialize_xsd_types!(GMonth, xsd_types::GMonth);

#[derive(Debug, Clone, Deref, DerefMut, From, Into)]
pub struct GMonthDay(pub xsd_types::GMonthDay);

impl_de_serialize_xsd_types!(GMonthDay, xsd_types::GMonthDay);

#[derive(Debug, Clone, Deref, DerefMut, From, Into)]
pub struct GYear(pub xsd_types::GYear);

impl_de_serialize_xsd_types!(GYear, xsd_types::GYear);

#[derive(Debug, Clone, Deref, DerefMut, From, Into)]
pub struct GYearMonth(pub xsd_types::GYearMonth);

impl_de_serialize_xsd_types!(GYearMonth, xsd_types::GYearMonth);

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deref, DerefMut, From, Into)]
pub struct Id(pub xsd_types::IdBuf);

impl_de_serialize_xsd_types!(Id, xsd_types::IdBuf);

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deref, DerefMut, From, Into)]
pub struct IdRef(pub xsd_types::IdRefBuf);

impl_de_serialize_xsd_types!(IdRef, xsd_types::IdRefBuf);

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deref, DerefMut, From, Into)]
pub struct Integer(pub xsd_types::Integer);

impl_de_serialize_xsd_types!(Integer, xsd_types::Integer);

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deref, DerefMut, From, Into)]
pub struct Language(pub xsd_types::LanguageBuf);

impl_de_serialize_xsd_types!(Language, xsd_types::LanguageBuf);

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deref, DerefMut, From, Into)]
pub struct NCName(pub xsd_types::NCNameBuf);

impl_de_serialize_xsd_types!(NCName, xsd_types::NCNameBuf);

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deref, DerefMut, From, Into)]
pub struct NMToken(pub xsd_types::NMTokenBuf);

impl_de_serialize_xsd_types!(NMToken, xsd_types::NMTokenBuf);

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deref, DerefMut, From, Into)]
pub struct Name(pub xsd_types::NameBuf);

impl_de_serialize_xsd_types!(Name, xsd_types::NameBuf);

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deref, DerefMut, From, Into)]
pub struct NegativeInteger(pub xsd_types::NegativeInteger);

impl_de_serialize_xsd_types!(NegativeInteger, xsd_types::NegativeInteger);

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deref, DerefMut, From, Into)]
pub struct NonNegativeInteger(pub xsd_types::NonNegativeInteger);

impl_de_serialize_xsd_types!(NonNegativeInteger, xsd_types::NonNegativeInteger);

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deref, DerefMut, From, Into)]
pub struct NonPositiveInteger(pub xsd_types::NonPositiveInteger);

impl_de_serialize_xsd_types!(NonPositiveInteger, xsd_types::NonPositiveInteger);

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deref, DerefMut, From, Into)]
pub struct NormalizedString(pub xsd_types::NormalizedString);

impl_de_serialize_xsd_types!(NormalizedString, xsd_types::NormalizedString);

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deref, DerefMut, From, Into)]
pub struct PositiveInteger(pub xsd_types::PositiveInteger);

impl_de_serialize_xsd_types!(PositiveInteger, xsd_types::PositiveInteger);

#[derive(Debug, Clone, Deref, DerefMut, From, Into)]
pub struct Time(pub xsd_types::Time);

impl_de_serialize_xsd_types!(Time, xsd_types::Time);

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deref, DerefMut, From, Into)]
pub struct Token(pub xsd_types::TokenBuf);

impl_de_serialize_xsd_types!(Token, xsd_types::TokenBuf);

#[derive(Debug, Clone, Deref, DerefMut, From, Into)]
pub struct YearMonthDuration(pub xsd_types::YearMonthDuration);

impl_de_serialize_xsd_types!(YearMonthDuration, xsd_types::YearMonthDuration);
