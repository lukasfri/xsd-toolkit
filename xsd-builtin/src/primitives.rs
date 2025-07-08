/// https://www.w3.org/TR/2012/REC-xmlschema11-2-20120405/datatypes.html#built-in-primitive-datatypes
//TODO: - https://www.w3.org/TR/2012/REC-xmlschema11-2-20120405/datatypes.html#cos-applicable-facets
use std::num::NonZeroUsize;

use crate::facets::{fundamental::*, numeric::*, special::*, PossibleFacet, RequiredFacet};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pattern<'a>(pub &'a str);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Assertion<'a>(pub &'a str);

/// https://www.w3.org/TR/2012/REC-xmlschema11-2-20120405/datatypes.html#string
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct String<'a> {
    pub white_space: RequiredFacet<WhiteSpaceValue>,
    pub length: PossibleFacet<usize>,
    pub min_length: PossibleFacet<usize>,
    pub max_length: PossibleFacet<usize>,
    pub patterns: PossibleFacet<&'a [Pattern<'a>]>,
    pub enumeration: PossibleFacet<&'a [&'a str]>,
    pub assertions: PossibleFacet<&'a [Assertion<'a>]>,
}

impl String<'_> {
    pub const STRING: String<'static> = String {
        white_space: RequiredFacet::Unfixed(WhiteSpaceValue::Preserve),
        length: PossibleFacet::None,
        min_length: PossibleFacet::None,
        max_length: PossibleFacet::None,
        patterns: PossibleFacet::None,
        enumeration: PossibleFacet::None,
        assertions: PossibleFacet::None,
    };
}

impl<'a> Default for String<'a> {
    fn default() -> Self {
        Self::STRING
    }
}

impl Ordered for String<'_> {
    fn ordered(&self) -> OrderValue {
        OrderValue::False
    }
}

impl Bounded for String<'_> {
    fn bounded(&self) -> bool {
        false
    }
}

impl Cardinality for String<'_> {
    fn cardinality(&self) -> CardinalValue {
        CardinalValue::CountablyInfinite
    }
}

impl Numeric for String<'_> {
    fn numeric(&self) -> bool {
        false
    }
}

impl Fundamental for String<'_> {}

impl WhiteSpace for String<'_> {
    fn white_space(&self) -> Option<WhiteSpaceValue> {
        self.white_space.into()
    }
}

/// https://www.w3.org/TR/2012/REC-xmlschema11-2-20120405/datatypes.html#boolean
pub struct Boolean {
    // TODO: Is enumeration allowed for boolean?
    // pub enumeration: &'static [&'static str],
    pub patterns: &'static [&'static str],
    pub assertions: &'static [&'static str],
}

impl WhiteSpace for Boolean {
    fn white_space(&self) -> Option<WhiteSpaceValue> {
        RequiredFacet::Fixed(WhiteSpaceValue::Collapse).into()
    }
}

impl Ordered for Boolean {
    fn ordered(&self) -> OrderValue {
        OrderValue::False
    }
}

impl Bounded for Boolean {
    fn bounded(&self) -> bool {
        false
    }
}

impl Cardinality for Boolean {
    fn cardinality(&self) -> CardinalValue {
        CardinalValue::Finite
    }
}

impl Numeric for Boolean {
    fn numeric(&self) -> bool {
        false
    }
}

impl Fundamental for Boolean {}

/// https://www.w3.org/TR/2012/REC-xmlschema11-2-20120405/datatypes.html#decimal
pub struct Decimal<'a> {
    pub total_digits: PossibleFacet<NonZeroUsize>,
    pub fraction_digits: PossibleFacet<usize>,
    pub patterns: PossibleFacet<&'a [Pattern<'a>]>,
    pub enumeration: PossibleFacet<&'a [&'a str]>,
    pub max_inclusive: PossibleFacet<&'a str>,
    pub max_exclusive: PossibleFacet<&'a str>,
    pub min_inclusive: PossibleFacet<&'a str>,
    pub min_exclusive: PossibleFacet<&'a str>,
    pub assertions: PossibleFacet<&'a [Assertion<'a>]>,
}

impl Decimal<'_> {
    pub const DECIMAL: Decimal<'static> = Decimal {
        total_digits: PossibleFacet::None,
        fraction_digits: PossibleFacet::None,
        patterns: PossibleFacet::None,
        enumeration: PossibleFacet::None,
        max_inclusive: PossibleFacet::None,
        max_exclusive: PossibleFacet::None,
        min_inclusive: PossibleFacet::None,
        min_exclusive: PossibleFacet::None,
        assertions: PossibleFacet::None,
    };
}

impl Default for Decimal<'_> {
    fn default() -> Self {
        Self::DECIMAL
    }
}

impl WhiteSpace for Decimal<'_> {
    fn white_space(&self) -> Option<WhiteSpaceValue> {
        RequiredFacet::Fixed(WhiteSpaceValue::Collapse).into()
    }
}

impl Ordered for Decimal<'_> {
    fn ordered(&self) -> OrderValue {
        OrderValue::Total
    }
}

impl Bounded for Decimal<'_> {
    fn bounded(&self) -> bool {
        false
    }
}

impl Cardinality for Decimal<'_> {
    fn cardinality(&self) -> CardinalValue {
        CardinalValue::CountablyInfinite
    }
}

impl Numeric for Decimal<'_> {
    fn numeric(&self) -> bool {
        true
    }
}

