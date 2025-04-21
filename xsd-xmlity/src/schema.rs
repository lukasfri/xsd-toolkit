use core::fmt;
use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};

use xmlity::{
    types::string::FromStrVisitor, DeserializationGroup, Deserialize, SerializationGroup,
    Serialize, SerializeAttribute, XmlNamespace, XmlValue,
};

pub const NS_XSD: XmlNamespace = XmlNamespace::new_dangerous("http://www.w3.org/2001/XMLSchema");
pub const NS_XML: XmlNamespace =
    XmlNamespace::new_dangerous("http://www.w3.org/XML/1998/namespace");

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QName(pub String);

impl Display for QName {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for QName {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.to_string()))
    }
}

macro_rules! impl_from_str_deserialize {
    ($ty:ty) => {
        impl<'de> Deserialize<'de> for $ty {
            fn deserialize<D: xmlity::Deserializer<'de>>(reader: D) -> Result<Self, D::Error> {
                reader.deserialize_any(FromStrVisitor::default())
            }
        }
    };
    ($($ty:ty),+) => ($(
        impl_from_str_deserialize!($ty);
    )+);
}

macro_rules! impl_to_string_serialize  {
    ($ty:ty) => {
        impl Serialize for $ty {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: xmlity::Serializer,
            {
                serializer.serialize_text(&self.to_string())
            }
        }
    };
    ($($ty:ty),+) => ($(
        impl_to_string_serialize!($ty);
    )+);
}

/// Represents the minimum occurrence of types or elements
#[derive(Debug, Clone, Eq, PartialEq, SerializeAttribute, Deserialize, Default)]
#[xattribute(name = "minOccurs")]
pub struct MinOccurs(pub usize);

/// Represents the maximum occurrence of types or elements
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Default)]
pub enum MaxOccursValue {
    /// The occurrence is unbounded.
    #[default]
    Unbounded,

    /// The occurrence is bound to the specified limit.
    Bounded(usize),
}

impl FromStr for MaxOccursValue {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "unbounded" => Ok(Self::Unbounded),
            _ => Ok(Self::Bounded(usize::from_str(s)?)),
        }
    }
}

impl Display for MaxOccursValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unbounded => write!(f, "unbounded"),
            Self::Bounded(n) => write!(f, "{n}"),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, SerializeAttribute, Deserialize, Default)]
#[xattribute(name = "maxOccurs")]
pub struct MaxOccurs(pub MaxOccursValue);

#[derive(Debug, Clone, Eq, PartialEq, SerializeAttribute, Deserialize)]
#[xattribute(name = "id")]
pub struct Id(pub String);

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Error {
    UnknownOrInvalidValue { value: String },
    Utf8Error(std::string::FromUtf8Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownOrInvalidValue { value } => {
                write!(f, "unknown or invalid value: {value}")
            }
            Self::Utf8Error(err) => write!(f, "utf8 error: {err}"),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, SerializeAttribute, Deserialize)]
#[xattribute(name = "version")]
pub struct Version(pub String);

#[derive(Debug, Clone, Eq, PartialEq, SerializeAttribute, Deserialize)]
#[xattribute(name = "finalDefault")]
pub struct FinalDefault(pub FullDerivationSetType);

#[derive(Debug, Clone, Eq, PartialEq, SerializeAttribute, Deserialize)]
#[xattribute(name = "blockDefault")]
pub struct BlockDefault(pub BlockSetType);

#[derive(Debug, Clone, Eq, PartialEq, SerializeAttribute, Deserialize)]
#[xattribute(name = "attributeFormDefault")]
pub struct AttributeFormDefault(pub FormChoiceType);

#[derive(Debug, Clone, Eq, PartialEq, SerializeAttribute, Deserialize)]
#[xattribute(name = "elementFormDefault")]
pub struct ElementFormDefault(pub FormChoiceType);

#[derive(Debug, Clone, Eq, PartialEq, SerializeAttribute, Deserialize)]
#[xattribute(name = "defaultAttributes")]
pub struct DefaultAttributes(pub QName);

#[derive(Debug, Clone, Eq, PartialEq, SerializeAttribute, Deserialize)]
#[xattribute(name = "xpathDefaultNamespace")]
pub struct XPathDefaultNamespace(pub XpathDefaultNamespaceType);

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[xelement(
    name = "schema",
    namespace_expr = NS_XSD,
    preferred_prefix = "xs"
)]
pub struct Schema {
    #[xattribute(default)]
    pub target_namespace: Option<TargetNamespace>,
    #[xattribute(default)]
    pub version: Option<Version>,
    #[xattribute(default)]
    pub final_default: Option<FinalDefault>,
    #[xattribute(default)]
    pub block_default: Option<BlockDefault>,
    #[xattribute(default)]
    pub attribute_form_default: Option<AttributeFormDefault>,
    #[xattribute(default)]
    pub element_form_default: Option<ElementFormDefault>,
    #[xattribute(default)]
    pub default_attributes: Option<DefaultAttributes>,
    #[xattribute(default)]
    pub xpath_default_namespace: Option<XPathDefaultNamespace>,
    #[xattribute(default)]
    pub id: Option<Id>,
    #[xattribute(default)]
    pub lang: Option<XmlLang>,
    #[xvalue(default)]
    pub compositions: Vec<Composition>,
    #[xvalue(default)]
    pub open_content: Vec<DefaultOpenContent>,
    #[xvalue(default)]
    pub schema_top: Vec<SchemaTop>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum Composition {
    Include(Include),
    Import(Import),
    Redefine(Redefine),
    Override(Override),
    Annotation(Annotation),
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum Redefineable {
    SimpleType(TopLevelSimpleType),
    ComplexType(TopLevelComplexType),
    Group(GroupType),
    AttributeGroup(AttributeGroupType),
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
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
        FullDerivationSetType::TypeDerivationControlList(TypeDerivationControlList(Vec::new()))
    }
    #[must_use]
    pub fn default_block_default() -> BlockSetType {
        BlockSetType::BlockSetItemList(BlockSetItemList(Vec::new()))
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
    TypeDerivationControlList(TypeDerivationControlList),
}
impl FromStr for FullDerivationSetType {
    type Err = Error;

    fn from_str(bytes: &str) -> Result<Self, Self::Err> {
        match bytes {
            "#all" => Ok(Self::All),
            bytes => Ok(Self::TypeDerivationControlList(
                TypeDerivationControlList::from_str(bytes)?,
            )),
        }
    }
}

impl Display for FullDerivationSetType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::All => write!(f, "#all"),
            Self::TypeDerivationControlList(tdctl) => write!(f, "{tdctl}"),
        }
    }
}

impl_from_str_deserialize!(FullDerivationSetType);
impl_to_string_serialize!(FullDerivationSetType);

#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct TypeDerivationControlList(pub Vec<TypeDerivationControlType>);
impl FromStr for TypeDerivationControlList {
    type Err = Error;

    fn from_str(bytes: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            bytes
                .split([' ', '|', ',', ';'])
                .map(TypeDerivationControlType::from_str)
                .collect::<Result<Vec<_>, _>>()?,
        ))
    }
}

impl Display for TypeDerivationControlList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .map(|a| a.to_string())
                .collect::<Vec<_>>()
                .join(" ")
        )
    }
}

impl_from_str_deserialize!(TypeDerivationControlList);
impl_to_string_serialize!(TypeDerivationControlList);

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum BlockSetType {
    All,
    BlockSetItemList(BlockSetItemList),
}
impl FromStr for BlockSetType {
    type Err = Error;

    fn from_str(bytes: &str) -> Result<Self, Self::Err> {
        match bytes {
            "#all" => Ok(Self::All),
            bytes => Ok(Self::BlockSetItemList(BlockSetItemList::from_str(bytes)?)),
        }
    }
}

impl Display for BlockSetType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::All => write!(f, "#all"),
            Self::BlockSetItemList(list) => write!(f, "{}", list),
        }
    }
}

impl_from_str_deserialize!(BlockSetType);
impl_to_string_serialize!(BlockSetType);

#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct BlockSetItemList(pub Vec<BlockSetItemType>);
impl FromStr for BlockSetItemList {
    type Err = Error;

    fn from_str(bytes: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            bytes
                .split([' ', '|', ',', ';'])
                .map(BlockSetItemType::from_str)
                .collect::<Result<Vec<_>, _>>()?,
        ))
    }
}

impl Display for BlockSetItemList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .map(|a| a.to_string())
                .collect::<Vec<_>>()
                .join(" ")
        )
    }
}

impl_from_str_deserialize!(BlockSetItemList);
impl_to_string_serialize!(BlockSetItemList);

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum FormChoiceType {
    Qualified,
    Unqualified,
}

impl FromStr for FormChoiceType {
    type Err = Error;

    fn from_str(bytes: &str) -> Result<Self, Self::Err> {
        match bytes {
            "qualified" => Ok(Self::Qualified),
            "unqualified" => Ok(Self::Unqualified),
            bytes => Err(Error::UnknownOrInvalidValue {
                value: bytes.to_string(),
            }),
        }
    }
}

impl Display for FormChoiceType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Qualified => "qualified",
                Self::Unqualified => "unqualified",
            }
        )
    }
}

impl_from_str_deserialize!(FormChoiceType);
impl_to_string_serialize!(FormChoiceType);

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum XpathDefaultNamespaceType {
    String(String),
    DefaultNamespace,
    TargetNamespace,
    Local,
}
impl FromStr for XpathDefaultNamespaceType {
    type Err = Error;

    fn from_str(bytes: &str) -> Result<Self, Self::Err> {
        match bytes {
            "##defaultNamespace" => Ok(Self::DefaultNamespace),
            "##targetNamespace" => Ok(Self::TargetNamespace),
            "##local" => Ok(Self::Local),
            bytes => Ok(Self::String(bytes.to_string())),
        }
    }
}

impl Display for XpathDefaultNamespaceType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::String(s) => s,
                Self::DefaultNamespace => "##defaultNamespace",
                Self::TargetNamespace => "##targetNamespace",
                Self::Local => "##local",
            }
        )
    }
}

impl_from_str_deserialize!(XpathDefaultNamespaceType);
impl_to_string_serialize!(XpathDefaultNamespaceType);

#[derive(Debug, Clone, Eq, PartialEq, SerializeAttribute, Deserialize)]
#[xattribute(name = "schemaLocation")]
pub struct SchemaLocation(pub String);

