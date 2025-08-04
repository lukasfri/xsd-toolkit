//! Simple type fragments for XSD processing.

use std::{any::type_name, num::NonZeroUsize};
use xsd::{ns, xs};

use xmlity::{ExpandedName, LocalName, XmlNamespace};

use crate::{
    fragments::{
        complex::XmlNamespaceExt, FragmentAccess, FragmentCollection, FragmentIdx,
        HasFragmentCollection, NamespaceIdx,
    },
    NamedOrAnonymous,
};
use std::collections::VecDeque;

/// Fragment representing a simple type restriction.
#[derive(Debug, Clone, PartialEq)]
pub struct RestrictionFragment {
    /// The base type being restricted.
    pub base: Option<ExpandedName<'static>>,
    /// Facets applied in this restriction.
    pub facets: Vec<FragmentIdx<FacetFragment>>,
    /// Inline simple type definition.
    pub simple_type: Option<FragmentIdx<SimpleTypeRootFragment>>,
    /// ID attribute for the restriction.
    pub id: Option<String>,
}

/// Root fragment for a simple type definition.
#[derive(Debug, Clone, PartialEq)]
pub struct SimpleTypeRootFragment {
    /// Name of the simple type (None for anonymous types).
    pub name: Option<LocalName<'static>>,
    /// How the simple type is derived.
    pub simple_derivation: SimpleDerivation,
}

/// Fragment representing a list type.
#[derive(Debug, Clone, PartialEq)]
pub struct ListFragment {
    /// Type of items in the list.
    pub item_type: NamedOrAnonymous<FragmentIdx<SimpleTypeRootFragment>>,
    /// ID attribute for the list.
    pub id: Option<String>,
}

/// Fragment representing a union type.
#[derive(Debug, Clone, PartialEq)]
pub struct UnionFragment {
    /// Named member types.
    pub member_types: VecDeque<ExpandedName<'static>>,
    /// Inline simple type definitions.
    pub simple_types: VecDeque<FragmentIdx<SimpleTypeRootFragment>>,
    /// ID attribute for the union.
    pub id: Option<String>,
}

/// Fragment representing a group reference.
#[derive(Debug, Clone, PartialEq)]
pub struct GroupRefFragment {
    /// Reference to the group.
    pub ref_: ExpandedName<'static>,
}

/// A value used in facets.
#[derive(Debug, Clone, PartialEq)]
pub struct Value(pub String);

/// A pattern used in facets.
#[derive(Debug, Clone, PartialEq)]
pub struct Pattern(pub String);

/// White space handling options.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WhiteSpaceValue {
    /// Preserve all whitespace.
    Preserve,
    /// Replace tabs and newlines with spaces.
    Replace,
    /// Collapse consecutive whitespace into single spaces.
    Collapse,
}

impl From<xs::white_space_items::ValueValue> for WhiteSpaceValue {
    fn from(value: xs::white_space_items::ValueValue) -> Self {
        match value {
            xs::white_space_items::ValueValue::Preserve => WhiteSpaceValue::Preserve,
            xs::white_space_items::ValueValue::Replace => WhiteSpaceValue::Replace,
            xs::white_space_items::ValueValue::Collapse => WhiteSpaceValue::Collapse,
        }
    }
}

impl From<WhiteSpaceValue> for xs::white_space_items::ValueValue {
    fn from(value: WhiteSpaceValue) -> Self {
        match value {
            WhiteSpaceValue::Preserve => xs::white_space_items::ValueValue::Preserve,
            WhiteSpaceValue::Replace => xs::white_space_items::ValueValue::Replace,
            WhiteSpaceValue::Collapse => xs::white_space_items::ValueValue::Collapse,
        }
    }
}

/// Explicit timezone handling options.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ExplicitTimezoneValue {
    /// Timezone is required.
    Required,
    /// Timezone is prohibited.
    Prohibited,
    /// Timezone is optional.
    Optional,
}

impl From<xs::explicit_timezone_items::ValueValue> for ExplicitTimezoneValue {
    fn from(value: xs::explicit_timezone_items::ValueValue) -> Self {
        match value {
            xs::explicit_timezone_items::ValueValue::Required => ExplicitTimezoneValue::Required,
            xs::explicit_timezone_items::ValueValue::Prohibited => {
                ExplicitTimezoneValue::Prohibited
            }
            xs::explicit_timezone_items::ValueValue::Optional => ExplicitTimezoneValue::Optional,
        }
    }
}

impl From<ExplicitTimezoneValue> for xs::explicit_timezone_items::ValueValue {
    fn from(value: ExplicitTimezoneValue) -> Self {
        match value {
            ExplicitTimezoneValue::Required => xs::explicit_timezone_items::ValueValue::Required,
            ExplicitTimezoneValue::Prohibited => {
                xs::explicit_timezone_items::ValueValue::Prohibited
            }
            ExplicitTimezoneValue::Optional => xs::explicit_timezone_items::ValueValue::Optional,
        }
    }
}

/// An assertion for simple types.
#[derive(Debug, Clone, PartialEq)]
pub struct Assertion(pub String);

/// Fragment representing various facets that can be applied to simple types.
#[derive(Debug, Clone, PartialEq)]
pub enum FacetFragment {
    /// Exact length facet.
    Length {
        /// The required length.
        value: usize,
    },
    /// Minimum length facet.
    MinLength {
        /// The minimum length.
        value: usize,
    },
    /// Maximum length facet.
    MaxLength {
        /// The maximum length.
        value: usize,
    },
    /// Minimum exclusive bound facet.
    MinExclusive {
        /// The exclusive minimum value.
        value: Value,
    },
    /// Minimum inclusive bound facet.
    MinInclusive {
        /// The inclusive minimum value.
        value: Value,
    },
    /// Maximum exclusive bound facet.
    MaxExclusive {
        /// The exclusive maximum value.
        value: Value,
    },
    /// Maximum inclusive bound facet.
    MaxInclusive {
        /// The inclusive maximum value.
        value: Value,
    },
    /// Enumeration facet.
    Enumeration {
        /// An allowed enumeration value.
        value: Value,
    },
    /// Total digits facet for decimal types.
    TotalDigits {
        /// Maximum number of digits.
        value: NonZeroUsize,
    },
    /// Fraction digits facet for decimal types.
    FractionDigits {
        /// Number of fractional digits.
        value: usize,
    },
    /// White space handling facet.
    WhiteSpace {
        /// How to handle whitespace.
        value: WhiteSpaceValue,
    },
    /// Pattern facet.
    Pattern {
        /// Regular expression pattern.
        value: Pattern,
    },
    /// Assertion facet.
    Assertion {
        /// XPath test expression.
        test: Option<Assertion>,
    },
    /// Explicit timezone facet.
    ExplicitTimezone {
        /// Timezone requirement.
        value: ExplicitTimezoneValue,
    },
}