impl Fundamental for Decimal<'_> {}

// 3.3.4.3 Facets

// The float datatype and all datatypes derived from it by restriction have the following ·constraining facets· with fixed values; these facets must not be changed from the values shown:

//     whiteSpace = collapse (fixed)

// Datatypes derived by restriction from float may also specify values for the following ·constraining facets·:

//     pattern
//     enumeration
//     maxInclusive
//     maxExclusive
//     minInclusive
//     minExclusive
//     assertions

// The float datatype has the following values for its ·fundamental facets·:

//     ordered = partial
//     bounded = true
//     cardinality = finite
//     numeric = true
/// https://www.w3.org/TR/2012/REC-xmlschema11-2-20120405/datatypes.html#float
pub struct Float<'a> {
    pub patterns: PossibleFacet<&'a [Pattern<'a>]>,
    pub enumeration: PossibleFacet<&'a [&'a str]>,
    pub max_inclusive: PossibleFacet<&'a str>,
    pub max_exclusive: PossibleFacet<&'a str>,
    pub min_inclusive: PossibleFacet<&'a str>,
    pub min_exclusive: PossibleFacet<&'a str>,
    pub assertions: PossibleFacet<&'a [Assertion<'a>]>,
}

impl Float<'_> {
    pub const FLOAT: Float<'static> = Float {
        patterns: PossibleFacet::None,
        enumeration: PossibleFacet::None,
        max_inclusive: PossibleFacet::None,
        max_exclusive: PossibleFacet::None,
        min_inclusive: PossibleFacet::None,
        min_exclusive: PossibleFacet::None,
        assertions: PossibleFacet::None,
    };
}

impl WhiteSpace for Float<'_> {
    fn white_space(&self) -> Option<WhiteSpaceValue> {
        RequiredFacet::Fixed(WhiteSpaceValue::Collapse).into()
    }
}

impl Ordered for Float<'_> {
    fn ordered(&self) -> OrderValue {
        OrderValue::Partial
    }
}

impl Bounded for Float<'_> {
    fn bounded(&self) -> bool {
        true
    }
}

impl Cardinality for Float<'_> {
    fn cardinality(&self) -> CardinalValue {
        CardinalValue::Finite
    }
}

impl Numeric for Float<'_> {
    fn numeric(&self) -> bool {
        true
    }
}

impl Fundamental for Float<'_> {}

/// https://www.w3.org/TR/2012/REC-xmlschema11-2-20120405/datatypes.html#double
pub struct Double<'a> {
    pub patterns: PossibleFacet<&'a [Pattern<'a>]>,
    pub enumeration: PossibleFacet<&'a [&'a str]>,
    pub max_inclusive: PossibleFacet<&'a str>,
    pub max_exclusive: PossibleFacet<&'a str>,
    pub min_inclusive: PossibleFacet<&'a str>,
    pub min_exclusive: PossibleFacet<&'a str>,
    pub assertions: PossibleFacet<&'a [Assertion<'a>]>,
}

impl WhiteSpace for Double<'_> {
    fn white_space(&self) -> Option<WhiteSpaceValue> {
        RequiredFacet::Fixed(WhiteSpaceValue::Collapse).into()
    }
}

impl Ordered for Double<'_> {
    fn ordered(&self) -> OrderValue {
        OrderValue::Partial
    }
}

impl Bounded for Double<'_> {
    fn bounded(&self) -> bool {
        true
    }
}

impl Cardinality for Double<'_> {
    fn cardinality(&self) -> CardinalValue {
        CardinalValue::Finite
    }
}

impl Numeric for Double<'_> {
    fn numeric(&self) -> bool {
        true
    }
}

impl Fundamental for Double<'_> {}

/// https://www.w3.org/TR/2012/REC-xmlschema11-2-20120405/datatypes.html#duration
pub struct Duration<'a> {
    pub patterns: RequiredFacet<&'a [&'a str]>,
    pub enumeration: PossibleFacet<&'a [&'a str]>,
    pub max_inclusive: PossibleFacet<&'a str>,
    pub max_exclusive: PossibleFacet<&'a str>,
    pub min_inclusive: PossibleFacet<&'a str>,
    pub min_exclusive: PossibleFacet<&'a str>,
    pub assertions: PossibleFacet<&'a [Assertion<'a>]>,
}

impl<'a> Duration<'a> {
    pub(crate) const DURATION_PATTERN: &'static str =
        r###"-?P[0-9]+Y?([0-9]+M)?([0-9]+D)?(T([0-9]+H)?([0-9]+M)?([0-9]+(\.[0-9]+)?S)?)?"###;

    pub const DURATION: Duration<'static> = Duration {
        patterns: RequiredFacet::Unfixed(&[Self::DURATION_PATTERN]),
        enumeration: PossibleFacet::None,
        max_inclusive: PossibleFacet::None,
        max_exclusive: PossibleFacet::None,
        min_inclusive: PossibleFacet::None,
        min_exclusive: PossibleFacet::None,
        assertions: PossibleFacet::None,
    };
}

impl Default for Duration<'_> {
    fn default() -> Self {
        Self::DURATION
    }
}

impl WhiteSpace for Duration<'_> {
    fn white_space(&self) -> Option<WhiteSpaceValue> {
        RequiredFacet::Fixed(WhiteSpaceValue::Collapse).into()
    }
}