#[derive(Debug, Clone, Eq, PartialEq, SerializeAttribute, Deserialize)]
#[xattribute(name = "namespace")]
pub struct Namespace(pub String);

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[xelement(name = "include", namespace_expr = NS_XSD)]
pub struct Include {
    #[xattribute]
    pub id: Option<Id>,
    #[xattribute]
    pub schema_location: SchemaLocation,
    pub annotation: Option<Annotation>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[xelement(name = "import", namespace_expr = NS_XSD)]
pub struct Import {
    #[xattribute(default)]
    pub id: Option<Id>,
    #[xattribute(default)]
    pub namespace: Option<Namespace>,
    #[xattribute(default)]
    pub schema_location: Option<SchemaLocation>,
    #[xvalue(default)]
    pub annotation: Option<Annotation>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[xelement(name = "redefine", namespace_expr = NS_XSD)]
pub struct Redefine {
    pub schema_location: String,
    #[xattribute]
    pub id: Option<Id>,
    pub content: Vec<RedefineContent>,
}
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum RedefineContent {
    Annotation(Annotation),
    SimpleType(LocalSimpleType),
    ComplexType(LocalComplexType),
    Group(GroupType),
    AttributeGroup(AttributeGroupType),
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[xelement(name = "override", namespace_expr = NS_XSD)]
pub struct Override {
    pub schema_location: String,
    #[xattribute]
    pub id: Option<Id>,
    pub content: Vec<OverrideContent>,
}
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[xelement(name = "annotation", namespace_expr = NS_XSD)]
pub struct Annotation {
    #[xattribute(default = true)]
    pub id: Option<Id>,
    pub content: Vec<AnnotationContent>,
}

#[derive(Debug, Clone, Eq, PartialEq, SerializationGroup, DeserializationGroup)]
pub struct Annotated {
    pub annotations: Vec<Annotation>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum AnnotationContent {
    Appinfo(Appinfo),
    Documentation(Documentation),
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[xelement(
    name = "defaultOpenContent",
    namespace_expr = NS_XSD
)]
pub struct DefaultOpenContent {
    #[xattribute]
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

#[derive(Debug, Clone, Eq, PartialEq, SerializeAttribute, Deserialize)]
#[xattribute(name = "final")]
pub struct Final2(pub SimpleDerivationSetType);

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[xelement(name = "simpleType", namespace_expr = NS_XSD)]
pub struct LocalSimpleType {
    #[xattribute(default)]
    pub id: Option<Id>,
    #[xattribute(default)]
    pub final_: Option<Final2>,
    #[xvalue(default)]
    pub annotation: Option<Annotation>,
    pub content: SimpleDerivation,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[xelement(name = "simpleType", namespace_expr = NS_XSD)]
pub struct TopLevelSimpleType {
    #[xattribute(default)]
    pub id: Option<Id>,
    #[xattribute(default)]
    pub final_: Option<Final2>,
    #[xattribute(default)]
    pub name: Option<Name>,
    #[xvalue(default)]
    pub annotation: Option<Annotation>,
    pub content: SimpleDerivation,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum SimpleDerivation {
    Restriction(Box<LocalRestriction>),
    List(Box<List>),
    Union(Box<Union>),
}

#[derive(Debug, Clone, Eq, PartialEq, SerializeAttribute, Deserialize)]
#[xattribute(name = "name")]
pub struct Name(pub String);

#[derive(Debug, Clone, Eq, PartialEq, SerializeAttribute, Deserialize)]
#[xattribute(name = "mixed")]
pub struct Mixed(pub bool);

#[derive(Debug, Clone, Eq, PartialEq, SerializeAttribute, Deserialize, Default)]
#[xattribute(name = "abstract")]
pub struct Abstract(pub bool);

#[derive(Debug, Clone, Eq, PartialEq, SerializeAttribute, Deserialize)]
#[xattribute(name = "final")]
pub struct Final(pub DerivationSetType);

#[derive(Debug, Clone, Eq, PartialEq, SerializeAttribute, Deserialize)]
#[xattribute(name = "block")]
pub struct Block(pub DerivationSetType);

#[derive(Debug, Clone, Eq, PartialEq, SerializeAttribute, Deserialize, Default)]
#[xattribute(name = "defaultAttributesApply")]
pub struct DefaultAttributesApply(pub bool);

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[xelement(name = "complexType", namespace_expr = NS_XSD)]
pub struct LocalComplexType {
    #[xattribute(default)]
    pub id: Option<Id>,
    #[xattribute(default)]
    pub mixed: Option<Mixed>,
    #[xattribute(default)]
    pub abstract_: Option<Abstract>,
    #[xattribute(default)]
    pub final_: Option<Final>,
    #[xattribute(default)]
    pub block: Option<Block>,
    #[xattribute(default)]
    pub default_attributes_apply: Option<DefaultAttributesApply>,
    #[xvalue(default)]
    pub annotation: Option<Annotation>,
    pub content: Vec<ComplexBaseTypeContent>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[xelement(name = "complexType", namespace_expr = NS_XSD)]
pub struct TopLevelComplexType {
    #[xattribute(default)]
    pub id: Option<Id>,
    #[xattribute(default)]
    pub name: Option<Name>,
    #[xattribute(default)]
    pub mixed: Option<Mixed>,
    #[xattribute(default)]
    pub abstract_: Option<Abstract>,
    #[xattribute(default)]
    pub final_: Option<Final>,
    #[xattribute(default)]
    pub block: Option<Block>,
    #[xattribute(default)]
    pub default_attributes_apply: Option<DefaultAttributesApply>,
    #[xvalue(default)]
    pub annotation: Option<Annotation>,
    pub content: Vec<ComplexBaseTypeContent>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[xelement(name = "group", namespace_expr = NS_XSD)]
pub struct GroupType {
    #[xattribute(default)]
    pub id: Option<Id>,
    #[xattribute(default)]
    pub name: Option<Name>,
    #[xattribute(default)]
    pub ref_: Option<Ref>,
    #[xattribute(default)]
    pub min_occurs: Option<MinOccurs>,
    #[xattribute(default)]
    pub max_occurs: Option<MaxOccurs>,
    #[xvalue(default)]
    pub annotation: Option<Annotation>,
    #[xvalue(default)]
    pub content: Vec<GroupTypeContent>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[xelement(name = "all", namespace_expr = NS_XSD)]
pub struct AllType {
    #[xattribute(default)]
    pub id: Option<Id>,
    #[xattribute(default)]
    pub name: Option<Name>,
    #[xattribute(default)]
    pub ref_: Option<Ref>,
    #[xattribute(default)]
    pub min_occurs: Option<MinOccurs>,
    #[xattribute(default)]
    pub max_occurs: Option<MaxOccurs>,
    pub content: Vec<GroupTypeContent>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[xelement(name = "choice", namespace_expr = NS_XSD)]
pub struct ChoiceType {
    #[xattribute(default)]
    pub id: Option<Id>,
    #[xattribute(default)]
    pub name: Option<Name>,
    #[xattribute(default)]
    pub ref_: Option<Ref>,
    #[xattribute(default)]
    pub min_occurs: Option<MinOccurs>,
    #[xattribute(default)]
    pub max_occurs: Option<MaxOccurs>,
    pub content: Vec<GroupTypeContent>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[xelement(name = "sequence", namespace_expr = NS_XSD)]
pub struct SequenceType {
    #[xattribute(default)]
    pub id: Option<Id>,
    #[xattribute(default)]
    pub name: Option<Name>,
    #[xattribute(default)]
    pub ref_: Option<Ref>,
    #[xattribute(default)]
    pub min_occurs: Option<MinOccurs>,
    #[xattribute(default)]
    pub max_occurs: Option<MaxOccurs>,
    pub content: Vec<GroupTypeContent>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[xelement(
    name = "attributeGroup",
    namespace_expr = NS_XSD
)]
pub struct AttributeGroupType {
    #[xattribute(default)]
    pub id: Option<Id>,
    #[xattribute(default)]
    pub name: Option<Name>,
    #[xattribute(default)]
    pub ref_: Option<Ref>,
    #[xvalue(default)]
    pub annotation: Option<Annotation>,
    #[xvalue(default)]
    pub content: Vec<AttributeGroupTypeContent>,
}
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum AttributeGroupTypeContent {
    Attribute(Box<LocalAttribute>),
    AttributeGroup(Box<AttributeGroupType>),
    AnyAttribute(Box<AnyAttribute>),
}

#[derive(Debug, Clone, Eq, PartialEq, SerializeAttribute, Deserialize)]
#[xattribute(name = "ref")]
pub struct Ref(pub QName);

#[derive(Debug, Clone, Eq, PartialEq, SerializeAttribute, Deserialize)]
#[xattribute(name = "type")]
pub struct Type(pub QName);

#[derive(Debug, Clone, Eq, PartialEq, SerializeAttribute, Deserialize)]
#[xattribute(name = "substitutionGroup")]
pub struct SubstitutionGroup(pub ElementSubstitutionGroupType);

#[derive(Debug, Clone, Eq, PartialEq, SerializeAttribute, Deserialize)]
#[xattribute(name = "default")]
pub struct DefaultAttr(pub String);

#[derive(Debug, Clone, Eq, PartialEq, SerializeAttribute, Deserialize)]
#[xattribute(name = "fixed")]
pub struct Fixed(pub String);

#[derive(Debug, Clone, Eq, PartialEq, SerializeAttribute, Deserialize)]
#[xattribute(name = "nillable")]
pub struct Nillable(pub bool);

#[derive(Debug, Clone, Eq, PartialEq, SerializeAttribute, Deserialize)]
#[xattribute(name = "block")]
pub struct Block2(pub BlockSetType);

#[derive(Debug, Clone, Eq, PartialEq, SerializeAttribute, Deserialize)]
#[xattribute(name = "form")]
pub struct Form(pub FormChoiceType);

#[derive(Debug, Clone, Eq, PartialEq, SerializeAttribute, Deserialize)]
#[xattribute(name = "targetNamespace")]
pub struct TargetNamespace(pub String);

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[xelement(name = "element", namespace_expr = NS_XSD)]
pub struct LocalElement {
    #[xattribute(default)]
    pub id: Option<Id>,
    #[xattribute(default)]
    pub name: Option<Name>,
    #[xattribute(default)]
    pub ref_: Option<Ref>,
    #[xattribute(default)]
    pub type_: Option<Type>,
    #[xattribute(default)]
    pub substitution_group: Option<SubstitutionGroup>,
    #[xattribute(default)]
    pub min_occurs: Option<MinOccurs>,
    #[xattribute(default)]
    pub max_occurs: Option<MaxOccurs>,
    #[xattribute(default)]
    pub default: Option<DefaultAttr>,
    #[xattribute(default)]
    pub fixed: Option<Fixed>,
    #[xattribute(default)]
    pub nillable: Option<Nillable>,
    #[xattribute(default)]
    pub abstract_: Option<Abstract>,
    #[xattribute(default)]
    pub final_: Option<Final>,
    #[xattribute(default)]
    pub block: Option<Block2>,
    #[xattribute(default)]
    pub form: Option<Form>,
    #[xattribute(default)]
    pub target_namespace: Option<TargetNamespace>,
    #[xvalue(default)]
    pub annotation: Option<Annotation>,
    #[xvalue(default)]
    pub type_choice: Option<ElementTypeContent>,
    #[xvalue(default)]
    pub alternatives: Vec<AltType>,
    #[xvalue(default)]
    pub identity_constraints: Vec<IdentityConstraint>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[xelement(name = "element", namespace_expr = NS_XSD)]
pub struct TopLevelElement {
    #[xattribute(default)]
    pub id: Option<Id>,
    #[xattribute(default)]
    pub name: Option<Name>,
    #[xattribute(default)]
    pub type_: Option<Type>,
    #[xattribute(default)]
    pub substitution_group: Option<SubstitutionGroup>,
    #[xattribute(default)]
    pub min_occurs: Option<MinOccurs>,
    #[xattribute(default)]
    pub max_occurs: Option<MaxOccurs>,
    #[xattribute(default)]
    pub default: Option<DefaultAttr>,
    #[xattribute(default)]
    pub fixed: Option<Fixed>,
    #[xattribute(default)]
    pub nillable: Option<Nillable>,
    #[xattribute(default)]
    pub abstract_: Option<Abstract>,
    #[xattribute(default)]
    pub final_: Option<Final>,
    #[xattribute(default)]
    pub block: Option<Block2>,
    #[xattribute(default)]
    pub form: Option<Form>,
    #[xattribute(default)]
    pub target_namespace: Option<TargetNamespace>,
    #[xvalue(default)]
    pub annotation: Option<Annotation>,
    #[xvalue(default)]
    pub type_choice: Option<ElementTypeContent>,
    #[xvalue(default)]
    pub alternatives: Vec<AltType>,
    #[xvalue(default)]
    pub identity_constraints: Vec<IdentityConstraint>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum ElementTypeContent {
    SimpleType(Box<LocalSimpleType>),
    ComplexType(Box<LocalComplexType>),
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Eq, PartialEq, SerializeAttribute, Deserialize)]
#[xattribute(name = "use")]
pub struct AttrUse(pub AttributeUseType);

#[derive(Debug, Clone, Eq, PartialEq, SerializeAttribute, Deserialize)]
#[xattribute(name = "inheritable")]
pub struct Inheritable(pub bool);

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[xelement(name = "attribute", namespace_expr = NS_XSD)]
pub struct LocalAttribute {
    #[xattribute(default)]
    pub id: Option<Id>,
    #[xattribute(default)]
    pub name: Option<Name>,
    #[xattribute(default)]
    pub ref_: Option<Ref>,
    #[xattribute(default)]
    pub type_: Option<Type>,
    #[xattribute(default)]
    pub use_: Option<AttrUse>,
    #[xattribute(default)]
    pub default: Option<DefaultAttr>,
    #[xattribute(default)]
    pub fixed: Option<Fixed>,
    #[xattribute(default)]
    pub form: Option<Form>,
    #[xattribute(default)]
    pub target_namespace: Option<TargetNamespace>,
    #[xattribute(default)]
    pub inheritable: Option<Inheritable>,
    #[xvalue(default)]
    pub annotation: Option<Annotation>,
    #[xvalue(default)]
    pub simple_type: Option<LocalSimpleType>,
}
impl LocalAttribute {
    #[must_use]
    pub fn default_use_() -> AttributeUseType {
        AttributeUseType::Optional
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[xelement(name = "attribute", namespace_expr = NS_XSD)]
pub struct TopLevelAttribute {
    #[xattribute(default)]
    pub id: Option<Id>,
    #[xattribute(default)]
    pub name: Option<Name>,
    #[xattribute(default)]
    pub type_: Option<Type>,
    #[xattribute(default)]
    pub use_: Option<AttrUse>,
    #[xattribute(default)]
    pub default: Option<DefaultAttr>,
    #[xattribute(default)]
    pub fixed: Option<Fixed>,
    #[xattribute(default)]
    pub form: Option<Form>,
    #[xattribute(default)]
    pub target_namespace: Option<TargetNamespace>,
    #[xattribute(default)]
    pub inheritable: Option<Inheritable>,
    #[xvalue(default)]
    pub annotation: Option<Annotation>,
    #[xvalue(default)]
    pub simple_type: Option<LocalSimpleType>,
}
impl TopLevelAttribute {
    #[must_use]
    pub fn default_use_() -> AttributeUseType {
        AttributeUseType::Optional
    }
}

#[derive(Debug, Clone, Eq, PartialEq, SerializeAttribute, Deserialize)]
#[xattribute(name = "public")]
pub struct Public(pub String);

#[derive(Debug, Clone, Eq, PartialEq, SerializeAttribute, Deserialize)]
#[xattribute(name = "system")]
pub struct System(pub String);

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[xelement(name = "notation", namespace_expr = NS_XSD)]
pub struct Notation {
    #[xattribute(default)]
    pub id: Option<Id>,
    #[xattribute]
    pub name: Name,
    #[xattribute(default)]
    pub public: Option<Public>,
    #[xattribute(default)]
    pub system: Option<System>,
    #[xvalue(default)]
    pub annotation: Option<Annotation>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum TypeDerivationControlType {
    Extension,
    Restriction,
    List,
    Union,
}
impl FromStr for TypeDerivationControlType {
    type Err = Error;

    fn from_str(bytes: &str) -> Result<Self, Self::Err> {
        match bytes {
            "extension" => Ok(Self::Extension),
            "restriction" => Ok(Self::Restriction),
            "list" => Ok(Self::List),
            "union" => Ok(Self::Union),
            bytes => Err(Error::UnknownOrInvalidValue {
                value: bytes.to_string(),
            }),
        }
    }
}

impl Display for TypeDerivationControlType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Extension => write!(f, "extension"),
            Self::Restriction => write!(f, "restriction"),
            Self::List => write!(f, "list"),
            Self::Union => write!(f, "union"),
        }
    }
}

impl_from_str_deserialize!(TypeDerivationControlType);
impl_to_string_serialize!(TypeDerivationControlType);

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum BlockSetItemType {
    Extension,
    Restriction,
    Substitution,
}
impl FromStr for BlockSetItemType {
    type Err = Error;

    fn from_str(bytes: &str) -> Result<Self, Self::Err> {
        match bytes {
            "extension" => Ok(Self::Extension),
            "restriction" => Ok(Self::Restriction),
            "substitution" => Ok(Self::Substitution),
            bytes => Err(Error::UnknownOrInvalidValue {
                value: bytes.to_string(),
            }),
        }
    }
}

impl Display for BlockSetItemType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Extension => write!(f, "extension"),
            Self::Restriction => write!(f, "restriction"),
            Self::Substitution => write!(f, "substitution"),
        }
    }
}

impl_from_str_deserialize!(BlockSetItemType);
impl_to_string_serialize!(BlockSetItemType);

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[xelement(name = "appinfo", namespace_expr = NS_XSD)]
pub struct Appinfo {
    #[xattribute(default)]
    pub source: Option<Source>,
}

#[derive(Debug, Clone, Eq, PartialEq, SerializeAttribute, Deserialize)]
#[xattribute(name = "source")]
pub struct Source(pub String);

#[derive(Debug, Clone, Eq, PartialEq, SerializeAttribute, Deserialize)]
#[xattribute(name = "lang", namespace_expr = NS_XML)]
pub struct XmlLang(pub String);

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[xelement(name = "documentation", namespace_expr = NS_XSD)]
pub struct Documentation {
    #[xattribute(default)]
    pub source: Option<Source>,
    #[xattribute(default)]
    pub lang: Option<XmlLang>,
    #[xvalue(default)]
    pub any: Vec<xmlity::XmlValue>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum DefaultOpenContentModeType {
    Interleave,
    Suffix,
}
impl FromStr for DefaultOpenContentModeType {
    type Err = Error;

    fn from_str(bytes: &str) -> Result<Self, Self::Err> {
        match bytes {
            "interleave" => Ok(Self::Interleave),
            "suffix" => Ok(Self::Suffix),
            bytes => Err(Error::UnknownOrInvalidValue {
                value: bytes.to_string(),
            }),
        }
    }
}

impl Display for DefaultOpenContentModeType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Interleave => write!(f, "interleave"),
            Self::Suffix => write!(f, "suffix"),
        }
    }
}

impl_from_str_deserialize!(DefaultOpenContentModeType);
impl_to_string_serialize!(DefaultOpenContentModeType);

#[derive(Debug, Clone, Eq, PartialEq, SerializationGroup, DeserializationGroup)]
pub struct WildcardType {
    #[xattribute]
    pub id: Option<Id>,
    pub namespace: Option<NamespaceListType>,
    pub not_namespace: Option<BasicNamespaceListType>,
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
    SimpleDerivationSetItemList(SimpleDerivationSetItemList),
}
impl FromStr for SimpleDerivationSetType {
    type Err = Error;

    fn from_str(bytes: &str) -> Result<Self, Self::Err> {
        match bytes {
            "#all" => Ok(Self::All),
            bytes => Ok(Self::SimpleDerivationSetItemList(
                SimpleDerivationSetItemList::from_str(bytes)?,
            )),
        }
    }
}

impl Display for SimpleDerivationSetType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::All => f.write_str("#all"),
            Self::SimpleDerivationSetItemList(list) => list.fmt(f),
        }
    }
}

impl_from_str_deserialize!(SimpleDerivationSetType);
impl_to_string_serialize!(SimpleDerivationSetType);

#[derive(Debug, Clone, Eq, PartialEq, SerializeAttribute, Deserialize)]
#[xattribute(name = "itemType")]
pub struct ItemType(pub QName);

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[xelement(name = "list", namespace_expr = NS_XSD)]
pub struct List {
    #[xattribute(default)]
    pub id: Option<Id>,
    #[xvalue(default)]
    pub annotation: Option<Annotation>,
    #[xvalue(default)]
    pub simple_type: Option<LocalSimpleType>,
    #[xattribute(default)]
    pub item_type: Option<ItemType>,
}

#[derive(Debug, Clone, Eq, PartialEq, SerializeAttribute, Deserialize)]
#[xattribute(name = "memberTypes")]
pub struct MemberTypes(pub ElementSubstitutionGroupType);

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[xelement(name = "union", namespace_expr = NS_XSD)]
pub struct Union {
    #[xattribute(default)]
    pub id: Option<Id>,
    #[xvalue(default)]
    pub annotation: Option<Annotation>,
    #[xvalue(default)]
    pub simple_types: Vec<LocalSimpleType>,

    #[xattribute(default)]
    pub member_types: Option<MemberTypes>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum DerivationSetType {
    All,
    ReducedDerivationControlList(ReducedDerivationControlList),
}
impl FromStr for DerivationSetType {
    type Err = Error;

    fn from_str(bytes: &str) -> Result<Self, Self::Err> {
        match bytes {
            "#all" => Ok(Self::All),
            bytes => Ok(Self::ReducedDerivationControlList(
                ReducedDerivationControlList::from_str(bytes)?,
            )),
        }
    }
}

impl fmt::Display for DerivationSetType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::All => write!(f, "#all"),
            Self::ReducedDerivationControlList(inner) => write!(f, "{}", inner),
        }
    }
}

impl_from_str_deserialize!(DerivationSetType);
impl_to_string_serialize!(DerivationSetType);

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[xelement(name = "simpleContent", namespace_expr = NS_XSD)]
pub struct SimpleContent {
    #[xattribute]
    pub id: Option<Id>,
    pub content: Vec<SimpleContentContent>,
}
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum SimpleContentContent {
    Annotation(Box<Annotation>),
    Restriction(Box<Restriction>),
    Extension(Box<ExtensionType>),
}
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[xelement(
    name = "complexContent",
    namespace_expr = NS_XSD
)]
pub struct ComplexContent {
    #[xattribute(default)]
    pub id: Option<Id>,
    #[xattribute(default)]
    pub mixed: Option<Mixed>,
    pub content: Vec<ComplexContentContent>,
}
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum ComplexContentContent {
    Annotation(Box<Annotation>),
    Restriction(Box<ComplexRestrictionType>),
    Extension(Box<ExtensionType>),
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[xelement(name = "any", namespace_expr = NS_XSD)]
pub struct AnyType {
    #[xgroup]
    pub wildcard: WildcardType,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[xelement(name = "openContent", namespace_expr = NS_XSD)]
pub struct OpenContent {
    #[xattribute]
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

#[derive(Debug, Clone, Eq, PartialEq, SerializeAttribute, Deserialize)]
#[xattribute(name = "namespace")]
pub struct NamespaceAnyAttribute(pub NamespaceListType);

#[derive(Debug, Clone, Eq, PartialEq, SerializeAttribute, Deserialize)]
#[xattribute(name = "notNamespace")]
pub struct NotNamespaceAnyAttribute(pub NamespaceListType);

#[derive(Debug, Clone, Eq, PartialEq, SerializeAttribute, Deserialize)]
#[xattribute(name = "processContents")]
pub struct ProcessContents(pub ProcessContentsType);

#[derive(Debug, Clone, Eq, PartialEq, SerializeAttribute, Deserialize)]
#[xattribute(name = "notQName")]
pub struct NotQName(pub QnameListAType);

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[xelement(name = "anyAttribute", namespace_expr = NS_XSD)]
pub struct AnyAttribute {
    #[xattribute(default)]
    pub id: Option<Id>,
    #[xattribute(default)]
    pub namespace: Option<NamespaceAnyAttribute>,
    #[xattribute(default)]
    pub not_namespace: Option<NotNamespaceAnyAttribute>,
    #[xattribute]
    pub process_contents: ProcessContents,
    #[xattribute(default)]
    pub not_q_name: Option<NotQName>,
    #[xvalue(default)]
    pub annotation: Option<Annotation>,
}
impl AnyAttribute {
    #[must_use]
    pub fn default_process_contents() -> ProcessContentsType {
        ProcessContentsType::Strict
    }
}
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[xelement(name = "assertion", namespace_expr = NS_XSD)]
pub struct AssertionType {
    #[xattribute(default)]
    pub id: Option<Id>,
    #[xattribute(default)]
    pub test: Option<Test>,
    #[xattribute(default)]
    #[xattribute(default)]
    pub xpath_default_namespace: Option<XPathDefaultNamespace>,
    #[xvalue(default)]
    pub annotation: Option<Annotation>,
}

#[derive(Debug, Clone, Eq, PartialEq, SerializeAttribute, Deserialize)]
#[xattribute(name = "namespace")]
pub struct AnyNamespace(pub NamespaceListType);

#[derive(Debug, Clone, Eq, PartialEq, SerializeAttribute, Deserialize)]
#[xattribute(name = "notNamespace")]
pub struct BasicNotNamespaceAnyAttribute(pub BasicNamespaceListType);

#[derive(Debug, Clone, Eq, PartialEq, SerializeAttribute, Deserialize)]
#[xattribute(name = "notQName")]
pub struct NotQNameAnyAttribute(pub QnameListType);

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[xelement(name = "any", namespace_expr = NS_XSD)]
pub struct Any {
    #[xattribute(default)]
    pub id: Option<Id>,
    #[xattribute(default)]
    pub namespace: Option<AnyNamespace>,
    #[xattribute(default)]
    pub not_namespace: Option<NotNamespaceAnyAttribute>,
    #[xattribute]
    pub process_contents: ProcessContents,
    #[xattribute(default)]
    pub not_q_name: Option<NotQNameAnyAttribute>,
    #[xattribute(default)]
    pub min_occurs: Option<MinOccurs>,
    #[xattribute(default)]
    pub max_occurs: Option<MaxOccurs>,
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

#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct ElementSubstitutionGroupType(pub Vec<QName>);
impl FromStr for ElementSubstitutionGroupType {
    type Err = <QName as FromStr>::Err;

    fn from_str(bytes: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            bytes
                .split([' ', '|', ',', ';'])
                .map(QName::from_str)
                .collect::<Result<Vec<_>, _>>()?,
        ))
    }
}

impl Display for ElementSubstitutionGroupType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .map(|a| a.to_string())
                .collect::<Vec<_>>()
                .join(" ")
        )
    }
}

impl_from_str_deserialize!(ElementSubstitutionGroupType);
impl_to_string_serialize!(ElementSubstitutionGroupType);

#[derive(Debug, Clone, Eq, PartialEq, SerializeAttribute, Deserialize)]
#[xattribute(name = "test")]
pub struct Test(pub String);

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[xelement(name = "alternative", namespace_expr = NS_XSD)]
pub struct AltType {
    #[xattribute(default)]
    pub id: Option<Id>,
    #[xattribute(default)]
    pub test: Option<Test>,
    #[xattribute(default)]
    pub type_: Option<Type>,
    #[xattribute(default)]
    pub xpath_default_namespace: Option<XPathDefaultNamespace>,
    #[xvalue(default)]
    pub content: Vec<AltTypeContent>,
}
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum AltTypeContent {
    Annotation(Box<Annotation>),
    SimpleType(Box<LocalSimpleType>),
    ComplexType(Box<LocalComplexType>),
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[xelement(name = "unique", namespace_expr = NS_XSD)]
pub struct Unique {
    #[xattribute(default)]
    pub id: Option<Id>,
    #[xattribute(default)]
    pub name: Option<Name>,
    #[xattribute(default)]
    pub ref_: Option<Ref>,
    #[xgroup]
    pub content: Option<KeybaseTypeContent>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[xelement(name = "key", namespace_expr = NS_XSD)]
pub struct Key {
    #[xattribute(default)]
    pub id: Option<Id>,
    #[xattribute(default)]
    pub name: Option<Name>,
    #[xattribute(default)]
    pub ref_: Option<Ref>,
    // #[xgroup]
    // pub content: Option<KeybaseTypeContent>,
    pub selector: Selector,
    pub field: Vec<Field>,
}

#[derive(Debug, Clone, Eq, PartialEq, SerializationGroup, DeserializationGroup)]
pub struct KeybaseTypeContent {
    #[xvalue(default)]
    pub annotations: Vec<Annotation>,
    pub selector: Selector,
    pub field: Vec<Field>,
}

#[derive(Debug, Clone, Eq, PartialEq, SerializeAttribute, Deserialize)]
#[xattribute(name = "refer")]
pub struct Refer(pub QName);

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[xelement(name = "keyref", namespace_expr = NS_XSD)]
pub struct Keyref {
    #[xattribute(default)]
    pub id: Option<Id>,
    #[xattribute(default)]
    pub name: Option<Name>,
    #[xattribute(default)]
    pub ref_: Option<Ref>,
    #[xattribute(default)]
    pub refer: Option<Refer>,
    #[xgroup]
    pub content: Option<KeybaseTypeContent>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum AttributeUseType {
    Prohibited,
    Optional,
    Required,
}
impl FromStr for AttributeUseType {
    type Err = Error;

    fn from_str(bytes: &str) -> Result<Self, Self::Err> {
        match bytes {
            "prohibited" => Ok(Self::Prohibited),
            "optional" => Ok(Self::Optional),
            "required" => Ok(Self::Required),
            bytes => Err(Error::UnknownOrInvalidValue {
                value: bytes.to_string(),
            }),
        }
    }
}

impl Display for AttributeUseType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Prohibited => write!(f, "prohibited"),
            Self::Optional => write!(f, "optional"),
            Self::Required => write!(f, "required"),
        }
    }
}

impl_from_str_deserialize!(AttributeUseType);
impl_to_string_serialize!(AttributeUseType);

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum NamespaceListType {
    Any,
    Other,
    BasicNamespaceList(BasicNamespaceListType),
}
impl FromStr for NamespaceListType {
    type Err = Error;

    fn from_str(bytes: &str) -> Result<Self, Self::Err> {
        match bytes {
            "##any" => Ok(Self::Any),
            "##other" => Ok(Self::Other),
            bytes => Ok(Self::BasicNamespaceList(BasicNamespaceListType::from_str(
                bytes,
            )?)),
        }
    }
}

impl Display for NamespaceListType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Any => write!(f, "##any"),
            Self::Other => write!(f, "##other"),
            Self::BasicNamespaceList(list) => write!(f, "{}", list),
        }
    }
}

impl_from_str_deserialize!(NamespaceListType);
impl_to_string_serialize!(NamespaceListType);

#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct BasicNamespaceListType(pub Vec<BasicNamespaceListItemType>);
impl FromStr for BasicNamespaceListType {
    type Err = Error;

