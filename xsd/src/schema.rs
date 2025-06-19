use core::fmt;
use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};

use bon::Builder;
use xmlity::{
    de::NamespaceContext, types::string::FromStrVisitor, DeserializationGroup, Deserialize,
    ExpandedName, LocalName, Prefix, SerializationGroup, Serialize, SerializeAttribute,
    XmlNamespace,
};

pub const NS_XSD: XmlNamespace = XmlNamespace::new_dangerous("http://www.w3.org/2001/XMLSchema");
pub const NS_XML: XmlNamespace =
    XmlNamespace::new_dangerous("http://www.w3.org/XML/1998/namespace");

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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QName(pub ExpandedName<'static>);

impl_to_string_serialize!(QName);

struct QNameVisitor;

impl<'de> xmlity::de::Visitor<'de> for QNameVisitor {
    type Value = QName;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a QName")
    }

    fn visit_text<E, V>(self, value: V) -> Result<Self::Value, E>
    where
        E: xmlity::de::Error,
        V: xmlity::de::XmlText<'de>,
    {
        let mut qname_parts = value.as_str().split(":");
        let first_part = qname_parts.next().expect("Always has at least one part.");
        let last_part = qname_parts.next();

        let expanded_name = match last_part {
            Some(last_part) => {
                let prefix = Prefix::new(first_part).unwrap();
                let local_name = LocalName::new(last_part).unwrap().into_owned();

                let namespace = value
                    .namespace_context()
                    .resolve_prefix(prefix)
                    .unwrap()
                    .into_owned();

                ExpandedName::new(local_name, Some(namespace))
            }
            None => {
                let local_name = LocalName::new(first_part).unwrap().into_owned();

                let default_namespace = value
                    .namespace_context()
                    .default_namespace()
                    .map(XmlNamespace::into_owned);

                return Ok(QName(ExpandedName::new(local_name, default_namespace)));
            }
        };

        Ok(QName(expanded_name))
    }
}

impl<'de> Deserialize<'de> for QName {
    fn deserialize<D: xmlity::Deserializer<'de>>(reader: D) -> Result<Self, D::Error> {
        reader.deserialize_any(QNameVisitor)
    }
}

impl Display for QName {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for QName {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        //TODO: REDO REQUIRES SUPPORT IN XMLITY
        let mut name_parts = s.split(":");
        let mut localname = name_parts.next().unwrap();

        if let Some(localname_new) = name_parts.next() {
            localname = localname_new;
        }
        let local_name = LocalName::new(localname).unwrap().into_owned();

        let expanded_name = if s.starts_with("xs:") {
            ExpandedName::new(local_name, Some(XmlNamespace::XS))
        } else if s.starts_with("xml:") {
            ExpandedName::new(local_name, Some(XmlNamespace::XML))
        } else {
            todo!()
        };

        Ok(QName(expanded_name))
    }
}

/// Represents the minimum occurrence of types or elements
#[derive(Debug, Clone, Copy, Eq, PartialEq, SerializeAttribute, Deserialize, Default)]
#[xattribute(name = "minOccurs")]
pub struct MinOccurs(pub usize);

/// Represents the maximum occurrence of types or elements
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub enum MaxOccursValue {
    /// The occurrence is unbounded.
    Unbounded,

    /// The occurrence is bound to the specified limit.
    Bounded(usize),
}

impl Default for MaxOccursValue {
    fn default() -> Self {
        Self::Bounded(1)
    }
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

impl_to_string_serialize!(MaxOccursValue);
impl_from_str_deserialize!(MaxOccursValue);

#[derive(Debug, Clone, Copy, Eq, PartialEq, SerializeAttribute, Deserialize, Default)]
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
    #[xattribute(name = "targetNamespace")]
    pub target_namespace: XmlNamespace<'static>,
    #[xattribute(deferred = true, default)]
    pub version: Option<Version>,
    #[xattribute(deferred = true, default)]
    pub final_default: Option<FinalDefault>,
    #[xattribute(deferred = true, default)]
    pub block_default: Option<BlockDefault>,
    #[xattribute(deferred = true, default)]
    pub attribute_form_default: Option<AttributeFormDefault>,
    #[xattribute(deferred = true, default)]
    pub element_form_default: Option<ElementFormDefault>,
    #[xattribute(deferred = true, default)]
    pub default_attributes: Option<DefaultAttributes>,
    #[xattribute(deferred = true, default)]
    pub xpath_default_namespace: Option<XPathDefaultNamespace>,
    #[xattribute(deferred = true, default)]
    pub id: Option<Id>,
    #[xattribute(deferred = true, default)]
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
    #[xattribute(deferred = true)]
    pub id: Option<Id>,
    #[xattribute(deferred = true)]
    pub schema_location: SchemaLocation,
    pub annotation: Option<Annotation>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[xelement(name = "import", namespace_expr = NS_XSD)]
pub struct Import {
    #[xattribute(deferred = true, default)]
    pub id: Option<Id>,
    #[xattribute(deferred = true, default)]
    pub namespace: Option<Namespace>,
    #[xattribute(deferred = true, default)]
    pub schema_location: Option<SchemaLocation>,
    #[xvalue(default)]
    pub annotation: Option<Annotation>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[xelement(name = "redefine", namespace_expr = NS_XSD)]
pub struct Redefine {
    pub schema_location: String,
    #[xattribute(deferred = true)]
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
    #[xattribute(deferred = true)]
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

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Builder)]
#[xelement(name = "annotation", namespace_expr = NS_XSD)]
pub struct Annotation {
    #[xattribute(deferred = true, default = true)]
    pub id: Option<Id>,
    pub content: Vec<AnnotationContent>,
}

#[derive(Debug, Clone, Eq, PartialEq, SerializationGroup, DeserializationGroup)]
pub struct Annotated {
    pub annotations: Vec<Annotation>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, ::derive_more::derive::From)]
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
    #[xattribute(deferred = true)]
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