impl Ordered for Duration<'_> {
    fn ordered(&self) -> OrderValue {
        OrderValue::Partial
    }
}

impl Bounded for Duration<'_> {
    fn bounded(&self) -> bool {
        false
    }
}

impl Cardinality for Duration<'_> {
    fn cardinality(&self) -> CardinalValue {
        CardinalValue::CountablyInfinite
    }
}

impl Numeric for Duration<'_> {
    fn numeric(&self) -> bool {
        false
    }
}

impl Fundamental for Duration<'_> {}

/// https://www.w3.org/TR/2012/REC-xmlschema11-2-20120405/datatypes.html#dateTime
pub struct DateTime<'a> {
    pub explicit_timezone: RequiredFacet<ExplicitTimezoneValue>,
    pub patterns: PossibleFacet<&'a [Pattern<'a>]>,
    pub enumeration: PossibleFacet<&'a [&'a str]>,
    pub max_inclusive: PossibleFacet<&'a str>,
    pub max_exclusive: PossibleFacet<&'a str>,
    pub min_inclusive: PossibleFacet<&'a str>,
    pub min_exclusive: PossibleFacet<&'a str>,
    pub assertions: PossibleFacet<&'a [Assertion<'a>]>,
}

impl DateTime<'_> {
    pub const DATE_TIME: DateTime<'static> = DateTime {
        explicit_timezone: RequiredFacet::Unfixed(ExplicitTimezoneValue::Optional),
        patterns: PossibleFacet::None,
        enumeration: PossibleFacet::None,
        max_inclusive: PossibleFacet::None,
        max_exclusive: PossibleFacet::None,
        min_inclusive: PossibleFacet::None,
        min_exclusive: PossibleFacet::None,
        assertions: PossibleFacet::None,
    };
}

impl Default for DateTime<'_> {
    fn default() -> Self {
        Self::DATE_TIME
    }
}

impl WhiteSpace for DateTime<'_> {
    fn white_space(&self) -> Option<WhiteSpaceValue> {
        RequiredFacet::Fixed(WhiteSpaceValue::Collapse).into()
    }
}

impl Ordered for DateTime<'_> {
    fn ordered(&self) -> OrderValue {
        OrderValue::Partial
    }
}

impl Bounded for DateTime<'_> {
    fn bounded(&self) -> bool {
        false
    }
}

impl Cardinality for DateTime<'_> {
    fn cardinality(&self) -> CardinalValue {
        CardinalValue::CountablyInfinite
    }
}

impl Numeric for DateTime<'_> {
    fn numeric(&self) -> bool {
        false
    }
}

impl Fundamental for DateTime<'_> {}

/// https://www.w3.org/TR/2012/REC-xmlschema11-2-20120405/datatypes.html#time
pub struct Time<'a> {
    pub explicit_timezone: RequiredFacet<ExplicitTimezoneValue>,
    pub patterns: PossibleFacet<&'a [Pattern<'a>]>,
    pub enumeration: PossibleFacet<&'a [&'a str]>,
    pub max_inclusive: PossibleFacet<&'a str>,
    pub max_exclusive: PossibleFacet<&'a str>,
    pub min_inclusive: PossibleFacet<&'a str>,
    pub min_exclusive: PossibleFacet<&'a str>,
    pub assertions: PossibleFacet<&'a [Assertion<'a>]>,
}

impl Time<'_> {
    pub const TIME: Time<'static> = Time {
        explicit_timezone: RequiredFacet::Unfixed(ExplicitTimezoneValue::Optional),
        patterns: PossibleFacet::None,
        enumeration: PossibleFacet::None,
        max_inclusive: PossibleFacet::None,
        max_exclusive: PossibleFacet::None,
        min_inclusive: PossibleFacet::None,
        min_exclusive: PossibleFacet::None,
        assertions: PossibleFacet::None,
    };
}

impl Default for Time<'_> {
    fn default() -> Self {
        Self::TIME
    }
}

impl WhiteSpace for Time<'_> {
    fn white_space(&self) -> Option<WhiteSpaceValue> {
        RequiredFacet::Fixed(WhiteSpaceValue::Collapse).into()
    }
}

impl Ordered for Time<'_> {
    fn ordered(&self) -> OrderValue {
        OrderValue::Partial
    }
}

impl Bounded for Time<'_> {
    fn bounded(&self) -> bool {
        false
    }
}

impl Cardinality for Time<'_> {
    fn cardinality(&self) -> CardinalValue {
        CardinalValue::CountablyInfinite
    }
}

impl Numeric for Time<'_> {
    fn numeric(&self) -> bool {
        false
    }
}

impl Fundamental for Time<'_> {}

/// https://www.w3.org/TR/2012/REC-xmlschema11-2-20120405/datatypes.html#date
pub struct Date<'a> {
    pub explicit_timezone: RequiredFacet<ExplicitTimezoneValue>,
    pub patterns: PossibleFacet<&'a [Pattern<'a>]>,
    pub enumeration: PossibleFacet<&'a [&'a str]>,
    pub max_inclusive: PossibleFacet<&'a str>,
    pub max_exclusive: PossibleFacet<&'a str>,
    pub min_inclusive: PossibleFacet<&'a str>,
    pub min_exclusive: PossibleFacet<&'a str>,
    pub assertions: PossibleFacet<&'a [Assertion<'a>]>,
}