/// Compiler for simple type fragments.
#[derive(Debug, Clone)]
pub struct SimpleTypeFragmentCompiler {
    namespace: XmlNamespace<'static>,
    simple_types: FragmentCollection<SimpleTypeRootFragment>,
    restrictions: FragmentCollection<RestrictionFragment>,
    facets: FragmentCollection<FacetFragment>,
    lists: FragmentCollection<ListFragment>,
    unions: FragmentCollection<UnionFragment>,
    group_refs: FragmentCollection<GroupRefFragment>,
}

impl AsMut<SimpleTypeFragmentCompiler> for SimpleTypeFragmentCompiler {
    fn as_mut(&mut self) -> &mut SimpleTypeFragmentCompiler {
        self
    }
}

impl AsRef<SimpleTypeFragmentCompiler> for SimpleTypeFragmentCompiler {
    fn as_ref(&self) -> &SimpleTypeFragmentCompiler {
        self
    }
}

impl SimpleTypeFragmentCompiler {
    /// Creates a new [`SimpleTypeFragmentCompiler`] with the given namespace and namespace index.
    pub fn new(namespace: XmlNamespace<'static>, namespace_idx: NamespaceIdx) -> Self {
        Self {
            namespace,
            simple_types: FragmentCollection::new(namespace_idx),
            restrictions: FragmentCollection::new(namespace_idx),
            facets: FragmentCollection::new(namespace_idx),
            lists: FragmentCollection::new(namespace_idx),
            unions: FragmentCollection::new(namespace_idx),
            group_refs: FragmentCollection::new(namespace_idx),
        }
    }
}

impl HasFragmentCollection<SimpleTypeRootFragment> for SimpleTypeFragmentCompiler {
    fn get_fragment_collection(&self) -> &FragmentCollection<SimpleTypeRootFragment> {
        &self.simple_types
    }
    fn get_fragment_collection_mut(&mut self) -> &mut FragmentCollection<SimpleTypeRootFragment> {
        &mut self.simple_types
    }
}

impl HasFragmentCollection<RestrictionFragment> for SimpleTypeFragmentCompiler {
    fn get_fragment_collection(&self) -> &FragmentCollection<RestrictionFragment> {
        &self.restrictions
    }
    fn get_fragment_collection_mut(&mut self) -> &mut FragmentCollection<RestrictionFragment> {
        &mut self.restrictions
    }
}

impl HasFragmentCollection<FacetFragment> for SimpleTypeFragmentCompiler {
    fn get_fragment_collection(&self) -> &FragmentCollection<FacetFragment> {
        &self.facets
    }
    fn get_fragment_collection_mut(&mut self) -> &mut FragmentCollection<FacetFragment> {
        &mut self.facets
    }
}

impl HasFragmentCollection<ListFragment> for SimpleTypeFragmentCompiler {
    fn get_fragment_collection(&self) -> &FragmentCollection<ListFragment> {
        &self.lists
    }
    fn get_fragment_collection_mut(&mut self) -> &mut FragmentCollection<ListFragment> {
        &mut self.lists
    }
}

impl HasFragmentCollection<UnionFragment> for SimpleTypeFragmentCompiler {
    fn get_fragment_collection(&self) -> &FragmentCollection<UnionFragment> {
        &self.unions
    }
    fn get_fragment_collection_mut(&mut self) -> &mut FragmentCollection<UnionFragment> {
        &mut self.unions
    }
}

impl HasFragmentCollection<GroupRefFragment> for SimpleTypeFragmentCompiler {
    fn get_fragment_collection(&self) -> &FragmentCollection<GroupRefFragment> {
        &self.group_refs
    }
    fn get_fragment_collection_mut(&mut self) -> &mut FragmentCollection<GroupRefFragment> {
        &mut self.group_refs
    }
}

impl<T: 'static> FragmentAccess<T> for SimpleTypeFragmentCompiler
where
    SimpleTypeFragmentCompiler: HasFragmentCollection<T>,
{
    fn get_fragment(&self, fragment_id: &FragmentIdx<T>) -> Option<&T> {
        self.get_fragment_collection().get_fragment(fragment_id)
    }

    fn get_fragment_mut(&mut self, fragment_id: &FragmentIdx<T>) -> Option<&mut T> {
        self.get_fragment_collection_mut()
            .get_fragment_mut(fragment_id)
    }

    fn push_fragment(&mut self, fragment: T) -> FragmentIdx<T> {
        self.get_fragment_collection_mut().push_fragment(fragment)
    }

    fn iter_fragment_ids(&self) -> Vec<FragmentIdx<T>> {
        self.get_fragment_collection().iter_fragment_ids()
    }
}

/// Error type for [`SimpleTypeFragmentCompiler`] operations.
#[derive(Debug, Clone)]
pub enum Error {
    /// List type is missing its item type definition.
    ListMissingType,
    /// Substitution group is not supported.
    SubstitutionGroupNotSupported {
        /// Name of the element with unsupported substitution group.
        fragment_type: &'static str,
    },
}

/// Trait for types that can be converted to and from simple type fragments.
pub trait SimpleFragmentEquivalent: Sized {
    /// The fragment identifier type for this equivalent.
    type FragmentId;

    /// Converts this type to simple fragments in the compiler.
    fn to_simple_fragments<T: AsMut<SimpleTypeFragmentCompiler>>(
        &self,
        compiler: T,
    ) -> Result<Self::FragmentId, Error>;

    /// Creates this type from simple fragments in the compiler.
    fn from_simple_fragments<T: AsRef<SimpleTypeFragmentCompiler>>(
        _compiler: T,
        _fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error>;
}

impl SimpleFragmentEquivalent for xs::types::TopLevelSimpleType {
    type FragmentId = FragmentIdx<SimpleTypeRootFragment>;

