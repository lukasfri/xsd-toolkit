use std::num::NonZeroUsize;

use xmlity::ExpandedName;
use xsd_fragments::fragments::simple::{
    self as sm, Assertion, ExplicitTimezoneValue, Pattern, Value,
};

pub mod identify_simple_type;
pub use identify_simple_type::IdentifySimpleType;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Fragment not found in the context")]
    TypeNotFound { name: ExpandedName<'static> },
    #[error("Type {name} is not a simple type")]
    TypeNotSimpleType { name: ExpandedName<'static> },
    #[error("Fragment not found in the context")]
    FragmentNotFound {},
}

#[derive(Default, Debug)]
pub struct ParsedFacets<'a> {
    pub min_length: Option<&'a usize>,
    pub max_length: Option<&'a usize>,
    pub length: Option<&'a usize>,
    pub min_inclusive: Option<&'a Value>,
    pub min_exclusive: Option<&'a Value>,
    pub max_inclusive: Option<&'a Value>,
    pub max_exclusive: Option<&'a Value>,
    pub white_space: Option<&'a sm::WhiteSpaceValue>,
    pub enumerations: Vec<&'a Value>,
    pub patterns: Vec<&'a Pattern>,
    pub assertions: Vec<&'a Assertion>,
    pub total_digits: Option<&'a NonZeroUsize>,
    pub fraction_digits: Option<&'a usize>,
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