impl Date<'_> {
    pub const DATE: Date<'static> = Date {
        explicit_timezone: RequiredFacet::Unfixed(ExplicitTimezoneValue::Optional),
        patterns: PossibleFacet::None,
        enumeration: PossibleFacet::None,
        max_inclusive: PossibleFacet::None,
        max_exclusive: PossibleFacet::None,
        min_inclusive: PossibleFacet::None,
        min_exclusive: PossibleFacet::None,
        assertions: PossibleFacet::None,
    };
}

impl Default for Date<'_> {
    fn default() -> Self {
        Self::DATE
    }
}

impl WhiteSpace for Date<'_> {
    fn white_space(&self) -> Option<WhiteSpaceValue> {
        RequiredFacet::Fixed(WhiteSpaceValue::Collapse).into()
    }
}

impl Ordered for Date<'_> {
    fn ordered(&self) -> OrderValue {
        OrderValue::Partial
    }
}

impl Bounded for Date<'_> {
    fn bounded(&self) -> bool {
        false
    }
}

impl Cardinality for Date<'_> {
    fn cardinality(&self) -> CardinalValue {
        CardinalValue::CountablyInfinite
    }
}

impl Numeric for Date<'_> {
    fn numeric(&self) -> bool {
        false
    }
}

impl Fundamental for Date<'_> {}

// The gYearMonth datatype and all datatypes derived from it by restriction have the following ·constraining facets· with fixed values; these facets must not be changed from the values shown:

//     whiteSpace = collapse (fixed)

// The gYearMonth datatype has the following ·constraining facets· with the values shown; these facets may be specified in the derivation of new types, if the value given is at least as restrictive as the one shown:

//     explicitTimezone = optional

// Datatypes derived by restriction from gYearMonth may also specify values for the following ·constraining facets·:

//     pattern
//     enumeration
//     maxInclusive
//     maxExclusive
//     minInclusive
//     minExclusive
//     assertions

// The gYearMonth datatype has the following values for its ·fundamental facets·:

//     ordered = partial
//     bounded = false
//     cardinality = countably infinite
//     numeric = false
/// https://www.w3.org/TR/2012/REC-xmlschema11-2-20120405/datatypes.html#gYearMonth
pub struct GYearMonth<'a> {
    pub explicit_timezone: RequiredFacet<ExplicitTimezoneValue>,
    pub patterns: PossibleFacet<&'a [Pattern<'a>]>,
    pub enumeration: PossibleFacet<&'a [&'a str]>,
    pub max_inclusive: PossibleFacet<&'a str>,
    pub max_exclusive: PossibleFacet<&'a str>,
    pub min_inclusive: PossibleFacet<&'a str>,
    pub min_exclusive: PossibleFacet<&'a str>,
    pub assertions: PossibleFacet<&'a [Assertion<'a>]>,
}

impl<'a> GYearMonth<'a> {
    pub const GYEARMONTH: GYearMonth<'static> = GYearMonth {
        explicit_timezone: RequiredFacet::Unfixed(ExplicitTimezoneValue::Optional),
        patterns: PossibleFacet::None,
        enumeration: PossibleFacet::None,
        max_inclusive: PossibleFacet::None,
        max_exclusive: PossibleFacet::None,
        min_inclusive: PossibleFacet::None,
        min_exclusive: PossibleFacet::None,
        assertions: PossibleFacet::None,
    };
}

impl Default for GYearMonth<'_> {
    fn default() -> Self {
        Self::GYEARMONTH
    }
}

impl WhiteSpace for GYearMonth<'_> {
    fn white_space(&self) -> Option<WhiteSpaceValue> {
        RequiredFacet::Fixed(WhiteSpaceValue::Collapse).into()
    }
}

impl Ordered for GYearMonth<'_> {
    fn ordered(&self) -> OrderValue {
        OrderValue::Partial
    }
}

impl Bounded for GYearMonth<'_> {
    fn bounded(&self) -> bool {
        false
    }
}

impl Cardinality for GYearMonth<'_> {
    fn cardinality(&self) -> CardinalValue {
        CardinalValue::CountablyInfinite
    }
}

impl Numeric for GYearMonth<'_> {
    fn numeric(&self) -> bool {
        false
    }
}

impl Fundamental for GYearMonth<'_> {}

// The gYear datatype and all datatypes derived from it by restriction have the following ·constraining facets· with fixed values; these facets must not be changed from the values shown:

//     whiteSpace = collapse (fixed)

// The gYear datatype has the following ·constraining facets· with the values shown; these facets may be specified in the derivation of new types, if the value given is at least as restrictive as the one shown:

//     explicitTimezone = optional

// Datatypes derived by restriction from gYear may also specify values for the following ·constraining facets·:

//     pattern
//     enumeration
//     maxInclusive
//     maxExclusive
//     minInclusive
//     minExclusive
//     assertions

// The gYear datatype has the following values for its ·fundamental facets·:

//     ordered = partial
//     bounded = false
//     cardinality = countably infinite
//     numeric = false
/// https://www.w3.org/TR/2012/REC-xmlschema11-2-20120405/datatypes.html#gYear
pub struct GYear<'a> {
    pub explicit_timezone: RequiredFacet<ExplicitTimezoneValue>,
    pub patterns: PossibleFacet<&'a [Pattern<'a>]>,
    pub enumeration: PossibleFacet<&'a [&'a str]>,
    pub max_inclusive: PossibleFacet<&'a str>,
    pub max_exclusive: PossibleFacet<&'a str>,
    pub min_inclusive: PossibleFacet<&'a str>,
    pub min_exclusive: PossibleFacet<&'a str>,
    pub assertions: PossibleFacet<&'a [Assertion<'a>]>,
}