    fn to_simple_fragments<T: AsMut<SimpleTypeFragmentCompiler>>(
        &self,
        mut compiler: T,
    ) -> Result<Self::FragmentId, Error> {
        let mut compiler = compiler.as_mut();

        let simple_derivation = self.simple_derivation.to_simple_fragments(&mut compiler)?;

        Ok(compiler.push_fragment(SimpleTypeRootFragment {
            name: Some(self.name.clone()),
            simple_derivation,
        }))
    }

    fn from_simple_fragments<T: AsRef<SimpleTypeFragmentCompiler>>(
        compiler: T,
        fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        let compiler = compiler.as_ref();

        let fragment = compiler
            .get_fragment(fragment_id)
            .expect("Fragment not found in compiler.");

        let name = fragment
            .name
            .clone()
            .expect("Name should be present for top-level simple type");

        let simple_derivation = xs::groups::SimpleDerivation::from_simple_fragments(
            compiler,
            &fragment.simple_derivation,
        )?;

        Ok(xs::types::TopLevelSimpleType::builder()
            .name(name)
            .simple_derivation(simple_derivation.into())
            .any_attributes(ns::AnyAttributes::default())
            .build())
    }
}

impl SimpleFragmentEquivalent for xs::types::LocalSimpleType {
    type FragmentId = FragmentIdx<SimpleTypeRootFragment>;

    fn to_simple_fragments<T: AsMut<SimpleTypeFragmentCompiler>>(
        &self,
        mut compiler: T,
    ) -> Result<Self::FragmentId, Error> {
        let mut compiler = compiler.as_mut();

        let simple_derivation = self.simple_derivation.to_simple_fragments(&mut compiler)?;

        Ok(compiler.push_fragment(SimpleTypeRootFragment {
            name: None,
            simple_derivation,
        }))
    }

    fn from_simple_fragments<T: AsRef<SimpleTypeFragmentCompiler>>(
        compiler: T,
        fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        let compiler = compiler.as_ref();

        let fragment = compiler
            .get_fragment(fragment_id)
            .expect("Fragment not found in compiler.");

        let simple_derivation = xs::groups::SimpleDerivation::from_simple_fragments(
            compiler,
            &fragment.simple_derivation,
        )?;

        Ok(xs::types::LocalSimpleType::builder()
            .simple_derivation(simple_derivation.into())
            .any_attributes(ns::AnyAttributes::default())
            .build())
    }
}

impl SimpleFragmentEquivalent for xs::types::SimpleRestrictionType {
    type FragmentId = FragmentIdx<RestrictionFragment>;

    fn to_simple_fragments<T: AsMut<SimpleTypeFragmentCompiler>>(
        &self,
        mut compiler: T,
    ) -> Result<Self::FragmentId, Error> {
        let mut compiler = compiler.as_mut();

        let base = self.base.0.clone();

        let simple_restriction_model = self
            .simple_restriction_model
            .as_ref()
            .map(|a| &a.simple_restriction_model);

        let simple_type = simple_restriction_model
            .and_then(|a| a.simple_type.as_ref())
            .map(|simple_type| simple_type.to_simple_fragments(&mut compiler))
            .transpose()?;

        let facets = simple_restriction_model
            .map(|a| {
                a.child_1
                    .iter()
                    .filter_map(|a| match a {
                        xs::groups::simple_restriction_model_items::Child1::Facet(facet) => {
                            Some(facet)
                        }
                        _ => None,
                    })
                    .map(|facet| facet.to_simple_fragments(&mut compiler))
                    .collect::<Result<Vec<_>, _>>()
            })
            .transpose()?
            .unwrap_or_default();

        Ok(compiler.push_fragment(RestrictionFragment {
            base: Some(base),
            facets,
            simple_type,
            id: self.id.clone(),
        }))
    }

    fn from_simple_fragments<T: AsRef<SimpleTypeFragmentCompiler>>(
        compiler: T,
        fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        let compiler = compiler.as_ref();

        let fragment = compiler
            .get_fragment(fragment_id)
            .expect("Fragment not found in compiler.");

        let base = xs::types::QName(
            fragment
                .base
                .clone()
                .unwrap_or_else(|| panic!("Base type must be present for SimpleRestrictionType")),
        );

        let simple_restriction_model =
            if fragment.simple_type.is_some() || !fragment.facets.is_empty() {
                let simple_type = fragment
                    .simple_type
                    .as_ref()
                    .map(|simple_type| {
                        xs::types::LocalSimpleType::from_simple_fragments(compiler, simple_type)
                    })
                    .transpose()?;

                let facets = fragment
                    .facets
                    .iter()
                    .map(|facet| xs::Facet::from_simple_fragments(compiler, facet))
                    .collect::<Result<Vec<_>, _>>()?;

                let child_1 = if facets.is_empty() {
                    None
                } else {
                    Some(
                        facets
                            .into_iter()
                            .map(xs::groups::simple_restriction_model_items::Child1::from)
                            .collect(),
                    )
                };

                Some(
                    xs::types::simple_restriction_type_items::SimpleRestrictionModel::builder()
                        .simple_restriction_model(Box::new(
                            xs::groups::SimpleRestrictionModel::builder()
                                .maybe_simple_type(simple_type.map(Box::new))
                                .maybe_child_1(child_1)
                                .build(),
                        ))
                        .build(),
                )
            } else {
                None
            };

        Ok(xs::types::SimpleRestrictionType::builder()
            .base(base)
            .maybe_id(fragment.id.clone())
            .any_attributes(ns::AnyAttributes::default())
            .maybe_simple_restriction_model(simple_restriction_model)
            .attr_decls(Box::new(xs::groups::AttrDecls::builder().build()))
            .assertions(Box::new(xs::groups::Assertions::builder().build()))
            .build())
    }
}

impl SimpleFragmentEquivalent for xs::Facet {
    type FragmentId = FragmentIdx<FacetFragment>;

    fn to_simple_fragments<T: AsMut<SimpleTypeFragmentCompiler>>(
        &self,
        compiler: T,
    ) -> Result<Self::FragmentId, Error> {
        use xs::Facet as F;
        match self {
            F::MinExclusive(f) => f.to_simple_fragments(compiler),
            F::MinInclusive(f) => f.to_simple_fragments(compiler),
            F::MaxExclusive(f) => f.to_simple_fragments(compiler),
            F::MaxInclusive(f) => f.to_simple_fragments(compiler),
            F::Enumeration(f) => f.to_simple_fragments(compiler),
            F::TotalDigits(f) => f.to_simple_fragments(compiler),
            F::FractionDigits(f) => f.to_simple_fragments(compiler),
            F::Length(f) => f.to_simple_fragments(compiler),
            F::MinLength(f) => f.to_simple_fragments(compiler),
            F::MaxLength(f) => f.to_simple_fragments(compiler),
            F::WhiteSpace(f) => f.to_simple_fragments(compiler),
            F::Pattern(f) => f.to_simple_fragments(compiler),
            F::Assertion(f) => f.to_simple_fragments(compiler),
            F::ExplicitTimezone(f) => f.to_simple_fragments(compiler),
            _ => Err(Error::SubstitutionGroupNotSupported {
                fragment_type: type_name::<Self>(),
            }),
        }
    }

