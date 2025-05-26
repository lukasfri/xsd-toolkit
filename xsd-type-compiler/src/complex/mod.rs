//! This module contains the logic for compiling complex types into fragments.
//!
//! It is naturally dependent on the simple type compiler, as complex types can contain simple types.

pub mod transformers;

use std::{
    collections::{BTreeMap, VecDeque},
    marker::PhantomData,
    sync::LazyLock,
};

use crate::{
    simple::{self, SimpleTypeFragmentCompiler, ToSimpleFragments},
    NamedOrAnonymous, SimpleTypeIdent,
};
use xmlity::{ExpandedName, LocalName, XmlNamespace};

use xsd::schema::{self as xs, GroupTypeContent};

#[derive(Debug)]
pub struct FragmentIdx<T>(usize, PhantomData<T>);

impl<T> FragmentIdx<T> {
    pub fn new(index: usize) -> Self {
        Self(index, PhantomData)
    }
}

impl<T> Clone for FragmentIdx<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for FragmentIdx<T> {}
impl<T> PartialEq for FragmentIdx<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}
impl<T> Eq for FragmentIdx<T> {}
impl<T> PartialOrd for FragmentIdx<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl<T> Ord for FragmentIdx<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0).then_with(|| self.1.cmp(&other.1))
    }
}
impl<T> std::hash::Hash for FragmentIdx<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
        self.1.hash(state);
    }
}

// #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// pub struct FragmentId(pub XmlNamespace<'static>, pub FragmentIdx);

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TypeDefParticleId {
    Group(FragmentIdx<GroupRefFragment>),
    All(FragmentIdx<AllFragment>),
    Sequence(FragmentIdx<SequenceFragment>),
    Choice(FragmentIdx<ChoiceFragment>),
}