impl<'a> GYear<'a> {
    pub const GYEAR: GYear<'static> = GYear {
        explicit_timezone: RequiredFacet::Unfixed(ExplicitTimezoneValue::Optional),
        patterns: PossibleFacet::None,
        enumeration: PossibleFacet::None,
        max_inclusive: PossibleFacet::None,
        max_exclusive: PossibleFacet::None,
        min_inclusive: PossibleFacet::None,
        min_exclusive: PossibleFacet::None,
        assertions: PossibleFacet::None,
    };
}

impl Default for GYear<'_> {
    fn default() -> Self {
        Self::GYEAR
    }
}

impl WhiteSpace for GYear<'_> {
    fn white_space(&self) -> Option<WhiteSpaceValue> {
        RequiredFacet::Fixed(WhiteSpaceValue::Collapse).into()
    }
}

impl Ordered for GYear<'_> {
    fn ordered(&self) -> OrderValue {
        OrderValue::Partial
    }
}

impl Bounded for GYear<'_> {
    fn bounded(&self) -> bool {
        false
    }
}

impl Cardinality for GYear<'_> {
    fn cardinality(&self) -> CardinalValue {
        CardinalValue::CountablyInfinite
    }
}

impl Numeric for GYear<'_> {
    fn numeric(&self) -> bool {
        false
    }
}

impl Fundamental for GYear<'_> {}

// The gMonthDay datatype and all datatypes derived from it by restriction have the following ·constraining facets· with fixed values; these facets must not be changed from the values shown:

//     whiteSpace = collapse (fixed)

// The gMonthDay datatype has the following ·constraining facets· with the values shown; these facets may be specified in the derivation of new types, if the value given is at least as restrictive as the one shown:

//     explicitTimezone = optional

// Datatypes derived by restriction from gMonthDay may also specify values for the following ·constraining facets·:

//     pattern
//     enumeration
//     maxInclusive
//     maxExclusive
//     minInclusive
//     minExclusive
//     assertions

// The gMonthDay datatype has the following values for its ·fundamental facets·:

//     ordered = partial
//     bounded = false
//     cardinality = countably infinite
//     numeric = false
/// https://www.w3.org/TR/2012/REC-xmlschema11-2-20120405/datatypes.html#gMonthDay
pub struct GMonthDay<'a> {
    pub explicit_timezone: RequiredFacet<ExplicitTimezoneValue>,
    pub patterns: PossibleFacet<&'a [Pattern<'a>]>,
    pub enumeration: PossibleFacet<&'a [&'a str]>,
    pub max_inclusive: PossibleFacet<&'a str>,
    pub max_exclusive: PossibleFacet<&'a str>,
    pub min_inclusive: PossibleFacet<&'a str>,
    pub min_exclusive: PossibleFacet<&'a str>,
    pub assertions: PossibleFacet<&'a [Assertion<'a>]>,
}

impl<'a> GMonthDay<'a> {
    pub const GMONTHDAY: GMonthDay<'static> = GMonthDay {
        explicit_timezone: RequiredFacet::Unfixed(ExplicitTimezoneValue::Optional),
        patterns: PossibleFacet::None,
        enumeration: PossibleFacet::None,
        max_inclusive: PossibleFacet::None,
        max_exclusive: PossibleFacet::None,
        min_inclusive: PossibleFacet::None,
        min_exclusive: PossibleFacet::None,
        assertions: PossibleFacet::None,
    };
}

impl Default for GMonthDay<'_> {
    fn default() -> Self {
        Self::GMONTHDAY
    }
}

impl WhiteSpace for GMonthDay<'_> {
    fn white_space(&self) -> Option<WhiteSpaceValue> {
        RequiredFacet::Fixed(WhiteSpaceValue::Collapse).into()
    }
}

impl Ordered for GMonthDay<'_> {
    fn ordered(&self) -> OrderValue {
        OrderValue::Partial
    }
}

impl Bounded for GMonthDay<'_> {
    fn bounded(&self) -> bool {
        false
    }
}

impl Cardinality for GMonthDay<'_> {
    fn cardinality(&self) -> CardinalValue {
        CardinalValue::CountablyInfinite
    }
}

impl Numeric for GMonthDay<'_> {
    fn numeric(&self) -> bool {
        false
    }
}

impl Fundamental for GMonthDay<'_> {}

// The gDay datatype and all datatypes derived from it by restriction have the following ·constraining facets· with fixed values; these facets must not be changed from the values shown:

//     whiteSpace = collapse (fixed)

// The gDay datatype has the following ·constraining facets· with the values shown; these facets may be specified in the derivation of new types, if the value given is at least as restrictive as the one shown:

//     explicitTimezone = optional

// Datatypes derived by restriction from gDay may also specify values for the following ·constraining facets·:

//     pattern
//     enumeration
//     maxInclusive
//     maxExclusive
//     minInclusive
//     minExclusive
//     assertions

// The gDay datatype has the following values for its ·fundamental facets·:

//     ordered = partial
//     bounded = false
//     cardinality = countably infinite
//     numeric = false
/// https://www.w3.org/TR/2012/REC-xmlschema11-2-20120405/datatypes.html#gDay
pub struct GDay<'a> {
    pub explicit_timezone: RequiredFacet<ExplicitTimezoneValue>,
    pub patterns: PossibleFacet<&'a [Pattern<'a>]>,
    pub enumeration: PossibleFacet<&'a [&'a str]>,
    pub max_inclusive: PossibleFacet<&'a str>,
    pub max_exclusive: PossibleFacet<&'a str>,
    pub min_inclusive: PossibleFacet<&'a str>,
    pub min_exclusive: PossibleFacet<&'a str>,
    pub assertions: PossibleFacet<&'a [Assertion<'a>]>,
}

impl<'a> GDay<'a> {
    pub const GDAY: GDay<'static> = GDay {
        explicit_timezone: RequiredFacet::Unfixed(ExplicitTimezoneValue::Optional),
        patterns: PossibleFacet::None,
        enumeration: PossibleFacet::None,
        max_inclusive: PossibleFacet::None,
        max_exclusive: PossibleFacet::None,
        min_inclusive: PossibleFacet::None,
        min_exclusive: PossibleFacet::None,
        assertions: PossibleFacet::None,
    };
}

impl Default for GDay<'_> {
    fn default() -> Self {
        Self::GDAY
    }
}

impl WhiteSpace for GDay<'_> {
    fn white_space(&self) -> Option<WhiteSpaceValue> {
        RequiredFacet::Fixed(WhiteSpaceValue::Collapse).into()
    }
}

impl Ordered for GDay<'_> {
    fn ordered(&self) -> OrderValue {
        OrderValue::Partial
    }
}

impl Bounded for GDay<'_> {
    fn bounded(&self) -> bool {
        false
    }
}

impl Cardinality for GDay<'_> {
    fn cardinality(&self) -> CardinalValue {
        CardinalValue::CountablyInfinite
    }
}

impl Numeric for GDay<'_> {
    fn numeric(&self) -> bool {
        false
    }
}

impl Fundamental for GDay<'_> {}

// The gMonth datatype and all datatypes derived from it by restriction have the following ·constraining facets· with fixed values; these facets must not be changed from the values shown:

//     whiteSpace = collapse (fixed)

// The gMonth datatype has the following ·constraining facets· with the values shown; these facets may be specified in the derivation of new types, if the value given is at least as restrictive as the one shown:

//     explicitTimezone = optional

// Datatypes derived by restriction from gMonth may also specify values for the following ·constraining facets·:

//     pattern
//     enumeration
//     maxInclusive
//     maxExclusive
//     minInclusive
//     minExclusive
//     assertions

// The gMonth datatype has the following values for its ·fundamental facets·:

//     ordered = partial
//     bounded = false
//     cardinality = countably infinite
//     numeric = false
/// https://www.w3.org/TR/2012/REC-xmlschema11-2-20120405/datatypes.html#gMonth
pub struct GMonth<'a> {
    pub explicit_timezone: RequiredFacet<ExplicitTimezoneValue>,
    pub patterns: PossibleFacet<&'a [Pattern<'a>]>,
    pub enumeration: PossibleFacet<&'a [&'a str]>,
    pub max_inclusive: PossibleFacet<&'a str>,
    pub max_exclusive: PossibleFacet<&'a str>,
    pub min_inclusive: PossibleFacet<&'a str>,
    pub min_exclusive: PossibleFacet<&'a str>,
    pub assertions: PossibleFacet<&'a [Assertion<'a>]>,
}

impl<'a> GMonth<'a> {
    pub const GMONTH: GMonth<'static> = GMonth {
        explicit_timezone: RequiredFacet::Unfixed(ExplicitTimezoneValue::Optional),
        patterns: PossibleFacet::None,
        enumeration: PossibleFacet::None,
        max_inclusive: PossibleFacet::None,
        max_exclusive: PossibleFacet::None,
        min_inclusive: PossibleFacet::None,
        min_exclusive: PossibleFacet::None,
        assertions: PossibleFacet::None,
    };
}

impl Default for GMonth<'_> {
    fn default() -> Self {
        Self::GMONTH
    }
}

impl WhiteSpace for GMonth<'_> {
    fn white_space(&self) -> Option<WhiteSpaceValue> {
        RequiredFacet::Fixed(WhiteSpaceValue::Collapse).into()
    }
}

impl Ordered for GMonth<'_> {
    fn ordered(&self) -> OrderValue {
        OrderValue::Partial
    }
}

impl Bounded for GMonth<'_> {
    fn bounded(&self) -> bool {
        false
    }
}

impl Cardinality for GMonth<'_> {
    fn cardinality(&self) -> CardinalValue {
        CardinalValue::CountablyInfinite
    }
}

impl Numeric for GMonth<'_> {
    fn numeric(&self) -> bool {
        false
    }
}

impl Fundamental for GMonth<'_> {}

// The hexBinary datatype and all datatypes derived from it by restriction have the following ·constraining facets· with fixed values; these facets must not be changed from the values shown:

//     whiteSpace = collapse (fixed)

// Datatypes derived by restriction from hexBinary may also specify values for the following ·constraining facets·:

//     length
//     minLength
//     maxLength
//     pattern
//     enumeration
//     assertions

// The hexBinary datatype has the following values for its ·fundamental facets·:

//     ordered = false
//     bounded = false
//     cardinality = countably infinite
//     numeric = false
/// https://www.w3.org/TR/2012/REC-xmlschema11-2-20120405/datatypes.html#hexBinary
pub struct HexBinary<'a> {
    pub length: PossibleFacet<usize>,
    pub min_length: PossibleFacet<usize>,
    pub max_length: PossibleFacet<usize>,
    pub patterns: PossibleFacet<&'a [Pattern<'a>]>,
    pub enumeration: PossibleFacet<&'a [&'a str]>,
    pub assertions: PossibleFacet<&'a [Assertion<'a>]>,
}

impl<'a> HexBinary<'a> {
    pub const HEXBINARY: HexBinary<'static> = HexBinary {
        length: PossibleFacet::None,
        min_length: PossibleFacet::None,
        max_length: PossibleFacet::None,
        patterns: PossibleFacet::None,
        enumeration: PossibleFacet::None,
        assertions: PossibleFacet::None,
    };
}

impl Default for HexBinary<'_> {
    fn default() -> Self {
        Self::HEXBINARY
    }
}

impl WhiteSpace for HexBinary<'_> {
    fn white_space(&self) -> Option<WhiteSpaceValue> {
        RequiredFacet::Fixed(WhiteSpaceValue::Collapse).into()
    }
}

impl Ordered for HexBinary<'_> {
    fn ordered(&self) -> OrderValue {
        OrderValue::False
    }
}

impl Bounded for HexBinary<'_> {
    fn bounded(&self) -> bool {
        false
    }
}

impl Cardinality for HexBinary<'_> {
    fn cardinality(&self) -> CardinalValue {
        CardinalValue::CountablyInfinite
    }
}

impl Numeric for HexBinary<'_> {
    fn numeric(&self) -> bool {
        false
    }
}

impl Fundamental for HexBinary<'_> {}

// The base64Binary datatype and all datatypes derived from it by restriction have the following ·constraining facets· with fixed values; these facets must not be changed from the values shown:

//     whiteSpace = collapse (fixed)

// Datatypes derived by restriction from base64Binary may also specify values for the following ·constraining facets·:

//     length
//     minLength
//     maxLength
//     pattern
//     enumeration
//     assertions

// The base64Binary datatype has the following values for its ·fundamental facets·:

//     ordered = false
//     bounded = false
//     cardinality = countably infinite
//     numeric = false
/// https://www.w3.org/TR/2012/REC-xmlschema11-2-20120405/datatypes.html#base64Binary
pub struct Base64Binary<'a> {
    pub length: PossibleFacet<usize>,
    pub min_length: PossibleFacet<usize>,
    pub max_length: PossibleFacet<usize>,
    pub patterns: PossibleFacet<&'a [Pattern<'a>]>,
    pub enumeration: PossibleFacet<&'a [&'a str]>,
    pub assertions: PossibleFacet<&'a [Assertion<'a>]>,
}

impl<'a> Base64Binary<'a> {
    pub const BASE64BINARY: Base64Binary<'static> = Base64Binary {
        length: PossibleFacet::None,
        min_length: PossibleFacet::None,
        max_length: PossibleFacet::None,
        patterns: PossibleFacet::None,
        enumeration: PossibleFacet::None,
        assertions: PossibleFacet::None,
    };
}

impl Default for Base64Binary<'_> {
    fn default() -> Self {
        Self::BASE64BINARY
    }
}

impl WhiteSpace for Base64Binary<'_> {
    fn white_space(&self) -> Option<WhiteSpaceValue> {
        RequiredFacet::Fixed(WhiteSpaceValue::Collapse).into()
    }
}

impl Ordered for Base64Binary<'_> {
    fn ordered(&self) -> OrderValue {
        OrderValue::False
    }
}

impl Bounded for Base64Binary<'_> {
    fn bounded(&self) -> bool {
        false
    }
}

impl Cardinality for Base64Binary<'_> {
    fn cardinality(&self) -> CardinalValue {
        CardinalValue::CountablyInfinite
    }
}

impl Numeric for Base64Binary<'_> {
    fn numeric(&self) -> bool {
        false
    }
}

impl Fundamental for Base64Binary<'_> {}

// The anyURI datatype and all datatypes derived from it by restriction have the following ·constraining facets· with fixed values; these facets must not be changed from the values shown:

//     whiteSpace = collapse (fixed)

// Datatypes derived by restriction from anyURI may also specify values for the following ·constraining facets·:

//     length
//     minLength
//     maxLength
//     pattern
//     enumeration
//     assertions

// The anyURI datatype has the following values for its ·fundamental facets·:

//     ordered = false
//     bounded = false
//     cardinality = countably infinite
//     numeric = false
/// https://www.w3.org/TR/2012/REC-xmlschema11-2-20120405/datatypes.html#anyURI
pub struct AnyURI<'a> {
    pub length: PossibleFacet<usize>,
    pub min_length: PossibleFacet<usize>,
    pub max_length: PossibleFacet<usize>,
    pub patterns: PossibleFacet<&'a [Pattern<'a>]>,
    pub enumeration: PossibleFacet<&'a [&'a str]>,
    pub assertions: PossibleFacet<&'a [Assertion<'a>]>,
}

impl<'a> AnyURI<'a> {
    pub const ANYURI: AnyURI<'static> = AnyURI {
        length: PossibleFacet::None,
        min_length: PossibleFacet::None,
        max_length: PossibleFacet::None,
        patterns: PossibleFacet::None,
        enumeration: PossibleFacet::None,
        assertions: PossibleFacet::None,
    };
}