#[derive(
    Debug,
    Clone,
    Eq,
    PartialEq,
    Serialize,
    Deserialize,
    SerializationGroup,
    DeserializationGroup,
    Builder,
)]
#[xelement(name = "simpleType", namespace_expr = NS_XSD)]
pub struct LocalSimpleType {
    #[xattribute(deferred = true, default)]
    pub id: Option<Id>,
    #[xattribute(deferred = true, default)]
    pub final_: Option<Final2>,
    #[xvalue(default)]
    pub annotation: Option<Annotation>,
    pub content: SimpleDerivation,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Builder)]
#[xelement(name = "simpleType", namespace_expr = NS_XSD)]
pub struct TopLevelSimpleType {
    #[xattribute(deferred = true, default)]
    pub id: Option<Id>,
    #[xattribute(deferred = true, default)]
    pub final_: Option<Final2>,
    #[xattribute(name = "name")]
    pub name: LocalName<'static>,
    #[xvalue(default)]
    pub annotation: Option<Annotation>,
    pub content: SimpleDerivation,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum SimpleDerivation {
    Restriction(Box<SimpleRestrictionType>),
    List(Box<List>),
    Union(Box<Union>),
}

impl From<SimpleRestrictionType> for SimpleDerivation {
    fn from(value: SimpleRestrictionType) -> Self {
        Self::Restriction(Box::new(value))
    }
}

impl From<List> for SimpleDerivation {
    fn from(value: List) -> Self {
        Self::List(Box::new(value))
    }
}
impl From<Union> for SimpleDerivation {
    fn from(value: Union) -> Self {
        Self::Union(Box::new(value))
    }
}

#[derive(Debug, Clone, Eq, PartialEq, SerializeAttribute, Deserialize)]
#[xattribute(name = "name")]
pub struct NameAttr(pub LocalName<'static>);

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

#[derive(
    Debug,
    Clone,
    Eq,
    PartialEq,
    Serialize,
    Deserialize,
    SerializationGroup,
    DeserializationGroup,
    Builder,
)]
#[xelement(name = "complexType", namespace_expr = NS_XSD)]
pub struct LocalComplexType {
    #[xattribute(name = "id", optional)]
    pub id: Option<String>,
    #[xattribute(name = "mixed", optional)]
    pub mixed: Option<bool>,
    #[xattribute(name = "abstract", optional)]
    pub abstract_: Option<bool>,
    #[xattribute(name = "final", optional)]
    pub final_: Option<DerivationSetType>,
    #[xattribute(name = "block", optional)]
    pub block: Option<DerivationSetType>,
    #[xattribute(name = "defaultAttributesApply", optional)]
    pub default_attributes_apply: Option<bool>,
    #[xvalue(default)]
    pub annotation: Option<Annotation>,
    pub content: ComplexTypeModel,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Builder)]
