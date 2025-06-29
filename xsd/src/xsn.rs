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

pub static ANY_TYPE: LLEN = LazyLock::new(|| xs_name!("anyType"));
pub static SIMPLE_ANY_TYPE: LLEN = LazyLock::new(|| xs_name!("anySimpleType"));

type LLEN = LazyLock<ExpandedName<'static>>;

// Primitive types
pub static STRING: LLEN = LazyLock::new(|| xs_name!("string"));
pub static BOOLEAN: LLEN = LazyLock::new(|| xs_name!("boolean"));
pub static DECIMAL: LLEN = LazyLock::new(|| xs_name!("decimal"));
pub static FLOAT: LLEN = LazyLock::new(|| xs_name!("float"));
pub static DOUBLE: LLEN = LazyLock::new(|| xs_name!("double"));
pub static DURATION: LLEN = LazyLock::new(|| xs_name!("duration"));
pub static DATE_TIME: LLEN = LazyLock::new(|| xs_name!("dateTime"));
pub static TIME: LLEN = LazyLock::new(|| xs_name!("time"));
pub static DATE: LLEN = LazyLock::new(|| xs_name!("date"));
pub static G_YEAR_MONTH: LLEN = LazyLock::new(|| xs_name!("gYearMonth"));
pub static G_YEAR: LLEN = LazyLock::new(|| xs_name!("gYear"));
pub static G_MONTH_DAY: LLEN = LazyLock::new(|| xs_name!("gMonthDay"));
pub static G_DAY: LLEN = LazyLock::new(|| xs_name!("gDay"));
pub static G_MONTH: LLEN = LazyLock::new(|| xs_name!("gMonth"));
pub static HEX_BINARY: LLEN = LazyLock::new(|| xs_name!("hexBinary"));
pub static BASE64_BINARY: LLEN = LazyLock::new(|| xs_name!("base64Binary"));
pub static ANY_URI: LLEN = LazyLock::new(|| xs_name!("anyURI"));
pub static QNAME: LLEN = LazyLock::new(|| xs_name!("QName"));
pub static NOTATION: LLEN = LazyLock::new(|| xs_name!("NOTATION"));
pub static NORMALIZED_STRING: LLEN = LazyLock::new(|| xs_name!("normalizedString"));
pub static TOKEN: LLEN = LazyLock::new(|| xs_name!("token"));
pub static LANGUAGE: LLEN = LazyLock::new(|| xs_name!("language"));
pub static NMTOKEN: LLEN = LazyLock::new(|| xs_name!("NMTOKEN"));
pub static NMTOKENS: LLEN = LazyLock::new(|| xs_name!("NMTOKENS"));
pub static NAME: LLEN = LazyLock::new(|| xs_name!("NAME"));
pub static NCNAME: LLEN = LazyLock::new(|| xs_name!("NCNAME"));
pub static ID: LLEN = LazyLock::new(|| xs_name!("ID"));
pub static IDREF: LLEN = LazyLock::new(|| xs_name!("IDREF"));
pub static IDREFS: LLEN = LazyLock::new(|| xs_name!("IDREFS"));
pub static ENTITY: LLEN = LazyLock::new(|| xs_name!("ENTITY"));
pub static ENTITIES: LLEN = LazyLock::new(|| xs_name!("ENTITIES"));
pub static INTEGER: LLEN = LazyLock::new(|| xs_name!("integer"));
pub static NON_POSITIVE_INTEGER: LLEN = LazyLock::new(|| xs_name!("nonPositiveInteger"));
pub static NEGATIVE_INTEGER: LLEN = LazyLock::new(|| xs_name!("negativeInteger"));
pub static LONG: LLEN = LazyLock::new(|| xs_name!("long"));
pub static INT: LLEN = LazyLock::new(|| xs_name!("int"));
pub static SHORT: LLEN = LazyLock::new(|| xs_name!("short"));
pub static BYTE: LLEN = LazyLock::new(|| xs_name!("byte"));
pub static NON_NEGATIVE_INTEGER: LLEN = LazyLock::new(|| xs_name!("nonNegativeInteger"));
pub static UNSIGNED_LONG: LLEN = LazyLock::new(|| xs_name!("unsignedLong"));
pub static UNSIGNED_INT: LLEN = LazyLock::new(|| xs_name!("unsignedInt"));
pub static UNSIGNED_SHORT: LLEN = LazyLock::new(|| xs_name!("unsignedShort"));
pub static UNSIGNED_BYTE: LLEN = LazyLock::new(|| xs_name!("unsignedByte"));
pub static POSITIVE_INTEGER: LLEN = LazyLock::new(|| xs_name!("positiveInteger"));
pub static YEAR_MONTH_DURATION: LLEN = LazyLock::new(|| xs_name!("yearMonthDuration"));
pub static DAY_TIME_DURATION: LLEN = LazyLock::new(|| xs_name!("dayTimeDuration"));
pub static DATE_TIME_STAMP: LLEN = LazyLock::new(|| xs_name!("dateTimeStamp"));