impl Default for AnyURI<'_> {
    fn default() -> Self {
        Self::ANYURI
    }
}

impl WhiteSpace for AnyURI<'_> {
    fn white_space(&self) -> Option<WhiteSpaceValue> {
        RequiredFacet::Fixed(WhiteSpaceValue::Collapse).into()
    }
}

impl Ordered for AnyURI<'_> {
    fn ordered(&self) -> OrderValue {
        OrderValue::False
    }
}

impl Bounded for AnyURI<'_> {
    fn bounded(&self) -> bool {
        false
    }
}

impl Cardinality for AnyURI<'_> {
    fn cardinality(&self) -> CardinalValue {
        CardinalValue::CountablyInfinite
    }
}

impl Numeric for AnyURI<'_> {
    fn numeric(&self) -> bool {
        false
    }
}

impl Fundamental for AnyURI<'_> {}

// The QName datatype and all datatypes derived from it by restriction have the following ·constraining facets· with fixed values; these facets must not be changed from the values shown:

//     whiteSpace = collapse (fixed)

// Datatypes derived by restriction from QName may also specify values for the following ·constraining facets·:

//     length
//     minLength
//     maxLength
//     pattern
//     enumeration
//     assertions

// The QName datatype has the following values for its ·fundamental facets·:

//     ordered = false
//     bounded = false
//     cardinality = countably infinite
//     numeric = false
/// https://www.w3.org/TR/2012/REC-xmlschema11-2-20120405/datatypes.html#QName
pub struct QName<'a> {
    pub length: PossibleFacet<usize>,
    pub min_length: PossibleFacet<usize>,
    pub max_length: PossibleFacet<usize>,
    pub patterns: PossibleFacet<&'a [Pattern<'a>]>,
    pub enumeration: PossibleFacet<&'a [&'a str]>,
    pub assertions: PossibleFacet<&'a [Assertion<'a>]>,
}

impl<'a> QName<'a> {
    pub const QNAME: QName<'static> = QName {
        length: PossibleFacet::None,
        min_length: PossibleFacet::None,
        max_length: PossibleFacet::None,
        patterns: PossibleFacet::None,
        enumeration: PossibleFacet::None,
        assertions: PossibleFacet::None,
    };
}

impl Default for QName<'_> {
    fn default() -> Self {
        Self::QNAME
    }
}

impl WhiteSpace for QName<'_> {
    fn white_space(&self) -> Option<WhiteSpaceValue> {
        RequiredFacet::Fixed(WhiteSpaceValue::Collapse).into()
    }
}

impl Ordered for QName<'_> {
    fn ordered(&self) -> OrderValue {
        OrderValue::False
    }
}

impl Bounded for QName<'_> {
    fn bounded(&self) -> bool {
        false
    }
}

impl Cardinality for QName<'_> {
    fn cardinality(&self) -> CardinalValue {
        CardinalValue::CountablyInfinite
    }
}

impl Numeric for QName<'_> {
    fn numeric(&self) -> bool {
        false
    }
}

impl Fundamental for QName<'_> {}

// The NOTATION datatype and all datatypes derived from it by restriction have the following ·constraining facets· with fixed values; these facets must not be changed from the values shown:

//     whiteSpace = collapse (fixed)

// Datatypes derived by restriction from NOTATION may also specify values for the following ·constraining facets·:

//     length
//     minLength
//     maxLength
//     pattern
//     enumeration
//     assertions

// The NOTATION datatype has the following values for its ·fundamental facets·:

//     ordered = false
//     bounded = false
//     cardinality = countably infinite
//     numeric = false
/// https://www.w3.org/TR/2012/REC-xmlschema11-2-20120405/datatypes.html#NOTATION
pub struct NOTATION<'a> {
    pub length: PossibleFacet<usize>,
    pub min_length: PossibleFacet<usize>,
    pub max_length: PossibleFacet<usize>,
    pub patterns: PossibleFacet<&'a [Pattern<'a>]>,
    pub enumeration: PossibleFacet<&'a [&'a str]>,
    pub assertions: PossibleFacet<&'a [Assertion<'a>]>,
}

impl<'a> NOTATION<'a> {
    pub const NOTATION: NOTATION<'static> = NOTATION {
        length: PossibleFacet::None,
        min_length: PossibleFacet::None,
        max_length: PossibleFacet::None,
        patterns: PossibleFacet::None,
        enumeration: PossibleFacet::None,
        assertions: PossibleFacet::None,
    };
}

impl Default for NOTATION<'_> {
    fn default() -> Self {
        Self::NOTATION
    }
}

impl WhiteSpace for NOTATION<'_> {
    fn white_space(&self) -> Option<WhiteSpaceValue> {
        RequiredFacet::Fixed(WhiteSpaceValue::Collapse).into()
    }
}

impl Ordered for NOTATION<'_> {
    fn ordered(&self) -> OrderValue {
        OrderValue::False
    }
}

impl Bounded for NOTATION<'_> {
    fn bounded(&self) -> bool {
        false
    }
}

impl Cardinality for NOTATION<'_> {
    fn cardinality(&self) -> CardinalValue {
        CardinalValue::CountablyInfinite
    }
}

impl Numeric for NOTATION<'_> {
    fn numeric(&self) -> bool {
        false
    }
}

impl Fundamental for NOTATION<'_> {}