#[xelement(name = "complexType", namespace_expr = NS_XSD)]
pub struct TopLevelComplexType {
    #[xattribute(deferred = true, default)]
    pub id: Option<Id>,
    #[xattribute(name = "name")]
    pub name: LocalName<'static>,
    #[xattribute(name = "mixed", optional)]
    pub mixed: Option<bool>,
    #[xattribute(deferred = true, default)]
    pub abstract_: Option<Abstract>,
    #[xattribute(deferred = true, default)]
    pub final_: Option<Final>,
    #[xattribute(deferred = true, default)]
    pub block: Option<Block>,
    #[xattribute(deferred = true, default)]
    pub default_attributes_apply: Option<DefaultAttributesApply>,
    #[xvalue(default)]
    pub annotation: Option<Annotation>,
    pub content: ComplexTypeModel,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum ComplexTypeModel {
    SimpleContent(SimpleContent),
    ComplexContent(ComplexContent),
    Other {
        #[xvalue(default)]
        open_content: Option<OpenContent>,
        #[xvalue(default)]
        type_def_particle: Option<TypeDefParticle>,
        attr_decls: AttrDecls,
        // #[xvalue(default)]
        // assertions: Assertions,
    },
}

impl From<SimpleContent> for ComplexTypeModel {
    fn from(value: SimpleContent) -> Self {
        Self::SimpleContent(value)
    }
}

impl From<ComplexContent> for ComplexTypeModel {
    fn from(value: ComplexContent) -> Self {
        Self::ComplexContent(value)
    }
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

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Builder)]
#[xelement(name = "group", namespace_expr = NS_XSD)]
pub struct GroupType {
    #[xattribute(deferred = true, default)]
    pub id: Option<Id>,
    #[xattribute(name = "name")]
    pub name: LocalName<'static>,
    #[xattribute(deferred = true, default)]
    pub min_occurs: Option<MinOccurs>,
    #[xattribute(deferred = true, default)]
    pub max_occurs: Option<MaxOccurs>,
    #[xvalue(default)]
    pub annotation: Option<Annotation>,
    pub content: NamedGroupTypeContent,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum NamedGroupTypeContent {
    All(Box<AllType>),
    Choice(Box<ChoiceType>),
    Sequence(Box<SequenceType>),
}

impl From<AllType> for NamedGroupTypeContent {
    fn from(value: AllType) -> Self {
        Self::All(Box::new(value))
    }
}

impl From<ChoiceType> for NamedGroupTypeContent {
    fn from(value: ChoiceType) -> Self {
        Self::Choice(Box::new(value))
    }
}

impl From<SequenceType> for NamedGroupTypeContent {
    fn from(value: SequenceType) -> Self {
        Self::Sequence(Box::new(value))
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Builder)]
#[xelement(name = "group", namespace_expr = NS_XSD)]
pub struct GroupRef {
    #[xattribute(deferred = true, default)]
    pub id: Option<Id>,
    #[xattribute(name = "ref")]
    pub ref_: QName,
    #[xattribute(deferred = true, default)]
    pub min_occurs: Option<MinOccurs>,
    #[xattribute(deferred = true, default)]
    pub max_occurs: Option<MaxOccurs>,
    #[xvalue(default)]
    pub annotation: Option<Annotation>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Builder)]
#[xelement(name = "all", namespace_expr = NS_XSD)]
pub struct AllType {
    #[xattribute(deferred = true, default)]
    pub id: Option<Id>,
    #[xattribute(deferred = true, default)]
    pub name: Option<NameAttr>,
    #[xattribute(name = "ref", optional)]
    pub ref_: Option<QName>,
    #[xattribute(deferred = true, default)]
    pub min_occurs: Option<MinOccurs>,
    #[xattribute(deferred = true, default)]
    pub max_occurs: Option<MaxOccurs>,
    pub content: Vec<GroupTypeContent>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Builder)]
#[xelement(name = "choice", namespace_expr = NS_XSD)]
pub struct ChoiceType {
    #[xattribute(deferred = true, default)]
    pub id: Option<Id>,
    #[xattribute(deferred = true, default)]
    pub name: Option<NameAttr>,
    #[xattribute(name = "ref", optional)]
    pub ref_: Option<QName>,
    #[xattribute(deferred = true, default)]
    pub min_occurs: Option<MinOccurs>,
    #[xattribute(deferred = true, default)]
    pub max_occurs: Option<MaxOccurs>,
    #[xvalue(default)]
    pub annotation: Option<Annotation>,
    pub content: Vec<GroupTypeContent>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Builder)]
