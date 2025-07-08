#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PossibleFacet<T> {
    Fixed(T),
    Unfixed(T),
    None,
}

impl<T> From<PossibleFacet<T>> for Option<T> {
    fn from(value: PossibleFacet<T>) -> Self {
        match value {
            PossibleFacet::Fixed(v) => Some(v),
            PossibleFacet::Unfixed(v) => Some(v),
            PossibleFacet::None => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RequiredFacet<T> {
    Fixed(T),
    Unfixed(T),
}

impl<T> From<RequiredFacet<T>> for Option<T> {
    fn from(value: RequiredFacet<T>) -> Self {
        match value {
            RequiredFacet::Fixed(v) => Some(v),
            RequiredFacet::Unfixed(v) => Some(v),
        }
    }
}

pub mod fundamental {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum OrderValue {
        Total,
        Partial,
        False,
    }

    pub trait Ordered {
        fn ordered(&self) -> OrderValue;
    }

    pub trait Bounded {
        fn bounded(&self) -> bool;
    }

    pub enum CardinalValue {
        Finite,
        CountablyInfinite,
    }

    pub trait Cardinality {
        fn cardinality(&self) -> CardinalValue;
    }

    pub trait Numeric {
        fn numeric(&self) -> bool;
    }

    pub trait Fundamental: Ordered + Bounded + Cardinality + Numeric {}
}

pub mod numeric {
    use std::num::NonZeroUsize;

    pub trait MinInclusive {
        fn min_inclusive(&self) -> Option<&str>;
    }

    pub trait MaxInclusive {
        fn max_inclusive(&self) -> Option<&str>;
    }

    pub trait MinExclusive {
        fn min_exclusive(&self) -> Option<&str>;
    }

    pub trait MaxExclusive {
        fn max_exclusive(&self) -> Option<&str>;
    }

    pub trait TotalDigits {
        fn total_digits(&self) -> Option<NonZeroUsize>;
    }

    pub trait FractionDigits {
        fn fraction_digits(&self) -> Option<usize>;
    }
}

pub mod special {

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum WhiteSpaceValue {
        Preserve,
        Replace,
        Collapse,
    }

    pub trait WhiteSpace {
        fn white_space(&self) -> Option<WhiteSpaceValue>;
    }

    pub enum ExplicitTimezoneValue {
        Required,
        Prohibited,
        Optional,
    }

    pub trait ExplicitTimezone {
        fn explicit_timezone(&self) -> Option<ExplicitTimezoneValue>;
    }
}