impl From<FragmentIdx<GroupRefFragment>> for TypeDefParticleId {
    fn from(value: FragmentIdx<GroupRefFragment>) -> Self {
        Self::Group(value)
    }
}
impl From<FragmentIdx<AllFragment>> for TypeDefParticleId {
    fn from(value: FragmentIdx<AllFragment>) -> Self {
        Self::All(value)
    }
}
impl From<FragmentIdx<SequenceFragment>> for TypeDefParticleId {
    fn from(value: FragmentIdx<SequenceFragment>) -> Self {
        Self::Sequence(value)
    }
}
impl From<FragmentIdx<ChoiceFragment>> for TypeDefParticleId {
    fn from(value: FragmentIdx<ChoiceFragment>) -> Self {
        Self::Choice(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExtensionFragment {
    pub base: ExpandedName<'static>,
    pub content_fragment: Option<TypeDefParticleId>,
    pub attribute_declarations: VecDeque<AttributeDeclarationId>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RestrictionFragment {
    pub base: ExpandedName<'static>,
    pub content_fragment: Option<TypeDefParticleId>,
    pub attribute_declarations: VecDeque<AttributeDeclarationId>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AttributeDeclarationId {
    Attribute(FragmentIdx<LocalAttributeFragment>),
    AttributeGroupRef(FragmentIdx<AttributeGroupRefFragment>),
}

impl From<FragmentIdx<LocalAttributeFragment>> for AttributeDeclarationId {
    fn from(value: FragmentIdx<LocalAttributeFragment>) -> Self {
        Self::Attribute(value)
    }
}
impl From<FragmentIdx<AttributeGroupRefFragment>> for AttributeDeclarationId {
    fn from(value: FragmentIdx<AttributeGroupRefFragment>) -> Self {
        Self::AttributeGroupRef(value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum AttributeUse {
    Required,
    #[default]
    Optional,
    Prohibited,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DeclaredAttributeFragment {
    pub name: LocalName<'static>,
    pub type_: Option<NamedOrAnonymous<simple::FragmentId>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ReferenceAttributeFragment {
    pub name: ExpandedName<'static>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LocalAttributeFragmentTypeMode {
    Declared(DeclaredAttributeFragment),
    Reference(ReferenceAttributeFragment),
}

#[derive(Debug, Clone, PartialEq)]
pub struct LocalAttributeFragment {
    pub type_mode: LocalAttributeFragmentTypeMode,
    pub use_: Option<AttributeUse>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TopLevelAttributeFragment {
    pub name: LocalName<'static>,
    pub type_: Option<NamedOrAnonymous<simple::FragmentId>>,
    pub use_: Option<AttributeUse>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AttributeGroupRefFragment {}

#[derive(Debug, Clone, PartialEq)]
pub enum SimpleContentFragment {}

#[derive(Debug, Clone, PartialEq)]
pub struct ComplexContentFragment {
    pub mixed: Option<bool>,
    pub content_fragment: ComplexContentChildId,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ComplexContentChildId {
    Extension(FragmentIdx<ExtensionFragment>),
    Restriction(FragmentIdx<RestrictionFragment>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct DeclaredElementFragment {
    pub name: LocalName<'static>,
    pub type_: NamedOrAnonymous<ElementTypeContentId>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ReferenceElementFragment {
    pub name: ExpandedName<'static>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LocalElementFragmentType {
    Local(DeclaredElementFragment),
    Reference(ReferenceElementFragment),
}

#[derive(Debug, Clone, PartialEq)]
pub struct LocalElementFragment {
    pub min_occurs: Option<xs::MinOccurs>,
    pub max_occurs: Option<xs::MaxOccursValue>,
    pub type_: LocalElementFragmentType,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TopLevelElementFragment {
    pub name: LocalName<'static>,
    pub type_: NamedOrAnonymous<ElementTypeContentId>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GroupRefFragment {
    pub ref_: ExpandedName<'static>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AllFragment {
    pub min_occurs: Option<xs::MinOccurs>,
    pub max_occurs: Option<xs::MaxOccursValue>,
    pub fragments: VecDeque<GroupTypeContentId>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ChoiceFragment {
    pub min_occurs: Option<xs::MinOccurs>,
    pub max_occurs: Option<xs::MaxOccursValue>,
    pub fragments: VecDeque<GroupTypeContentId>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SequenceFragment {
    pub min_occurs: Option<xs::MinOccurs>,
    pub max_occurs: Option<xs::MaxOccursValue>,
    pub fragments: VecDeque<GroupTypeContentId>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SimpleTypeFragment {
    pub type_: SimpleTypeIdent,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ComplexTypeModelId {
    SimpleContent(FragmentIdx<SimpleContentFragment>),
    ComplexContent(FragmentIdx<ComplexContentFragment>),
    Other { particle: TypeDefParticleId },
}

#[derive(Debug, Clone, PartialEq)]
pub struct ComplexTypeRootFragment {
    pub name: Option<LocalName<'static>>,
    pub content: ComplexTypeModelId,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AnyFragment {}

#[derive(Debug, Clone)]
pub struct FragmentCollection<T> {
    fragment_id_count: usize,
    pub fragments: BTreeMap<FragmentIdx<T>, T>,
}

impl<T> FragmentCollection<T> {
    pub fn new() -> Self {
        Self {
            fragment_id_count: 0,
            fragments: BTreeMap::new(),
        }
    }

    fn generate_fragment_id(&mut self) -> FragmentIdx<T> {
        let fragment_id = FragmentIdx::new(self.fragment_id_count);
        self.fragment_id_count += 1;
        fragment_id
    }

    fn len(&self) -> usize {
        self.fragments.len()
    }
}

impl<T> FragmentCollection<T> {
    fn get_fragment(&self, fragment_id: &FragmentIdx<T>) -> Option<&T> {
        self.fragments.get(&fragment_id)
    }

    fn get_fragment_mut(&mut self, fragment_id: &FragmentIdx<T>) -> Option<&mut T> {
        self.fragments.get_mut(&fragment_id)
    }

    fn push_fragment(&mut self, fragment: T) -> FragmentIdx<T> {
        let fragment_id = self.generate_fragment_id();
        self.fragments.insert(fragment_id, fragment);
        fragment_id
    }

    pub fn iter_fragment_ids(&self) -> Vec<FragmentIdx<T>> {
        self.fragments.keys().map(|idx| *idx).collect::<Vec<_>>()
    }
}

#[derive(Debug, Clone)]
pub struct ComplexTypeFragmentCompiler {
    pub namespace: XmlNamespace<'static>,
    pub simple_type_fragments: SimpleTypeFragmentCompiler,
    pub complex_types: FragmentCollection<ComplexTypeRootFragment>,
    pub simple_contents: FragmentCollection<SimpleContentFragment>,
    pub restrictions: FragmentCollection<RestrictionFragment>,
    pub extensions: FragmentCollection<ExtensionFragment>,
    pub complex_contents: FragmentCollection<ComplexContentFragment>,
    pub group_refs: FragmentCollection<GroupRefFragment>,
    pub alls: FragmentCollection<AllFragment>,
    pub choices: FragmentCollection<ChoiceFragment>,
    pub sequences: FragmentCollection<SequenceFragment>,
    pub anys: FragmentCollection<AnyFragment>,
    pub elements: FragmentCollection<LocalElementFragment>,
    pub top_level_elements: FragmentCollection<TopLevelElementFragment>,
    pub local_attributes: FragmentCollection<LocalAttributeFragment>,
    pub top_level_attributes: FragmentCollection<TopLevelAttributeFragment>,
    pub simple_types: FragmentCollection<SimpleTypeFragment>,
}

pub trait FragmentAccess<F>: Sized {
    fn get_fragment(&self, fragment_id: &FragmentIdx<F>) -> Option<&F>;
    fn get_fragment_mut(&mut self, fragment_id: &FragmentIdx<F>) -> Option<&mut F>;

    fn push_fragment(&mut self, fragment: F) -> FragmentIdx<F>;

    fn iter_fragment_ids(&self) -> Vec<FragmentIdx<F>>;
}

trait HasFragmentCollection<F> {
    fn get_fragment_collection(&self) -> &FragmentCollection<F>;
    fn get_fragment_collection_mut(&mut self) -> &mut FragmentCollection<F>;
}

impl HasFragmentCollection<ComplexTypeRootFragment> for ComplexTypeFragmentCompiler {
    fn get_fragment_collection(&self) -> &FragmentCollection<ComplexTypeRootFragment> {
        &self.complex_types
    }
    fn get_fragment_collection_mut(&mut self) -> &mut FragmentCollection<ComplexTypeRootFragment> {
        &mut self.complex_types
    }
}

impl HasFragmentCollection<SimpleContentFragment> for ComplexTypeFragmentCompiler {
    fn get_fragment_collection(&self) -> &FragmentCollection<SimpleContentFragment> {
        &self.simple_contents
    }
    fn get_fragment_collection_mut(&mut self) -> &mut FragmentCollection<SimpleContentFragment> {
        &mut self.simple_contents
    }
}

impl HasFragmentCollection<ExtensionFragment> for ComplexTypeFragmentCompiler {
    fn get_fragment_collection(&self) -> &FragmentCollection<ExtensionFragment> {
        &self.extensions
    }
    fn get_fragment_collection_mut(&mut self) -> &mut FragmentCollection<ExtensionFragment> {
        &mut self.extensions
    }
}

impl HasFragmentCollection<RestrictionFragment> for ComplexTypeFragmentCompiler {
    fn get_fragment_collection(&self) -> &FragmentCollection<RestrictionFragment> {
        &self.restrictions
    }
    fn get_fragment_collection_mut(&mut self) -> &mut FragmentCollection<RestrictionFragment> {
        &mut self.restrictions
    }
}

impl HasFragmentCollection<ComplexContentFragment> for ComplexTypeFragmentCompiler {
    fn get_fragment_collection(&self) -> &FragmentCollection<ComplexContentFragment> {
        &self.complex_contents
    }
    fn get_fragment_collection_mut(&mut self) -> &mut FragmentCollection<ComplexContentFragment> {
        &mut self.complex_contents
    }
}

impl HasFragmentCollection<GroupRefFragment> for ComplexTypeFragmentCompiler {
    fn get_fragment_collection(&self) -> &FragmentCollection<GroupRefFragment> {
        &self.group_refs
    }
    fn get_fragment_collection_mut(&mut self) -> &mut FragmentCollection<GroupRefFragment> {
        &mut self.group_refs
    }
}

impl HasFragmentCollection<AllFragment> for ComplexTypeFragmentCompiler {
    fn get_fragment_collection(&self) -> &FragmentCollection<AllFragment> {
        &self.alls
    }
    fn get_fragment_collection_mut(&mut self) -> &mut FragmentCollection<AllFragment> {
        &mut self.alls
    }
}

impl HasFragmentCollection<ChoiceFragment> for ComplexTypeFragmentCompiler {
    fn get_fragment_collection(&self) -> &FragmentCollection<ChoiceFragment> {
        &self.choices
    }
    fn get_fragment_collection_mut(&mut self) -> &mut FragmentCollection<ChoiceFragment> {
        &mut self.choices
    }
}

impl HasFragmentCollection<SequenceFragment> for ComplexTypeFragmentCompiler {
    fn get_fragment_collection(&self) -> &FragmentCollection<SequenceFragment> {
        &self.sequences
    }
    fn get_fragment_collection_mut(&mut self) -> &mut FragmentCollection<SequenceFragment> {
        &mut self.sequences
    }
}

impl HasFragmentCollection<AnyFragment> for ComplexTypeFragmentCompiler {
    fn get_fragment_collection(&self) -> &FragmentCollection<AnyFragment> {
        &self.anys
    }
    fn get_fragment_collection_mut(&mut self) -> &mut FragmentCollection<AnyFragment> {
        &mut self.anys
    }
}

impl HasFragmentCollection<LocalElementFragment> for ComplexTypeFragmentCompiler {
    fn get_fragment_collection(&self) -> &FragmentCollection<LocalElementFragment> {
        &self.elements
    }
    fn get_fragment_collection_mut(&mut self) -> &mut FragmentCollection<LocalElementFragment> {
        &mut self.elements
    }
}

impl HasFragmentCollection<TopLevelElementFragment> for ComplexTypeFragmentCompiler {
    fn get_fragment_collection(&self) -> &FragmentCollection<TopLevelElementFragment> {
        &self.top_level_elements
    }
    fn get_fragment_collection_mut(&mut self) -> &mut FragmentCollection<TopLevelElementFragment> {
        &mut self.top_level_elements
    }
}

impl HasFragmentCollection<LocalAttributeFragment> for ComplexTypeFragmentCompiler {
    fn get_fragment_collection(&self) -> &FragmentCollection<LocalAttributeFragment> {
        &self.local_attributes
    }
    fn get_fragment_collection_mut(&mut self) -> &mut FragmentCollection<LocalAttributeFragment> {
        &mut self.local_attributes
    }
}

impl HasFragmentCollection<TopLevelAttributeFragment> for ComplexTypeFragmentCompiler {
    fn get_fragment_collection(&self) -> &FragmentCollection<TopLevelAttributeFragment> {
        &self.top_level_attributes
    }
    fn get_fragment_collection_mut(
        &mut self,
    ) -> &mut FragmentCollection<TopLevelAttributeFragment> {
        &mut self.top_level_attributes
    }
}

impl HasFragmentCollection<SimpleTypeFragment> for ComplexTypeFragmentCompiler {
    fn get_fragment_collection(&self) -> &FragmentCollection<SimpleTypeFragment> {
        &self.simple_types
    }
    fn get_fragment_collection_mut(&mut self) -> &mut FragmentCollection<SimpleTypeFragment> {
        &mut self.simple_types
    }
}

impl<T: 'static> FragmentAccess<T> for ComplexTypeFragmentCompiler
where
    ComplexTypeFragmentCompiler: HasFragmentCollection<T>,
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

impl ComplexTypeFragmentCompiler {
    pub fn new(
        namespace: XmlNamespace<'static>,
        simple_type_fragments: SimpleTypeFragmentCompiler,
    ) -> Self {
        Self {
            namespace,
            simple_type_fragments,
            complex_types: FragmentCollection::new(),
            simple_contents: FragmentCollection::new(),
            restrictions: FragmentCollection::new(),
            extensions: FragmentCollection::new(),
            complex_contents: FragmentCollection::new(),
            group_refs: FragmentCollection::new(),
            alls: FragmentCollection::new(),
            choices: FragmentCollection::new(),
            sequences: FragmentCollection::new(),
            anys: FragmentCollection::new(),
            elements: FragmentCollection::new(),
            top_level_elements: FragmentCollection::new(),
            local_attributes: FragmentCollection::new(),
            top_level_attributes: FragmentCollection::new(),
            simple_types: FragmentCollection::new(),
        }
    }
}

impl AsMut<SimpleTypeFragmentCompiler> for ComplexTypeFragmentCompiler {
    fn as_mut(&mut self) -> &mut SimpleTypeFragmentCompiler {
        &mut self.simple_type_fragments
    }
}

impl AsMut<ComplexTypeFragmentCompiler> for ComplexTypeFragmentCompiler {
    fn as_mut(&mut self) -> &mut ComplexTypeFragmentCompiler {
        self
    }
}

impl AsRef<SimpleTypeFragmentCompiler> for ComplexTypeFragmentCompiler {
    fn as_ref(&self) -> &SimpleTypeFragmentCompiler {
        &self.simple_type_fragments
    }
}

impl AsRef<ComplexTypeFragmentCompiler> for ComplexTypeFragmentCompiler {
    fn as_ref(&self) -> &ComplexTypeFragmentCompiler {
        self
    }
}

#[derive(Debug, Clone)]
pub struct Error;

pub trait ComplexFragmentEquivalent: Sized {
    type FragmentId;

    fn to_complex_fragments<T: AsMut<ComplexTypeFragmentCompiler>>(
        &self,
        compiler: T,
    ) -> Self::FragmentId;

    fn from_complex_fragments<T: AsRef<ComplexTypeFragmentCompiler>>(
        compiler: T,
        fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error>;
}

#[derive(Debug, Clone, PartialEq)]
pub enum ElementTypeContentId {
    SimpleType(simple::FragmentId),
    ComplexType(FragmentIdx<ComplexTypeRootFragment>),
}

impl ComplexFragmentEquivalent for xs::ElementTypeContent {
    type FragmentId = ElementTypeContentId;

    fn to_complex_fragments<T: AsMut<ComplexTypeFragmentCompiler>>(
        &self,
        mut compiler: T,
    ) -> Self::FragmentId {
        let mut compiler = compiler.as_mut();

        match self {
            xs::ElementTypeContent::SimpleType(local_simple_type) => {
                let simple_type_fragment = local_simple_type.to_simple_fragments(&mut compiler);

                ElementTypeContentId::SimpleType(simple_type_fragment)
            }
            xs::ElementTypeContent::ComplexType(local_complex_type) => {
                let complex_type_fragment = local_complex_type.to_complex_fragments(compiler);

                ElementTypeContentId::ComplexType(complex_type_fragment)
            }
        }
    }

    fn from_complex_fragments<T: AsRef<ComplexTypeFragmentCompiler>>(
        compiler: T,
        fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        let compiler = compiler.as_ref();

        match fragment_id {
            ElementTypeContentId::SimpleType(fragment_id) => {
                xs::LocalSimpleType::from_complex_fragments(compiler, fragment_id)
                    .map(xs::ElementTypeContent::from)
            }
            ElementTypeContentId::ComplexType(fragment_idx) => {
                xs::LocalComplexType::from_complex_fragments(compiler, fragment_idx)
                    .map(xs::ElementTypeContent::from)
            }
        }
    }
}

impl ComplexFragmentEquivalent for xs::LocalElement {
    type FragmentId = FragmentIdx<LocalElementFragment>;

    fn to_complex_fragments<T: AsMut<ComplexTypeFragmentCompiler>>(
        &self,
        mut compiler: T,
    ) -> Self::FragmentId {
        let mut compiler = compiler.as_mut();

        let max_occurs = self.max_occurs.as_ref().map(|a| a.0);
        let min_occurs = self.min_occurs.clone();

        let type_ = if let Some(ref_) = self.ref_.as_ref() {
            LocalElementFragmentType::Reference(ReferenceElementFragment {
                name: ref_.0 .0.clone(),
            })
        } else {
            let name = self
                .name
                .as_ref()
                .map(|a| a.0.clone())
                .expect("If ref is none, type_choice should be Some");

            let type_ = if let Some(type_) = self.type_.as_ref() {
                NamedOrAnonymous::Named(type_.0 .0.clone())
            } else {
                let type_choice = self
                    .type_choice
                    .as_ref()
                    .expect("If ref is none and type is none, type_choice should be Some");

                let content_type = type_choice.to_complex_fragments(&mut compiler);

                NamedOrAnonymous::Anonymous(content_type)
            };

            LocalElementFragmentType::Local(DeclaredElementFragment { name, type_ })
        };

        compiler.push_fragment(LocalElementFragment {
            type_,
            max_occurs,
            min_occurs,
        })
    }

    fn from_complex_fragments<T: AsRef<ComplexTypeFragmentCompiler>>(
        compiler: T,
        fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        let compiler = compiler.as_ref();

        let fragment = compiler.get_fragment(fragment_id).unwrap();

        let element_builder = xs::LocalElement::builder()
            .maybe_min_occurs(fragment.min_occurs)
            .maybe_max_occurs(fragment.max_occurs.map(xs::MaxOccurs));

        match &fragment.type_ {
            LocalElementFragmentType::Local(fragment) => Ok(element_builder
                .name(xs::Name(fragment.name.clone()))
                .maybe_type_(match &fragment.type_ {
                    NamedOrAnonymous::Named(expanded_name) => {
                        Some(xs::Type(xs::QName(expanded_name.clone())))
                    }
                    NamedOrAnonymous::Anonymous(_) => None,
                })
                .maybe_type_choice(
                    match &fragment.type_ {
                        NamedOrAnonymous::Anonymous(content_type) => Some(
                            xs::ElementTypeContent::from_complex_fragments(compiler, content_type),
                        ),
                        NamedOrAnonymous::Named(_) => None,
                    }
                    .transpose()?,
                )
                .build()),
            LocalElementFragmentType::Reference(fragment) => Ok(element_builder
                .ref_(xs::Ref(xs::QName(fragment.name.clone())))
                .build()),
        }
    }
}

impl ComplexFragmentEquivalent for xs::TopLevelElement {
    type FragmentId = FragmentIdx<TopLevelElementFragment>;

    fn to_complex_fragments<T: AsMut<ComplexTypeFragmentCompiler>>(
        &self,
        mut compiler: T,
    ) -> Self::FragmentId {
        let mut compiler = compiler.as_mut();

        let name = self.name.0.clone();

        let type_ = if let Some(type_) = self.type_.as_ref() {
            NamedOrAnonymous::Named(type_.0 .0.clone())
        } else {
            let type_choice = self
                .type_choice
                .as_ref()
                .expect("If ref is none and type is none, type_choice should be Some");

            let content_type = type_choice.to_complex_fragments(&mut compiler);

            NamedOrAnonymous::Anonymous(content_type)
        };

        compiler.push_fragment(TopLevelElementFragment { name, type_ })
    }

    fn from_complex_fragments<T: AsRef<ComplexTypeFragmentCompiler>>(
        compiler: T,
        fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        let compiler = compiler.as_ref();

        let fragment = compiler.get_fragment(fragment_id).unwrap();

        Ok(xs::TopLevelElement::builder()
            .name(xs::Name(fragment.name.clone()))
            .maybe_type_(match &fragment.type_ {
                NamedOrAnonymous::Named(expanded_name) => {
                    Some(xs::Type(xs::QName(expanded_name.clone())))
                }
                NamedOrAnonymous::Anonymous(_) => None,
            })
            .maybe_type_choice(
                match &fragment.type_ {
                    NamedOrAnonymous::Anonymous(content_type) => Some(
                        xs::ElementTypeContent::from_complex_fragments(compiler, content_type),
                    ),
                    NamedOrAnonymous::Named(_) => None,
                }
                .transpose()?,
            )
            .build())
    }
}

impl ComplexFragmentEquivalent for xs::GroupRef {
    type FragmentId = FragmentIdx<GroupRefFragment>;

    fn to_complex_fragments<T: AsMut<ComplexTypeFragmentCompiler>>(
        &self,
        mut compiler: T,
    ) -> Self::FragmentId {
        let compiler = compiler.as_mut();

        compiler.push_fragment(GroupRefFragment {
            ref_: self.ref_.0 .0.clone(),
        })
    }

    fn from_complex_fragments<T: AsRef<ComplexTypeFragmentCompiler>>(
        compiler: T,
        fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        let compiler = compiler.as_ref();

        let fragment = compiler.get_fragment(fragment_id).unwrap();

        Ok(xs::GroupRef {
            id: None,
            ref_: xs::Ref(xs::QName(fragment.ref_.clone())),
            annotation: None,
        })
    }
}

impl ComplexFragmentEquivalent for xs::Any {
    type FragmentId = FragmentIdx<AnyFragment>;

    fn to_complex_fragments<T: AsMut<ComplexTypeFragmentCompiler>>(
        &self,
        mut compiler: T,
    ) -> Self::FragmentId {
        let _compiler = compiler.as_mut();

        todo!()
    }

    fn from_complex_fragments<T: AsRef<ComplexTypeFragmentCompiler>>(
        compiler: T,
        _fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        let _compiler = compiler.as_ref();

        todo!()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GroupTypeContentId {
    Element(FragmentIdx<LocalElementFragment>),
    Group(FragmentIdx<GroupRefFragment>),
    All(FragmentIdx<AllFragment>),
    Choice(FragmentIdx<ChoiceFragment>),
    Sequence(FragmentIdx<SequenceFragment>),
    Any(FragmentIdx<AnyFragment>),
}

impl From<TypeDefParticleId> for GroupTypeContentId {
    fn from(value: TypeDefParticleId) -> Self {
        match value {
            TypeDefParticleId::Group(fragment_idx) => Self::Group(fragment_idx),
            TypeDefParticleId::All(fragment_idx) => Self::All(fragment_idx),
            TypeDefParticleId::Choice(fragment_idx) => Self::Choice(fragment_idx),
            TypeDefParticleId::Sequence(fragment_idx) => Self::Sequence(fragment_idx),
        }
    }
}

impl From<FragmentIdx<LocalElementFragment>> for GroupTypeContentId {
    fn from(value: FragmentIdx<LocalElementFragment>) -> Self {
        Self::Element(value)
    }
}
impl From<FragmentIdx<GroupRefFragment>> for GroupTypeContentId {
    fn from(value: FragmentIdx<GroupRefFragment>) -> Self {
        Self::Group(value)
    }
}
impl From<FragmentIdx<AllFragment>> for GroupTypeContentId {
    fn from(value: FragmentIdx<AllFragment>) -> Self {
        Self::All(value)
    }
}
impl From<FragmentIdx<ChoiceFragment>> for GroupTypeContentId {
    fn from(value: FragmentIdx<ChoiceFragment>) -> Self {
        Self::Choice(value)
    }
}
impl From<FragmentIdx<SequenceFragment>> for GroupTypeContentId {
    fn from(value: FragmentIdx<SequenceFragment>) -> Self {
        Self::Sequence(value)
    }
}

impl From<FragmentIdx<AnyFragment>> for GroupTypeContentId {
    fn from(value: FragmentIdx<AnyFragment>) -> Self {
        Self::Any(value)
    }
}

impl ComplexFragmentEquivalent for xs::GroupTypeContent {
    type FragmentId = GroupTypeContentId;

    fn to_complex_fragments<T: AsMut<ComplexTypeFragmentCompiler>>(
        &self,
        mut compiler: T,
    ) -> Self::FragmentId {
        let compiler: &mut ComplexTypeFragmentCompiler = compiler.as_mut();

        match self {
            xs::GroupTypeContent::Element(local_element) => {
                local_element.to_complex_fragments(compiler).into()
            }
            xs::GroupTypeContent::Group(group_type) => {
                group_type.to_complex_fragments(compiler).into()
            }
            xs::GroupTypeContent::All(all_type) => all_type.to_complex_fragments(compiler).into(),
            xs::GroupTypeContent::Choice(choice_type) => {
                choice_type.to_complex_fragments(compiler).into()
            }
            xs::GroupTypeContent::Sequence(sequence_type) => {
                sequence_type.to_complex_fragments(compiler).into()
            }
            xs::GroupTypeContent::Any(any) => any.to_complex_fragments(compiler).into(),
        }
    }

    fn from_complex_fragments<T: AsRef<ComplexTypeFragmentCompiler>>(
        compiler: T,
        fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        let compiler = compiler.as_ref();
        match fragment_id {
            GroupTypeContentId::Element(fragment_idx) => {
                xs::LocalElement::from_complex_fragments(compiler, fragment_idx)
                    .map(GroupTypeContent::from)
            }
            GroupTypeContentId::Group(fragment_idx) => {
                xs::GroupRef::from_complex_fragments(compiler, fragment_idx)
                    .map(GroupTypeContent::from)
            }
            GroupTypeContentId::All(fragment_idx) => {
                xs::AllType::from_complex_fragments(compiler, fragment_idx)
                    .map(GroupTypeContent::from)
            }
            GroupTypeContentId::Choice(fragment_idx) => {
                xs::ChoiceType::from_complex_fragments(compiler, fragment_idx)
                    .map(GroupTypeContent::from)
            }
            GroupTypeContentId::Sequence(fragment_idx) => {
                xs::SequenceType::from_complex_fragments(compiler, fragment_idx)
                    .map(GroupTypeContent::from)
            }
            GroupTypeContentId::Any(_) => {
                unreachable!()
            }
        }
    }
}

impl ComplexFragmentEquivalent for xs::AllType {
    type FragmentId = FragmentIdx<AllFragment>;

    fn to_complex_fragments<T: AsMut<ComplexTypeFragmentCompiler>>(
        &self,
        mut compiler: T,
    ) -> Self::FragmentId {
        let mut compiler = compiler.as_mut();

        let all = AllFragment {
            min_occurs: self.min_occurs,
            max_occurs: self.max_occurs.map(|a| a.0),
            fragments: self
                .content
                .iter()
                .map(|content| content.to_complex_fragments(&mut compiler))
                .collect(),
        };

        compiler.push_fragment(all)
    }

    fn from_complex_fragments<T: AsRef<ComplexTypeFragmentCompiler>>(
        compiler: T,
        fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        let compiler = compiler.as_ref();

        todo!()
    }
}

impl ComplexFragmentEquivalent for xs::ChoiceType {
    type FragmentId = FragmentIdx<ChoiceFragment>;

    fn to_complex_fragments<T: AsMut<ComplexTypeFragmentCompiler>>(
        &self,
        mut compiler: T,
    ) -> Self::FragmentId {
        let mut compiler = compiler.as_mut();

        let all = ChoiceFragment {
            min_occurs: self.min_occurs,
            max_occurs: self.max_occurs.map(|a| a.0),
            fragments: self
                .content
                .iter()
                .map(|content| content.to_complex_fragments(&mut compiler))
                .collect(),
        };

        compiler.push_fragment(all)
    }

    fn from_complex_fragments<T: AsRef<ComplexTypeFragmentCompiler>>(
        compiler: T,
        fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        let compiler = compiler.as_ref();
        let fragment = compiler.get_fragment(fragment_id).unwrap();

        Ok(xs::ChoiceType::builder()
            .maybe_min_occurs(fragment.min_occurs)
            .maybe_max_occurs(fragment.max_occurs.map(xs::MaxOccurs))
            .content(
                fragment
                    .fragments
                    .iter()
                    .map(|fragment| {
                        xs::GroupTypeContent::from_complex_fragments(compiler, fragment)
                    })
                    .collect::<Result<_, _>>()?,
            )
            .build())
    }
}

impl ComplexFragmentEquivalent for xs::SequenceType {
    type FragmentId = FragmentIdx<SequenceFragment>;

    fn to_complex_fragments<T: AsMut<ComplexTypeFragmentCompiler>>(
        &self,
        mut compiler: T,
    ) -> Self::FragmentId {
        let mut compiler = compiler.as_mut();

        let all = SequenceFragment {
            min_occurs: self.min_occurs,
            max_occurs: self.max_occurs.map(|a| a.0),
            fragments: self
                .content
                .iter()
                .map(|content| content.to_complex_fragments(&mut compiler))
                .collect(),
        };

        compiler.push_fragment(all)
    }

    fn from_complex_fragments<T: AsRef<ComplexTypeFragmentCompiler>>(
        compiler: T,
        fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        let compiler = compiler.as_ref();
        let fragment = compiler.get_fragment(fragment_id).unwrap();

        Ok(xs::SequenceType {
            id: None,
            name: None,
            ref_: None,
            min_occurs: fragment.min_occurs,
            max_occurs: fragment.max_occurs.map(xs::MaxOccurs),
            content: fragment
                .fragments
                .iter()
                .map(|fragment| xs::GroupTypeContent::from_complex_fragments(compiler, fragment))
                .collect::<Result<_, _>>()?,
        })
    }
}

impl ComplexFragmentEquivalent for xs::TypeDefParticle {
    type FragmentId = TypeDefParticleId;

    fn to_complex_fragments<T: AsMut<ComplexTypeFragmentCompiler>>(
        &self,
        mut compiler: T,
    ) -> Self::FragmentId {
        let compiler = compiler.as_mut();

        match self {
            xs::TypeDefParticle::Group(group_ref) => {
                let ref_ = group_ref.ref_.0 .0.clone();

                compiler.push_fragment(GroupRefFragment { ref_ }).into()
            }
            xs::TypeDefParticle::All(all) => all.to_complex_fragments(compiler).into(),
            xs::TypeDefParticle::Choice(choice) => choice.to_complex_fragments(compiler).into(),
            xs::TypeDefParticle::Sequence(sequence) => {
                sequence.to_complex_fragments(compiler).into()
            }
        }
    }

    fn from_complex_fragments<T: AsRef<ComplexTypeFragmentCompiler>>(
        compiler: T,
        fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        match fragment_id {
            TypeDefParticleId::Group(group_ref) => {
                let group_ref = xs::GroupRef::from_complex_fragments(compiler, group_ref)?;
                Ok(xs::TypeDefParticle::Group(group_ref))
            }
            TypeDefParticleId::All(all) => {
                let all = xs::AllType::from_complex_fragments(compiler, all)?;
                Ok(xs::TypeDefParticle::All(all))
            }
            TypeDefParticleId::Choice(choice) => {
                let choice = xs::ChoiceType::from_complex_fragments(compiler, choice)?;
                Ok(xs::TypeDefParticle::Choice(choice))
            }
            TypeDefParticleId::Sequence(sequence) => {
                let sequence = xs::SequenceType::from_complex_fragments(compiler, sequence)?;
                Ok(xs::TypeDefParticle::Sequence(sequence))
            }
        }
    }
}

impl ComplexFragmentEquivalent for xs::ExtensionType {
    type FragmentId = FragmentIdx<ExtensionFragment>;

    /// This method expects all references to already be defined.
    fn to_complex_fragments<T: AsMut<ComplexTypeFragmentCompiler>>(
        &self,
        mut compiler: T,
    ) -> Self::FragmentId {
        let mut compiler = compiler.as_mut();

        let base = self.base.0 .0.clone();

        let content_fragment = self
            .particle
            .as_ref()
            .map(|content| content.to_complex_fragments(&mut compiler));

        let attribute_declarations = self
            .attributes
            .iter()
            .map(|attribute| attribute.to_complex_fragments(&mut compiler))
            .collect();

        let root_fragment = ExtensionFragment {
            base,
            content_fragment,
            attribute_declarations,
        };

        compiler.push_fragment(root_fragment)
    }

    fn from_complex_fragments<T: AsRef<ComplexTypeFragmentCompiler>>(
        compiler: T,
        fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        let compiler = compiler.as_ref();

        let extension = compiler.get_fragment(fragment_id).unwrap();

        let particle = extension
            .content_fragment
            .as_ref()
            .map(|fragment_id| xs::TypeDefParticle::from_complex_fragments(compiler, fragment_id))
            .transpose()?;

        let attributes = extension
            .attribute_declarations
            .iter()
            .map(|a| xs::AttributeDeclaration::from_complex_fragments(compiler, a))
            .collect::<Result<_, _>>()?;

        Ok(Self {
            base: xs::Base(xs::QName(extension.base.clone())),
            particle,
            id: None,
            annotation: None,
            open_content: None,
            attributes,
        })
    }
}

impl ComplexFragmentEquivalent for xs::ComplexRestrictionType {
    type FragmentId = FragmentIdx<RestrictionFragment>;

    fn to_complex_fragments<T: AsMut<ComplexTypeFragmentCompiler>>(
        &self,
        mut compiler: T,
    ) -> Self::FragmentId {
        let mut compiler = compiler.as_mut();

        let base = self.base.0 .0.clone();

        let content_fragment = self
            .particle
            .as_ref()
            .map(|particle| particle.to_complex_fragments(&mut compiler));

        let attribute_declarations = self
            .attributes
            .iter()
            .map(|attribute| attribute.to_complex_fragments(&mut compiler))
            .collect();

        let root_fragment = RestrictionFragment {
            base,
            content_fragment,
            attribute_declarations,
        };

        compiler.push_fragment(root_fragment)
    }

    fn from_complex_fragments<T: AsRef<ComplexTypeFragmentCompiler>>(
        compiler: T,
        fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        let compiler = compiler.as_ref();

        let fragment = compiler.get_fragment(fragment_id).unwrap();

        let particle = fragment
            .content_fragment
            .map(|a| xs::TypeDefParticle::from_complex_fragments(compiler, &a))
            .transpose()?;

        let attributes = fragment
            .attribute_declarations
            .iter()
            .map(|a| xs::AttributeDeclaration::from_complex_fragments(compiler, a))
            .collect::<Result<_, _>>()?;

        Ok(xs::ComplexRestrictionType {
            annotation: None,
            base: xs::Base(xs::QName(fragment.base.clone())),
            id: None,
            simple_type: None,
            open_content: None,
            particle,
            attributes,
        })
    }
}

impl ComplexFragmentEquivalent for xs::AttributeDeclaration {
    type FragmentId = AttributeDeclarationId;

    fn to_complex_fragments<T: AsMut<ComplexTypeFragmentCompiler>>(
        &self,
        mut compiler: T,
    ) -> Self::FragmentId {
        let compiler = compiler.as_mut();

        match self {
            xs::AttributeDeclaration::Attribute(local) => {
                AttributeDeclarationId::Attribute(local.to_complex_fragments(compiler))
            }
            xs::AttributeDeclaration::AttributeGroup(group) => {
                AttributeDeclarationId::AttributeGroupRef(group.to_complex_fragments(compiler))
            }
        }
    }

    fn from_complex_fragments<T: AsRef<ComplexTypeFragmentCompiler>>(
        compiler: T,
        fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        let compiler = compiler.as_ref();

        match fragment_id {
            AttributeDeclarationId::Attribute(fragment_idx) => {
                xs::LocalAttribute::from_complex_fragments(compiler, fragment_idx)
                    .map(xs::AttributeDeclaration::Attribute)
            }
            AttributeDeclarationId::AttributeGroupRef(fragment_idx) => {
                xs::AttributeGroupRefType::from_complex_fragments(compiler, fragment_idx)
                    .map(xs::AttributeDeclaration::AttributeGroup)
            }
        }
    }
}

impl ComplexFragmentEquivalent for xs::LocalSimpleType {
    type FragmentId = simple::FragmentId;

    fn to_complex_fragments<T: AsMut<ComplexTypeFragmentCompiler>>(
        &self,
        compiler: T,
    ) -> Self::FragmentId {
        todo!()
    }

    fn from_complex_fragments<T: AsRef<ComplexTypeFragmentCompiler>>(
        compiler: T,
        fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        todo!()
    }
}

impl ComplexFragmentEquivalent for xs::LocalAttribute {
    type FragmentId = FragmentIdx<LocalAttributeFragment>;

    fn to_complex_fragments<T: AsMut<ComplexTypeFragmentCompiler>>(
        &self,
        mut compiler: T,
    ) -> Self::FragmentId {
        let mut compiler = compiler.as_mut();

        let use_ = self.use_.as_ref().map(|a| match a.0 {
            xs::AttributeUseType::Prohibited => AttributeUse::Prohibited,
            xs::AttributeUseType::Optional => AttributeUse::Optional,
            xs::AttributeUseType::Required => AttributeUse::Required,
        });

        let type_mode = if let Some(ref ref_) = self.ref_ {
            LocalAttributeFragmentTypeMode::Reference(ReferenceAttributeFragment {
                name: ref_.0 .0.clone(),
            })
        } else {
            let name = self
                .name
                .as_ref()
                .expect("name is required if not a reference");

            let type_ = if let Some(type_) = self.type_.as_ref() {
                Some(NamedOrAnonymous::Named(type_.0 .0.clone()))
            } else if let Some(simple_type) = self.simple_type.as_ref() {
                Some(NamedOrAnonymous::Anonymous(
                    simple_type.to_complex_fragments(&mut compiler),
                ))
            } else {
                None
            };

            LocalAttributeFragmentTypeMode::Declared(DeclaredAttributeFragment {
                name: name.0.clone(),
                type_,
            })
        };
        compiler.push_fragment(LocalAttributeFragment { type_mode, use_ })
    }

    fn from_complex_fragments<T: AsRef<ComplexTypeFragmentCompiler>>(
        compiler: T,
        fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        let compiler = compiler.as_ref();

        let fragment = compiler.get_fragment(fragment_id).unwrap();

        match &fragment.type_mode {
            LocalAttributeFragmentTypeMode::Declared(local) => {
                let name = local.name.clone();
                let type_ = local.type_.as_ref().unwrap();
                let type_ = match type_ {
                    NamedOrAnonymous::Named(ref_) => Some(xs::QName(ref_.clone())),
                    NamedOrAnonymous::Anonymous(_) => None,
                };
                let use_ = fragment.use_.map(|a| match a {
                    AttributeUse::Required => xs::AttributeUseType::Required,
                    AttributeUse::Optional => xs::AttributeUseType::Optional,
                    AttributeUse::Prohibited => xs::AttributeUseType::Prohibited,
                });
                Ok(xs::LocalAttribute::builder()
                    .name(xs::Name(name))
                    .maybe_type_(type_.map(xs::Type))
                    .maybe_use_(use_.map(xs::AttrUse))
                    .build())
            }
            _ => todo!(),
        }
    }
}

impl ComplexFragmentEquivalent for xs::TopLevelAttribute {
    type FragmentId = FragmentIdx<TopLevelAttributeFragment>;

    fn to_complex_fragments<T: AsMut<ComplexTypeFragmentCompiler>>(
        &self,
        mut compiler: T,
    ) -> Self::FragmentId {
        let mut compiler = compiler.as_mut();

        let use_ = self.use_.as_ref().map(|a| match a.0 {
            xs::AttributeUseType::Prohibited => AttributeUse::Prohibited,
            xs::AttributeUseType::Optional => AttributeUse::Optional,
            xs::AttributeUseType::Required => AttributeUse::Required,
        });

        let name = self.name.0.clone();

        let type_ = if let Some(type_) = self.type_.as_ref() {
            Some(NamedOrAnonymous::Named(type_.0 .0.clone()))
        } else if let Some(simple_type) = self.simple_type.as_ref() {
            Some(NamedOrAnonymous::Anonymous(
                simple_type.to_complex_fragments(&mut compiler),
            ))
        } else {
            None
        };

        compiler.push_fragment(TopLevelAttributeFragment { name, type_, use_ })
    }

    fn from_complex_fragments<T: AsRef<ComplexTypeFragmentCompiler>>(
        compiler: T,
        fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        let compiler = compiler.as_ref();

        let fragment = compiler.get_fragment(fragment_id).unwrap();

        let name = fragment.name.clone();
        let type_ = fragment.type_.as_ref().unwrap();
        let type_ = match type_ {
            NamedOrAnonymous::Named(ref_) => Some(xs::QName(ref_.clone())),
            NamedOrAnonymous::Anonymous(_) => None,
        };
        let use_ = fragment.use_.map(|a| match a {
            AttributeUse::Required => xs::AttributeUseType::Required,
            AttributeUse::Optional => xs::AttributeUseType::Optional,
            AttributeUse::Prohibited => xs::AttributeUseType::Prohibited,
        });
        Ok(xs::TopLevelAttribute::builder()
            .name(xs::Name(name))
            .maybe_type_(type_.map(xs::Type))
            .maybe_use_(use_.map(xs::AttrUse))
            .build())
    }
}

impl ComplexFragmentEquivalent for xs::AttributeGroupRefType {
    type FragmentId = FragmentIdx<AttributeGroupRefFragment>;

    fn to_complex_fragments<T: AsMut<ComplexTypeFragmentCompiler>>(
        &self,
        mut compiler: T,
    ) -> Self::FragmentId {
        let compiler = compiler.as_mut();

        todo!()
    }

    fn from_complex_fragments<T: AsRef<ComplexTypeFragmentCompiler>>(
        compiler: T,
        fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        let compiler = compiler.as_ref();

        todo!()
    }
}

impl ComplexFragmentEquivalent for xs::ComplexContent {
    type FragmentId = FragmentIdx<ComplexContentFragment>;

    /// This method expects all references to already be defined.
    fn to_complex_fragments<T: AsMut<ComplexTypeFragmentCompiler>>(
        &self,
        mut compiler: T,
    ) -> Self::FragmentId {
        let mut compiler = compiler.as_mut();

        let content_fragment = match &self.content {
            xs::ComplexContentContent::Extension(extension) => {
                let fragment_id = extension.to_complex_fragments(&mut compiler);

                ComplexContentChildId::Extension(fragment_id)
            }
            xs::ComplexContentContent::Restriction(restriction) => {
                let fragment_id = restriction.to_complex_fragments(&mut compiler);

                ComplexContentChildId::Restriction(fragment_id)
            }
        };

        compiler.push_fragment(ComplexContentFragment {
            content_fragment,
            mixed: self.mixed.as_ref().map(|a| a.0),
        })
    }

    fn from_complex_fragments<T: AsRef<ComplexTypeFragmentCompiler>>(
        compiler: T,
        fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        let compiler = compiler.as_ref();

        let fragment = compiler.get_fragment(fragment_id).unwrap();
        let content = match &fragment.content_fragment {
            ComplexContentChildId::Extension(fragment_id) => {
                xs::ExtensionType::from_complex_fragments(compiler, fragment_id)?.into()
            }
            ComplexContentChildId::Restriction(fragment_id) => {
                xs::ComplexRestrictionType::from_complex_fragments(compiler, fragment_id)?.into()
            }
        };

        Ok(xs::ComplexContent {
            annotation: None,
            id: None,
            mixed: fragment.mixed.map(|a| xs::Mixed(a)),
            content,
        })
    }
}

impl ComplexFragmentEquivalent for xs::ComplexTypeModel {
    type FragmentId = ComplexTypeModelId;

    /// This method expects all references to already be defined.
    fn to_complex_fragments<T: AsMut<ComplexTypeFragmentCompiler>>(
        &self,
        mut compiler: T,
    ) -> Self::FragmentId {
        let compiler = compiler.as_mut();

        match self {
            xs::ComplexTypeModel::SimpleContent(_simple_content) => {
                todo!()
            }
            xs::ComplexTypeModel::ComplexContent(complex_content) => {
                ComplexTypeModelId::ComplexContent(complex_content.to_complex_fragments(compiler))
            }
            xs::ComplexTypeModel::Other {
                open_content,
                type_def_particle,
            } => {
                //TODO: Review open content
                let type_def_particle = type_def_particle.as_ref().unwrap();

                let particle = type_def_particle.to_complex_fragments(compiler);

                ComplexTypeModelId::Other { particle }
            }
        }
    }

    fn from_complex_fragments<T: AsRef<ComplexTypeFragmentCompiler>>(
        compiler: T,
        fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        let compiler = compiler.as_ref();

        match fragment_id {
            ComplexTypeModelId::SimpleContent(fragment_idx) => {
                // xs::SimpleContent::from_complex_fragments(compiler, fragment_idx)
                //     .map(xs::ComplexTypeModel::SimpleContent)
                todo!()
            }
            ComplexTypeModelId::ComplexContent(fragment_idx) => {
                xs::ComplexContent::from_complex_fragments(compiler, fragment_idx)
                    .map(xs::ComplexTypeModel::ComplexContent)
            }
            ComplexTypeModelId::Other { particle } => {
                let type_def_particle =
                    xs::TypeDefParticle::from_complex_fragments(compiler, particle)?;
                Ok(xs::ComplexTypeModel::Other {
                    type_def_particle: Some(type_def_particle),
                    open_content: None,
                })
            }
        }
    }
}

impl ComplexFragmentEquivalent for xs::TopLevelComplexType {
    type FragmentId = FragmentIdx<ComplexTypeRootFragment>;

    fn to_complex_fragments<T: AsMut<ComplexTypeFragmentCompiler>>(
        &self,
        mut compiler: T,
    ) -> Self::FragmentId {
        let mut compiler = compiler.as_mut();

        let content = self.content.to_complex_fragments(&mut compiler);

        let fragment = ComplexTypeRootFragment {
            name: Some(self.name.clone()),
            content,
        };

        compiler.push_fragment(fragment)
    }

    fn from_complex_fragments<T: AsRef<ComplexTypeFragmentCompiler>>(
        compiler: T,
        fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        let compiler = compiler.as_ref();

        let complex_type_root_fragment = compiler.get_fragment(fragment_id).unwrap();

        Ok(Self {
            id: None,
            name: complex_type_root_fragment
                .name
                .clone()
                .ok_or_else(|| todo!())?,
            mixed: None,
            abstract_: None,
            final_: None,
            block: None,
            default_attributes_apply: None,
            annotation: None,
            content: xs::ComplexTypeModel::from_complex_fragments(
                compiler,
                &complex_type_root_fragment.content,
            )?,
        })
    }
}

impl ComplexFragmentEquivalent for xs::LocalComplexType {
    type FragmentId = FragmentIdx<ComplexTypeRootFragment>;

    fn to_complex_fragments<T: AsMut<ComplexTypeFragmentCompiler>>(
        &self,
        mut compiler: T,
    ) -> Self::FragmentId {
        let mut compiler = compiler.as_mut();

        let content = self.content.to_complex_fragments(&mut compiler);

        let fragment = ComplexTypeRootFragment {
            name: None,
            content,
        };

        compiler.push_fragment(fragment)
    }

    fn from_complex_fragments<T: AsRef<ComplexTypeFragmentCompiler>>(
        compiler: T,
        fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        todo!()
    }
}

// impl ComplexFragmentEquivalent for xs::NamedGroupTypeContent {
//     type FragmentId = todo!();

//     fn to_complex_fragments<T: AsMut<ComplexTypeFragmentCompiler>>(
//         &self,
//         mut compiler: T,
//     ) -> Self::FragmentId {
//         let compiler = compiler.as_mut();

//         match self {
//             xs::NamedGroupTypeContent::All(all_type) => all_type.to_complex_fragments(compiler),
//             xs::NamedGroupTypeContent::Choice(choice_type) => {
//                 choice_type.to_complex_fragments(compiler)
//             }
//             xs::NamedGroupTypeContent::Sequence(sequence_type) => {
//                 sequence_type.to_complex_fragments(compiler)
//             }
//         }
//     }
// }

pub static ANY_TYPE_EXPANDED_NAME: LazyLock<ExpandedName<'static>> = LazyLock::new(|| {
    ExpandedName::new(
        LocalName::new("anyType").unwrap(),
        Some(XmlNamespace::XMLNS),
    )
});

#[cfg(test)]
mod tests {

    use super::*;

    /// <xs:element name="complexContent" id="complexContent">
    ///     <xs:annotation>
    ///       <xs:documentation
    ///            source="../structures/structures.html#element-complexContent"/>
    ///     </xs:annotation>
    ///     <xs:complexType>
    ///       <xs:complexContent>
    ///         <xs:extension base="xs:annotated">
    ///           <xs:choice>
    ///             <xs:element name="restriction" type="xs:complexRestrictionType"/>
    ///             <xs:element name="extension" type="xs:extensionType"/>
    ///           </xs:choice>
    ///           <xs:attribute name="mixed" type="xs:boolean">
    ///             <xs:annotation>
    ///               <xs:documentation>
    ///        Overrides any setting on complexType parent.</xs:documentation>
    ///             </xs:annotation>
    ///           </xs:attribute>
    ///         </xs:extension>
    ///       </xs:complexContent>
    ///     </xs:complexType>
    ///   </xs:element>
    ///
    /// This should become a top level element with a reference to an anonymous complex type
    struct _Todo;

    /// <xs:complexType>
    ///   <xs:complexContent>
    ///     <xs:extension base="xs:annotated">
    ///       <xs:choice>
    ///         <xs:element name="restriction" type="xs:complexRestrictionType"/>
    ///         <xs:element name="extension" type="xs:extensionType"/>
    ///       </xs:choice>
    ///       <xs:attribute name="mixed" type="xs:boolean">
    ///         <xs:annotation>
    ///           <xs:documentation>
    ///             Overrides any setting on complexType parent.
    ///           </xs:documentation>
    ///         </xs:annotation>
    ///       </xs:attribute>
    ///     </xs:extension>
    ///   </xs:complexContent>
    /// </xs:complexType>
    struct _Todo2;

    // //Flatten nested choices
    // #[test]
    // fn flatten_nested_choices() {
    //     let mut fragments = ComplexTypeFragmentCollection::new(
    //         XmlNamespace::new_dangerous("http://localhost"),
    //         SimpleTypeFragmentCollection {},
    //     );

    //     let element1 = ComplexTypeFragment::Element(ElementTypeFragment::Reference {
    //         name: ExpandedName::new(
    //             LocalName::new("element1").unwrap(),
    //             Some(XmlNamespace::XMLNS),
    //         ),
    //         min_occurs: Some(MinOccurs(1)),
    //         max_occurs: None,
    //     });
    //     let element1 = fragments.push_fragment(element1);

    //     let element2 = ComplexTypeFragment::Element(ElementTypeFragment::Reference {
    //         name: ExpandedName::new(
    //             LocalName::new("element2").unwrap(),
    //             Some(XmlNamespace::XMLNS),
    //         ),
    //         min_occurs: Some(MinOccurs(1)),
    //         max_occurs: None,
    //     });
    //     let element2 = fragments.push_fragment(element2);

    //     let choice1 = ComplexTypeFragment::Choice {
    //         fragments: vec_deque![element1.clone()],
    //     };
    //     let choice1 = fragments.push_fragment(choice1);

    //     let choice2 = ComplexTypeFragment::Choice {
    //         fragments: vec_deque![choice1, element2.clone()],
    //     };
    //     let choice2 = fragments.push_fragment(choice2);

    //     FlattenNestedChoices.transform(&mut fragments);

    //     let expected = ComplexTypeFragment::Choice {
    //         fragments: vec_deque![element1, element2],
    //     };

    //     let actual = fragments.get_fragment(&choice2).unwrap();

    //     assert_eq!(*actual, expected);
    // }

    // // Flatten nested sequences
    // // This can only be done if the sequence does not by itself have a minOccurs or maxOccurs that would affect how the fragments are nested
    // #[test]
    // fn flatten_nested_sequences() {
    //     let mut fragments = ComplexTypeFragmentCollection::new(
    //         XmlNamespace::new_dangerous("http://localhost"),
    //         SimpleTypeFragmentCollection {},
    //     );

    //     let element1 = ComplexTypeFragment::Element(ElementTypeFragment::Reference {
    //         name: ExpandedName::new(
    //             LocalName::new("element1").unwrap(),
    //             Some(XmlNamespace::XMLNS),
    //         ),
    //         min_occurs: Some(MinOccurs(1)),
    //         max_occurs: None,
    //     });
    //     let element1 = fragments.push_fragment(element1);
    //     let element2 = ComplexTypeFragment::Element(ElementTypeFragment::Reference {
    //         name: ExpandedName::new(
    //             LocalName::new("element2").unwrap(),
    //             Some(XmlNamespace::XMLNS),
    //         ),
    //         min_occurs: Some(MinOccurs(1)),
    //         max_occurs: None,
    //     });
    //     let element2 = fragments.push_fragment(element2);
    //     let sequence1 = ComplexTypeFragment::Sequence {
    //         fragments: vec_deque![element1.clone()],
    //     };
    //     let sequence1 = fragments.push_fragment(sequence1);
    //     let sequence2 = ComplexTypeFragment::Sequence {
    //         fragments: vec_deque![sequence1, element2.clone()],
    //     };
    //     let sequence2 = fragments.push_fragment(sequence2);

    //     FlattenNestedSequences.transform(&mut fragments);

    //     let actual = fragments.get_fragment(&sequence2).unwrap();
    //     let expected = ComplexTypeFragment::Sequence {
    //         fragments: vec_deque![element1, element2],
    //     };

    //     assert_eq!(*actual, expected);
    // }

    // // Expand extensions
    // #[test]
    // fn expand_extensions() {
    //     let mut fragments = ComplexTypeFragmentCollection::new(
    //         XmlNamespace::new_dangerous("http://localhost"),
    //         SimpleTypeFragmentCollection {},
    //     );

    //     let element1 = ComplexTypeFragment::Element(ElementTypeFragment::Reference {
    //         name: ExpandedName::new(
    //             LocalName::new("element1").unwrap(),
    //             Some(XmlNamespace::XMLNS),
    //         ),
    //         min_occurs: Some(MinOccurs(1)),
    //         max_occurs: None,
    //     });

    //     let element1 = fragments.push_fragment(element1);

    //     let element2 = ComplexTypeFragment::Element(ElementTypeFragment::Reference {
    //         name: ExpandedName::new(
    //             LocalName::new("element2").unwrap(),
    //             Some(XmlNamespace::XMLNS),
    //         ),
    //         min_occurs: Some(MinOccurs(1)),
    //         max_occurs: None,
    //     });

    //     let element2 = fragments.push_fragment(element2);

    //     let base = ComplexTypeFragment::ComplexContent(ComplexContentFragment::Restriction {
    //         base: TypeIdent(TypeIdent::External(any_type_expanded_name())),
    //         content_fragment: vec_deque![element1.clone()],
    //     });

    //     let base_type_ident = TypeIdent(TypeIdent::Local(LocalTypeIdent::Named(
    //         LocalName::new("base").unwrap(),
    //     )));
    //     let base = fragments.push_fragment(base);
    //     fragments
    //         .fragment_names
    //         .insert(base_type_ident.clone(), base);

    //     let derivative = ComplexTypeFragment::ComplexContent(ComplexContentFragment::Extension {
    //         base: base_type_ident,
    //         content_fragment: vec_deque![element2.clone()],
    //     });

    //     let derivative = fragments.push_fragment(derivative);

    //     ExpandExtensions.transform(&mut fragments);

    //     let expected = ComplexTypeFragment::ComplexContent(ComplexContentFragment::Restriction {
    //         base: TypeIdent(TypeIdent::External(any_type_expanded_name())),
    //         content_fragment: vec_deque![element1, element2],
    //     });

    //     let actual = fragments.get_fragment(&derivative).unwrap();

    //     assert_eq!(*actual, expected);
    // }

    /// # Chain of extensions and restrictions test
    ///
    /// PS: This is not a direct copy from XMLSchema. "annotated" is not a restriction of "anyType" but an extension of "openAttrs",
    /// however for this example we've deleted all attributes and changed the base to "anyType" to make it a restriction.
    /// Groups ("identityConstraint") have also been removed.
    ///
    /// ## "annotated"
    /// ```xml
    /// <xs:complexType name="annotated">
    ///     <xs:complexContent>
    ///         <xs:restriction base="xs:anyType">
    ///             <xs:sequence>
    ///                 <xs:element ref="xs:annotation" minOccurs="0"/>
    ///             </xs:sequence>
    ///         </xs:restriction>
    ///     </xs:complexContent>
    /// </xs:complexType>
    /// ```
    ///
    /// ## "element"
    /// ```xml
    /// <xs:complexType name="element" abstract="true">
    ///     <xs:complexContent>
    ///         <xs:extension base="xs:annotated">
    ///             <xs:sequence>
    ///                 <xs:choice minOccurs="0">
    ///                     <xs:element name="simpleType" type="xs:localSimpleType"/>
    ///                     <xs:element name="complexType" type="xs:localComplexType"/>
    ///                 </xs:choice>
    ///                 <xs:element name="alternative" type="xs:altType"
    ///                         minOccurs="0" maxOccurs="unbounded"/>
    ///             </xs:sequence>
    ///         </xs:extension>
    ///     </xs:complexContent>
    /// </xs:complexType>
    /// ```
    ///
    /// ## "topLevelElement"
    /// ```xml
    /// <xs:complexType name="topLevelElement">
    ///     <xs:complexContent>
    ///         <xs:restriction base="xs:element">
    ///             <xs:sequence>
    ///                 <xs:element ref="xs:annotation" minOccurs="0"/>
    ///                 <xs:choice minOccurs="0">
    ///                     <xs:element name="simpleType" type="xs:localSimpleType"/>
    ///                     <xs:element name="complexType" type="xs:localComplexType"/>
    ///                 </xs:choice>
    ///                 <xs:element name="alternative" type="xs:altType"
    ///                     minOccurs="0" maxOccurs="unbounded"/>
    ///             </xs:sequence>
    ///         </xs:restriction>
    ///     </xs:complexContent>
    /// </xs:complexType>
    /// ```
    // #[test]
    // fn test_expand_extensions_restrictions() {
    //     let mut fragments = ComplexTypeFragmentCollection::new(
    //         XmlNamespace::new_dangerous("http://localhost"),
    //         SimpleTypeFragmentCollection {},
    //     );

    //     let annotation_element_name = ExpandedName::new(
    //         LocalName::new("annotation").unwrap(),
    //         Some(XmlNamespace::XMLNS),
    //     );
    //     let annotation_element = ComplexTypeFragment::Element(ElementTypeFragment::Reference {
    //         name: annotation_element_name.clone(),
    //         min_occurs: Some(MinOccurs(0)),
    //         max_occurs: None,
    //     });

    //     let annotation_element = fragments.push_fragment(annotation_element);

    //     let annotated_name =
    //         TypeIdent(LocalTypeIdent::Named(LocalName::new("annotated").unwrap()).into());
    //     let annotated = ComplexTypeFragment::ComplexContent(ComplexContentFragment::Restriction {
    //         base: TypeIdent(TypeIdent::External(any_type_expanded_name())),
    //         content_fragment: vec_deque![annotation_element,],
    //     });

    //     let annotated = fragments.push_fragment(annotated);
    //     fragments.fragment_names.insert(annotated_name, annotated);

    //     let simple_type_element_type_name =
    //         TypeIdent(TypeIdent::External(ExpandedName::new(
    //             LocalName::new("localSimpleType").unwrap(),
    //             Some(XmlNamespace::XMLNS),
    //         )));
    //     let simple_type_element_type =
    //         ComplexTypeFragment::ComplexContent(ComplexContentFragment::Restriction {
    //             base: TypeIdent(TypeIdent::External(any_type_expanded_name())),
    //             content_fragment: vec_deque![],
    //         });

    //     let simple_type_element_type = fragments.push_fragment(simple_type_element_type);
    //     fragments
    //         .fragment_names
    //         .insert(simple_type_element_type_name, simple_type_element_type);

    //     let simple_type_element_type_name =
    //         TypeIdent(TypeIdent::External(ExpandedName::new(
    //             LocalName::new("localComplexType").unwrap(),
    //             Some(XmlNamespace::XMLNS),
    //         )));
    //     let simple_type_element_type =
    //         ComplexTypeFragment::ComplexContent(ComplexContentFragment::Restriction {
    //             base: TypeIdent(TypeIdent::External(any_type_expanded_name())),
    //             content_fragment: vec_deque![],
    //         });

    //     let simple_type_element_type = fragments.push_fragment(simple_type_element_type);
    //     fragments
    //         .fragment_names
    //         .insert(simple_type_element_type_name, simple_type_element_type);

    //     let simple_type_element_type_name =
    //         TypeIdent(TypeIdent::External(ExpandedName::new(
    //             LocalName::new("altType").unwrap(),
    //             Some(XmlNamespace::XMLNS),
    //         )));
    //     let simple_type_element_type =
    //         ComplexTypeFragment::ComplexContent(ComplexContentFragment::Restriction {
    //             base: TypeIdent(TypeIdent::External(any_type_expanded_name())),
    //             content_fragment: vec_deque![],
    //         });

    //     let simple_type_element_type = fragments.push_fragment(simple_type_element_type);
    //     fragments
    //         .fragment_names
    //         .insert(simple_type_element_type_name, simple_type_element_type);

    //     let element = ComplexTypeFragment::ComplexContent(ComplexContentFragment::Extension {
    //         base: TypeIdent(TypeIdent::External(annotation_element_name)),
    //         content_fragment: vec_deque![],
    //     });
    // }

    #[test]
    fn convert_annotated_to_fragments() {
        let namespace = XmlNamespace::new_dangerous("http://localhost");

        let simple_type_compiler = SimpleTypeFragmentCompiler::new(namespace.clone());
        let mut fragment_compiler =
            ComplexTypeFragmentCompiler::new(namespace.clone(), simple_type_compiler);

        let id = xs::TopLevelComplexType::builder()
            .name(LocalName::new_dangerous("annotated"))
            .content(
                xs::ComplexContent::builder()
                    .content(
                        xs::ComplexRestrictionType::builder()
                            .base(xs::Base(xs::QName(ANY_TYPE_EXPANDED_NAME.clone())))
                            .particle(
                                xs::SequenceType::builder()
                                    .content(vec![xs::LocalElement::builder()
                                        .ref_(xs::Ref(xs::QName(ExpandedName::new(
                                            LocalName::new_dangerous("annotation"),
                                            Some(XmlNamespace::XMLNS),
                                        ))))
                                        .min_occurs(xs::MinOccurs(0))
                                        .build()
                                        .into()])
                                    .build()
                                    .into(),
                            )
                            .build()
                            .into(),
                    )
                    .build()
                    .into(),
            )
            .build()
            .to_complex_fragments(&mut fragment_compiler);

        assert_eq!(id, FragmentIdx::new(0));
        assert_eq!(fragment_compiler.complex_types.len(), 1);
        assert_eq!(fragment_compiler.elements.len(), 1);
        assert_eq!(fragment_compiler.sequences.len(), 1);
        assert_eq!(fragment_compiler.complex_contents.len(), 1);

        println!("{:#?}", fragment_compiler);
    }

    #[test]
    fn convert_element_to_fragments() {
        let namespace = XmlNamespace::new_dangerous("http://localhost");

        let simple_type_compiler = SimpleTypeFragmentCompiler::new(namespace.clone());
        let mut fragment_compiler =
            ComplexTypeFragmentCompiler::new(namespace.clone(), simple_type_compiler);

        let annotated_name = LocalName::new_dangerous("annotated");
        let annotated_expanded_name = ExpandedName::new(
            annotated_name.clone(),
            Some(fragment_compiler.namespace.clone()),
        );

        let annotated = xs::TopLevelComplexType::builder()
            .name(annotated_name)
            .content(
                xs::ComplexContent::builder()
                    .content(
                        xs::ComplexRestrictionType::builder()
                            .base(xs::Base(xs::QName(ANY_TYPE_EXPANDED_NAME.clone())))
                            .particle(
                                xs::SequenceType::builder()
                                    .content(vec![xs::LocalElement::builder()
                                        .ref_(xs::Ref(xs::QName(ExpandedName::new(
                                            LocalName::new_dangerous("annotation"),
                                            Some(XmlNamespace::XMLNS),
                                        ))))
                                        .min_occurs(xs::MinOccurs(0))
                                        .build()
                                        .into()])
                                    .build()
                                    .into(),
                            )
                            .build()
                            .into(),
                    )
                    .build()
                    .into(),
            )
            .build()
            .to_complex_fragments(&mut fragment_compiler);

        assert_eq!(annotated, FragmentIdx::new(0));
        assert_eq!(fragment_compiler.complex_types.len(), 1);
        assert_eq!(fragment_compiler.elements.len(), 1);
        assert_eq!(fragment_compiler.sequences.len(), 1);
        assert_eq!(fragment_compiler.complex_contents.len(), 1);

        // ## "element"
        // ```xml
        // <xs:complexType name="element" abstract="true">
        //     <xs:complexContent>
        //         <xs:extension base="xs:annotated">
        //             <xs:sequence>
        //                 <xs:choice minOccurs="0">
        //                     <xs:element name="simpleType" type="xs:localSimpleType"/>
        //                     <xs:element name="complexType" type="xs:localComplexType"/>
        //                 </xs:choice>
        //                 <xs:element name="alternative" type="xs:altType"
        //                         minOccurs="0" maxOccurs="unbounded"/>
        //             </xs:sequence>
        //         </xs:extension>
        //     </xs:complexContent>
        // </xs:complexType>
        // ```
        let element_id = xs::TopLevelComplexType::builder()
            .name(LocalName::new_dangerous("element"))
            .content(
                xs::ComplexContent::builder()
                    .content(
                        xs::ExtensionType::builder()
                            .base(xs::Base(xs::QName(annotated_expanded_name)))
                            .particle(
                                xs::SequenceType::builder()
                                    .content(vec![
                                        xs::ChoiceType::builder()
                                            .min_occurs(xs::MinOccurs(0))
                                            .content(vec![
                                                xs::LocalElement::builder()
                                                    .name(xs::Name(LocalName::new_dangerous(
                                                        "simpleType",
                                                    )))
                                                    .type_(xs::Type(xs::QName(ExpandedName::new(
                                                        LocalName::new_dangerous("localSimpleType"),
                                                        Some(XmlNamespace::XMLNS),
                                                    ))))
                                                    .min_occurs(xs::MinOccurs(0))
                                                    .build()
                                                    .into(),
                                                xs::LocalElement::builder()
                                                    .name(xs::Name(LocalName::new_dangerous(
                                                        "complexType",
                                                    )))
                                                    .type_(xs::Type(xs::QName(ExpandedName::new(
                                                        LocalName::new_dangerous(
                                                            "localComplexType",
                                                        ),
                                                        Some(XmlNamespace::XMLNS),
                                                    ))))
                                                    .min_occurs(xs::MinOccurs(0))
                                                    .build()
                                                    .into(),
                                            ])
                                            .build()
                                            .into(),
                                        xs::LocalElement::builder()
                                            .name(xs::Name(LocalName::new_dangerous("complexType")))
                                            .type_(xs::Type(xs::QName(ExpandedName::new(
                                                LocalName::new_dangerous("altType"),
                                                Some(XmlNamespace::XMLNS),
                                            ))))
                                            .min_occurs(xs::MinOccurs(0))
                                            .max_occurs(xs::MaxOccurs(
                                                xs::MaxOccursValue::Unbounded,
                                            ))
                                            .build()
                                            .into(),
                                    ])
                                    .build()
                                    .into(),
                            )
                            .build()
                            .into(),
                    )
                    .build()
                    .into(),
            )
            .build()
            .to_complex_fragments(&mut fragment_compiler);

        assert_eq!(element_id, FragmentIdx::new(1));
        assert_eq!(fragment_compiler.complex_types.len(), 2);
        assert_eq!(fragment_compiler.elements.len(), 4);
        assert_eq!(fragment_compiler.sequences.len(), 2);
        assert_eq!(fragment_compiler.choices.len(), 1);
        assert_eq!(fragment_compiler.complex_contents.len(), 2);

        // assert_eq!(fragment_compiler.fragments.len(), 9);

        // assert_eq!(element_id, FragmentId(namespace.clone(), FragmentIdx(8)));

        // assert!(matches!(
        //     fragment_compiler.fragments[&FragmentIdx(3)],
        //     ComplexTypeFragment::Element(_)
        // ));
        // assert!(matches!(
        //     fragment_compiler.fragments[&FragmentIdx(4)],
        //     ComplexTypeFragment::Element(_)
        // ));
        // assert!(matches!(
        //     fragment_compiler.fragments[&FragmentIdx(5)],
        //     ComplexTypeFragment::Choice { .. }
        // ));
        // assert!(matches!(
        //     fragment_compiler.fragments[&FragmentIdx(6)],
        //     ComplexTypeFragment::Element(_)
        // ));
        // assert!(matches!(
        //     fragment_compiler.fragments[&FragmentIdx(7)],
        //     ComplexTypeFragment::Sequence { .. }
        // ));
        // assert!(matches!(
        //     fragment_compiler.fragments[&FragmentIdx(8)],
        //     ComplexTypeFragment::ComplexContent(_)
        // ));

        println!("{:#?}", fragment_compiler);
    }
}