#[xelement(name = "sequence", namespace_expr = NS_XSD)]
pub struct SequenceType {
    #[xattribute(deferred = true, default)]
    pub id: Option<Id>,
    #[xattribute(deferred = true, default)]
    pub name: Option<NameAttr>,
    #[xattribute(name = "ref", optional)]
    pub ref_: Option<QName>,
    #[xattribute(deferred = true, default)]
    pub min_occurs: Option<MinOccurs>,
    #[xattribute(deferred = true, default)]
    pub max_occurs: Option<MaxOccurs>,
    pub content: Vec<GroupTypeContent>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum GroupTypeContent {
    Element(Box<LocalElement>),
    Group(Box<GroupRef>),
    All(Box<AllType>),
    Choice(Box<ChoiceType>),
    Sequence(Box<SequenceType>),
    Any(Box<Any>),
}

impl From<LocalElement> for GroupTypeContent {
    fn from(value: LocalElement) -> Self {
        Self::Element(Box::new(value))
    }
}

impl From<GroupRef> for GroupTypeContent {
    fn from(value: GroupRef) -> Self {
        Self::Group(Box::new(value))
    }
}
impl From<AllType> for GroupTypeContent {
    fn from(value: AllType) -> Self {
        Self::All(Box::new(value))
    }
}
impl From<ChoiceType> for GroupTypeContent {
    fn from(value: ChoiceType) -> Self {
        Self::Choice(Box::new(value))
    }
}
impl From<SequenceType> for GroupTypeContent {
    fn from(value: SequenceType) -> Self {
        Self::Sequence(Box::new(value))
    }
}
impl From<Any> for GroupTypeContent {
    fn from(value: Any) -> Self {
        Self::Any(Box::new(value))
    }
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

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Builder)]
#[xelement(
    name = "attributeGroup",
    namespace_expr = NS_XSD
)]
pub struct AttributeGroupType {
    #[xattribute(deferred = true, default)]
    pub id: Option<Id>,
    #[xattribute(name = "name")]
    pub name: LocalName<'static>,
    #[xvalue(default)]
    pub annotation: Option<Annotation>,
    pub attr_decls: AttrDecls,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Builder)]
pub struct AttrDecls {
    #[builder(default)]
    pub declarations: Vec<AttributeDeclaration>,
    pub any: Option<AnyAttribute>,
}

impl Default for AttrDecls {
    fn default() -> Self {
        Self {
            declarations: Vec::new(),
            any: None,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Builder)]
#[xelement(
    name = "attributeGroup",
    namespace_expr = NS_XSD
)]
pub struct AttributeGroupRefType {
    #[xattribute(deferred = true, default)]
    pub id: Option<Id>,
    #[xattribute(name = "ref")]
    pub ref_: QName,
    #[xvalue(default)]
    pub annotation: Option<Annotation>,
}

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

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TargetNamespace(pub XmlNamespace<'static>);

impl Display for TargetNamespace {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for TargetNamespace {
    type Err = <XmlNamespace<'static> as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(XmlNamespace::from_str(s)?))
    }
}

impl_to_string_serialize!(TargetNamespace);
impl_from_str_deserialize!(TargetNamespace);

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Builder)]
#[xelement(name = "element", namespace_expr = NS_XSD)]
pub struct LocalElement {
    #[xattribute(deferred = true, default)]
    pub id: Option<Id>,
    #[xattribute(name = "name", optional)]
    pub name: Option<LocalName<'static>>,
    #[xattribute(name = "ref", optional)]
    pub ref_: Option<QName>,
    #[xattribute(name = "type", optional)]
    pub type_: Option<QName>,
    #[xattribute(deferred = true, default)]
    pub min_occurs: Option<MinOccurs>,
    #[xattribute(deferred = true, default)]
    pub max_occurs: Option<MaxOccurs>,
    #[xattribute(name = "default", optional)]
    pub default: Option<String>,
    #[xattribute(deferred = true, default)]
    pub fixed: Option<Fixed>,
    #[xattribute(deferred = true, default)]
    pub nillable: Option<Nillable>,
    #[xattribute(deferred = true, default)]
    pub block: Option<Block2>,
    #[xattribute(deferred = true, default)]
    pub form: Option<Form>,
    #[xattribute(name = "targetNamespace", optional)]
    pub target_namespace: Option<TargetNamespace>,
    #[xvalue(default)]
    pub annotation: Option<Annotation>,
    #[xvalue(default)]
    pub type_choice: Option<types::top_level_element_items::Child1>,
    #[xvalue(default)]
    #[builder(default)]
    pub alternatives: Vec<AltType>,
    #[xvalue(default)]
    #[builder(default)]
    pub identity_constraints: Vec<IdentityConstraint>,
}

impl LocalElement {
    pub fn new_ref_typed(name: LocalName<'static>, type_: ExpandedName<'static>) -> Self {
        Self {
            id: None,
            name: Some(name),
            ref_: None,
            type_: Some(QName(type_)),
            min_occurs: None,
            max_occurs: None,
            default: None,
            fixed: None,
            nillable: None,
            block: None,
            form: None,
            target_namespace: None,
            annotation: None,
            type_choice: None,
            alternatives: vec![],
            identity_constraints: vec![],
        }
    }
}