    fn from_str(bytes: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            bytes
                .split([' ', '|', ',', ';'])
                .map(BasicNamespaceListItemType::from_str)
                .collect::<Result<Vec<_>, _>>()?,
        ))
    }
}

impl Display for BasicNamespaceListType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join(" ")
        )
    }
}

impl_from_str_deserialize!(BasicNamespaceListType);
impl_to_string_serialize!(BasicNamespaceListType);

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ProcessContentsType {
    Skip,
    Lax,
    Strict,
}
impl FromStr for ProcessContentsType {
    type Err = Error;

    fn from_str(bytes: &str) -> Result<Self, Self::Err> {
        match bytes {
            "skip" => Ok(Self::Skip),
            "lax" => Ok(Self::Lax),
            "strict" => Ok(Self::Strict),
            bytes => Err(Error::UnknownOrInvalidValue {
                value: bytes.to_string(),
            }),
        }
    }
}
impl Display for ProcessContentsType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Skip => "skip",
                Self::Lax => "lax",
                Self::Strict => "strict",
            }
        )
    }
}

impl_from_str_deserialize!(ProcessContentsType);
impl_to_string_serialize!(ProcessContentsType);

#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct SimpleDerivationSetItemList(pub Vec<SimpleDerivationSetItemType>);
impl FromStr for SimpleDerivationSetItemList {
    type Err = Error;

    fn from_str(bytes: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            bytes
                .split([' ', '|', ',', ';'])
                .map(SimpleDerivationSetItemType::from_str)
                .collect::<Result<Vec<_>, _>>()?,
        ))
    }
}

impl Display for SimpleDerivationSetItemList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join(" ")
        )
    }
}

#[derive(Debug, Clone, Eq, PartialEq, SerializeAttribute, Deserialize)]
#[xattribute(name = "value")]
pub struct ValueAttr(pub String);

impl_from_str_deserialize!(SimpleDerivationSetItemList);
impl_to_string_serialize!(SimpleDerivationSetItemList);

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[xelement(name = "enumeration", namespace_expr = NS_XSD)]
pub struct Enumeration {
    #[xattribute(default)]
    pub fixed: Option<Fixed>,
    #[xattribute]
    pub value: ValueAttr,
}

#[derive(Debug, Clone, Eq, PartialEq, SerializationGroup, DeserializationGroup)]
#[xgroup]
pub struct FacetType {
    #[xattribute(default)]
    pub fixed: Option<Fixed>,
    #[xvalue(default)]
    pub annotation: Option<Annotation>,
    #[xattribute]
    pub value: ValueAttr,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[xelement(name = "minExclusive", namespace_expr = NS_XSD)]
pub struct MinExclusive {
    #[xgroup]
    pub facet_type: FacetType,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[xelement(name = "minInclusive", namespace_expr = NS_XSD)]
pub struct MinInclusive {
    #[xgroup]
    pub facet_type: FacetType,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[xelement(name = "maxExclusive", namespace_expr = NS_XSD)]
pub struct MaxExclusive {
    #[xgroup]
    pub facet_type: FacetType,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[xelement(name = "maxInclusive", namespace_expr = NS_XSD)]
pub struct MaxInclusive {
    #[xgroup]
    pub facet_type: FacetType,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct ReducedDerivationControlList(pub Vec<ReducedDerivationControlType>);
impl FromStr for ReducedDerivationControlList {
    type Err = Error;

    fn from_str(bytes: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            bytes
                .split([' ', '|', ',', ';'])
                .map(ReducedDerivationControlType::from_str)
                .collect::<Result<Vec<_>, _>>()?,
        ))
    }
}

impl Display for ReducedDerivationControlList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join(" ")
        )
    }
}

impl_from_str_deserialize!(ReducedDerivationControlList);
impl_to_string_serialize!(ReducedDerivationControlList);

#[derive(Debug, Clone, Eq, PartialEq, SerializeAttribute, Deserialize)]
#[xattribute(name = "base")]
pub struct Base(pub QName);

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[xelement(name = "restriction", namespace_expr = NS_XSD)]
pub struct Restriction {
    #[xattribute(default)]
    pub id: Option<Id>,
    #[xattribute]
    pub base: Base,
    #[xvalue(default)]
    pub content: Vec<RestrictionTypeContent>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[xelement(name = "restriction", namespace_expr = NS_XSD)]
pub struct LocalRestriction {
    #[xattribute(default)]
    pub id: Option<Id>,
    #[xattribute]
    pub base: Base,
    #[xvalue(default)]
    pub annotation: Option<Annotation>,
    #[xvalue(default)]
    pub simple_type: Option<LocalSimpleType>,
    #[xvalue(default)]
    pub facets: Vec<Facet>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[xelement(name = "restriction", namespace_expr = NS_XSD)]
pub struct SimpleRestrictionType {
    #[xattribute(default)]
    pub id: Option<Id>,
    #[xattribute]
    pub base: Base,
    #[xvalue(default)]
    pub annotations: Option<Annotation>,
    #[xvalue(default)]
    pub simple_type: Option<LocalSimpleType>,
    #[xvalue(default)]
    pub facets: Vec<Facet>,
    //attrDecals
    //assertions
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[xelement(name = "restriction", namespace_expr = NS_XSD)]
pub struct ComplexRestrictionType {
    #[xattribute(default)]
    pub id: Option<Id>,
    #[xvalue(default)]
    pub annotation: Option<Annotation>,
    #[xattribute(default)]
    pub base: Option<Base>,
    #[xvalue(default)]
    pub simple_type: Option<LocalSimpleType>,
    #[xvalue(default)]
    pub content: Vec<RestrictionContent>,
    // #[xvalue(default)]
    // pub attr_decls: Option<AttrDeclsType>,
    // #[xvalue(default)]
    // pub assertions: Option<AssertionsType>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum RestrictionContent {
    // Facet(Box<Facet>),
    //TODO: Huh?
    Other(Box<XmlValue>),
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[xelement(name = "extension", namespace_expr = NS_XSD)]
pub struct ExtensionType {
    #[xattribute(default)]
    pub id: Option<Id>,
    #[xattribute]
    pub base: Base,
    #[xvalue(default)]
    pub content: Vec<ExtensionTypeContent>,
}
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
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
impl FromStr for OpenContentModeType {
    type Err = Error;

    fn from_str(bytes: &str) -> Result<Self, Self::Err> {
        match bytes {
            "none" => Ok(Self::None),
            "interleave" => Ok(Self::Interleave),
            "suffix" => Ok(Self::Suffix),
            bytes => Err(Error::UnknownOrInvalidValue {
                value: bytes.to_string(),
            }),
        }
    }
}

impl Display for OpenContentModeType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            OpenContentModeType::None => write!(f, "none"),
            OpenContentModeType::Interleave => write!(f, "interleave"),
            OpenContentModeType::Suffix => write!(f, "suffix"),
        }
    }
}

impl_from_str_deserialize!(OpenContentModeType);
impl_to_string_serialize!(OpenContentModeType);

#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct QnameListAType(pub Vec<QnameListAItemType>);
impl FromStr for QnameListAType {
    type Err = <QnameListAItemType as FromStr>::Err;

    fn from_str(bytes: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            bytes
                .split([' ', '|', ',', ';'])
                .map(QnameListAItemType::from_str)
                .collect::<Result<Vec<_>, _>>()?,
        ))
    }
}

impl Display for QnameListAType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join(" ")
        )
    }
}

impl_from_str_deserialize!(QnameListAType);
impl_to_string_serialize!(QnameListAType);

#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct QnameListType(pub Vec<QnameListItemType>);
impl FromStr for QnameListType {
    type Err = <QnameListItemType as FromStr>::Err;

    fn from_str(bytes: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            bytes
                .split([' ', '|', ',', ';'])
                .map(QnameListItemType::from_str)
                .collect::<Result<Vec<_>, _>>()?,
        ))
    }
}

impl Display for QnameListType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .map(|a| a.to_string())
                .collect::<Vec<_>>()
                .join(" ")
        )
    }
}

impl_from_str_deserialize!(QnameListType);
impl_to_string_serialize!(QnameListType);

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[xelement(name = "field", namespace_expr = NS_XSD)]
pub struct Field {
    #[xattribute(default)]
    pub id: Option<Id>,
    #[xattribute]
    pub xpath: XPath,
    #[xattribute(default)]
    pub xpath_default_namespace: Option<XPathDefaultNamespace>,
    #[xvalue(default)]
    pub annotation: Option<Annotation>,
}

#[derive(Debug, Clone, Eq, PartialEq, SerializeAttribute, Deserialize)]
#[xattribute(name = "xpath")]
pub struct XPath(pub String);

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[xelement(name = "selector", namespace_expr = NS_XSD)]
pub struct Selector {
    #[xattribute(default)]
    pub id: Option<Id>,
    #[xattribute]
    pub xpath: XPath,
    #[xattribute(default)]
    pub xpath_default_namespace: Option<XPathDefaultNamespace>,
    #[xvalue(default)]
    pub annotations: Vec<Annotation>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum BasicNamespaceListItemType {
    String(String),
    TargetNamespace,
    Local,
}
impl FromStr for BasicNamespaceListItemType {
    type Err = Error;

    fn from_str(bytes: &str) -> Result<Self, Self::Err> {
        match bytes {
            "##targetNamespace" => Ok(Self::TargetNamespace),
            "##local" => Ok(Self::Local),
            bytes => Ok(Self::String(bytes.to_string())),
        }
    }
}

impl Display for BasicNamespaceListItemType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::String(string) => write!(f, "{}", string),
            Self::TargetNamespace => write!(f, "##targetNamespace"),
            Self::Local => write!(f, "##local"),
        }
    }
}

impl_from_str_deserialize!(BasicNamespaceListItemType);
impl_to_string_serialize!(BasicNamespaceListItemType);

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum SimpleDerivationSetItemType {
    List,
    Union,
    Restriction,
    Extension,
}
impl FromStr for SimpleDerivationSetItemType {
    type Err = Error;

    fn from_str(bytes: &str) -> Result<Self, Self::Err> {
        match bytes {
            "list" => Ok(Self::List),
            "union" => Ok(Self::Union),
            "restriction" => Ok(Self::Restriction),
            "extension" => Ok(Self::Extension),
            bytes => Err(Error::UnknownOrInvalidValue {
                value: bytes.to_string(),
            }),
        }
    }
}

impl Display for SimpleDerivationSetItemType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::List => write!(f, "list"),
            Self::Union => write!(f, "union"),
            Self::Restriction => write!(f, "restriction"),
            Self::Extension => write!(f, "extension"),
        }
    }
}

impl_from_str_deserialize!(SimpleDerivationSetItemType);
impl_to_string_serialize!(SimpleDerivationSetItemType);
// impl FacetType {
//     #[must_use]
//     pub fn default_fixed() -> bool {
//         false
//     }
// }
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ReducedDerivationControlType {
    Extension,
    Restriction,
}
impl FromStr for ReducedDerivationControlType {
    type Err = Error;

    fn from_str(bytes: &str) -> Result<Self, Self::Err> {
        match bytes {
            "extension" => Ok(Self::Extension),
            "restriction" => Ok(Self::Restriction),
            bytes => Err(Error::UnknownOrInvalidValue {
                value: bytes.to_string(),
            }),
        }
    }
}

impl Display for ReducedDerivationControlType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Extension => write!(f, "extension"),
            Self::Restriction => write!(f, "restriction"),
        }
    }
}

impl_from_str_deserialize!(ReducedDerivationControlType);
impl_to_string_serialize!(ReducedDerivationControlType);

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum QnameListAItemType {
    Qname(QName),
    Defined,
}
impl FromStr for QnameListAItemType {
    type Err = <QName as FromStr>::Err;

    fn from_str(bytes: &str) -> Result<Self, Self::Err> {
        match bytes {
            "##defined" => Ok(Self::Defined),
            bytes => QName::from_str(bytes).map(Self::Qname),
        }
    }
}

impl Display for QnameListAItemType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Qname(qname) => write!(f, "{qname}"),
            Self::Defined => write!(f, "##defined"),
        }
    }
}

impl_from_str_deserialize!(QnameListAItemType);
impl_to_string_serialize!(QnameListAItemType);

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum QnameListItemType {
    Qname(QName),
    Defined,
    DefinedSibling,
}
impl FromStr for QnameListItemType {
    type Err = <QName as FromStr>::Err;

    fn from_str(bytes: &str) -> Result<Self, Self::Err> {
        match bytes {
            "##defined" => Ok(Self::Defined),
            "##definedSibling" => Ok(Self::DefinedSibling),
            bytes => QName::from_str(bytes).map(Self::Qname),
        }
    }
}

impl Display for QnameListItemType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Defined => write!(f, "##defined"),
            Self::DefinedSibling => write!(f, "##definedSibling"),
            Self::Qname(n) => write!(f, "{n}"),
        }
    }
}

