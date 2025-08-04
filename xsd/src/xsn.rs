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

/// The anyType type, base of all XSD types.
pub static ANY_TYPE: LazyLockEN = LazyLock::new(|| xs_name!("anyType"));
/// The anySimpleType type, base of all simple types.
pub static SIMPLE_ANY_TYPE: LazyLockEN = LazyLock::new(|| xs_name!("anySimpleType"));

type LazyLockEN = LazyLock<ExpandedName<'static>>;

// Primitive types
/// The string primitive type.
pub static STRING: LazyLockEN = LazyLock::new(|| xs_name!("string"));
/// The boolean primitive type.
pub static BOOLEAN: LazyLockEN = LazyLock::new(|| xs_name!("boolean"));
/// The decimal primitive type.
pub static DECIMAL: LazyLockEN = LazyLock::new(|| xs_name!("decimal"));
/// The float primitive type.
pub static FLOAT: LazyLockEN = LazyLock::new(|| xs_name!("float"));
/// The double primitive type.
pub static DOUBLE: LazyLockEN = LazyLock::new(|| xs_name!("double"));
/// The duration primitive type.
pub static DURATION: LazyLockEN = LazyLock::new(|| xs_name!("duration"));
/// The dateTime primitive type.
pub static DATE_TIME: LazyLockEN = LazyLock::new(|| xs_name!("dateTime"));
/// The time primitive type.
pub static TIME: LazyLockEN = LazyLock::new(|| xs_name!("time"));
/// The date primitive type.
pub static DATE: LazyLockEN = LazyLock::new(|| xs_name!("date"));
/// The gYearMonth primitive type.
pub static G_YEAR_MONTH: LazyLockEN = LazyLock::new(|| xs_name!("gYearMonth"));
/// The gYear primitive type.
pub static G_YEAR: LazyLockEN = LazyLock::new(|| xs_name!("gYear"));
/// The gMonthDay primitive type.
pub static G_MONTH_DAY: LazyLockEN = LazyLock::new(|| xs_name!("gMonthDay"));
/// The gDay primitive type.
pub static G_DAY: LazyLockEN = LazyLock::new(|| xs_name!("gDay"));
/// The gMonth primitive type.
pub static G_MONTH: LazyLockEN = LazyLock::new(|| xs_name!("gMonth"));
/// The hexBinary primitive type.
pub static HEX_BINARY: LazyLockEN = LazyLock::new(|| xs_name!("hexBinary"));
/// The base64Binary primitive type.
pub static BASE64_BINARY: LazyLockEN = LazyLock::new(|| xs_name!("base64Binary"));
/// The anyURI primitive type.
pub static ANY_URI: LazyLockEN = LazyLock::new(|| xs_name!("anyURI"));
/// The QName primitive type.
pub static QNAME: LazyLockEN = LazyLock::new(|| xs_name!("QName"));
/// The NOTATION primitive type.
pub static NOTATION: LazyLockEN = LazyLock::new(|| xs_name!("NOTATION"));
/// The normalizedString derived type.
pub static NORMALIZED_STRING: LazyLockEN = LazyLock::new(|| xs_name!("normalizedString"));
/// The token derived type.
pub static TOKEN: LazyLockEN = LazyLock::new(|| xs_name!("token"));
/// The language derived type.
pub static LANGUAGE: LazyLockEN = LazyLock::new(|| xs_name!("language"));
/// The NMTOKEN derived type.
pub static NMTOKEN: LazyLockEN = LazyLock::new(|| xs_name!("NMTOKEN"));
/// The NMTOKENS derived type.
pub static NMTOKENS: LazyLockEN = LazyLock::new(|| xs_name!("NMTOKENS"));
/// The NAME derived type.
pub static NAME: LazyLockEN = LazyLock::new(|| xs_name!("NAME"));
/// The NCName derived type.
pub static NCNAME: LazyLockEN = LazyLock::new(|| xs_name!("NCName"));
/// The ID derived type.
pub static ID: LazyLockEN = LazyLock::new(|| xs_name!("ID"));
/// The IDREF derived type.
pub static IDREF: LazyLockEN = LazyLock::new(|| xs_name!("IDREF"));
/// The IDREFS derived type.
pub static IDREFS: LazyLockEN = LazyLock::new(|| xs_name!("IDREFS"));
/// The ENTITY derived type.
pub static ENTITY: LazyLockEN = LazyLock::new(|| xs_name!("ENTITY"));
/// The ENTITIES derived type.
pub static ENTITIES: LazyLockEN = LazyLock::new(|| xs_name!("ENTITIES"));
/// The integer derived type.
pub static INTEGER: LazyLockEN = LazyLock::new(|| xs_name!("integer"));
/// The nonPositiveInteger derived type.
pub static NON_POSITIVE_INTEGER: LazyLockEN = LazyLock::new(|| xs_name!("nonPositiveInteger"));
/// The negativeInteger derived type.
pub static NEGATIVE_INTEGER: LazyLockEN = LazyLock::new(|| xs_name!("negativeInteger"));
/// The long derived type.
pub static LONG: LazyLockEN = LazyLock::new(|| xs_name!("long"));
/// The int derived type.
pub static INT: LazyLockEN = LazyLock::new(|| xs_name!("int"));
/// The short derived type.
pub static SHORT: LazyLockEN = LazyLock::new(|| xs_name!("short"));
/// The byte derived type.
pub static BYTE: LazyLockEN = LazyLock::new(|| xs_name!("byte"));
/// The nonNegativeInteger derived type.
pub static NON_NEGATIVE_INTEGER: LazyLockEN = LazyLock::new(|| xs_name!("nonNegativeInteger"));
/// The unsignedLong derived type.
pub static UNSIGNED_LONG: LazyLockEN = LazyLock::new(|| xs_name!("unsignedLong"));
/// The unsignedInt derived type.
pub static UNSIGNED_INT: LazyLockEN = LazyLock::new(|| xs_name!("unsignedInt"));
/// The unsignedShort derived type.
pub static UNSIGNED_SHORT: LazyLockEN = LazyLock::new(|| xs_name!("unsignedShort"));
/// The unsignedByte derived type.
pub static UNSIGNED_BYTE: LazyLockEN = LazyLock::new(|| xs_name!("unsignedByte"));
/// The positiveInteger derived type.
pub static POSITIVE_INTEGER: LazyLockEN = LazyLock::new(|| xs_name!("positiveInteger"));
/// The yearMonthDuration derived type.
pub static YEAR_MONTH_DURATION: LazyLockEN = LazyLock::new(|| xs_name!("yearMonthDuration"));
/// The dayTimeDuration derived type.
pub static DAY_TIME_DURATION: LazyLockEN = LazyLock::new(|| xs_name!("dayTimeDuration"));
/// The dateTimeStamp derived type.
pub static DATE_TIME_STAMP: LazyLockEN = LazyLock::new(|| xs_name!("dateTimeStamp"));