    fn from_simple_fragments<T: AsRef<SimpleTypeFragmentCompiler>>(
        compiler: T,
        fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        let compiler = compiler.as_ref();

        let fragment = compiler
            .get_fragment(fragment_id)
            .expect("Fragment not found in compiler.");

        match fragment {
            FacetFragment::Length { .. } => {
                xs::Length::from_simple_fragments(compiler, fragment_id).map(From::from)
            }
            FacetFragment::MinLength { .. } => {
                xs::MinLength::from_simple_fragments(compiler, fragment_id).map(From::from)
            }
            FacetFragment::MaxLength { .. } => {
                xs::MaxLength::from_simple_fragments(compiler, fragment_id).map(From::from)
            }
            FacetFragment::MinExclusive { .. } => {
                xs::MinExclusive::from_simple_fragments(compiler, fragment_id).map(From::from)
            }
            FacetFragment::MinInclusive { .. } => {
                xs::MinInclusive::from_simple_fragments(compiler, fragment_id).map(From::from)
            }
            FacetFragment::MaxExclusive { .. } => {
                xs::MaxExclusive::from_simple_fragments(compiler, fragment_id).map(From::from)
            }
            FacetFragment::MaxInclusive { .. } => {
                xs::MaxInclusive::from_simple_fragments(compiler, fragment_id).map(From::from)
            }
            FacetFragment::Enumeration { .. } => {
                xs::Enumeration::from_simple_fragments(compiler, fragment_id).map(From::from)
            }
            FacetFragment::TotalDigits { .. } => {
                xs::TotalDigits::from_simple_fragments(compiler, fragment_id).map(From::from)
            }
            FacetFragment::FractionDigits { .. } => {
                xs::FractionDigits::from_simple_fragments(compiler, fragment_id).map(From::from)
            }
            FacetFragment::WhiteSpace { .. } => {
                xs::WhiteSpace::from_simple_fragments(compiler, fragment_id).map(From::from)
            }
            FacetFragment::Pattern { .. } => {
                xs::Pattern::from_simple_fragments(compiler, fragment_id).map(From::from)
            }
            FacetFragment::Assertion { .. } => {
                xs::Assertion::from_simple_fragments(compiler, fragment_id).map(From::from)
            }
            FacetFragment::ExplicitTimezone { .. } => {
                xs::ExplicitTimezone::from_simple_fragments(compiler, fragment_id).map(From::from)
            }
        }
    }
}

impl SimpleFragmentEquivalent for xs::MinExclusive {
    type FragmentId = FragmentIdx<FacetFragment>;

    fn to_simple_fragments<T: AsMut<SimpleTypeFragmentCompiler>>(
        &self,
        mut compiler: T,
    ) -> Result<Self::FragmentId, Error> {
        let compiler = compiler.as_mut();

        let facet = match self {
            xs::MinExclusive::MinExclusive(facet) => facet,
            _ => {
                return Err(Error::SubstitutionGroupNotSupported {
                    fragment_type: type_name::<Self>(),
                })
            }
        };

        Ok(compiler.push_fragment(FacetFragment::MinExclusive {
            value: Value(facet.value.clone()),
        }))
    }

    fn from_simple_fragments<T: AsRef<SimpleTypeFragmentCompiler>>(
        compiler: T,
        fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        let compiler = compiler.as_ref();

        let fragment = compiler
            .get_fragment(fragment_id)
            .expect("Fragment not found in compiler.");

        match fragment {
            FacetFragment::MinExclusive { value } => Ok(xs::MinExclusive::from(
                xs::types::Facet::builder().value(value.0.clone()).build(),
            )),
            _ => Err(Error::SubstitutionGroupNotSupported {
                fragment_type: type_name::<Self>(),
            }),
        }
    }
}

impl SimpleFragmentEquivalent for xs::MinInclusive {
    type FragmentId = FragmentIdx<FacetFragment>;

    fn to_simple_fragments<T: AsMut<SimpleTypeFragmentCompiler>>(
        &self,
        mut compiler: T,
    ) -> Result<Self::FragmentId, Error> {
        let compiler = compiler.as_mut();

        let facet = match self {
            xs::MinInclusive::MinInclusive(facet) => facet,
            _ => {
                return Err(Error::SubstitutionGroupNotSupported {
                    fragment_type: type_name::<Self>(),
                })
            }
        };

        Ok(compiler.push_fragment(FacetFragment::MinInclusive {
            value: Value(facet.value.clone()),
        }))
    }

    fn from_simple_fragments<T: AsRef<SimpleTypeFragmentCompiler>>(
        compiler: T,
        fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        let compiler = compiler.as_ref();

        let fragment = compiler
            .get_fragment(fragment_id)
            .expect("Fragment not found in compiler.");

        match fragment {
            FacetFragment::MinInclusive { value } => Ok(xs::MinInclusive::from(
                xs::types::Facet::builder().value(value.0.clone()).build(),
            )),
            _ => Err(Error::SubstitutionGroupNotSupported {
                fragment_type: type_name::<Self>(),
            }),
        }
    }
}

impl SimpleFragmentEquivalent for xs::MaxExclusive {
    type FragmentId = FragmentIdx<FacetFragment>;

    fn to_simple_fragments<T: AsMut<SimpleTypeFragmentCompiler>>(
        &self,
        mut compiler: T,
    ) -> Result<Self::FragmentId, Error> {
        let compiler = compiler.as_mut();

        let facet = match self {
            xs::MaxExclusive::MaxExclusive(facet) => facet,
            _ => {
                return Err(Error::SubstitutionGroupNotSupported {
                    fragment_type: type_name::<Self>(),
                })
            }
        };

        Ok(compiler.push_fragment(FacetFragment::MaxExclusive {
            value: Value(facet.value.clone()),
        }))
    }