// #[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Builder)]
// #[xelement(name = "element", namespace_expr = NS_XSD)]
// pub struct TopLevelElement {
//     #[xattribute(deferred = true, default)]
//     pub id: Option<Id>,
//     #[xattribute(deferred = true)]
//     pub name: Name,
//     #[xattribute(deferred = true, default)]
//     pub type_: Option<Type>,
//     #[xattribute(deferred = true, default)]
//     pub substitution_group: Option<SubstitutionGroup>,
//     #[xattribute(deferred = true, default)]
//     pub default: Option<DefaultAttr>,
//     #[xattribute(deferred = true, default)]
//     pub fixed: Option<Fixed>,
//     #[xattribute(deferred = true, default)]
//     pub nillable: Option<Nillable>,
//     #[xattribute(deferred = true, default)]
//     pub abstract_: Option<Abstract>,
//     #[xattribute(deferred = true, default)]
//     pub final_: Option<Final>,
//     #[xattribute(deferred = true, default)]
//     pub block: Option<Block2>,
//     #[xattribute(deferred = true, default)]
//     pub form: Option<Form>,
//     #[xattribute(deferred = true, default)]
//     pub target_namespace: Option<TargetNamespace>,
//     #[xvalue(default)]
//     pub annotation: Option<Annotation>,
//     #[xvalue(default)]
//     pub type_choice: Option<ElementTypeContent>,
//     #[xvalue(default)]
//     #[builder(default)]
//     pub alternatives: Vec<AltType>,
//     #[xvalue(default)]
//     #[builder(default)]
//     pub identity_constraints: Vec<IdentityConstraint>,
// }

pub mod types {
    use bon::Builder;
    use xmlity::LocalName;

    pub mod top_level_element_items {
        #[derive(
            ::core::fmt::Debug,
            Clone,
            Eq,
            PartialEq,
            ::xmlity::Serialize,
            ::xmlity::Deserialize,
            ::derive_more::derive::From,
        )]
        pub enum Child1 {
            #[xelement(name = "simpleType", namespace = "http://www.w3.org/2001/XMLSchema")]
            SimpleType(#[xgroup] super::super::LocalSimpleType),
            #[xelement(name = "complexType", namespace = "http://www.w3.org/2001/XMLSchema")]
            ComplexType(#[xgroup] super::super::LocalComplexType),
        }
    }

    #[derive(
        ::core::fmt::Debug,
        Clone,
        Eq,
        PartialEq,
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        Builder,
    )]
    #[xgroup(children_order = "strict")]
    pub struct TopLevelElement {
        #[xattribute(name = "id", optional)]
        pub id: Option<String>,
        #[xattribute(name = "type", optional)]
        pub type_: Option<super::QName>,
        #[xattribute(name = "substitutionGroup", optional)]
        pub substitution_group: Option<String>,
        #[xattribute(name = "default", optional)]
        pub default: Option<String>,
        #[xattribute(name = "fixed", optional)]
        pub fixed: Option<String>,
        #[xattribute(name = "nillable", optional)]
        pub nillable: Option<bool>,
        #[xattribute(name = "abstract", optional)]
        pub abstract_: Option<bool>,
        #[xattribute(name = "final", optional)]
        pub final_: Option<super::DerivationSetType>,
        #[xattribute(name = "block", optional)]
        pub block: Option<super::BlockSetType>,
        #[xattribute(name = "name")]
        pub name: LocalName<'static>,
        #[xvalue(default)]
        pub annotation: Option<super::Annotation>,
        #[xvalue(default)]
        pub child_1: Option<top_level_element_items::Child1>,
        // #[xelement(
        //     name = "alternative",
        //     namespace = "http://www.w3.org/2001/XMLSchema",
        //     group,
        //     optional,
        //     default
        // )]
        #[xvalue(default)]
        pub alternative: Option<super::AltType>,
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, ::derive_more::derive::From)]
#[xelement(name = "element", namespace_expr = NS_XSD)]
pub struct TopLevelElement(#[xgroup] pub types::TopLevelElement);

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum ElementTypeContent {
    SimpleType(Box<LocalSimpleType>),
    ComplexType(Box<LocalComplexType>),
}

impl From<LocalSimpleType> for ElementTypeContent {
    fn from(value: LocalSimpleType) -> Self {
        Self::SimpleType(Box::new(value))
    }
}

impl From<LocalComplexType> for ElementTypeContent {
    fn from(value: LocalComplexType) -> Self {
        Self::ComplexType(Box::new(value))
    }
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

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Builder)]
#[xelement(name = "attribute", namespace_expr = NS_XSD)]
pub struct LocalAttribute {
    #[xattribute(deferred = true, default)]
    pub id: Option<Id>,
    #[xattribute(name = "name", optional)]
    pub name: Option<LocalName<'static>>,
    #[xattribute(name = "ref", optional)]
    pub ref_: Option<QName>,
    #[xattribute(name = "type", optional)]
    pub type_: Option<QName>,
    #[xattribute(name = "use", optional)]
    pub use_: Option<AttributeUseType>,
    #[xattribute(name = "default", optional)]
    pub default: Option<String>,
    #[xattribute(deferred = true, default)]
    pub fixed: Option<Fixed>,
    #[xattribute(deferred = true, default)]
    pub form: Option<Form>,
    #[xattribute(name = "targetNamespace", optional)]
    pub target_namespace: Option<TargetNamespace>,
    #[xattribute(deferred = true, default)]
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

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Builder)]
#[xelement(name = "attribute", namespace_expr = NS_XSD)]
pub struct TopLevelAttribute {
    #[xattribute(deferred = true, default)]
    pub id: Option<Id>,
    #[xattribute(deferred = true)]
    pub name: NameAttr,
    #[xattribute(deferred = true, default)]
    pub type_: Option<Type>,
    #[xattribute(deferred = true, default)]
    pub use_: Option<AttrUse>,
    #[xattribute(name = "default", optional)]
    pub default: Option<String>,
    #[xattribute(deferred = true, default)]
    pub fixed: Option<Fixed>,
    #[xattribute(deferred = true, default)]
    pub form: Option<Form>,
    #[xattribute(name = "targetNamespace", optional)]
    pub target_namespace: Option<TargetNamespace>,
    #[xattribute(deferred = true, default)]
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
    #[xattribute(deferred = true, default)]
    pub id: Option<Id>,
    #[xattribute(deferred = true)]
    pub name: NameAttr,
    #[xattribute(deferred = true, default)]
    pub public: Option<Public>,
    #[xattribute(deferred = true, default)]
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
    #[xattribute(name = "source", optional)]
    pub source: Option<String>,
}

