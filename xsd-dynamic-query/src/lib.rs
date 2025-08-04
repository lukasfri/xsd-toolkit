//! This crate provides utilities for dynamically querying XML Schema (XSD) definitions
//! and extracting information about elements, attributes, and types.
use std::num::NonZeroUsize;

use xmlity::ExpandedName;
use xsd_fragments::fragments::simple::{
    self as sm, Assertion, ExplicitTimezoneValue, Pattern, Value,
};

/// Error types for dynamic query operations.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Named type was not found.
    #[error("Fragment not found in the context")]
    TypeNotFound {
        /// The name that was not found.
        name: ExpandedName<'static>,
    },
    /// The named type is not a simple type.
    #[error("Type {name} is not a simple type")]
    TypeNotSimpleType {
        /// The name of the type that is not simple.
        name: ExpandedName<'static>,
    },
    /// Fragment was not found in the context.
    #[error("Fragment not found in the context")]
    FragmentNotFound {},
}

/// A collection of parsed facets from an XML Schema simple type.
/// This struct is used to hold various facets such as length, min/max values,
/// enumerations, patterns, and assertions that are extracted from the schema.
#[derive(Default, Debug)]
pub struct ParsedFacets<'a> {
    /// Minimum length facet.
    pub min_length: Option<&'a usize>,
    /// Maximum length facet.
    pub max_length: Option<&'a usize>,
    /// Exact length facet.
    pub length: Option<&'a usize>,
    /// Minimum inclusive value facet.
    pub min_inclusive: Option<&'a Value>,
    /// Minimum exclusive value facet.
    pub min_exclusive: Option<&'a Value>,
    /// Maximum inclusive value facet.
    pub max_inclusive: Option<&'a Value>,
    /// Maximum exclusive value facet.
    pub max_exclusive: Option<&'a Value>,
    /// Whitespace handling facet.
    pub white_space: Option<&'a sm::WhiteSpaceValue>,
    /// Enumeration values.
    pub enumerations: Vec<&'a Value>,
    /// Pattern restrictions.
    pub patterns: Vec<&'a Pattern>,
    /// Assertion restrictions.
    pub assertions: Vec<&'a Assertion>,
    /// Total digits facet.
    pub total_digits: Option<&'a NonZeroUsize>,
    /// Fraction digits facet.
    pub fraction_digits: Option<&'a usize>,
    /// Explicit timezone facet.
    pub explicit_timezone: Option<&'a ExplicitTimezoneValue>,
}

impl<'a> ParsedFacets<'a> {
    /// Add a facet to the parsed facets
    /// Returns true if the facet was already present
    pub fn add_facet(&mut self, facet: &'a sm::FacetFragment) -> bool {
        match facet {
            sm::FacetFragment::Length { value } => self.length.replace(value).is_some(),
            sm::FacetFragment::MinLength { value } => self.min_length.replace(value).is_some(),
            sm::FacetFragment::MaxLength { value } => self.max_length.replace(value).is_some(),
            sm::FacetFragment::MinExclusive { value } => {
                self.min_exclusive.replace(value).is_some()
            }
            sm::FacetFragment::MinInclusive { value } => {
                self.min_inclusive.replace(value).is_some()
            }
            sm::FacetFragment::MaxExclusive { value } => {
                self.max_exclusive.replace(value).is_some()
            }
            sm::FacetFragment::MaxInclusive { value } => {
                self.max_inclusive.replace(value).is_some()
            }
            sm::FacetFragment::Enumeration { value } => {
                if self
                    .enumerations
                    .iter()
                    .any(|e| e.0.trim() == value.0.trim())
                {
                    true // The enumeration is already present
                } else {
                    self.enumerations.push(value);
                    false
                }
            }
            sm::FacetFragment::TotalDigits { value } => self.total_digits.replace(value).is_some(),
            sm::FacetFragment::FractionDigits { value } => {
                self.fraction_digits.replace(value).is_some()
            }
            sm::FacetFragment::WhiteSpace { value } => self.white_space.replace(value).is_some(),
            sm::FacetFragment::Pattern { value } => {
                if self.patterns.iter().any(|e| e.0.trim() == value.0.trim()) {
                    true // The pattern is already present
                } else {
                    self.patterns.push(value);
                    false
                }
            }
            sm::FacetFragment::Assertion { test } => {
                let Some(test) = test.as_ref() else {
                    return false; // No assertion to add
                };

                if self.assertions.contains(&test) {
                    true // The assertion is already present
                } else {
                    self.assertions.push(test);
                    false
                }
            }
            sm::FacetFragment::ExplicitTimezone { value } => {
                if self.explicit_timezone.replace(value).is_some() {
                    true // The explicit timezone is already present
                } else {
                    false // The explicit timezone was added
                }
            }
        }
    }
}

impl<'a> FromIterator<&'a sm::FacetFragment> for ParsedFacets<'a> {
    fn from_iter<T: IntoIterator<Item = &'a sm::FacetFragment>>(iter: T) -> Self {
        let mut facets = ParsedFacets::default();
        facets.extend(iter);
        facets
    }
}

impl<'a> Extend<&'a sm::FacetFragment> for ParsedFacets<'a> {
    fn extend<T: IntoIterator<Item = &'a sm::FacetFragment>>(&mut self, iter: T) {
        iter.into_iter().for_each(|facet| {
            self.add_facet(facet);
        });
    }
}