    fn from_simple_fragments<T: AsRef<SimpleTypeFragmentCompiler>>(
        compiler: T,
        fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        let compiler = compiler.as_ref();

        let fragment = compiler
            .get_fragment(fragment_id)
            .expect("Fragment not found in compiler.");

        match fragment {
            FacetFragment::MaxExclusive { value } => Ok(xs::MaxExclusive::from(
                xs::types::Facet::builder().value(value.0.clone()).build(),
            )),
            _ => Err(Error::SubstitutionGroupNotSupported {
                fragment_type: type_name::<Self>(),
            }),
        }
    }
}

impl SimpleFragmentEquivalent for xs::MaxInclusive {
    type FragmentId = FragmentIdx<FacetFragment>;

    fn to_simple_fragments<T: AsMut<SimpleTypeFragmentCompiler>>(
        &self,
        mut compiler: T,
    ) -> Result<Self::FragmentId, Error> {
        let compiler = compiler.as_mut();

        let facet = match self {
            xs::MaxInclusive::MaxInclusive(facet) => facet,
            _ => {
                return Err(Error::SubstitutionGroupNotSupported {
                    fragment_type: type_name::<Self>(),
                })
            }
        };

        Ok(compiler.push_fragment(FacetFragment::MaxInclusive {
            value: Value(facet.value.clone()),
        }))
    }

    fn from_simple_fragments<T: AsRef<SimpleTypeFragmentCompiler>>(
        compiler: T,
        fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        let compiler = compiler.as_ref();

        let fragment = compiler
            .get_fragment(fragment_id)
            .expect("Fragment not found in compiler.");

        match fragment {
            FacetFragment::MaxInclusive { value } => Ok(xs::MaxInclusive::from(
                xs::types::Facet::builder().value(value.0.clone()).build(),
            )),
            _ => Err(Error::SubstitutionGroupNotSupported {
                fragment_type: type_name::<Self>(),
            }),
        }
    }
}

impl SimpleFragmentEquivalent for xs::Enumeration {
    type FragmentId = FragmentIdx<FacetFragment>;

    fn to_simple_fragments<T: AsMut<SimpleTypeFragmentCompiler>>(
        &self,
        mut compiler: T,
    ) -> Result<Self::FragmentId, Error> {
        let compiler = compiler.as_mut();

        let facet = match self {
            xs::Enumeration::Enumeration(facet) => facet,
            _ => {
                return Err(Error::SubstitutionGroupNotSupported {
                    fragment_type: type_name::<Self>(),
                })
            }
        };

        Ok(compiler.push_fragment(FacetFragment::Enumeration {
            value: Value(facet.value.clone()),
        }))
    }

    fn from_simple_fragments<T: AsRef<SimpleTypeFragmentCompiler>>(
        compiler: T,
        fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        let compiler = compiler.as_ref();

        let fragment = compiler
            .get_fragment(fragment_id)
            .expect("Fragment not found in compiler.");

        match fragment {
            FacetFragment::Enumeration { value } => Ok(xs::Enumeration::from(
                xs::types::NoFixedFacet::builder()
                    .value(value.0.clone())
                    .any_attributes(ns::AnyAttributes::default())
                    .build(),
            )),
            _ => Err(Error::SubstitutionGroupNotSupported {
                fragment_type: type_name::<Self>(),
            }),
        }
    }
}

impl SimpleFragmentEquivalent for xs::TotalDigits {
    type FragmentId = FragmentIdx<FacetFragment>;

    fn to_simple_fragments<T: AsMut<SimpleTypeFragmentCompiler>>(
        &self,
        mut compiler: T,
    ) -> Result<Self::FragmentId, Error> {
        let compiler = compiler.as_mut();

        let facet = match self {
            xs::TotalDigits::TotalDigits(facet) => facet,
            _ => {
                return Err(Error::SubstitutionGroupNotSupported {
                    fragment_type: type_name::<Self>(),
                })
            }
        };

        Ok(compiler.push_fragment(FacetFragment::TotalDigits { value: facet.value }))
    }

    fn from_simple_fragments<T: AsRef<SimpleTypeFragmentCompiler>>(
        compiler: T,
        fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        let compiler = compiler.as_ref();

        let fragment = compiler
            .get_fragment(fragment_id)
            .expect("Fragment not found in compiler.");

        match fragment {
            FacetFragment::TotalDigits { value } => Ok(xs::TotalDigits::from(
                xs::total_digits_items::TotalDigits::builder()
                    .value(*value)
                    .any_attributes(ns::AnyAttributes::default())
                    .build(),
            )),
            _ => Err(Error::SubstitutionGroupNotSupported {
                fragment_type: type_name::<Self>(),
            }),
        }
    }
}

impl SimpleFragmentEquivalent for xs::FractionDigits {
    type FragmentId = FragmentIdx<FacetFragment>;

    fn to_simple_fragments<T: AsMut<SimpleTypeFragmentCompiler>>(
        &self,
        mut compiler: T,
    ) -> Result<Self::FragmentId, Error> {
        let compiler = compiler.as_mut();

        let facet = match self {
            xs::FractionDigits::FractionDigits(facet) => facet,
            _ => {
                return Err(Error::SubstitutionGroupNotSupported {
                    fragment_type: type_name::<Self>(),
                })
            }
        };

        Ok(compiler.push_fragment(FacetFragment::FractionDigits { value: facet.value }))
    }

    fn from_simple_fragments<T: AsRef<SimpleTypeFragmentCompiler>>(
        compiler: T,
        fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        let compiler = compiler.as_ref();

        let fragment = compiler
            .get_fragment(fragment_id)
            .expect("Fragment not found in compiler.");

        match fragment {
            FacetFragment::FractionDigits { value } => Ok(xs::FractionDigits::from(
                xs::types::NumFacet::builder()
                    .value(*value)
                    .any_attributes(ns::AnyAttributes::default())
                    .build(),
            )),
            _ => Err(Error::SubstitutionGroupNotSupported {
                fragment_type: type_name::<Self>(),
            }),
        }
    }
}

impl SimpleFragmentEquivalent for xs::Length {
    type FragmentId = FragmentIdx<FacetFragment>;

    fn to_simple_fragments<T: AsMut<SimpleTypeFragmentCompiler>>(
        &self,
        mut compiler: T,
    ) -> Result<Self::FragmentId, Error> {
        let compiler = compiler.as_mut();

        let facet = match self {
            xs::Length::Length(facet) => facet,
            _ => {
                return Err(Error::SubstitutionGroupNotSupported {
                    fragment_type: type_name::<Self>(),
                })
            }
        };

        Ok(compiler.push_fragment(FacetFragment::Length { value: facet.value }))
    }

