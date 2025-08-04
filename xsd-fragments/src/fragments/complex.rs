//! This module contains the logic for compiling complex types into fragments.
//!
//! It is naturally dependent on the simple type compiler, as complex types can contain simple types.

use std::{any::type_name, collections::VecDeque, ops::Deref};

use crate::{
    fragments::{
        simple::{self, SimpleFragmentEquivalent, SimpleTypeFragmentCompiler},
        FragmentAccess, FragmentCollection, FragmentIdx, HasFragmentCollection, NamespaceIdx,
    },
    NamedOrAnonymous,
};
use xmlity::{ExpandedName, LocalName, XmlNamespace};

use xsd::{ns, xs};

/// Extension trait for [`ExpandedName`] to handle default namespaces.
pub trait XmlNamespaceExt<'a> {
    /// Sets a default namespace if none is present.
    fn with_default_namespace<F: FnOnce() -> XmlNamespace<'a>>(self, f: F) -> Self;
}

impl<'a> XmlNamespaceExt<'a> for ExpandedName<'a> {
    fn with_default_namespace<F: FnOnce() -> XmlNamespace<'a>>(self, f: F) -> Self {
        let (local_name, mut namespace) = self.into_parts();

        namespace = Some(namespace.unwrap_or_else(f));

        ExpandedName::new(local_name.into_owned(), namespace)
    }
}

/// Identifier for type definition particles in complex types.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TypeDefParticleId {
    /// Group reference particle.
    Group(FragmentIdx<GroupRefFragment>),
    /// All particle.
    All(FragmentIdx<AllFragment>),
    /// Sequence particle.
    Sequence(FragmentIdx<SequenceFragment>),
    /// Choice particle.
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

/// Fragment representing a complex type extension.
#[derive(Debug, Clone, PartialEq)]
pub struct ExtensionFragment {
    /// The base type being extended.
    pub base: ExpandedName<'static>,
    /// Optional content particle.
    pub content_fragment: Option<TypeDefParticleId>,
    /// Attribute declarations for this extension.
    pub attribute_declarations: FragmentIdx<AttributeDeclarationsFragment>,
    /// Assertions for this extension.
    pub assertions: FragmentIdx<AssertionsFragment>,
}

/// Fragment representing a complex type restriction.
#[derive(Debug, Clone, PartialEq)]
pub struct RestrictionFragment {
    /// The base type being restricted.
    pub base: ExpandedName<'static>,
    /// Optional content particle.
    pub content_fragment: Option<TypeDefParticleId>,
    /// Attribute declarations for this restriction.
    pub attribute_declarations: FragmentIdx<AttributeDeclarationsFragment>,
    /// Assertions for this restriction.
    pub assertions: FragmentIdx<AssertionsFragment>,
}

/// Identifier for attribute declarations, either direct attributes or attribute group references.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AttributeDeclarationId {
    /// Direct attribute declaration.
    Attribute(FragmentIdx<LocalAttributeFragment>),
    /// Attribute group reference.
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

/// How an attribute is used in a complex type.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum AttributeUse {
    /// Attribute is required.
    Required,
    /// Attribute is optional.
    #[default]
    Optional,
    /// Attribute is prohibited.
    Prohibited,
}

impl From<xs::types::attribute_items::UseValue> for AttributeUse {
    fn from(value: xs::types::attribute_items::UseValue) -> Self {
        match value {
            xs::types::attribute_items::UseValue::Prohibited => Self::Prohibited,
            xs::types::attribute_items::UseValue::Optional => Self::Optional,
            xs::types::attribute_items::UseValue::Required => Self::Required,
        }
    }
}

impl From<AttributeUse> for xs::types::attribute_items::UseValue {
    fn from(value: AttributeUse) -> Self {
        match value {
            AttributeUse::Prohibited => xs::types::attribute_items::UseValue::Prohibited,
            AttributeUse::Optional => xs::types::attribute_items::UseValue::Optional,
            AttributeUse::Required => xs::types::attribute_items::UseValue::Required,
        }
    }
}

/// Fragment for a declared attribute with a local name.
#[derive(Debug, Clone, PartialEq)]
pub struct DeclaredAttributeFragment {
    /// Local name of the attribute.
    pub name: LocalName<'static>,
    /// Type of the attribute.
    pub type_: Option<NamedOrAnonymous<FragmentIdx<simple::SimpleTypeRootFragment>>>,
}

/// Fragment for a reference to a top-level attribute.
#[derive(Debug, Clone, PartialEq)]
pub struct ReferenceAttributeFragment {
    /// Reference to the top-level attribute.
    pub ref_: ExpandedName<'static>,
}

/// Type mode for local attribute fragments.
#[derive(Debug, Clone, PartialEq)]
pub enum LocalAttributeFragmentTypeMode {
    /// Declared attribute with local name and type.
    Declared(DeclaredAttributeFragment),
    /// Reference to a top-level attribute.
    Reference(ReferenceAttributeFragment),
}

/// Fragment representing a local attribute declaration.
#[derive(Debug, Clone, PartialEq)]
pub struct LocalAttributeFragment {
    /// How the attribute is declared (local or reference).
    pub type_mode: LocalAttributeFragmentTypeMode,
    /// How the attribute is used.
    pub use_: Option<AttributeUse>,
    /// Default value for the attribute.
    pub default: Option<String>,
}

/// Fragment representing a top-level attribute declaration.
#[derive(Debug, Clone, PartialEq)]
pub struct TopLevelAttributeFragment {
    /// Name of the attribute.
    pub name: LocalName<'static>,
    /// Type of the attribute.
    pub type_: Option<NamedOrAnonymous<FragmentIdx<simple::SimpleTypeRootFragment>>>,
}

/// Fragment representing a top-level group declaration.
#[derive(Debug, Clone, PartialEq)]
pub struct TopLevelGroupFragment {
    /// Name of the group.
    pub name: LocalName<'static>,
    /// Content of the group.
    pub content: NamedGroupTypeContentId,
}

/// Fragment representing a reference to an attribute group.
#[derive(Debug, Clone, PartialEq)]
pub struct AttributeGroupRefFragment {
    /// Reference to the attribute group.
    pub ref_: ExpandedName<'static>,
}

/// Fragment representing a top-level attribute group declaration.
#[derive(Debug, Clone, PartialEq)]
pub struct TopLevelAttributeGroupFragment {
    /// Name of the attribute group.
    pub name: LocalName<'static>,
    /// Attribute declarations in this group.
    pub attr_decls: FragmentIdx<AttributeDeclarationsFragment>,
}

/// Fragment containing attribute declarations and any-attribute.
#[derive(Debug, Clone, PartialEq)]
pub struct AttributeDeclarationsFragment {
    /// List of attribute declarations.
    pub declarations: VecDeque<AttributeDeclarationId>,
    /// Optional any-attribute declaration.
    pub any_attribute: Option<FragmentIdx<AnyAttributeFragment>>,
}

/// Fragment representing simple content in a complex type.
#[derive(Debug, Clone, PartialEq)]
pub struct SimpleContentFragment {
    /// The content fragment (extension or restriction).
    pub content_fragment: SimpleContentChildId,
}

/// Fragment representing a simple content extension.
#[derive(Debug, Clone, PartialEq)]
pub struct SimpleExtensionFragment {
    /// The base type being extended.
    pub base: ExpandedName<'static>,
    /// Attribute declarations for this extension.
    pub attribute_declarations: FragmentIdx<AttributeDeclarationsFragment>,
    /// Assertions for this extension.
    pub assertions: FragmentIdx<AssertionsFragment>,
}

/// Fragment representing a simple content restriction.
#[derive(Debug, Clone, PartialEq)]
pub struct SimpleRestrictionFragment {
    /// The base type being restricted.
    pub base: ExpandedName<'static>,
    /// Attribute declarations for this restriction.
    pub attribute_declarations: FragmentIdx<AttributeDeclarationsFragment>,
    /// Assertions for this restriction.
    pub assertions: FragmentIdx<AssertionsFragment>,
}

/// Identifier for simple content child fragments.
#[derive(Debug, Clone, PartialEq)]
pub enum SimpleContentChildId {
    /// Simple content extension.
    Extension(FragmentIdx<SimpleExtensionFragment>),
    /// Simple content restriction.
    Restriction(FragmentIdx<SimpleRestrictionFragment>),
}

/// Fragment representing complex content in a complex type.
#[derive(Debug, Clone, PartialEq)]
pub struct ComplexContentFragment {
    /// Whether the content is mixed.
    pub mixed: Option<bool>,
    /// The content fragment (extension or restriction).
    pub content_fragment: ComplexContentChildId,
}

/// Identifier for complex content child fragments.
#[derive(Debug, Clone, PartialEq)]
pub enum ComplexContentChildId {
    /// Complex content extension.
    Extension(FragmentIdx<ExtensionFragment>),
    /// Complex content restriction.
    Restriction(FragmentIdx<RestrictionFragment>),
}

/// Fragment for a declared element with local name and type.
#[derive(Debug, Clone, PartialEq)]
pub struct DeclaredElementFragment {
    /// Local name of the element.
    pub name: LocalName<'static>,
    /// Type of the element.
    pub type_: NamedOrAnonymous<ElementTypeContentId>,
}

/// Fragment for a reference to a top-level element.
#[derive(Debug, Clone, PartialEq)]
pub struct ReferenceElementFragment {
    /// Reference to the top-level element.
    pub ref_: ExpandedName<'static>,
}

/// Type of local element fragment.
#[derive(Debug, Clone, PartialEq)]
pub enum LocalElementFragmentType {
    /// Local element declaration.
    Local(DeclaredElementFragment),
    /// Reference to top-level element.
    Reference(ReferenceElementFragment),
}

