use xsd::schema::{Namespace, XmlValue};

pub mod schema;

impl From<schema::Schema> for xsd::schema::Schema {
    fn from(value: schema::Schema) -> Self {
        Self {
            target_namespace: value.target_namespace.map(|a| a.0),
            version: value.version.map(Into::into),
            final_default: value.final_default.map(|a| a.0).map(Into::into),
            block_default: value.block_default.map(|a| a.0).map(Into::into),
            attribute_form_default: value.attribute_form_default.map(|a| a.0).map(Into::into),
            element_form_default: value.element_form_default.map(|a| a.0).map(Into::into),
            default_attributes: value.default_attributes.map(|a| a.0).map(Into::into),
            xpath_default_namespace: value.xpath_default_namespace.map(|a| a.0).map(Into::into),
            id: value.id.map(Into::into),
            lang: value.lang.map(Into::into),
            compositions: value.compositions.into_iter().map(Into::into).collect(),
            open_content: value.open_content.into_iter().map(Into::into).collect(),
            schema_top: value.schema_top.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<schema::Version> for xsd::schema::Version {
    fn from(value: schema::Version) -> Self {
        Self(value.0)
    }
}

impl From<schema::TypeDerivationControlType> for xsd::schema::TypeDerivationControlType {
    fn from(value: schema::TypeDerivationControlType) -> Self {
        match value {
            schema::TypeDerivationControlType::Extension => Self::Extension,
            schema::TypeDerivationControlType::Restriction => Self::Restriction,
            schema::TypeDerivationControlType::List => Self::List,
            schema::TypeDerivationControlType::Union => Self::Union,
        }
    }
}

impl From<schema::FullDerivationSetType> for xsd::schema::FullDerivationSetType {
    fn from(value: schema::FullDerivationSetType) -> Self {
        match value {
            schema::FullDerivationSetType::All => Self::All,
            schema::FullDerivationSetType::TypeDerivationControlList(
                type_derivation_control_list,
            ) => Self::TypeDerivationControlList(
                type_derivation_control_list
                    .0
                    .into_iter()
                    .map(Into::into)
                    .collect(),
            ),
        }
    }
}

impl From<schema::BlockSetItemType> for xsd::schema::BlockSetItemType {
    fn from(value: schema::BlockSetItemType) -> Self {
        match value {
            schema::BlockSetItemType::Extension => Self::Extension,
            schema::BlockSetItemType::Restriction => Self::Restriction,
            schema::BlockSetItemType::Substitution => Self::Substitution,
        }
    }
}

impl From<schema::BlockSetType> for xsd::schema::BlockSetType {
    fn from(value: schema::BlockSetType) -> Self {
        match value {
            schema::BlockSetType::All => Self::All,
            schema::BlockSetType::BlockSetItemList(block_set_item_list) => {
                Self::BlockSetItemList(block_set_item_list.0.into_iter().map(Into::into).collect())
            }
        }
    }
}

impl From<schema::FormChoiceType> for xsd::schema::FormChoiceType {
    fn from(value: schema::FormChoiceType) -> Self {
        match value {
            schema::FormChoiceType::Qualified => Self::Qualified,
            schema::FormChoiceType::Unqualified => Self::Unqualified,
        }
    }
}

impl From<schema::QName> for xsd::schema::QNameRef {
    fn from(value: schema::QName) -> Self {
        //TODO: REDO REQUIRES SUPPORT IN XMLITY
        let mut name_parts = value.0.split(":");
        let mut localname = name_parts.next().unwrap();

        if let Some(localname_new) = name_parts.next() {
            localname = localname_new;
        }
        if value.0.starts_with("xs:") {
            Self {
                namespace: Some(Namespace("http://www.w3.org/2001/XMLSchema".to_owned())),
                name: localname.to_owned(),
            }
        } else if value.0.starts_with("xml:") {
            Self {
                namespace: Some(Namespace("http://www.w3.org/XML/1998/namespace".to_owned())),
                name: localname.to_owned(),
            }
        } else {
            todo!()
        }
    }
}

impl From<schema::XpathDefaultNamespaceType> for xsd::schema::XpathDefaultNamespaceType {
    fn from(value: schema::XpathDefaultNamespaceType) -> Self {
        match value {
            schema::XpathDefaultNamespaceType::String(a) => Self::String(a),
            schema::XpathDefaultNamespaceType::DefaultNamespace => Self::DefaultNamespace,
            schema::XpathDefaultNamespaceType::TargetNamespace => Self::TargetNamespace,
            schema::XpathDefaultNamespaceType::Local => Self::Local,
        }
    }
}

impl From<schema::Id> for xsd::schema::Id {
    fn from(value: schema::Id) -> Self {
        Self(value.0)
    }
}

impl From<schema::XmlLang> for xsd::schema::XmlLang {
    fn from(value: schema::XmlLang) -> Self {
        Self(value.0)
    }
}

impl From<schema::SchemaLocation> for xsd::schema::SchemaLocation {
    fn from(value: schema::SchemaLocation) -> Self {
        Self(value.0)
    }
}

impl From<schema::Include> for xsd::schema::Include {
    fn from(value: schema::Include) -> Self {
        Self {
            id: value.id.map(Into::into),
            schema_location: value.schema_location.into(),
            annotation: value.annotation.map(Into::into),
        }
    }
}

impl From<schema::Namespace> for xsd::schema::Namespace {
    fn from(value: schema::Namespace) -> Self {
        Self(value.0)
    }
}

impl From<schema::Import> for xsd::schema::Import {
    fn from(value: schema::Import) -> Self {
        Self {
            id: value.id.map(Into::into),
            namespace: value.namespace.map(Into::into),
            schema_location: value.schema_location.map(Into::into),
            annotation: value.annotation.map(Into::into),
        }
    }
}

impl From<schema::LocalRestriction> for xsd::schema::LocalRestriction {
    fn from(value: schema::LocalRestriction) -> Self {
        Self {
            id: value.id.map(Into::into),
            base: value.base.0.into(),
            annotation: value.annotation.map(Into::into),
            simple_type: value.simple_type.map(Into::into),
            facets: value.facets.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<schema::List> for xsd::schema::List {
    fn from(value: schema::List) -> Self {
        Self {
            id: value.id.map(Into::into),
            annotation: value.annotation.map(Into::into),
            simple_type: value.simple_type.map(Into::into),
            item_type: value.item_type.map(|a| a.0).map(Into::into),
        }
    }
}

impl From<schema::Union> for xsd::schema::Union {
    fn from(value: schema::Union) -> Self {
        Self {
            id: value.id.map(Into::into),
            annotation: value.annotation.map(Into::into),
            simple_types: value.simple_types.into_iter().map(Into::into).collect(),
            member_types: value
                .member_types
                .map(|a| a.0 .0.into_iter().map(Into::into).collect()),
        }
    }
}

impl From<schema::SimpleDerivation> for xsd::schema::SimpleDerivation {
    fn from(value: schema::SimpleDerivation) -> Self {
        match value {
            schema::SimpleDerivation::Restriction(a) => Self::Restriction(Box::new((*a).into())),
            schema::SimpleDerivation::List(a) => Self::List(Box::new((*a).into())),
            schema::SimpleDerivation::Union(a) => Self::Union(Box::new((*a).into())),
        }
    }
}

impl From<schema::SimpleDerivationSetItemType> for xsd::schema::SimpleDerivationSetItemType {
    fn from(value: schema::SimpleDerivationSetItemType) -> Self {
        match value {
            schema::SimpleDerivationSetItemType::Restriction => Self::Restriction,
            schema::SimpleDerivationSetItemType::List => Self::List,
            schema::SimpleDerivationSetItemType::Union => Self::Union,
            schema::SimpleDerivationSetItemType::Extension => Self::Extension,
        }
    }
}

impl From<schema::SimpleDerivationSetType> for xsd::schema::SimpleDerivationSetType {
    fn from(value: schema::SimpleDerivationSetType) -> Self {
        match value {
            schema::SimpleDerivationSetType::All => Self::All,
            schema::SimpleDerivationSetType::SimpleDerivationSetItemList(
                simple_derivation_set_item_list,
            ) => Self::SimpleDerivationSetItemList(
                simple_derivation_set_item_list
                    .0
                    .into_iter()
                    .map(Into::into)
                    .collect(),
            ),
        }
    }
}

impl From<schema::LocalSimpleType> for xsd::schema::LocalSimpleType {
    fn from(value: schema::LocalSimpleType) -> Self {
        Self {
            id: value.id.map(Into::into),
            final_: value.final_.map(|a| a.0).map(Into::into),
            annotation: value.annotation.map(Into::into),
            content: value.content.into(),
        }
    }
}

impl From<schema::ReducedDerivationControlType> for xsd::schema::ReducedDerivationControlType {
    fn from(value: schema::ReducedDerivationControlType) -> Self {
        match value {
            schema::ReducedDerivationControlType::Extension => Self::Extension,
            schema::ReducedDerivationControlType::Restriction => Self::Restriction,
        }
    }
}

impl From<schema::DerivationSetType> for xsd::schema::DerivationSetType {
    fn from(value: schema::DerivationSetType) -> Self {
        match value {
            schema::DerivationSetType::All => Self::All,
            schema::DerivationSetType::ReducedDerivationControlList(reduction_set_item_list) => {
                Self::ReducedDerivationControlList(
                    reduction_set_item_list
                        .0
                        .into_iter()
                        .map(Into::into)
                        .collect(),
                )
            }
        }
    }
}

impl From<schema::OpenContentModeType> for xsd::schema::OpenContentModeType {
    fn from(value: schema::OpenContentModeType) -> Self {
        match value {
            schema::OpenContentModeType::Interleave => Self::Interleave,
            schema::OpenContentModeType::Suffix => Self::Suffix,
            schema::OpenContentModeType::None => Self::None,
        }
    }
}

impl From<schema::BasicNamespaceListItemType> for xsd::schema::BasicNamespaceListItemType {
    fn from(value: schema::BasicNamespaceListItemType) -> Self {
        match value {
            schema::BasicNamespaceListItemType::String(string) => Self::String(string),
            schema::BasicNamespaceListItemType::TargetNamespace => Self::TargetNamespace,
            schema::BasicNamespaceListItemType::Local => Self::Local,
        }
    }
}

impl From<schema::NamespaceListType> for xsd::schema::NamespaceListType {
    fn from(value: schema::NamespaceListType) -> Self {
        match value {
            schema::NamespaceListType::Any => Self::Any,
            schema::NamespaceListType::Other => Self::Other,
            schema::NamespaceListType::BasicNamespaceList(basic_namespace_list_type) => {
                Self::BasicNamespaceList(
                    basic_namespace_list_type
                        .0
                        .into_iter()
                        .map(Into::into)
                        .collect(),
                )
            }
        }
    }
}

impl From<schema::ProcessContentsType> for xsd::schema::ProcessContentsType {
    fn from(value: schema::ProcessContentsType) -> Self {
        match value {
            schema::ProcessContentsType::Skip => Self::Skip,
            schema::ProcessContentsType::Lax => Self::Lax,
            schema::ProcessContentsType::Strict => Self::Strict,
        }
    }
}

impl From<schema::WildcardType> for xsd::schema::WildcardType {
    fn from(value: schema::WildcardType) -> Self {
        Self {
            id: value.id.map(Into::into),
            namespace: value.namespace.map(Into::into),
            not_namespace: value
                .not_namespace
                .map(|a| a.0.into_iter().map(Into::into).collect::<Vec<_>>()),
            process_contents: value.process_contents.into(),
            annotation: value.annotation.map(Into::into),
        }
    }
}

impl From<schema::AnyType> for xsd::schema::AnyType {
    fn from(value: schema::AnyType) -> Self {
        Self {
            wildcard: value.wildcard.into(),
        }
    }
}

impl From<schema::OpenContent> for xsd::schema::OpenContent {
    fn from(value: schema::OpenContent) -> Self {
        Self {
            id: value.id.map(Into::into),
            mode: value.mode.into(),
            annotation: value.annotation.map(Into::into),
            any: value.any.map(Into::into),
        }
    }
}

impl From<schema::MaxOccursValue> for xsd::schema::MaxOccursValue {
    fn from(value: schema::MaxOccursValue) -> Self {
        match value {
            schema::MaxOccursValue::Unbounded => Self::Unbounded,
            schema::MaxOccursValue::Bounded(bounded) => Self::Bounded(bounded),
        }
    }
}

impl From<schema::ChoiceType> for xsd::schema::ChoiceType {
    fn from(value: schema::ChoiceType) -> Self {
        Self {
            id: value.id.map(Into::into),
            name: value.name.map(Into::into),
            ref_: value.ref_.map(|a| a.0).map(Into::into),
            min_occurs: value.min_occurs.map(|a| a.0),
            max_occurs: value.max_occurs.map(|a| a.0).map(Into::into),
            content: value.content.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<schema::SequenceType> for xsd::schema::SequenceType {
    fn from(value: schema::SequenceType) -> Self {
        Self {
            id: value.id.map(Into::into),
            name: value.name.map(Into::into),
            ref_: value.ref_.map(|a| a.0).map(Into::into),
            min_occurs: value.min_occurs.map(|a| a.0),
            max_occurs: value.max_occurs.map(|a| a.0).map(Into::into),
            content: value.content.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<schema::ElementTypeContent> for xsd::schema::ElementTypeContent {
    fn from(value: schema::ElementTypeContent) -> Self {
        match value {
            schema::ElementTypeContent::SimpleType(a) => Self::SimpleType(Box::new((*a).into())),
            schema::ElementTypeContent::ComplexType(a) => Self::ComplexType(Box::new((*a).into())),
        }
    }
}

impl From<schema::AltTypeContent> for xsd::schema::AltTypeContent {
    fn from(value: schema::AltTypeContent) -> Self {
        match value {
            schema::AltTypeContent::Annotation(a) => Self::Annotation(Box::new((*a).into())),
            schema::AltTypeContent::SimpleType(a) => Self::SimpleType(Box::new((*a).into())),
            schema::AltTypeContent::ComplexType(a) => Self::ComplexType(Box::new((*a).into())),
        }
    }
}

impl From<schema::AltType> for xsd::schema::AltType {
    fn from(value: schema::AltType) -> Self {
        Self {
            id: value.id.map(Into::into),
            test: value.test.map(|a| a.0),
            type_: value.type_.map(|a| a.0).map(Into::into),
            xpath_default_namespace: value.xpath_default_namespace.map(|a| a.0).map(Into::into),
            content: value.content.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<schema::XPath> for xsd::schema::XPath {
    fn from(value: schema::XPath) -> Self {
        Self(value.0)
    }
}

impl From<schema::Selector> for xsd::schema::Selector {
    fn from(value: schema::Selector) -> Self {
        Self {
            id: value.id.map(Into::into),
            xpath: value.xpath.into(),
            xpath_default_namespace: value.xpath_default_namespace.map(|a| a.0).map(Into::into),
            annotations: value.annotations.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<schema::Field> for xsd::schema::Field {
    fn from(value: schema::Field) -> Self {
        Self {
            id: value.id.map(Into::into),
            xpath: value.xpath.into(),
            xpath_default_namespace: value.xpath_default_namespace.map(|a| a.0).map(Into::into),
            annotation: value.annotation.map(Into::into),
        }
    }
}

impl From<schema::KeybaseTypeContent> for xsd::schema::KeybaseTypeContent {
    fn from(value: schema::KeybaseTypeContent) -> Self {
        Self {
            annotations: value.annotations.into_iter().map(Into::into).collect(),
            selector: value.selector.into(),
            field: value.field.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<schema::Unique> for xsd::schema::Unique {
    fn from(value: schema::Unique) -> Self {
        Self {
            id: value.id.map(Into::into),
            name: value.name.map(Into::into),
            ref_: value.ref_.map(|a| a.0).map(Into::into),
            content: value.content.map(Into::into),
        }
    }
}

impl From<schema::Key> for xsd::schema::Key {
    fn from(value: schema::Key) -> Self {
        Self {
            id: value.id.map(Into::into),
            name: value.name.map(Into::into),
            ref_: value.ref_.map(|a| a.0).map(Into::into),
            selector: value.selector.into(),
            field: value.field.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<schema::Refer> for xsd::schema::Refer {
    fn from(value: schema::Refer) -> Self {
        Self(value.0.into())
    }
}

impl From<schema::Keyref> for xsd::schema::Keyref {
    fn from(value: schema::Keyref) -> Self {
        Self {
            id: value.id.map(Into::into),
            name: value.name.map(Into::into),
            ref_: value.ref_.map(|a| a.0).map(Into::into),
            refer: value.refer.map(Into::into),
            content: value.content.map(Into::into),
        }
    }
}

impl From<schema::IdentityConstraint> for xsd::schema::IdentityConstraint {
    fn from(value: schema::IdentityConstraint) -> Self {
        match value {
            schema::IdentityConstraint::Unique(a) => Self::Unique(Box::new((*a).into())),
            schema::IdentityConstraint::Key(a) => Self::Key(Box::new((*a).into())),
            schema::IdentityConstraint::Keyref(a) => Self::Keyref(Box::new((*a).into())),
        }
    }
}

impl From<schema::LocalElement> for xsd::schema::LocalElement {
    fn from(value: schema::LocalElement) -> Self {
        Self {
            id: value.id.map(Into::into),
            name: value.name.map(Into::into),
            ref_: value.ref_.map(|a| a.0).map(Into::into),
            type_: value.type_.map(|a| a.0).map(Into::into),
            substitution_group: value
                .substitution_group
                .map(|a| a.0 .0.into_iter().map(Into::into).collect())
                .unwrap_or_default(),
            min_occurs: value.min_occurs.map(|a| a.0),
            max_occurs: value.max_occurs.map(|a| a.0).map(Into::into),
            default: value.default.map(|a| a.0),
            fixed: value.fixed.map(|a| a.0),
            nillable: value.nillable.map(|a| a.0),
            abstract_: value.abstract_.map(|a| a.0),
            final_: value.final_.map(|a| a.0).map(Into::into),
            block: value.block.map(|a| a.0).map(Into::into),
            form: value.form.map(|a| a.0).map(Into::into),
            target_namespace: value.target_namespace.map(|a| a.0),
            annotation: value.annotation.map(Into::into),
            type_choice: value.type_choice.map(Into::into),
            alternatives: value.alternatives.into_iter().map(Into::into).collect(),
            identity_constraints: value
                .identity_constraints
                .into_iter()
                .map(Into::into)
                .collect(),
        }
    }
}

impl From<schema::QnameListItemType> for xsd::schema::QnameListItemType {
    fn from(value: schema::QnameListItemType) -> Self {
        match value {
            schema::QnameListItemType::Qname(qname) => Self::Qname(qname.into()),
            schema::QnameListItemType::Defined => Self::Defined,
            schema::QnameListItemType::DefinedSibling => Self::DefinedSibling,
        }
    }
}

impl From<schema::Any> for xsd::schema::Any {
    fn from(value: schema::Any) -> Self {
        Self {
            id: value.id.map(Into::into),
            namespace: value.namespace.map(|a| a.0).map(Into::into),
            not_namespace: value.not_namespace.map(|a| a.0).map(Into::into),
            process_contents: value.process_contents.0.into(),
            not_q_name: value
                .not_q_name
                .map(|a| a.0 .0.into_iter().map(Into::into).collect()),
            min_occurs: value.min_occurs.map(|a| a.0),
            max_occurs: value.max_occurs.map(|a| a.0).map(Into::into),
            annotation: value.annotation.map(Into::into),
        }
    }
}

impl From<schema::GroupTypeContent> for xsd::schema::GroupTypeContent {
    fn from(value: schema::GroupTypeContent) -> Self {
        match value {
            schema::GroupTypeContent::All(all) => Self::All(Box::new((*all).into())),
            schema::GroupTypeContent::Choice(choice) => Self::Choice(Box::new((*choice).into())),
            schema::GroupTypeContent::Sequence(sequence) => {
                Self::Sequence(Box::new((*sequence).into()))
            }
            schema::GroupTypeContent::Element(local_element) => {
                Self::Element(Box::new((*local_element).into()))
            }
            schema::GroupTypeContent::Group(group_type) => {
                Self::Group(Box::new((*group_type).into()))
            }
            schema::GroupTypeContent::Any(any) => Self::Any(Box::new((*any).into())),
        }
    }
}

impl From<schema::AllType> for xsd::schema::AllType {
    fn from(value: schema::AllType) -> Self {
        Self {
            id: value.id.map(Into::into),
            name: value.name.map(Into::into),
            ref_: value.ref_.map(|a| a.0).map(Into::into),
            min_occurs: value.min_occurs.map(|a| a.0),
            max_occurs: value.max_occurs.map(|a| a.0).map(Into::into),
            content: value.content.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<schema::FacetType> for xsd::schema::FacetType {
    fn from(value: schema::FacetType) -> Self {
        Self {
            fixed: value.fixed.map(|a| a.0),
            annotation: value.annotation.map(Into::into),
            value: value.value.0,
        }
    }
}

impl From<schema::MinExclusive> for xsd::schema::MinExclusive {
    fn from(value: schema::MinExclusive) -> Self {
        Self {
            facet_type: value.facet_type.into(),
        }
    }
}

impl From<schema::MinInclusive> for xsd::schema::MinInclusive {
    fn from(value: schema::MinInclusive) -> Self {
        Self {
            facet_type: value.facet_type.into(),
        }
    }
}

impl From<schema::MaxExclusive> for xsd::schema::MaxExclusive {
    fn from(value: schema::MaxExclusive) -> Self {
        Self {
            facet_type: value.facet_type.into(),
        }
    }
}

impl From<schema::MaxInclusive> for xsd::schema::MaxInclusive {
    fn from(value: schema::MaxInclusive) -> Self {
        Self {
            facet_type: value.facet_type.into(),
        }
    }
}

impl From<schema::Enumeration> for xsd::schema::Enumeration {
    fn from(value: schema::Enumeration) -> Self {
        Self {
            fixed: value.fixed.map(|a| a.0),
            value: value.value.0,
        }
    }
}

impl From<schema::Facet> for xsd::schema::Facet {
    fn from(value: schema::Facet) -> Self {
        match value {
            schema::Facet::MinExclusive(a) => Self::MinExclusive(Box::new((*a).into())),
            schema::Facet::MinInclusive(a) => Self::MinInclusive(Box::new((*a).into())),
            schema::Facet::MaxExclusive(a) => Self::MaxExclusive(Box::new((*a).into())),
            schema::Facet::MaxInclusive(a) => Self::MaxInclusive(Box::new((*a).into())),
            schema::Facet::Enumeration(a) => Self::Enumeration(Box::new((*a).into())),
        }
    }
}

impl From<schema::AttributeUseType> for xsd::schema::AttributeUseType {
    fn from(value: schema::AttributeUseType) -> Self {
        match value {
            schema::AttributeUseType::Prohibited => Self::Prohibited,
            schema::AttributeUseType::Optional => Self::Optional,
            schema::AttributeUseType::Required => Self::Required,
        }
    }
}

impl From<schema::LocalAttribute> for xsd::schema::LocalAttribute {
    fn from(value: schema::LocalAttribute) -> Self {
        Self {
            id: value.id.map(Into::into),
            name: value.name.map(Into::into),
            ref_: value.ref_.map(|a| a.0).map(Into::into),
            type_: value.type_.map(|a| a.0).map(Into::into),
            use_: value.use_.map(|a| a.0).map(Into::into),
            default: value.default.map(|a| a.0),
            fixed: value.fixed.map(|a| a.0),
            form: value.form.map(|a| a.0).map(Into::into),
            target_namespace: value.target_namespace.map(|a| a.0),
            inheritable: value.inheritable.map(|a| a.0),
            annotation: value.annotation.map(Into::into),
            simple_type: value.simple_type.map(Into::into),
        }
    }
}

impl From<schema::QnameListAItemType> for xsd::schema::QnameListAItemType {
    fn from(value: schema::QnameListAItemType) -> Self {
        match value {
            schema::QnameListAItemType::Qname(qname) => Self::Qname(qname.into()),
            schema::QnameListAItemType::Defined => Self::Defined,
        }
    }
}

impl From<schema::AnyAttribute> for xsd::schema::AnyAttribute {
    fn from(value: schema::AnyAttribute) -> Self {
        Self {
            id: value.id.map(Into::into),
            namespace: value.namespace.map(|a| a.0).map(Into::into),
            not_namespace: value.not_namespace.map(|a| a.0).map(Into::into),
            process_contents: value.process_contents.0.into(),
            not_q_name: value
                .not_q_name
                .map(|a| a.0 .0.into_iter().map(Into::into).collect()),
            annotation: value.annotation.map(Into::into),
        }
    }
}

impl From<schema::AssertionType> for xsd::schema::AssertionType {
    fn from(value: schema::AssertionType) -> Self {
        Self {
            id: value.id.map(Into::into),
            test: value.test.map(|a| a.0),
            xpath_default_namespace: value.xpath_default_namespace.map(|a| a.0).map(Into::into),
            annotation: value.annotation.map(Into::into),
        }
    }
}

impl From<schema::RestrictionTypeContent> for xsd::schema::RestrictionTypeContent {
    fn from(value: schema::RestrictionTypeContent) -> Self {
        match value {
            schema::RestrictionTypeContent::SimpleType(simple_type) => {
                Self::SimpleType(Box::new((*simple_type).into()))
            }
            schema::RestrictionTypeContent::Annotation(annotation) => {
                Self::Annotation(Box::new((*annotation).into()))
            }
            schema::RestrictionTypeContent::OpenContent(open_content) => {
                Self::OpenContent(Box::new((*open_content).into()))
            }
            schema::RestrictionTypeContent::Group(group_type) => {
                Self::Group(Box::new((*group_type).into()))
            }
            schema::RestrictionTypeContent::All(all_type) => {
                Self::All(Box::new((*all_type).into()))
            }
            schema::RestrictionTypeContent::Choice(choice_type) => {
                Self::Choice(Box::new((*choice_type).into()))
            }
            schema::RestrictionTypeContent::Sequence(sequence_type) => {
                Self::Sequence(Box::new((*sequence_type).into()))
            }
            schema::RestrictionTypeContent::Facet(facet) => Self::Facet(Box::new((*facet).into())),
            schema::RestrictionTypeContent::Attribute(local_attribute) => {
                Self::Attribute(Box::new((*local_attribute).into()))
            }
            schema::RestrictionTypeContent::AttributeGroup(attribute_group_type) => {
                Self::AttributeGroup(Box::new((*attribute_group_type).into()))
            }
            schema::RestrictionTypeContent::AnyAttribute(any_attribute) => {
                Self::AnyAttribute(Box::new((*any_attribute).into()))
            }
            schema::RestrictionTypeContent::Assert(assertion_type) => {
                Self::Assert(Box::new((*assertion_type).into()))
            }
        }
    }
}

impl From<schema::Restriction> for xsd::schema::Restriction {
    fn from(value: schema::Restriction) -> Self {
        Self {
            id: value.id.map(Into::into),
            base: value.base.0.into(),
            content: value.content.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<schema::ExtensionTypeContent> for xsd::schema::ExtensionTypeContent {
    fn from(value: schema::ExtensionTypeContent) -> Self {
        use schema::ExtensionTypeContent as Variant;
        match value {
            Variant::Annotation(annotation) => Self::Annotation(Box::new((*annotation).into())),
            Variant::OpenContent(open_content) => {
                Self::OpenContent(Box::new((*open_content).into()))
            }
            Variant::Group(group_type) => Self::Group(Box::new((*group_type).into())),
            Variant::All(all_type) => Self::All(Box::new((*all_type).into())),
            Variant::Choice(choice_type) => Self::Choice(Box::new((*choice_type).into())),
            Variant::Sequence(sequence_type) => Self::Sequence(Box::new((*sequence_type).into())),
            Variant::Attribute(local_attribute) => {
                Self::Attribute(Box::new((*local_attribute).into()))
            }
            Variant::AttributeGroup(attribute_group_type) => {
                Self::AttributeGroup(Box::new((*attribute_group_type).into()))
            }
            Variant::AnyAttribute(any_attribute) => {
                Self::AnyAttribute(Box::new((*any_attribute).into()))
            }
            Variant::Assert(assertion_type) => Self::Assert(Box::new((*assertion_type).into())),
        }
    }
}

impl From<schema::ExtensionType> for xsd::schema::ExtensionType {
    fn from(value: schema::ExtensionType) -> Self {
        Self {
            id: value.id.map(Into::into),
            base: value.base.0.into(),
            content: value.content.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<schema::SimpleContentContent> for xsd::schema::SimpleContentContent {
    fn from(value: schema::SimpleContentContent) -> Self {
        match value {
            schema::SimpleContentContent::Restriction(restriction) => {
                Self::Restriction(Box::new((*restriction).into()))
            }
            schema::SimpleContentContent::Extension(extension) => {
                Self::Extension(Box::new((*extension).into()))
            }
            schema::SimpleContentContent::Annotation(annotation) => {
                Self::Annotation(Box::new((*annotation).into()))
            }
        }
    }
}

impl From<schema::SimpleContent> for xsd::schema::SimpleContent {
    fn from(value: schema::SimpleContent) -> Self {
        Self {
            id: value.id.map(Into::into),
            content: value.content.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<schema::RestrictionContent> for xsd::schema::RestrictionContent {
    fn from(value: schema::RestrictionContent) -> Self {
        match value {
            schema::RestrictionContent::Other(xml_value) => {
                //TODO
                Self::Other(xml_value)
            }
        }
    }
}

impl From<schema::ComplexRestrictionType> for xsd::schema::ComplexRestrictionType {
    fn from(value: schema::ComplexRestrictionType) -> Self {
        Self {
            id: value.id.map(Into::into),
            annotation: value.annotation.map(Into::into),
            base: value.base.map(|a| a.0).map(Into::into),
            simple_type: value.simple_type.map(Into::into),
            content: value.content.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<schema::ComplexContentContent> for xsd::schema::ComplexContentContent {
    fn from(value: schema::ComplexContentContent) -> Self {
        match value {
            schema::ComplexContentContent::Annotation(annotation) => {
                Self::Annotation(Box::new((*annotation).into()))
            }
            schema::ComplexContentContent::Restriction(complex_restriction_type) => {
                Self::Restriction(Box::new((*complex_restriction_type).into()))
            }
            schema::ComplexContentContent::Extension(extension_type) => {
                Self::Extension(Box::new((*extension_type).into()))
            }
        }
    }
}

impl From<schema::ComplexContent> for xsd::schema::ComplexContent {
    fn from(value: schema::ComplexContent) -> Self {
        Self {
            id: value.id.map(Into::into),
            content: value.content.into_iter().map(Into::into).collect(),
            mixed: value.mixed.map(|a| a.0),
        }
    }
}

impl From<schema::ComplexBaseTypeContent> for xsd::schema::ComplexBaseTypeContent {
    fn from(value: schema::ComplexBaseTypeContent) -> Self {
        match value {
            schema::ComplexBaseTypeContent::SimpleContent(simple_content) => {
                Self::SimpleContent(simple_content.into())
            }
            schema::ComplexBaseTypeContent::ComplexContent(complex_content) => {
                Self::ComplexContent(complex_content.into())
            }
            schema::ComplexBaseTypeContent::OpenContent(open_content) => {
                Self::OpenContent(open_content.into())
            }
            schema::ComplexBaseTypeContent::Group(group_type) => Self::Group(group_type.into()),
            schema::ComplexBaseTypeContent::All(all_type) => Self::All(all_type.into()),
            schema::ComplexBaseTypeContent::Choice(choice_type) => Self::Choice(choice_type.into()),
            schema::ComplexBaseTypeContent::Sequence(sequence_type) => {
                Self::Sequence(sequence_type.into())
            }
            schema::ComplexBaseTypeContent::Attribute(local_attribute) => {
                Self::Attribute(local_attribute.into())
            }
            schema::ComplexBaseTypeContent::AttributeGroup(attribute_group_type) => {
                Self::AttributeGroup(attribute_group_type.into())
            }
            schema::ComplexBaseTypeContent::AnyAttribute(any_attribute) => {
                Self::AnyAttribute(any_attribute.into())
            }
            schema::ComplexBaseTypeContent::Assert(assertion_type) => {
                Self::Assert(assertion_type.into())
            }
        }
    }
}

impl From<schema::LocalComplexType> for xsd::schema::LocalComplexType {
    fn from(value: schema::LocalComplexType) -> Self {
        Self {
            id: value.id.map(Into::into),
            mixed: value.mixed.map(|a| a.0),
            abstract_: value.abstract_.map(|a| a.0),
            final_: value.final_.map(|a| a.0).map(Into::into),
            block: value.block.map(|a| a.0).map(Into::into),
            default_attributes_apply: value.default_attributes_apply.map(|a| a.0),
            annotation: value.annotation.map(Into::into),
            content: value.content.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<schema::RedefineContent> for xsd::schema::RedefineContent {
    fn from(value: schema::RedefineContent) -> Self {
        match value {
            schema::RedefineContent::SimpleType(simple_type) => {
                Self::SimpleType(simple_type.into())
            }
            schema::RedefineContent::ComplexType(complex_type) => {
                Self::ComplexType(complex_type.into())
            }
            schema::RedefineContent::Group(group) => Self::Group(group.into()),
            schema::RedefineContent::AttributeGroup(attribute_group) => {
                Self::AttributeGroup(attribute_group.into())
            }
            schema::RedefineContent::Annotation(annotation) => Self::Annotation(annotation.into()),
        }
    }
}

impl From<schema::Redefine> for xsd::schema::Redefine {
    fn from(value: schema::Redefine) -> Self {
        Self {
            schema_location: value.schema_location,
            id: value.id.map(Into::into),
            content: value.content.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<schema::OverrideContent> for xsd::schema::OverrideContent {
    fn from(value: schema::OverrideContent) -> Self {
        match value {
            schema::OverrideContent::Annotation(a) => Self::Annotation(a.into()),
            schema::OverrideContent::SimpleType(a) => Self::SimpleType(a.into()),
            schema::OverrideContent::ComplexType(a) => Self::ComplexType(a.into()),
            schema::OverrideContent::Group(a) => Self::Group(a.into()),
            schema::OverrideContent::AttributeGroup(a) => Self::AttributeGroup(a.into()),
            schema::OverrideContent::Element(a) => Self::Element(a.into()),
            schema::OverrideContent::Attribute(a) => Self::Attribute(a.into()),
            schema::OverrideContent::Notation(a) => Self::Notation(a.into()),
        }
    }
}

impl From<schema::Override> for xsd::schema::Override {
    fn from(value: schema::Override) -> Self {
        Self {
            schema_location: value.schema_location,
            id: value.id.map(Into::into),
            content: value.content.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<schema::Source> for xsd::schema::Source {
    fn from(value: schema::Source) -> Self {
        Self(value.0)
    }
}

impl From<schema::Appinfo> for xsd::schema::Appinfo {
    fn from(value: schema::Appinfo) -> Self {
        Self {
            source: value.source.map(Into::into),
        }
    }
}

impl From<schema::Documentation> for xsd::schema::Documentation {
    fn from(value: schema::Documentation) -> Self {
        Self {
            source: value.source.map(Into::into),
            lang: value.lang.map(Into::into),
            //TODO
            any: value.any,
        }
    }
}

impl From<schema::AnnotationContent> for xsd::schema::AnnotationContent {
    fn from(value: schema::AnnotationContent) -> Self {
        match value {
            schema::AnnotationContent::Appinfo(appinfo) => Self::Appinfo(appinfo.into()),
            schema::AnnotationContent::Documentation(documentation) => {
                Self::Documentation(documentation.into())
            }
        }
    }
}

impl From<schema::Annotation> for xsd::schema::Annotation {
    fn from(value: schema::Annotation) -> Self {
        Self {
            id: value.id.map(Into::into),
            content: value.content.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<schema::Composition> for xsd::schema::Composition {
    fn from(value: schema::Composition) -> Self {
        match value {
            schema::Composition::Include(include) => Self::Include(include.into()),
            schema::Composition::Import(import) => Self::Import(import.into()),
            schema::Composition::Redefine(redefine) => Self::Redefine(redefine.into()),
            schema::Composition::Override(override_) => Self::Override(override_.into()),
            schema::Composition::Annotation(annotation) => Self::Annotation(annotation.into()),
        }
    }
}

impl From<schema::DefaultOpenContentModeType> for xsd::schema::DefaultOpenContentModeType {
    fn from(value: schema::DefaultOpenContentModeType) -> Self {
        match value {
            schema::DefaultOpenContentModeType::Interleave => Self::Interleave,
            schema::DefaultOpenContentModeType::Suffix => Self::Suffix,
        }
    }
}

impl From<schema::DefaultOpenContent> for xsd::schema::DefaultOpenContent {
    fn from(value: schema::DefaultOpenContent) -> Self {
        Self {
            id: value.id.map(Into::into),
            applies_to_empty: value.applies_to_empty,
            mode: value.mode.into(),
            annotation: value.annotation.map(Into::into),
            any: value.any.into(),
        }
    }
}

impl From<schema::TopLevelElement> for xsd::schema::TopLevelElement {
    fn from(value: schema::TopLevelElement) -> Self {
        Self {
            id: value.id.map(Into::into),
            name: value.name.map(Into::into),
            type_: value.type_.map(|a| a.0).map(Into::into),
            substitution_group: value
                .substitution_group
                .map(|a| a.0 .0.into_iter().map(Into::into).collect())
                .unwrap_or_default(),
            min_occurs: value.min_occurs.map(|a| a.0),
            max_occurs: value.max_occurs.map(|a| a.0).map(Into::into),
            default: value.default.map(|a| a.0),
            fixed: value.fixed.map(|a| a.0),
            nillable: value.nillable.map(|a| a.0),
            abstract_: value.abstract_.map(|a| a.0),
            final_: value.final_.map(|a| a.0).map(Into::into),
            block: value.block.map(|a| a.0).map(Into::into),
            form: value.form.map(|a| a.0).map(Into::into),
            target_namespace: value.target_namespace.map(|a| a.0),
            annotation: value.annotation.map(Into::into),
            type_choice: value.type_choice.map(Into::into),
            alternatives: value.alternatives.into_iter().map(Into::into).collect(),
            identity_constraints: value
                .identity_constraints
                .into_iter()
                .map(Into::into)
                .collect(),
        }
    }
}

impl From<schema::TopLevelAttribute> for xsd::schema::TopLevelAttribute {
    fn from(value: schema::TopLevelAttribute) -> Self {
        Self {
            id: value.id.map(Into::into),
            name: value.name.map(Into::into),
            type_: value.type_.map(|a| a.0).map(Into::into),
            use_: value.use_.map(|a| a.0).map(Into::into),
            default: value.default.map(|a| a.0),
            fixed: value.fixed.map(|a| a.0),
            form: value.form.map(|a| a.0).map(Into::into),
            target_namespace: value.target_namespace.map(|a| a.0),
            inheritable: value.inheritable.map(|a| a.0),
            annotation: value.annotation.map(Into::into),
            simple_type: value.simple_type.map(Into::into),
        }
    }
}

impl From<schema::TopLevelSimpleType> for xsd::schema::TopLevelSimpleType {
    fn from(value: schema::TopLevelSimpleType) -> Self {
        Self {
            id: value.id.map(Into::into),
            name: value.name.map(Into::into),
            final_: value.final_.map(|a| a.0).map(Into::into),
            annotation: value.annotation.map(Into::into),
            content: value.content.into(),
        }
    }
}

impl From<schema::TopLevelComplexType> for xsd::schema::TopLevelComplexType {
    fn from(value: schema::TopLevelComplexType) -> Self {
        Self {
            id: value.id.map(Into::into),
            name: value.name.map(Into::into),
            mixed: value.mixed.map(|a| a.0),
            abstract_: value.abstract_.map(|a| a.0),
            final_: value.final_.map(|a| a.0).map(Into::into),
            block: value.block.map(|a| a.0).map(Into::into),
            default_attributes_apply: value.default_attributes_apply.map(|a| a.0),
            annotation: value.annotation.map(Into::into),
            content: value.content.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<schema::GroupType> for xsd::schema::GroupType {
    fn from(value: schema::GroupType) -> Self {
        Self {
            id: value.id.map(Into::into),
            name: value.name.map(Into::into),
            ref_: value.ref_.map(|a| a.0).map(Into::into),
            min_occurs: value.min_occurs.map(|a| a.0),
            max_occurs: value.max_occurs.map(|a| a.0).map(Into::into),
            annotation: value.annotation.map(Into::into),
            content: value.content.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<schema::AttributeGroupTypeContent> for xsd::schema::AttributeGroupTypeContent {
    fn from(value: schema::AttributeGroupTypeContent) -> Self {
        use schema::AttributeGroupTypeContent as S;
        match value {
            S::Attribute(a) => Self::Attribute(Box::new((*a).into())),
            S::AttributeGroup(a) => Self::AttributeGroup(Box::new((*a).into())),
            S::AnyAttribute(a) => Self::AnyAttribute(Box::new((*a).into())),
        }
    }
}

impl From<schema::AttributeGroupType> for xsd::schema::AttributeGroupType {
    fn from(value: schema::AttributeGroupType) -> Self {
        Self {
            id: value.id.map(Into::into),
            name: value.name.map(Into::into),
            ref_: value.ref_.map(|a| a.0).map(Into::into),
            annotation: value.annotation.map(Into::into),
            content: value.content.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<schema::Redefineable> for xsd::schema::Redefineable {
    fn from(value: schema::Redefineable) -> Self {
        match value {
            schema::Redefineable::SimpleType(top_level_simple_type) => {
                Self::SimpleType(top_level_simple_type.into())
            }
            schema::Redefineable::ComplexType(top_level_complex_type) => {
                Self::ComplexType(top_level_complex_type.into())
            }
            schema::Redefineable::Group(group_type) => Self::Group(group_type.into()),
            schema::Redefineable::AttributeGroup(attribute_group_type) => {
                Self::AttributeGroup(attribute_group_type.into())
            }
        }
    }
}

impl From<schema::Name> for xsd::schema::Name {
    fn from(value: schema::Name) -> Self {
        Self(value.0)
    }
}

impl From<schema::Notation> for xsd::schema::Notation {
    fn from(value: schema::Notation) -> Self {
        Self {
            id: value.id.map(Into::into),
            name: value.name.into(),
            public: value.public.map(|a| a.0),
            system: value.system.map(|a| a.0),
            annotation: value.annotation.map(Into::into),
        }
    }
}

impl From<schema::SchemaTop> for xsd::schema::SchemaTop {
    fn from(value: schema::SchemaTop) -> Self {
        match value {
            schema::SchemaTop::Element(element) => Self::Element(element.into()),
            schema::SchemaTop::Attribute(attribute) => Self::Attribute(attribute.into()),
            schema::SchemaTop::Redefineable(redefineable) => {
                Self::Redefineable(redefineable.into())
            }
            schema::SchemaTop::Annotation(annotation) => Self::Annotation(annotation.into()),
            schema::SchemaTop::Notation(notation) => Self::Notation(notation.into()),
        }
    }
}