    fn from_simple_fragments<T: AsRef<SimpleTypeFragmentCompiler>>(
        compiler: T,
        fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        let compiler = compiler.as_ref();

        let fragment = compiler
            .get_fragment(fragment_id)
            .expect("Fragment not found in compiler.");

        match fragment {
            FacetFragment::Length { value } => Ok(xs::Length::from(
                xs::types::NumFacet::builder()
                    .value(*value)
                    .any_attributes(ns::AnyAttributes::default())
                    .build(),
            )),
            _ => Err(Error::SubstitutionGroupNotSupported {
                fragment_type: type_name::<Self>(),
            }),
        }
    }
}

impl SimpleFragmentEquivalent for xs::MinLength {
    type FragmentId = FragmentIdx<FacetFragment>;

    fn to_simple_fragments<T: AsMut<SimpleTypeFragmentCompiler>>(
        &self,
        mut compiler: T,
    ) -> Result<Self::FragmentId, Error> {
        let compiler = compiler.as_mut();

        let facet = match self {
            xs::MinLength::MinLength(facet) => facet,
            _ => {
                return Err(Error::SubstitutionGroupNotSupported {
                    fragment_type: type_name::<Self>(),
                })
            }
        };

        Ok(compiler.push_fragment(FacetFragment::MinLength { value: facet.value }))
    }

    fn from_simple_fragments<T: AsRef<SimpleTypeFragmentCompiler>>(
        compiler: T,
        fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        let compiler = compiler.as_ref();

        let fragment = compiler
            .get_fragment(fragment_id)
            .expect("Fragment not found in compiler.");

        match fragment {
            FacetFragment::MinLength { value } => Ok(xs::MinLength::from(
                xs::types::NumFacet::builder()
                    .value(*value)
                    .any_attributes(ns::AnyAttributes::default())
                    .build(),
            )),
            _ => Err(Error::SubstitutionGroupNotSupported {
                fragment_type: type_name::<Self>(),
            }),
        }
    }
}

impl SimpleFragmentEquivalent for xs::MaxLength {
    type FragmentId = FragmentIdx<FacetFragment>;

    fn to_simple_fragments<T: AsMut<SimpleTypeFragmentCompiler>>(
        &self,
        mut compiler: T,
    ) -> Result<Self::FragmentId, Error> {
        let compiler = compiler.as_mut();

        let facet = match self {
            xs::MaxLength::MaxLength(facet) => facet,
            _ => {
                return Err(Error::SubstitutionGroupNotSupported {
                    fragment_type: type_name::<Self>(),
                })
            }
        };

        Ok(compiler.push_fragment(FacetFragment::MaxLength { value: facet.value }))
    }

    fn from_simple_fragments<T: AsRef<SimpleTypeFragmentCompiler>>(
        compiler: T,
        fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        let compiler = compiler.as_ref();

        let fragment = compiler
            .get_fragment(fragment_id)
            .expect("Fragment not found in compiler.");

        match fragment {
            FacetFragment::MaxLength { value } => Ok(xs::MaxLength::from(
                xs::types::NumFacet::builder()
                    .value(*value)
                    .any_attributes(ns::AnyAttributes::default())
                    .build(),
            )),
            _ => Err(Error::SubstitutionGroupNotSupported {
                fragment_type: type_name::<Self>(),
            }),
        }
    }
}

impl SimpleFragmentEquivalent for xs::WhiteSpace {
    type FragmentId = FragmentIdx<FacetFragment>;

    fn to_simple_fragments<T: AsMut<SimpleTypeFragmentCompiler>>(
        &self,
        mut compiler: T,
    ) -> Result<Self::FragmentId, Error> {
        let compiler = compiler.as_mut();

        let facet = match self {
            xs::WhiteSpace::WhiteSpace(facet) => facet,
            _ => {
                return Err(Error::SubstitutionGroupNotSupported {
                    fragment_type: type_name::<Self>(),
                })
            }
        };

        Ok(compiler.push_fragment(FacetFragment::WhiteSpace {
            value: facet.value.into(),
        }))
    }

    fn from_simple_fragments<T: AsRef<SimpleTypeFragmentCompiler>>(
        compiler: T,
        fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        let compiler = compiler.as_ref();

        let fragment = compiler
            .get_fragment(fragment_id)
            .expect("Fragment not found in compiler.");

        match fragment {
            FacetFragment::WhiteSpace { value } => Ok(xs::WhiteSpace::from(
                xs::white_space_items::WhiteSpace::builder()
                    .value(xs::white_space_items::ValueValue::from(*value))
                    .any_attributes(ns::AnyAttributes::default())
                    .build(),
            )),
            _ => Err(Error::SubstitutionGroupNotSupported {
                fragment_type: type_name::<Self>(),
            }),
        }
    }
}

impl SimpleFragmentEquivalent for xs::Pattern {
    type FragmentId = FragmentIdx<FacetFragment>;

    fn to_simple_fragments<T: AsMut<SimpleTypeFragmentCompiler>>(
        &self,
        mut compiler: T,
    ) -> Result<Self::FragmentId, Error> {
        let compiler = compiler.as_mut();

        let facet = match self {
            xs::Pattern::Pattern(facet) => facet,
            _ => {
                return Err(Error::SubstitutionGroupNotSupported {
                    fragment_type: type_name::<Self>(),
                })
            }
        };

        Ok(compiler.push_fragment(FacetFragment::Pattern {
            value: Pattern(facet.value.clone()),
        }))
    }

    fn from_simple_fragments<T: AsRef<SimpleTypeFragmentCompiler>>(
        compiler: T,
        fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        let compiler = compiler.as_ref();

        let fragment = compiler
            .get_fragment(fragment_id)
            .expect("Fragment not found in compiler.");

        match fragment {
            FacetFragment::Pattern { value } => Ok(xs::Pattern::from(
                xs::pattern_items::Pattern::builder()
                    .value(value.0.clone())
                    .any_attributes(ns::AnyAttributes::default())
                    .build(),
            )),
            _ => Err(Error::SubstitutionGroupNotSupported {
                fragment_type: type_name::<Self>(),
            }),
        }
    }
}

impl SimpleFragmentEquivalent for xs::Assertion {
    type FragmentId = FragmentIdx<FacetFragment>;