#[derive(Debug, Clone, Eq, PartialEq, SerializeAttribute, Deserialize)]
#[xattribute(name = "lang", namespace_expr = NS_XML)]
pub struct XmlLang(pub String);

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Builder)]
#[xelement(name = "documentation", namespace_expr = NS_XSD)]
pub struct Documentation {
    #[xattribute(name = "source", optional)]
    pub source: Option<String>,
    #[xattribute(deferred = true, default)]
    pub lang: Option<XmlLang>,
    #[xvalue(default)]
    #[builder(default)]
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
    #[xattribute(deferred = true)]
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

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Builder)]
#[xelement(name = "list", namespace_expr = NS_XSD)]
pub struct List {
    #[xattribute(deferred = true, default)]
    pub id: Option<Id>,
    #[xvalue(default)]
    pub annotation: Option<Annotation>,
    #[xvalue(default)]
    pub simple_type: Option<LocalSimpleType>,
    #[xattribute(deferred = true, default)]
    pub item_type: Option<ItemType>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Builder)]
#[xelement(name = "union", namespace_expr = NS_XSD)]
pub struct Union {
    #[xattribute(deferred = true, default)]
    pub id: Option<Id>,
    #[xattribute(name = "memberTypes", optional)]
    pub member_types: Option<ElementSubstitutionGroupType>,
    #[xvalue(default)]
    pub annotation: Option<Annotation>,
    #[xvalue(default)]
    pub simple_types: Vec<LocalSimpleType>,
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

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Builder)]
#[xelement(name = "simpleContent", namespace_expr = NS_XSD)]
pub struct SimpleContent {
    #[xattribute(deferred = true)]
    pub id: Option<Id>,
    pub content: Vec<SimpleContentContent>,
}
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum SimpleContentContent {
    Annotation(Box<Annotation>),
    Restriction(Box<Restriction>),
    Extension(Box<ExtensionType>),
}
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Builder)]
#[xelement(
    name = "complexContent",
    namespace_expr = NS_XSD
)]
pub struct ComplexContent {
    #[xattribute(deferred = true, default)]
    pub id: Option<Id>,
    #[xattribute(name = "mixed", optional)]
    pub mixed: Option<bool>,
    #[xvalue(default)]
    pub annotation: Option<Annotation>,
    pub content: ComplexContentContent,
}
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum ComplexContentContent {
    Restriction(Box<ComplexRestrictionType>),
    Extension(Box<ExtensionType>),
}

impl From<ComplexRestrictionType> for ComplexContentContent {
    fn from(value: ComplexRestrictionType) -> Self {
        Self::Restriction(Box::new(value))
    }
}

impl From<ExtensionType> for ComplexContentContent {
    fn from(value: ExtensionType) -> Self {
        Self::Extension(Box::new(value))
    }
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
    #[xattribute(deferred = true)]
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
#[xattribute(name = "notQName")]
pub struct NotQName(pub QnameListAType);

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Builder)]
#[xelement(name = "anyAttribute", namespace_expr = NS_XSD)]
pub struct AnyAttribute {
    #[xattribute(deferred = true, default)]
    pub id: Option<Id>,
    #[xattribute(name = "namespace", optional)]
    pub namespace: Option<NamespaceListType>,
    #[xattribute(name = "notNamespace", optional)]
    pub not_namespace: Option<NamespaceListType>,
    #[xattribute(name = "processContents")]
    pub process_contents: ProcessContentsType,
    #[xattribute(deferred = true, default)]
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
    #[xattribute(deferred = true, default)]
    pub id: Option<Id>,
    #[xattribute(deferred = true, default)]
    pub test: Option<Test>,
    #[xattribute(deferred = true, default)]
    #[xattribute(deferred = true, default)]
    pub xpath_default_namespace: Option<XPathDefaultNamespace>,
    #[xvalue(default)]
    pub annotation: Option<Annotation>,
}