/// Fragment representing a local element declaration.
#[derive(Debug, Clone, PartialEq)]
pub struct LocalElementFragment {
    /// Minimum number of occurrences.
    pub min_occurs: Option<usize>,
    /// Maximum number of occurrences.
    pub max_occurs: Option<AllNNI>,
    /// Type of the element (local or reference).
    pub type_: LocalElementFragmentType,
}

/// Fragment representing a top-level element declaration.
#[derive(Debug, Clone, PartialEq)]
pub struct TopLevelElementFragment {
    /// Name of the element.
    pub name: LocalName<'static>,
    /// Type of the element (named or anonymous).
    pub type_: Option<NamedOrAnonymous<ElementTypeContentId>>,
    /// List of substitution groups this element belongs to.
    pub substitution_groups: Vec<ExpandedName<'static>>,
    /// Whether the element is abstract.
    pub abstract_: bool,
}

/// Fragment representing a reference to a group.
#[derive(Debug, Clone, PartialEq)]
pub struct GroupRefFragment {
    /// Minimum number of occurrences.
    pub min_occurs: Option<usize>,
    /// Maximum number of occurrences.
    pub max_occurs: Option<AllNNI>,
    /// Reference to the group.
    pub ref_: ExpandedName<'static>,
}

/// Fragment representing an "all" compositor.
#[derive(Debug, Clone, PartialEq)]
pub struct AllFragment {
    /// Minimum number of occurrences.
    pub min_occurs: Option<usize>,
    /// Maximum number of occurrences.
    pub max_occurs: Option<AllNNI>,
    /// Child particles.
    pub fragments: VecDeque<NestedParticleId>,
}

/// Fragment representing a "choice" compositor.
#[derive(Debug, Clone, PartialEq)]
pub struct ChoiceFragment {
    /// Minimum number of occurrences.
    pub min_occurs: Option<usize>,
    /// Maximum number of occurrences.
    pub max_occurs: Option<AllNNI>,
    /// Child particles.
    pub fragments: VecDeque<NestedParticleId>,
}

/// Represents the maximum occurrence of types or elements
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub enum AllNNI {
    /// The occurrence is unbounded.
    Unbounded,

    /// The occurrence is bound to the specified limit.
    Bounded(usize),
}

impl Default for AllNNI {
    fn default() -> Self {
        Self::Bounded(1)
    }
}

impl From<xs::types::AllNNI> for AllNNI {
    fn from(value: xs::types::AllNNI) -> Self {
        AllNNI::from(&value)
    }
}

impl<'a> From<&'a xs::types::AllNNI> for AllNNI {
    fn from(value: &'a xs::types::AllNNI) -> Self {
        match value {
            xs::types::all_nni_items::AllNNI::NonNegativeInteger(a) => Self::Bounded(**a),
            xs::types::all_nni_items::AllNNI::Variant0(variant0) => match **variant0 {
                xs::types::all_nni_items::all_nni_variants::Variant0::Unbounded => Self::Unbounded,
            },
        }
    }
}

impl From<AllNNI> for xs::types::AllNNI {
    fn from(value: AllNNI) -> Self {
        match value {
            AllNNI::Unbounded => xs::types::AllNNI::Variant0(Box::new(
                xs::types::all_nni_items::all_nni_variants::Variant0::Unbounded,
            )),
            AllNNI::Bounded(a) => xs::types::AllNNI::NonNegativeInteger(Box::new(a)),
        }
    }
}

impl From<AllNNI> for xs::types::all_items::MaxOccursValue {
    fn from(value: AllNNI) -> Self {
        match value {
            AllNNI::Bounded(n) => xs::types::all_items::MaxOccursValue::from(n),
            AllNNI::Unbounded => xs::types::all_items::MaxOccursValue::from(
                xs::types::all_items::max_occurs_value_variants::Variant0::Unbounded,
            ),
        }
    }
}

/// Fragment representing a "sequence" compositor.
#[derive(Debug, Clone, PartialEq)]
pub struct SequenceFragment {
    /// Optional identifier.
    pub id: Option<String>,
    /// Minimum number of occurrences.
    pub min_occurs: Option<usize>,
    /// Maximum number of occurrences.
    pub max_occurs: Option<AllNNI>,
    /// Child particles.
    pub fragments: VecDeque<NestedParticleId>,
}

/// Identifier for complex type content models.
#[derive(Debug, Clone, PartialEq)]
pub enum ComplexTypeModelId {
    /// Simple content model.
    SimpleContent(FragmentIdx<SimpleContentFragment>),
    /// Complex content model.
    ComplexContent(FragmentIdx<ComplexContentFragment>),
    /// Other content model with particles and attributes.
    Other {
        //TODO: Add open content
        // open_content: Option<OpenContentId>,
        /// Optional particle content.
        particle: Option<TypeDefParticleId>,
        /// Attribute declarations.
        attr_decls: FragmentIdx<AttributeDeclarationsFragment>,
        /// Assertions for the content model.
        assertions: FragmentIdx<AssertionsFragment>,
    },
}

/// Fragment representing a complex type definition.
#[derive(Debug, Clone, PartialEq)]
pub struct ComplexTypeRootFragment {
    /// Optional name for named types.
    pub name: Option<LocalName<'static>>,
    /// Content model of the complex type.
    pub content: ComplexTypeModelId,
    /// Whether content is mixed.
    pub mixed: Option<bool>,
    /// Whether the type is abstract.
    pub abstract_: Option<bool>,
}

/// Process contents for any wildcard.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AnyProcessContents {
    /// Skip processing of contents.
    Skip,
    /// Lax processing of contents.
    Lax,
    /// Strict processing of contents.
    Strict,
}

impl From<xs::any_items::ProcessContentsValue> for AnyProcessContents {
    fn from(value: xs::any_items::ProcessContentsValue) -> Self {
        match value {
            xs::any_items::ProcessContentsValue::Skip => Self::Skip,
            xs::any_items::ProcessContentsValue::Lax => Self::Lax,
            xs::any_items::ProcessContentsValue::Strict => Self::Strict,
        }
    }
}

impl From<AnyProcessContents> for xs::any_items::ProcessContentsValue {
    fn from(value: AnyProcessContents) -> Self {
        match value {
            AnyProcessContents::Skip => Self::Skip,
            AnyProcessContents::Lax => Self::Lax,
            AnyProcessContents::Strict => Self::Strict,
        }
    }
}

/// Fragment representing any element wildcard.
#[derive(Debug, Clone, PartialEq)]
pub struct AnyFragment {
    /// Optional identifier for the fragment.
    pub id: Option<String>,
    /// Process contents for the any element.
    pub process_contents: Option<AnyProcessContents>,
}

/// Process contents for any attribute wildcard.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AnyAttributeProcessContents {
    /// Skip processing of attributes.
    Skip,
    /// Lax processing of attributes.
    Lax,
    /// Strict processing of attributes.
    Strict,
}

impl From<xs::any_attribute_items::ProcessContentsValue> for AnyAttributeProcessContents {
    fn from(value: xs::any_attribute_items::ProcessContentsValue) -> Self {
        match value {
            xs::any_attribute_items::ProcessContentsValue::Skip => Self::Skip,
            xs::any_attribute_items::ProcessContentsValue::Lax => Self::Lax,
            xs::any_attribute_items::ProcessContentsValue::Strict => Self::Strict,
        }
    }
}

impl From<AnyAttributeProcessContents> for xs::any_attribute_items::ProcessContentsValue {
    fn from(value: AnyAttributeProcessContents) -> Self {
        match value {
            AnyAttributeProcessContents::Skip => Self::Skip,
            AnyAttributeProcessContents::Lax => Self::Lax,
            AnyAttributeProcessContents::Strict => Self::Strict,
        }
    }
}

/// Fragment representing any attribute wildcard.
#[derive(Debug, Clone, PartialEq)]
pub struct AnyAttributeFragment {
    /// Optional identifier for the fragment.
    pub id: Option<String>,
    /// Process contents for the any attribute.
    pub process_contents: Option<AnyAttributeProcessContents>,
}

/// Fragment representing an assertion in a complex type.
#[derive(Debug, Clone, PartialEq)]
pub struct AssertionFragment {
    /// Optional identifier for the assertion.
    pub id: Option<String>,
    /// The assertion expression.
    pub test: Option<String>,
}

/// Fragment representing any attribute wildcard.
#[derive(Debug, Clone, PartialEq)]
pub struct AssertionsFragment {
    /// List of assertions in this fragment.
    pub assertions: VecDeque<FragmentIdx<AssertionFragment>>,
}

/// Complex type fragment compiler responsible for converting XSD complex types to fragment representations.
#[derive(Debug, Clone)]
pub struct ComplexTypeFragmentCompiler {
    namespace: XmlNamespace<'static>,
    /// The [`SimpleTypeFragmentCompiler`] for this complex type compiler.
    pub simple_type_compiler: SimpleTypeFragmentCompiler,
    complex_types: FragmentCollection<ComplexTypeRootFragment>,
    simple_restrictions: FragmentCollection<SimpleRestrictionFragment>,
    simple_extensions: FragmentCollection<SimpleExtensionFragment>,
    simple_contents: FragmentCollection<SimpleContentFragment>,
    restrictions: FragmentCollection<RestrictionFragment>,
    extensions: FragmentCollection<ExtensionFragment>,
    complex_contents: FragmentCollection<ComplexContentFragment>,
    group_refs: FragmentCollection<GroupRefFragment>,
    alls: FragmentCollection<AllFragment>,
    choices: FragmentCollection<ChoiceFragment>,
    sequences: FragmentCollection<SequenceFragment>,
    anys: FragmentCollection<AnyFragment>,
    elements: FragmentCollection<LocalElementFragment>,
    top_level_elements: FragmentCollection<TopLevelElementFragment>,
    local_attributes: FragmentCollection<LocalAttributeFragment>,
    top_level_attributes: FragmentCollection<TopLevelAttributeFragment>,
    attribute_group_refs: FragmentCollection<AttributeGroupRefFragment>,
    groups: FragmentCollection<TopLevelGroupFragment>,
    attribute_groups: FragmentCollection<TopLevelAttributeGroupFragment>,
    attribute_declarations: FragmentCollection<AttributeDeclarationsFragment>,
    any_attributes: FragmentCollection<AnyAttributeFragment>,
    assertions: FragmentCollection<AssertionFragment>,
    assertion_groups: FragmentCollection<AssertionsFragment>,
}

