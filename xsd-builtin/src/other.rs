/// https://www.w3.org/TR/2012/REC-xmlschema11-2-20120405/datatypes.html#ordinary-built-ins
use crate::{
    facets::{
        special::{ExplicitTimezoneValue, WhiteSpaceValue},
        PossibleFacet, RequiredFacet,
    },
    primitives::{DateTime, Decimal, Duration, Pattern, String},
    List,
};

impl String<'_> {
    /// https://www.w3.org/TR/2012/REC-xmlschema11-2-20120405/datatypes.html#normalizedString
    const NORMALIZED_STRING: String<'static> = String {
        white_space: RequiredFacet::Unfixed(WhiteSpaceValue::Replace),
        ..Self::STRING
    };

    /// https://www.w3.org/TR/2012/REC-xmlschema11-2-20120405/datatypes.html#token
    pub const TOKEN: String<'static> = String {
        white_space: RequiredFacet::Unfixed(WhiteSpaceValue::Collapse),
        ..Self::NORMALIZED_STRING
    };

    /// https://www.w3.org/TR/2012/REC-xmlschema11-2-20120405/datatypes.html#language
    pub const LANGUAGE: String<'static> = String {
        patterns: PossibleFacet::Unfixed(&[Pattern(r###"[a-zA-Z]{1,8}(-[a-zA-Z0-9]{1,8})*"###)]),
        ..Self::TOKEN
    };

    /// https://www.w3.org/TR/2012/REC-xmlschema11-2-20120405/datatypes.html#NMTOKEN
    pub const NMTOKEN: String<'static> = String {
        patterns: PossibleFacet::Unfixed(&[Pattern(r###"\c+"###)]),
        ..Self::TOKEN
    };

    /// https://www.w3.org/TR/2012/REC-xmlschema11-2-20120405/datatypes.html#NMTOKENS
    pub const NMTOKENS: List = List {
      // Item: NMTOKEN
    };

    /// https://www.w3.org/TR/2012/REC-xmlschema11-2-20120405/datatypes.html#Name
    pub const NAME: String<'static> = String {
        patterns: PossibleFacet::Unfixed(&[Pattern(r###"\i\c*"###)]),
        ..Self::TOKEN
    };

    /// https://www.w3.org/TR/2012/REC-xmlschema11-2-20120405/datatypes.html#NCName
    pub const NCNAME: String<'static> = String {
        patterns: PossibleFacet::Unfixed(&[
            Pattern(r###"\i\c*"###),
            Pattern(r###"[\i-[:]][\c-[:]]*"###),
        ]),
        ..Self::NAME
    };

    /// https://www.w3.org/TR/2012/REC-xmlschema11-2-20120405/datatypes.html#ID
    pub const ID: String<'static> = String {
        patterns: PossibleFacet::Unfixed(&[
            Pattern(r###"\i\c*"###),
            Pattern(r###"[\i-[:]][\c-[:]]*"###),
        ]),
        ..Self::NCNAME
    };

    /// https://www.w3.org/TR/2012/REC-xmlschema11-2-20120405/datatypes.html#IDREF
    pub const IDREF: String<'static> = String {
        patterns: PossibleFacet::Unfixed(&[
            Pattern(r###"\i\c*"###),
            Pattern(r###"[\i-[:]][\c-[:]]*"###),
        ]),
        ..Self::NCNAME
    };

    /// https://www.w3.org/TR/2012/REC-xmlschema11-2-20120405/datatypes.html#IDREFS
    pub const IDREFS: List = List {
        // Item: IDREF
    };

    /// https://www.w3.org/TR/2012/REC-xmlschema11-2-20120405/datatypes.html#ENTITY
    pub const ENTITY: String<'static> = String {
        patterns: PossibleFacet::Unfixed(&[
            Pattern(r###"\i\c*"###),
            Pattern(r###"[\i-[:]][\c-[:]]*"###),
        ]),
        ..Self::NCNAME
    };

    /// https://www.w3.org/TR/2012/REC-xmlschema11-2-20120405/datatypes.html#ENTITIES
    pub const ENTITIES: List = List {
      // Item: ENTITY
    };
}

impl Decimal<'_> {
    /// https://www.w3.org/TR/2012/REC-xmlschema11-2-20120405/datatypes.html#integer
    pub const INTEGER: Decimal<'static> = Decimal {
        fraction_digits: PossibleFacet::Fixed(0),
        patterns: PossibleFacet::Unfixed(&[Pattern(r###"[\-+]?[0-9]+"###)]),
        ..Self::DECIMAL
    };

    /// https://www.w3.org/TR/2012/REC-xmlschema11-2-20120405/datatypes.html#nonPositiveInteger
    pub const NON_POSITIVE_INTEGER: Decimal<'static> = Decimal {
        max_inclusive: PossibleFacet::Unfixed("0"),
        ..Self::INTEGER
    };

    /// https://www.w3.org/TR/2012/REC-xmlschema11-2-20120405/datatypes.html#negativeInteger
    pub const NEGATIVE_INTEGER: Decimal<'static> = Decimal {
        max_inclusive: PossibleFacet::Unfixed("-1"),
        ..Self::NON_POSITIVE_INTEGER
    };

    /// https://www.w3.org/TR/2012/REC-xmlschema11-2-20120405/datatypes.html#long
    pub const LONG: Decimal<'static> = Decimal {
        max_inclusive: PossibleFacet::Unfixed("9223372036854775807"),
        min_inclusive: PossibleFacet::Unfixed("-9223372036854775808"),
        ..Self::INTEGER
    };

    /// https://www.w3.org/TR/2012/REC-xmlschema11-2-20120405/datatypes.html#int
    pub const INT: Decimal<'static> = Decimal {
        max_inclusive: PossibleFacet::Unfixed("2147483647"),
        min_inclusive: PossibleFacet::Unfixed("-2147483648"),
        ..Self::LONG
    };

    /// https://www.w3.org/TR/2012/REC-xmlschema11-2-20120405/datatypes.html#short
    pub const SHORT: Decimal<'static> = Decimal {
        max_inclusive: PossibleFacet::Unfixed("32767"),
        min_inclusive: PossibleFacet::Unfixed("-32768"),
        ..Self::INT
    };

    /// https://www.w3.org/TR/2012/REC-xmlschema11-2-20120405/datatypes.html#byte
    pub const BYTE: Decimal<'static> = Decimal {
        max_inclusive: PossibleFacet::Unfixed("127"),
        min_inclusive: PossibleFacet::Unfixed("-128"),
        ..Self::SHORT
    };

    /// https://www.w3.org/TR/2012/REC-xmlschema11-2-20120405/datatypes.html#nonNegativeInteger
    pub const NON_NEGATIVE_INTEGER: Decimal<'static> = Decimal {
        min_inclusive: PossibleFacet::Unfixed("0"),
        ..Self::INTEGER
    };

    /// https://www.w3.org/TR/2012/REC-xmlschema11-2-20120405/datatypes.html#unsignedLong
    pub const UNSIGNED_LONG: Decimal<'static> = Decimal {
        max_inclusive: PossibleFacet::Unfixed("18446744073709551615"),
        ..Self::NON_NEGATIVE_INTEGER
    };

    /// https://www.w3.org/TR/2012/REC-xmlschema11-2-20120405/datatypes.html#unsignedInt
    pub const UNSIGNED_INT: Decimal<'static> = Decimal {
        max_inclusive: PossibleFacet::Unfixed("4294967295"),
        ..Self::UNSIGNED_LONG
    };

    /// https://www.w3.org/TR/2012/REC-xmlschema11-2-20120405/datatypes.html#unsignedShort
    pub const UNSIGNED_SHORT: Decimal<'static> = Decimal {
        max_inclusive: PossibleFacet::Unfixed("65535"),
        ..Self::UNSIGNED_INT
    };

    /// https://www.w3.org/TR/2012/REC-xmlschema11-2-20120405/datatypes.html#unsignedByte
    pub const UNSIGNED_BYTE: Decimal<'static> = Decimal {
        max_inclusive: PossibleFacet::Unfixed("255"),
        ..Self::UNSIGNED_SHORT
    };

    /// https://www.w3.org/TR/2012/REC-xmlschema11-2-20120405/datatypes.html#positiveInteger
    pub const POSITIVE_INTEGER: Decimal<'static> = Decimal {
        min_inclusive: PossibleFacet::Unfixed("1"),
        ..Self::NON_NEGATIVE_INTEGER
    };
}