#[derive(Debug, Clone, Eq, PartialEq, SerializeAttribute, Deserialize)]
#[xattribute(name = "notQName")]
pub struct NotQNameAnyAttribute(pub QnameListType);

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Builder)]
#[xelement(name = "any", namespace_expr = NS_XSD)]
pub struct Any {
    #[xattribute(deferred = true, default)]
    pub id: Option<Id>,
    #[xattribute(name = "namespace", optional)]
    pub namespace: Option<NamespaceListType>,
    #[xattribute(name = "notNamespace", optional)]
    pub not_namespace: Option<NamespaceListType>,
    #[xattribute(name = "processContents")]
    pub process_contents: ProcessContentsType,
    #[xattribute(deferred = true, default)]
    pub not_q_name: Option<NotQNameAnyAttribute>,
    #[xattribute(deferred = true, default)]
    pub min_occurs: Option<MinOccurs>,
    #[xattribute(deferred = true, default)]
    pub max_occurs: Option<MaxOccurs>,
    #[xvalue(default)]
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
    #[xattribute(deferred = true, default)]
    pub id: Option<Id>,
    #[xattribute(deferred = true, default)]
    pub test: Option<Test>,
    #[xattribute(deferred = true, default)]
    pub type_: Option<Type>,
    #[xattribute(deferred = true, default)]
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
    #[xattribute(deferred = true, default)]
    pub id: Option<Id>,
    #[xattribute(deferred = true, default)]
    pub name: Option<NameAttr>,
    #[xattribute(name = "ref", optional)]
    pub ref_: Option<QName>,
    #[xgroup]
    pub content: Option<KeybaseTypeContent>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[xelement(name = "key", namespace_expr = NS_XSD)]
pub struct Key {
    #[xattribute(deferred = true, default)]
    pub id: Option<Id>,
    #[xattribute(deferred = true, default)]
    pub name: Option<NameAttr>,
    #[xattribute(name = "ref", optional)]
    pub ref_: Option<QName>,
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
    #[xattribute(deferred = true, default)]
    pub id: Option<Id>,
    #[xattribute(deferred = true, default)]
    pub name: Option<NameAttr>,
    #[xattribute(name = "ref", optional)]
    pub ref_: Option<QName>,
    #[xattribute(deferred = true, default)]
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

impl_from_str_deserialize!(SimpleDerivationSetItemList);
impl_to_string_serialize!(SimpleDerivationSetItemList);

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Builder)]
#[xelement(name = "enumeration", namespace_expr = NS_XSD)]
pub struct Enumeration {
    #[xattribute(deferred = true, default)]
    pub fixed: Option<Fixed>,
    #[xattribute(name = "value")]
    pub value: String,
}

#[derive(Debug, Clone, Eq, PartialEq, SerializationGroup, DeserializationGroup, Builder)]
#[xgroup]
pub struct FacetType {
    #[xattribute(deferred = true, default)]
    pub fixed: Option<Fixed>,
    #[xvalue(default)]
    pub annotation: Option<Annotation>,
    #[xattribute(name = "value")]
    pub value: String,
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
#[xelement(name = "minLength", namespace_expr = NS_XSD)]
pub struct MinLength(#[xgroup] pub FacetType);

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, derive_more::derive::From)]
pub enum Facet {
    MinExclusive(Box<MinExclusive>),
    MinInclusive(Box<MinInclusive>),
    MaxExclusive(Box<MaxExclusive>),
    MaxInclusive(Box<MaxInclusive>),
    // TotalDigits(Box<FacetType>),
    // FractionDigits(Box<FacetType>),
    // Length(Box<FacetType>),
    MinLength(Box<MinLength>),
    // MaxLength(Box<FacetType>),
    Enumeration(Box<Enumeration>),
    // WhiteSpace(Box<FacetType>),
    // Pattern(Box<FacetType>),
    // Assertion(Box<AssertionType>),
    // ExplicitTimezone(Box<FacetType>),
}

impl From<MinExclusive> for Facet {
    fn from(value: MinExclusive) -> Self {
        Self::MinExclusive(Box::new(value))
    }
}

impl From<MinInclusive> for Facet {
    fn from(value: MinInclusive) -> Self {
        Self::MinInclusive(Box::new(value))
    }
}