    fn to_simple_fragments<T: AsMut<SimpleTypeFragmentCompiler>>(
        &self,
        mut compiler: T,
    ) -> Result<Self::FragmentId, Error> {
        let compiler = compiler.as_mut();

        let facet = match self {
            xs::Assertion::Assertion(facet) => facet,
            _ => {
                return Err(Error::SubstitutionGroupNotSupported {
                    fragment_type: type_name::<Self>(),
                })
            }
        };

        Ok(compiler.push_fragment(FacetFragment::Assertion {
            test: facet.test.clone().map(Assertion),
        }))
    }

    fn from_simple_fragments<T: AsRef<SimpleTypeFragmentCompiler>>(
        compiler: T,
        fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        let compiler = compiler.as_ref();

        let fragment = compiler
            .get_fragment(fragment_id)
            .expect("Fragment not found in compiler.");

        match fragment {
            FacetFragment::Assertion { test } => Ok(xs::Assertion::from(
                xs::types::Assertion::builder()
                    .maybe_test(test.as_ref().map(|a| a.0.clone()))
                    .build(),
            )),
            _ => Err(Error::SubstitutionGroupNotSupported {
                fragment_type: type_name::<Self>(),
            }),
        }
    }
}

impl SimpleFragmentEquivalent for xs::ExplicitTimezone {
    type FragmentId = FragmentIdx<FacetFragment>;

    fn to_simple_fragments<T: AsMut<SimpleTypeFragmentCompiler>>(
        &self,
        mut compiler: T,
    ) -> Result<Self::FragmentId, Error> {
        let compiler = compiler.as_mut();

        let facet = match self {
            xs::ExplicitTimezone::ExplicitTimezone(facet) => facet,
            _ => {
                return Err(Error::SubstitutionGroupNotSupported {
                    fragment_type: type_name::<Self>(),
                })
            }
        };

        Ok(compiler.push_fragment(FacetFragment::ExplicitTimezone {
            value: facet.value.into(),
        }))
    }

    fn from_simple_fragments<T: AsRef<SimpleTypeFragmentCompiler>>(
        compiler: T,
        fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        let compiler = compiler.as_ref();

        let fragment = compiler
            .get_fragment(fragment_id)
            .expect("Fragment not found in compiler.");

        match fragment {
            FacetFragment::ExplicitTimezone { value } => Ok(xs::ExplicitTimezone::from(
                xs::explicit_timezone_items::ExplicitTimezone::builder()
                    .value(xs::explicit_timezone_items::ValueValue::from(*value))
                    .any_attributes(ns::AnyAttributes::default())
                    .build(),
            )),
            _ => Err(Error::SubstitutionGroupNotSupported {
                fragment_type: type_name::<Self>(),
            }),
        }
    }
}

impl SimpleFragmentEquivalent for xs::List {
    type FragmentId = FragmentIdx<ListFragment>;

    fn to_simple_fragments<T: AsMut<SimpleTypeFragmentCompiler>>(
        &self,
        mut compiler: T,
    ) -> Result<Self::FragmentId, Error> {
        let mut compiler = compiler.as_mut();

        let list = match self {
            xs::List::List(list) => list,
            _ => {
                return Err(Error::SubstitutionGroupNotSupported {
                    fragment_type: type_name::<Self>(),
                })
            }
        };

        let item_type = if let Some(item_type) = list.item_type.as_ref() {
            NamedOrAnonymous::Named(item_type.0.clone())
        } else if let Some(simple_type) = list.simple_type.as_ref() {
            NamedOrAnonymous::Anonymous(simple_type.to_simple_fragments(&mut compiler)?)
        } else {
            //ERROR
            return Err(Error::ListMissingType);
        };

        Ok(compiler.push_fragment(ListFragment {
            item_type,
            id: list.id.clone(),
        }))
    }

    fn from_simple_fragments<T: AsRef<SimpleTypeFragmentCompiler>>(
        compiler: T,
        fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        let compiler = compiler.as_ref();

        let fragment = compiler
            .get_fragment(fragment_id)
            .expect("Fragment not found in compiler.");

        let (item_type, simple_type) = match &fragment.item_type {
            NamedOrAnonymous::Named(name) => (Some(xs::types::QName(name.clone())), None),
            NamedOrAnonymous::Anonymous(fragment_id) => (
                None,
                Some(Box::new(xs::types::LocalSimpleType::from_simple_fragments(
                    compiler,
                    fragment_id,
                )?)),
            ),
        };

        Ok(xs::list_items::List::builder()
            .maybe_item_type(item_type)
            .maybe_simple_type(simple_type)
            .maybe_id(fragment.id.clone())
            .build()
            .into())
    }
}

impl SimpleFragmentEquivalent for xs::union_items::SimpleType {
    type FragmentId = FragmentIdx<SimpleTypeRootFragment>;

    fn to_simple_fragments<T: AsMut<SimpleTypeFragmentCompiler>>(
        &self,
        compiler: T,
    ) -> Result<Self::FragmentId, Error> {
        self.0.to_simple_fragments(compiler)
    }

    fn from_simple_fragments<T: AsRef<SimpleTypeFragmentCompiler>>(
        compiler: T,
        fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        let compiler = compiler.as_ref();

        xs::types::LocalSimpleType::from_simple_fragments(compiler, fragment_id).map(Into::into)
    }
}

impl SimpleFragmentEquivalent for xs::Union {
    type FragmentId = FragmentIdx<UnionFragment>;

    fn to_simple_fragments<T: AsMut<SimpleTypeFragmentCompiler>>(
        &self,
        mut compiler: T,
    ) -> Result<Self::FragmentId, Error> {
        let mut compiler = compiler.as_mut();

        let union = match self {
            xs::Union::Union(union) => union,
            _ => {
                return Err(Error::SubstitutionGroupNotSupported {
                    fragment_type: type_name::<Self>(),
                })
            }
        };

        let member_types = union
            .member_types
            .as_ref()
            .map(|member_type| {
                member_type
                    .0
                    .iter()
                    .map(|a| {
                        a.0.clone()
                            .with_default_namespace(|| compiler.namespace.clone())
                    })
                    .collect()
            })
            .unwrap_or_default();

        let simple_types = union
            .simple_type
            .iter()
            .map(|simple_type| simple_type.to_simple_fragments(&mut compiler))
            .collect::<Result<VecDeque<_>, _>>()?;

        Ok(compiler.push_fragment(UnionFragment {
            member_types,
            simple_types,
            id: union.id.clone(),
        }))
    }