impl HasFragmentCollection<ComplexTypeRootFragment> for ComplexTypeFragmentCompiler {
    fn get_fragment_collection(&self) -> &FragmentCollection<ComplexTypeRootFragment> {
        &self.complex_types
    }
    fn get_fragment_collection_mut(&mut self) -> &mut FragmentCollection<ComplexTypeRootFragment> {
        &mut self.complex_types
    }
}

impl HasFragmentCollection<SimpleRestrictionFragment> for ComplexTypeFragmentCompiler {
    fn get_fragment_collection(&self) -> &FragmentCollection<SimpleRestrictionFragment> {
        &self.simple_restrictions
    }
    fn get_fragment_collection_mut(
        &mut self,
    ) -> &mut FragmentCollection<SimpleRestrictionFragment> {
        &mut self.simple_restrictions
    }
}

impl HasFragmentCollection<SimpleExtensionFragment> for ComplexTypeFragmentCompiler {
    fn get_fragment_collection(&self) -> &FragmentCollection<SimpleExtensionFragment> {
        &self.simple_extensions
    }
    fn get_fragment_collection_mut(&mut self) -> &mut FragmentCollection<SimpleExtensionFragment> {
        &mut self.simple_extensions
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

impl HasFragmentCollection<AnyAttributeFragment> for ComplexTypeFragmentCompiler {
    fn get_fragment_collection(&self) -> &FragmentCollection<AnyAttributeFragment> {
        &self.any_attributes
    }
    fn get_fragment_collection_mut(&mut self) -> &mut FragmentCollection<AnyAttributeFragment> {
        &mut self.any_attributes
    }
}

impl HasFragmentCollection<AssertionsFragment> for ComplexTypeFragmentCompiler {
    fn get_fragment_collection(&self) -> &FragmentCollection<AssertionsFragment> {
        &self.assertion_groups
    }
    fn get_fragment_collection_mut(&mut self) -> &mut FragmentCollection<AssertionsFragment> {
        &mut self.assertion_groups
    }
}

impl HasFragmentCollection<AssertionFragment> for ComplexTypeFragmentCompiler {
    fn get_fragment_collection(&self) -> &FragmentCollection<AssertionFragment> {
        &self.assertions
    }
    fn get_fragment_collection_mut(&mut self) -> &mut FragmentCollection<AssertionFragment> {
        &mut self.assertions
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
    /// Creates a new [`ComplexTypeFragmentCompiler`] with the given namespace and namespace index.
    pub fn new(namespace: XmlNamespace<'static>, namespace_idx: NamespaceIdx) -> Self {
        Self::new_with_simple_compiler(
            namespace.clone(),
            namespace_idx,
            SimpleTypeFragmentCompiler::new(namespace, namespace_idx),
        )
    }

    /// Creates a new [`ComplexTypeFragmentCompiler`] with a given [`SimpleTypeFragmentCompiler`].
    pub fn new_with_simple_compiler(
        namespace: XmlNamespace<'static>,
        namespace_idx: NamespaceIdx,
        simple_type_compiler: SimpleTypeFragmentCompiler,
    ) -> Self {
        Self {
            namespace,
            simple_type_compiler,
            complex_types: FragmentCollection::new(namespace_idx),
            simple_restrictions: FragmentCollection::new(namespace_idx),
            simple_extensions: FragmentCollection::new(namespace_idx),
            simple_contents: FragmentCollection::new(namespace_idx),
            restrictions: FragmentCollection::new(namespace_idx),
            extensions: FragmentCollection::new(namespace_idx),
            complex_contents: FragmentCollection::new(namespace_idx),
            group_refs: FragmentCollection::new(namespace_idx),
            alls: FragmentCollection::new(namespace_idx),
            choices: FragmentCollection::new(namespace_idx),
            sequences: FragmentCollection::new(namespace_idx),
            anys: FragmentCollection::new(namespace_idx),
            elements: FragmentCollection::new(namespace_idx),
            top_level_elements: FragmentCollection::new(namespace_idx),
            local_attributes: FragmentCollection::new(namespace_idx),
            top_level_attributes: FragmentCollection::new(namespace_idx),
            attribute_group_refs: FragmentCollection::new(namespace_idx),
            groups: FragmentCollection::new(namespace_idx),
            attribute_groups: FragmentCollection::new(namespace_idx),
            attribute_declarations: FragmentCollection::new(namespace_idx),
            any_attributes: FragmentCollection::new(namespace_idx),
            assertions: FragmentCollection::new(namespace_idx),
            assertion_groups: FragmentCollection::new(namespace_idx),
        }
    }
}

impl AsMut<SimpleTypeFragmentCompiler> for ComplexTypeFragmentCompiler {
    fn as_mut(&mut self) -> &mut SimpleTypeFragmentCompiler {
        &mut self.simple_type_compiler
    }
}

impl AsMut<ComplexTypeFragmentCompiler> for ComplexTypeFragmentCompiler {
    fn as_mut(&mut self) -> &mut ComplexTypeFragmentCompiler {
        self
    }
}

impl AsRef<SimpleTypeFragmentCompiler> for ComplexTypeFragmentCompiler {
    fn as_ref(&self) -> &SimpleTypeFragmentCompiler {
        &self.simple_type_compiler
    }
}

impl AsRef<ComplexTypeFragmentCompiler> for ComplexTypeFragmentCompiler {
    fn as_ref(&self) -> &ComplexTypeFragmentCompiler {
        self
    }
}

/// Error types for complex fragment operations.
#[derive(Debug, Clone)]
pub enum Error {
    /// Error from simple type processing.
    Simple(simple::Error),
    /// Name is missing in top-level declaration.
    NameMissingInTopLevel,
    /// Element type is invalid, both type and type_choice are present.
    TypeAttributeAndTypeContentBothPresent {
        /// Name of the element with conflicting type representations.
        name: LocalName<'static>,
    },
    /// Substitution group is not supported.
    SubstitutionGroupNotSupported {
        /// Name of the element with unsupported substitution group.
        fragment_type: &'static str,
    },
}

impl From<simple::Error> for Error {
    fn from(err: simple::Error) -> Self {
        Error::Simple(err)
    }
}

/// Trait for types that can be converted to and from [`ComplexTypeFragmentCompiler`] fragments.
pub trait ComplexFragmentEquivalent: Sized {
    /// The identifier type for the fragment.
    type FragmentId;

    /// Converts this type to complex fragments.
    fn to_complex_fragments<T: AsMut<ComplexTypeFragmentCompiler>>(
        &self,
        compiler: T,
    ) -> Result<Self::FragmentId, Error>;

    /// Reconstructs this type from complex fragments.
    fn from_complex_fragments<T: AsRef<ComplexTypeFragmentCompiler>>(
        compiler: T,
        fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error>;
}

/// Identifier for element type content (simple or complex type).
#[derive(Debug, Clone, PartialEq)]
pub enum ElementTypeContentId {
    /// Simple type content.
    SimpleType(FragmentIdx<simple::SimpleTypeRootFragment>),
    /// Complex type content.
    ComplexType(FragmentIdx<ComplexTypeRootFragment>),
}

impl ComplexFragmentEquivalent for xs::types::top_level_element_items::Type {
    type FragmentId = ElementTypeContentId;

    fn to_complex_fragments<T: AsMut<ComplexTypeFragmentCompiler>>(
        &self,
        mut compiler: T,
    ) -> Result<Self::FragmentId, Error> {
        match self {
            xs::types::top_level_element_items::Type::SimpleType(local_simple_type) => {
                let simple_type_fragment =
                    local_simple_type.to_simple_fragments(compiler.as_mut())?;

                Ok(ElementTypeContentId::SimpleType(simple_type_fragment))
            }
            xs::types::top_level_element_items::Type::ComplexType(local_complex_type) => {
                let complex_type_fragment = local_complex_type.to_complex_fragments(compiler)?;

                Ok(ElementTypeContentId::ComplexType(complex_type_fragment))
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
    ) -> Result<Self::FragmentId, Error> {
        let mut compiler = compiler.as_mut();

        match self {
            xs::types::local_element_items::Type::SimpleType(local_simple_type) => {
                let simple_type_fragment = local_simple_type
                    .to_simple_fragments(&mut compiler)
                    .expect("Failed to convert simple type to fragments");

                Ok(ElementTypeContentId::SimpleType(simple_type_fragment))
            }
            xs::types::local_element_items::Type::ComplexType(local_complex_type) => {
                let complex_type_fragment = local_complex_type.to_complex_fragments(compiler)?;

                Ok(ElementTypeContentId::ComplexType(complex_type_fragment))
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
    ) -> Result<Self::FragmentId, Error> {
        let mut compiler = compiler.as_mut();

        let max_occurs = self.max_occurs.clone().map(|a| AllNNI::from(*a));
        let min_occurs = self.min_occurs;

        let type_ = if let Some(ref_) = self.ref_.as_ref() {
            LocalElementFragmentType::Reference(ReferenceElementFragment {
                ref_: ref_
                    .0
                    .clone()
                    .with_default_namespace(|| compiler.namespace.clone()),
            })
        } else {
            let name = self
                .name
                .clone()
                .expect("If ref is none, name should be Some");

            let type_ = if let Some(type_) = self.type_attribute.as_ref() {
                NamedOrAnonymous::Named(
                    type_
                        .0
                        .clone()
                        .with_default_namespace(|| compiler.namespace.clone()),
                )
            } else {
                let type_choice = self
                    .type_
                    .as_ref()
                    .expect("If ref is none and type is none, type_choice should be Some");
                let content_type = type_choice.to_complex_fragments(&mut compiler)?;

                NamedOrAnonymous::Anonymous(content_type)
            };

            LocalElementFragmentType::Local(DeclaredElementFragment { name, type_ })
        };

        Ok(compiler.push_fragment(LocalElementFragment {
            type_,
            max_occurs,
            min_occurs,
        }))
    }

    fn from_complex_fragments<T: AsRef<ComplexTypeFragmentCompiler>>(
        compiler: T,
        fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        let compiler = compiler.as_ref();

        let fragment = compiler
            .get_fragment(fragment_id)
            .expect("Fragment not found in compiler.");

        let element_builder = xs::types::LocalElement::builder()
            .maybe_min_occurs(fragment.min_occurs)
            .maybe_max_occurs(fragment.max_occurs.map(From::from).map(Box::new));

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
                .any_attributes(ns::AnyAttributes::default())
                .build()),
            LocalElementFragmentType::Reference(fragment) => Ok(element_builder
                .ref_(xs::types::QName(fragment.ref_.clone()))
                .any_attributes(ns::AnyAttributes::default())
                .build()),
        }
    }
}

impl ComplexFragmentEquivalent for xs::types::TopLevelElement {
    type FragmentId = FragmentIdx<TopLevelElementFragment>;

    fn to_complex_fragments<T: AsMut<ComplexTypeFragmentCompiler>>(
        &self,
        mut compiler: T,
    ) -> Result<Self::FragmentId, Error> {
        let mut compiler = compiler.as_mut();

        let name = self.name.clone();

        let type_ = match (self.type_attribute.as_ref(), self.type_.as_ref()) {
            (Some(type_), None) => Some(NamedOrAnonymous::Named(
                type_
                    .0
                    .clone()
                    .with_default_namespace(|| compiler.namespace.clone()),
            )),
            (None, Some(type_choice)) => {
                let content_type = type_choice.to_complex_fragments(&mut compiler)?;
                Some(NamedOrAnonymous::Anonymous(content_type))
            }
            (Some(_), Some(_)) => {
                return Err(Error::TypeAttributeAndTypeContentBothPresent { name })
            }
            (None, None) => None,
        };

        let substitution_groups = self
            .substitution_group
            .iter()
            .flat_map(|list| list.0.iter())
            .map(|expanded_name| expanded_name.0.clone())
            .collect();

        Ok(compiler.push_fragment(TopLevelElementFragment {
            name,
            type_,
            substitution_groups,
            abstract_: self.abstract_.unwrap_or_default(),
        }))
    }

    fn from_complex_fragments<T: AsRef<ComplexTypeFragmentCompiler>>(
        compiler: T,
        fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        let compiler = compiler.as_ref();

        let fragment = compiler
            .get_fragment(fragment_id)
            .expect("Fragment not found in compiler.");

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
            .any_attributes(ns::AnyAttributes::default())
            .build())
    }
}

impl ComplexFragmentEquivalent for xs::types::GroupRef {
    type FragmentId = FragmentIdx<GroupRefFragment>;

    fn to_complex_fragments<T: AsMut<ComplexTypeFragmentCompiler>>(
        &self,
        mut compiler: T,
    ) -> Result<Self::FragmentId, Error> {
        let compiler = compiler.as_mut();

        let ref_ = self
            .ref_
            .0
            .clone()
            .with_default_namespace(|| compiler.namespace.clone());

        Ok(compiler.push_fragment(GroupRefFragment {
            min_occurs: self.min_occurs,
            max_occurs: self.max_occurs.clone().map(|a| AllNNI::from(*a)),
            ref_,
        }))
    }

    fn from_complex_fragments<T: AsRef<ComplexTypeFragmentCompiler>>(
        compiler: T,
        fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        let compiler = compiler.as_ref();

        let fragment = compiler
            .get_fragment(fragment_id)
            .expect("Fragment not found in compiler.");

        Ok(xs::types::GroupRef {
            id: None,
            min_occurs: fragment.min_occurs,
            max_occurs: fragment.max_occurs.map(From::from).map(Box::new),
            ref_: xs::types::QName(fragment.ref_.clone()),
            annotation: None,
            any_attributes: ns::AnyAttributes::default(),
        })
    }
}

impl ComplexFragmentEquivalent for xs::Any {
    type FragmentId = FragmentIdx<AnyFragment>;

    fn to_complex_fragments<T: AsMut<ComplexTypeFragmentCompiler>>(
        &self,
        mut compiler: T,
    ) -> Result<Self::FragmentId, Error> {
        let compiler = compiler.as_mut();

        let xs::Any::Any(any) = self else {
            return Err(Error::SubstitutionGroupNotSupported {
                fragment_type: type_name::<Self>(),
            });
        };

        Ok(compiler.push_fragment(AnyFragment {
            id: any.id.clone(),
            process_contents: any.process_contents.map(AnyProcessContents::from),
        }))
    }

    fn from_complex_fragments<T: AsRef<ComplexTypeFragmentCompiler>>(
        compiler: T,
        fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        let compiler = compiler.as_ref();

        let fragment = compiler
            .get_fragment(fragment_id)
            .expect("Fragment not found in compiler.");

        Ok(xs::Any::from(
            xs::any_items::Any::builder()
                .maybe_id(fragment.id.clone())
                .maybe_process_contents(fragment.process_contents.map(AnyProcessContents::into))
                .build(),
        ))
    }
}

/// Identifier for nested particles within complex type content.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NestedParticleId {
    /// Element particle.
    Element(FragmentIdx<LocalElementFragment>),
    /// Group reference particle.
    Group(FragmentIdx<GroupRefFragment>),
    /// Choice particle.
    Choice(FragmentIdx<ChoiceFragment>),
    /// Sequence particle.
    Sequence(FragmentIdx<SequenceFragment>),
    /// Any element wildcard particle.
    Any(FragmentIdx<AnyFragment>),
}

impl TryFrom<TypeDefParticleId> for NestedParticleId {
    type Error = FragmentIdx<AllFragment>;

    fn try_from(value: TypeDefParticleId) -> Result<Self, Self::Error> {
        match value {
            TypeDefParticleId::Group(fragment_idx) => Ok(Self::Group(fragment_idx)),
            TypeDefParticleId::All(fragment_idx) => return Err(fragment_idx),
            TypeDefParticleId::Choice(fragment_idx) => Ok(Self::Choice(fragment_idx)),
            TypeDefParticleId::Sequence(fragment_idx) => Ok(Self::Sequence(fragment_idx)),
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
    ) -> Result<Self::FragmentId, Error> {
        let compiler: &mut ComplexTypeFragmentCompiler = compiler.as_mut();

        use xs::groups::NestedParticle;

        match self {
            NestedParticle::Element(local_element) => local_element
                .to_complex_fragments(compiler)
                .map(NestedParticleId::Element),
            NestedParticle::Group(group_type) => group_type
                .to_complex_fragments(compiler)
                .map(NestedParticleId::Group),
            NestedParticle::Choice(choice_type) => choice_type
                .to_complex_fragments(compiler)
                .map(NestedParticleId::Choice),
            NestedParticle::Sequence(sequence_type) => sequence_type
                .to_complex_fragments(compiler)
                .map(NestedParticleId::Sequence),
            NestedParticle::Any(any) => any
                .to_complex_fragments(compiler)
                .map(NestedParticleId::Any),
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
        mut compiler: T,
    ) -> Result<Self::FragmentId, Error> {
        let compiler: &mut ComplexTypeFragmentCompiler = compiler.as_mut();

        match self {
            xs::groups::all_model_items::Child1::Element(local_element) => local_element
                .to_complex_fragments(compiler)
                .map(NestedParticleId::Element),
            xs::groups::all_model_items::Child1::Any(any) => any
                .to_complex_fragments(compiler)
                .map(NestedParticleId::Any),
            xs::groups::all_model_items::Child1::Group {
                id: _,
                ref_,
                min_occurs,
                max_occurs,
                annotation: _,
            } => Ok(compiler
                .push_fragment(GroupRefFragment {
                    min_occurs: *min_occurs,
                    max_occurs: max_occurs.map(AllNNI::Bounded),
                    ref_: ref_
                        .0
                        .clone()
                        .with_default_namespace(|| compiler.namespace.clone()),
                })
                .into()),
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
                    .map(Box::new)
                    .map(xs::groups::all_model_items::Child1::Element)
            }
            NestedParticleId::Any(fragment_idx) => {
                xs::Any::from_complex_fragments(compiler, fragment_idx)
                    .map(Box::new)
                    .map(xs::groups::all_model_items::Child1::Any)
            }
            NestedParticleId::Group(fragment_idx) => {
                let fragment = compiler
                    .get_fragment(fragment_idx)
                    .expect("Fragment not found in compiler.");

                Ok(xs::groups::all_model_items::Child1::Group {
                    id: None,
                    ref_: xs::types::QName(fragment.ref_.clone()),
                    min_occurs: fragment.min_occurs,
                    max_occurs: fragment.max_occurs.map(|a| match a {
                        AllNNI::Bounded(n) => n,
                        AllNNI::Unbounded => panic!("Unbounded not allowed in all groups"),
                    }),
                    annotation: None,
                })
            }
            _ => {
                return Err(Error::SubstitutionGroupNotSupported {
                    fragment_type: type_name::<Self>(),
                });
            }
        }
    }
}

impl ComplexFragmentEquivalent for xs::All {
    type FragmentId = FragmentIdx<AllFragment>;

    fn to_complex_fragments<T: AsMut<ComplexTypeFragmentCompiler>>(
        &self,
        mut compiler: T,
    ) -> Result<Self::FragmentId, Error> {
        let xs::All::All(all) = self else {
            return Err(Error::SubstitutionGroupNotSupported {
                fragment_type: type_name::<Self>(),
            });
        };

        let mut compiler = compiler.as_mut();

        let fragments = all
            .all_model
            .child_1
            .iter()
            .map(|content| content.to_complex_fragments(&mut compiler))
            .collect::<Result<_, _>>()?;

        let all = AllFragment {
            min_occurs: None,
            max_occurs: None,
            fragments,
        };

        Ok(compiler.push_fragment(all))
    }

    fn from_complex_fragments<T: AsRef<ComplexTypeFragmentCompiler>>(
        compiler: T,
        fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        let compiler = compiler.as_ref();

        let fragment = compiler
            .get_fragment(fragment_id)
            .expect("Fragment not found in compiler.");

        let child_1 = fragment
            .fragments
            .iter()
            .map(|nested_id| {
                xs::groups::all_model_items::Child1::from_complex_fragments(compiler, nested_id)
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(xs::All::All(Box::new(
            xs::types::All::builder()
                .maybe_min_occurs(
                    fragment
                        .min_occurs
                        .map(|min| min.try_into().expect("Invalid min occurs value")),
                )
                .maybe_max_occurs(fragment.max_occurs.map(|a| a.into()))
                .all_model(
                    xs::groups::AllModel::builder()
                        .child_1(child_1)
                        .build()
                        .into(),
                )
                .any_attributes(ns::AnyAttributes::default())
                .build(),
        )))
    }
}

impl ComplexFragmentEquivalent for xs::Choice {
    type FragmentId = FragmentIdx<ChoiceFragment>;

    fn to_complex_fragments<T: AsMut<ComplexTypeFragmentCompiler>>(
        &self,
        mut compiler: T,
    ) -> Result<Self::FragmentId, Error> {
        let mut compiler = compiler.as_mut();

        let xs::Choice::Choice(choice) = self else {
            return Err(Error::SubstitutionGroupNotSupported {
                fragment_type: type_name::<Self>(),
            });
        };

        let all = ChoiceFragment {
            min_occurs: choice.min_occurs,
            max_occurs: choice.max_occurs.clone().map(|a| AllNNI::from(*a)),
            fragments: choice
                .nested_particle
                .iter()
                .map(|content| content.to_complex_fragments(&mut compiler))
                .collect::<Result<_, _>>()?,
        };

        Ok(compiler.push_fragment(all))
    }

    fn from_complex_fragments<T: AsRef<ComplexTypeFragmentCompiler>>(
        compiler: T,
        fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        let compiler = compiler.as_ref();
        let fragment = compiler
            .get_fragment(fragment_id)
            .expect("Fragment not found in compiler.");

        Ok(xs::Choice::from(
            xs::types::ExplicitGroup::builder()
                .maybe_min_occurs(fragment.min_occurs)
                .maybe_max_occurs(fragment.max_occurs.map(From::from).map(Box::new))
                .nested_particle(
                    fragment
                        .fragments
                        .iter()
                        .map(|fragment| {
                            xs::groups::NestedParticle::from_complex_fragments(compiler, fragment)
                        })
                        .collect::<Result<_, _>>()?,
                )
                .any_attributes(ns::AnyAttributes::default())
                .build(),
        ))
    }
}

impl ComplexFragmentEquivalent for xs::Sequence {
    type FragmentId = FragmentIdx<SequenceFragment>;

    fn to_complex_fragments<T: AsMut<ComplexTypeFragmentCompiler>>(
        &self,
        mut compiler: T,
    ) -> Result<Self::FragmentId, Error> {
        let mut compiler = compiler.as_mut();

        let xs::Sequence::Sequence(sequence) = self else {
            return Err(Error::SubstitutionGroupNotSupported {
                fragment_type: type_name::<Self>(),
            });
        };

        let seq = SequenceFragment {
            id: sequence.id.clone(),
            min_occurs: sequence.min_occurs,
            max_occurs: sequence.max_occurs.clone().map(|a| AllNNI::from(*a)),
            fragments: sequence
                .nested_particle
                .iter()
                .map(|content| content.to_complex_fragments(&mut compiler))
                .collect::<Result<_, _>>()?,
        };

        Ok(compiler.push_fragment(seq))
    }

    fn from_complex_fragments<T: AsRef<ComplexTypeFragmentCompiler>>(
        compiler: T,
        fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        let compiler = compiler.as_ref();
        let fragment = compiler
            .get_fragment(fragment_id)
            .expect("Fragment not found in compiler.");

        Ok(xs::Sequence::from(
            xs::types::ExplicitGroup::builder()
                .maybe_min_occurs(fragment.min_occurs)
                .maybe_max_occurs(fragment.max_occurs.map(From::from).map(Box::new))
                .nested_particle(
                    fragment
                        .fragments
                        .iter()
                        .map(|fragment| {
                            xs::groups::NestedParticle::from_complex_fragments(compiler, fragment)
                        })
                        .collect::<Result<_, _>>()?,
                )
                .any_attributes(ns::AnyAttributes::default())
                .build(),
        ))
    }
}

impl ComplexFragmentEquivalent for xs::groups::TypeDefParticle {
    type FragmentId = TypeDefParticleId;

    fn to_complex_fragments<T: AsMut<ComplexTypeFragmentCompiler>>(
        &self,
        mut compiler: T,
    ) -> Result<Self::FragmentId, Error> {
        let compiler = compiler.as_mut();

        use xs::groups::TypeDefParticle;

        match self {
            TypeDefParticle::Group(group_ref) => group_ref
                .to_complex_fragments(compiler)
                .map(TypeDefParticleId::Group),
            TypeDefParticle::All(all) => all
                .to_complex_fragments(compiler)
                .map(TypeDefParticleId::All),
            TypeDefParticle::Choice(choice) => choice
                .to_complex_fragments(compiler)
                .map(TypeDefParticleId::Choice),
            TypeDefParticle::Sequence(sequence) => sequence
                .to_complex_fragments(compiler)
                .map(TypeDefParticleId::Sequence),
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

impl ComplexFragmentEquivalent for xs::types::Assertion {
    type FragmentId = FragmentIdx<AssertionFragment>;

    fn to_complex_fragments<T: AsMut<ComplexTypeFragmentCompiler>>(
        &self,
        mut compiler: T,
    ) -> Result<Self::FragmentId, Error> {
        let compiler = compiler.as_mut();

        let fragment = AssertionFragment {
            id: self.id.clone(),
            test: self.test.clone(),
        };

        Ok(compiler.push_fragment(fragment))
    }

    fn from_complex_fragments<T: AsRef<ComplexTypeFragmentCompiler>>(
        compiler: T,
        fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        let compiler = compiler.as_ref();

        let fragment = compiler
            .get_fragment(fragment_id)
            .expect("Fragment not found in compiler.");

        Ok(xs::types::Assertion::builder()
            .maybe_id(fragment.id.clone())
            .maybe_test(fragment.test.clone())
            .build())
    }
}

impl ComplexFragmentEquivalent for xs::groups::Assertions {
    type FragmentId = FragmentIdx<AssertionsFragment>;

    fn to_complex_fragments<T: AsMut<ComplexTypeFragmentCompiler>>(
        &self,
        mut compiler: T,
    ) -> Result<Self::FragmentId, Error> {
        let mut compiler = compiler.as_mut();

        let assertions = self
            .assert
            .iter()
            .map(|assertion| assertion.0.to_complex_fragments(&mut compiler))
            .collect::<Result<_, _>>()?;

        let root_fragment = AssertionsFragment { assertions };

        Ok(compiler.push_fragment(root_fragment))
    }

    fn from_complex_fragments<T: AsRef<ComplexTypeFragmentCompiler>>(
        compiler: T,
        fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        let compiler = compiler.as_ref();

        let fragment = compiler
            .get_fragment(fragment_id)
            .expect("Fragment not found in compiler.");

        let assertions = fragment
            .assertions
            .iter()
            .map(|assertion_id| {
                xs::types::Assertion::from_complex_fragments(compiler, assertion_id)
                    .map(xs::groups::assertions_items::Assert::from)
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(xs::groups::Assertions::builder()
            .assert(assertions)
            .build()
            .into())
    }
}

impl ComplexFragmentEquivalent for xs::types::ExtensionType {
    type FragmentId = FragmentIdx<ExtensionFragment>;

    /// This method expects all references to already be defined.
    fn to_complex_fragments<T: AsMut<ComplexTypeFragmentCompiler>>(
        &self,
        mut compiler: T,
    ) -> Result<Self::FragmentId, Error> {
        let mut compiler = compiler.as_mut();

        let base = self.base.0.clone();

        let content_fragment = self
            .type_def_particle
            .as_ref()
            .map(|content| content.to_complex_fragments(&mut compiler))
            .transpose()?;

        let attribute_declarations = self.attr_decls.to_complex_fragments(&mut compiler)?;

        let assertions = self.assertions.to_complex_fragments(&mut compiler)?;

        let root_fragment = ExtensionFragment {
            base,
            content_fragment,
            attribute_declarations,
            assertions,
        };

        Ok(compiler.push_fragment(root_fragment))
    }

    fn from_complex_fragments<T: AsRef<ComplexTypeFragmentCompiler>>(
        compiler: T,
        fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        let compiler = compiler.as_ref();

        let extension = compiler
            .get_fragment(fragment_id)
            .expect("Fragment not found in compiler.");

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

        let assertions =
            xs::groups::Assertions::from_complex_fragments(compiler, &extension.assertions)?;

        Ok(Self::builder()
            .base(xs::types::QName(extension.base.clone()))
            .maybe_type_def_particle(particle)
            .attr_decls(attr_decls.into())
            .assertions(assertions.into())
            .build())
    }
}

impl ComplexFragmentEquivalent for xs::types::ComplexRestrictionType {
    type FragmentId = FragmentIdx<RestrictionFragment>;

    fn to_complex_fragments<T: AsMut<ComplexTypeFragmentCompiler>>(
        &self,
        mut compiler: T,
    ) -> Result<Self::FragmentId, Error> {
        let mut compiler = compiler.as_mut();

        let base = self.base.0.clone();

        let content_fragment = self
            .child_1
            .as_ref()
            .map(|particle| {
                particle
                    .type_def_particle
                    .to_complex_fragments(&mut compiler)
            })
            .transpose()?;

        let attribute_declarations = self.attr_decls.to_complex_fragments(&mut compiler)?;

        let assertions = self.assertions.to_complex_fragments(&mut compiler)?;

        let root_fragment = RestrictionFragment {
            base,
            content_fragment,
            attribute_declarations,
            assertions,
        };

        Ok(compiler.push_fragment(root_fragment))
    }

    fn from_complex_fragments<T: AsRef<ComplexTypeFragmentCompiler>>(
        compiler: T,
        fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        let compiler = compiler.as_ref();

        let fragment = compiler
            .get_fragment(fragment_id)
            .expect("Fragment not found in compiler.");

        let particle = fragment
            .content_fragment
            .map(|a| xs::groups::TypeDefParticle::from_complex_fragments(compiler, &a))
            .transpose()?;

        let attr_decls = xs::groups::AttrDecls::from_complex_fragments(
            compiler,
            &fragment.attribute_declarations,
        )?;

        let assertions =
            xs::groups::Assertions::from_complex_fragments(compiler, &fragment.assertions)?;

        Ok(xs::types::ComplexRestrictionType::builder()
            .base(xs::types::QName(fragment.base.clone()))
            .maybe_child_1(particle.map(|particle| {
                xs::types::complex_restriction_type_items::Child1 {
                    open_content: None,
                    type_def_particle: Box::new(particle),
                }
            }))
            .attr_decls(attr_decls.into())
            .assertions(assertions.into())
            .any_attributes(ns::AnyAttributes::default())
            .build())
    }
}

impl ComplexFragmentEquivalent for xs::groups::attr_decls_items::Attribute {
    type FragmentId = AttributeDeclarationId;

    fn to_complex_fragments<T: AsMut<ComplexTypeFragmentCompiler>>(
        &self,
        mut compiler: T,
    ) -> Result<Self::FragmentId, Error> {
        let compiler = compiler.as_mut();
        use xs::groups::attr_decls_items::Attribute;

        match self {
            Attribute::Attribute(local) => Ok(AttributeDeclarationId::Attribute(
                local.to_complex_fragments(compiler)?,
            )),
            Attribute::AttributeGroup(group) => Ok(AttributeDeclarationId::AttributeGroupRef(
                group.to_complex_fragments(compiler)?,
            )),
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
    ) -> Result<Self::FragmentId, Error> {
        let mut compiler = compiler.as_mut();

        let type_mode = if let Some(ref ref_) = self.ref_ {
            LocalAttributeFragmentTypeMode::Reference(ReferenceAttributeFragment {
                ref_: ref_
                    .0
                    .clone()
                    .with_default_namespace(|| compiler.namespace.clone()),
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
                    NamedOrAnonymous::Anonymous(
                        simple_type
                            .to_simple_fragments(&mut compiler)
                            .expect("Failed to convert simple type to fragments"),
                    )
                })
            };

            LocalAttributeFragmentTypeMode::Declared(DeclaredAttributeFragment {
                name: name.clone(),
                type_,
            })
        };
        Ok(compiler.push_fragment(LocalAttributeFragment {
            type_mode,
            use_: self.use_.map(|a| a.into()),
            default: self.default.clone(),
        }))
    }

    fn from_complex_fragments<T: AsRef<ComplexTypeFragmentCompiler>>(
        compiler: T,
        fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        let compiler = compiler.as_ref();

        let fragment = compiler
            .get_fragment(fragment_id)
            .expect("Fragment not found in compiler.");

        match &fragment.type_mode {
            LocalAttributeFragmentTypeMode::Declared(local) => {
                let name = local.name.clone();
                let type_ = local.type_.as_ref().and_then(|a| match a {
                    NamedOrAnonymous::Named(ref_) => Some(xs::types::QName(ref_.clone())),
                    NamedOrAnonymous::Anonymous(_) => None,
                });
                let use_ = fragment.use_.map(|a| a.into());
                Ok(xs::types::Attribute::builder()
                    .name(name)
                    .maybe_type_(type_)
                    .maybe_use_(use_)
                    .maybe_default(fragment.default.clone())
                    .build())
            }
            LocalAttributeFragmentTypeMode::Reference(ref_) => Ok(xs::types::Attribute::builder()
                .ref_(xs::types::QName(ref_.ref_.clone()))
                .build()),
        }
    }
}

impl ComplexFragmentEquivalent for xs::types::TopLevelAttribute {
    type FragmentId = FragmentIdx<TopLevelAttributeFragment>;

    fn to_complex_fragments<T: AsMut<ComplexTypeFragmentCompiler>>(
        &self,
        mut compiler: T,
    ) -> Result<Self::FragmentId, Error> {
        let mut compiler = compiler.as_mut();

        let name = self.name.clone();

        let type_ = match (self.type_.as_ref(), self.simple_type.as_ref()) {
            (None, Some(s)) => Some(NamedOrAnonymous::Anonymous(
                s.to_simple_fragments(&mut compiler)
                    .expect("Failed to convert simple type to fragments"),
            )),
            (Some(t), None) => Some(NamedOrAnonymous::Named(t.0.clone())),
            (Some(_), Some(_)) => {
                return Err(Error::SubstitutionGroupNotSupported {
                    fragment_type: "TopLevelAttribute with both type and simpleType",
                });
            }
            (None, None) => None,
        };

        Ok(compiler.push_fragment(TopLevelAttributeFragment { name, type_ }))
    }

    fn from_complex_fragments<T: AsRef<ComplexTypeFragmentCompiler>>(
        compiler: T,
        fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        let compiler = compiler.as_ref();

        let fragment = compiler
            .get_fragment(fragment_id)
            .expect("Fragment not found in compiler.");

        let name = fragment.name.clone();
        let type_ = fragment.type_.as_ref().and_then(|a| match a {
            NamedOrAnonymous::Named(ref_) => Some(xs::types::QName(ref_.clone())),
            NamedOrAnonymous::Anonymous(_) => None,
        });
        Ok(xs::types::TopLevelAttribute::builder()
            .name(name)
            .maybe_type_(type_)
            .any_attributes(ns::AnyAttributes::default())
            .build())
    }
}

impl ComplexFragmentEquivalent for xs::types::AttributeGroupRef {
    type FragmentId = FragmentIdx<AttributeGroupRefFragment>;

    fn to_complex_fragments<T: AsMut<ComplexTypeFragmentCompiler>>(
        &self,
        mut compiler: T,
    ) -> Result<Self::FragmentId, Error> {
        let compiler = compiler.as_mut();

        Ok(compiler.push_fragment(AttributeGroupRefFragment {
            ref_: self
                .ref_
                .0
                .clone()
                .with_default_namespace(|| compiler.namespace.clone()),
        }))
    }

    fn from_complex_fragments<T: AsRef<ComplexTypeFragmentCompiler>>(
        compiler: T,
        fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        let compiler = compiler.as_ref();

        let fragment = compiler
            .get_fragment(fragment_id)
            .expect("Fragment not found in compiler.");

        Ok(xs::types::AttributeGroupRef::builder()
            .ref_(xs::types::QName(fragment.ref_.clone()))
            .any_attributes(ns::AnyAttributes::default())
            .build())
    }
}

impl ComplexFragmentEquivalent for xs::types::SimpleExtensionType {
    type FragmentId = FragmentIdx<SimpleExtensionFragment>;

    fn to_complex_fragments<T: AsMut<ComplexTypeFragmentCompiler>>(
        &self,
        mut compiler: T,
    ) -> Result<Self::FragmentId, Error> {
        let compiler = compiler.as_mut();

        let attribute_declarations = self.attr_decls.to_complex_fragments(&mut *compiler)?;

        let assertions = self.assertions.to_complex_fragments(&mut *compiler)?;

        Ok(compiler.push_fragment(SimpleExtensionFragment {
            base: self
                .base
                .0
                .clone()
                .with_default_namespace(|| compiler.namespace.clone()),
            attribute_declarations,
            assertions,
        }))
    }

    fn from_complex_fragments<T: AsRef<ComplexTypeFragmentCompiler>>(
        compiler: T,
        fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        let compiler = compiler.as_ref();

        let fragment = compiler
            .get_fragment(fragment_id)
            .expect("Fragment not found in compiler.");

        let attr_decls = xs::groups::AttrDecls::from_complex_fragments(
            compiler,
            &fragment.attribute_declarations,
        )?;

        let assertions =
            xs::groups::Assertions::from_complex_fragments(compiler, &fragment.assertions)?;

        Ok(Self::builder()
            .base(xs::types::QName(fragment.base.clone()))
            .attr_decls(attr_decls.into())
            .assertions(assertions.into())
            .any_attributes(ns::AnyAttributes::default())
            .build())
    }
}

impl ComplexFragmentEquivalent for xs::types::SimpleRestrictionType {
    type FragmentId = FragmentIdx<SimpleRestrictionFragment>;

    fn to_complex_fragments<T: AsMut<ComplexTypeFragmentCompiler>>(
        &self,
        mut compiler: T,
    ) -> Result<Self::FragmentId, Error> {
        let mut compiler = compiler.as_mut();

        let base = self
            .base
            .0
            .clone()
            .with_default_namespace(|| compiler.namespace.clone());

        let attribute_declarations = self.attr_decls.to_complex_fragments(&mut compiler)?;

        let assertions = self.assertions.to_complex_fragments(&mut compiler)?;

        Ok(compiler.push_fragment(SimpleRestrictionFragment {
            base,
            attribute_declarations,
            assertions,
        }))
    }

    fn from_complex_fragments<T: AsRef<ComplexTypeFragmentCompiler>>(
        compiler: T,
        fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        let compiler = compiler.as_ref();

        let fragment = compiler
            .get_fragment(fragment_id)
            .expect("Fragment not found in compiler.");

        let attr_decls = xs::groups::AttrDecls::from_complex_fragments(
            compiler,
            &fragment.attribute_declarations,
        )?;

        let assertions =
            xs::groups::Assertions::from_complex_fragments(compiler, &fragment.assertions)?;

        Ok(Self::builder()
            .base(xs::types::QName(fragment.base.clone()))
            .attr_decls(attr_decls.into())
            .assertions(assertions.into())
            .any_attributes(ns::AnyAttributes::default())
            .build())
    }
}

impl ComplexFragmentEquivalent for xs::SimpleContent {
    type FragmentId = FragmentIdx<SimpleContentFragment>;

    /// This method expects all references to already be defined.
    fn to_complex_fragments<T: AsMut<ComplexTypeFragmentCompiler>>(
        &self,
        mut compiler: T,
    ) -> Result<Self::FragmentId, Error> {
        let mut compiler = compiler.as_mut();

        let xs::SimpleContent::SimpleContent(simple_content) = self else {
            return Err(Error::SubstitutionGroupNotSupported {
                fragment_type: type_name::<Self>(),
            });
        };

        let content_fragment = match &simple_content.child_1 {
            xs::simple_content_items::Child1::Extension(extension) => {
                let fragment_id = extension.to_complex_fragments(&mut compiler)?;

                SimpleContentChildId::Extension(fragment_id)
            }
            xs::simple_content_items::Child1::Restriction(restriction) => {
                let fragment_id = restriction.to_complex_fragments(&mut compiler)?;

                SimpleContentChildId::Restriction(fragment_id)
            }
        };

        Ok(compiler.push_fragment(SimpleContentFragment { content_fragment }))
    }

    fn from_complex_fragments<T: AsRef<ComplexTypeFragmentCompiler>>(
        compiler: T,
        fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        let compiler = compiler.as_ref();

        let fragment = compiler
            .get_fragment(fragment_id)
            .expect("Fragment not found in compiler.");

        let child_1 = match &fragment.content_fragment {
            SimpleContentChildId::Extension(fragment_id) => {
                xs::types::SimpleExtensionType::from_complex_fragments(compiler, fragment_id)?
                    .into()
            }
            SimpleContentChildId::Restriction(fragment_id) => {
                xs::types::SimpleRestrictionType::from_complex_fragments(compiler, fragment_id)?
                    .into()
            }
        };

        Ok(xs::SimpleContent::from(
            xs::simple_content_items::SimpleContent {
                annotation: None,
                id: None,
                child_1,
            },
        ))
    }
}

impl ComplexFragmentEquivalent for xs::ComplexContent {
    type FragmentId = FragmentIdx<ComplexContentFragment>;

    /// This method expects all references to already be defined.
    fn to_complex_fragments<T: AsMut<ComplexTypeFragmentCompiler>>(
        &self,
        mut compiler: T,
    ) -> Result<Self::FragmentId, Error> {
        let mut compiler = compiler.as_mut();

        let xs::ComplexContent::ComplexContent(complex_content) = self else {
            return Err(Error::SubstitutionGroupNotSupported {
                fragment_type: type_name::<Self>(),
            });
        };

        let content_fragment = match &complex_content.child_1 {
            xs::complex_content_items::Child1::Extension(extension) => {
                let fragment_id = extension.to_complex_fragments(&mut compiler)?;

                ComplexContentChildId::Extension(fragment_id)
            }
            xs::complex_content_items::Child1::Restriction(restriction) => {
                let fragment_id = restriction.to_complex_fragments(&mut compiler)?;

                ComplexContentChildId::Restriction(fragment_id)
            }
        };

        Ok(compiler.push_fragment(ComplexContentFragment {
            content_fragment,
            mixed: complex_content.mixed,
        }))
    }

    fn from_complex_fragments<T: AsRef<ComplexTypeFragmentCompiler>>(
        compiler: T,
        fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        let compiler = compiler.as_ref();

        let fragment = compiler
            .get_fragment(fragment_id)
            .expect("Fragment not found in compiler.");

        let child_1 = match &fragment.content_fragment {
            ComplexContentChildId::Extension(fragment_id) => {
                xs::types::ExtensionType::from_complex_fragments(compiler, fragment_id)?.into()
            }
            ComplexContentChildId::Restriction(fragment_id) => {
                xs::types::ComplexRestrictionType::from_complex_fragments(compiler, fragment_id)?
                    .into()
            }
        };

        Ok(xs::ComplexContent::from(
            xs::complex_content_items::ComplexContent {
                annotation: None,
                id: None,
                mixed: fragment.mixed,
                child_1,
            },
        ))
    }
}

impl ComplexFragmentEquivalent for xs::groups::ComplexTypeModel {
    type FragmentId = ComplexTypeModelId;

    /// This method expects all references to already be defined.
    fn to_complex_fragments<T: AsMut<ComplexTypeFragmentCompiler>>(
        &self,
        mut compiler: T,
    ) -> Result<Self::FragmentId, Error> {
        let mut compiler = compiler.as_mut();

        use xs::groups::ComplexTypeModel;

        match self {
            ComplexTypeModel::SimpleContent(simple_content) => simple_content
                .to_complex_fragments(compiler)
                .map(ComplexTypeModelId::SimpleContent),
            ComplexTypeModel::ComplexContent(complex_content) => complex_content
                .to_complex_fragments(compiler)
                .map(ComplexTypeModelId::ComplexContent),
            ComplexTypeModel::Variant2(variant_2) => {
                let xs::groups::complex_type_model_items::complex_type_model_variants::Variant2 {
                    type_def_particle,
                    attr_decls,
                    assertions,
                    ..
                } = variant_2.deref();

                //TODO: Review open content
                let particle = type_def_particle
                    .as_deref()
                    .map(|a| a.to_complex_fragments(&mut compiler))
                    .transpose()?;

                let attr_decls = attr_decls.to_complex_fragments(&mut compiler)?;

                let assertions = assertions.to_complex_fragments(&mut compiler)?;

                Ok(ComplexTypeModelId::Other {
                    particle,
                    attr_decls,
                    assertions,
                })
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
                xs::SimpleContent::from_complex_fragments(compiler, fragment_idx)
                    .map(xs::groups::ComplexTypeModel::from)
            }
            ComplexTypeModelId::ComplexContent(fragment_idx) => {
                xs::ComplexContent::from_complex_fragments(compiler, fragment_idx)
                    .map(xs::groups::ComplexTypeModel::from)
            }
            ComplexTypeModelId::Other {
                particle,
                attr_decls,
                assertions,
            } => {
                let type_def_particle = particle
                    .as_ref()
                    .map(|fragment_id| {
                        xs::groups::TypeDefParticle::from_complex_fragments(compiler, fragment_id)
                            .map(Box::new)
                    })
                    .transpose()?;

                let attr_decls =
                    xs::groups::AttrDecls::from_complex_fragments(compiler, attr_decls)?;

                let assertions =
                    xs::groups::Assertions::from_complex_fragments(compiler, &assertions)?;

                Ok(
                    xs::groups::complex_type_model_items::complex_type_model_variants::Variant2 {
                        open_content: None,
                        type_def_particle,
                        attr_decls,
                        assertions,
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
    ) -> Result<Self::FragmentId, Error> {
        let mut compiler = compiler.as_mut();

        let content = self
            .complex_type_model
            .to_complex_fragments(&mut compiler)?;

        let fragment = ComplexTypeRootFragment {
            name: Some(self.name.clone()),
            content,
            mixed: self.mixed,
            abstract_: self.abstract_,
        };

        Ok(compiler.push_fragment(fragment))
    }

    fn from_complex_fragments<T: AsRef<ComplexTypeFragmentCompiler>>(
        compiler: T,
        fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        let compiler = compiler.as_ref();

        let fragment = compiler
            .get_fragment(fragment_id)
            .expect("Fragment not found in compiler.");

        let complex_type_model =
            xs::groups::ComplexTypeModel::from_complex_fragments(compiler, &fragment.content)?;

        Ok(Self::builder()
            //TODO
            .name(fragment.name.clone().ok_or(Error::NameMissingInTopLevel)?)
            .maybe_mixed(fragment.mixed)
            .complex_type_model(complex_type_model.into())
            .any_attributes(ns::AnyAttributes::default())
            .build())
    }
}

impl ComplexFragmentEquivalent for xs::types::LocalComplexType {
    type FragmentId = FragmentIdx<ComplexTypeRootFragment>;

    fn to_complex_fragments<T: AsMut<ComplexTypeFragmentCompiler>>(
        &self,
        mut compiler: T,
    ) -> Result<Self::FragmentId, Error> {
        let mut compiler = compiler.as_mut();

        let content = self
            .complex_type_model
            .to_complex_fragments(&mut compiler)?;

        let fragment = ComplexTypeRootFragment {
            name: None,
            content,
            mixed: self.mixed,
            abstract_: None,
        };

        Ok(compiler.push_fragment(fragment))
    }

    fn from_complex_fragments<T: AsRef<ComplexTypeFragmentCompiler>>(
        compiler: T,
        fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        let compiler = compiler.as_ref();

        let fragment = compiler
            .get_fragment(fragment_id)
            .expect("Fragment not found in compiler.");

        let content =
            xs::groups::ComplexTypeModel::from_complex_fragments(compiler, &fragment.content)?;

        Ok(xs::types::LocalComplexType::builder()
            .complex_type_model(content.into())
            .maybe_mixed(fragment.mixed)
            .any_attributes(ns::AnyAttributes::default())
            .build())
    }
}

/// Identifier for named group type content.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NamedGroupTypeContentId {
    /// All compositor content.
    All(FragmentIdx<AllFragment>),
    /// Sequence compositor content.
    Sequence(FragmentIdx<SequenceFragment>),
    /// Choice compositor content.
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
    ) -> Result<Self::FragmentId, Error> {
        let mut compiler = compiler.as_mut();
        use xs::types::named_group_items::Child1;

        match self {
            Child1::All {
                min_occurs,
                max_occurs,
                all_model,
                ..
            } => {
                //TODO: Handle any_attributes
                let fragments = all_model
                    .child_1
                    .iter()
                    .map(|content| content.to_complex_fragments(&mut compiler))
                    .collect::<Result<_, _>>()?;

                let fragment = AllFragment {
                    min_occurs: *min_occurs,
                    max_occurs: max_occurs.as_ref().map(|a| AllNNI::from(&**a)),
                    fragments,
                };

                Ok(NamedGroupTypeContentId::All(
                    compiler.push_fragment(fragment),
                ))
            }
            Child1::Choice(choice) => {
                let fragments = choice
                    .nested_particle
                    .iter()
                    .map(|content| content.to_complex_fragments(&mut compiler))
                    .collect::<Result<_, _>>()?;

                let fragment = ChoiceFragment {
                    min_occurs: None,
                    max_occurs: None,
                    fragments,
                };

                Ok(NamedGroupTypeContentId::Choice(
                    compiler.push_fragment(fragment),
                ))
            }
            Child1::Sequence(sequence) => {
                let fragments = sequence
                    .nested_particle
                    .iter()
                    .map(|content| content.to_complex_fragments(&mut compiler))
                    .collect::<Result<_, _>>()?;

                let fragment = SequenceFragment {
                    id: None,
                    min_occurs: None,
                    max_occurs: None,
                    fragments,
                };

                Ok(NamedGroupTypeContentId::Sequence(
                    compiler.push_fragment(fragment),
                ))
            }
        }
    }

    fn from_complex_fragments<T: AsRef<ComplexTypeFragmentCompiler>>(
        compiler: T,
        fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        let compiler = compiler.as_ref();

        match fragment_id {
            NamedGroupTypeContentId::All(all) => {
                let all = compiler
                    .get_fragment(all)
                    .expect("Fragment not found in compiler.");

                Ok(xs::types::named_group_items::Child1::All {
                    id: None,
                    min_occurs: all.min_occurs,
                    max_occurs: all.max_occurs.map(xs::types::AllNNI::from).map(Box::new),
                    all_model: xs::groups::AllModel::builder()
                        .child_1(
                            all.fragments
                                .iter()
                                .map(|fragment| {
                                    xs::groups::all_model_items::Child1::from_complex_fragments(
                                        compiler, fragment,
                                    )
                                })
                                .collect::<Result<_, _>>()?,
                        )
                        .build()
                        .into(),
                    any_attributes: ns::AnyAttributes::default(),
                })
            }
            NamedGroupTypeContentId::Choice(choice) => {
                let choice = compiler
                    .get_fragment(choice)
                    .expect("Fragment not found in compiler.");

                Ok(xs::types::named_group_items::Child1::Choice(
                    xs::types::SimpleExplicitGroup::builder()
                        .nested_particle(
                            choice
                                .fragments
                                .iter()
                                .map(|fragment| {
                                    xs::groups::NestedParticle::from_complex_fragments(
                                        compiler, fragment,
                                    )
                                })
                                .collect::<Result<_, _>>()?,
                        )
                        .any_attributes(ns::AnyAttributes::default())
                        .build()
                        .into(),
                ))
            }
            NamedGroupTypeContentId::Sequence(sequence) => {
                let sequence = compiler
                    .get_fragment(sequence)
                    .expect("Fragment not found in compiler.");

                Ok(xs::types::named_group_items::Child1::Sequence(
                    xs::types::SimpleExplicitGroup::builder()
                        .nested_particle(
                            sequence
                                .fragments
                                .iter()
                                .map(|fragment| {
                                    xs::groups::NestedParticle::from_complex_fragments(
                                        compiler, fragment,
                                    )
                                })
                                .collect::<Result<_, _>>()?,
                        )
                        .any_attributes(ns::AnyAttributes::default())
                        .build()
                        .into(),
                ))
            }
        }
    }
}

impl ComplexFragmentEquivalent for xs::types::NamedGroup {
    type FragmentId = FragmentIdx<TopLevelGroupFragment>;

    fn to_complex_fragments<T: AsMut<ComplexTypeFragmentCompiler>>(
        &self,
        mut compiler: T,
    ) -> Result<Self::FragmentId, Error> {
        let mut compiler = compiler.as_mut();

        let content = self.child_1.to_complex_fragments(&mut compiler)?;

        let fragment = TopLevelGroupFragment {
            name: self.name.clone(),
            content,
        };

        Ok(compiler.push_fragment(fragment))
    }

    fn from_complex_fragments<T: AsRef<ComplexTypeFragmentCompiler>>(
        compiler: T,
        fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        let compiler = compiler.as_ref();

        let fragment = compiler
            .get_fragment(fragment_id)
            .expect("Fragment not found in compiler.");

        let content = xs::types::named_group_items::Child1::from_complex_fragments(
            compiler,
            &fragment.content,
        )?;

        Ok(Self::builder()
            .name(fragment.name.clone())
            .child_1(content)
            .any_attributes(ns::AnyAttributes::default())
            .build())
    }
}

impl ComplexFragmentEquivalent for xs::AnyAttribute {
    type FragmentId = FragmentIdx<AnyAttributeFragment>;

    fn to_complex_fragments<T: AsMut<ComplexTypeFragmentCompiler>>(
        &self,
        mut compiler: T,
    ) -> Result<Self::FragmentId, Error> {
        let compiler = compiler.as_mut();

        let xs::AnyAttribute::AnyAttribute(any_attribute) = self else {
            return Err(Error::SubstitutionGroupNotSupported {
                fragment_type: type_name::<Self>(),
            });
        };

        //TODO: Handle any_attribute
        Ok(compiler.push_fragment(AnyAttributeFragment {
            id: any_attribute.id.clone(),
            process_contents: any_attribute.process_contents.map(|a| a.into()),
        }))
    }

    fn from_complex_fragments<T: AsRef<ComplexTypeFragmentCompiler>>(
        compiler: T,
        fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        let compiler = compiler.as_ref();

        let fragment = compiler
            .get_fragment(fragment_id)
            .expect("Fragment not found in compiler.");

        Ok(xs::AnyAttribute::from(
            xs::any_attribute_items::AnyAttribute::builder()
                .maybe_id(fragment.id.clone())
                .maybe_process_contents(fragment.process_contents.map(|a| a.into()))
                .build(),
        ))
    }
}

impl ComplexFragmentEquivalent for xs::groups::AttrDecls {
    type FragmentId = FragmentIdx<AttributeDeclarationsFragment>;

    fn to_complex_fragments<T: AsMut<ComplexTypeFragmentCompiler>>(
        &self,
        mut compiler: T,
    ) -> Result<Self::FragmentId, Error> {
        let mut compiler = compiler.as_mut();

        let declarations = self
            .attribute
            .iter()
            .map(|decl| decl.to_complex_fragments(&mut compiler))
            .collect::<Result<_, _>>()?;

        let any_attribute = self
            .any_attribute
            .as_ref()
            .map(|a| a.to_complex_fragments(&mut compiler))
            .transpose()?;

        Ok(compiler.push_fragment(AttributeDeclarationsFragment {
            declarations,
            any_attribute,
        }))
    }

    fn from_complex_fragments<T: AsRef<ComplexTypeFragmentCompiler>>(
        compiler: T,
        fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        let compiler = compiler.as_ref();

        let fragment = compiler
            .get_fragment(fragment_id)
            .expect("Fragment not found in compiler.");

        let attributes = fragment
            .declarations
            .iter()
            .map(|attr| {
                xs::groups::attr_decls_items::Attribute::from_complex_fragments(compiler, attr)
            })
            .collect::<Result<_, _>>()?;

        let any_attribute = fragment
            .any_attribute
            .as_ref()
            .map(|any_attr| {
                xs::AnyAttribute::from_complex_fragments(compiler, any_attr).map(Box::new)
            })
            .transpose()?;

        Ok(xs::groups::AttrDecls::builder()
            .attribute(attributes)
            .maybe_any_attribute(any_attribute)
            .build())
    }
}

impl ComplexFragmentEquivalent for xs::types::NamedAttributeGroup {
    type FragmentId = FragmentIdx<TopLevelAttributeGroupFragment>;

    fn to_complex_fragments<T: AsMut<ComplexTypeFragmentCompiler>>(
        &self,
        mut compiler: T,
    ) -> Result<Self::FragmentId, Error> {
        let mut compiler = compiler.as_mut();

        let attr_decls = self.attr_decls.to_complex_fragments(&mut compiler)?;

        Ok(compiler.push_fragment(TopLevelAttributeGroupFragment {
            name: self.name.clone(),
            attr_decls,
        }))
    }

    fn from_complex_fragments<T: AsRef<ComplexTypeFragmentCompiler>>(
        compiler: T,
        fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        let compiler = compiler.as_ref();
        let fragment = compiler
            .get_fragment(fragment_id)
            .expect("Fragment not found in compiler.");

        let attr_decls =
            xs::groups::AttrDecls::from_complex_fragments(compiler, &fragment.attr_decls)?;

        Ok(Self::builder()
            .name(fragment.name.clone())
            .attr_decls(attr_decls.into())
            .any_attributes(ns::AnyAttributes::default())
            .build())
    }
}
