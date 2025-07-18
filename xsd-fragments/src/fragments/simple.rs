use std::num::NonZeroUsize;
use xsd::xs;

use xmlity::{ExpandedName, LocalName, XmlNamespace};

use crate::{
    fragments::{
        FragmentAccess, FragmentCollection, FragmentIdx, HasFragmentCollection, NamespaceIdx,
    },
    NamedOrAnonymous,
};
use std::collections::VecDeque;

#[derive(Debug, Clone, PartialEq)]
pub struct ExtensionFragment {
    pub base: ExpandedName<'static>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RestrictionFragment {
    pub base: Option<ExpandedName<'static>>,
    pub facets: Vec<FragmentIdx<FacetFragment>>,
    pub simple_type: Option<FragmentIdx<SimpleTypeRootFragment>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SimpleTypeRootFragment {
    pub name: Option<LocalName<'static>>,
    pub simple_derivation: SimpleDerivation,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ListFragment {
    pub item_type: NamedOrAnonymous<FragmentIdx<SimpleTypeRootFragment>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UnionFragment {
    pub member_types: VecDeque<ExpandedName<'static>>,
    pub simple_types: VecDeque<FragmentIdx<SimpleTypeRootFragment>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GroupRefFragment {
    pub ref_: ExpandedName<'static>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Value(pub String);

#[derive(Debug, Clone, PartialEq)]
pub struct Pattern(pub String);

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WhiteSpaceValue {
    Preserve,
    Replace,
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

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ExplicitTimezoneValue {
    Required,
    Prohibited,
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

#[derive(Debug, Clone, PartialEq)]
pub struct Assertion(pub String);

#[derive(Debug, Clone, PartialEq)]
pub enum FacetFragment {
    Length { value: usize },
    MinLength { value: usize },
    MaxLength { value: usize },
    MinExclusive { value: Value },
    MinInclusive { value: Value },
    MaxExclusive { value: Value },
    MaxInclusive { value: Value },
    Enumeration { value: Value },
    TotalDigits { value: NonZeroUsize },
    FractionDigits { value: usize },
    WhiteSpace { value: WhiteSpaceValue },
    Pattern { value: Pattern },
    Assertion { test: Option<Assertion> },
    ExplicitTimezone { value: ExplicitTimezoneValue },
}

#[derive(Debug, Clone)]
pub struct SimpleTypeFragmentCompiler {
    pub namespace: XmlNamespace<'static>,
    pub simple_types: FragmentCollection<SimpleTypeRootFragment>,
    pub restrictions: FragmentCollection<RestrictionFragment>,
    pub extensions: FragmentCollection<ExtensionFragment>,
    pub facets: FragmentCollection<FacetFragment>,
    pub lists: FragmentCollection<ListFragment>,
    pub unions: FragmentCollection<UnionFragment>,
    pub group_refs: FragmentCollection<GroupRefFragment>,
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
    pub fn new(namespace: XmlNamespace<'static>, namespace_idx: NamespaceIdx) -> Self {
        Self {
            namespace,
            simple_types: FragmentCollection::new(namespace_idx),
            restrictions: FragmentCollection::new(namespace_idx),
            extensions: FragmentCollection::new(namespace_idx),
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

impl HasFragmentCollection<ExtensionFragment> for SimpleTypeFragmentCompiler {
    fn get_fragment_collection(&self) -> &FragmentCollection<ExtensionFragment> {
        &self.extensions
    }
    fn get_fragment_collection_mut(&mut self) -> &mut FragmentCollection<ExtensionFragment> {
        &mut self.extensions
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

#[derive(Debug, Clone)]
pub struct Error;

pub trait SimpleFragmentEquivalent: Sized {
    type FragmentId;

    fn to_simple_fragments<T: AsMut<SimpleTypeFragmentCompiler>>(
        &self,
        compiler: T,
    ) -> Self::FragmentId;

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
    ) -> Self::FragmentId {
        let mut compiler = compiler.as_mut();

        let simple_derivation = self.simple_derivation.to_simple_fragments(&mut compiler);

        compiler.push_fragment(SimpleTypeRootFragment {
            name: Some(self.name.clone()),
            simple_derivation,
        })
    }

    fn from_simple_fragments<T: AsRef<SimpleTypeFragmentCompiler>>(
        compiler: T,
        fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        let compiler = compiler.as_ref();

        let fragment = compiler.get_fragment(fragment_id).ok_or(Error)?;

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
            .build())
    }
}

impl SimpleFragmentEquivalent for xs::types::LocalSimpleType {
    type FragmentId = FragmentIdx<SimpleTypeRootFragment>;

    fn to_simple_fragments<T: AsMut<SimpleTypeFragmentCompiler>>(
        &self,
        mut compiler: T,
    ) -> Self::FragmentId {
        let mut compiler = compiler.as_mut();

        let simple_derivation = self.simple_derivation.to_simple_fragments(&mut compiler);

        compiler.push_fragment(SimpleTypeRootFragment {
            name: None,
            simple_derivation,
        })
    }

    fn from_simple_fragments<T: AsRef<SimpleTypeFragmentCompiler>>(
        compiler: T,
        fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        let compiler = compiler.as_ref();

        let fragment = compiler.get_fragment(fragment_id).ok_or(Error)?;

        let simple_derivation = xs::groups::SimpleDerivation::from_simple_fragments(
            compiler,
            &fragment.simple_derivation,
        )?;

        Ok(xs::types::LocalSimpleType::builder()
            .simple_derivation(simple_derivation.into())
            .build())
    }
}

impl SimpleFragmentEquivalent for xs::types::SimpleRestrictionType {
    type FragmentId = FragmentIdx<RestrictionFragment>;

    fn to_simple_fragments<T: AsMut<SimpleTypeFragmentCompiler>>(
        &self,
        mut compiler: T,
    ) -> Self::FragmentId {
        let mut compiler = compiler.as_mut();

        let base = self.base.0.clone();

        let simple_restriction_model = self
            .simple_restriction_model
            .as_ref()
            .map(|a| &a.simple_restriction_model);

        let simple_type = simple_restriction_model
            .and_then(|a| a.simple_type.as_ref())
            .map(|simple_type| simple_type.to_simple_fragments(&mut compiler));

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
                    .collect()
            })
            .unwrap_or_default();

        compiler.push_fragment(RestrictionFragment {
            base: Some(base),
            facets,
            simple_type,
        })
    }

    fn from_simple_fragments<T: AsRef<SimpleTypeFragmentCompiler>>(
        _compiler: T,
        _fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        unimplemented!()
    }
}

impl SimpleFragmentEquivalent for xs::Facet {
    type FragmentId = FragmentIdx<FacetFragment>;

    fn to_simple_fragments<T: AsMut<SimpleTypeFragmentCompiler>>(
        &self,
        compiler: T,
    ) -> Self::FragmentId {
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
        }
    }

    fn from_simple_fragments<T: AsRef<SimpleTypeFragmentCompiler>>(
        compiler: T,
        fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        let compiler = compiler.as_ref();

        let fragment = compiler.get_fragment(fragment_id).unwrap();

        let facet: xs::Facet = match fragment {
            FacetFragment::Length { value } => {
                xs::Length::from(xs::types::NumFacet::builder().value(*value).build()).into()
            }
            FacetFragment::MinLength { value } => {
                xs::MinLength::from(xs::types::NumFacet::builder().value(*value).build()).into()
            }
            FacetFragment::MaxLength { value } => {
                xs::MaxLength::from(xs::types::NumFacet::builder().value(*value).build()).into()
            }
            FacetFragment::MinExclusive { value } => {
                xs::MinExclusive::from(xs::types::Facet::builder().value(value.0.clone()).build())
                    .into()
            }
            FacetFragment::MinInclusive { value } => {
                xs::MinInclusive::from(xs::types::Facet::builder().value(value.0.clone()).build())
                    .into()
            }
            FacetFragment::MaxExclusive { value } => {
                xs::MaxExclusive::from(xs::types::Facet::builder().value(value.0.clone()).build())
                    .into()
            }
            FacetFragment::MaxInclusive { value } => {
                xs::MaxInclusive::from(xs::types::Facet::builder().value(value.0.clone()).build())
                    .into()
            }
            FacetFragment::Enumeration { value } => xs::Enumeration::from(
                xs::types::NoFixedFacet::builder()
                    .value(value.0.clone())
                    .build(),
            )
            .into(),
            FacetFragment::TotalDigits { value } => xs::TotalDigits::from(
                xs::total_digits_items::TotalDigits::builder()
                    .value(*value)
                    .build(),
            )
            .into(),
            FacetFragment::FractionDigits { value } => {
                xs::FractionDigits::from(xs::types::NumFacet::builder().value(*value).build())
                    .into()
            }
            FacetFragment::WhiteSpace { value } => xs::WhiteSpace::from(
                xs::white_space_items::WhiteSpace::builder()
                    .value(xs::white_space_items::ValueValue::from(*value))
                    .build(),
            )
            .into(),
            FacetFragment::Pattern { value } => xs::Pattern::from(
                xs::pattern_items::Pattern::builder()
                    .value(value.0.clone())
                    .build(),
            )
            .into(),
            FacetFragment::Assertion { test } => xs::Assertion::from(
                xs::types::Assertion::builder()
                    .maybe_test(test.as_ref().map(|a| a.0.clone()))
                    .build(),
            )
            .into(),
            FacetFragment::ExplicitTimezone { value } => xs::ExplicitTimezone::from(
                xs::explicit_timezone_items::ExplicitTimezone::builder()
                    .value(xs::explicit_timezone_items::ValueValue::from(*value))
                    .build(),
            )
            .into(),
        };

        Ok(facet)
    }
}

impl SimpleFragmentEquivalent for xs::MinExclusive {
    type FragmentId = FragmentIdx<FacetFragment>;

    fn to_simple_fragments<T: AsMut<SimpleTypeFragmentCompiler>>(
        &self,
        mut compiler: T,
    ) -> Self::FragmentId {
        let compiler = compiler.as_mut();

        let facet = match self {
            xs::MinExclusive::MinExclusive(facet) => facet,
            _ => panic!("Expected a minExclusive facet"),
        };

        compiler.push_fragment(FacetFragment::MinExclusive {
            value: Value(facet.value.clone()),
        })
    }

    fn from_simple_fragments<T: AsRef<SimpleTypeFragmentCompiler>>(
        _compiler: T,
        _fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        unimplemented!()
    }
}

impl SimpleFragmentEquivalent for xs::MinInclusive {
    type FragmentId = FragmentIdx<FacetFragment>;

    fn to_simple_fragments<T: AsMut<SimpleTypeFragmentCompiler>>(
        &self,
        mut compiler: T,
    ) -> Self::FragmentId {
        let compiler = compiler.as_mut();

        let facet = match self {
            xs::MinInclusive::MinInclusive(facet) => facet,
            _ => panic!("Expected a minInclusive facet"),
        };

        compiler.push_fragment(FacetFragment::MinInclusive {
            value: Value(facet.value.clone()),
        })
    }

    fn from_simple_fragments<T: AsRef<SimpleTypeFragmentCompiler>>(
        _compiler: T,
        _fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        unimplemented!()
    }
}

impl SimpleFragmentEquivalent for xs::MaxExclusive {
    type FragmentId = FragmentIdx<FacetFragment>;

    fn to_simple_fragments<T: AsMut<SimpleTypeFragmentCompiler>>(
        &self,
        mut compiler: T,
    ) -> Self::FragmentId {
        let compiler = compiler.as_mut();

        let facet = match self {
            xs::MaxExclusive::MaxExclusive(facet) => facet,
            _ => panic!("Expected a maxExclusive facet"),
        };

        compiler.push_fragment(FacetFragment::MaxExclusive {
            value: Value(facet.value.clone()),
        })
    }

    fn from_simple_fragments<T: AsRef<SimpleTypeFragmentCompiler>>(
        _compiler: T,
        _fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        unimplemented!()
    }
}

impl SimpleFragmentEquivalent for xs::MaxInclusive {
    type FragmentId = FragmentIdx<FacetFragment>;

    fn to_simple_fragments<T: AsMut<SimpleTypeFragmentCompiler>>(
        &self,
        mut compiler: T,
    ) -> Self::FragmentId {
        let compiler = compiler.as_mut();

        let facet = match self {
            xs::MaxInclusive::MaxInclusive(facet) => facet,
            _ => panic!("Expected a maxInclusive facet"),
        };

        compiler.push_fragment(FacetFragment::MaxInclusive {
            value: Value(facet.value.clone()),
        })
    }

    fn from_simple_fragments<T: AsRef<SimpleTypeFragmentCompiler>>(
        _compiler: T,
        _fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        unimplemented!()
    }
}

impl SimpleFragmentEquivalent for xs::Enumeration {
    type FragmentId = FragmentIdx<FacetFragment>;

    fn to_simple_fragments<T: AsMut<SimpleTypeFragmentCompiler>>(
        &self,
        mut compiler: T,
    ) -> Self::FragmentId {
        let compiler = compiler.as_mut();

        let facet = match self {
            xs::Enumeration::Enumeration(facet) => facet,
            _ => panic!("Expected an enumeration facet"),
        };

        compiler.push_fragment(FacetFragment::Enumeration {
            value: Value(facet.value.clone()),
        })
    }

    fn from_simple_fragments<T: AsRef<SimpleTypeFragmentCompiler>>(
        _compiler: T,
        _fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        unimplemented!()
    }
}

impl SimpleFragmentEquivalent for xs::TotalDigits {
    type FragmentId = FragmentIdx<FacetFragment>;

    fn to_simple_fragments<T: AsMut<SimpleTypeFragmentCompiler>>(
        &self,
        mut compiler: T,
    ) -> Self::FragmentId {
        let compiler = compiler.as_mut();

        let facet = match self {
            xs::TotalDigits::TotalDigits(facet) => facet,
            _ => panic!("Expected a total digits facet"),
        };

        compiler.push_fragment(FacetFragment::TotalDigits { value: facet.value })
    }

    fn from_simple_fragments<T: AsRef<SimpleTypeFragmentCompiler>>(
        _compiler: T,
        _fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        unimplemented!()
    }
}

impl SimpleFragmentEquivalent for xs::FractionDigits {
    type FragmentId = FragmentIdx<FacetFragment>;

    fn to_simple_fragments<T: AsMut<SimpleTypeFragmentCompiler>>(
        &self,
        mut compiler: T,
    ) -> Self::FragmentId {
        let compiler = compiler.as_mut();

        let facet = match self {
            xs::FractionDigits::FractionDigits(facet) => facet,
            _ => panic!("Expected a fraction digits facet"),
        };

        compiler.push_fragment(FacetFragment::FractionDigits { value: facet.value })
    }

    fn from_simple_fragments<T: AsRef<SimpleTypeFragmentCompiler>>(
        _compiler: T,
        _fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        unimplemented!()
    }
}

impl SimpleFragmentEquivalent for xs::Length {
    type FragmentId = FragmentIdx<FacetFragment>;

    fn to_simple_fragments<T: AsMut<SimpleTypeFragmentCompiler>>(
        &self,
        mut compiler: T,
    ) -> Self::FragmentId {
        let compiler = compiler.as_mut();

        let facet = match self {
            xs::Length::Length(facet) => facet,
            _ => panic!("Expected a length facet"),
        };

        compiler.push_fragment(FacetFragment::Length { value: facet.value })
    }

    fn from_simple_fragments<T: AsRef<SimpleTypeFragmentCompiler>>(
        _compiler: T,
        _fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        unimplemented!()
    }
}

impl SimpleFragmentEquivalent for xs::MinLength {
    type FragmentId = FragmentIdx<FacetFragment>;

    fn to_simple_fragments<T: AsMut<SimpleTypeFragmentCompiler>>(
        &self,
        mut compiler: T,
    ) -> Self::FragmentId {
        let compiler = compiler.as_mut();

        let facet = match self {
            xs::MinLength::MinLength(facet) => facet,
            _ => panic!("Expected a min length facet"),
        };

        compiler.push_fragment(FacetFragment::MinLength { value: facet.value })
    }

    fn from_simple_fragments<T: AsRef<SimpleTypeFragmentCompiler>>(
        _compiler: T,
        _fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        unimplemented!()
    }
}

impl SimpleFragmentEquivalent for xs::MaxLength {
    type FragmentId = FragmentIdx<FacetFragment>;

    fn to_simple_fragments<T: AsMut<SimpleTypeFragmentCompiler>>(
        &self,
        mut compiler: T,
    ) -> Self::FragmentId {
        let compiler = compiler.as_mut();

        let facet = match self {
            xs::MaxLength::MaxLength(facet) => facet,
            _ => panic!("Expected a max length facet"),
        };

        compiler.push_fragment(FacetFragment::MaxLength { value: facet.value })
    }

    fn from_simple_fragments<T: AsRef<SimpleTypeFragmentCompiler>>(
        _compiler: T,
        _fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        unimplemented!()
    }
}

impl SimpleFragmentEquivalent for xs::WhiteSpace {
    type FragmentId = FragmentIdx<FacetFragment>;

    fn to_simple_fragments<T: AsMut<SimpleTypeFragmentCompiler>>(
        &self,
        mut compiler: T,
    ) -> Self::FragmentId {
        let compiler = compiler.as_mut();

        let facet = match self {
            xs::WhiteSpace::WhiteSpace(facet) => facet,
            _ => panic!("Expected a white space facet"),
        };

        compiler.push_fragment(FacetFragment::WhiteSpace {
            value: facet.value.into(),
        })
    }

    fn from_simple_fragments<T: AsRef<SimpleTypeFragmentCompiler>>(
        _compiler: T,
        _fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        unimplemented!()
    }
}

impl SimpleFragmentEquivalent for xs::Pattern {
    type FragmentId = FragmentIdx<FacetFragment>;

    fn to_simple_fragments<T: AsMut<SimpleTypeFragmentCompiler>>(
        &self,
        mut compiler: T,
    ) -> Self::FragmentId {
        let compiler = compiler.as_mut();

        let facet = match self {
            xs::Pattern::Pattern(facet) => facet,
            _ => panic!("Expected a pattern facet"),
        };

        compiler.push_fragment(FacetFragment::Pattern {
            value: Pattern(facet.value.clone()),
        })
    }

    fn from_simple_fragments<T: AsRef<SimpleTypeFragmentCompiler>>(
        _compiler: T,
        _fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        unimplemented!()
    }
}

impl SimpleFragmentEquivalent for xs::Assertion {
    type FragmentId = FragmentIdx<FacetFragment>;

    fn to_simple_fragments<T: AsMut<SimpleTypeFragmentCompiler>>(
        &self,
        mut compiler: T,
    ) -> Self::FragmentId {
        let compiler = compiler.as_mut();

        let facet = match self {
            xs::Assertion::Assertion(facet) => facet,
            _ => panic!("Expected an assertion facet"),
        };

        compiler.push_fragment(FacetFragment::Assertion {
            test: facet.test.clone().map(Assertion),
        })
    }

    fn from_simple_fragments<T: AsRef<SimpleTypeFragmentCompiler>>(
        _compiler: T,
        _fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        unimplemented!()
    }
}

impl SimpleFragmentEquivalent for xs::ExplicitTimezone {
    type FragmentId = FragmentIdx<FacetFragment>;

    fn to_simple_fragments<T: AsMut<SimpleTypeFragmentCompiler>>(
        &self,
        mut compiler: T,
    ) -> Self::FragmentId {
        let compiler = compiler.as_mut();

        let facet = match self {
            xs::ExplicitTimezone::ExplicitTimezone(facet) => facet,
            _ => panic!("Expected an explicit timezone facet"),
        };

        compiler.push_fragment(FacetFragment::ExplicitTimezone {
            value: facet.value.into(),
        })
    }

    fn from_simple_fragments<T: AsRef<SimpleTypeFragmentCompiler>>(
        _compiler: T,
        _fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        unimplemented!()
    }
}

impl SimpleFragmentEquivalent for xs::List {
    type FragmentId = FragmentIdx<ListFragment>;

    fn to_simple_fragments<T: AsMut<SimpleTypeFragmentCompiler>>(
        &self,
        mut compiler: T,
    ) -> Self::FragmentId {
        let mut compiler = compiler.as_mut();

        let list = match self {
            xs::List::List(list) => list,
            _ => panic!("Expected a list"),
        };

        let item_type = if let Some(item_type) = list.item_type.as_ref() {
            NamedOrAnonymous::Named(item_type.0.clone())
        } else {
            let simple_type = list.simple_type.as_ref().unwrap();

            NamedOrAnonymous::Anonymous(simple_type.to_simple_fragments(&mut compiler))
        };

        compiler.push_fragment(ListFragment { item_type })
    }

    fn from_simple_fragments<T: AsRef<SimpleTypeFragmentCompiler>>(
        compiler: T,
        fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        let compiler = compiler.as_ref();

        let fragment = compiler.get_fragment(fragment_id).ok_or(Error)?;

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
            .build()
            .into())
    }
}

impl SimpleFragmentEquivalent for xs::union_items::SimpleType {
    type FragmentId = FragmentIdx<SimpleTypeRootFragment>;

    fn to_simple_fragments<T: AsMut<SimpleTypeFragmentCompiler>>(
        &self,
        compiler: T,
    ) -> Self::FragmentId {
        self.0.to_simple_fragments(compiler)
    }

    fn from_simple_fragments<T: AsRef<SimpleTypeFragmentCompiler>>(
        compiler: T,
        fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        let compiler = compiler.as_ref();

        let simple_type = xs::types::LocalSimpleType::from_simple_fragments(compiler, fragment_id)?;

        Ok(simple_type.into())
    }
}

impl SimpleFragmentEquivalent for xs::Union {
    type FragmentId = FragmentIdx<UnionFragment>;

    fn to_simple_fragments<T: AsMut<SimpleTypeFragmentCompiler>>(
        &self,
        mut compiler: T,
    ) -> Self::FragmentId {
        let mut compiler = compiler.as_mut();

        let union = match self {
            xs::Union::Union(union) => union,
            _ => panic!("Expected a union"),
        };

        let member_types = union
            .member_types
            .as_ref()
            .map(|member_type| member_type.0.iter().map(|a| a.0.clone()).collect())
            .unwrap_or_default();

        let simple_types = union
            .simple_type
            .iter()
            .map(|simple_type| simple_type.to_simple_fragments(&mut compiler))
            .collect();

        compiler.push_fragment(UnionFragment {
            member_types,
            simple_types,
        })
    }

    fn from_simple_fragments<T: AsRef<SimpleTypeFragmentCompiler>>(
        compiler: T,
        fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        let compiler = compiler.as_ref();

        let fragment = compiler.get_fragment(fragment_id).ok_or(Error)?;

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
            .build()
            .into())
    }
}

impl SimpleFragmentEquivalent for xs::Restriction {
    type FragmentId = FragmentIdx<RestrictionFragment>;

    fn to_simple_fragments<T: AsMut<SimpleTypeFragmentCompiler>>(
        &self,
        mut compiler: T,
    ) -> Self::FragmentId {
        let mut compiler = compiler.as_mut();

        let restriction = match self {
            xs::Restriction::Restriction(restriction) => restriction,
            _ => panic!("Expected a restriction"),
        };

        let base = restriction.base.as_ref().map(|a| a.0.clone());

        let simple_type = restriction
            .simple_restriction_model
            .simple_type
            .as_ref()
            .map(|simple_type| simple_type.to_simple_fragments(&mut compiler));

        let facets = restriction
            .simple_restriction_model
            .child_1
            .iter()
            .filter_map(|a| match a {
                xs::groups::simple_restriction_model_items::Child1::Facet(facet) => Some(facet),
                _ => None,
            })
            .map(|facet| facet.to_simple_fragments(&mut compiler))
            .collect();

        compiler.push_fragment(RestrictionFragment {
            base,
            simple_type,
            facets,
        })
    }

    fn from_simple_fragments<T: AsRef<SimpleTypeFragmentCompiler>>(
        compiler: T,
        fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        let compiler = compiler.as_ref();

        let fragment = compiler.get_fragment(fragment_id).ok_or(Error)?;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SimpleDerivation {
    Restriction(FragmentIdx<RestrictionFragment>),
    List(FragmentIdx<ListFragment>),
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
    ) -> Self::FragmentId {
        use xs::groups::SimpleDerivation as S;
        match self {
            S::Restriction(restriction) => restriction.to_simple_fragments(compiler).into(),
            S::List(list) => list.to_simple_fragments(compiler).into(),
            S::Union(union) => union.to_simple_fragments(compiler).into(),
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

// #[cfg(test)]
// mod tests {
//     use xmlity::{ExpandedName, LocalName};
//     use xs::{
//         Facet as XsdFacet, QName, SimpleDerivation, SimpleRestrictionType, TopLevelSimpleType,
//     };

//     use super::*;

//     #[test]
//     fn convert_annotated_to_fragments() {
//         let namespace = XmlNamespace::new_dangerous("http://localhost");

//         let mut fragment_compiler = SimpleTypeFragmentCompiler::new(namespace.clone());

//         // <xs:simpleType name="formChoice">
//         //     <xs:annotation>
//         //         <xs:documentation>
//         //     A utility type, not for public use</xs:documentation>
//         //     </xs:annotation>
//         //     <xs:restriction base="xs:NMTOKEN">
//         //         <xs:enumeration value="qualified"/>
//         //         <xs:enumeration value="unqualified"/>
//         //     </xs:restriction>
//         // </xs:simpleType>
//         let id = TopLevelSimpleType {
//             id: None,
//             name: LocalName::new_dangerous("annotated"),
//             final_: None,
//             annotation: None,
//             content: SimpleDerivation::Restriction(Box::new(SimpleRestrictionType {
//                 id: None,
//                 base: QName(ExpandedName::new(
//                     LocalName::new_dangerous("NMTOKEN"),
//                     Some(XmlNamespace::XS),
//                 )),
//                 annotation: None,
//                 simple_type: None,
//                 facets: vec![
//                     XsdFacet::Enumeration(Box::new(xs::Enumeration {
//                         fixed: None,
//                         value: "qualified".to_string(),
//                     })),
//                     XsdFacet::Enumeration(Box::new(xs::Enumeration {
//                         fixed: None,
//                         value: "unqualified".to_string(),
//                     })),
//                 ],
//             })),
//         }
//         .to_simple_fragments(&mut fragment_compiler);

//         assert_eq!(id, FragmentId(namespace, FragmentIdx(2)));
//         assert_eq!(fragment_compiler.fragments.len(), 3);

//         assert!(matches!(
//             fragment_compiler.fragments[&FragmentIdx(0)],
//             SimpleTypeFragment::Facet(Facet::Enumeration { .. })
//         ));
//         assert!(matches!(
//             fragment_compiler.fragments[&FragmentIdx(1)],
//             SimpleTypeFragment::Facet(Facet::Enumeration { .. })
//         ));
//         assert!(matches!(
//             fragment_compiler.fragments[&FragmentIdx(2)],
//             SimpleTypeFragment::Restriction(_)
//         ));

//         println!("{:#?}", fragment_compiler);
//     }
// }