impl Duration<'_> {
    /// https://www.w3.org/TR/2012/REC-xmlschema11-2-20120405/datatypes.html#yearMonthDuration
    pub const YEAR_MONTH_DURATION: Duration<'static> = Duration {
        //TODO: This is the formal definition, but it is possible better to use the more optimized pattern.
        patterns: RequiredFacet::Unfixed(&[Self::DURATION_PATTERN, r###"[^DT]*"###]),
        ..Duration::DURATION
    };

    // https://www.w3.org/TR/2012/REC-xmlschema11-2-20120405/datatypes.html#dayTimeDuration
    pub const DAY_TIME_DURATION: Duration<'static> = Duration {
        //TODO: This is the formal definition, but it is possible better to use the more optimized pattern.
        patterns: RequiredFacet::Unfixed(&[Self::DURATION_PATTERN, r###"[^YM]*[DT].*"###]),
        ..Duration::DURATION
    };
}

impl DateTime<'_> {
    /// https://www.w3.org/TR/2012/REC-xmlschema11-2-20120405/datatypes.html#dateTimeStamp
    pub const DATE_TIME_STAMP: DateTime<'static> = DateTime {
        explicit_timezone: RequiredFacet::Fixed(ExplicitTimezoneValue::Required),
        ..DateTime::DATE_TIME
    };
}
