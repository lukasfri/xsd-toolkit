pub use xmlity::XmlValue;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Namespace(pub String);

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct QNameRef {
    pub namespace: Option<Namespace>,
    pub name: String,
}

/// Represents the minimum occurrence of types or elements
pub type MinOccurs = usize;

/// Represents the maximum occurrence of types or elements
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Default)]
pub enum MaxOccursValue {
    /// The occurrence is unbounded.
    #[default]
    Unbounded,

    /// The occurrence is bound to the specified limit.
    Bounded(usize),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Id(pub String);

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Version(pub String);

#[derive(Debug, Clone, Eq, PartialEq)]

pub struct Schema {
    pub target_namespace: Option<TargetNamespace>,
    pub version: Option<Version>,
    pub final_default: Option<FullDerivationSetType>,
    pub block_default: Option<BlockSetType>,
    pub attribute_form_default: Option<FormChoiceType>,
    pub element_form_default: Option<FormChoiceType>,
    pub default_attributes: Option<QNameRef>,
    pub xpath_default_namespace: Option<XpathDefaultNamespaceType>,
    pub id: Option<Id>,
    pub lang: Option<XmlLang>,
    pub compositions: Vec<Composition>,
    pub open_content: Vec<DefaultOpenContent>,
    pub schema_top: Vec<SchemaTop>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Composition {
    Include(Include),
    Import(Import),
    Redefine(Redefine),
    Override(Override),
    Annotation(Annotation),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Redefineable {
    SimpleType(TopLevelSimpleType),
    ComplexType(TopLevelComplexType),
    Group(GroupType),
    AttributeGroup(AttributeGroupType),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum SchemaTop {
    Redefineable(Redefineable),
    Element(TopLevelElement),
    Attribute(TopLevelAttribute),
    Notation(Notation),
    Annotation(Annotation),
}

impl Schema {
    #[must_use]
    pub fn default_final_default() -> FullDerivationSetType {
        FullDerivationSetType::TypeDerivationControlList(Vec::new())
    }
    #[must_use]
    pub fn default_block_default() -> BlockSetType {
        BlockSetType::BlockSetItemList(Vec::new())
    }
    #[must_use]
    pub fn default_attribute_form_default() -> FormChoiceType {
        FormChoiceType::Unqualified
    }
    #[must_use]
    pub fn default_element_form_default() -> FormChoiceType {
        FormChoiceType::Unqualified
    }
    #[must_use]
    pub fn default_xpath_default_namespace() -> XpathDefaultNamespaceType {
        XpathDefaultNamespaceType::String(String::from("##local"))
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum FullDerivationSetType {
    All,
    TypeDerivationControlList(Vec<TypeDerivationControlType>),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum BlockSetType {
    All,
    BlockSetItemList(Vec<BlockSetItemType>),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum FormChoiceType {
    Qualified,
    Unqualified,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum XpathDefaultNamespaceType {
    String(String),
    DefaultNamespace,
    TargetNamespace,
    Local,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SchemaLocation(pub String);

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Include {
    pub id: Option<Id>,
    pub schema_location: SchemaLocation,
    pub annotation: Option<Annotation>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Import {
    pub id: Option<Id>,
    pub namespace: Option<Namespace>,
    pub schema_location: Option<SchemaLocation>,
    pub annotation: Option<Annotation>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Redefine {
    pub schema_location: String,
    pub id: Option<Id>,
    pub content: Vec<RedefineContent>,
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum RedefineContent {
    Annotation(Annotation),
    SimpleType(LocalSimpleType),
    ComplexType(LocalComplexType),
    Group(GroupType),
    AttributeGroup(AttributeGroupType),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Override {
    pub schema_location: String,
    pub id: Option<Id>,
    pub content: Vec<OverrideContent>,
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum OverrideContent {
    Annotation(Annotation),
    SimpleType(LocalSimpleType),
    ComplexType(LocalComplexType),
    Group(GroupType),
    AttributeGroup(AttributeGroupType),
    Element(LocalElement),
    Attribute(LocalAttribute),
    Notation(Notation),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Annotation {
    pub id: Option<Id>,
    pub content: Vec<AnnotationContent>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Annotated {
    pub annotations: Vec<Annotation>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum AnnotationContent {
    Appinfo(Appinfo),
    Documentation(Documentation),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct DefaultOpenContent {
    pub id: Option<Id>,
    pub applies_to_empty: bool,
    pub mode: DefaultOpenContentModeType,
    pub annotation: Option<Annotation>,
    pub any: Any,
}
impl DefaultOpenContent {
    #[must_use]
    pub fn default_applies_to_empty() -> bool {
        false
    }
    #[must_use]
    pub fn default_mode() -> DefaultOpenContentModeType {
        DefaultOpenContentModeType::Interleave
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct LocalSimpleType {
    pub id: Option<Id>,
    pub final_: Option<SimpleDerivationSetType>,
    pub annotation: Option<Annotation>,
    pub content: SimpleDerivation,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TopLevelSimpleType {
    pub id: Option<Id>,
    pub final_: Option<SimpleDerivationSetType>,
    pub name: Option<Name>,
    pub annotation: Option<Annotation>,
    pub content: SimpleDerivation,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum SimpleDerivation {
    Restriction(Box<LocalRestriction>),
    List(Box<List>),
    Union(Box<Union>),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Name(pub String);

pub type Mixed = bool;

pub type Abstract = bool;

pub type Final = DerivationSetType;

pub type Block = DerivationSetType;

pub type DefaultAttributesApply = bool;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct LocalComplexType {
    pub id: Option<Id>,
    pub mixed: Option<Mixed>,
    pub abstract_: Option<Abstract>,
    pub final_: Option<Final>,
    pub block: Option<Block>,
    pub default_attributes_apply: Option<DefaultAttributesApply>,
    pub annotation: Option<Annotation>,
    pub content: Vec<ComplexBaseTypeContent>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TopLevelComplexType {
    pub id: Option<Id>,
    pub name: Option<Name>,
    pub mixed: Option<Mixed>,
    pub abstract_: Option<Abstract>,
    pub final_: Option<Final>,
    pub block: Option<Block>,
    pub default_attributes_apply: Option<DefaultAttributesApply>,
    pub annotation: Option<Annotation>,
    pub content: Vec<ComplexBaseTypeContent>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ComplexBaseTypeContent {
    SimpleContent(SimpleContent),
    ComplexContent(ComplexContent),
    OpenContent(OpenContent),
    Group(GroupType),
    All(AllType),
    Choice(ChoiceType),
    Sequence(SequenceType),
    Attribute(LocalAttribute),
    AttributeGroup(AttributeGroupType),
    AnyAttribute(AnyAttribute),
    Assert(AssertionType),
}

impl LocalComplexType {
    #[must_use]
    pub fn default_abstract_() -> bool {
        false
    }
    #[must_use]
    pub fn default_default_attributes_apply() -> bool {
        true
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct GroupType {
    pub id: Option<Id>,
    pub name: Option<Name>,
    pub ref_: Option<QNameRef>,
    pub min_occurs: Option<MinOccurs>,
    pub max_occurs: Option<MaxOccursValue>,
    pub annotation: Option<Annotation>,
    pub content: Vec<GroupTypeContent>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AllType {
    pub id: Option<Id>,
    pub name: Option<Name>,
    pub ref_: Option<QNameRef>,
    pub min_occurs: Option<MinOccurs>,
    pub max_occurs: Option<MaxOccursValue>,
    pub content: Vec<GroupTypeContent>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ChoiceType {
    pub id: Option<Id>,
    pub name: Option<Name>,
    pub ref_: Option<QNameRef>,
    pub min_occurs: Option<MinOccurs>,
    pub max_occurs: Option<MaxOccursValue>,
    pub content: Vec<GroupTypeContent>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SequenceType {
    pub id: Option<Id>,
    pub name: Option<Name>,
    pub ref_: Option<QNameRef>,
    pub min_occurs: Option<MinOccurs>,
    pub max_occurs: Option<MaxOccursValue>,
    pub content: Vec<GroupTypeContent>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum GroupTypeContent {
    Element(Box<LocalElement>),
    Group(Box<GroupType>),
    All(Box<AllType>),
    Choice(Box<ChoiceType>),
    Sequence(Box<SequenceType>),
    Any(Box<Any>),
}
impl GroupType {
    #[must_use]
    pub fn default_min_occurs() -> usize {
        1usize
    }
    #[must_use]
    pub fn default_max_occurs() -> MaxOccursValue {
        MaxOccursValue::Bounded(1usize)
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AttributeGroupType {
    pub id: Option<Id>,
    pub name: Option<Name>,
    pub ref_: Option<QNameRef>,
    pub annotation: Option<Annotation>,
    pub content: Vec<AttributeGroupTypeContent>,
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum AttributeGroupTypeContent {
    Attribute(Box<LocalAttribute>),
    AttributeGroup(Box<AttributeGroupType>),
    AnyAttribute(Box<AnyAttribute>),
}

pub type DefaultAttr = String;

pub type Fixed = String;

pub type Nillable = bool;

pub type TargetNamespace = String;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct LocalElement {
    pub id: Option<Id>,
    pub name: Option<Name>,
    pub ref_: Option<QNameRef>,
    pub type_: Option<QNameRef>,
    pub substitution_group: Vec<QNameRef>,
    pub min_occurs: Option<MinOccurs>,
    pub max_occurs: Option<MaxOccursValue>,
    pub default: Option<DefaultAttr>,
    pub fixed: Option<Fixed>,
    pub nillable: Option<Nillable>,
    pub abstract_: Option<Abstract>,
    pub final_: Option<Final>,
    pub block: Option<BlockSetType>,
    pub form: Option<FormChoiceType>,
    pub target_namespace: Option<TargetNamespace>,
    pub annotation: Option<Annotation>,
    pub type_choice: Option<ElementTypeContent>,
    pub alternatives: Vec<AltType>,
    pub identity_constraints: Vec<IdentityConstraint>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TopLevelElement {
    pub id: Option<Id>,
    pub name: Option<Name>,
    pub type_: Option<QNameRef>,
    pub substitution_group: Vec<QNameRef>,
    pub min_occurs: Option<MinOccurs>,
    pub max_occurs: Option<MaxOccursValue>,
    pub default: Option<DefaultAttr>,
    pub fixed: Option<Fixed>,
    pub nillable: Option<Nillable>,
    pub abstract_: Option<Abstract>,
    pub final_: Option<Final>,
    pub block: Option<BlockSetType>,
    pub form: Option<FormChoiceType>,
    pub target_namespace: Option<TargetNamespace>,
    pub annotation: Option<Annotation>,
    pub type_choice: Option<ElementTypeContent>,
    pub alternatives: Vec<AltType>,
    pub identity_constraints: Vec<IdentityConstraint>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ElementTypeContent {
    SimpleType(Box<LocalSimpleType>),
    ComplexType(Box<LocalComplexType>),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum IdentityConstraint {
    Unique(Box<Unique>),
    Key(Box<Key>),
    Keyref(Box<Keyref>),
}

impl LocalElement {
    #[must_use]
    pub fn default_min_occurs() -> usize {
        1usize
    }
    #[must_use]
    pub fn default_max_occurs() -> MaxOccursValue {
        MaxOccursValue::Bounded(1usize)
    }
    #[must_use]
    pub fn default_abstract_() -> bool {
        false
    }
}

pub type Inheritable = bool;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct LocalAttribute {
    pub id: Option<Id>,
    pub name: Option<Name>,
    pub ref_: Option<QNameRef>,
    pub type_: Option<QNameRef>,
    pub use_: Option<AttributeUseType>,
    pub default: Option<DefaultAttr>,
    pub fixed: Option<Fixed>,
    pub form: Option<FormChoiceType>,
    pub target_namespace: Option<TargetNamespace>,
    pub inheritable: Option<Inheritable>,
    pub annotation: Option<Annotation>,
    pub simple_type: Option<LocalSimpleType>,
}
impl LocalAttribute {
    #[must_use]
    pub fn default_use_() -> AttributeUseType {
        AttributeUseType::Optional
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TopLevelAttribute {
    pub id: Option<Id>,
    pub name: Option<Name>,
    pub type_: Option<QNameRef>,
    pub use_: Option<AttributeUseType>,
    pub default: Option<DefaultAttr>,
    pub fixed: Option<Fixed>,
    pub form: Option<FormChoiceType>,
    pub target_namespace: Option<TargetNamespace>,
    pub inheritable: Option<Inheritable>,
    pub annotation: Option<Annotation>,
    pub simple_type: Option<LocalSimpleType>,
}
impl TopLevelAttribute {
    #[must_use]
    pub fn default_use_() -> AttributeUseType {
        AttributeUseType::Optional
    }
}

pub type Public = String;

pub type System = String;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Notation {
    pub id: Option<Id>,
    pub name: Name,
    pub public: Option<Public>,
    pub system: Option<System>,
    pub annotation: Option<Annotation>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum TypeDerivationControlType {
    Extension,
    Restriction,
    List,
    Union,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum BlockSetItemType {
    Extension,
    Restriction,
    Substitution,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Appinfo {
    pub source: Option<Source>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Source(pub String);

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct XmlLang(pub String);

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Documentation {
    pub source: Option<Source>,
    pub lang: Option<XmlLang>,
    pub any: Vec<XmlValue>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum DefaultOpenContentModeType {
    Interleave,
    Suffix,
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct WildcardType {
    pub id: Option<Id>,
    pub namespace: Option<NamespaceListType>,
    pub not_namespace: Option<Vec<BasicNamespaceListItemType>>,
    pub process_contents: ProcessContentsType,
    pub annotation: Option<Annotation>,
}
impl WildcardType {
    #[must_use]
    pub fn default_process_contents() -> ProcessContentsType {
        ProcessContentsType::Strict
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum SimpleDerivationSetType {
    All,
    SimpleDerivationSetItemList(Vec<SimpleDerivationSetItemType>),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct List {
    pub id: Option<Id>,
    pub annotation: Option<Annotation>,
    pub simple_type: Option<LocalSimpleType>,
    pub item_type: Option<QNameRef>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Union {
    pub id: Option<Id>,
    pub annotation: Option<Annotation>,
    pub simple_types: Vec<LocalSimpleType>,

    pub member_types: Option<Vec<QNameRef>>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum DerivationSetType {
    All,
    ReducedDerivationControlList(Vec<ReducedDerivationControlType>),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SimpleContent {
    pub id: Option<Id>,
    pub content: Vec<SimpleContentContent>,
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum SimpleContentContent {
    Annotation(Box<Annotation>),
    Restriction(Box<Restriction>),
    Extension(Box<ExtensionType>),
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ComplexContent {
    pub id: Option<Id>,
    pub mixed: Option<Mixed>,
    pub content: Vec<ComplexContentContent>,
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ComplexContentContent {
    Annotation(Box<Annotation>),
    Restriction(Box<ComplexRestrictionType>),
    Extension(Box<ExtensionType>),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AnyType {
    pub wildcard: WildcardType,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct OpenContent {
    pub id: Option<Id>,
    pub mode: OpenContentModeType,
    pub annotation: Option<Annotation>,
    pub any: Option<AnyType>,
}
impl OpenContent {
    #[must_use]
    pub fn default_mode() -> OpenContentModeType {
        OpenContentModeType::Interleave
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AnyAttribute {
    pub id: Option<Id>,
    pub namespace: Option<NamespaceListType>,
    pub not_namespace: Option<NamespaceListType>,
    pub process_contents: ProcessContentsType,
    pub not_q_name: Option<Vec<QnameListAItemType>>,
    pub annotation: Option<Annotation>,
}
impl AnyAttribute {
    #[must_use]
    pub fn default_process_contents() -> ProcessContentsType {
        ProcessContentsType::Strict
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AssertionType {
    pub id: Option<Id>,
    pub test: Option<String>,
    pub xpath_default_namespace: Option<XpathDefaultNamespaceType>,
    pub annotation: Option<Annotation>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Any {
    pub id: Option<Id>,
    pub namespace: Option<NamespaceListType>,
    pub not_namespace: Option<NamespaceListType>,
    pub process_contents: ProcessContentsType,
    pub not_q_name: Option<Vec<QnameListItemType>>,
    pub min_occurs: Option<MinOccurs>,
    pub max_occurs: Option<MaxOccursValue>,
    pub annotation: Option<Annotation>,
}
impl Any {
    #[must_use]
    pub fn default_process_contents() -> ProcessContentsType {
        ProcessContentsType::Strict
    }
    #[must_use]
    pub fn default_min_occurs() -> usize {
        1usize
    }
    #[must_use]
    pub fn default_max_occurs() -> MaxOccursValue {
        MaxOccursValue::Bounded(1usize)
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AltType {
    pub id: Option<Id>,
    pub test: Option<String>,
    pub type_: Option<QNameRef>,
    pub xpath_default_namespace: Option<XpathDefaultNamespaceType>,
    pub content: Vec<AltTypeContent>,
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum AltTypeContent {
    Annotation(Box<Annotation>),
    SimpleType(Box<LocalSimpleType>),
    ComplexType(Box<LocalComplexType>),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Unique {
    pub id: Option<Id>,
    pub name: Option<Name>,
    pub ref_: Option<QNameRef>,
    pub content: Option<KeybaseTypeContent>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Key {
    pub id: Option<Id>,
    pub name: Option<Name>,
    pub ref_: Option<QNameRef>,
    pub selector: Selector,
    pub field: Vec<Field>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct KeybaseTypeContent {
    pub annotations: Vec<Annotation>,
    pub selector: Selector,
    pub field: Vec<Field>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Refer(pub QNameRef);

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Keyref {
    pub id: Option<Id>,
    pub name: Option<Name>,
    pub ref_: Option<QNameRef>,
    pub refer: Option<Refer>,
    pub content: Option<KeybaseTypeContent>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum AttributeUseType {
    Prohibited,
    Optional,
    Required,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum NamespaceListType {
    Any,
    Other,
    BasicNamespaceList(Vec<BasicNamespaceListItemType>),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ProcessContentsType {
    Skip,
    Lax,
    Strict,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Enumeration {
    pub fixed: Option<Fixed>,
    pub value: String,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct FacetType {
    pub fixed: Option<Fixed>,
    pub annotation: Option<Annotation>,
    pub value: String,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MinExclusive {
    pub facet_type: FacetType,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MinInclusive {
    pub facet_type: FacetType,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MaxExclusive {
    pub facet_type: FacetType,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MaxInclusive {
    pub facet_type: FacetType,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Facet {
    MinExclusive(Box<MinExclusive>),
    MinInclusive(Box<MinInclusive>),
    MaxExclusive(Box<MaxExclusive>),
    MaxInclusive(Box<MaxInclusive>),
    // TotalDigits(Box<FacetType>),
    // FractionDigits(Box<FacetType>),
    // Length(Box<FacetType>),
    // MinLength(Box<FacetType>),
    // MaxLength(Box<FacetType>),
    Enumeration(Box<Enumeration>),
    // WhiteSpace(Box<FacetType>),
    // Pattern(Box<FacetType>),
    // Assertion(Box<AssertionType>),
    // ExplicitTimezone(Box<FacetType>),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Restriction {
    pub id: Option<Id>,
    pub base: QNameRef,
    pub content: Vec<RestrictionTypeContent>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum RestrictionTypeContent {
    Annotation(Box<Annotation>),
    OpenContent(Box<OpenContent>),
    Group(Box<GroupType>),
    All(Box<AllType>),
    Choice(Box<ChoiceType>),
    Sequence(Box<SequenceType>),
    SimpleType(Box<LocalSimpleType>),
    Facet(Box<Facet>),
    Attribute(Box<LocalAttribute>),
    AttributeGroup(Box<AttributeGroupType>),
    AnyAttribute(Box<AnyAttribute>),
    Assert(Box<AssertionType>),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct LocalRestriction {
    pub id: Option<Id>,
    pub base: QNameRef,
    pub annotation: Option<Annotation>,
    pub simple_type: Option<LocalSimpleType>,
    pub facets: Vec<Facet>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SimpleRestrictionType {
    pub id: Option<Id>,
    pub base: QNameRef,

    pub annotation: Option<Annotation>,
    pub simple_type: Option<LocalSimpleType>,
    pub facets: Vec<Facet>,
    //attrDecals
    //assertions
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ComplexRestrictionType {
    pub id: Option<Id>,
    pub annotation: Option<Annotation>,
    pub base: Option<QNameRef>,
    pub simple_type: Option<LocalSimpleType>,
    pub content: Vec<RestrictionContent>,
    // #[xvalue(default)]
    // pub attr_decls: Option<AttrDeclsType>,
    // #[xvalue(default)]
    // pub assertions: Option<AssertionsType>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum RestrictionContent {
    // Facet(Box<Facet>),
    //TODO: Huh?
    Other(Box<XmlValue>),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ExtensionType {
    pub id: Option<Id>,
    pub base: QNameRef,
    pub content: Vec<ExtensionTypeContent>,
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ExtensionTypeContent {
    Annotation(Box<Annotation>),
    OpenContent(Box<OpenContent>),
    Group(Box<GroupType>),
    All(Box<AllType>),
    Choice(Box<ChoiceType>),
    Sequence(Box<SequenceType>),
    Attribute(Box<LocalAttribute>),
    AttributeGroup(Box<AttributeGroupType>),
    AnyAttribute(Box<AnyAttribute>),
    Assert(Box<AssertionType>),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum OpenContentModeType {
    None,
    Interleave,
    Suffix,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Field {
    pub id: Option<Id>,
    pub xpath: XPath,
    pub xpath_default_namespace: Option<XpathDefaultNamespaceType>,
    pub annotation: Option<Annotation>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct XPath(pub String);

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Selector {
    pub id: Option<Id>,
    pub xpath: XPath,
    pub xpath_default_namespace: Option<XpathDefaultNamespaceType>,
    pub annotations: Vec<Annotation>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum BasicNamespaceListItemType {
    String(String),
    TargetNamespace,
    Local,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum SimpleDerivationSetItemType {
    List,
    Union,
    Restriction,
    Extension,
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ReducedDerivationControlType {
    Extension,
    Restriction,
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum QnameListAItemType {
    Qname(QNameRef),
    Defined,
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum QnameListItemType {
    Qname(QNameRef),
    Defined,
    DefinedSibling,
}