impl From<MaxExclusive> for Facet {
    fn from(value: MaxExclusive) -> Self {
        Self::MaxExclusive(Box::new(value))
    }
}

impl From<MaxInclusive> for Facet {
    fn from(value: MaxInclusive) -> Self {
        Self::MaxInclusive(Box::new(value))
    }
}

impl From<MinLength> for Facet {
    fn from(value: MinLength) -> Self {
        Self::MinLength(Box::new(value))
    }
}

impl From<Enumeration> for Facet {
    fn from(value: Enumeration) -> Self {
        Self::Enumeration(Box::new(value))
    }
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

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[xelement(name = "restriction", namespace_expr = NS_XSD)]
pub struct Restriction {
    #[xattribute(deferred = true, default)]
    pub id: Option<Id>,
    #[xattribute(name = "base")]
    pub base: QName,
    #[xvalue(default)]
    pub content: Vec<RestrictionTypeContent>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum RestrictionTypeContent {
    Annotation(Box<Annotation>),
    OpenContent(Box<OpenContent>),
    Group(Box<GroupRef>),
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

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Builder)]
#[xelement(name = "restriction", namespace_expr = NS_XSD)]
pub struct SimpleRestrictionType {
    #[xattribute(deferred = true, default)]
    pub id: Option<Id>,
    #[xattribute(name = "base")]
    pub base: QName,
    #[xvalue(default)]
    pub annotation: Option<Annotation>,
    #[xvalue(default)]
    pub simple_type: Option<LocalSimpleType>,
    #[xvalue(default)]
    pub facets: Vec<Facet>,
    //attrDecals
    //assertions
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Builder)]
#[xelement(name = "restriction", namespace_expr = NS_XSD)]
pub struct ComplexRestrictionType {
    #[xattribute(deferred = true, default)]
    pub id: Option<Id>,
    #[xvalue(default)]
    pub annotation: Option<Annotation>,
    #[xattribute(name = "base")]
    pub base: QName,
    #[xvalue(default)]
    pub simple_type: Option<LocalSimpleType>,
    #[xvalue(default)]
    pub open_content: Option<OpenContent>,
    #[xvalue(default)]
    pub particle: Option<TypeDefParticle>,
    #[builder(default)]
    pub attr_decls: AttrDecls,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum AttributeDeclaration {
    Attribute(LocalAttribute),
    AttributeGroup(AttributeGroupRefType),
}

impl From<LocalAttribute> for AttributeDeclaration {
    fn from(value: LocalAttribute) -> Self {
        Self::Attribute(value)
    }
}
impl From<AttributeGroupRefType> for AttributeDeclaration {
    fn from(value: AttributeGroupRefType) -> Self {
        Self::AttributeGroup(value)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum TypeDefParticle {
    Group(GroupRef),
    All(AllType),
    Choice(ChoiceType),
    Sequence(SequenceType),
}

impl From<GroupRef> for TypeDefParticle {
    fn from(value: GroupRef) -> Self {
        Self::Group(value)
    }
}
impl From<AllType> for TypeDefParticle {
    fn from(value: AllType) -> Self {
        Self::All(value)
    }
}
impl From<ChoiceType> for TypeDefParticle {
    fn from(value: ChoiceType) -> Self {
        Self::Choice(value)
    }
}
impl From<SequenceType> for TypeDefParticle {
    fn from(value: SequenceType) -> Self {
        Self::Sequence(value)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Builder)]
#[xelement(name = "extension", namespace_expr = NS_XSD)]
pub struct ExtensionType {
    #[xattribute(deferred = true, default)]
    pub id: Option<Id>,
    #[xattribute(name = "base")]
    pub base: QName,
    #[xvalue(default)]
    pub annotation: Option<Annotation>,
    #[xvalue(default)]
    pub open_content: Option<OpenContent>,
    #[xvalue(default)]
    pub particle: Option<TypeDefParticle>,
    #[builder(default)]
    pub attr_decls: AttrDecls,
    // #[xvalue(default)]
    // pub attr_decls: Option<AttrDeclsType>,
    // #[xvalue(default)]
    // pub assertions: Option<AssertionsType>,
}
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum ExtensionTypeContent {
    Annotation(Box<Annotation>),
    OpenContent(Box<OpenContent>),
    Group(Box<GroupRef>),
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
    #[xattribute(deferred = true, default)]
    pub id: Option<Id>,
    #[xattribute(deferred = true)]
    pub xpath: XPath,
    #[xattribute(deferred = true, default)]
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
    #[xattribute(deferred = true, default)]
    pub id: Option<Id>,
    #[xattribute(deferred = true)]
    pub xpath: XPath,
    #[xattribute(deferred = true, default)]
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
