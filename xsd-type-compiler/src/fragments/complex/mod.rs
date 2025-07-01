//! This module contains the logic for compiling complex types into fragments.
//!
//! It is naturally dependent on the simple type compiler, as complex types can contain simple types.

pub mod transformers;

use std::{
    collections::{BTreeMap, VecDeque},
    marker::PhantomData,
    ops::Deref,
};

use crate::{
    fragments::{
        simple::{self, SimpleFragmentEquivalent, SimpleTypeFragmentCompiler},
        FragmentAccess, FragmentCollection, FragmentIdx, HasFragmentCollection,
    },
    NamedOrAnonymous,
};
use xmlity::{ExpandedName, LocalName, XmlNamespace};

use xsd::xs;

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
    pub attribute_declarations: FragmentIdx<AttributeDeclarationsFragment>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RestrictionFragment {
    pub base: ExpandedName<'static>,
    pub content_fragment: Option<TypeDefParticleId>,
    pub attribute_declarations: FragmentIdx<AttributeDeclarationsFragment>,
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
    pub type_: Option<NamedOrAnonymous<FragmentIdx<simple::SimpleTypeRootFragment>>>,
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
    pub default: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TopLevelAttributeFragment {
    pub name: LocalName<'static>,
    pub type_: Option<NamedOrAnonymous<FragmentIdx<simple::SimpleTypeRootFragment>>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TopLevelGroupFragment {
    pub name: LocalName<'static>,
    pub content: NamedGroupTypeContentId,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AttributeGroupRefFragment {
    ref_: ExpandedName<'static>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TopLevelAttributeGroupFragment {
    pub name: LocalName<'static>,
    pub attr_decls: FragmentIdx<AttributeDeclarationsFragment>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AttributeDeclarationsFragment {
    pub declarations: VecDeque<AttributeDeclarationId>,
    //TODO: Any attribute
}

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
    pub min_occurs: Option<usize>,
    pub max_occurs: Option<xs::types::AllNNI>,
    pub type_: LocalElementFragmentType,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TopLevelElementFragment {
    pub name: LocalName<'static>,
    pub type_: Option<NamedOrAnonymous<ElementTypeContentId>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GroupRefFragment {
    pub min_occurs: Option<usize>,
    pub max_occurs: Option<xs::types::AllNNI>,
    pub ref_: ExpandedName<'static>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AllFragment {
    pub min_occurs: Option<usize>,
    pub max_occurs: Option<xs::types::AllNNI>,
    pub fragments: VecDeque<NestedParticleId>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ChoiceFragment {
    pub min_occurs: Option<usize>,
    pub max_occurs: Option<xs::types::AllNNI>,
    pub fragments: VecDeque<NestedParticleId>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SequenceFragment {
    pub id: Option<String>,
    pub min_occurs: Option<usize>,
    pub max_occurs: Option<xs::types::AllNNI>,
    pub fragments: VecDeque<NestedParticleId>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ComplexTypeModelId {
    SimpleContent(FragmentIdx<SimpleContentFragment>),
    ComplexContent(FragmentIdx<ComplexContentFragment>),
    Other {
        // open_content: Option<OpenContentId>,
        particle: Option<TypeDefParticleId>,
        attr_decls: FragmentIdx<AttributeDeclarationsFragment>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct ComplexTypeRootFragment {
    pub name: Option<LocalName<'static>>,
    pub content: ComplexTypeModelId,
    pub mixed: Option<bool>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AnyFragment {}

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
    pub attribute_group_refs: FragmentCollection<AttributeGroupRefFragment>,
    pub groups: FragmentCollection<TopLevelGroupFragment>,
    pub attribute_groups: FragmentCollection<TopLevelAttributeGroupFragment>,
    pub attribute_declarations: FragmentCollection<AttributeDeclarationsFragment>,
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

impl HasFragmentCollection<AttributeGroupRefFragment> for ComplexTypeFragmentCompiler {
    fn get_fragment_collection(&self) -> &FragmentCollection<AttributeGroupRefFragment> {
        &self.attribute_group_refs
    }
    fn get_fragment_collection_mut(
        &mut self,
    ) -> &mut FragmentCollection<AttributeGroupRefFragment> {
        &mut self.attribute_group_refs
    }
}

impl HasFragmentCollection<TopLevelGroupFragment> for ComplexTypeFragmentCompiler {
    fn get_fragment_collection(&self) -> &FragmentCollection<TopLevelGroupFragment> {
        &self.groups
    }
    fn get_fragment_collection_mut(&mut self) -> &mut FragmentCollection<TopLevelGroupFragment> {
        &mut self.groups
    }
}

impl HasFragmentCollection<TopLevelAttributeGroupFragment> for ComplexTypeFragmentCompiler {
    fn get_fragment_collection(&self) -> &FragmentCollection<TopLevelAttributeGroupFragment> {
        &self.attribute_groups
    }
    fn get_fragment_collection_mut(
        &mut self,
    ) -> &mut FragmentCollection<TopLevelAttributeGroupFragment> {
        &mut self.attribute_groups
    }
}

impl HasFragmentCollection<AttributeDeclarationsFragment> for ComplexTypeFragmentCompiler {
    fn get_fragment_collection(&self) -> &FragmentCollection<AttributeDeclarationsFragment> {
        &self.attribute_declarations
    }
    fn get_fragment_collection_mut(
        &mut self,
    ) -> &mut FragmentCollection<AttributeDeclarationsFragment> {
        &mut self.attribute_declarations
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
            attribute_group_refs: FragmentCollection::new(),
            groups: FragmentCollection::new(),
            attribute_groups: FragmentCollection::new(),
            attribute_declarations: FragmentCollection::new(),
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

impl From<simple::Error> for Error {
    fn from(err: simple::Error) -> Self {
        Error
    }
}

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
    SimpleType(FragmentIdx<simple::SimpleTypeRootFragment>),
    ComplexType(FragmentIdx<ComplexTypeRootFragment>),
}

impl ComplexFragmentEquivalent for xs::types::top_level_element_items::Type {
    type FragmentId = ElementTypeContentId;

    fn to_complex_fragments<T: AsMut<ComplexTypeFragmentCompiler>>(
        &self,
        mut compiler: T,
    ) -> Self::FragmentId {
        let mut compiler = compiler.as_mut();

        match self {
            xs::types::top_level_element_items::Type::SimpleType(local_simple_type) => {
                let simple_type_fragment = local_simple_type.to_simple_fragments(&mut compiler);

                ElementTypeContentId::SimpleType(simple_type_fragment)
            }
            xs::types::top_level_element_items::Type::ComplexType(local_complex_type) => {
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
                xs::types::LocalSimpleType::from_simple_fragments(compiler, fragment_id)
                    .map(Box::new)
                    .map(xs::types::top_level_element_items::Type::SimpleType)
                    .map_err(From::from)
            }
            ElementTypeContentId::ComplexType(fragment_idx) => {
                xs::types::LocalComplexType::from_complex_fragments(compiler, fragment_idx)
                    .map(Box::new)
                    .map(xs::types::top_level_element_items::Type::ComplexType)
            }
        }
    }
}

impl ComplexFragmentEquivalent for xs::types::local_element_items::Type {
    type FragmentId = ElementTypeContentId;

    fn to_complex_fragments<T: AsMut<ComplexTypeFragmentCompiler>>(
        &self,
        mut compiler: T,
    ) -> Self::FragmentId {
        let mut compiler = compiler.as_mut();

        match self {
            xs::types::local_element_items::Type::SimpleType(local_simple_type) => {
                let simple_type_fragment = local_simple_type.to_simple_fragments(&mut compiler);

                ElementTypeContentId::SimpleType(simple_type_fragment)
            }
            xs::types::local_element_items::Type::ComplexType(local_complex_type) => {
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
                xs::types::LocalSimpleType::from_simple_fragments(compiler, fragment_id)
                    .map(Box::new)
                    .map(xs::types::local_element_items::Type::SimpleType)
                    .map_err(From::from)
            }
            ElementTypeContentId::ComplexType(fragment_idx) => {
                xs::types::LocalComplexType::from_complex_fragments(compiler, fragment_idx)
                    .map(Box::new)
                    .map(xs::types::local_element_items::Type::ComplexType)
            }
        }
    }
}

impl ComplexFragmentEquivalent for xs::types::LocalElement {
    type FragmentId = FragmentIdx<LocalElementFragment>;

    fn to_complex_fragments<T: AsMut<ComplexTypeFragmentCompiler>>(
        &self,
        mut compiler: T,
    ) -> Self::FragmentId {
        let mut compiler = compiler.as_mut();

        let max_occurs = self.max_occurs;
        let min_occurs = self.min_occurs;

        let type_ = if let Some(ref_) = self.ref_.as_ref() {
            LocalElementFragmentType::Reference(ReferenceElementFragment {
                name: ref_.0.clone(),
            })
        } else {
            let name = self
                .name
                .clone()
                .expect("If ref is none, type_choice should be Some");

            let type_ = if let Some(type_) = self.type_attribute.as_ref() {
                NamedOrAnonymous::Named(type_.0.clone())
            } else {
                let type_choice = self
                    .type_
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

        let element_builder = xs::types::LocalElement::builder()
            .maybe_min_occurs(fragment.min_occurs)
            .maybe_max_occurs(fragment.max_occurs);

        match &fragment.type_ {
            LocalElementFragmentType::Local(fragment) => Ok(element_builder
                .name(fragment.name.clone())
                .maybe_type_attribute(match &fragment.type_ {
                    NamedOrAnonymous::Named(expanded_name) => {
                        Some(xs::types::QName(expanded_name.clone()))
                    }
                    NamedOrAnonymous::Anonymous(_) => None,
                })
                .maybe_type_(
                    match &fragment.type_ {
                        NamedOrAnonymous::Anonymous(content_type) => Some(
                            xs::types::local_element_items::Type::from_complex_fragments(
                                compiler,
                                content_type,
                            ),
                        ),
                        NamedOrAnonymous::Named(_) => None,
                    }
                    .transpose()?,
                )
                .build()),
            LocalElementFragmentType::Reference(fragment) => Ok(element_builder
                .ref_(xs::types::QName(fragment.name.clone()))
                .build()),
        }
    }
}

impl ComplexFragmentEquivalent for xs::types::TopLevelElement {
    type FragmentId = FragmentIdx<TopLevelElementFragment>;

    fn to_complex_fragments<T: AsMut<ComplexTypeFragmentCompiler>>(
        &self,
        mut compiler: T,
    ) -> Self::FragmentId {
        let mut compiler = compiler.as_mut();

        let name = self.name.clone();

        let type_ = match (self.type_attribute.as_ref(), self.type_.as_ref()) {
            (Some(type_), None) => Some(NamedOrAnonymous::Named(type_.0.clone())),
            (None, Some(type_choice)) => {
                let content_type = type_choice.to_complex_fragments(&mut compiler);
                Some(NamedOrAnonymous::Anonymous(content_type))
            }
            (Some(_), Some(_)) => panic!("Both type and type_choice are Some. Name: {name}"),
            (None, None) => None,
        };

        compiler.push_fragment(TopLevelElementFragment { name, type_ })
    }

    fn from_complex_fragments<T: AsRef<ComplexTypeFragmentCompiler>>(
        compiler: T,
        fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        let compiler = compiler.as_ref();

        let fragment = compiler.get_fragment(fragment_id).unwrap();

        Ok(xs::types::TopLevelElement::builder()
            .name(fragment.name.clone())
            .maybe_type_attribute(fragment.type_.as_ref().and_then(|f| match f {
                NamedOrAnonymous::Named(expanded_name) => {
                    Some(xs::types::QName(expanded_name.clone()))
                }
                NamedOrAnonymous::Anonymous(_) => None,
            }))
            .maybe_type_(
                fragment
                    .type_
                    .as_ref()
                    .and_then(|f| match f {
                        NamedOrAnonymous::Anonymous(content_type) => Some(
                            xs::types::top_level_element_items::Type::from_complex_fragments(
                                compiler,
                                content_type,
                            ),
                        ),
                        NamedOrAnonymous::Named(_) => None,
                    })
                    .transpose()?,
            )
            .build())
    }
}

impl ComplexFragmentEquivalent for xs::types::GroupRef {
    type FragmentId = FragmentIdx<GroupRefFragment>;

    fn to_complex_fragments<T: AsMut<ComplexTypeFragmentCompiler>>(
        &self,
        mut compiler: T,
    ) -> Self::FragmentId {
        let compiler = compiler.as_mut();

        compiler.push_fragment(GroupRefFragment {
            min_occurs: self.min_occurs,
            max_occurs: self.max_occurs,
            ref_: self.ref_.0.clone(),
        })
    }

    fn from_complex_fragments<T: AsRef<ComplexTypeFragmentCompiler>>(
        compiler: T,
        fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        let compiler = compiler.as_ref();

        let fragment = compiler.get_fragment(fragment_id).unwrap();

        Ok(xs::types::GroupRef {
            id: None,
            min_occurs: fragment.min_occurs,
            max_occurs: fragment.max_occurs,
            ref_: xs::types::QName(fragment.ref_.clone()),
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
        let compiler = compiler.as_mut();

        compiler.push_fragment(AnyFragment {})
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
pub enum NestedParticleId {
    Element(FragmentIdx<LocalElementFragment>),
    Group(FragmentIdx<GroupRefFragment>),
    Choice(FragmentIdx<ChoiceFragment>),
    Sequence(FragmentIdx<SequenceFragment>),
    Any(FragmentIdx<AnyFragment>),
}

impl From<TypeDefParticleId> for NestedParticleId {
    fn from(value: TypeDefParticleId) -> Self {
        match value {
            TypeDefParticleId::Group(fragment_idx) => Self::Group(fragment_idx),
            TypeDefParticleId::All(fragment_idx) => todo!(),
            TypeDefParticleId::Choice(fragment_idx) => Self::Choice(fragment_idx),
            TypeDefParticleId::Sequence(fragment_idx) => Self::Sequence(fragment_idx),
        }
    }
}

impl From<FragmentIdx<LocalElementFragment>> for NestedParticleId {
    fn from(value: FragmentIdx<LocalElementFragment>) -> Self {
        Self::Element(value)
    }
}
impl From<FragmentIdx<GroupRefFragment>> for NestedParticleId {
    fn from(value: FragmentIdx<GroupRefFragment>) -> Self {
        Self::Group(value)
    }
}
impl From<FragmentIdx<ChoiceFragment>> for NestedParticleId {
    fn from(value: FragmentIdx<ChoiceFragment>) -> Self {
        Self::Choice(value)
    }
}
impl From<FragmentIdx<SequenceFragment>> for NestedParticleId {
    fn from(value: FragmentIdx<SequenceFragment>) -> Self {
        Self::Sequence(value)
    }
}

impl From<FragmentIdx<AnyFragment>> for NestedParticleId {
    fn from(value: FragmentIdx<AnyFragment>) -> Self {
        Self::Any(value)
    }
}

impl ComplexFragmentEquivalent for xs::groups::NestedParticle {
    type FragmentId = NestedParticleId;

    fn to_complex_fragments<T: AsMut<ComplexTypeFragmentCompiler>>(
        &self,
        mut compiler: T,
    ) -> Self::FragmentId {
        let compiler: &mut ComplexTypeFragmentCompiler = compiler.as_mut();

        use xs::groups::NestedParticle;

        match self {
            NestedParticle::Element(local_element) => {
                local_element.to_complex_fragments(compiler).into()
            }
            NestedParticle::Group(group_type) => group_type.to_complex_fragments(compiler).into(),
            NestedParticle::Choice(choice_type) => {
                choice_type.to_complex_fragments(compiler).into()
            }
            NestedParticle::Sequence(sequence_type) => {
                sequence_type.to_complex_fragments(compiler).into()
            }
            NestedParticle::Any(any) => any.to_complex_fragments(compiler).into(),
        }
    }

    fn from_complex_fragments<T: AsRef<ComplexTypeFragmentCompiler>>(
        compiler: T,
        fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        let compiler = compiler.as_ref();
        match fragment_id {
            NestedParticleId::Element(fragment_idx) => {
                xs::types::LocalElement::from_complex_fragments(compiler, fragment_idx)
                    .map(xs::groups::NestedParticle::from)
            }
            NestedParticleId::Group(fragment_idx) => {
                xs::types::GroupRef::from_complex_fragments(compiler, fragment_idx)
                    .map(xs::groups::NestedParticle::from)
            }
            NestedParticleId::Choice(fragment_idx) => {
                xs::Choice::from_complex_fragments(compiler, fragment_idx)
                    .map(xs::groups::NestedParticle::from)
            }
            NestedParticleId::Sequence(fragment_idx) => {
                xs::Sequence::from_complex_fragments(compiler, fragment_idx)
                    .map(xs::groups::NestedParticle::from)
            }
            NestedParticleId::Any(_) => {
                unreachable!()
            }
        }
    }
}

impl ComplexFragmentEquivalent for xs::groups::all_model_items::Child1 {
    type FragmentId = NestedParticleId;

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

impl ComplexFragmentEquivalent for xs::All {
    type FragmentId = FragmentIdx<AllFragment>;

    fn to_complex_fragments<T: AsMut<ComplexTypeFragmentCompiler>>(
        &self,
        mut compiler: T,
    ) -> Self::FragmentId {
        let mut compiler = compiler.as_mut();

        let all = AllFragment {
            min_occurs: None,
            max_occurs: None,
            fragments: self
                .0
                .all_model
                .child_1
                .iter()
                .map(|content| content.to_complex_fragments(&mut compiler))
                .collect(),
        };

        compiler.push_fragment(all)
    }

    fn from_complex_fragments<T: AsRef<ComplexTypeFragmentCompiler>>(
        compiler: T,
        _fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        let _compiler = compiler.as_ref();

        todo!()
    }
}

impl ComplexFragmentEquivalent for xs::Choice {
    type FragmentId = FragmentIdx<ChoiceFragment>;

    fn to_complex_fragments<T: AsMut<ComplexTypeFragmentCompiler>>(
        &self,
        mut compiler: T,
    ) -> Self::FragmentId {
        let mut compiler = compiler.as_mut();

        let all = ChoiceFragment {
            min_occurs: self.0.min_occurs,
            max_occurs: self.0.max_occurs,
            fragments: self
                .0
                .nested_particle
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

        Ok(xs::Choice(
            xs::types::ExplicitGroup::builder()
                .maybe_min_occurs(fragment.min_occurs)
                .maybe_max_occurs(fragment.max_occurs)
                .nested_particle(
                    fragment
                        .fragments
                        .iter()
                        .map(|fragment| {
                            xs::groups::NestedParticle::from_complex_fragments(compiler, fragment)
                        })
                        .collect::<Result<_, _>>()?,
                )
                .build()
                .into(),
        ))
    }
}

impl ComplexFragmentEquivalent for xs::Sequence {
    type FragmentId = FragmentIdx<SequenceFragment>;

    fn to_complex_fragments<T: AsMut<ComplexTypeFragmentCompiler>>(
        &self,
        mut compiler: T,
    ) -> Self::FragmentId {
        let mut compiler = compiler.as_mut();

        let seq = SequenceFragment {
            id: self.0.id.clone(),
            min_occurs: self.0.min_occurs,
            max_occurs: self.0.max_occurs,
            fragments: self
                .0
                .nested_particle
                .iter()
                .map(|content| content.to_complex_fragments(&mut compiler))
                .collect(),
        };

        compiler.push_fragment(seq)
    }

    fn from_complex_fragments<T: AsRef<ComplexTypeFragmentCompiler>>(
        compiler: T,
        fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        let compiler = compiler.as_ref();
        let fragment = compiler.get_fragment(fragment_id).unwrap();

        Ok(xs::Sequence(Box::new(
            xs::types::ExplicitGroup::builder()
                .maybe_min_occurs(fragment.min_occurs)
                .maybe_max_occurs(fragment.max_occurs)
                .nested_particle(
                    fragment
                        .fragments
                        .iter()
                        .map(|fragment| {
                            xs::groups::NestedParticle::from_complex_fragments(compiler, fragment)
                        })
                        .collect::<Result<_, _>>()?,
                )
                .build(),
        )))
    }
}

impl ComplexFragmentEquivalent for xs::groups::TypeDefParticle {
    type FragmentId = TypeDefParticleId;

    fn to_complex_fragments<T: AsMut<ComplexTypeFragmentCompiler>>(
        &self,
        mut compiler: T,
    ) -> Self::FragmentId {
        let compiler = compiler.as_mut();

        use xs::groups::TypeDefParticle;

        match self {
            TypeDefParticle::Group(group_ref) => group_ref.to_complex_fragments(compiler).into(),
            TypeDefParticle::All(all) => all.to_complex_fragments(compiler).into(),
            TypeDefParticle::Choice(choice) => choice.to_complex_fragments(compiler).into(),
            TypeDefParticle::Sequence(sequence) => sequence.to_complex_fragments(compiler).into(),
        }
    }

    fn from_complex_fragments<T: AsRef<ComplexTypeFragmentCompiler>>(
        compiler: T,
        fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        match fragment_id {
            TypeDefParticleId::Group(group_ref) => {
                let group_ref = xs::types::GroupRef::from_complex_fragments(compiler, group_ref)?;
                Ok(xs::groups::TypeDefParticle::Group(Box::new(group_ref)))
            }
            TypeDefParticleId::All(all) => {
                let all = xs::All::from_complex_fragments(compiler, all)?;
                Ok(xs::groups::TypeDefParticle::All(Box::new(all)))
            }
            TypeDefParticleId::Choice(choice) => {
                let choice = xs::Choice::from_complex_fragments(compiler, choice)?;
                Ok(xs::groups::TypeDefParticle::Choice(Box::new(choice)))
            }
            TypeDefParticleId::Sequence(sequence) => {
                let sequence = xs::Sequence::from_complex_fragments(compiler, sequence)?;
                Ok(xs::groups::TypeDefParticle::Sequence(Box::new(sequence)))
            }
        }
    }
}

impl ComplexFragmentEquivalent for xs::types::ExtensionType {
    type FragmentId = FragmentIdx<ExtensionFragment>;

    /// This method expects all references to already be defined.
    fn to_complex_fragments<T: AsMut<ComplexTypeFragmentCompiler>>(
        &self,
        mut compiler: T,
    ) -> Self::FragmentId {
        let mut compiler = compiler.as_mut();

        let base = self.base.0.clone();

        let content_fragment = self
            .type_def_particle
            .as_ref()
            .map(|content| content.to_complex_fragments(&mut compiler));

        let attribute_declarations = self.attr_decls.to_complex_fragments(&mut compiler);

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
            .map(|fragment_id| {
                xs::groups::TypeDefParticle::from_complex_fragments(compiler, fragment_id)
                    .map(Box::new)
            })
            .transpose()?;

        let attr_decls = xs::groups::AttrDecls::from_complex_fragments(
            compiler,
            &extension.attribute_declarations,
        )?;

        Ok(Self::builder()
            .base(xs::types::QName(extension.base.clone()))
            .maybe_type_def_particle(particle)
            .attr_decls(attr_decls.into())
            .assertions(xs::groups::Assertions::builder().build().into())
            .build())
    }
}

impl ComplexFragmentEquivalent for xs::types::ComplexRestrictionType {
    type FragmentId = FragmentIdx<RestrictionFragment>;

    fn to_complex_fragments<T: AsMut<ComplexTypeFragmentCompiler>>(
        &self,
        mut compiler: T,
    ) -> Self::FragmentId {
        let mut compiler = compiler.as_mut();

        let base = self.base.0.clone();

        let content_fragment = self.child_1.as_ref().map(|particle| {
            particle
                .type_def_particle
                .to_complex_fragments(&mut compiler)
        });

        let attribute_declarations = self.attr_decls.to_complex_fragments(&mut compiler);

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
            .map(|a| xs::groups::TypeDefParticle::from_complex_fragments(compiler, &a))
            .transpose()?;

        let attr_decls = xs::groups::AttrDecls::from_complex_fragments(
            compiler,
            &fragment.attribute_declarations,
        )?;

        Ok(xs::types::ComplexRestrictionType::builder()
            .base(xs::types::QName(fragment.base.clone()))
            .maybe_child_1(particle.map(|particle| {
                xs::types::complex_restriction_type_items::Child1 {
                    open_content: None,
                    type_def_particle: Box::new(particle),
                }
                .into()
            }))
            .attr_decls(attr_decls.into())
            .assertions(xs::groups::Assertions::builder().build().into())
            .build())
    }
}

impl ComplexFragmentEquivalent for xs::groups::attr_decls_items::Attribute {
    type FragmentId = AttributeDeclarationId;

    fn to_complex_fragments<T: AsMut<ComplexTypeFragmentCompiler>>(
        &self,
        mut compiler: T,
    ) -> Self::FragmentId {
        let compiler = compiler.as_mut();
        use xs::groups::attr_decls_items::Attribute;

        match self {
            Attribute::Attribute(local) => {
                AttributeDeclarationId::Attribute(local.to_complex_fragments(compiler))
            }
            Attribute::AttributeGroup(group) => {
                AttributeDeclarationId::AttributeGroupRef(group.to_complex_fragments(compiler))
            }
        }
    }

    fn from_complex_fragments<T: AsRef<ComplexTypeFragmentCompiler>>(
        compiler: T,
        fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        let compiler = compiler.as_ref();
        use xs::groups::attr_decls_items::Attribute;

        match fragment_id {
            AttributeDeclarationId::Attribute(fragment_idx) => {
                xs::types::Attribute::from_complex_fragments(compiler, fragment_idx)
                    .map(Attribute::from)
            }
            AttributeDeclarationId::AttributeGroupRef(fragment_idx) => {
                xs::types::AttributeGroupRef::from_complex_fragments(compiler, fragment_idx)
                    .map(Attribute::from)
            }
        }
    }
}

impl ComplexFragmentEquivalent for xs::types::Attribute {
    type FragmentId = FragmentIdx<LocalAttributeFragment>;

    fn to_complex_fragments<T: AsMut<ComplexTypeFragmentCompiler>>(
        &self,
        mut compiler: T,
    ) -> Self::FragmentId {
        let mut compiler = compiler.as_mut();

        let use_ = self
            .use_
            .as_ref()
            .map(|a| match a {
                xs::types::attribute_items::UseValue::Prohibited => AttributeUse::Prohibited,
                xs::types::attribute_items::UseValue::Optional => AttributeUse::Optional,
                xs::types::attribute_items::UseValue::Required => AttributeUse::Required,
            });

        let type_mode = if let Some(ref ref_) = self.ref_ {
            LocalAttributeFragmentTypeMode::Reference(ReferenceAttributeFragment {
                name: ref_.0.clone(),
            })
        } else {
            let name = self
                .name
                .as_ref()
                .expect("name is required if not a reference");

            let type_ = if let Some(type_) = self.type_.as_ref() {
                Some(NamedOrAnonymous::Named(type_.0.clone()))
            } else {
                self.simple_type.as_ref().map(|simple_type| {
                    NamedOrAnonymous::Anonymous(simple_type.to_simple_fragments(&mut compiler))
                })
            };

            LocalAttributeFragmentTypeMode::Declared(DeclaredAttributeFragment {
                name: name.clone(),
                type_,
            })
        };
        compiler.push_fragment(LocalAttributeFragment {
            type_mode,
            use_,
            default: self.default.clone(),
        })
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
                    NamedOrAnonymous::Named(ref_) => Some(xs::types::QName(ref_.clone())),
                    NamedOrAnonymous::Anonymous(_) => None,
                };
                let use_ = fragment
                    .use_
                    .map(|a| match a {
                        AttributeUse::Required => xs::types::attribute_items::UseValue::Required,
                        AttributeUse::Optional => xs::types::attribute_items::UseValue::Optional,
                        AttributeUse::Prohibited => xs::types::attribute_items::UseValue::Prohibited,
                    });
                Ok(xs::types::Attribute::builder()
                    .name(name)
                    .maybe_type_(type_)
                    .maybe_use_(use_)
                    .maybe_default(fragment.default.clone())
                    .build())
            }
            _ => todo!(),
        }
    }
}

impl ComplexFragmentEquivalent for xs::types::TopLevelAttribute {
    type FragmentId = FragmentIdx<TopLevelAttributeFragment>;

    fn to_complex_fragments<T: AsMut<ComplexTypeFragmentCompiler>>(
        &self,
        mut compiler: T,
    ) -> Self::FragmentId {
        let mut compiler = compiler.as_mut();

        let name = self.name.clone();

        let type_ = match (self.type_.as_ref(), self.simple_type.as_ref()) {
            (None, Some(s)) => Some(NamedOrAnonymous::Anonymous(
                s.to_simple_fragments(&mut compiler),
            )),
            (Some(t), None) => Some(NamedOrAnonymous::Named(t.0.clone())),
            (Some(_), Some(_)) => todo!("Cannot have both type and simpleType"),
            (None, None) => None,
        };

        compiler.push_fragment(TopLevelAttributeFragment { name, type_ })
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
            NamedOrAnonymous::Named(ref_) => Some(xs::types::QName(ref_.clone())),
            NamedOrAnonymous::Anonymous(_) => None,
        };
        Ok(xs::types::TopLevelAttribute::builder()
            .name(name)
            .maybe_type_(type_)
            .build())
    }
}

impl ComplexFragmentEquivalent for xs::types::AttributeGroupRef {
    type FragmentId = FragmentIdx<AttributeGroupRefFragment>;

    fn to_complex_fragments<T: AsMut<ComplexTypeFragmentCompiler>>(
        &self,
        mut compiler: T,
    ) -> Self::FragmentId {
        let compiler = compiler.as_mut();

        //TODO

        compiler.push_fragment(AttributeGroupRefFragment {
            ref_: self.ref_.0.clone(),
        })
    }

    fn from_complex_fragments<T: AsRef<ComplexTypeFragmentCompiler>>(
        compiler: T,
        fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        let compiler = compiler.as_ref();

        let fragment = compiler.get_fragment(fragment_id).unwrap();

        Ok(xs::types::AttributeGroupRef::builder()
            .ref_(xs::types::QName(fragment.ref_.clone()))
            .build())
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

        let content_fragment = match &self.child_1 {
            xs::complex_content_items::Child1::Extension(extension) => {
                let fragment_id = extension.to_complex_fragments(&mut compiler);

                ComplexContentChildId::Extension(fragment_id)
            }
            xs::complex_content_items::Child1::Restriction(restriction) => {
                let fragment_id = restriction.to_complex_fragments(&mut compiler);

                ComplexContentChildId::Restriction(fragment_id)
            }
        };

        compiler.push_fragment(ComplexContentFragment {
            content_fragment,
            mixed: self.mixed,
        })
    }

    fn from_complex_fragments<T: AsRef<ComplexTypeFragmentCompiler>>(
        compiler: T,
        fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        let compiler = compiler.as_ref();

        let fragment = compiler.get_fragment(fragment_id).unwrap();
        let child_1 = match &fragment.content_fragment {
            ComplexContentChildId::Extension(fragment_id) => {
                xs::types::ExtensionType::from_complex_fragments(compiler, fragment_id)?.into()
            }
            ComplexContentChildId::Restriction(fragment_id) => {
                xs::types::ComplexRestrictionType::from_complex_fragments(compiler, fragment_id)?
                    .into()
            }
        };

        Ok(xs::ComplexContent {
            annotation: None,
            id: None,
            mixed: fragment.mixed,
            child_1,
        })
    }
}

impl ComplexFragmentEquivalent for xs::groups::ComplexTypeModel {
    type FragmentId = ComplexTypeModelId;

    /// This method expects all references to already be defined.
    fn to_complex_fragments<T: AsMut<ComplexTypeFragmentCompiler>>(
        &self,
        mut compiler: T,
    ) -> Self::FragmentId {
        let mut compiler = compiler.as_mut();

        use xs::groups::ComplexTypeModel;

        match self {
            ComplexTypeModel::SimpleContent(_simple_content) => {
                todo!()
            }
            ComplexTypeModel::ComplexContent(complex_content) => {
                ComplexTypeModelId::ComplexContent(complex_content.to_complex_fragments(compiler))
            }
            ComplexTypeModel::Variant2(variant_2) => {
                let xs::groups::complex_type_model_items::complex_type_model_variants::Variant2 {
                    open_content,
                    type_def_particle,
                    attr_decls,
                    assertions,
                } = variant_2.deref();

                //TODO: Review open content
                let particle = type_def_particle
                    .as_deref()
                    .map(|a| a.to_complex_fragments(&mut compiler));

                let attributes = attr_decls.to_complex_fragments(&mut compiler);

                ComplexTypeModelId::Other {
                    particle,
                    attr_decls: attributes,
                }
            }
        }
    }

    fn from_complex_fragments<T: AsRef<ComplexTypeFragmentCompiler>>(
        compiler: T,
        fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        let compiler = compiler.as_ref();

        match fragment_id {
            ComplexTypeModelId::SimpleContent(_fragment_idx) => {
                // xs::SimpleContent::from_complex_fragments(compiler, fragment_idx)
                //     .map(xs::groups::ComplexTypeModel::from)
                todo!()
            }
            ComplexTypeModelId::ComplexContent(fragment_idx) => {
                xs::ComplexContent::from_complex_fragments(compiler, fragment_idx)
                    .map(xs::groups::ComplexTypeModel::from)
            }
            ComplexTypeModelId::Other {
                particle,
                attr_decls,
            } => {
                let type_def_particle = particle
                    .as_ref()
                    .map(|fragment_id| {
                        xs::groups::TypeDefParticle::from_complex_fragments(compiler, fragment_id)
                            .map(Box::new)
                    })
                    .transpose()?;

                let attributes =
                    xs::groups::AttrDecls::from_complex_fragments(compiler, attr_decls)?;

                Ok(
                    xs::groups::complex_type_model_items::complex_type_model_variants::Variant2 {
                        open_content: None,
                        type_def_particle,
                        attr_decls: attributes,
                        assertions: xs::groups::Assertions::builder().build(),
                    }
                    .into(),
                )
            }
        }
    }
}

impl ComplexFragmentEquivalent for xs::types::TopLevelComplexType {
    type FragmentId = FragmentIdx<ComplexTypeRootFragment>;

    fn to_complex_fragments<T: AsMut<ComplexTypeFragmentCompiler>>(
        &self,
        mut compiler: T,
    ) -> Self::FragmentId {
        let mut compiler = compiler.as_mut();

        let content = self.complex_type_model.to_complex_fragments(&mut compiler);

        let fragment = ComplexTypeRootFragment {
            name: Some(self.name.clone()),
            content,
            mixed: self.mixed,
        };

        compiler.push_fragment(fragment)
    }

    fn from_complex_fragments<T: AsRef<ComplexTypeFragmentCompiler>>(
        compiler: T,
        fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        let compiler = compiler.as_ref();

        let fragment = compiler.get_fragment(fragment_id).unwrap();

        let complex_type_model =
            xs::groups::ComplexTypeModel::from_complex_fragments(compiler, &fragment.content)?;

        Ok(Self::builder()
            .name(fragment.name.clone().ok_or_else(|| 
                //TODO
                Error)?)
            .maybe_mixed(fragment.mixed)
            .complex_type_model(complex_type_model.into())
            .build())
    }
}

impl ComplexFragmentEquivalent for xs::types::LocalComplexType {
    type FragmentId = FragmentIdx<ComplexTypeRootFragment>;

    fn to_complex_fragments<T: AsMut<ComplexTypeFragmentCompiler>>(
        &self,
        mut compiler: T,
    ) -> Self::FragmentId {
        let mut compiler = compiler.as_mut();

        let content = self.complex_type_model.to_complex_fragments(&mut compiler);

        let fragment = ComplexTypeRootFragment {
            name: None,
            content,
            mixed: self.mixed,
        };

        compiler.push_fragment(fragment)
    }

    fn from_complex_fragments<T: AsRef<ComplexTypeFragmentCompiler>>(
        compiler: T,
        fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        let compiler = compiler.as_ref();

        let fragment = compiler.get_fragment(fragment_id).unwrap();

        let content =
            xs::groups::ComplexTypeModel::from_complex_fragments(compiler, &fragment.content)?;

        Ok(xs::types::LocalComplexType::builder()
            .complex_type_model(content.into())
            .maybe_mixed(fragment.mixed)
            .build())
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NamedGroupTypeContentId {
    All(FragmentIdx<AllFragment>),
    Sequence(FragmentIdx<SequenceFragment>),
    Choice(FragmentIdx<ChoiceFragment>),
}

impl From<FragmentIdx<AllFragment>> for NamedGroupTypeContentId {
    fn from(value: FragmentIdx<AllFragment>) -> Self {
        Self::All(value)
    }
}

impl From<FragmentIdx<SequenceFragment>> for NamedGroupTypeContentId {
    fn from(value: FragmentIdx<SequenceFragment>) -> Self {
        Self::Sequence(value)
    }
}

impl From<FragmentIdx<ChoiceFragment>> for NamedGroupTypeContentId {
    fn from(value: FragmentIdx<ChoiceFragment>) -> Self {
        Self::Choice(value)
    }
}

impl ComplexFragmentEquivalent for xs::types::named_group_items::Child1 {
    type FragmentId = NamedGroupTypeContentId;

    fn to_complex_fragments<T: AsMut<ComplexTypeFragmentCompiler>>(
        &self,
        mut compiler: T,
    ) -> Self::FragmentId {
        let mut compiler = compiler.as_mut();
        use xs::types::named_group_items::Child1;

        match self {
            Child1::All {
                id,
                min_occurs,
                max_occurs,
                all_model,
            } => {
                let fragment = AllFragment {
                    min_occurs: None,
                    max_occurs: None,
                    fragments: all_model
                        .child_1
                        .iter()
                        .map(|content| content.to_complex_fragments(&mut compiler))
                        .collect(),
                };

                NamedGroupTypeContentId::All(compiler.push_fragment(fragment))
            }
            Child1::Choice(choice) => {
                let fragment = ChoiceFragment {
                    min_occurs: None,
                    max_occurs: None,
                    fragments: choice
                        .nested_particle
                        .iter()
                        .map(|content| content.to_complex_fragments(&mut compiler))
                        .collect(),
                };

                NamedGroupTypeContentId::Choice(compiler.push_fragment(fragment))
            }
            Child1::Sequence(sequence) => {
                let fragment = SequenceFragment {
                    id: None,
                    min_occurs: None,
                    max_occurs: None,
                    fragments: sequence
                        .nested_particle
                        .iter()
                        .map(|content| content.to_complex_fragments(&mut compiler))
                        .collect(),
                };

                NamedGroupTypeContentId::Sequence(compiler.push_fragment(fragment))
            }
        }
    }

    fn from_complex_fragments<T: AsRef<ComplexTypeFragmentCompiler>>(
        compiler: T,
        fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        todo!()
        // match fragment_id {
        //     NamedGroupTypeContentId::All(all) => {
        //         let all = xs::AllType::from_complex_fragments(compiler, all)?;
        //         Ok(xs::NamedGroupTypeContent::All(Box::new(all)))
        //     }
        //     NamedGroupTypeContentId::Choice(choice) => {
        //         let choice = xs::ChoiceType::from_complex_fragments(compiler, choice)?;
        //         Ok(xs::NamedGroupTypeContent::Choice(Box::new(choice)))
        //     }
        //     NamedGroupTypeContentId::Sequence(sequence) => {
        //         let sequence: xs::SequenceType =
        //             xs::SequenceType::from_complex_fragments(compiler, sequence)?;
        //         Ok(xs::NamedGroupTypeContent::Sequence(Box::new(sequence)))
        //     }
        // }
    }
}

impl ComplexFragmentEquivalent for xs::types::NamedGroup {
    type FragmentId = FragmentIdx<TopLevelGroupFragment>;

    fn to_complex_fragments<T: AsMut<ComplexTypeFragmentCompiler>>(
        &self,
        mut compiler: T,
    ) -> Self::FragmentId {
        let mut compiler = compiler.as_mut();

        let content = self.child_1.to_complex_fragments(&mut compiler);

        let fragment = TopLevelGroupFragment {
            name: self.name.clone(),
            content,
        };

        compiler.push_fragment(fragment)
    }

    fn from_complex_fragments<T: AsRef<ComplexTypeFragmentCompiler>>(
        compiler: T,
        fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        let compiler = compiler.as_ref();

        let fragment = compiler.get_fragment(fragment_id).unwrap();

        let content = xs::types::named_group_items::Child1::from_complex_fragments(
            compiler,
            &fragment.content,
        )?;

        Ok(Self::builder()
            .name(fragment.name.clone())
            .child_1(content.into())
            .build())
    }
}
impl ComplexFragmentEquivalent for xs::groups::AttrDecls {
    type FragmentId = FragmentIdx<AttributeDeclarationsFragment>;

    fn to_complex_fragments<T: AsMut<ComplexTypeFragmentCompiler>>(
        &self,
        mut compiler: T,
    ) -> Self::FragmentId {
        let mut compiler = compiler.as_mut();

        let attributes = self
            .attribute
            .iter()
            .map(|decl| decl.to_complex_fragments(&mut compiler))
            .collect();

        compiler.push_fragment(AttributeDeclarationsFragment {
            declarations: attributes,
        })
    }

    fn from_complex_fragments<T: AsRef<ComplexTypeFragmentCompiler>>(
        compiler: T,
        fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        let compiler = compiler.as_ref();

        let fragment = compiler.get_fragment(fragment_id).unwrap();

        Ok(xs::groups::AttrDecls::builder()
            .attribute(
                fragment
                    .declarations
                    .iter()
                    .map(|attr| {
                        xs::groups::attr_decls_items::Attribute::from_complex_fragments(
                            compiler, attr,
                        )
                    })
                    .collect::<Result<_, _>>()?,
            )
            .build())
    }
}

impl ComplexFragmentEquivalent for xs::types::NamedAttributeGroup {
    type FragmentId = FragmentIdx<TopLevelAttributeGroupFragment>;

    fn to_complex_fragments<T: AsMut<ComplexTypeFragmentCompiler>>(
        &self,
        mut compiler: T,
    ) -> Self::FragmentId {
        let mut compiler = compiler.as_mut();

        let attr_decls = self.attr_decls.to_complex_fragments(&mut compiler);

        compiler.push_fragment(TopLevelAttributeGroupFragment {
            name: self.name.clone(),
            attr_decls,
        })
    }

    fn from_complex_fragments<T: AsRef<ComplexTypeFragmentCompiler>>(
        compiler: T,
        fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        let compiler = compiler.as_ref();
        let fragment = compiler.get_fragment(fragment_id).unwrap();

        let attr_decls =
            xs::groups::AttrDecls::from_complex_fragments(compiler, &fragment.attr_decls)?;

        Ok(Self::builder()
            .name(fragment.name.clone())
            .attr_decls(attr_decls.into())
            .build())
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use xsd::schema_names as xsn;

//     #[test]
//     fn convert_annotated_to_fragments() {
//         let namespace = XmlNamespace::new_dangerous("http://localhost");

//         let simple_type_compiler = SimpleTypeFragmentCompiler::new(namespace.clone());
//         let mut fragment_compiler =
//             ComplexTypeFragmentCompiler::new(namespace.clone(), simple_type_compiler);

//         let id = xs::TopLevelComplexType::builder()
//             .name(LocalName::new_dangerous("annotated"))
//             .content(
//                 xs::ComplexContent::builder()
//                     .content(
//                         xs::ComplexRestrictionType::builder()
//                             .base(xs::types::QName(xsn::ANY_TYPE.clone()))
//                             .particle(
//                                 xs::SequenceType::builder()
//                                     .content(vec![xs::LocalElement::builder()
//                                         .ref_(xs::types::QName(ExpandedName::new(
//                                             LocalName::new_dangerous("annotation"),
//                                             Some(XmlNamespace::XS),
//                                         )))
//                                         .min_occurs((0))
//                                         .build()
//                                         .into()])
//                                     .build()
//                                     .into(),
//                             )
//                             .build()
//                             .into(),
//                     )
//                     .build()
//                     .into(),
//             )
//             .build()
//             .to_complex_fragments(&mut fragment_compiler);

//         assert_eq!(id, FragmentIdx::new(0));
//         assert_eq!(fragment_compiler.complex_types.len(), 1);
//         assert_eq!(fragment_compiler.elements.len(), 1);
//         assert_eq!(fragment_compiler.sequences.len(), 1);
//         assert_eq!(fragment_compiler.complex_contents.len(), 1);

//         println!("{:#?}", fragment_compiler);
//     }

//     #[test]
//     fn convert_element_to_fragments() {
//         let namespace = XmlNamespace::new_dangerous("http://localhost");

//         let simple_type_compiler = SimpleTypeFragmentCompiler::new(namespace.clone());
//         let mut fragment_compiler =
//             ComplexTypeFragmentCompiler::new(namespace.clone(), simple_type_compiler);

//         let annotated_name = LocalName::new_dangerous("annotated");
//         let annotated_expanded_name = ExpandedName::new(
//             annotated_name.clone(),
//             Some(fragment_compiler.namespace.clone()),
//         );

//         let annotated = xs::TopLevelComplexType::builder()
//             .name(annotated_name)
//             .content(
//                 xs::ComplexContent::builder()
//                     .content(
//                         xs::ComplexRestrictionType::builder()
//                             .base(xs::types::QName(xsn::ANY_TYPE.clone()))
//                             .particle(
//                                 xs::SequenceType::builder()
//                                     .content(vec![xs::LocalElement::builder()
//                                         .ref_(xs::types::QName(ExpandedName::new(
//                                             LocalName::new_dangerous("annotation"),
//                                             Some(XmlNamespace::XS),
//                                         )))
//                                         .min_occurs((0))
//                                         .build()
//                                         .into()])
//                                     .build()
//                                     .into(),
//                             )
//                             .build()
//                             .into(),
//                     )
//                     .build()
//                     .into(),
//             )
//             .build()
//             .to_complex_fragments(&mut fragment_compiler);

//         assert_eq!(annotated, FragmentIdx::new(0));
//         assert_eq!(fragment_compiler.complex_types.len(), 1);
//         assert_eq!(fragment_compiler.elements.len(), 1);
//         assert_eq!(fragment_compiler.sequences.len(), 1);
//         assert_eq!(fragment_compiler.complex_contents.len(), 1);

//         // ## "element"
//         // ```xml
//         // <xs:complexType name="element" abstract="true">
//         //     <xs:complexContent>
//         //         <xs:extension base="xs:annotated">
//         //             <xs:sequence>
//         //                 <xs:choice minOccurs="0">
//         //                     <xs:element name="simpleType" type="xs:localSimpleType"/>
//         //                     <xs:element name="complexType" type="xs:localComplexType"/>
//         //                 </xs:choice>
//         //                 <xs:element name="alternative" type="xs:altType"
//         //                         minOccurs="0" maxOccurs="unbounded"/>
//         //             </xs:sequence>
//         //         </xs:extension>
//         //     </xs:complexContent>
//         // </xs:complexType>
//         // ```
//         let element_id = xs::TopLevelComplexType::builder()
//             .name(LocalName::new_dangerous("element"))
//             .content(
//                 xs::ComplexContent::builder()
//                     .content(
//                         xs::ExtensionType::builder()
//                             .base(xs::types::QName(annotated_expanded_name))
//                             .particle(
//                                 xs::SequenceType::builder()
//                                     .content(vec![
//                                         xs::ChoiceType::builder()
//                                             .min_occurs((0))
//                                             .content(vec![
//                                                 xs::LocalElement::builder()
//                                                     .name(LocalName::new_dangerous("simpleType"))
//                                                     .type_(xs::types::QName(ExpandedName::new(
//                                                         LocalName::new_dangerous("localSimpleType"),
//                                                         Some(XmlNamespace::XS),
//                                                     )))
//                                                     .min_occurs((0))
//                                                     .build()
//                                                     .into(),
//                                                 xs::LocalElement::builder()
//                                                     .name(LocalName::new_dangerous("complexType"))
//                                                     .type_(xs::types::QName(ExpandedName::new(
//                                                         LocalName::new_dangerous(
//                                                             "localComplexType",
//                                                         ),
//                                                         Some(XmlNamespace::XS),
//                                                     )))
//                                                     .min_occurs((0))
//                                                     .build()
//                                                     .into(),
//                                             ])
//                                             .build()
//                                             .into(),
//                                         xs::LocalElement::builder()
//                                             .name(LocalName::new_dangerous("complexType"))
//                                             .type_(xs::types::QName(ExpandedName::new(
//                                                 LocalName::new_dangerous("altType"),
//                                                 Some(XmlNamespace::XS),
//                                             )))
//                                             .min_occurs((0))
//                                             .max_occurs((Value::Unbounded,))
//                                             .build()
//                                             .into(),
//                                     ])
//                                     .build()
//                                     .into(),
//                             )
//                             .build()
//                             .into(),
//                     )
//                     .build()
//                     .into(),
//             )
//             .build()
//             .to_complex_fragments(&mut fragment_compiler);

//         assert_eq!(element_id, FragmentIdx::new(1));
//         assert_eq!(fragment_compiler.complex_types.len(), 2);
//         assert_eq!(fragment_compiler.elements.len(), 4);
//         assert_eq!(fragment_compiler.sequences.len(), 2);
//         assert_eq!(fragment_compiler.choices.len(), 1);
//         assert_eq!(fragment_compiler.complex_contents.len(), 2);

//         // assert_eq!(fragment_compiler.fragments.len(), 9);

//         // assert_eq!(element_id, FragmentId(namespace.clone(), FragmentIdx(8)));

//         // assert!(matches!(
//         //     fragment_compiler.fragments[&FragmentIdx(3)],
//         //     ComplexTypeFragment::Element(_)
//         // ));
//         // assert!(matches!(
//         //     fragment_compiler.fragments[&FragmentIdx(4)],
//         //     ComplexTypeFragment::Element(_)
//         // ));
//         // assert!(matches!(
//         //     fragment_compiler.fragments[&FragmentIdx(5)],
//         //     ComplexTypeFragment::Choice { .. }
//         // ));
//         // assert!(matches!(
//         //     fragment_compiler.fragments[&FragmentIdx(6)],
//         //     ComplexTypeFragment::Element(_)
//         // ));
//         // assert!(matches!(
//         //     fragment_compiler.fragments[&FragmentIdx(7)],
//         //     ComplexTypeFragment::Sequence { .. }
//         // ));
//         // assert!(matches!(
//         //     fragment_compiler.fragments[&FragmentIdx(8)],
//         //     ComplexTypeFragment::ComplexContent(_)
//         // ));

//         println!("{:#?}", fragment_compiler);
//     }
// }