    fn from_simple_fragments<T: AsRef<SimpleTypeFragmentCompiler>>(
        compiler: T,
        fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        let compiler = compiler.as_ref();

        let fragment = compiler
            .get_fragment(fragment_id)
            .expect("Fragment not found in compiler.");

        let member_types = fragment
            .member_types
            .iter()
            .map(|name| xs::types::QName(name.clone()))
            .collect::<xsd::ns::List<_>>();

        let member_types = if member_types.is_empty() {
            None
        } else {
            Some(member_types)
        };

        let simple_type = fragment
            .simple_types
            .iter()
            .map(|simple_type| {
                xs::union_items::SimpleType::from_simple_fragments(compiler, simple_type)
            })
            .collect::<Result<Vec<_>, _>>()?;

        let simple_type = if simple_type.is_empty() {
            None
        } else {
            Some(simple_type)
        };

        Ok(xs::union_items::Union::builder()
            .maybe_member_types(member_types)
            .maybe_simple_type(simple_type)
            .maybe_id(fragment.id.clone())
            .build()
            .into())
    }
}

impl SimpleFragmentEquivalent for xs::Restriction {
    type FragmentId = FragmentIdx<RestrictionFragment>;

    fn to_simple_fragments<T: AsMut<SimpleTypeFragmentCompiler>>(
        &self,
        mut compiler: T,
    ) -> Result<Self::FragmentId, Error> {
        let mut compiler = compiler.as_mut();

        let restriction = match self {
            xs::Restriction::Restriction(restriction) => restriction,
            _ => {
                return Err(Error::SubstitutionGroupNotSupported {
                    fragment_type: type_name::<Self>(),
                })
            }
        };

        let base = restriction.base.as_ref().map(|a| a.0.clone());

        let simple_type = restriction
            .simple_restriction_model
            .simple_type
            .as_ref()
            .map(|simple_type| simple_type.to_simple_fragments(&mut compiler))
            .transpose()?;

        let facets = restriction
            .simple_restriction_model
            .child_1
            .iter()
            .filter_map(|a| match a {
                xs::groups::simple_restriction_model_items::Child1::Facet(facet) => Some(facet),
                _ => None,
            })
            .map(|facet| facet.to_simple_fragments(&mut compiler))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(compiler.push_fragment(RestrictionFragment {
            base,
            simple_type,
            facets,
            id: restriction.id.clone(),
        }))
    }

    fn from_simple_fragments<T: AsRef<SimpleTypeFragmentCompiler>>(
        compiler: T,
        fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        let compiler = compiler.as_ref();

        let fragment = compiler
            .get_fragment(fragment_id)
            .expect("Fragment not found in compiler.");

        let base = fragment.base.clone().map(xs::types::QName);

        let simple_type = fragment
            .simple_type
            .as_ref()
            .map(|simple_type| {
                xs::types::LocalSimpleType::from_simple_fragments(compiler, simple_type)
            })
            .transpose()?;

        let facets = fragment
            .facets
            .iter()
            .map(|facet| xs::Facet::from_simple_fragments(compiler, facet))
            .collect::<Result<Vec<_>, _>>()?;

        let child_1 = if facets.is_empty() {
            None
        } else {
            Some(
                facets
                    .into_iter()
                    .map(xs::groups::simple_restriction_model_items::Child1::from)
                    .collect(),
            )
        };

        Ok(xs::restriction_items::Restriction::builder()
            .maybe_base(base)
            .maybe_id(fragment.id.clone())
            .simple_restriction_model(
                xs::groups::SimpleRestrictionModel::builder()
                    .maybe_simple_type(simple_type.map(Box::new))
                    .maybe_child_1(child_1)
                    .build(),
            )
            .build()
            .into())
    }
}

/// Simple type derivation methods.
///
/// Represents the three ways a simple type can be derived in XML Schema.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SimpleDerivation {
    /// Derivation by restriction, constraining an existing type.
    Restriction(FragmentIdx<RestrictionFragment>),
    /// Derivation by list, creating a space-separated list of base type values.
    List(FragmentIdx<ListFragment>),
    /// Derivation by union, allowing values from multiple member types.
    Union(FragmentIdx<UnionFragment>),
}

impl From<FragmentIdx<RestrictionFragment>> for SimpleDerivation {
    fn from(fragment_id: FragmentIdx<RestrictionFragment>) -> Self {
        SimpleDerivation::Restriction(fragment_id)
    }
}

impl From<FragmentIdx<ListFragment>> for SimpleDerivation {
    fn from(fragment_id: FragmentIdx<ListFragment>) -> Self {
        SimpleDerivation::List(fragment_id)
    }
}

impl From<FragmentIdx<UnionFragment>> for SimpleDerivation {
    fn from(fragment_id: FragmentIdx<UnionFragment>) -> Self {
        SimpleDerivation::Union(fragment_id)
    }
}

impl SimpleFragmentEquivalent for xs::groups::SimpleDerivation {
    type FragmentId = SimpleDerivation;

    fn to_simple_fragments<T: AsMut<SimpleTypeFragmentCompiler>>(
        &self,
        compiler: T,
    ) -> Result<Self::FragmentId, Error> {
        use xs::groups::SimpleDerivation as S;
        match self {
            S::Restriction(restriction) => {
                restriction.to_simple_fragments(compiler).map(Into::into)
            }
            S::List(list) => list.to_simple_fragments(compiler).map(Into::into),
            S::Union(union) => union.to_simple_fragments(compiler).map(Into::into),
        }
    }

    fn from_simple_fragments<T: AsRef<SimpleTypeFragmentCompiler>>(
        compiler: T,
        fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        match fragment_id {
            SimpleDerivation::Restriction(fragment_id) => {
                xs::Restriction::from_simple_fragments(compiler, fragment_id)
                    .map(xs::groups::SimpleDerivation::from)
            }
            SimpleDerivation::List(fragment_id) => {
                xs::List::from_simple_fragments(compiler, fragment_id)
                    .map(xs::groups::SimpleDerivation::from)
            }
            SimpleDerivation::Union(fragment_id) => {
                xs::Union::from_simple_fragments(compiler, fragment_id)
                    .map(xs::groups::SimpleDerivation::from)
            }
        }
    }
}
