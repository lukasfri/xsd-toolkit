use std::sync::LazyLock;

use xmlity::{ExpandedName, LocalName, XmlNamespace};

macro_rules! xs_name {
    ($local_name:expr) => {
        ExpandedName::new(
            LocalName::new_dangerous($local_name),
            Some(XmlNamespace::XS),
        )
    };
}

pub static ANY_TYPE: LazyLockEN = LazyLock::new(|| xs_name!("anyType"));
pub static SIMPLE_ANY_TYPE: LazyLockEN = LazyLock::new(|| xs_name!("anySimpleType"));

type LazyLockEN = LazyLock<ExpandedName<'static>>;

// Primitive types
pub static STRING: LazyLockEN = LazyLock::new(|| xs_name!("string"));
pub static BOOLEAN: LazyLockEN = LazyLock::new(|| xs_name!("boolean"));
pub static DECIMAL: LazyLockEN = LazyLock::new(|| xs_name!("decimal"));
pub static FLOAT: LazyLockEN = LazyLock::new(|| xs_name!("float"));
pub static DOUBLE: LazyLockEN = LazyLock::new(|| xs_name!("double"));
pub static DURATION: LazyLockEN = LazyLock::new(|| xs_name!("duration"));
pub static DATE_TIME: LazyLockEN = LazyLock::new(|| xs_name!("dateTime"));
pub static TIME: LazyLockEN = LazyLock::new(|| xs_name!("time"));
pub static DATE: LazyLockEN = LazyLock::new(|| xs_name!("date"));
pub static G_YEAR_MONTH: LazyLockEN = LazyLock::new(|| xs_name!("gYearMonth"));
pub static G_YEAR: LazyLockEN = LazyLock::new(|| xs_name!("gYear"));
pub static G_MONTH_DAY: LazyLockEN = LazyLock::new(|| xs_name!("gMonthDay"));
pub static G_DAY: LazyLockEN = LazyLock::new(|| xs_name!("gDay"));
pub static G_MONTH: LazyLockEN = LazyLock::new(|| xs_name!("gMonth"));
pub static HEX_BINARY: LazyLockEN = LazyLock::new(|| xs_name!("hexBinary"));
pub static BASE64_BINARY: LazyLockEN = LazyLock::new(|| xs_name!("base64Binary"));
pub static ANY_URI: LazyLockEN = LazyLock::new(|| xs_name!("anyURI"));
pub static QNAME: LazyLockEN = LazyLock::new(|| xs_name!("QName"));
pub static NOTATION: LazyLockEN = LazyLock::new(|| xs_name!("NOTATION"));
pub static NORMALIZED_STRING: LazyLockEN = LazyLock::new(|| xs_name!("normalizedString"));
pub static TOKEN: LazyLockEN = LazyLock::new(|| xs_name!("token"));
pub static LANGUAGE: LazyLockEN = LazyLock::new(|| xs_name!("language"));
pub static NMTOKEN: LazyLockEN = LazyLock::new(|| xs_name!("NMTOKEN"));
pub static NMTOKENS: LazyLockEN = LazyLock::new(|| xs_name!("NMTOKENS"));
pub static NAME: LazyLockEN = LazyLock::new(|| xs_name!("NAME"));
pub static NCNAME: LazyLockEN = LazyLock::new(|| xs_name!("NCNAME"));
pub static ID: LazyLockEN = LazyLock::new(|| xs_name!("ID"));
pub static IDREF: LazyLockEN = LazyLock::new(|| xs_name!("IDREF"));
pub static IDREFS: LazyLockEN = LazyLock::new(|| xs_name!("IDREFS"));
pub static ENTITY: LazyLockEN = LazyLock::new(|| xs_name!("ENTITY"));
pub static ENTITIES: LazyLockEN = LazyLock::new(|| xs_name!("ENTITIES"));
pub static INTEGER: LazyLockEN = LazyLock::new(|| xs_name!("integer"));
pub static NON_POSITIVE_INTEGER: LazyLockEN = LazyLock::new(|| xs_name!("nonPositiveInteger"));
pub static NEGATIVE_INTEGER: LazyLockEN = LazyLock::new(|| xs_name!("negativeInteger"));
pub static LONG: LazyLockEN = LazyLock::new(|| xs_name!("long"));
pub static INT: LazyLockEN = LazyLock::new(|| xs_name!("int"));
pub static SHORT: LazyLockEN = LazyLock::new(|| xs_name!("short"));
pub static BYTE: LazyLockEN = LazyLock::new(|| xs_name!("byte"));
pub static NON_NEGATIVE_INTEGER: LazyLockEN = LazyLock::new(|| xs_name!("nonNegativeInteger"));
pub static UNSIGNED_LONG: LazyLockEN = LazyLock::new(|| xs_name!("unsignedLong"));
pub static UNSIGNED_INT: LazyLockEN = LazyLock::new(|| xs_name!("unsignedInt"));
pub static UNSIGNED_SHORT: LazyLockEN = LazyLock::new(|| xs_name!("unsignedShort"));
pub static UNSIGNED_BYTE: LazyLockEN = LazyLock::new(|| xs_name!("unsignedByte"));
pub static POSITIVE_INTEGER: LazyLockEN = LazyLock::new(|| xs_name!("positiveInteger"));
pub static YEAR_MONTH_DURATION: LazyLockEN = LazyLock::new(|| xs_name!("yearMonthDuration"));
pub static DAY_TIME_DURATION: LazyLockEN = LazyLock::new(|| xs_name!("dayTimeDuration"));
pub static DATE_TIME_STAMP: LazyLockEN = LazyLock::new(|| xs_name!("dateTimeStamp"));