impl_from_str_deserialize!(QnameListItemType);
impl_to_string_serialize!(QnameListItemType);

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(
        r###"
        <xs:documentation xmlns:xs="http://www.w3.org/2001/XMLSchema">
            Part 1 version: structures.xsd (rec-20120405)
            Part 2 version: datatypes.xsd (rec-20120405)
        </xs:documentation>
        "###
    )]
    fn deserialize_documentation(#[case] xml: &str) {
        let result: Documentation = xmlity_quick_xml::from_str(xml).unwrap();

        println!("{result:#?}");
    }

    #[rstest]
    #[case(
        r###"
        <xs:documentation xmlns:xs="http://www.w3.org/2001/XMLSchema"
            source="../structures/structures.html#element-schema"/>
        "###
    )]
    fn deserialize_documentation_source(#[case] xml: &str) {
        let result: Documentation = xmlity_quick_xml::from_str(xml).unwrap();

        println!("{result:#?}");
    }

    #[rstest]
    #[case(
        r###"
        <xs:annotation xmlns:xs="http://www.w3.org/2001/XMLSchema">
            <xs:documentation>
                Part 1 version: structures.xsd (rec-20120405)
                Part 2 version: datatypes.xsd (rec-20120405)
            </xs:documentation>
        </xs:annotation>
        "###
    )]
    #[case(
        r###"
        <xs:annotation xmlns:xs="http://www.w3.org/2001/XMLSchema">
            <xs:documentation
                source="../structures/structures.html#element-defaultOpenContent"/>
        </xs:annotation>
        "###
    )]
    #[case(
        r###"
    <xs:annotation xmlns:xs="http://www.w3.org/2001/XMLSchema">
            <xs:documentation>
        A utility type, not for public use</xs:documentation>
            <xs:documentation>
        A public identifier, per ISO 8879</xs:documentation>
        </xs:annotation>
        "###
    )]

    fn deserialize_annotation(#[case] xml: &str) {
        let result: Annotation = xmlity_quick_xml::from_str(xml).unwrap();

        println!("{result:#?}");
    }

    #[rstest]
    #[case(
        r###"
        <xs:annotation xmlns:xs="http://www.w3.org/2001/XMLSchema">
            <xs:documentation
                source="../structures/structures.html#element-schema"/>
        </xs:annotation>
        "###
    )]
    fn deserialize_annotation_source(#[case] xml: &str) {
        let result: Annotation = xmlity_quick_xml::from_str(xml).unwrap();

        println!("{result:#?}");
    }

    #[rstest]
    #[case(r###"
        <xs:import xmlns:xs="http://www.w3.org/2001/XMLSchema" namespace="http://www.w3.org/XML/1998/namespace"
                schemaLocation="xml.xsd">
            <xs:annotation>
                <xs:documentation>
                Get access to the xml: attribute groups for xml:lang
                as declared on 'schema' and 'documentation' below
                </xs:documentation>
            </xs:annotation>
        </xs:import>
        "###)]
    fn deserialize_import(#[case] xml: &str) {
        let result: Import = xmlity_quick_xml::from_str(xml).unwrap();

        println!("{result:#?}");
    }

    #[rstest]
    #[case(
        r###"
    <xs:complexType xmlns:xs="http://www.w3.org/2001/XMLSchema" name="openAttrs">
        <xs:annotation>
            <xs:documentation>
            This type is extended by almost all schema types
            to allow attributes from other namespaces to be
            added to user schemas.
            </xs:documentation>
        </xs:annotation>
        <xs:complexContent>
            <xs:restriction base="xs:anyType">
                <xs:anyAttribute namespace="##other" processContents="lax"/>
            </xs:restriction>
        </xs:complexContent>
    </xs:complexType>
    "###
    )]
    #[case(
        r###"
        <xs:complexType xmlns:xs="http://www.w3.org/2001/XMLSchema">
            <xs:complexContent>
                <xs:extension base="xs:openAttrs">
                    <xs:sequence>
                        <xs:group ref="xs:composition" minOccurs="0" maxOccurs="unbounded"/>
                        <xs:sequence minOccurs="0">
                            <xs:element ref="xs:defaultOpenContent"/>
                            <xs:element ref="xs:annotation" minOccurs="0"
                                        maxOccurs="unbounded"/>
                        </xs:sequence>
                        <xs:sequence minOccurs="0" maxOccurs="unbounded">
                            <xs:group ref="xs:schemaTop"/>
                            <xs:element ref="xs:annotation" minOccurs="0"
                                        maxOccurs="unbounded"/>
                        </xs:sequence>
                    </xs:sequence>
                    <xs:attribute name="targetNamespace" type="xs:anyURI"/>
                    <xs:attribute name="version" type="xs:token"/>
                    <xs:attribute name="finalDefault" type="xs:fullDerivationSet"
                                    default="" use="optional"/>
                    <xs:attribute name="blockDefault" type="xs:blockSet" default=""
                                    use="optional"/>
                    <xs:attribute name="attributeFormDefault" type="xs:formChoice"
                                    default="unqualified" use="optional"/>
                    <xs:attribute name="elementFormDefault" type="xs:formChoice"
                                    default="unqualified" use="optional"/>
                    <xs:attribute name="defaultAttributes" type="xs:QName"/>
                    <xs:attribute name="xpathDefaultNamespace" type="xs:xpathDefaultNamespace"
                                    default="##local" use="optional"/>
                    <xs:attribute name="id" type="xs:ID"/>
                    <xs:attribute ref="xml:lang"/>
                </xs:extension>
            </xs:complexContent>
        </xs:complexType>
        "###
    )]
    #[case(
        r###"
        <xs:complexType xmlns:xs="http://www.w3.org/2001/XMLSchema">
            <xs:complexContent>
                <xs:extension base="xs:annotated">
                <xs:sequence>
                    <xs:element name="any" type="xs:wildcard"/>
                </xs:sequence>
                <xs:attribute name="appliesToEmpty" type="xs:boolean"
                                default="false" use="optional"/>
                <xs:attribute name="mode" default="interleave" use="optional">
                    <xs:simpleType>
                    <xs:restriction base="xs:NMTOKEN">
                        <xs:enumeration value="interleave"/>
                        <xs:enumeration value="suffix"/>
                    </xs:restriction>
                    </xs:simpleType>
                </xs:attribute>

                </xs:extension>
            </xs:complexContent>
        </xs:complexType>
        "###
    )]
    #[case(
        r###"
        <xs:complexType xmlns:xs="http://www.w3.org/2001/XMLSchema" name="complexBaseType" abstract="true">
            <xs:complexContent>
                <xs:extension base="xs:annotated">
                    <xs:group ref="xs:complexTypeModel"/>
                    <xs:attribute name="name" type="xs:NCName">
                    <xs:annotation>
                        <xs:documentation>
                Will be restricted to required or prohibited</xs:documentation>
                    </xs:annotation>
                    </xs:attribute>
                    <xs:attribute name="mixed" type="xs:boolean" use="optional">
                    <xs:annotation>
                        <xs:documentation>
                Not allowed if simpleContent child is chosen.
                May be overridden by setting on complexContent child.</xs:documentation>
                    </xs:annotation>
                    </xs:attribute>
                    <xs:attribute name="abstract" type="xs:boolean" default="false"
                                use="optional"/>
                    <xs:attribute name="final" type="xs:derivationSet"/>
                    <xs:attribute name="block" type="xs:derivationSet"/>
                    <xs:attribute name="defaultAttributesApply" type="xs:boolean"
                                default="true" use="optional"/>
                </xs:extension>
            </xs:complexContent>
        </xs:complexType>
        "###
    )]
    #[case(
        r###"
      <xs:complexType xmlns:xs="http://www.w3.org/2001/XMLSchema" name="annotated">
        <xs:annotation>
          <xs:documentation>
          This type is extended by all types which allow annotation
          other than &lt;schema> itself
        </xs:documentation>
        </xs:annotation>
        <xs:complexContent>
          <xs:extension base="xs:openAttrs">
            <xs:sequence>
              <xs:element ref="xs:annotation" minOccurs="0"/>
            </xs:sequence>
            <xs:attribute name="id" type="xs:ID"/>
          </xs:extension>
        </xs:complexContent>
      </xs:complexType>
      "###
    )]
    fn deserialize_complex_type(#[case] xml: &str) {
        let result: LocalComplexType = xmlity_quick_xml::from_str(xml).unwrap();

        println!("{result:#?}");
    }

    #[rstest]
    #[case(
        r###"
        <xs:sequence xmlns:xs="http://www.w3.org/2001/XMLSchema">
            <xs:element name="any" type="xs:wildcard"/>
        </xs:sequence>
        "###
    )]
    #[case(
        r###"
        <xs:attribute xmlns:xs="http://www.w3.org/2001/XMLSchema" name="appliesToEmpty" type="xs:boolean"
            default="false" use="optional"/>
        "###
    )]
    #[case(
        r###"
        <xs:attribute xmlns:xs="http://www.w3.org/2001/XMLSchema" name="mode" default="interleave" use="optional">
            <xs:simpleType>
            <xs:restriction base="xs:NMTOKEN">
                <xs:enumeration value="interleave"/>
                <xs:enumeration value="suffix"/>
            </xs:restriction>
            </xs:simpleType>
        </xs:attribute>
        "###
    )]
    // #[case(
    //     r###"
    //     "###
    // )]

    fn deserialize_extension_content(#[case] xml: &str) {
        let result: ExtensionTypeContent = xmlity_quick_xml::from_str(xml).unwrap();

        println!("{result:#?}");
    }

    #[rstest]
    #[case(
        r###"
        <xs:extension xmlns:xs="http://www.w3.org/2001/XMLSchema" base="xs:openAttrs">
            <xs:sequence>
                <xs:group ref="xs:composition" minOccurs="0" maxOccurs="unbounded"/>
                <xs:sequence minOccurs="0">
                    <xs:element ref="xs:defaultOpenContent"/>
                    <xs:element ref="xs:annotation" minOccurs="0"
                                maxOccurs="unbounded"/>
                </xs:sequence>
                <xs:sequence minOccurs="0" maxOccurs="unbounded">
                    <xs:group ref="xs:schemaTop"/>
                    <xs:element ref="xs:annotation" minOccurs="0"
                                maxOccurs="unbounded"/>
                </xs:sequence>
            </xs:sequence>
            <xs:attribute name="targetNamespace" type="xs:anyURI"/>
            <xs:attribute name="version" type="xs:token"/>
            <xs:attribute name="finalDefault" type="xs:fullDerivationSet"
                            default="" use="optional"/>
            <xs:attribute name="blockDefault" type="xs:blockSet" default=""
                            use="optional"/>
            <xs:attribute name="attributeFormDefault" type="xs:formChoice"
                            default="unqualified" use="optional"/>
            <xs:attribute name="elementFormDefault" type="xs:formChoice"
                            default="unqualified" use="optional"/>
            <xs:attribute name="defaultAttributes" type="xs:QName"/>
            <xs:attribute name="xpathDefaultNamespace" type="xs:xpathDefaultNamespace"
                            default="##local" use="optional"/>
            <xs:attribute name="id" type="xs:ID"/>
            <xs:attribute ref="xml:lang"/>
        </xs:extension>
        "###
    )]
    #[case(
        r###"
        <xs:extension xmlns:xs="http://www.w3.org/2001/XMLSchema" base="xs:annotated">
            <xs:sequence>
                <xs:element name="any" type="xs:wildcard"/>
            </xs:sequence>
            <xs:attribute name="appliesToEmpty" type="xs:boolean"
                            default="false" use="optional"/>
            <xs:attribute name="mode" default="interleave" use="optional">
                <xs:simpleType>
                <xs:restriction base="xs:NMTOKEN">
                    <xs:enumeration value="interleave"/>
                    <xs:enumeration value="suffix"/>
                </xs:restriction>
                </xs:simpleType>
            </xs:attribute>
        </xs:extension>
        "###
    )]
    #[case(
        r###"
        <xs:extension xmlns:xs="http://www.w3.org/2001/XMLSchema" base="xs:annotated">
            <xs:group ref="xs:complexTypeModel"/>
            <xs:attribute name="name" type="xs:NCName">
            <xs:annotation>
                <xs:documentation>
        Will be restricted to required or prohibited</xs:documentation>
            </xs:annotation>
            </xs:attribute>
            <xs:attribute name="mixed" type="xs:boolean" use="optional">
            <xs:annotation>
                <xs:documentation>
        Not allowed if simpleContent child is chosen.
        May be overridden by setting on complexContent child.</xs:documentation>
            </xs:annotation>
            </xs:attribute>
            <xs:attribute name="abstract" type="xs:boolean" default="false"
                        use="optional"/>
            <xs:attribute name="final" type="xs:derivationSet"/>
            <xs:attribute name="block" type="xs:derivationSet"/>
            <xs:attribute name="defaultAttributesApply" type="xs:boolean"
                        default="true" use="optional"/>
        </xs:extension>
        "###
    )]
    fn deserialize_extension(#[case] xml: &str) {
        let result: ExtensionType = xmlity_quick_xml::from_str(xml).unwrap();

        println!("{result:#?}");
    }

    #[rstest]
    #[case(
        r###"
        <xs:restriction xmlns:xs="http://www.w3.org/2001/XMLSchema" base="xs:anyType">
            <xs:anyAttribute namespace="##other" processContents="lax"/>
        </xs:restriction>
        "###
    )]
    fn deserialize_complex_content_content(#[case] xml: &str) {
        let result: ComplexContentContent = xmlity_quick_xml::from_str(xml).unwrap();

        println!("{result:#?}");
    }

    #[rstest]
    #[case(
        r###"
        <xs:complexContent xmlns:xs="http://www.w3.org/2001/XMLSchema">
            <xs:restriction base="xs:anyType">
                <xs:anyAttribute namespace="##other" processContents="lax"/>
            </xs:restriction>
        </xs:complexContent>
        "###
    )]
    #[case(
        r###"
        <xs:complexContent xmlns:xs="http://www.w3.org/2001/XMLSchema">
            <xs:extension base="xs:annotated">
                <xs:sequence>
                    <xs:element name="any" type="xs:wildcard"/>
                </xs:sequence>
                <xs:attribute name="appliesToEmpty" type="xs:boolean"
                                default="false" use="optional"/>
                <xs:attribute name="mode" default="interleave" use="optional">
                    <xs:simpleType>
                    <xs:restriction base="xs:NMTOKEN">
                        <xs:enumeration value="interleave"/>
                        <xs:enumeration value="suffix"/>
                    </xs:restriction>
                    </xs:simpleType>
                </xs:attribute>
            </xs:extension>
        </xs:complexContent>
        "###
    )]
    fn deserialize_complex_content(#[case] xml: &str) {
        let result: ComplexContent = xmlity_quick_xml::from_str(xml).unwrap();

        println!("{result:#?}");
    }

    #[rstest]
    #[case(r###"
        <xs:anyAttribute xmlns:xs="http://www.w3.org/2001/XMLSchema" namespace="##other" processContents="lax"/>
        "###)]
    fn deserialize_any_attribute(#[case] xml: &str) {
        let result: AnyAttribute = xmlity_quick_xml::from_str(xml).unwrap();

        println!("{result:#?}");
    }

    #[rstest]
    #[case(
        r###"
        <xs:enumeration xmlns:xs="http://www.w3.org/2001/XMLSchema" value="qualified"/>
        "###
    )]
    #[case(
        r###"
        <xs:enumeration xmlns:xs="http://www.w3.org/2001/XMLSchema" value="suffix"/>
        "###
    )]
    fn deserialize_enumeration(#[case] xml: &str) {
        let result: Enumeration = xmlity_quick_xml::from_str(xml).unwrap();

        println!("{result:#?}");
    }

    #[rstest]
    #[case(
        r###"
        <xs:minExclusive xmlns:xs="http://www.w3.org/2001/XMLSchema" value="99"/>
        "###
    )]
    fn deserialize_min_exclusive(#[case] xml: &str) {
        let result: MinExclusive = xmlity_quick_xml::from_str(xml).unwrap();

        println!("{result:#?}");
    }

    #[rstest]
    #[case(
        r###"
        <xs:enumeration xmlns:xs="http://www.w3.org/2001/XMLSchema" value="qualified"/>
        "###
    )]
    #[case(
        r###"
        <xs:enumeration xmlns:xs="http://www.w3.org/2001/XMLSchema" value="suffix"/>
        "###
    )]
    fn deserialize_facet(#[case] xml: &str) {
        let result: Facet = xmlity_quick_xml::from_str(xml).unwrap();

        println!("{result:#?}");
    }

    #[rstest]
    #[case(
        r###"
        <xs:restriction xmlns:xs="http://www.w3.org/2001/XMLSchema" base="xs:anyType">
            <xs:anyAttribute namespace="##other" processContents="lax"/>
        </xs:restriction>
        "###
    )]
    #[case(
        r###"
        <xs:restriction xmlns:xs="http://www.w3.org/2001/XMLSchema" base="xs:NMTOKEN">
            <xs:enumeration value="qualified"/>
            <xs:enumeration value="unqualified"/>
        </xs:restriction>
        "###
    )]
    #[case(
        r###"
        <xs:restriction xmlns:xs="http://www.w3.org/2001/XMLSchema" base="xs:NMTOKEN">
            <xs:enumeration value="interleave"/>
            <xs:enumeration value="suffix"/>
        </xs:restriction>
        "###
    )]
    #[case(
        r###"
        <xs:restriction xmlns:xs="http://www.w3.org/2001/XMLSchema" base="xs:token"/>
        "###
    )]
    fn deserialize_restriction(#[case] xml: &str) {
        let result: ComplexRestrictionType = xmlity_quick_xml::from_str(xml).unwrap();

        println!("{result:#?}");
    }

    #[rstest]
    #[case(
        r###"
        <xs:choice xmlns:xs="http://www.w3.org/2001/XMLSchema">
            <xs:element ref="xs:include"/>
            <xs:element ref="xs:import"/>
            <xs:element ref="xs:redefine"/>
            <xs:element ref="xs:override"/>
            <xs:element ref="xs:annotation"/>
        </xs:choice>
        "###
    )]
    fn deserialize_choice(#[case] xml: &str) {
        let result: ChoiceType = xmlity_quick_xml::from_str(xml).unwrap();

        println!("{result:#?}");
    }

    #[rstest]
    #[case(
        r###"
        <xs:group xmlns:xs="http://www.w3.org/2001/XMLSchema" name="composition">
            <xs:choice>
                <xs:element ref="xs:include"/>
                <xs:element ref="xs:import"/>
                <xs:element ref="xs:redefine"/>
                <xs:element ref="xs:override"/>
                <xs:element ref="xs:annotation"/>
            </xs:choice>
        </xs:group>
        "###
    )]
    #[case(r###"
        <xs:group xmlns:xs="http://www.w3.org/2001/XMLSchema" ref="xs:composition" minOccurs="0" maxOccurs="unbounded"/>
        "###)]
    fn deserialize_group(#[case] xml: &str) {
        let result: GroupType = xmlity_quick_xml::from_str(xml).unwrap();

        println!("{result:#?}");
    }

    #[rstest]
    #[case(
        r###"
        <xs:restriction xmlns:xs="http://www.w3.org/2001/XMLSchema" base="xs:NMTOKEN">
            <xs:enumeration value="qualified"/>
            <xs:enumeration value="unqualified"/>
        </xs:restriction>
        "###
    )]
    #[case(
        r###"
        <xs:restriction xmlns:xs="http://www.w3.org/2001/XMLSchema" base="xs:NMTOKEN">
            <xs:enumeration value="interleave"/>
            <xs:enumeration value="suffix"/>
        </xs:restriction>
        "###
    )]
    #[case(
        r###"
        <xs:restriction xmlns:xs="http://www.w3.org/2001/XMLSchema" base="xs:token"/>
        "###
    )]
    fn deserialize_simple_derivation(#[case] xml: &str) {
        let result: SimpleDerivation = xmlity_quick_xml::from_str(xml).unwrap();

        println!("{result:#?}");
    }

    #[rstest]
    #[case(
        r###"
        <xs:union xmlns:xs="http://www.w3.org/2001/XMLSchema">
            <xs:simpleType>
                <xs:restriction base="xs:token">
                <xs:enumeration value="#all"/>
                </xs:restriction>
            </xs:simpleType>
            <xs:simpleType>
                <xs:list itemType="xs:reducedDerivationControl"/>
            </xs:simpleType>
        </xs:union>
        "###
    )]
    #[case(
        r###"
        <xs:union xmlns:xs="http://www.w3.org/2001/XMLSchema" memberTypes="xs:anyURI">
            <xs:simpleType>
                <xs:restriction base="xs:token">
                <xs:enumeration value="##targetNamespace"/>
                <xs:enumeration value="##local"/>
                </xs:restriction>
            </xs:simpleType>
        </xs:union>
        "###
    )]
    fn deserialize_union(#[case] xml: &str) {
        let result: Union = xmlity_quick_xml::from_str(xml).unwrap();

        println!("{result:#?}");
    }

    #[rstest]
    #[case(
        r###"
        <xs:list xmlns:xs="http://www.w3.org/2001/XMLSchema">
            <xs:simpleType>
                <xs:union memberTypes="xs:anyURI">
                    <xs:simpleType>
                        <xs:restriction base="xs:token">
                        <xs:enumeration value="##targetNamespace"/>
                        <xs:enumeration value="##local"/>
                        </xs:restriction>
                    </xs:simpleType>
                </xs:union>
            </xs:simpleType>
        </xs:list>
        "###
    )]
    fn deserialize_list(#[case] xml: &str) {
        let result: List = xmlity_quick_xml::from_str(xml).unwrap();

        println!("{result:#?}");
    }

    #[rstest]
    #[case(
        r###"
        <xs:simpleType xmlns:xs="http://www.w3.org/2001/XMLSchema" name="formChoice">
            <xs:annotation>
                <xs:documentation>
                A utility type, not for public use</xs:documentation>
                </xs:annotation>
            <xs:restriction base="xs:NMTOKEN">
                <xs:enumeration value="qualified"/>
                <xs:enumeration value="unqualified"/>
            </xs:restriction>
        </xs:simpleType>
        "###
    )]
    #[case(
        r###"
        <xs:simpleType xmlns:xs="http://www.w3.org/2001/XMLSchema">
            <xs:restriction base="xs:NMTOKEN">
                <xs:enumeration value="interleave"/>
                <xs:enumeration value="suffix"/>
            </xs:restriction>
        </xs:simpleType>
        "###
    )]
    #[case(
        r###"
        <xs:simpleType xmlns:xs="http://www.w3.org/2001/XMLSchema" name="derivationSet">
            <xs:annotation>
                <xs:documentation>
                A utility type, not for public use</xs:documentation>
                <xs:documentation>
                #all or (possibly empty) subset of {extension, restriction}</xs:documentation>
            </xs:annotation>
            <xs:union>
                <xs:simpleType>
                    <xs:restriction base="xs:token">
                    <xs:enumeration value="#all"/>
                    </xs:restriction>
                </xs:simpleType>
                <xs:simpleType>
                    <xs:list itemType="xs:reducedDerivationControl"/>
                </xs:simpleType>
            </xs:union>
        </xs:simpleType>
        "###
    )]
    #[case(
        r###"
        <xs:simpleType xmlns:xs="http://www.w3.org/2001/XMLSchema" name="basicNamespaceList">
            <xs:annotation>
                <xs:documentation>
            A utility type, not for public use</xs:documentation>
            </xs:annotation>
            <xs:list>
                <xs:simpleType>
                    <xs:union memberTypes="xs:anyURI">
                        <xs:simpleType>
                            <xs:restriction base="xs:token">
                            <xs:enumeration value="##targetNamespace"/>
                            <xs:enumeration value="##local"/>
                            </xs:restriction>
                        </xs:simpleType>
                    </xs:union>
                </xs:simpleType>
            </xs:list>
        </xs:simpleType>
        "###
    )]
    #[case(
        r###"
        <xs:simpleType xmlns:xs="http://www.w3.org/2001/XMLSchema" name="basicNamespaceList">
            <xs:annotation>
                <xs:documentation>
            A utility type, not for public use</xs:documentation>
            </xs:annotation>
            <xs:list>
                <xs:simpleType>
                    <xs:union memberTypes="xs:anyURI">
                        <xs:simpleType>
                            <xs:restriction base="xs:token">
                            <xs:enumeration value="##targetNamespace"/>
                            <xs:enumeration value="##local"/>
                            </xs:restriction>
                        </xs:simpleType>
                    </xs:union>
                </xs:simpleType>
            </xs:list>
        </xs:simpleType>
        "###
    )]
    #[case(
        r###"
        <xs:simpleType xmlns:xs="http://www.w3.org/2001/XMLSchema" name="public">
        <xs:annotation>
            <xs:documentation>
        A utility type, not for public use</xs:documentation>
            <xs:documentation>
        A public identifier, per ISO 8879</xs:documentation>
            </xs:annotation>
        <xs:restriction base="xs:token"/>
    </xs:simpleType>
        "###
    )]
    fn deserialize_simple_type(#[case] xml: &str) {
        let result: LocalSimpleType = xmlity_quick_xml::from_str(xml).unwrap();

        println!("{result:#?}");
    }

    #[rstest]
    #[case(
        r###"
    <xs:sequence xmlns:xs="http://www.w3.org/2001/XMLSchema">
        <xs:group ref="xs:composition" minOccurs="0" maxOccurs="unbounded"/>
        <xs:sequence minOccurs="0">
            <xs:element ref="xs:defaultOpenContent"/>
            <xs:element ref="xs:annotation" minOccurs="0"
                        maxOccurs="unbounded"/>
        </xs:sequence>
        <xs:sequence minOccurs="0" maxOccurs="unbounded">
            <xs:group ref="xs:schemaTop"/>
            <xs:element ref="xs:annotation" minOccurs="0"
                        maxOccurs="unbounded"/>
        </xs:sequence>
    </xs:sequence>
    "###
    )]
    #[case(
        r###"
    <xs:sequence xmlns:xs="http://www.w3.org/2001/XMLSchema" minOccurs="0">
        <xs:element ref="xs:defaultOpenContent"/>
        <xs:element ref="xs:annotation" minOccurs="0"
                    maxOccurs="unbounded"/>
    </xs:sequence>
    "###
    )]
    #[case(
        r###"
    <xs:sequence xmlns:xs="http://www.w3.org/2001/XMLSchema" minOccurs="0" maxOccurs="unbounded">
        <xs:group ref="xs:schemaTop"/>
        <xs:element ref="xs:annotation" minOccurs="0"
                    maxOccurs="unbounded"/>
    </xs:sequence>
    "###
    )]
    #[case(
        r###"
        <xs:sequence xmlns:xs="http://www.w3.org/2001/XMLSchema">
            <xs:element name="any" type="xs:wildcard"/>
        </xs:sequence>
        "###
    )]
    fn deserialize_sequence(#[case] xml: &str) {
        let result: SequenceType = xmlity_quick_xml::from_str(xml).unwrap();

        println!("{result:#?}");
    }

    #[rstest]
    #[case(
        r###"
        <xs:selector xmlns:xs="http://www.w3.org/2001/XMLSchema" xpath="xs:element"/>
        "###
    )]
    fn deserialize_selector(#[case] xml: &str) {
        let result: Selector = xmlity_quick_xml::from_str(xml).unwrap();

        println!("{result:#?}");
    }

    #[rstest]
    #[case(
        r###"
        <xs:field xmlns:xs="http://www.w3.org/2001/XMLSchema" xpath="@name"/>
        "###
    )]
    fn deserialize_field(#[case] xml: &str) {
        let result: Field = xmlity_quick_xml::from_str(xml).unwrap();

        println!("{result:#?}");
    }

    #[rstest]
    #[case(
        r###"
        <xs:key xmlns:xs="http://www.w3.org/2001/XMLSchema" name="element">
            <xs:selector xpath="xs:element"/>
            <xs:field xpath="@name"/>
        </xs:key>
        "###
    )]
    fn deserialize_key(#[case] xml: &str) {
        let result: Key = xmlity_quick_xml::from_str(xml).unwrap();

        println!("{result:#?}");
    }

    #[rstest]
    #[case(
        r###"
        <xs:key xmlns:xs="http://www.w3.org/2001/XMLSchema" name="element">
            <xs:selector xpath="xs:element"/>
            <xs:field xpath="@name"/>
        </xs:key>
        "###
    )]
    fn deserialize_identity_constraint(#[case] xml: &str) {
        let result: IdentityConstraint = xmlity_quick_xml::from_str(xml).unwrap();

        println!("{result:#?}");
    }

    #[rstest]
    #[case(r###"<xs:element xmlns:xs="http://www.w3.org/2001/XMLSchema" ref="xs:defaultOpenContent"/>"###)]
    #[case(
        r###"<xs:element xmlns:xs="http://www.w3.org/2001/XMLSchema" ref="xs:annotation" minOccurs="0"
                maxOccurs="unbounded"/>"###
    )]
    fn deserialize_local_element(#[case] xml: &str) {
        let result: LocalElement = xmlity_quick_xml::from_str(xml).unwrap();

        println!("{result:#?}");
    }

    #[rstest]
    #[case(r###"<xs:element xmlns:xs="http://www.w3.org/2001/XMLSchema" ref="xs:include"/>"###)]
    #[case(
        r###"
        <xs:element xmlns:xs="http://www.w3.org/2001/XMLSchema" name="schema" id="schema">
            <xs:annotation>
                <xs:documentation
                    source="../structures/structures.html#element-schema"/>
            </xs:annotation>
            <xs:complexType>
                <xs:complexContent>
                    <xs:extension base="xs:openAttrs">
                        <xs:sequence>
                            <xs:group ref="xs:composition" minOccurs="0" maxOccurs="unbounded"/>
                            <xs:sequence minOccurs="0">
                            <xs:element ref="xs:defaultOpenContent"/>
                            <xs:element ref="xs:annotation" minOccurs="0"
                                        maxOccurs="unbounded"/>
                            </xs:sequence>
                            <xs:sequence minOccurs="0" maxOccurs="unbounded">
                            <xs:group ref="xs:schemaTop"/>
                            <xs:element ref="xs:annotation" minOccurs="0"
                                        maxOccurs="unbounded"/>
                        </xs:sequence>
                        </xs:sequence>
                        <xs:attribute name="targetNamespace" type="xs:anyURI"/>
                        <xs:attribute name="version" type="xs:token"/>
                        <xs:attribute name="finalDefault" type="xs:fullDerivationSet"
                                        default="" use="optional"/>
                        <xs:attribute name="blockDefault" type="xs:blockSet" default=""
                                        use="optional"/>
                        <xs:attribute name="attributeFormDefault" type="xs:formChoice"
                                        default="unqualified" use="optional"/>
                        <xs:attribute name="elementFormDefault" type="xs:formChoice"
                                        default="unqualified" use="optional"/>
                        <xs:attribute name="defaultAttributes" type="xs:QName"/>
                        <xs:attribute name="xpathDefaultNamespace" type="xs:xpathDefaultNamespace"
                                        default="##local" use="optional"/>
                        <xs:attribute name="id" type="xs:ID"/>
                        <xs:attribute ref="xml:lang"/>
                    </xs:extension>
                </xs:complexContent>
            </xs:complexType>
            <xs:key name="element">
                <xs:selector xpath="xs:element"/>
                <xs:field xpath="@name"/>
            </xs:key>
            <xs:key name="attribute">
                <xs:selector xpath="xs:attribute"/>
                <xs:field xpath="@name"/>
            </xs:key>
            <xs:key name="type">
                <xs:selector xpath="xs:complexType|xs:simpleType"/>
                <xs:field xpath="@name"/>
            </xs:key>
            <xs:key name="group">
                <xs:selector xpath="xs:group"/>
                <xs:field xpath="@name"/>
            </xs:key>
            <xs:key name="attributeGroup">
                <xs:selector xpath="xs:attributeGroup"/>
                <xs:field xpath="@name"/>
            </xs:key>
            <xs:key name="notation">
                <xs:selector xpath="xs:notation"/>
                <xs:field xpath="@name"/>
            </xs:key>
            <xs:key name="identityConstraint">
                <xs:selector xpath=".//xs:key|.//xs:unique|.//xs:keyref"/>
                <xs:field xpath="@name"/>
            </xs:key>
        </xs:element>
        "###
    )]
    #[case(
        r###"
        <xs:element xmlns:xs="http://www.w3.org/2001/XMLSchema" name="defaultOpenContent" id="defaultOpenContent">
            <xs:annotation>
                <xs:documentation
                    source="../structures/structures.html#element-defaultOpenContent"/>
            </xs:annotation>
            <xs:complexType>
            <xs:complexContent>
                <xs:extension base="xs:annotated">
                <xs:sequence>
                    <xs:element name="any" type="xs:wildcard"/>
                </xs:sequence>
                <xs:attribute name="appliesToEmpty" type="xs:boolean"
                                default="false" use="optional"/>
                <xs:attribute name="mode" default="interleave" use="optional">
                    <xs:simpleType>
                    <xs:restriction base="xs:NMTOKEN">
                        <xs:enumeration value="interleave"/>
                        <xs:enumeration value="suffix"/>
                    </xs:restriction>
                    </xs:simpleType>
                </xs:attribute>

                </xs:extension>
            </xs:complexContent>
            </xs:complexType>
        </xs:element>
    "###
    )]
    #[case(
        r###"
        <xs:element xmlns:xs="http://www.w3.org/2001/XMLSchema" name="override" id="override">
            <xs:annotation>
            <xs:documentation
                source="../structures/structures.html#element-override"/>
            </xs:annotation>
            <xs:complexType>
            <xs:complexContent>
                <xs:extension base="xs:openAttrs">
                <xs:sequence>
                    <xs:element ref="xs:annotation" minOccurs="0"/>
                    <xs:group ref="xs:schemaTop" minOccurs="0" maxOccurs="unbounded"/>
                </xs:sequence>
                <xs:attribute name="schemaLocation" type="xs:anyURI" use="required"/>
                <xs:attribute name="id" type="xs:ID"/>
                </xs:extension>
              </xs:complexContent>
            </xs:complexType>
        </xs:element>
    "###
    )]
    #[case(
        r###"
        <xs:element xmlns:xs="http://www.w3.org/2001/XMLSchema" name="facet" abstract="true">
          <xs:annotation>
            <xs:documentation>
              An abstract element, representing facets in general.
              The facets defined by this spec are substitutable for
              this element, and implementation-defined facets should
              also name this as a substitution-group head.
            </xs:documentation>
          </xs:annotation>
        </xs:element>
      "###
    )]

    fn deserialize_top_level_element(#[case] xml: &str) {
        let result: TopLevelElement = xmlity_quick_xml::from_str(xml).unwrap();

        println!("{result:#?}");
    }

    #[rstest]
    #[case(
        r###"<xs:attribute xmlns:xs="http://www.w3.org/2001/XMLSchema" name="minOccurs" type="xs:nonNegativeInteger" default="1"
                        use="optional"/>"###
    )]
    #[case(r###"<xs:attribute xmlns:xs="http://www.w3.org/2001/XMLSchema" name="maxOccurs" type="xs:allNNI" default="1" use="optional"/>"###)]
    #[case(r###"
    <xs:attribute xmlns:xs="http://www.w3.org/2001/XMLSchema" name="mode" default="interleave" use="optional">
        <xs:simpleType>
            <xs:restriction base="xs:NMTOKEN">
                <xs:enumeration value="interleave"/>
                <xs:enumeration value="suffix"/>
            </xs:restriction>
        </xs:simpleType>
    </xs:attribute>
    "###)]
    fn deserialize_attribute(#[case] xml: &str) {
        let result: LocalAttribute = xmlity_quick_xml::from_str(xml).unwrap();

        println!("{result:#?}");
    }

    #[rstest]
    #[case(
        r###"
        <xs:attributeGroup xmlns:xs="http://www.w3.org/2001/XMLSchema" name="occurs">
            <xs:annotation>
            <xs:documentation>
            for all particles</xs:documentation>
            </xs:annotation>
            <xs:attribute name="minOccurs" type="xs:nonNegativeInteger" default="1"
                        use="optional"/>
            <xs:attribute name="maxOccurs" type="xs:allNNI" default="1" use="optional"/>
        </xs:attributeGroup>
        "###
    )]
    #[case(
        r###"
        <xs:attributeGroup xmlns:xs="http://www.w3.org/2001/XMLSchema" ref="xs:defRef"/>
        "###
    )]
    fn deserialize_attribute_group(#[case] xml: &str) {
        let result: AttributeGroupType = xmlity_quick_xml::from_str(xml).unwrap();

        println!("{result:#?}");
    }

    #[rstest]
    #[case(
        r###"<xs:notation xmlns:xs="http://www.w3.org/2001/XMLSchema" name="XMLSchemaStructures" public="structures"
               system="http://www.w3.org/2000/08/XMLSchema.xsd"/>"###
    )]
    #[case(
        r###"<xs:notation xmlns:xs="http://www.w3.org/2001/XMLSchema" name="XML" public="REC-xml-19980210"
               system="http://www.w3.org/TR/1998/REC-xml-19980210"/>"###
    )]
    fn deserialize_notation(#[case] xml: &str) {
        let result: Notation = xmlity_quick_xml::from_str(xml).unwrap();

        println!("{result:#?}");
    }

    #[rstest]
    #[case(
        r###"<xs:schema xmlns:xs="http://www.w3.org/2001/XMLSchema"
        elementFormDefault="qualified" xml:lang="EN"
        targetNamespace="http://www.w3.org/2001/XMLSchema"
        version="1.0" />"###
    )]
    #[case(
        r###"<xs:schema xmlns:xs="http://www.w3.org/2001/XMLSchema"
        elementFormDefault="qualified" xml:lang="EN"
        targetNamespace="http://www.w3.org/2001/XMLSchema"
        version="1.0">
            <xs:annotation>
            <xs:documentation>
                Part 1 version: structures.xsd (rec-20120405)
                Part 2 version: datatypes.xsd (rec-20120405)
            </xs:documentation>
            </xs:annotation>

            <xs:annotation>
                <xs:documentation  source="../structures/structures.html">
            The schema corresponding to this document is normative,
            with respect to the syntactic constraints it expresses in the
            XML Schema Definition Language.  The documentation (within 'documentation' elements)
            below, is not normative, but rather highlights important aspects of
            the W3C Recommendation of which this is a part.

                See below (at the bottom of this document) for information about
                the revision and namespace-versioning policy governing this
                schema document.

                </xs:documentation>
            </xs:annotation>
            <xs:annotation>
                <xs:documentation>
            The simpleType element and all of its members are defined
            towards the end of this schema document.</xs:documentation>
            </xs:annotation>
            <xs:import namespace="http://www.w3.org/XML/1998/namespace"
                        schemaLocation="xml.xsd">
                <xs:annotation>
                <xs:documentation>
                Get access to the xml: attribute groups for xml:lang
                as declared on 'schema' and 'documentation' below
                </xs:documentation>
                </xs:annotation>
            </xs:import>
            <xs:complexType name="openAttrs">
                <xs:annotation>
                <xs:documentation>
                This type is extended by almost all schema types
                to allow attributes from other namespaces to be
                added to user schemas.
                </xs:documentation>
                </xs:annotation>
                <xs:complexContent>
                <xs:restriction base="xs:anyType">
                    <xs:anyAttribute namespace="##other" processContents="lax"/>
                </xs:restriction>
                </xs:complexContent>
            </xs:complexType>
        </xs:schema>
        "###
    )]
    #[case(
        r###"<xs:schema xmlns:xs="http://www.w3.org/2001/XMLSchema"
        elementFormDefault="qualified" xml:lang="EN"
        targetNamespace="http://www.w3.org/2001/XMLSchema"
        version="1.0">
            <xs:annotation>
            <xs:documentation>
                Part 1 version: structures.xsd (rec-20120405)
                Part 2 version: datatypes.xsd (rec-20120405)
            </xs:documentation>
            </xs:annotation>

            <xs:annotation>
                <xs:documentation  source="../structures/structures.html">
            The schema corresponding to this document is normative,
            with respect to the syntactic constraints it expresses in the
            XML Schema Definition Language.  The documentation (within 'documentation' elements)
            below, is not normative, but rather highlights important aspects of
            the W3C Recommendation of which this is a part.

                See below (at the bottom of this document) for information about
                the revision and namespace-versioning policy governing this
                schema document.

                </xs:documentation>
            </xs:annotation>
            <xs:annotation>
                <xs:documentation>
            The simpleType element and all of its members are defined
            towards the end of this schema document.</xs:documentation>
            </xs:annotation>
            <xs:import namespace="http://www.w3.org/XML/1998/namespace"
                        schemaLocation="xml.xsd">
                <xs:annotation>
                <xs:documentation>
                Get access to the xml: attribute groups for xml:lang
                as declared on 'schema' and 'documentation' below
                </xs:documentation>
                </xs:annotation>
            </xs:import>
            <xs:complexType name="openAttrs">
                <xs:annotation>
                <xs:documentation>
                This type is extended by almost all schema types
                to allow attributes from other namespaces to be
                added to user schemas.
                </xs:documentation>
                </xs:annotation>
                <xs:complexContent>
                <xs:restriction base="xs:anyType">
                    <xs:anyAttribute namespace="##other" processContents="lax"/>
                </xs:restriction>
                </xs:complexContent>xmlns
                <xs:complexContent>
                <xs:extension base="xs:openAttrs">
                    <xs:sequence>
                    <xs:element ref="xs:annotation" minOccurs="0"/>
                    </xs:sequence>
                    <xs:attribute name="id" type="xs:ID"/>
                </xs:extension>
                </xs:complexContent>
            </xs:complexType>
            <xs:group name="composition">
                <xs:choice>
                <xs:element ref="xs:include"/>
                <xs:element ref="xs:import"/>
                <xs:element ref="xs:redefine"/>
                <xs:element ref="xs:override"/>
                <xs:element ref="xs:annotation"/>
                </xs:choice>
            </xs:group>
            <xs:group name="schemaTop">
                <xs:annotation>
                <xs:documentation>
            This group is for the
            elements which occur freely at the top level of schemas.
            All of their types are based on the "annotated" type by extension.</xs:documentation>
                </xs:annotation>
                <xs:choice>
                <xs:group ref="xs:redefinable"/>
                <xs:element ref="xs:element"/>
                <xs:element ref="xs:attribute"/>
                <xs:element ref="xs:notation"/>
                </xs:choice>
            </xs:group>
            <xs:group name="redefinable">
                <xs:annotation>
                <xs:documentation>
            This group is for the
            elements which can self-redefine (see &lt;redefine> below).</xs:documentation>
                </xs:annotation>
                <xs:choice>
                <xs:element ref="xs:simpleType"/>
                <xs:element ref="xs:complexType"/>
                <xs:element ref="xs:group"/>
                <xs:element ref="xs:attributeGroup"/>
                </xs:choice>
            </xs:group>
            <xs:simpleType name="formChoice">
                <xs:annotation>
                <xs:documentation>
            A utility type, not for public use</xs:documentation>
                </xs:annotation>
                <xs:restriction base="xs:NMTOKEN">
                <xs:enumeration value="qualified"/>
                <xs:enumeration value="unqualified"/>
                </xs:restriction>
            </xs:simpleType>
            <xs:simpleType name="reducedDerivationControl">
                <xs:annotation>
                <xs:documentation>
            A utility type, not for public use</xs:documentation>
                </xs:annotation>
                <xs:restriction base="xs:derivationControl">
                <xs:enumeration value="extension"/>
                <xs:enumeration value="restriction"/>
                </xs:restriction>
            </xs:simpleType>
            <xs:simpleType name="derivationSet">
                <xs:annotation>
                <xs:documentation>
            A utility type, not for public use</xs:documentation>
                <xs:documentation>
            #all or (possibly empty) subset of {extension, restriction}</xs:documentation>
                </xs:annotation>
                <xs:union>
                <xs:simpleType>
                    <xs:restriction base="xs:token">
                    <xs:enumeration value="#all"/>
                    </xs:restriction>
                </xs:simpleType>
                <xs:simpleType>
                    <xs:list itemType="xs:reducedDerivationControl"/>
                </xs:simpleType>
                </xs:union>
            </xs:simpleType>
            <xs:simpleType name="typeDerivationControl">
                <xs:annotation>
                <xs:documentation>
            A utility type, not for public use</xs:documentation>
                </xs:annotation>
                <xs:restriction base="xs:derivationControl">
                <xs:enumeration value="extension"/>
                <xs:enumeration value="restriction"/>
                <xs:enumeration value="list"/>
                <xs:enumeration value="union"/>
                </xs:restriction>
            </xs:simpleType>
            <xs:simpleType name="fullDerivationSet">
                <xs:annotation>
                <xs:documentation>
            A utility type, not for public use</xs:documentation>
                <xs:documentation>
            #all or (possibly empty) subset of {extension, restriction, list, union}</xs:documentation>
                </xs:annotation>
                <xs:union>
                <xs:simpleType>
                    <xs:restriction base="xs:token">
                    <xs:enumeration value="#all"/>
                    </xs:restriction>
                </xs:simpleType>
                <xs:simpleType>
                    <xs:list itemType="xs:typeDerivationControl"/>
                </xs:simpleType>
                </xs:union>
            </xs:simpleType>
            <xs:element name="schema" id="schema">
                <xs:annotation>
                <xs:documentation
                    source="../structures/structures.html#element-schema"/>
                </xs:annotation>
                <xs:complexType>
                <xs:complexContent>
                    <xs:extension base="xs:openAttrs">
                    <xs:sequence>
                        <xs:group ref="xs:composition" minOccurs="0" maxOccurs="unbounded"/>
                        <xs:sequence minOccurs="0">
                        <xs:element ref="xs:defaultOpenContent"/>
                        <xs:element ref="xs:annotation" minOccurs="0"
                                    maxOccurs="unbounded"/>
                        </xs:sequence>
                        <xs:sequence minOccurs="0" maxOccurs="unbounded">
                        <xs:group ref="xs:schemaTop"/>
                        <xs:element ref="xs:annotation" minOccurs="0"
                                    maxOccurs="unbounded"/>
                        </xs:sequence>
                    </xs:sequence>
                    <xs:attribute name="targetNamespace" type="xs:anyURI"/>
                    <xs:attribute name="version" type="xs:token"/>
                    <xs:attribute name="finalDefault" type="xs:fullDerivationSet"
                                    default="" use="optional"/>
                    <xs:attribute name="blockDefault" type="xs:blockSet" default=""
                                    use="optional"/>
                    <xs:attribute name="attributeFormDefault" type="xs:formChoice"
                                    default="unqualified" use="optional"/>
                    <xs:attribute name="elementFormDefault" type="xs:formChoice"
                                    default="unqualified" use="optional"/>
                    <xs:attribute name="defaultAttributes" type="xs:QName"/>
                    <xs:attribute name="xpathDefaultNamespace" type="xs:xpathDefaultNamespace"
                                    default="##local" use="optional"/>
                    <xs:attribute name="id" type="xs:ID"/>
                    <xs:attribute ref="xml:lang"/>
                    </xs:extension>
                </xs:complexContent>
                </xs:complexType>
                <xs:key name="element">
                <xs:selector xpath="xs:element"/>
                <xs:field xpath="@name"/>
                </xs:key>
                <xs:key name="attribute">
                <xs:selector xpath="xs:attribute"/>
                <xs:field xpath="@name"/>
                </xs:key>
                <xs:key name="type">
                <xs:selector xpath="xs:complexType|xs:simpleType"/>
                <xs:field xpath="@name"/>
                </xs:key>
                <xs:key name="group">
                <xs:selector xpath="xs:group"/>
                <xs:field xpath="@name"/>
                </xs:key>
                <xs:key name="attributeGroup">
                <xs:selector xpath="xs:attributeGroup"/>
                <xs:field xpath="@name"/>
                </xs:key>
                <xs:key name="notation">
                <xs:selector xpath="xs:notation"/>
                <xs:field xpath="@name"/>
                </xs:key>
                <xs:key name="identityConstraint">
                <xs:selector xpath=".//xs:key|.//xs:unique|.//xs:keyref"/>
                <xs:field xpath="@name"/>
                </xs:key>
            </xs:element>
            <xs:simpleType name="allNNI">
                <xs:annotation>
                <xs:documentation>
            for maxOccurs</xs:documentation>
                </xs:annotation>
                <xs:union memberTypes="xs:nonNegativeInteger">
                <xs:simpleType>
                    <xs:restriction base="xs:NMTOKEN">
                    <xs:enumeration value="unbounded"/>
                    </xs:restriction>
                </xs:simpleType>
                </xs:union>
            </xs:simpleType>
            <xs:attributeGroup name="occurs">
                <xs:annotation>
                <xs:documentation>
            for all particles</xs:documentation>
                </xs:annotation>
                <xs:attribute name="minOccurs" type="xs:nonNegativeInteger" default="1"
                            use="optional"/>
                <xs:attribute name="maxOccurs" type="xs:allNNI" default="1" use="optional"/>
            </xs:attributeGroup>
            <xs:attributeGroup name="defRef">
                <xs:annotation>
                <xs:documentation>
            for element, group and attributeGroup,
            which both define and reference</xs:documentation>
                </xs:annotation>
                <xs:attribute name="name" type="xs:NCName"/>
                <xs:attribute name="ref" type="xs:QName"/>
            </xs:attributeGroup>
            <xs:group name="typeDefParticle">
                <xs:annotation>
                <xs:documentation>
            'complexType' uses this</xs:documentation>
                </xs:annotation>
                <xs:choice>
                <xs:element name="group" type="xs:groupRef"/>
                <xs:element ref="xs:all"/>
                <xs:element ref="xs:choice"/>
                <xs:element ref="xs:sequence"/>
                </xs:choice>
            </xs:group>
            <xs:group name="nestedParticle">
                <xs:choice>
                <xs:element name="element" type="xs:localElement"/>
                <xs:element name="group" type="xs:groupRef"/>

                <xs:element ref="xs:choice"/>
                <xs:element ref="xs:sequence"/>
                <xs:element ref="xs:any"/>
                </xs:choice>
            </xs:group>
            <xs:group name="particle">
                <xs:choice>
                <xs:element name="element" type="xs:localElement"/>
                <xs:element name="group" type="xs:groupRef"/>
                <xs:element ref="xs:all"/>
                <xs:element ref="xs:choice"/>
                <xs:element ref="xs:sequence"/>
                <xs:element ref="xs:any"/>
                </xs:choice>
            </xs:group>
            <xs:complexType name="attribute">
                <xs:complexContent>
                <xs:extension base="xs:annotated">
                    <xs:sequence>
                    <xs:element name="simpleType" type="xs:localSimpleType" minOccurs="0"/>
                    </xs:sequence>
                    <xs:attributeGroup ref="xs:defRef"/>
                    <xs:attribute name="type" type="xs:QName"/>
                    <xs:attribute name="use" default="optional" use="optional">
                    <xs:simpleType>
                        <xs:restriction base="xs:NMTOKEN">
                        <xs:enumeration value="prohibited"/>
                        <xs:enumeration value="optional"/>
                        <xs:enumeration value="required"/>
                        </xs:restriction>
                    </xs:simpleType>
                    </xs:attribute>
                    <xs:attribute name="default" type="xs:string"/>
                    <xs:attribute name="fixed" type="xs:string"/>
                    <xs:attribute name="form" type="xs:formChoice"/>
                    <xs:attribute name="targetNamespace" type="xs:anyURI"/>

                    <xs:attribute name="inheritable" type="xs:boolean"/>
                </xs:extension>
                </xs:complexContent>
            </xs:complexType>
            <xs:complexType name="topLevelAttribute">
                <xs:complexContent>
                <xs:restriction base="xs:attribute">
                    <xs:sequence>
                    <xs:element ref="xs:annotation" minOccurs="0"/>
                    <xs:element name="simpleType" type="xs:localSimpleType" minOccurs="0"/>
                    </xs:sequence>
                    <xs:attribute name="ref" use="prohibited"/>
                    <xs:attribute name="form" use="prohibited"/>
                    <xs:attribute name="use" use="prohibited"/>
                    <xs:attribute name="targetNamespace" use="prohibited"/>
                    <xs:attribute name="name" type="xs:NCName" use="required"/>
                    <xs:attribute name="inheritable" type="xs:boolean"/>
                    <xs:anyAttribute namespace="##other" processContents="lax"/>
                </xs:restriction>
                </xs:complexContent>
            </xs:complexType>
            <xs:group name="attrDecls">
                <xs:sequence>
                <xs:choice minOccurs="0" maxOccurs="unbounded">
                    <xs:element name="attribute" type="xs:attribute"/>
                    <xs:element name="attributeGroup" type="xs:attributeGroupRef"/>
                </xs:choice>
                <xs:element ref="xs:anyAttribute" minOccurs="0"/>
                </xs:sequence>
            </xs:group>
            <xs:element name="anyAttribute"  id="anyAttribute">
                <xs:annotation>
                <xs:documentation
                    source="../structures/structures.html#element-anyAttribute"/>
                </xs:annotation>
                <xs:complexType>
                <xs:complexContent>
                    <xs:extension base="xs:wildcard">
                    <xs:attribute name="notQName" type="xs:qnameListA"
                                    use="optional"/>
                    </xs:extension>
                </xs:complexContent>
                </xs:complexType>
            </xs:element>
            <xs:group name="assertions">
                <xs:sequence>
                <xs:element name="assert" type="xs:assertion"
                            minOccurs="0" maxOccurs="unbounded"/>
                </xs:sequence>
            </xs:group>
            <xs:complexType name="assertion">
                <xs:complexContent>
                <xs:extension base="xs:annotated">
                    <xs:attribute name="test" type="xs:string"/>
                    <xs:attribute name="xpathDefaultNamespace" type="xs:xpathDefaultNamespace"/>
                </xs:extension>
                </xs:complexContent>
            </xs:complexType>
            <xs:group name="complexTypeModel">
                <xs:choice>
                <xs:element ref="xs:simpleContent"/>
                <xs:element ref="xs:complexContent"/>
                <xs:sequence>
                    <xs:annotation>
                    <xs:documentation>
            This branch is short for
            &lt;complexContent>
            &lt;restriction base="xs:anyType">
            ...
            &lt;/restriction>
            &lt;/complexContent></xs:documentation>
                    </xs:annotation>
                    <xs:element ref="xs:openContent" minOccurs="0"/>
                    <xs:group ref="xs:typeDefParticle" minOccurs="0"/>
                    <xs:group ref="xs:attrDecls"/>
                    <xs:group ref="xs:assertions"/>
                </xs:sequence>
                </xs:choice>
            </xs:group>
            <xs:complexType name="complexBaseType" abstract="true">
                <xs:complexContent>
                <xs:extension base="xs:annotated">
                    <xs:group ref="xs:complexTypeModel"/>
                    <xs:attribute name="name" type="xs:NCName">
                    <xs:annotation>
                        <xs:documentation>
                Will be restricted to required or prohibited</xs:documentation>
                    </xs:annotation>
                    </xs:attribute>
                    <xs:attribute name="mixed" type="xs:boolean" use="optional">
                    <xs:annotation>
                        <xs:documentation>
                Not allowed if simpleContent child is chosen.
                May be overridden by setting on complexContent child.</xs:documentation>
                    </xs:annotation>
                    </xs:attribute>
                    <xs:attribute name="abstract" type="xs:boolean" default="false"
                                use="optional"/>
                    <xs:attribute name="final" type="xs:derivationSet"/>
                    <xs:attribute name="block" type="xs:derivationSet"/>
                    <xs:attribute name="defaultAttributesApply" type="xs:boolean"
                                default="true" use="optional"/>
                </xs:extension>
                </xs:complexContent>
            </xs:complexType>
            <xs:complexType name="topLevelComplexType">
                <xs:complexContent>
                <xs:restriction base="xs:complexBaseType">
                    <xs:sequence>
                    <xs:element ref="xs:annotation" minOccurs="0"/>
                    <xs:group ref="xs:complexTypeModel"/>
                    </xs:sequence>
                    <xs:attribute name="name" type="xs:NCName" use="required"/>
                    <xs:anyAttribute namespace="##other" processContents="lax"/>
                </xs:restriction>
                </xs:complexContent>
            </xs:complexType>
            <xs:complexType name="localComplexType">
                <xs:complexContent>
                <xs:restriction base="xs:complexBaseType">
                    <xs:sequence>
                    <xs:element ref="xs:annotation" minOccurs="0"/>
                    <xs:group ref="xs:complexTypeModel"/>
                    </xs:sequence>
                    <xs:attribute name="name" use="prohibited"/>
                    <xs:attribute name="abstract" use="prohibited"/>
                    <xs:attribute name="final" use="prohibited"/>
                    <xs:attribute name="block" use="prohibited"/>
                    <xs:anyAttribute namespace="##other" processContents="lax"/>
                </xs:restriction>
                </xs:complexContent>
            </xs:complexType>
            <xs:complexType name="restrictionType">
                <xs:complexContent>
                <xs:extension base="xs:annotated">
                    <xs:sequence>
                    <xs:choice minOccurs="0">
                        <xs:sequence>
                        <xs:element ref="xs:openContent" minOccurs="0"/>
                        <xs:group ref="xs:typeDefParticle"/>
                        </xs:sequence>
                        <xs:group ref="xs:simpleRestrictionModel"/>
                    </xs:choice>
                    <xs:group ref="xs:attrDecls"/>
                    <xs:group ref="xs:assertions"/>
                    </xs:sequence>
                    <xs:attribute name="base" type="xs:QName" use="required"/>
                </xs:extension>
                </xs:complexContent>
            </xs:complexType>
            <xs:complexType name="complexRestrictionType">
                <xs:complexContent>
                <xs:restriction base="xs:restrictionType">
                    <xs:sequence>
                    <xs:element ref="xs:annotation" minOccurs="0"/>
                    <xs:choice minOccurs="0">
                        <xs:annotation>
                        <xs:documentation>This choice is added simply to
                            make this a valid restriction per the REC</xs:documentation>
                        </xs:annotation>

                        <xs:sequence>
                        <xs:element ref="xs:openContent" minOccurs="0"/>
                        <xs:group ref="xs:typeDefParticle"/>
                        </xs:sequence>
                    </xs:choice>
                    <xs:group ref="xs:attrDecls"/>
                    <xs:group ref="xs:assertions"/>
                    </xs:sequence>
                    <xs:anyAttribute namespace="##other" processContents="lax"/>
                </xs:restriction>
                </xs:complexContent>
            </xs:complexType>
            <xs:complexType name="extensionType">
                <xs:complexContent>
                <xs:extension base="xs:annotated">
                    <xs:sequence>
                    <xs:element ref="xs:openContent" minOccurs="0"/>
                    <xs:group ref="xs:typeDefParticle" minOccurs="0"/>
                    <xs:group ref="xs:attrDecls"/>
                    <xs:group ref="xs:assertions"/>
                    </xs:sequence>
                    <xs:attribute name="base" type="xs:QName" use="required"/>

                </xs:extension>
                </xs:complexContent>
            </xs:complexType>
            <xs:element name="complexContent" id="complexContent">
                <xs:annotation>
                <xs:documentation
                    source="../structures/structures.html#element-complexContent"/>
                </xs:annotation>
                <xs:complexType>
                <xs:complexContent>
                    <xs:extension base="xs:annotated">
                    <xs:choice>
                        <xs:element name="restriction" type="xs:complexRestrictionType"/>
                        <xs:element name="extension" type="xs:extensionType"/>
                    </xs:choice>
                    <xs:attribute name="mixed" type="xs:boolean">
                        <xs:annotation>
                        <xs:documentation>
                Overrides any setting on complexType parent.</xs:documentation>
                        </xs:annotation>
                    </xs:attribute>
                    </xs:extension>
                </xs:complexContent>
                </xs:complexType>
            </xs:element>
            <xs:element name="openContent" id="openContent">
                <xs:annotation>
                <xs:documentation
                    source="../structures/structures.html#element-openContent"/>
                </xs:annotation>
                <xs:complexType>
                <xs:complexContent>
                    <xs:extension base="xs:annotated">
                    <xs:sequence>
                        <xs:element name="any" minOccurs="0" type="xs:wildcard"/>
                    </xs:sequence>
                    <xs:attribute name="mode" default="interleave" use="optional">
                        <xs:simpleType>
                        <xs:restriction base="xs:NMTOKEN">
                            <xs:enumeration value="none"/>
                            <xs:enumeration value="interleave"/>
                            <xs:enumeration value="suffix"/>
                        </xs:restriction>
                        </xs:simpleType>
                    </xs:attribute>

                    </xs:extension>
                </xs:complexContent>
                </xs:complexType>
            </xs:element>
            <xs:element name="defaultOpenContent" id="defaultOpenContent">
    <xs:annotation>
      <xs:documentation
           source="../structures/structures.html#element-defaultOpenContent"/>
    </xs:annotation>
    <xs:complexType>
      <xs:complexContent>
        <xs:extension base="xs:annotated">
          <xs:sequence>
            <xs:element name="any" type="xs:wildcard"/>
          </xs:sequence>
          <xs:attribute name="appliesToEmpty" type="xs:boolean"
                        default="false" use="optional"/>
          <xs:attribute name="mode" default="interleave" use="optional">
            <xs:simpleType>
              <xs:restriction base="xs:NMTOKEN">
                <xs:enumeration value="interleave"/>
                <xs:enumeration value="suffix"/>
              </xs:restriction>
            </xs:simpleType>
          </xs:attribute>

        </xs:extension>
      </xs:complexContent>
    </xs:complexType>
  </xs:element>
  <xs:complexType name="simpleRestrictionType">
    <xs:complexContent>
      <xs:restriction base="xs:restrictionType">
        <xs:sequence>
          <xs:element ref="xs:annotation" minOccurs="0"/>
          <xs:choice minOccurs="0">
            <xs:annotation>
              <xs:documentation>This choice is added simply to
                   make this a valid restriction per the REC</xs:documentation>
            </xs:annotation>
            <xs:group ref="xs:simpleRestrictionModel"/>
          </xs:choice>
          <xs:group ref="xs:attrDecls"/>
          <xs:group ref="xs:assertions"/>
        </xs:sequence>
        <xs:anyAttribute namespace="##other" processContents="lax"/>
      </xs:restriction>
    </xs:complexContent>
  </xs:complexType>
  <xs:complexType name="simpleExtensionType">
    <xs:complexContent>
      <xs:restriction base="xs:extensionType">
        <xs:sequence>
          <xs:annotation>
            <xs:documentation>
      No typeDefParticle group reference</xs:documentation>
          </xs:annotation>
          <xs:element ref="xs:annotation" minOccurs="0"/>
          <xs:group ref="xs:attrDecls"/>
          <xs:group ref="xs:assertions"/>
        </xs:sequence>
        <xs:anyAttribute namespace="##other" processContents="lax"/>
      </xs:restriction>
    </xs:complexContent>
  </xs:complexType>
  <xs:element name="simpleContent" id="simpleContent">
    <xs:annotation>
      <xs:documentation
           source="../structures/structures.html#element-simpleContent"/>
    </xs:annotation>
    <xs:complexType>
      <xs:complexContent>
        <xs:extension base="xs:annotated">
          <xs:choice>
            <xs:element name="restriction" type="xs:simpleRestrictionType"/>
            <xs:element name="extension" type="xs:simpleExtensionType"/>
          </xs:choice>
        </xs:extension>
      </xs:complexContent>
    </xs:complexType>
  </xs:element>
  <xs:element name="complexType" type="xs:topLevelComplexType" id="complexType">
    <xs:annotation>
      <xs:documentation
           source="../structures/structures.html#element-complexType"/>
    </xs:annotation>
  </xs:element>
  <xs:simpleType name="blockSet">
    <xs:annotation>
      <xs:documentation>
    A utility type, not for public use</xs:documentation>
      <xs:documentation>
    #all or (possibly empty) subset of {substitution, extension,
    restriction}</xs:documentation>
    </xs:annotation>
    <xs:union>
      <xs:simpleType>
        <xs:restriction base="xs:token">
          <xs:enumeration value="#all"/>
        </xs:restriction>
      </xs:simpleType>
      <xs:simpleType>
        <xs:list>
          <xs:simpleType>
            <xs:restriction base="xs:derivationControl">
              <xs:enumeration value="extension"/>
              <xs:enumeration value="restriction"/>
              <xs:enumeration value="substitution"/>
            </xs:restriction>
          </xs:simpleType>
        </xs:list>
      </xs:simpleType>
    </xs:union>
  </xs:simpleType>
  <xs:complexType name="element" abstract="true">
    <xs:annotation>
      <xs:documentation>
   The element element can be used either
   at the top level to define an element-type binding globally,
   or within a content model to either reference a globally-defined
   element or type or declare an element-type binding locally.
   The ref form is not allowed at the top level.</xs:documentation>
    </xs:annotation>
    <xs:complexContent>
      <xs:extension base="xs:annotated">
        <xs:sequence>
          <xs:choice minOccurs="0">
            <xs:element name="simpleType" type="xs:localSimpleType"/>
            <xs:element name="complexType" type="xs:localComplexType"/>
          </xs:choice>
          <xs:element name="alternative" type="xs:altType"
                    minOccurs="0" maxOccurs="unbounded"/>
          <xs:group ref="xs:identityConstraint" minOccurs="0"
                    maxOccurs="unbounded"/>
        </xs:sequence>
        <xs:attributeGroup ref="xs:defRef"/>
        <xs:attribute name="type" type="xs:QName"/>

        <xs:attribute name="substitutionGroup">
         <xs:simpleType>
          <xs:list itemType="xs:QName"/>
         </xs:simpleType>
        </xs:attribute>
        <xs:attributeGroup ref="xs:occurs"/>
        <xs:attribute name="default" type="xs:string"/>
        <xs:attribute name="fixed" type="xs:string"/>
        <xs:attribute name="nillable" type="xs:boolean" use="optional"/>
        <xs:attribute name="abstract" type="xs:boolean" default="false"
                      use="optional"/>
        <xs:attribute name="final" type="xs:derivationSet"/>
        <xs:attribute name="block" type="xs:blockSet"/>
        <xs:attribute name="form" type="xs:formChoice"/>
        <xs:attribute name="targetNamespace" type="xs:anyURI"/>
      </xs:extension>
    </xs:complexContent>
  </xs:complexType>
  <xs:complexType name="topLevelElement">
    <xs:complexContent>
      <xs:restriction base="xs:element">
        <xs:sequence>
          <xs:element ref="xs:annotation" minOccurs="0"/>
          <xs:choice minOccurs="0">
            <xs:element name="simpleType" type="xs:localSimpleType"/>
            <xs:element name="complexType" type="xs:localComplexType"/>
          </xs:choice>
          <xs:element name="alternative" type="xs:altType"
                    minOccurs="0" maxOccurs="unbounded"/>
          <xs:group ref="xs:identityConstraint" minOccurs="0"
                    maxOccurs="unbounded"/>
        </xs:sequence>
        <xs:attribute name="ref" use="prohibited"/>
        <xs:attribute name="form" use="prohibited"/>
        <xs:attribute name="targetNamespace" use="prohibited"/>
        <xs:attribute name="minOccurs" use="prohibited"/>
        <xs:attribute name="maxOccurs" use="prohibited"/>
        <xs:attribute name="name" type="xs:NCName" use="required"/>
        <xs:anyAttribute namespace="##other" processContents="lax"/>
      </xs:restriction>
    </xs:complexContent>
  </xs:complexType>
  <xs:complexType name="localElement">
    <xs:complexContent>
      <xs:restriction base="xs:element">
        <xs:sequence>
          <xs:element ref="xs:annotation" minOccurs="0"/>
          <xs:choice minOccurs="0">
            <xs:element name="simpleType" type="xs:localSimpleType"/>
            <xs:element name="complexType" type="xs:localComplexType"/>
          </xs:choice>
          <xs:element name="alternative" type="xs:altType"
                    minOccurs="0" maxOccurs="unbounded"/>
          <xs:group ref="xs:identityConstraint" minOccurs="0"
                    maxOccurs="unbounded"/>
        </xs:sequence>
        <xs:attribute name="substitutionGroup" use="prohibited"/>
        <xs:attribute name="final" use="prohibited"/>
        <xs:attribute name="abstract" use="prohibited"/>
        <xs:anyAttribute namespace="##other" processContents="lax"/>
      </xs:restriction>
    </xs:complexContent>
  </xs:complexType>
  <xs:element name="element" type="xs:topLevelElement" id="element">
    <xs:annotation>
      <xs:documentation
           source="../structures/structures.html#element-element"/>
    </xs:annotation>
  </xs:element>
  <xs:complexType name="altType">
    <xs:annotation>
      <xs:documentation>
        This type is used for 'alternative' elements.
      </xs:documentation>
    </xs:annotation>
    <xs:complexContent>
      <xs:extension base="xs:annotated">
        <xs:choice minOccurs="0">
          <xs:element name="simpleType" type="xs:localSimpleType"/>
          <xs:element name="complexType" type="xs:localComplexType"/>
        </xs:choice>
        <xs:attribute name="test" type="xs:string" use="optional"/>
        <xs:attribute name="type" type="xs:QName" use="optional"/>
        <xs:attribute name="xpathDefaultNamespace" type="xs:xpathDefaultNamespace"/>
      </xs:extension>
    </xs:complexContent>
  </xs:complexType>
  <xs:complexType name="group" abstract="true">
    <xs:annotation>
      <xs:documentation>
   group type for explicit groups, named top-level groups and
   group references</xs:documentation>
    </xs:annotation>
    <xs:complexContent>
      <xs:extension base="xs:annotated">

          <xs:group ref="xs:particle" minOccurs="0" maxOccurs="unbounded"/>

        <xs:attributeGroup ref="xs:defRef"/>
        <xs:attributeGroup ref="xs:occurs"/>
      </xs:extension>
    </xs:complexContent>
  </xs:complexType>
  <xs:complexType name="realGroup">
    <xs:complexContent>
      <xs:restriction base="xs:group">
        <xs:sequence>
          <xs:element ref="xs:annotation" minOccurs="0"/>
          <xs:choice minOccurs="0" maxOccurs="1">
            <xs:element ref="xs:all"/>
            <xs:element ref="xs:choice"/>
            <xs:element ref="xs:sequence"/>
          </xs:choice>

        </xs:sequence>
        <xs:anyAttribute namespace="##other" processContents="lax"/>
      </xs:restriction>
    </xs:complexContent>
  </xs:complexType>
  <xs:complexType name="namedGroup">
    <xs:complexContent>
      <xs:restriction base="xs:realGroup">
        <xs:sequence>
          <xs:element ref="xs:annotation" minOccurs="0"/>
          <xs:choice minOccurs="1" maxOccurs="1">
            <xs:element name="all">
              <xs:complexType>
                <xs:complexContent>
                  <xs:restriction base="xs:all">
                    <xs:group ref="xs:allModel"/>
                    <xs:attribute name="minOccurs" use="prohibited"/>
                    <xs:attribute name="maxOccurs" use="prohibited"/>
                    <xs:anyAttribute namespace="##other" processContents="lax"/>
                  </xs:restriction>
                </xs:complexContent>
              </xs:complexType>
            </xs:element>
            <xs:element name="choice" type="xs:simpleExplicitGroup"/>
            <xs:element name="sequence" type="xs:simpleExplicitGroup"/>
          </xs:choice>
        </xs:sequence>
        <xs:attribute name="name" type="xs:NCName" use="required"/>
        <xs:attribute name="ref" use="prohibited"/>
        <xs:attribute name="minOccurs" use="prohibited"/>
        <xs:attribute name="maxOccurs" use="prohibited"/>
        <xs:anyAttribute namespace="##other" processContents="lax"/>
      </xs:restriction>
    </xs:complexContent>
  </xs:complexType>
  <xs:complexType name="groupRef">
    <xs:complexContent>
      <xs:restriction base="xs:realGroup">
        <xs:sequence>
          <xs:element ref="xs:annotation" minOccurs="0"/>
        </xs:sequence>
        <xs:attribute name="ref" type="xs:QName" use="required"/>
        <xs:attribute name="name" use="prohibited"/>
        <xs:anyAttribute namespace="##other" processContents="lax"/>
      </xs:restriction>
    </xs:complexContent>
  </xs:complexType>
  <xs:complexType name="explicitGroup">
    <xs:annotation>
      <xs:documentation>
   group type for the three kinds of group</xs:documentation>
    </xs:annotation>
    <xs:complexContent>
      <xs:restriction base="xs:group">
        <xs:sequence>
          <xs:element ref="xs:annotation" minOccurs="0"/>
          <xs:group ref="xs:nestedParticle" minOccurs="0" maxOccurs="unbounded"/>
        </xs:sequence>
        <xs:attribute name="name" use="prohibited"/>
        <xs:attribute name="ref" use="prohibited"/>
        <xs:anyAttribute namespace="##other" processContents="lax"/>
      </xs:restriction>
    </xs:complexContent>
  </xs:complexType>
  <xs:complexType name="simpleExplicitGroup">
    <xs:complexContent>
      <xs:restriction base="xs:explicitGroup">
        <xs:sequence>
          <xs:element ref="xs:annotation" minOccurs="0"/>
          <xs:group ref="xs:nestedParticle" minOccurs="0" maxOccurs="unbounded"/>
        </xs:sequence>
        <xs:attribute name="minOccurs" use="prohibited"/>
        <xs:attribute name="maxOccurs" use="prohibited"/>
        <xs:anyAttribute namespace="##other" processContents="lax"/>
      </xs:restriction>
    </xs:complexContent>
  </xs:complexType>
  <xs:group name="allModel">
    <xs:sequence>
      <xs:element ref="xs:annotation" minOccurs="0"/>
      <xs:choice minOccurs="0" maxOccurs="unbounded">
        <xs:annotation>
          <xs:documentation>This choice with min/max is here to
                          avoid a pblm with the Elt:All/Choice/Seq
                          Particle derivation constraint</xs:documentation>
        </xs:annotation>
        <xs:element name="element" type="xs:localElement"/>
        <xs:element ref="xs:any"/>
        <xs:element name="group">
          <xs:complexType>
            <xs:complexContent>
              <xs:restriction base="xs:groupRef">
                <xs:sequence>
                  <xs:element ref="xs:annotation" minOccurs="0"/>
                </xs:sequence>
                <xs:attribute name="minOccurs" fixed="1" type="xs:nonNegativeInteger"/>
                <xs:attribute name="maxOccurs" fixed="1" type="xs:nonNegativeInteger"/>
              </xs:restriction>
            </xs:complexContent>
          </xs:complexType>
        </xs:element>
      </xs:choice>
    </xs:sequence>
  </xs:group>
  <xs:complexType name="all">
    <xs:annotation>
      <xs:documentation>
   Only elements allowed inside</xs:documentation>
    </xs:annotation>
    <xs:complexContent>
      <xs:restriction base="xs:explicitGroup">
        <xs:group ref="xs:allModel"/>
        <xs:attribute name="minOccurs" default="1" use="optional">
          <xs:simpleType>
            <xs:restriction base="xs:nonNegativeInteger">
              <xs:enumeration value="0"/>
              <xs:enumeration value="1"/>
            </xs:restriction>
          </xs:simpleType>
        </xs:attribute>
        <xs:attribute name="maxOccurs" default="1" use="optional">
          <xs:simpleType>
            <xs:restriction base="xs:allNNI">
              <xs:enumeration value="0"/>
              <xs:enumeration value="1"/>
            </xs:restriction>
          </xs:simpleType>
        </xs:attribute>
        <xs:anyAttribute namespace="##other" processContents="lax"/>
      </xs:restriction>
    </xs:complexContent>
  </xs:complexType>
  <xs:element name="all" type="xs:all" id="all">
    <xs:annotation>
      <xs:documentation source="../structures/structures.html#element-all"/>
    </xs:annotation>
  </xs:element>
  <xs:element name="choice" type="xs:explicitGroup" id="choice">
    <xs:annotation>
      <xs:documentation
           source="../structures/structures.html#element-choice"/>
    </xs:annotation>
  </xs:element>
  <xs:element name="sequence" type="xs:explicitGroup" id="sequence">
    <xs:annotation>
      <xs:documentation
           source="../structures/structures.html#element-sequence"/>
    </xs:annotation>
  </xs:element>
  <xs:element name="group" type="xs:namedGroup" id="group">
    <xs:annotation>
      <xs:documentation source="../structures/structures.html#element-group"/>
    </xs:annotation>
  </xs:element>
  <xs:attributeGroup name="anyAttrGroup">
    <xs:attribute name="namespace" type="xs:namespaceList"
                  use="optional"/>
    <xs:attribute name="notNamespace" use="optional">
      <xs:simpleType>
        <xs:restriction base="xs:basicNamespaceList">
          <xs:minLength value="1"/>
        </xs:restriction>
      </xs:simpleType>
    </xs:attribute>
    <xs:attribute name="processContents" default="strict" use="optional">
      <xs:simpleType>
        <xs:restriction base="xs:NMTOKEN">
          <xs:enumeration value="skip"/>
          <xs:enumeration value="lax"/>
          <xs:enumeration value="strict"/>
        </xs:restriction>
      </xs:simpleType>
    </xs:attribute>
  </xs:attributeGroup>
  <xs:complexType name="wildcard">
    <xs:complexContent>
      <xs:extension base="xs:annotated">

         <xs:attributeGroup ref="xs:anyAttrGroup"/>
      </xs:extension>
    </xs:complexContent>
  </xs:complexType>
  <xs:element name="any" id="any">
    <xs:annotation>
      <xs:documentation source="../structures/structures.html#element-any"/>
    </xs:annotation>
    <xs:complexType>
      <xs:complexContent>
        <xs:extension base="xs:wildcard">
          <xs:attribute name="notQName" type="xs:qnameList"
                        use="optional"/>
          <xs:attributeGroup ref="xs:occurs"/>
        </xs:extension>
      </xs:complexContent>
    </xs:complexType>
  </xs:element>
  <xs:annotation>
    <xs:documentation>
   simple type for the value of the 'namespace' attr of
   'any' and 'anyAttribute'</xs:documentation>
  </xs:annotation>
  <xs:annotation>
    <xs:documentation>
   Value is
              ##any      - - any non-conflicting WFXML/attribute at all

              ##other    - - any non-conflicting WFXML/attribute from
                              namespace other than targetNS

              ##local    - - any unqualified non-conflicting WFXML/attribute

              one or     - - any non-conflicting WFXML/attribute from
              more URI        the listed namespaces
              references
              (space separated)

    ##targetNamespace or ##local may appear in the above list, to
        refer to the targetNamespace of the enclosing
        schema or an absent targetNamespace respectively</xs:documentation>
  </xs:annotation>
  <xs:simpleType name="namespaceList">
    <xs:annotation>
      <xs:documentation>
   A utility type, not for public use</xs:documentation>
    </xs:annotation>

    <xs:union memberTypes="xs:specialNamespaceList xs:basicNamespaceList" />
  </xs:simpleType>
  <xs:simpleType name="basicNamespaceList">
    <xs:annotation>
      <xs:documentation>
   A utility type, not for public use</xs:documentation>
    </xs:annotation>
    <xs:list>
      <xs:simpleType>
        <xs:union memberTypes="xs:anyURI">
          <xs:simpleType>
            <xs:restriction base="xs:token">
              <xs:enumeration value="##targetNamespace"/>
              <xs:enumeration value="##local"/>
            </xs:restriction>
          </xs:simpleType>
        </xs:union>
      </xs:simpleType>
    </xs:list>
  </xs:simpleType>
  <xs:simpleType name="specialNamespaceList">
    <xs:annotation>
      <xs:documentation>
   A utility type, not for public use</xs:documentation>
    </xs:annotation>
    <xs:restriction base="xs:token">
      <xs:enumeration value="##any"/>
      <xs:enumeration value="##other"/>
    </xs:restriction>
  </xs:simpleType>
  <xs:simpleType name="qnameList">
    <xs:annotation>
      <xs:documentation>
        A utility type, not for public use
      </xs:documentation>
    </xs:annotation>
    <xs:list>
      <xs:simpleType>
        <xs:union memberTypes="xs:QName">
          <xs:simpleType>
            <xs:restriction base="xs:token">
              <xs:enumeration value="##defined"/>
              <xs:enumeration value="##definedSibling"/>
            </xs:restriction>
          </xs:simpleType>
        </xs:union>
      </xs:simpleType>
    </xs:list>
  </xs:simpleType>
  <xs:simpleType name="qnameListA">
    <xs:annotation>
      <xs:documentation>
        A utility type, not for public use
      </xs:documentation>
    </xs:annotation>
    <xs:list>
      <xs:simpleType>
        <xs:union memberTypes="xs:QName">
          <xs:simpleType>
            <xs:restriction base="xs:token">
              <xs:enumeration value="##defined"/>
            </xs:restriction>
          </xs:simpleType>
        </xs:union>
      </xs:simpleType>
    </xs:list>
  </xs:simpleType>
  <xs:simpleType name="xpathDefaultNamespace">
    <xs:union memberTypes="xs:anyURI">
      <xs:simpleType>
        <xs:restriction base="xs:token">
          <xs:enumeration value="##defaultNamespace"/>
          <xs:enumeration value="##targetNamespace"/>
          <xs:enumeration value="##local"/>
        </xs:restriction>
      </xs:simpleType>
    </xs:union>
  </xs:simpleType>
  <xs:element name="attribute" type="xs:topLevelAttribute" id="attribute">
    <xs:annotation>
      <xs:documentation
           source="../structures/structures.html#element-attribute"/>
    </xs:annotation>
  </xs:element>
  <xs:complexType name="attributeGroup" abstract="true">
    <xs:complexContent>
      <xs:extension base="xs:annotated">

          <xs:group ref="xs:attrDecls"/>

        <xs:attributeGroup ref="xs:defRef"/>
      </xs:extension>
    </xs:complexContent>
  </xs:complexType>
  <xs:complexType name="namedAttributeGroup">
    <xs:complexContent>
      <xs:restriction base="xs:attributeGroup">
        <xs:sequence>
          <xs:element ref="xs:annotation" minOccurs="0"/>
          <xs:group ref="xs:attrDecls"/>

        </xs:sequence>
        <xs:attribute name="name" type="xs:NCName" use="required"/>
        <xs:attribute name="ref" use="prohibited"/>
        <xs:anyAttribute namespace="##other" processContents="lax"/>
      </xs:restriction>
    </xs:complexContent>
  </xs:complexType>
  <xs:complexType name="attributeGroupRef">
    <xs:complexContent>
      <xs:restriction base="xs:attributeGroup">
        <xs:sequence>
          <xs:element ref="xs:annotation" minOccurs="0"/>
        </xs:sequence>
        <xs:attribute name="ref" type="xs:QName" use="required"/>
        <xs:attribute name="name" use="prohibited"/>
        <xs:anyAttribute namespace="##other" processContents="lax"/>
      </xs:restriction>
    </xs:complexContent>
  </xs:complexType>
  <xs:element name="attributeGroup" type="xs:namedAttributeGroup"
              id="attributeGroup">
    <xs:annotation>
      <xs:documentation
           source="../structures/structures.html#element-attributeGroup"/>
    </xs:annotation>
  </xs:element>
  <xs:element name="include" id="include">
    <xs:annotation>
      <xs:documentation
           source="../structures/structures.html#element-include"/>
    </xs:annotation>
    <xs:complexType>
      <xs:complexContent>
        <xs:extension base="xs:annotated">
          <xs:attribute name="schemaLocation" type="xs:anyURI" use="required"/>
        </xs:extension>
      </xs:complexContent>
    </xs:complexType>
  </xs:element>
  <xs:element name="redefine" id="redefine">
    <xs:annotation>
      <xs:documentation
           source="../structures/structures.html#element-redefine"/>
    </xs:annotation>
    <xs:complexType>
      <xs:complexContent>
        <xs:extension base="xs:openAttrs">
          <xs:choice minOccurs="0" maxOccurs="unbounded">
            <xs:element ref="xs:annotation"/>
            <xs:group ref="xs:redefinable"/>
          </xs:choice>
          <xs:attribute name="schemaLocation" type="xs:anyURI" use="required"/>
          <xs:attribute name="id" type="xs:ID"/>
        </xs:extension>
      </xs:complexContent>
    </xs:complexType>
  </xs:element>

  <xs:element name="override" id="override">
    <xs:annotation>
      <xs:documentation
           source="../structures/structures.html#element-override"/>
    </xs:annotation>
    <xs:complexType>
      <xs:complexContent>
        <xs:extension base="xs:openAttrs">
          <xs:sequence>
            <xs:element ref="xs:annotation" minOccurs="0"/>
            <xs:group ref="xs:schemaTop" minOccurs="0" maxOccurs="unbounded"/>
          </xs:sequence>
          <xs:attribute name="schemaLocation" type="xs:anyURI" use="required"/>
          <xs:attribute name="id" type="xs:ID"/>
        </xs:extension>
      </xs:complexContent>
    </xs:complexType>
  </xs:element>
  <xs:element name="import" id="import">
    <xs:annotation>
      <xs:documentation
           source="../structures/structures.html#element-import"/>
    </xs:annotation>
    <xs:complexType>
      <xs:complexContent>
        <xs:extension base="xs:annotated">
          <xs:attribute name="namespace" type="xs:anyURI"/>
          <xs:attribute name="schemaLocation" type="xs:anyURI"/>
        </xs:extension>
      </xs:complexContent>
    </xs:complexType>
  </xs:element>
  <xs:element name="selector" id="selector">
    <xs:annotation>
      <xs:documentation
           source="../structures/structures.html#element-selector"/>
    </xs:annotation>
    <xs:complexType>
      <xs:complexContent>
        <xs:extension base="xs:annotated">
          <xs:attribute name="xpath" use="required">
            <xs:simpleType>
              <xs:annotation>
                <xs:documentation>A subset of XPath expressions for use
