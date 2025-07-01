pub mod transformers;

use std::num::NonZeroUsize;
use std::str::FromStr;
use xsd::xs;

use xmlity::{ExpandedName, LocalName, XmlNamespace};

use crate::{
    fragments::{FragmentAccess, FragmentCollection, FragmentIdx, HasFragmentCollection},
    NamedOrAnonymous,
};
use std::collections::VecDeque;

#[derive(Debug, Clone, PartialEq)]
pub struct ExtensionFragment {
    pub base: ExpandedName<'static>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RestrictionFragment {
    pub base: ExpandedName<'static>,
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
    pub simple_types: VecDeque<FragmentIdx<SimpleTypeRootFragment>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GroupRefFragment {
    pub ref_: ExpandedName<'static>,
}

// #[derive(Debug, Clone, PartialEq)]
// pub enum SimpleTypeRootFragment {
//     // GroupRef { ref_: SimpleTypeIdent },
//     Restriction(Restriction),
//     List { item_ident: SimpleTypeIdent },
//     Union { fragments: VecDeque<FragmentId> },
//     Facet(Facet),
// }

#[derive(Debug, Clone, PartialEq)]
pub struct Value(pub String);

#[derive(Debug, Clone, PartialEq)]
pub struct Pattern(pub String);

#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug, Clone, PartialEq)]
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

impl SimpleTypeFragmentCompiler {
    pub fn new(namespace: XmlNamespace<'static>) -> Self {
        Self {
            namespace,
            simple_types: FragmentCollection::new(),
            restrictions: FragmentCollection::new(),
            extensions: FragmentCollection::new(),
            facets: FragmentCollection::new(),
            lists: FragmentCollection::new(),
            unions: FragmentCollection::new(),
            group_refs: FragmentCollection::new(),
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

impl AsMut<SimpleTypeFragmentCompiler> for SimpleTypeFragmentCompiler {
    fn as_mut(&mut self) -> &mut SimpleTypeFragmentCompiler {
        self
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
        _compiler: T,
        _fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        unimplemented!()
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
        _compiler: T,
        _fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        unimplemented!()
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
                        //TODO
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
            base,
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
        _compiler: T,
        _fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        unimplemented!()
    }
}

impl SimpleFragmentEquivalent for xs::MinExclusive {
    type FragmentId = FragmentIdx<FacetFragment>;

    fn to_simple_fragments<T: AsMut<SimpleTypeFragmentCompiler>>(
        &self,
        mut compiler: T,
    ) -> Self::FragmentId {
        let compiler = compiler.as_mut();

        compiler.push_fragment(FacetFragment::MinExclusive {
            value: Value(self.0.value.clone()),
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

        compiler.push_fragment(FacetFragment::MinInclusive {
            value: Value(self.0.value.clone()),
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

        compiler.push_fragment(FacetFragment::MaxExclusive {
            value: Value(self.0.value.clone()),
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

        compiler.push_fragment(FacetFragment::MaxInclusive {
            value: Value(self.0.value.clone()),
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

        compiler.push_fragment(FacetFragment::Enumeration {
            value: Value(self.0.value.clone()),
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

        compiler.push_fragment(FacetFragment::TotalDigits {
            //TODO
            value: self.value,
        })
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

        compiler.push_fragment(FacetFragment::FractionDigits {
            value: self.0.value.clone(),
        })
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

        compiler.push_fragment(FacetFragment::Length {
            value: self.0.value.clone(),
        })
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

        compiler.push_fragment(FacetFragment::MinLength {
            value: self.0.value.clone(),
        })
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

        compiler.push_fragment(FacetFragment::MaxLength {
            value: self.0.value.clone(),
        })
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

        compiler.push_fragment(FacetFragment::WhiteSpace {
            // TODO
            value: self.value.into(),
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

        compiler.push_fragment(FacetFragment::Pattern {
            value: Pattern(self.value.clone()),
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

        compiler.push_fragment(FacetFragment::Assertion {
            test: self.0.test.clone().map(Assertion),
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

        compiler.push_fragment(FacetFragment::ExplicitTimezone {
            //TODO
            value: self.value.into(),
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

        let item_type = if let Some(item_type) = self.item_type.as_ref() {
            NamedOrAnonymous::Named(item_type.0.clone())
        } else {
            let simple_type = self.simple_type.as_ref().unwrap();

            NamedOrAnonymous::Anonymous(simple_type.to_simple_fragments(&mut compiler))
        };

        compiler.push_fragment(ListFragment { item_type })
    }

    fn from_simple_fragments<T: AsRef<SimpleTypeFragmentCompiler>>(
        _compiler: T,
        _fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        unimplemented!()
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
        _compiler: T,
        _fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        unimplemented!()
    }
}

impl SimpleFragmentEquivalent for xs::Union {
    type FragmentId = FragmentIdx<UnionFragment>;

    fn to_simple_fragments<T: AsMut<SimpleTypeFragmentCompiler>>(
        &self,
        mut compiler: T,
    ) -> Self::FragmentId {
        let mut compiler = compiler.as_mut();

        let simple_types = self
            .simple_type
            .iter()
            .map(|simple_type| simple_type.to_simple_fragments(&mut compiler))
            .collect();

        compiler.push_fragment(UnionFragment { simple_types })
    }

    fn from_simple_fragments<T: AsRef<SimpleTypeFragmentCompiler>>(
        _compiler: T,
        _fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        unimplemented!()
    }
}

impl SimpleFragmentEquivalent for xs::Restriction {
    type FragmentId = FragmentIdx<RestrictionFragment>;

    fn to_simple_fragments<T: AsMut<SimpleTypeFragmentCompiler>>(
        &self,
        mut compiler: T,
    ) -> Self::FragmentId {
        let mut compiler = compiler.as_mut();

        let base = self
            .base
            .as_ref()
            .map(|a| a.0.clone())
            .unwrap_or_else(|| todo!());

        let simple_type = self
            .simple_restriction_model
            .simple_type
            .as_ref()
            .map(|simple_type| simple_type.to_simple_fragments(&mut compiler));

        let facets = self
            .simple_restriction_model
            .child_1
            .iter()
            .filter_map(|a| match a {
                //TODO
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
        _compiler: T,
        _fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        unimplemented!()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
        _compiler: T,
        _fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        unimplemented!()
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