in selectors</xs:documentation>
                <xs:documentation>A utility type, not for public
use</xs:documentation>
              </xs:annotation>
              <xs:restriction base="xs:token"/>

            </xs:simpleType>
          </xs:attribute>
          <xs:attribute name="xpathDefaultNamespace" type="xs:xpathDefaultNamespace"/>
        </xs:extension>
      </xs:complexContent>
    </xs:complexType>
  </xs:element>
  <xs:element name="field" id="field">
    <xs:annotation>
      <xs:documentation source="../structures/structures.html#element-field"/>
    </xs:annotation>
    <xs:complexType>
      <xs:complexContent>
        <xs:extension base="xs:annotated">
          <xs:attribute name="xpath" use="required">
            <xs:simpleType>
              <xs:annotation>
                <xs:documentation>A subset of XPath expressions for use
in fields</xs:documentation>
                <xs:documentation>A utility type, not for public
use</xs:documentation>
              </xs:annotation>
              <xs:restriction base="xs:token"/>

            </xs:simpleType>
          </xs:attribute>
          <xs:attribute name="xpathDefaultNamespace" type="xs:xpathDefaultNamespace"/>
        </xs:extension>
      </xs:complexContent>
    </xs:complexType>
  </xs:element>
  <xs:complexType name="keybase">
    <xs:complexContent>
      <xs:extension base="xs:annotated">
        <xs:sequence minOccurs="0">
          <xs:element ref="xs:selector"/>
          <xs:element ref="xs:field" minOccurs="1" maxOccurs="unbounded"/>
        </xs:sequence>
        <xs:attribute name="name" type="xs:NCName"/>
        <xs:attribute name="ref" type="xs:QName"/>
      </xs:extension>
    </xs:complexContent>
  </xs:complexType>
  <xs:group name="identityConstraint">
    <xs:annotation>
      <xs:documentation>The three kinds of identity constraints, all with
                     type of or derived from 'keybase'.
   </xs:documentation>
    </xs:annotation>
    <xs:choice>
      <xs:element ref="xs:unique"/>
      <xs:element ref="xs:key"/>
      <xs:element ref="xs:keyref"/>
    </xs:choice>
  </xs:group>
  <xs:element name="unique" type="xs:keybase" id="unique">
    <xs:annotation>
      <xs:documentation
           source="../structures/structures.html#element-unique"/>
    </xs:annotation>
  </xs:element>
  <xs:element name="key" type="xs:keybase" id="key">
    <xs:annotation>
      <xs:documentation source="../structures/structures.html#element-key"/>
    </xs:annotation>
  </xs:element>
  <xs:element name="keyref" id="keyref">
    <xs:annotation>
      <xs:documentation
           source="../structures/structures.html#element-keyref"/>
    </xs:annotation>
    <xs:complexType>
      <xs:complexContent>
        <xs:extension base="xs:keybase">
          <xs:attribute name="refer" type="xs:QName"/>
        </xs:extension>
      </xs:complexContent>
    </xs:complexType>
  </xs:element>
  <xs:element name="notation" id="notation">
    <xs:annotation>
      <xs:documentation
           source="../structures/structures.html#element-notation"/>
    </xs:annotation>
    <xs:complexType>
      <xs:complexContent>
        <xs:extension base="xs:annotated">
          <xs:attribute name="name" type="xs:NCName" use="required"/>
          <xs:attribute name="public" type="xs:public"/>
          <xs:attribute name="system" type="xs:anyURI"/>
        </xs:extension>
      </xs:complexContent>
    </xs:complexType>
  </xs:element>
  <xs:simpleType name="public">
    <xs:annotation>
      <xs:documentation>
   A utility type, not for public use</xs:documentation>
      <xs:documentation>
   A public identifier, per ISO 8879</xs:documentation>
    </xs:annotation>
    <xs:restriction base="xs:token"/>
  </xs:simpleType>
  <xs:element name="appinfo" id="appinfo">
    <xs:annotation>
      <xs:documentation
           source="../structures/structures.html#element-appinfo"/>
    </xs:annotation>
    <xs:complexType mixed="true">
      <xs:sequence minOccurs="0" maxOccurs="unbounded">
        <xs:any processContents="lax"/>
      </xs:sequence>
      <xs:attribute name="source" type="xs:anyURI"/>
      <xs:anyAttribute namespace="##other" processContents="lax"/>
    </xs:complexType>
  </xs:element>
  <xs:element name="documentation" id="documentation">
    <xs:annotation>
      <xs:documentation
           source="../structures/structures.html#element-documentation"/>
    </xs:annotation>
    <xs:complexType mixed="true">
      <xs:sequence minOccurs="0" maxOccurs="unbounded">
        <xs:any processContents="lax"/>
      </xs:sequence>
      <xs:attribute name="source" type="xs:anyURI"/>
      <xs:attribute ref="xml:lang"/>
      <xs:anyAttribute namespace="##other" processContents="lax"/>
    </xs:complexType>
  </xs:element>
  <xs:element name="annotation" id="annotation">
    <xs:annotation>
      <xs:documentation
           source="../structures/structures.html#element-annotation"/>
    </xs:annotation>
    <xs:complexType>
      <xs:complexContent>
        <xs:extension base="xs:openAttrs">
          <xs:choice minOccurs="0" maxOccurs="unbounded">
            <xs:element ref="xs:appinfo"/>
            <xs:element ref="xs:documentation"/>
          </xs:choice>
          <xs:attribute name="id" type="xs:ID"/>
        </xs:extension>
      </xs:complexContent>
    </xs:complexType>
  </xs:element>
  <xs:annotation>
    <xs:documentation>
   notations for use within  schema documents</xs:documentation>
  </xs:annotation>
  <xs:notation name="XMLSchemaStructures" public="structures"
               system="http://www.w3.org/2000/08/XMLSchema.xsd"/>
  <xs:notation name="XML" public="REC-xml-19980210"
               system="http://www.w3.org/TR/1998/REC-xml-19980210"/>
  <xs:complexType name="anyType" mixed="true">
    <xs:annotation>
      <xs:documentation>
   Not the real urType, but as close an approximation as we can
   get in the XML representation</xs:documentation>
    </xs:annotation>
    <xs:sequence>
      <xs:any minOccurs="0" maxOccurs="unbounded" processContents="lax"/>
    </xs:sequence>
    <xs:anyAttribute processContents="lax"/>
  </xs:complexType>

  <xs:annotation>
    <xs:documentation>
      In keeping with the XML Schema WG's standard versioning policy,
      the material in this schema document will persist at the URI
      http://www.w3.org/2012/04/XMLSchema.xsd.

      At the date of issue it can also be found at the URI
      http://www.w3.org/2009/XMLSchema/XMLSchema.xsd.

      The schema document at that URI may however change in the future,
      in order to remain compatible with the latest version of XSD
      and its namespace.  In other words, if XSD or the XML Schema
      namespace change, the version of this document at
      http://www.w3.org/2009/XMLSchema/XMLSchema.xsd will change accordingly;
      the version at http://www.w3.org/2012/04/XMLSchema.xsd will not change.

      Previous dated (and unchanging) versions of this schema document
      include:

       http://www.w3.org/2012/01/XMLSchema.xsd
          (XSD 1.1 Proposed Recommendation)

        http://www.w3.org/2011/07/XMLSchema.xsd
          (XSD 1.1 Candidate Recommendation)

        http://www.w3.org/2009/04/XMLSchema.xsd
          (XSD 1.1 Candidate Recommendation)

        http://www.w3.org/2004/10/XMLSchema.xsd
          (XSD 1.0 Recommendation, Second Edition)

        http://www.w3.org/2001/05/XMLSchema.xsd
          (XSD 1.0 Recommendation, First Edition)


    </xs:documentation>
  </xs:annotation>




  <xs:simpleType name="derivationControl">
    <xs:annotation>
      <xs:documentation>
   A utility type, not for public use</xs:documentation>
    </xs:annotation>
    <xs:restriction base="xs:NMTOKEN">
      <xs:enumeration value="substitution"/>
      <xs:enumeration value="extension"/>
      <xs:enumeration value="restriction"/>
      <xs:enumeration value="list"/>
      <xs:enumeration value="union"/>
    </xs:restriction>
  </xs:simpleType>
  <xs:group name="simpleDerivation">
    <xs:choice>
      <xs:element ref="xs:restriction"/>
      <xs:element ref="xs:list"/>
      <xs:element ref="xs:union"/>
    </xs:choice>
  </xs:group>
  <xs:simpleType name="simpleDerivationSet">
    <xs:annotation>
      <xs:documentation>
   #all or (possibly empty) subset of {restriction, extension, union, list}
   </xs:documentation>
      <xs:documentation>
   A utility type, not for public use</xs:documentation>
    </xs:annotation>
    <xs:union>
      <xs:simpleType>
        <xs:restriction base="xs:token">
          <xs:enumeration value="#all"/>
        </xs:restriction>
      </xs:simpleType>
      <xs:simpleType>
        <xs:list>
          <xs:simpleType>
            <xs:restriction base="xs:derivationControl">
              <xs:enumeration value="list"/>
              <xs:enumeration value="union"/>
              <xs:enumeration value="restriction"/>
              <xs:enumeration value="extension"/>
            </xs:restriction>
          </xs:simpleType>
        </xs:list>
      </xs:simpleType>
    </xs:union>
  </xs:simpleType>
  <xs:complexType name="simpleBaseType" abstract="true">
    <xs:complexContent>
      <xs:extension base="xs:annotated">
        <xs:group ref="xs:simpleDerivation"/>
        <xs:attribute name="final" type="xs:simpleDerivationSet"/>
        <xs:attribute name="name" type="xs:NCName">
          <xs:annotation>
            <xs:documentation>
              Can be restricted to required or forbidden
            </xs:documentation>
          </xs:annotation>
        </xs:attribute>
      </xs:extension>
    </xs:complexContent>
  </xs:complexType>
  <xs:complexType name="topLevelSimpleType">
    <xs:complexContent>
      <xs:restriction base="xs:simpleBaseType">
        <xs:sequence>
          <xs:element ref="xs:annotation" minOccurs="0"/>
          <xs:group ref="xs:simpleDerivation"/>
        </xs:sequence>
        <xs:attribute name="name" type="xs:NCName" use="required">
          <xs:annotation>
            <xs:documentation>
              Required at the top level
            </xs:documentation>
          </xs:annotation>
        </xs:attribute>
        <xs:anyAttribute namespace="##other" processContents="lax"/>
      </xs:restriction>
    </xs:complexContent>
  </xs:complexType>
  <xs:complexType name="localSimpleType">
    <xs:complexContent>
      <xs:restriction base="xs:simpleBaseType">
        <xs:sequence>
          <xs:element ref="xs:annotation" minOccurs="0"/>
          <xs:group ref="xs:simpleDerivation"/>
        </xs:sequence>
        <xs:attribute name="name" use="prohibited">
          <xs:annotation>
            <xs:documentation>
              Forbidden when nested
            </xs:documentation>
          </xs:annotation>
        </xs:attribute>
        <xs:attribute name="final" use="prohibited"/>
        <xs:anyAttribute namespace="##other" processContents="lax"/>
      </xs:restriction>
    </xs:complexContent>
  </xs:complexType>
  <xs:element name="simpleType" type="xs:topLevelSimpleType" id="simpleType">
    <xs:annotation>
      <xs:documentation
           source="http://www.w3.org/TR/xmlschema11-2/#element-simpleType"/>
    </xs:annotation>
  </xs:element>
  <xs:element name="facet" abstract="true">
    <xs:annotation>
      <xs:documentation>
        An abstract element, representing facets in general.
        The facets defined by this spec are substitutable for
        this element, and implementation-defined facets should
        also name this as a substitution-group head.
      </xs:documentation>
    </xs:annotation>
  </xs:element>
  <xs:group name="simpleRestrictionModel">
    <xs:sequence>
      <xs:element name="simpleType" type="xs:localSimpleType" minOccurs="0"/>
      <xs:choice minOccurs="0"
          maxOccurs="unbounded">
        <xs:element ref="xs:facet"/>
        <xs:any processContents="lax"
            namespace="##other"/>
      </xs:choice>
    </xs:sequence>
  </xs:group>
  <xs:element name="restriction" id="restriction">
    <xs:complexType>
      <xs:annotation>
        <xs:documentation
             source="http://www.w3.org/TR/xmlschema11-2/#element-restriction">
          base attribute and simpleType child are mutually
          exclusive, but one or other is required
        </xs:documentation>
      </xs:annotation>
      <xs:complexContent>
        <xs:extension base="xs:annotated">
          <xs:group ref="xs:simpleRestrictionModel"/>
          <xs:attribute name="base" type="xs:QName" use="optional"/>
        </xs:extension>
      </xs:complexContent>
    </xs:complexType>
  </xs:element>
  <xs:element name="list" id="list">
    <xs:complexType>
      <xs:annotation>
        <xs:documentation
             source="http://www.w3.org/TR/xmlschema11-2/#element-list">
          itemType attribute and simpleType child are mutually
          exclusive, but one or other is required
        </xs:documentation>
      </xs:annotation>
      <xs:complexContent>
        <xs:extension base="xs:annotated">
          <xs:sequence>
            <xs:element name="simpleType" type="xs:localSimpleType"
                        minOccurs="0"/>
          </xs:sequence>
          <xs:attribute name="itemType" type="xs:QName" use="optional"/>
        </xs:extension>
      </xs:complexContent>
    </xs:complexType>
  </xs:element>
  <xs:element name="union" id="union">
    <xs:complexType>
      <xs:annotation>
        <xs:documentation
             source="http://www.w3.org/TR/xmlschema11-2/#element-union">
          memberTypes attribute must be non-empty or there must be
          at least one simpleType child
        </xs:documentation>
      </xs:annotation>
      <xs:complexContent>
        <xs:extension base="xs:annotated">
          <xs:sequence>
            <xs:element name="simpleType" type="xs:localSimpleType"
                        minOccurs="0" maxOccurs="unbounded"/>
          </xs:sequence>
          <xs:attribute name="memberTypes" use="optional">
            <xs:simpleType>
              <xs:list itemType="xs:QName"/>
            </xs:simpleType>
          </xs:attribute>
        </xs:extension>
      </xs:complexContent>
    </xs:complexType>
  </xs:element>
  <xs:complexType name="facet">
    <xs:complexContent>
      <xs:extension base="xs:annotated">
        <xs:attribute name="value" use="required"/>
        <xs:attribute name="fixed" type="xs:boolean" default="false"
                      use="optional"/>
      </xs:extension>
    </xs:complexContent>
  </xs:complexType>
  <xs:complexType name="noFixedFacet">
    <xs:complexContent>
      <xs:restriction base="xs:facet">
        <xs:sequence>
          <xs:element ref="xs:annotation" minOccurs="0"/>
        </xs:sequence>
        <xs:attribute name="fixed" use="prohibited"/>
        <xs:anyAttribute namespace="##other" processContents="lax"/>
      </xs:restriction>
    </xs:complexContent>
  </xs:complexType>
  <xs:element name="minExclusive" type="xs:facet"
    id="minExclusive"
    substitutionGroup="xs:facet">
    <xs:annotation>
      <xs:documentation
           source="http://www.w3.org/TR/xmlschema11-2/#element-minExclusive"/>
    </xs:annotation>
  </xs:element>
  <xs:element name="minInclusive" type="xs:facet"
    id="minInclusive"
    substitutionGroup="xs:facet">
    <xs:annotation>
      <xs:documentation
           source="http://www.w3.org/TR/xmlschema11-2/#element-minInclusive"/>
    </xs:annotation>
  </xs:element>
  <xs:element name="maxExclusive" type="xs:facet"
    id="maxExclusive"
    substitutionGroup="xs:facet">
    <xs:annotation>
      <xs:documentation
           source="http://www.w3.org/TR/xmlschema11-2/#element-maxExclusive"/>
    </xs:annotation>
  </xs:element>
  <xs:element name="maxInclusive" type="xs:facet"
    id="maxInclusive"
    substitutionGroup="xs:facet">
    <xs:annotation>
      <xs:documentation
           source="http://www.w3.org/TR/xmlschema11-2/#element-maxInclusive"/>
    </xs:annotation>
  </xs:element>
  <xs:complexType name="numFacet">
    <xs:complexContent>
      <xs:restriction base="xs:facet">
        <xs:sequence>
          <xs:element ref="xs:annotation" minOccurs="0"/>
        </xs:sequence>
        <xs:attribute name="value"
            type="xs:nonNegativeInteger" use="required"/>
        <xs:anyAttribute namespace="##other" processContents="lax"/>
      </xs:restriction>
    </xs:complexContent>
  </xs:complexType>

  <xs:complexType name="intFacet">
    <xs:complexContent>
      <xs:restriction base="xs:facet">
        <xs:sequence>
          <xs:element ref="xs:annotation" minOccurs="0"/>
        </xs:sequence>
        <xs:attribute name="value" type="xs:integer" use="required"/>
        <xs:anyAttribute namespace="##other" processContents="lax"/>
      </xs:restriction>
    </xs:complexContent>
  </xs:complexType>

  <xs:element name="totalDigits" id="totalDigits"
    substitutionGroup="xs:facet">
    <xs:annotation>
      <xs:documentation
           source="http://www.w3.org/TR/xmlschema11-2/#element-totalDigits"/>
    </xs:annotation>
    <xs:complexType>
      <xs:complexContent>
        <xs:restriction base="xs:numFacet">
          <xs:sequence>
            <xs:element ref="xs:annotation" minOccurs="0"/>
          </xs:sequence>
          <xs:attribute name="value" type="xs:positiveInteger" use="required"/>
          <xs:anyAttribute namespace="##other" processContents="lax"/>
        </xs:restriction>
      </xs:complexContent>
    </xs:complexType>
  </xs:element>
  <xs:element name="fractionDigits" type="xs:numFacet"
    id="fractionDigits"
    substitutionGroup="xs:facet">
    <xs:annotation>
      <xs:documentation
           source="http://www.w3.org/TR/xmlschema11-2/#element-fractionDigits"/>
    </xs:annotation>
  </xs:element>

  <xs:element name="length" type="xs:numFacet" id="length"
    substitutionGroup="xs:facet">
    <xs:annotation>
      <xs:documentation
           source="http://www.w3.org/TR/xmlschema11-2/#element-length"/>
    </xs:annotation>
  </xs:element>
  <xs:element name="minLength" type="xs:numFacet"
    id="minLength"
    substitutionGroup="xs:facet">
    <xs:annotation>
      <xs:documentation
           source="http://www.w3.org/TR/xmlschema11-2/#element-minLength"/>
    </xs:annotation>
  </xs:element>
  <xs:element name="maxLength" type="xs:numFacet"
    id="maxLength"
    substitutionGroup="xs:facet">
    <xs:annotation>
      <xs:documentation
           source="http://www.w3.org/TR/xmlschema11-2/#element-maxLength"/>
    </xs:annotation>
  </xs:element>
  <xs:element name="enumeration" type="xs:noFixedFacet"
    id="enumeration"
    substitutionGroup="xs:facet">
    <xs:annotation>
      <xs:documentation
           source="http://www.w3.org/TR/xmlschema11-2/#element-enumeration"/>
    </xs:annotation>
  </xs:element>
  <xs:element name="whiteSpace" id="whiteSpace"
    substitutionGroup="xs:facet">
    <xs:annotation>
      <xs:documentation
           source="http://www.w3.org/TR/xmlschema11-2/#element-whiteSpace"/>
    </xs:annotation>
    <xs:complexType>
      <xs:complexContent>
        <xs:restriction base="xs:facet">
          <xs:sequence>
            <xs:element ref="xs:annotation" minOccurs="0"/>
          </xs:sequence>
          <xs:attribute name="value" use="required">
            <xs:simpleType>
              <xs:restriction base="xs:NMTOKEN">
                <xs:enumeration value="preserve"/>
                <xs:enumeration value="replace"/>
                <xs:enumeration value="collapse"/>
              </xs:restriction>
            </xs:simpleType>
          </xs:attribute>
          <xs:anyAttribute namespace="##other" processContents="lax"/>
        </xs:restriction>
      </xs:complexContent>
    </xs:complexType>
  </xs:element>
  <xs:element name="pattern" id="pattern"
    substitutionGroup="xs:facet">
    <xs:annotation>
      <xs:documentation
           source="http://www.w3.org/TR/xmlschema11-2/#element-pattern"/>
    </xs:annotation>
    <xs:complexType>
      <xs:complexContent>
        <xs:restriction base="xs:noFixedFacet">
          <xs:sequence>
            <xs:element ref="xs:annotation" minOccurs="0"/>
          </xs:sequence>
          <xs:attribute name="value" type="xs:string"
              use="required"/>
          <xs:anyAttribute namespace="##other"
              processContents="lax"/>
        </xs:restriction>
      </xs:complexContent>
    </xs:complexType>
  </xs:element>
  <xs:element name="assertion" type="xs:assertion"
              id="assertion" substitutionGroup="xs:facet">
    <xs:annotation>
      <xs:documentation
           source="http://www.w3.org/TR/xmlschema11-2/#element-assertion"/>
    </xs:annotation>
  </xs:element>
  <xs:element name="explicitTimezone" id="explicitTimezone"
    substitutionGroup="xs:facet">
    <xs:annotation>
      <xs:documentation
           source="http://www.w3.org/TR/xmlschema11-2/#element-explicitTimezone"/>
    </xs:annotation>
    <xs:complexType>
      <xs:complexContent>
        <xs:restriction base="xs:facet">
          <xs:sequence>
            <xs:element ref="xs:annotation" minOccurs="0"/>
          </xs:sequence>
          <xs:attribute name="value" use="required">
            <xs:simpleType>
              <xs:restriction base="xs:NMTOKEN">
                <xs:enumeration value="optional"/>
                <xs:enumeration value="required"/>
                <xs:enumeration value="prohibited"/>
              </xs:restriction>
            </xs:simpleType>
          </xs:attribute>
          <xs:anyAttribute namespace="##other" processContents="lax"/>
        </xs:restriction>
      </xs:complexContent>
    </xs:complexType>
  </xs:element>

  <xs:annotation>
    <xs:documentation>
      In keeping with the XML Schema WG's standard versioning policy,
      this schema document will persist at the URI
      http://www.w3.org/2012/04/datatypes.xsd.

      At the date of issue it can also be found at the URI
      http://www.w3.org/2009/XMLSchema/datatypes.xsd.

      The schema document at that URI may however change in the future,
      in order to remain compatible with the latest version of XSD
      and its namespace.  In other words, if XSD or the XML Schema
      namespace change, the version of this document at
      http://www.w3.org/2009/XMLSchema/datatypes.xsd will change accordingly;
      the version at http://www.w3.org/2012/04/datatypes.xsd will not change.

      Previous dated (and unchanging) versions of this schema document
      include:

        http://www.w3.org/2012/01/datatypes.xsd
          (XSD 1.1 Proposed Recommendation)

        http://www.w3.org/2011/07/datatypes.xsd
          (XSD 1.1 Candidate Recommendation)

        http://www.w3.org/2009/04/datatypes.xsd
          (XSD 1.1 Candidate Recommendation)

        http://www.w3.org/2004/10/datatypes.xsd
          (XSD 1.0 Recommendation, Second Edition)

        http://www.w3.org/2001/05/datatypes.xsd
          (XSD 1.0 Recommendation, First Edition)

    </xs:documentation>
  </xs:annotation>

        </xs:schema>
        "###
    )]
    fn deserialize_schema(#[case] xml: &str) {
        let result: Schema = xmlity_quick_xml::from_str(xml).unwrap();

        println!("{result:#?}");
    }
}
