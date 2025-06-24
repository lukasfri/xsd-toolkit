pub mod types {
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::bon::Builder,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub struct All {
        #[xattribute(name = "id", optional, default)]
        pub id: ::core::option::Option<String>,
        pub all_model: ::std::boxed::Box<crate::xs::groups::AllModel>,
    }
    pub mod alt_type_items {
        impl ::core::convert::From<crate::xs::types::LocalSimpleType> for Type {
            fn from(value: crate::xs::types::LocalSimpleType) -> Self {
                Type::SimpleType(::std::boxed::Box::new(value))
            }
        }
        impl ::core::convert::From<crate::xs::types::LocalComplexType> for Type {
            fn from(value: crate::xs::types::LocalComplexType) -> Self {
                Type::ComplexType(::std::boxed::Box::new(value))
            }
        }
        #[derive(
            ::core::fmt::Debug,
            ::xmlity::Serialize,
            ::xmlity::Deserialize,
            ::core::cmp::PartialEq,
            ::core::clone::Clone
        )]
        pub enum Type {
            #[xelement(
                name = "simpleType",
                namespace = "http://www.w3.org/2001/XMLSchema",
                allow_unknown_attributes = "any"
            )]
            SimpleType(#[xgroup] ::std::boxed::Box<crate::xs::types::LocalSimpleType>),
            #[xelement(
                name = "complexType",
                namespace = "http://www.w3.org/2001/XMLSchema",
                allow_unknown_attributes = "any"
            )]
            ComplexType(#[xgroup] ::std::boxed::Box<crate::xs::types::LocalComplexType>),
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::bon::Builder,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xgroup(children_order = "strict")]
    pub struct AltType {
        #[xattribute(name = "id", optional, default)]
        pub id: ::core::option::Option<String>,
        #[xattribute(name = "test", optional, default)]
        pub test: ::core::option::Option<String>,
        #[xattribute(name = "type", optional, default)]
        pub type_attribute: ::core::option::Option<crate::xs::types::QName>,
        #[xattribute(name = "xpathDefaultNamespace", optional, default)]
        pub xpath_default_namespace: ::core::option::Option<
            crate::xs::types::XpathDefaultNamespaceType,
        >,
        #[xvalue(default)]
        pub annotation: ::core::option::Option<::std::boxed::Box<crate::xs::Annotation>>,
        #[xvalue(default)]
        pub type_: ::core::option::Option<alt_type_items::Type>,
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::bon::Builder,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xgroup(children_order = "strict")]
    pub struct Annotated {
        #[xattribute(name = "id", optional, default)]
        pub id: ::core::option::Option<String>,
        #[xvalue(default)]
        pub annotation: ::core::option::Option<::std::boxed::Box<crate::xs::Annotation>>,
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::bon::Builder,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xgroup(children_order = "strict")]
    pub struct AnyType {
        pub child_0: ::xmlity::XmlValue,
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::bon::Builder,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xgroup(children_order = "strict")]
    pub struct Assertion {
        #[xattribute(name = "id", optional, default)]
        pub id: ::core::option::Option<String>,
        #[xattribute(name = "test", optional, default)]
        pub test: ::core::option::Option<String>,
        #[xattribute(name = "xpathDefaultNamespace", optional, default)]
        pub xpath_default_namespace: ::core::option::Option<
            crate::xs::types::XpathDefaultNamespaceType,
        >,
        #[xvalue(default)]
        pub annotation: ::core::option::Option<::std::boxed::Box<crate::xs::Annotation>>,
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::bon::Builder,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xgroup(children_order = "strict")]
    pub struct Attribute {
        #[xattribute(name = "id", optional, default)]
        pub id: ::core::option::Option<String>,
        #[xattribute(name = "name", optional, default)]
        pub name: ::core::option::Option<::xmlity::LocalName<'static>>,
        #[xattribute(name = "ref", optional, default)]
        pub ref_: ::core::option::Option<crate::xs::types::QName>,
        #[xattribute(name = "type", optional, default)]
        pub type_: ::core::option::Option<crate::xs::types::QName>,
        #[xattribute(name = "use", optional, default)]
        pub use_: ::core::option::Option<String>,
        #[xattribute(name = "default", optional, default)]
        pub default: ::core::option::Option<String>,
        #[xattribute(name = "fixed", optional, default)]
        pub fixed: ::core::option::Option<String>,
        #[xattribute(name = "form", optional, default)]
        pub form: ::core::option::Option<crate::xs::types::FormChoiceType>,
        #[xattribute(name = "targetNamespace", optional, default)]
        pub target_namespace: ::core::option::Option<crate::xs::types::TargetNamespace>,
        #[xattribute(name = "inheritable", optional, default)]
        pub inheritable: ::core::option::Option<bool>,
        #[xvalue(default)]
        pub annotation: ::core::option::Option<::std::boxed::Box<crate::xs::Annotation>>,
        #[xelement(
            name = "simpleType",
            namespace = "http://www.w3.org/2001/XMLSchema",
            group,
            optional
        )]
        pub simple_type: ::core::option::Option<
            ::std::boxed::Box<crate::xs::types::LocalSimpleType>,
        >,
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::bon::Builder,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xgroup(children_order = "strict")]
    pub struct AttributeGroup {
        #[xattribute(name = "id", optional, default)]
        pub id: ::core::option::Option<String>,
        #[xattribute(name = "name", optional, default)]
        pub name: ::core::option::Option<::xmlity::LocalName<'static>>,
        #[xattribute(name = "ref", optional, default)]
        pub ref_: ::core::option::Option<crate::xs::types::QName>,
        #[xvalue(default)]
        pub annotation: ::core::option::Option<::std::boxed::Box<crate::xs::Annotation>>,
        pub attr_decls: ::std::boxed::Box<crate::xs::groups::AttrDecls>,
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::bon::Builder,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xgroup(children_order = "strict")]
    pub struct AttributeGroupRef {
        #[xattribute(name = "id", optional, default)]
        pub id: ::core::option::Option<String>,
        #[xattribute(name = "ref")]
        pub ref_: crate::xs::types::QName,
        #[xvalue(default)]
        pub annotation: ::core::option::Option<::std::boxed::Box<crate::xs::Annotation>>,
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::bon::Builder,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xgroup(children_order = "strict")]
    pub struct ComplexBaseType {
        #[xattribute(name = "id", optional, default)]
        pub id: ::core::option::Option<String>,
        #[xattribute(name = "name", optional, default)]
        pub name: ::core::option::Option<::xmlity::LocalName<'static>>,
        #[xattribute(name = "mixed", optional, default)]
        pub mixed: ::core::option::Option<bool>,
        #[xattribute(name = "abstract", optional, default)]
        pub abstract_: ::core::option::Option<bool>,
        #[xattribute(name = "final", optional, default)]
        pub final_: ::core::option::Option<crate::xs::types::DerivationSetType>,
        #[xattribute(name = "block", optional, default)]
        pub block: ::core::option::Option<crate::xs::types::DerivationSetType>,
        #[xattribute(name = "defaultAttributesApply", optional, default)]
        pub default_attributes_apply: ::core::option::Option<bool>,
        #[xvalue(default)]
        pub annotation: ::core::option::Option<::std::boxed::Box<crate::xs::Annotation>>,
        pub complex_type_model: ::std::boxed::Box<crate::xs::groups::ComplexTypeModel>,
    }
    pub mod complex_restriction_type_items {
        pub mod variant_0_variants {
            #[derive(
                ::core::fmt::Debug,
                ::xmlity::Serialize,
                ::xmlity::Deserialize,
                ::bon::Builder,
                ::core::cmp::PartialEq,
                ::core::clone::Clone
            )]
            #[xvalue(order = "strict")]
            pub struct Variant0 {
                #[xvalue(default)]
                pub open_content: ::core::option::Option<
                    ::std::boxed::Box<crate::xs::OpenContent>,
                >,
                pub type_def_particle: ::std::boxed::Box<
                    crate::xs::groups::TypeDefParticle,
                >,
            }
        }
        impl ::core::convert::From<variant_0_variants::Variant0> for Variant0 {
            fn from(value: variant_0_variants::Variant0) -> Self {
                Variant0::Variant0(::std::boxed::Box::new(value))
            }
        }
        #[derive(
            ::core::fmt::Debug,
            ::xmlity::Serialize,
            ::xmlity::Deserialize,
            ::core::cmp::PartialEq,
            ::core::clone::Clone
        )]
        pub enum Variant0 {
            Variant0(::std::boxed::Box<variant_0_variants::Variant0>),
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::bon::Builder,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xgroup(children_order = "strict")]
    pub struct ComplexRestrictionType {
        #[xattribute(name = "id", optional, default)]
        pub id: ::core::option::Option<String>,
        #[xattribute(name = "base")]
        pub base: crate::xs::types::QName,
        #[xvalue(default)]
        pub annotation: ::core::option::Option<::std::boxed::Box<crate::xs::Annotation>>,
        #[xvalue(default)]
        pub variant_0: ::core::option::Option<complex_restriction_type_items::Variant0>,
        pub attr_decls: ::std::boxed::Box<crate::xs::groups::AttrDecls>,
        pub assertions: ::std::boxed::Box<crate::xs::groups::Assertions>,
    }
    pub mod element_items {
        impl ::core::convert::From<crate::xs::types::LocalSimpleType> for Type {
            fn from(value: crate::xs::types::LocalSimpleType) -> Self {
                Type::SimpleType(::std::boxed::Box::new(value))
            }
        }
        impl ::core::convert::From<crate::xs::types::LocalComplexType> for Type {
            fn from(value: crate::xs::types::LocalComplexType) -> Self {
                Type::ComplexType(::std::boxed::Box::new(value))
            }
        }
        #[derive(
            ::core::fmt::Debug,
            ::xmlity::Serialize,
            ::xmlity::Deserialize,
            ::core::cmp::PartialEq,
            ::core::clone::Clone
        )]
        pub enum Type {
            #[xelement(
                name = "simpleType",
                namespace = "http://www.w3.org/2001/XMLSchema",
                allow_unknown_attributes = "any"
            )]
            SimpleType(#[xgroup] ::std::boxed::Box<crate::xs::types::LocalSimpleType>),
            #[xelement(
                name = "complexType",
                namespace = "http://www.w3.org/2001/XMLSchema",
                allow_unknown_attributes = "any"
            )]
            ComplexType(#[xgroup] ::std::boxed::Box<crate::xs::types::LocalComplexType>),
        }
        impl ::core::convert::From<crate::xs::types::AltType> for Alternative {
            fn from(value: crate::xs::types::AltType) -> Self {
                Alternative(::std::boxed::Box::new(value))
            }
        }
        #[derive(
            ::core::fmt::Debug,
            ::xmlity::Serialize,
            ::xmlity::Deserialize,
            ::core::cmp::PartialEq,
            ::core::clone::Clone
        )]
        #[xelement(
            name = "alternative",
            namespace = "http://www.w3.org/2001/XMLSchema",
            allow_unknown_attributes = "any"
        )]
        pub struct Alternative(
            #[xgroup]
            pub ::std::boxed::Box<crate::xs::types::AltType>,
        );
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::bon::Builder,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xgroup(children_order = "strict")]
    pub struct Element {
        #[xattribute(name = "id", optional, default)]
        pub id: ::core::option::Option<String>,
        #[xattribute(name = "name", optional, default)]
        pub name: ::core::option::Option<::xmlity::LocalName<'static>>,
        #[xattribute(name = "ref", optional, default)]
        pub ref_: ::core::option::Option<crate::xs::types::QName>,
        #[xattribute(name = "type", optional, default)]
        pub type_attribute: ::core::option::Option<crate::xs::types::QName>,
        #[xattribute(name = "substitutionGroup", optional, default)]
        pub substitution_group: ::core::option::Option<String>,
        #[xattribute(name = "minOccurs", optional, default)]
        pub min_occurs: ::core::option::Option<usize>,
        #[xattribute(name = "maxOccurs", optional, default)]
        pub max_occurs: ::core::option::Option<crate::xs::types::AllNNI>,
        #[xattribute(name = "default", optional, default)]
        pub default: ::core::option::Option<String>,
        #[xattribute(name = "fixed", optional, default)]
        pub fixed: ::core::option::Option<String>,
        #[xattribute(name = "nillable", optional, default)]
        pub nillable: ::core::option::Option<bool>,
        #[xattribute(name = "abstract", optional, default)]
        pub abstract_: ::core::option::Option<bool>,
        #[xattribute(name = "final", optional, default)]
        pub final_: ::core::option::Option<crate::xs::types::DerivationSetType>,
        #[xattribute(name = "block", optional, default)]
        pub block: ::core::option::Option<crate::xs::types::BlockSetType>,
        #[xattribute(name = "form", optional, default)]
        pub form: ::core::option::Option<crate::xs::types::FormChoiceType>,
        #[xattribute(name = "targetNamespace", optional, default)]
        pub target_namespace: ::core::option::Option<crate::xs::types::TargetNamespace>,
        #[xvalue(default)]
        pub annotation: ::core::option::Option<::std::boxed::Box<crate::xs::Annotation>>,
        #[xvalue(default)]
        pub type_: ::core::option::Option<element_items::Type>,
        #[xvalue(default)]
        #[builder(default)]
        pub alternative: ::std::vec::Vec<element_items::Alternative>,
        #[xvalue(default)]
        #[builder(default)]
        pub identity_constraint: ::std::vec::Vec<crate::xs::groups::IdentityConstraint>,
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::bon::Builder,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xgroup(children_order = "strict")]
    pub struct ExplicitGroup {
        #[xattribute(name = "id", optional, default)]
        pub id: ::core::option::Option<String>,
        #[xattribute(name = "minOccurs", optional, default)]
        pub min_occurs: ::core::option::Option<usize>,
        #[xattribute(name = "maxOccurs", optional, default)]
        pub max_occurs: ::core::option::Option<crate::xs::types::AllNNI>,
        #[xvalue(default)]
        pub annotation: ::core::option::Option<::std::boxed::Box<crate::xs::Annotation>>,
        #[xvalue(default)]
        #[builder(default)]
        pub nested_particle: ::std::vec::Vec<crate::xs::groups::NestedParticle>,
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::bon::Builder,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xgroup(children_order = "strict")]
    pub struct ExtensionType {
        #[xattribute(name = "id", optional, default)]
        pub id: ::core::option::Option<String>,
        #[xattribute(name = "base")]
        pub base: crate::xs::types::QName,
        #[xvalue(default)]
        pub annotation: ::core::option::Option<::std::boxed::Box<crate::xs::Annotation>>,
        #[xvalue(default)]
        pub open_content: ::core::option::Option<
            ::std::boxed::Box<crate::xs::OpenContent>,
        >,
        #[xvalue(default)]
        pub type_def_particle: ::core::option::Option<
            ::std::boxed::Box<crate::xs::groups::TypeDefParticle>,
        >,
        pub attr_decls: ::std::boxed::Box<crate::xs::groups::AttrDecls>,
        pub assertions: ::std::boxed::Box<crate::xs::groups::Assertions>,
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::bon::Builder,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xgroup(children_order = "strict")]
    pub struct Facet {
        #[xattribute(name = "id", optional, default)]
        pub id: ::core::option::Option<String>,
        #[xattribute(name = "value")]
        pub value: String,
        #[xattribute(name = "fixed", optional, default)]
        pub fixed: ::core::option::Option<bool>,
        #[xvalue(default)]
        pub annotation: ::core::option::Option<::std::boxed::Box<crate::xs::Annotation>>,
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::bon::Builder,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xgroup(children_order = "strict")]
    pub struct Group {
        #[xattribute(name = "id", optional, default)]
        pub id: ::core::option::Option<String>,
        #[xattribute(name = "name", optional, default)]
        pub name: ::core::option::Option<::xmlity::LocalName<'static>>,
        #[xattribute(name = "ref", optional, default)]
        pub ref_: ::core::option::Option<crate::xs::types::QName>,
        #[xattribute(name = "minOccurs", optional, default)]
        pub min_occurs: ::core::option::Option<usize>,
        #[xattribute(name = "maxOccurs", optional, default)]
        pub max_occurs: ::core::option::Option<crate::xs::types::AllNNI>,
        #[xvalue(default)]
        pub annotation: ::core::option::Option<::std::boxed::Box<crate::xs::Annotation>>,
        #[xvalue(default)]
        #[builder(default)]
        pub particle: ::std::vec::Vec<crate::xs::groups::Particle>,
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::bon::Builder,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xgroup(children_order = "strict")]
    pub struct GroupRef {
        #[xattribute(name = "id", optional, default)]
        pub id: ::core::option::Option<String>,
        #[xattribute(name = "ref")]
        pub ref_: crate::xs::types::QName,
        #[xattribute(name = "minOccurs", optional, default)]
        pub min_occurs: ::core::option::Option<usize>,
        #[xattribute(name = "maxOccurs", optional, default)]
        pub max_occurs: ::core::option::Option<crate::xs::types::AllNNI>,
        #[xvalue(default)]
        pub annotation: ::core::option::Option<::std::boxed::Box<crate::xs::Annotation>>,
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::bon::Builder,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xgroup(children_order = "strict")]
    pub struct IntFacet {
        #[xattribute(name = "id", optional, default)]
        pub id: ::core::option::Option<String>,
        #[xattribute(name = "value")]
        pub value: i32,
        #[xattribute(name = "fixed", optional, default)]
        pub fixed: ::core::option::Option<bool>,
        #[xvalue(default)]
        pub annotation: ::core::option::Option<::std::boxed::Box<crate::xs::Annotation>>,
    }
    pub mod keybase_items {
        #[derive(
            ::core::fmt::Debug,
            ::xmlity::Serialize,
            ::xmlity::Deserialize,
            ::bon::Builder,
            ::core::cmp::PartialEq,
            ::core::clone::Clone
        )]
        #[xvalue(order = "strict")]
        pub struct Child1 {
            pub selector: ::std::boxed::Box<crate::xs::Selector>,
            #[xvalue(default)]
            #[builder(default)]
            pub field: ::std::vec::Vec<crate::xs::Field>,
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::bon::Builder,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xgroup(children_order = "strict")]
    pub struct Keybase {
        #[xattribute(name = "id", optional, default)]
        pub id: ::core::option::Option<String>,
        #[xattribute(name = "name", optional, default)]
        pub name: ::core::option::Option<::xmlity::LocalName<'static>>,
        #[xattribute(name = "ref", optional, default)]
        pub ref_: ::core::option::Option<crate::xs::types::QName>,
        #[xvalue(default)]
        pub annotation: ::core::option::Option<::std::boxed::Box<crate::xs::Annotation>>,
        #[xvalue(default)]
        pub child_1: ::core::option::Option<keybase_items::Child1>,
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::bon::Builder,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xgroup(children_order = "strict")]
    pub struct LocalComplexType {
        #[xattribute(name = "id", optional, default)]
        pub id: ::core::option::Option<String>,
        #[xattribute(name = "mixed", optional, default)]
        pub mixed: ::core::option::Option<bool>,
        #[xattribute(name = "defaultAttributesApply", optional, default)]
        pub default_attributes_apply: ::core::option::Option<bool>,
        #[xvalue(default)]
        pub annotation: ::core::option::Option<::std::boxed::Box<crate::xs::Annotation>>,
        pub complex_type_model: ::std::boxed::Box<crate::xs::groups::ComplexTypeModel>,
    }
    pub mod local_element_items {
        impl ::core::convert::From<crate::xs::types::LocalSimpleType> for Type {
            fn from(value: crate::xs::types::LocalSimpleType) -> Self {
                Type::SimpleType(::std::boxed::Box::new(value))
            }
        }
        impl ::core::convert::From<crate::xs::types::LocalComplexType> for Type {
            fn from(value: crate::xs::types::LocalComplexType) -> Self {
                Type::ComplexType(::std::boxed::Box::new(value))
            }
        }
        #[derive(
            ::core::fmt::Debug,
            ::xmlity::Serialize,
            ::xmlity::Deserialize,
            ::core::cmp::PartialEq,
            ::core::clone::Clone
        )]
        pub enum Type {
            #[xelement(
                name = "simpleType",
                namespace = "http://www.w3.org/2001/XMLSchema",
                allow_unknown_attributes = "any"
            )]
            SimpleType(#[xgroup] ::std::boxed::Box<crate::xs::types::LocalSimpleType>),
            #[xelement(
                name = "complexType",
                namespace = "http://www.w3.org/2001/XMLSchema",
                allow_unknown_attributes = "any"
            )]
            ComplexType(#[xgroup] ::std::boxed::Box<crate::xs::types::LocalComplexType>),
        }
        impl ::core::convert::From<crate::xs::types::AltType> for Alternative {
            fn from(value: crate::xs::types::AltType) -> Self {
                Alternative(::std::boxed::Box::new(value))
            }
        }
        #[derive(
            ::core::fmt::Debug,
            ::xmlity::Serialize,
            ::xmlity::Deserialize,
            ::core::cmp::PartialEq,
            ::core::clone::Clone
        )]
        #[xelement(
            name = "alternative",
            namespace = "http://www.w3.org/2001/XMLSchema",
            allow_unknown_attributes = "any"
        )]
        pub struct Alternative(
            #[xgroup]
            pub ::std::boxed::Box<crate::xs::types::AltType>,
        );
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::bon::Builder,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xgroup(children_order = "strict")]
    pub struct LocalElement {
        #[xattribute(name = "id", optional, default)]
        pub id: ::core::option::Option<String>,
        #[xattribute(name = "name", optional, default)]
        pub name: ::core::option::Option<::xmlity::LocalName<'static>>,
        #[xattribute(name = "ref", optional, default)]
        pub ref_: ::core::option::Option<crate::xs::types::QName>,
        #[xattribute(name = "type", optional, default)]
        pub type_attribute: ::core::option::Option<crate::xs::types::QName>,
        #[xattribute(name = "minOccurs", optional, default)]
        pub min_occurs: ::core::option::Option<usize>,
        #[xattribute(name = "maxOccurs", optional, default)]
        pub max_occurs: ::core::option::Option<crate::xs::types::AllNNI>,
        #[xattribute(name = "default", optional, default)]
        pub default: ::core::option::Option<String>,
        #[xattribute(name = "fixed", optional, default)]
        pub fixed: ::core::option::Option<String>,
        #[xattribute(name = "nillable", optional, default)]
        pub nillable: ::core::option::Option<bool>,
        #[xattribute(name = "block", optional, default)]
        pub block: ::core::option::Option<crate::xs::types::BlockSetType>,
        #[xattribute(name = "form", optional, default)]
        pub form: ::core::option::Option<crate::xs::types::FormChoiceType>,
        #[xattribute(name = "targetNamespace", optional, default)]
        pub target_namespace: ::core::option::Option<crate::xs::types::TargetNamespace>,
        #[xvalue(default)]
        pub annotation: ::core::option::Option<::std::boxed::Box<crate::xs::Annotation>>,
        #[xvalue(default)]
        pub type_: ::core::option::Option<local_element_items::Type>,
        #[xvalue(default)]
        #[builder(default)]
        pub alternative: ::std::vec::Vec<local_element_items::Alternative>,
        #[xvalue(default)]
        #[builder(default)]
        pub identity_constraint: ::std::vec::Vec<crate::xs::groups::IdentityConstraint>,
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::bon::Builder,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xgroup(children_order = "strict")]
    pub struct LocalSimpleType {
        #[xattribute(name = "id", optional, default)]
        pub id: ::core::option::Option<String>,
        #[xvalue(default)]
        pub annotation: ::core::option::Option<::std::boxed::Box<crate::xs::Annotation>>,
        pub simple_derivation: ::std::boxed::Box<crate::xs::groups::SimpleDerivation>,
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::bon::Builder,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xgroup(children_order = "strict")]
    pub struct NamedAttributeGroup {
        #[xattribute(name = "id", optional, default)]
        pub id: ::core::option::Option<String>,
        #[xattribute(name = "name")]
        pub name: ::xmlity::LocalName<'static>,
        #[xvalue(default)]
        pub annotation: ::core::option::Option<::std::boxed::Box<crate::xs::Annotation>>,
        pub attr_decls: ::std::boxed::Box<crate::xs::groups::AttrDecls>,
    }
    pub mod named_group_items {
        #[derive(
            ::core::fmt::Debug,
            ::xmlity::Serialize,
            ::xmlity::Deserialize,
            ::core::cmp::PartialEq,
            ::core::clone::Clone
        )]
        pub enum Child1 {
            #[xelement(
                name = "all",
                namespace = "http://www.w3.org/2001/XMLSchema",
                allow_unknown_attributes = "any"
            )]
            All {
                #[xattribute(name = "id", optional, default)]
                id: ::core::option::Option<String>,
                #[xattribute(name = "minOccurs", optional, default)]
                min_occurs: ::core::option::Option<usize>,
                #[xattribute(name = "maxOccurs", optional, default)]
                max_occurs: ::core::option::Option<crate::xs::types::AllNNI>,
                all_model: ::std::boxed::Box<crate::xs::groups::AllModel>,
            },
            #[xelement(
                name = "choice",
                namespace = "http://www.w3.org/2001/XMLSchema",
                allow_unknown_attributes = "any"
            )]
            Choice(#[xgroup] ::std::boxed::Box<crate::xs::types::SimpleExplicitGroup>),
            #[xelement(
                name = "sequence",
                namespace = "http://www.w3.org/2001/XMLSchema",
                allow_unknown_attributes = "any"
            )]
            Sequence(#[xgroup] ::std::boxed::Box<crate::xs::types::SimpleExplicitGroup>),
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::bon::Builder,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xgroup(children_order = "strict")]
    pub struct NamedGroup {
        #[xattribute(name = "id", optional, default)]
        pub id: ::core::option::Option<String>,
        #[xattribute(name = "name")]
        pub name: ::xmlity::LocalName<'static>,
        #[xvalue(default)]
        pub annotation: ::core::option::Option<::std::boxed::Box<crate::xs::Annotation>>,
        pub child_1: named_group_items::Child1,
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::bon::Builder,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xgroup(children_order = "strict")]
    pub struct NoFixedFacet {
        #[xattribute(name = "id", optional, default)]
        pub id: ::core::option::Option<String>,
        #[xattribute(name = "value")]
        pub value: String,
        #[xvalue(default)]
        pub annotation: ::core::option::Option<::std::boxed::Box<crate::xs::Annotation>>,
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::bon::Builder,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xgroup(children_order = "strict")]
    pub struct NumFacet {
        #[xattribute(name = "id", optional, default)]
        pub id: ::core::option::Option<String>,
        #[xattribute(name = "value")]
        pub value: usize,
        #[xattribute(name = "fixed", optional, default)]
        pub fixed: ::core::option::Option<bool>,
        #[xvalue(default)]
        pub annotation: ::core::option::Option<::std::boxed::Box<crate::xs::Annotation>>,
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub struct OpenAttrs;
    pub mod real_group_items {
        impl ::core::convert::From<crate::xs::All> for Child1 {
            fn from(value: crate::xs::All) -> Self {
                Child1::All(::std::boxed::Box::new(value))
            }
        }
        impl ::core::convert::From<crate::xs::Choice> for Child1 {
            fn from(value: crate::xs::Choice) -> Self {
                Child1::Choice(::std::boxed::Box::new(value))
            }
        }
        impl ::core::convert::From<crate::xs::Sequence> for Child1 {
            fn from(value: crate::xs::Sequence) -> Self {
                Child1::Sequence(::std::boxed::Box::new(value))
            }
        }
        #[derive(
            ::core::fmt::Debug,
            ::xmlity::Serialize,
            ::xmlity::Deserialize,
            ::core::cmp::PartialEq,
            ::core::clone::Clone
        )]
        pub enum Child1 {
            All(::std::boxed::Box<crate::xs::All>),
            Choice(::std::boxed::Box<crate::xs::Choice>),
            Sequence(::std::boxed::Box<crate::xs::Sequence>),
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::bon::Builder,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xgroup(children_order = "strict")]
    pub struct RealGroup {
        #[xattribute(name = "id", optional, default)]
        pub id: ::core::option::Option<String>,
        #[xattribute(name = "name", optional, default)]
        pub name: ::core::option::Option<::xmlity::LocalName<'static>>,
        #[xattribute(name = "ref", optional, default)]
        pub ref_: ::core::option::Option<crate::xs::types::QName>,
        #[xattribute(name = "minOccurs", optional, default)]
        pub min_occurs: ::core::option::Option<usize>,
        #[xattribute(name = "maxOccurs", optional, default)]
        pub max_occurs: ::core::option::Option<crate::xs::types::AllNNI>,
        #[xvalue(default)]
        pub annotation: ::core::option::Option<::std::boxed::Box<crate::xs::Annotation>>,
        #[xvalue(default)]
        pub child_1: ::core::option::Option<real_group_items::Child1>,
    }
    pub mod restriction_type_items {
        pub mod child_1_variants {
            #[derive(
                ::core::fmt::Debug,
                ::xmlity::Serialize,
                ::xmlity::Deserialize,
                ::bon::Builder,
                ::core::cmp::PartialEq,
                ::core::clone::Clone
            )]
            #[xvalue(order = "strict")]
            pub struct Variant0 {
                #[xvalue(default)]
                pub open_content: ::core::option::Option<
                    ::std::boxed::Box<crate::xs::OpenContent>,
                >,
                pub type_def_particle: ::std::boxed::Box<
                    crate::xs::groups::TypeDefParticle,
                >,
            }
        }
        impl ::core::convert::From<child_1_variants::Variant0> for Child1 {
            fn from(value: child_1_variants::Variant0) -> Self {
                Child1::Variant0(::std::boxed::Box::new(value))
            }
        }
        impl ::core::convert::From<crate::xs::groups::SimpleRestrictionModel>
        for Child1 {
            fn from(value: crate::xs::groups::SimpleRestrictionModel) -> Self {
                Child1::SimpleRestrictionModel(::std::boxed::Box::new(value))
            }
        }
        #[derive(
            ::core::fmt::Debug,
            ::xmlity::Serialize,
            ::xmlity::Deserialize,
            ::core::cmp::PartialEq,
            ::core::clone::Clone
        )]
        pub enum Child1 {
            Variant0(::std::boxed::Box<child_1_variants::Variant0>),
            SimpleRestrictionModel(
                ::std::boxed::Box<crate::xs::groups::SimpleRestrictionModel>,
            ),
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::bon::Builder,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xgroup(children_order = "strict")]
    pub struct RestrictionType {
        #[xattribute(name = "id", optional, default)]
        pub id: ::core::option::Option<String>,
        #[xattribute(name = "base")]
        pub base: crate::xs::types::QName,
        #[xvalue(default)]
        pub annotation: ::core::option::Option<::std::boxed::Box<crate::xs::Annotation>>,
        #[xvalue(default)]
        pub child_1: ::core::option::Option<restriction_type_items::Child1>,
        pub attr_decls: ::std::boxed::Box<crate::xs::groups::AttrDecls>,
        pub assertions: ::std::boxed::Box<crate::xs::groups::Assertions>,
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::bon::Builder,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xgroup(children_order = "strict")]
    pub struct SimpleBaseType {
        #[xattribute(name = "id", optional, default)]
        pub id: ::core::option::Option<String>,
        #[xattribute(name = "final", optional, default)]
        pub final_: ::core::option::Option<String>,
        #[xattribute(name = "name", optional, default)]
        pub name: ::core::option::Option<::xmlity::LocalName<'static>>,
        #[xvalue(default)]
        pub annotation: ::core::option::Option<::std::boxed::Box<crate::xs::Annotation>>,
        pub simple_derivation: ::std::boxed::Box<crate::xs::groups::SimpleDerivation>,
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::bon::Builder,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xgroup(children_order = "strict")]
    pub struct SimpleExplicitGroup {
        #[xattribute(name = "id", optional, default)]
        pub id: ::core::option::Option<String>,
        #[xvalue(default)]
        pub annotation: ::core::option::Option<::std::boxed::Box<crate::xs::Annotation>>,
        #[xvalue(default)]
        #[builder(default)]
        pub nested_particle: ::std::vec::Vec<crate::xs::groups::NestedParticle>,
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::bon::Builder,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xgroup(children_order = "strict")]
    pub struct SimpleExtensionType {
        #[xattribute(name = "id", optional, default)]
        pub id: ::core::option::Option<String>,
        #[xattribute(name = "base")]
        pub base: crate::xs::types::QName,
        #[xvalue(default)]
        pub annotation: ::core::option::Option<::std::boxed::Box<crate::xs::Annotation>>,
        pub attr_decls: ::std::boxed::Box<crate::xs::groups::AttrDecls>,
        pub assertions: ::std::boxed::Box<crate::xs::groups::Assertions>,
    }
    pub mod simple_restriction_type_items {
        impl ::core::convert::From<crate::xs::groups::SimpleRestrictionModel>
        for SimpleRestrictionModel {
            fn from(value: crate::xs::groups::SimpleRestrictionModel) -> Self {
                SimpleRestrictionModel::SimpleRestrictionModel(
                    ::std::boxed::Box::new(value),
                )
            }
        }
        #[derive(
            ::core::fmt::Debug,
            ::xmlity::Serialize,
            ::xmlity::Deserialize,
            ::core::cmp::PartialEq,
            ::core::clone::Clone
        )]
        pub enum SimpleRestrictionModel {
            SimpleRestrictionModel(
                ::std::boxed::Box<crate::xs::groups::SimpleRestrictionModel>,
            ),
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::bon::Builder,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xgroup(children_order = "strict")]
    pub struct SimpleRestrictionType {
        #[xattribute(name = "id", optional, default)]
        pub id: ::core::option::Option<String>,
        #[xattribute(name = "base")]
        pub base: crate::xs::types::QName,
        #[xvalue(default)]
        pub annotation: ::core::option::Option<::std::boxed::Box<crate::xs::Annotation>>,
        #[xvalue(default)]
        pub simple_restriction_model: ::core::option::Option<
            simple_restriction_type_items::SimpleRestrictionModel,
        >,
        pub attr_decls: ::std::boxed::Box<crate::xs::groups::AttrDecls>,
        pub assertions: ::std::boxed::Box<crate::xs::groups::Assertions>,
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::bon::Builder,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xgroup(children_order = "strict")]
    pub struct TopLevelAttribute {
        #[xattribute(name = "id", optional, default)]
        pub id: ::core::option::Option<String>,
        #[xattribute(name = "name")]
        pub name: ::xmlity::LocalName<'static>,
        #[xattribute(name = "type", optional, default)]
        pub type_: ::core::option::Option<crate::xs::types::QName>,
        #[xattribute(name = "default", optional, default)]
        pub default: ::core::option::Option<String>,
        #[xattribute(name = "fixed", optional, default)]
        pub fixed: ::core::option::Option<String>,
        #[xattribute(name = "inheritable", optional, default)]
        pub inheritable: ::core::option::Option<bool>,
        #[xvalue(default)]
        pub annotation: ::core::option::Option<::std::boxed::Box<crate::xs::Annotation>>,
        #[xelement(
            name = "simpleType",
            namespace = "http://www.w3.org/2001/XMLSchema",
            group,
            optional
        )]
        pub simple_type: ::core::option::Option<
            ::std::boxed::Box<crate::xs::types::LocalSimpleType>,
        >,
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::bon::Builder,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xgroup(children_order = "strict")]
    pub struct TopLevelComplexType {
        #[xattribute(name = "id", optional, default)]
        pub id: ::core::option::Option<String>,
        #[xattribute(name = "name")]
        pub name: ::xmlity::LocalName<'static>,
        #[xattribute(name = "mixed", optional, default)]
        pub mixed: ::core::option::Option<bool>,
        #[xattribute(name = "abstract", optional, default)]
        pub abstract_: ::core::option::Option<bool>,
        #[xattribute(name = "final", optional, default)]
        pub final_: ::core::option::Option<crate::xs::types::DerivationSetType>,
        #[xattribute(name = "block", optional, default)]
        pub block: ::core::option::Option<crate::xs::types::DerivationSetType>,
        #[xattribute(name = "defaultAttributesApply", optional, default)]
        pub default_attributes_apply: ::core::option::Option<bool>,
        #[xvalue(default)]
        pub annotation: ::core::option::Option<::std::boxed::Box<crate::xs::Annotation>>,
        pub complex_type_model: ::std::boxed::Box<crate::xs::groups::ComplexTypeModel>,
    }
    pub mod top_level_element_items {
        impl ::core::convert::From<crate::xs::types::LocalSimpleType> for Type {
            fn from(value: crate::xs::types::LocalSimpleType) -> Self {
                Type::SimpleType(::std::boxed::Box::new(value))
            }
        }
        impl ::core::convert::From<crate::xs::types::LocalComplexType> for Type {
            fn from(value: crate::xs::types::LocalComplexType) -> Self {
                Type::ComplexType(::std::boxed::Box::new(value))
            }
        }
        #[derive(
            ::core::fmt::Debug,
            ::xmlity::Serialize,
            ::xmlity::Deserialize,
            ::core::cmp::PartialEq,
            ::core::clone::Clone
        )]
        pub enum Type {
            #[xelement(
                name = "simpleType",
                namespace = "http://www.w3.org/2001/XMLSchema",
                allow_unknown_attributes = "any"
            )]
            SimpleType(#[xgroup] ::std::boxed::Box<crate::xs::types::LocalSimpleType>),
            #[xelement(
                name = "complexType",
                namespace = "http://www.w3.org/2001/XMLSchema",
                allow_unknown_attributes = "any"
            )]
            ComplexType(#[xgroup] ::std::boxed::Box<crate::xs::types::LocalComplexType>),
        }
        impl ::core::convert::From<crate::xs::types::AltType> for Alternative {
            fn from(value: crate::xs::types::AltType) -> Self {
                Alternative(::std::boxed::Box::new(value))
            }
        }
        #[derive(
            ::core::fmt::Debug,
            ::xmlity::Serialize,
            ::xmlity::Deserialize,
            ::core::cmp::PartialEq,
            ::core::clone::Clone
        )]
        #[xelement(
            name = "alternative",
            namespace = "http://www.w3.org/2001/XMLSchema",
            allow_unknown_attributes = "any"
        )]
        pub struct Alternative(
            #[xgroup]
            pub ::std::boxed::Box<crate::xs::types::AltType>,
        );
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::bon::Builder,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xgroup(children_order = "strict")]
    pub struct TopLevelElement {
        #[xattribute(name = "id", optional, default)]
        pub id: ::core::option::Option<String>,
        #[xattribute(name = "name")]
        pub name: ::xmlity::LocalName<'static>,
        #[xattribute(name = "type", optional, default)]
        pub type_attribute: ::core::option::Option<crate::xs::types::QName>,
        #[xattribute(name = "substitutionGroup", optional, default)]
        pub substitution_group: ::core::option::Option<String>,
        #[xattribute(name = "default", optional, default)]
        pub default: ::core::option::Option<String>,
        #[xattribute(name = "fixed", optional, default)]
        pub fixed: ::core::option::Option<String>,
        #[xattribute(name = "nillable", optional, default)]
        pub nillable: ::core::option::Option<bool>,
        #[xattribute(name = "abstract", optional, default)]
        pub abstract_: ::core::option::Option<bool>,
        #[xattribute(name = "final", optional, default)]
        pub final_: ::core::option::Option<crate::xs::types::DerivationSetType>,
        #[xattribute(name = "block", optional, default)]
        pub block: ::core::option::Option<crate::xs::types::BlockSetType>,
        #[xvalue(default)]
        pub annotation: ::core::option::Option<::std::boxed::Box<crate::xs::Annotation>>,
        #[xvalue(default)]
        pub type_: ::core::option::Option<top_level_element_items::Type>,
        #[xvalue(default)]
        #[builder(default)]
        pub alternative: ::std::vec::Vec<top_level_element_items::Alternative>,
        #[xvalue(default)]
        #[builder(default)]
        pub identity_constraint: ::std::vec::Vec<crate::xs::groups::IdentityConstraint>,
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::bon::Builder,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xgroup(children_order = "strict")]
    pub struct TopLevelSimpleType {
        #[xattribute(name = "id", optional, default)]
        pub id: ::core::option::Option<String>,
        #[xattribute(name = "final", optional, default)]
        pub final_: ::core::option::Option<String>,
        #[xattribute(name = "name")]
        pub name: ::xmlity::LocalName<'static>,
        #[xvalue(default)]
        pub annotation: ::core::option::Option<::std::boxed::Box<crate::xs::Annotation>>,
        pub simple_derivation: ::std::boxed::Box<crate::xs::groups::SimpleDerivation>,
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::bon::Builder,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xgroup(children_order = "strict")]
    pub struct Wildcard {
        #[xattribute(name = "id", optional, default)]
        pub id: ::core::option::Option<String>,
        #[xattribute(name = "namespace", optional, default)]
        pub namespace: ::core::option::Option<crate::xs::types::NamespaceListType>,
        #[xattribute(name = "notNamespace", optional, default)]
        pub not_namespace: ::core::option::Option<String>,
        #[xattribute(name = "processContents", optional, default)]
        pub process_contents: ::core::option::Option<String>,
        #[xvalue(default)]
        pub annotation: ::core::option::Option<::std::boxed::Box<crate::xs::Annotation>>,
    }
}
pub mod attributes {}
pub mod groups {
    pub mod all_model_items {
        impl ::core::convert::From<crate::xs::types::LocalElement> for Child1 {
            fn from(value: crate::xs::types::LocalElement) -> Self {
                Child1::Element(::std::boxed::Box::new(value))
            }
        }
        impl ::core::convert::From<crate::xs::Any> for Child1 {
            fn from(value: crate::xs::Any) -> Self {
                Child1::Any(::std::boxed::Box::new(value))
            }
        }
        #[derive(
            ::core::fmt::Debug,
            ::xmlity::Serialize,
            ::xmlity::Deserialize,
            ::core::cmp::PartialEq,
            ::core::clone::Clone
        )]
        pub enum Child1 {
            #[xelement(
                name = "element",
                namespace = "http://www.w3.org/2001/XMLSchema",
                allow_unknown_attributes = "any"
            )]
            Element(#[xgroup] ::std::boxed::Box<crate::xs::types::LocalElement>),
            Any(::std::boxed::Box<crate::xs::Any>),
            #[xelement(
                name = "group",
                namespace = "http://www.w3.org/2001/XMLSchema",
                allow_unknown_attributes = "any",
                children_order = "strict"
            )]
            Group {
                #[xattribute(name = "id", optional, default)]
                id: ::core::option::Option<String>,
                #[xattribute(name = "ref")]
                ref_: crate::xs::types::QName,
                #[xattribute(name = "minOccurs", optional, default)]
                min_occurs: ::core::option::Option<usize>,
                #[xattribute(name = "maxOccurs", optional, default)]
                max_occurs: ::core::option::Option<usize>,
                #[xvalue(default)]
                annotation: ::core::option::Option<
                    ::std::boxed::Box<crate::xs::Annotation>,
                >,
            },
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::bon::Builder,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xvalue(order = "strict")]
    pub struct AllModel {
        #[xvalue(default)]
        pub annotation: ::core::option::Option<::std::boxed::Box<crate::xs::Annotation>>,
        #[xvalue(default)]
        #[builder(default)]
        pub child_1: ::std::vec::Vec<all_model_items::Child1>,
    }
    pub mod assertions_items {
        impl ::core::convert::From<crate::xs::types::Assertion> for Assert {
            fn from(value: crate::xs::types::Assertion) -> Self {
                Assert(::std::boxed::Box::new(value))
            }
        }
        #[derive(
            ::core::fmt::Debug,
            ::xmlity::Serialize,
            ::xmlity::Deserialize,
            ::core::cmp::PartialEq,
            ::core::clone::Clone
        )]
        #[xelement(
            name = "assert",
            namespace = "http://www.w3.org/2001/XMLSchema",
            allow_unknown_attributes = "any"
        )]
        pub struct Assert(#[xgroup] pub ::std::boxed::Box<crate::xs::types::Assertion>);
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::bon::Builder,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xvalue(order = "strict")]
    pub struct Assertions {
        #[xvalue(default)]
        #[builder(default)]
        pub assert: ::std::vec::Vec<assertions_items::Assert>,
    }
    pub mod attr_decls_items {
        impl ::core::convert::From<crate::xs::types::Attribute> for Attribute {
            fn from(value: crate::xs::types::Attribute) -> Self {
                Attribute::Attribute(::std::boxed::Box::new(value))
            }
        }
        impl ::core::convert::From<crate::xs::types::AttributeGroupRef> for Attribute {
            fn from(value: crate::xs::types::AttributeGroupRef) -> Self {
                Attribute::AttributeGroup(::std::boxed::Box::new(value))
            }
        }
        #[derive(
            ::core::fmt::Debug,
            ::xmlity::Serialize,
            ::xmlity::Deserialize,
            ::core::cmp::PartialEq,
            ::core::clone::Clone
        )]
        pub enum Attribute {
            #[xelement(
                name = "attribute",
                namespace = "http://www.w3.org/2001/XMLSchema",
                allow_unknown_attributes = "any"
            )]
            Attribute(#[xgroup] ::std::boxed::Box<crate::xs::types::Attribute>),
            #[xelement(
                name = "attributeGroup",
                namespace = "http://www.w3.org/2001/XMLSchema",
                allow_unknown_attributes = "any"
            )]
            AttributeGroup(
                #[xgroup]
                ::std::boxed::Box<crate::xs::types::AttributeGroupRef>,
            ),
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::bon::Builder,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xvalue(order = "strict")]
    pub struct AttrDecls {
        #[xvalue(default)]
        #[builder(default)]
        pub attribute: ::std::vec::Vec<attr_decls_items::Attribute>,
        #[xvalue(default)]
        pub any_attribute: ::core::option::Option<
            ::std::boxed::Box<crate::xs::AnyAttribute>,
        >,
    }
    pub mod complex_type_model_items {
        pub mod complex_type_model_variants {
            #[derive(
                ::core::fmt::Debug,
                ::xmlity::Serialize,
                ::xmlity::Deserialize,
                ::bon::Builder,
                ::core::cmp::PartialEq,
                ::core::clone::Clone
            )]
            #[xvalue(order = "strict")]
            pub struct Variant2 {
                #[xvalue(default)]
                pub open_content: ::core::option::Option<
                    ::std::boxed::Box<crate::xs::OpenContent>,
                >,
                #[xvalue(default)]
                pub type_def_particle: ::core::option::Option<
                    ::std::boxed::Box<crate::xs::groups::TypeDefParticle>,
                >,
                pub attr_decls: crate::xs::groups::AttrDecls,
                pub assertions: crate::xs::groups::Assertions,
            }
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub enum ComplexTypeModel {
        SimpleContent(::std::boxed::Box<crate::xs::SimpleContent>),
        ComplexContent(::std::boxed::Box<crate::xs::ComplexContent>),
        Variant2(
            ::std::boxed::Box<
                complex_type_model_items::complex_type_model_variants::Variant2,
            >,
        ),
    }
    impl ::core::convert::From<crate::xs::SimpleContent> for ComplexTypeModel {
        fn from(value: crate::xs::SimpleContent) -> Self {
            ComplexTypeModel::SimpleContent(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xs::ComplexContent> for ComplexTypeModel {
        fn from(value: crate::xs::ComplexContent) -> Self {
            ComplexTypeModel::ComplexContent(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<
        complex_type_model_items::complex_type_model_variants::Variant2,
    > for ComplexTypeModel {
        fn from(
            value: complex_type_model_items::complex_type_model_variants::Variant2,
        ) -> Self {
            ComplexTypeModel::Variant2(::std::boxed::Box::new(value))
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub enum Composition {
        Include(::std::boxed::Box<crate::xs::Include>),
        Import(::std::boxed::Box<crate::xs::Import>),
        Redefine(::std::boxed::Box<crate::xs::Redefine>),
        Override(::std::boxed::Box<crate::xs::Override>),
        Annotation(::std::boxed::Box<crate::xs::Annotation>),
    }
    impl ::core::convert::From<crate::xs::Include> for Composition {
        fn from(value: crate::xs::Include) -> Self {
            Composition::Include(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xs::Import> for Composition {
        fn from(value: crate::xs::Import) -> Self {
            Composition::Import(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xs::Redefine> for Composition {
        fn from(value: crate::xs::Redefine) -> Self {
            Composition::Redefine(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xs::Override> for Composition {
        fn from(value: crate::xs::Override) -> Self {
            Composition::Override(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xs::Annotation> for Composition {
        fn from(value: crate::xs::Annotation) -> Self {
            Composition::Annotation(::std::boxed::Box::new(value))
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub enum IdentityConstraint {
        Unique(::std::boxed::Box<crate::xs::Unique>),
        Key(::std::boxed::Box<crate::xs::Key>),
        Keyref(::std::boxed::Box<crate::xs::Keyref>),
    }
    impl ::core::convert::From<crate::xs::Unique> for IdentityConstraint {
        fn from(value: crate::xs::Unique) -> Self {
            IdentityConstraint::Unique(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xs::Key> for IdentityConstraint {
        fn from(value: crate::xs::Key) -> Self {
            IdentityConstraint::Key(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xs::Keyref> for IdentityConstraint {
        fn from(value: crate::xs::Keyref) -> Self {
            IdentityConstraint::Keyref(::std::boxed::Box::new(value))
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub enum NestedParticle {
        #[xelement(
            name = "element",
            namespace = "http://www.w3.org/2001/XMLSchema",
            allow_unknown_attributes = "any"
        )]
        Element(#[xgroup] ::std::boxed::Box<crate::xs::types::LocalElement>),
        #[xelement(
            name = "group",
            namespace = "http://www.w3.org/2001/XMLSchema",
            allow_unknown_attributes = "any"
        )]
        Group(#[xgroup] ::std::boxed::Box<crate::xs::types::GroupRef>),
        Choice(::std::boxed::Box<crate::xs::Choice>),
        Sequence(::std::boxed::Box<crate::xs::Sequence>),
        Any(::std::boxed::Box<crate::xs::Any>),
    }
    impl ::core::convert::From<crate::xs::types::LocalElement> for NestedParticle {
        fn from(value: crate::xs::types::LocalElement) -> Self {
            NestedParticle::Element(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xs::types::GroupRef> for NestedParticle {
        fn from(value: crate::xs::types::GroupRef) -> Self {
            NestedParticle::Group(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xs::Choice> for NestedParticle {
        fn from(value: crate::xs::Choice) -> Self {
            NestedParticle::Choice(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xs::Sequence> for NestedParticle {
        fn from(value: crate::xs::Sequence) -> Self {
            NestedParticle::Sequence(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xs::Any> for NestedParticle {
        fn from(value: crate::xs::Any) -> Self {
            NestedParticle::Any(::std::boxed::Box::new(value))
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub enum Particle {
        #[xelement(
            name = "element",
            namespace = "http://www.w3.org/2001/XMLSchema",
            allow_unknown_attributes = "any"
        )]
        Element(#[xgroup] ::std::boxed::Box<crate::xs::types::LocalElement>),
        #[xelement(
            name = "group",
            namespace = "http://www.w3.org/2001/XMLSchema",
            allow_unknown_attributes = "any"
        )]
        Group(#[xgroup] ::std::boxed::Box<crate::xs::types::GroupRef>),
        All(::std::boxed::Box<crate::xs::All>),
        Choice(::std::boxed::Box<crate::xs::Choice>),
        Sequence(::std::boxed::Box<crate::xs::Sequence>),
        Any(::std::boxed::Box<crate::xs::Any>),
    }
    impl ::core::convert::From<crate::xs::types::LocalElement> for Particle {
        fn from(value: crate::xs::types::LocalElement) -> Self {
            Particle::Element(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xs::types::GroupRef> for Particle {
        fn from(value: crate::xs::types::GroupRef) -> Self {
            Particle::Group(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xs::All> for Particle {
        fn from(value: crate::xs::All) -> Self {
            Particle::All(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xs::Choice> for Particle {
        fn from(value: crate::xs::Choice) -> Self {
            Particle::Choice(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xs::Sequence> for Particle {
        fn from(value: crate::xs::Sequence) -> Self {
            Particle::Sequence(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xs::Any> for Particle {
        fn from(value: crate::xs::Any) -> Self {
            Particle::Any(::std::boxed::Box::new(value))
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub enum Redefinable {
        SimpleType(::std::boxed::Box<crate::xs::SimpleType>),
        ComplexType(::std::boxed::Box<crate::xs::ComplexType>),
        Group(::std::boxed::Box<crate::xs::Group>),
        AttributeGroup(::std::boxed::Box<crate::xs::AttributeGroup>),
    }
    impl ::core::convert::From<crate::xs::SimpleType> for Redefinable {
        fn from(value: crate::xs::SimpleType) -> Self {
            Redefinable::SimpleType(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xs::ComplexType> for Redefinable {
        fn from(value: crate::xs::ComplexType) -> Self {
            Redefinable::ComplexType(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xs::Group> for Redefinable {
        fn from(value: crate::xs::Group) -> Self {
            Redefinable::Group(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xs::AttributeGroup> for Redefinable {
        fn from(value: crate::xs::AttributeGroup) -> Self {
            Redefinable::AttributeGroup(::std::boxed::Box::new(value))
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub enum SchemaTop {
        Redefinable(::std::boxed::Box<crate::xs::groups::Redefinable>),
        Element(::std::boxed::Box<crate::xs::Element>),
        Attribute(::std::boxed::Box<crate::xs::Attribute>),
        Notation(::std::boxed::Box<crate::xs::Notation>),
    }
    impl ::core::convert::From<crate::xs::groups::Redefinable> for SchemaTop {
        fn from(value: crate::xs::groups::Redefinable) -> Self {
            SchemaTop::Redefinable(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xs::Element> for SchemaTop {
        fn from(value: crate::xs::Element) -> Self {
            SchemaTop::Element(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xs::Attribute> for SchemaTop {
        fn from(value: crate::xs::Attribute) -> Self {
            SchemaTop::Attribute(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xs::Notation> for SchemaTop {
        fn from(value: crate::xs::Notation) -> Self {
            SchemaTop::Notation(::std::boxed::Box::new(value))
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub enum SimpleDerivation {
        Restriction(::std::boxed::Box<crate::xs::Restriction>),
        List(::std::boxed::Box<crate::xs::List>),
        Union(::std::boxed::Box<crate::xs::Union>),
    }
    impl ::core::convert::From<crate::xs::Restriction> for SimpleDerivation {
        fn from(value: crate::xs::Restriction) -> Self {
            SimpleDerivation::Restriction(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xs::List> for SimpleDerivation {
        fn from(value: crate::xs::List) -> Self {
            SimpleDerivation::List(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xs::Union> for SimpleDerivation {
        fn from(value: crate::xs::Union) -> Self {
            SimpleDerivation::Union(::std::boxed::Box::new(value))
        }
    }
    pub mod simple_restriction_model_items {
        impl ::core::convert::From<crate::xs::Facet> for Child1 {
            fn from(value: crate::xs::Facet) -> Self {
                Child1::Facet(::std::boxed::Box::new(value))
            }
        }
        impl ::core::convert::From<::xmlity::XmlValue> for Child1 {
            fn from(value: ::xmlity::XmlValue) -> Self {
                Child1::Variant1(::std::boxed::Box::new(value))
            }
        }
        #[derive(
            ::core::fmt::Debug,
            ::xmlity::Serialize,
            ::xmlity::Deserialize,
            ::core::cmp::PartialEq,
            ::core::clone::Clone
        )]
        pub enum Child1 {
            Facet(::std::boxed::Box<crate::xs::Facet>),
            Variant1(::std::boxed::Box<::xmlity::XmlValue>),
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::bon::Builder,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xvalue(order = "strict")]
    pub struct SimpleRestrictionModel {
        #[xelement(
            name = "simpleType",
            namespace = "http://www.w3.org/2001/XMLSchema",
            group,
            optional
        )]
        pub simple_type: ::core::option::Option<
            ::std::boxed::Box<crate::xs::types::LocalSimpleType>,
        >,
        #[xvalue(default)]
        #[builder(default)]
        pub child_1: ::std::vec::Vec<simple_restriction_model_items::Child1>,
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub enum TypeDefParticle {
        #[xelement(
            name = "group",
            namespace = "http://www.w3.org/2001/XMLSchema",
            allow_unknown_attributes = "any"
        )]
        Group(#[xgroup] ::std::boxed::Box<crate::xs::types::GroupRef>),
        All(::std::boxed::Box<crate::xs::All>),
        Choice(::std::boxed::Box<crate::xs::Choice>),
        Sequence(::std::boxed::Box<crate::xs::Sequence>),
    }
    impl ::core::convert::From<crate::xs::types::GroupRef> for TypeDefParticle {
        fn from(value: crate::xs::types::GroupRef) -> Self {
            TypeDefParticle::Group(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xs::All> for TypeDefParticle {
        fn from(value: crate::xs::All) -> Self {
            TypeDefParticle::All(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xs::Choice> for TypeDefParticle {
        fn from(value: crate::xs::Choice) -> Self {
            TypeDefParticle::Choice(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xs::Sequence> for TypeDefParticle {
        fn from(value: crate::xs::Sequence) -> Self {
            TypeDefParticle::Sequence(::std::boxed::Box::new(value))
        }
    }
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
#[xelement(
    name = "all",
    namespace = "http://www.w3.org/2001/XMLSchema",
    allow_unknown_attributes = "any"
)]
pub struct All(#[xgroup] pub ::std::boxed::Box<crate::xs::types::All>);
impl ::core::convert::From<crate::xs::types::All> for All {
    fn from(value: crate::xs::types::All) -> Self {
        All(::std::boxed::Box::new(value))
    }
}
pub mod annotation_items {
    impl ::core::convert::From<crate::xs::Appinfo> for Annotation {
        fn from(value: crate::xs::Appinfo) -> Self {
            Annotation::Appinfo(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xs::Documentation> for Annotation {
        fn from(value: crate::xs::Documentation) -> Self {
            Annotation::Documentation(::std::boxed::Box::new(value))
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub enum Annotation {
        Appinfo(::std::boxed::Box<crate::xs::Appinfo>),
        Documentation(::std::boxed::Box<crate::xs::Documentation>),
    }
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::bon::Builder,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
#[xelement(
    name = "annotation",
    namespace = "http://www.w3.org/2001/XMLSchema",
    allow_unknown_attributes = "any"
)]
pub struct Annotation {
    #[xattribute(name = "id", optional, default)]
    pub id: ::core::option::Option<String>,
    #[xvalue(default)]
    #[builder(default)]
    pub annotation: ::std::vec::Vec<annotation_items::Annotation>,
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::bon::Builder,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
#[xelement(
    name = "any",
    namespace = "http://www.w3.org/2001/XMLSchema",
    allow_unknown_attributes = "any",
    children_order = "strict"
)]
pub struct Any {
    #[xattribute(name = "id", optional, default)]
    pub id: ::core::option::Option<String>,
    #[xattribute(name = "namespace", optional, default)]
    pub namespace: ::core::option::Option<crate::xs::types::NamespaceListType>,
    #[xattribute(name = "notNamespace", optional, default)]
    pub not_namespace: ::core::option::Option<String>,
    #[xattribute(name = "processContents", optional, default)]
    pub process_contents: ::core::option::Option<String>,
    #[xattribute(name = "notQName", optional, default)]
    pub not_q_name: ::core::option::Option<crate::xs::types::QnameListType>,
    #[xattribute(name = "minOccurs", optional, default)]
    pub min_occurs: ::core::option::Option<usize>,
    #[xattribute(name = "maxOccurs", optional, default)]
    pub max_occurs: ::core::option::Option<crate::xs::types::AllNNI>,
    #[xvalue(default)]
    pub annotation: ::core::option::Option<crate::xs::Annotation>,
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::bon::Builder,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
#[xelement(
    name = "anyAttribute",
    namespace = "http://www.w3.org/2001/XMLSchema",
    allow_unknown_attributes = "any"
)]
pub struct AnyAttribute {
    #[xattribute(name = "id", optional, default)]
    pub id: ::core::option::Option<String>,
    #[xattribute(name = "namespace", optional, default)]
    pub namespace: ::core::option::Option<crate::xs::types::NamespaceListType>,
    #[xattribute(name = "notNamespace", optional, default)]
    pub not_namespace: ::core::option::Option<String>,
    #[xattribute(name = "processContents", optional, default)]
    pub process_contents: ::core::option::Option<String>,
    #[xattribute(name = "notQName", optional, default)]
    pub not_q_name: ::core::option::Option<crate::xs::types::QnameListAType>,
}
pub mod appinfo_items {
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::bon::Builder,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xvalue(order = "strict")]
    pub struct Child0 {
        pub child_0: ::xmlity::XmlValue,
    }
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::bon::Builder,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
#[xelement(
    name = "appinfo",
    namespace = "http://www.w3.org/2001/XMLSchema",
    allow_unknown_attributes = "any"
)]
pub struct Appinfo {
    #[xattribute(name = "source", optional, default)]
    pub source: ::core::option::Option<crate::xs::types::TargetNamespace>,
    #[xvalue(default)]
    #[builder(default)]
    pub particle: ::std::vec::Vec<appinfo_items::Child0>,
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
#[xelement(
    name = "assertion",
    namespace = "http://www.w3.org/2001/XMLSchema",
    allow_unknown_attributes = "any"
)]
pub struct Assertion(#[xgroup] pub ::std::boxed::Box<crate::xs::types::Assertion>);
impl ::core::convert::From<crate::xs::types::Assertion> for Assertion {
    fn from(value: crate::xs::types::Assertion) -> Self {
        Assertion(::std::boxed::Box::new(value))
    }
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
#[xelement(
    name = "attribute",
    namespace = "http://www.w3.org/2001/XMLSchema",
    allow_unknown_attributes = "any"
)]
pub struct Attribute(
    #[xgroup]
    pub ::std::boxed::Box<crate::xs::types::TopLevelAttribute>,
);
impl ::core::convert::From<crate::xs::types::TopLevelAttribute> for Attribute {
    fn from(value: crate::xs::types::TopLevelAttribute) -> Self {
        Attribute(::std::boxed::Box::new(value))
    }
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
#[xelement(
    name = "attributeGroup",
    namespace = "http://www.w3.org/2001/XMLSchema",
    allow_unknown_attributes = "any"
)]
pub struct AttributeGroup(
    #[xgroup]
    pub ::std::boxed::Box<crate::xs::types::NamedAttributeGroup>,
);
impl ::core::convert::From<crate::xs::types::NamedAttributeGroup> for AttributeGroup {
    fn from(value: crate::xs::types::NamedAttributeGroup) -> Self {
        AttributeGroup(::std::boxed::Box::new(value))
    }
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
#[xelement(
    name = "choice",
    namespace = "http://www.w3.org/2001/XMLSchema",
    allow_unknown_attributes = "any"
)]
pub struct Choice(#[xgroup] pub ::std::boxed::Box<crate::xs::types::ExplicitGroup>);
impl ::core::convert::From<crate::xs::types::ExplicitGroup> for Choice {
    fn from(value: crate::xs::types::ExplicitGroup) -> Self {
        Choice(::std::boxed::Box::new(value))
    }
}
pub mod complex_content_items {
    impl ::core::convert::From<crate::xs::types::ComplexRestrictionType> for Child1 {
        fn from(value: crate::xs::types::ComplexRestrictionType) -> Self {
            Child1::Restriction(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xs::types::ExtensionType> for Child1 {
        fn from(value: crate::xs::types::ExtensionType) -> Self {
            Child1::Extension(::std::boxed::Box::new(value))
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub enum Child1 {
        #[xelement(
            name = "restriction",
            namespace = "http://www.w3.org/2001/XMLSchema",
            allow_unknown_attributes = "any"
        )]
        Restriction(
            #[xgroup]
            ::std::boxed::Box<crate::xs::types::ComplexRestrictionType>,
        ),
        #[xelement(
            name = "extension",
            namespace = "http://www.w3.org/2001/XMLSchema",
            allow_unknown_attributes = "any"
        )]
        Extension(#[xgroup] ::std::boxed::Box<crate::xs::types::ExtensionType>),
    }
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::bon::Builder,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
#[xelement(
    name = "complexContent",
    namespace = "http://www.w3.org/2001/XMLSchema",
    allow_unknown_attributes = "any",
    children_order = "strict"
)]
pub struct ComplexContent {
    #[xattribute(name = "id", optional, default)]
    pub id: ::core::option::Option<String>,
    #[xattribute(name = "mixed", optional, default)]
    pub mixed: ::core::option::Option<bool>,
    #[xvalue(default)]
    pub annotation: ::core::option::Option<crate::xs::Annotation>,
    pub child_1: complex_content_items::Child1,
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
#[xelement(
    name = "complexType",
    namespace = "http://www.w3.org/2001/XMLSchema",
    allow_unknown_attributes = "any"
)]
pub struct ComplexType(
    #[xgroup]
    pub ::std::boxed::Box<crate::xs::types::TopLevelComplexType>,
);
impl ::core::convert::From<crate::xs::types::TopLevelComplexType> for ComplexType {
    fn from(value: crate::xs::types::TopLevelComplexType) -> Self {
        ComplexType(::std::boxed::Box::new(value))
    }
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::bon::Builder,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
#[xelement(
    name = "defaultOpenContent",
    namespace = "http://www.w3.org/2001/XMLSchema",
    allow_unknown_attributes = "any",
    children_order = "strict"
)]
pub struct DefaultOpenContent {
    #[xattribute(name = "id", optional, default)]
    pub id: ::core::option::Option<String>,
    #[xattribute(name = "appliesToEmpty", optional, default)]
    pub applies_to_empty: ::core::option::Option<bool>,
    #[xattribute(name = "mode", optional, default)]
    pub mode: ::core::option::Option<String>,
    #[xvalue(default)]
    pub annotation: ::core::option::Option<crate::xs::Annotation>,
    #[xelement(name = "any", namespace = "http://www.w3.org/2001/XMLSchema", group)]
    pub any: ::std::boxed::Box<crate::xs::types::Wildcard>,
}
pub mod documentation_items {
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::bon::Builder,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xvalue(order = "strict")]
    pub struct Child0 {
        pub child_0: ::xmlity::XmlValue,
    }
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::bon::Builder,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
#[xelement(
    name = "documentation",
    namespace = "http://www.w3.org/2001/XMLSchema",
    allow_unknown_attributes = "any"
)]
pub struct Documentation {
    #[xattribute(name = "source", optional, default)]
    pub source: ::core::option::Option<crate::xs::types::TargetNamespace>,
    #[xattribute(deferred = true, optional, default)]
    pub attribute_1: ::core::option::Option<crate::xml::Lang>,
    #[xvalue(default)]
    #[builder(default)]
    pub particle: ::std::vec::Vec<documentation_items::Child0>,
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
#[xelement(
    name = "element",
    namespace = "http://www.w3.org/2001/XMLSchema",
    allow_unknown_attributes = "any"
)]
pub struct Element(#[xgroup] pub ::std::boxed::Box<crate::xs::types::TopLevelElement>);
impl ::core::convert::From<crate::xs::types::TopLevelElement> for Element {
    fn from(value: crate::xs::types::TopLevelElement) -> Self {
        Element(::std::boxed::Box::new(value))
    }
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
#[xelement(
    name = "enumeration",
    namespace = "http://www.w3.org/2001/XMLSchema",
    allow_unknown_attributes = "any"
)]
pub struct Enumeration(#[xgroup] pub ::std::boxed::Box<crate::xs::types::NoFixedFacet>);
impl ::core::convert::From<crate::xs::types::NoFixedFacet> for Enumeration {
    fn from(value: crate::xs::types::NoFixedFacet) -> Self {
        Enumeration(::std::boxed::Box::new(value))
    }
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::bon::Builder,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
#[xelement(
    name = "explicitTimezone",
    namespace = "http://www.w3.org/2001/XMLSchema",
    allow_unknown_attributes = "any",
    children_order = "strict"
)]
pub struct ExplicitTimezone {
    #[xattribute(name = "id", optional, default)]
    pub id: ::core::option::Option<String>,
    #[xattribute(name = "value")]
    pub value: String,
    #[xattribute(name = "fixed", optional, default)]
    pub fixed: ::core::option::Option<bool>,
    #[xvalue(default)]
    pub annotation: ::core::option::Option<crate::xs::Annotation>,
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::bon::Builder,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
#[xelement(
    name = "field",
    namespace = "http://www.w3.org/2001/XMLSchema",
    allow_unknown_attributes = "any",
    children_order = "strict"
)]
pub struct Field {
    #[xattribute(name = "id", optional, default)]
    pub id: ::core::option::Option<String>,
    #[xattribute(name = "xpath")]
    pub xpath: String,
    #[xattribute(name = "xpathDefaultNamespace", optional, default)]
    pub xpath_default_namespace: ::core::option::Option<
        crate::xs::types::XpathDefaultNamespaceType,
    >,
    #[xvalue(default)]
    pub annotation: ::core::option::Option<crate::xs::Annotation>,
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
#[xelement(
    name = "fractionDigits",
    namespace = "http://www.w3.org/2001/XMLSchema",
    allow_unknown_attributes = "any"
)]
pub struct FractionDigits(#[xgroup] pub ::std::boxed::Box<crate::xs::types::NumFacet>);
impl ::core::convert::From<crate::xs::types::NumFacet> for FractionDigits {
    fn from(value: crate::xs::types::NumFacet) -> Self {
        FractionDigits(::std::boxed::Box::new(value))
    }
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
#[xelement(
    name = "group",
    namespace = "http://www.w3.org/2001/XMLSchema",
    allow_unknown_attributes = "any"
)]
pub struct Group(#[xgroup] pub ::std::boxed::Box<crate::xs::types::NamedGroup>);
impl ::core::convert::From<crate::xs::types::NamedGroup> for Group {
    fn from(value: crate::xs::types::NamedGroup) -> Self {
        Group(::std::boxed::Box::new(value))
    }
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::bon::Builder,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
#[xelement(
    name = "import",
    namespace = "http://www.w3.org/2001/XMLSchema",
    allow_unknown_attributes = "any",
    children_order = "strict"
)]
pub struct Import {
    #[xattribute(name = "id", optional, default)]
    pub id: ::core::option::Option<String>,
    #[xattribute(name = "namespace", optional, default)]
    pub namespace: ::core::option::Option<crate::xs::types::TargetNamespace>,
    #[xattribute(name = "schemaLocation", optional, default)]
    pub schema_location: ::core::option::Option<crate::xs::types::TargetNamespace>,
    #[xvalue(default)]
    pub annotation: ::core::option::Option<crate::xs::Annotation>,
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::bon::Builder,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
#[xelement(
    name = "include",
    namespace = "http://www.w3.org/2001/XMLSchema",
    allow_unknown_attributes = "any",
    children_order = "strict"
)]
pub struct Include {
    #[xattribute(name = "id", optional, default)]
    pub id: ::core::option::Option<String>,
    #[xattribute(name = "schemaLocation")]
    pub schema_location: crate::xs::types::TargetNamespace,
    #[xvalue(default)]
    pub annotation: ::core::option::Option<crate::xs::Annotation>,
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
#[xelement(
    name = "key",
    namespace = "http://www.w3.org/2001/XMLSchema",
    allow_unknown_attributes = "any"
)]
pub struct Key(#[xgroup] pub ::std::boxed::Box<crate::xs::types::Keybase>);
impl ::core::convert::From<crate::xs::types::Keybase> for Key {
    fn from(value: crate::xs::types::Keybase) -> Self {
        Key(::std::boxed::Box::new(value))
    }
}
pub mod keyref_items {
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::bon::Builder,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xvalue(order = "strict")]
    pub struct Child1 {
        pub selector: ::std::boxed::Box<crate::xs::Selector>,
        #[xvalue(default)]
        #[builder(default)]
        pub field: ::std::vec::Vec<crate::xs::Field>,
    }
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::bon::Builder,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
#[xelement(
    name = "keyref",
    namespace = "http://www.w3.org/2001/XMLSchema",
    allow_unknown_attributes = "any",
    children_order = "strict"
)]
pub struct Keyref {
    #[xattribute(name = "id", optional, default)]
    pub id: ::core::option::Option<String>,
    #[xattribute(name = "name", optional, default)]
    pub name: ::core::option::Option<::xmlity::LocalName<'static>>,
    #[xattribute(name = "ref", optional, default)]
    pub ref_: ::core::option::Option<crate::xs::types::QName>,
    #[xattribute(name = "refer", optional, default)]
    pub refer: ::core::option::Option<crate::xs::types::QName>,
    #[xvalue(default)]
    pub annotation: ::core::option::Option<crate::xs::Annotation>,
    #[xvalue(default)]
    pub child_1: ::core::option::Option<keyref_items::Child1>,
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
#[xelement(
    name = "length",
    namespace = "http://www.w3.org/2001/XMLSchema",
    allow_unknown_attributes = "any"
)]
pub struct Length(#[xgroup] pub ::std::boxed::Box<crate::xs::types::NumFacet>);
impl ::core::convert::From<crate::xs::types::NumFacet> for Length {
    fn from(value: crate::xs::types::NumFacet) -> Self {
        Length(::std::boxed::Box::new(value))
    }
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::bon::Builder,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
#[xelement(
    name = "list",
    namespace = "http://www.w3.org/2001/XMLSchema",
    allow_unknown_attributes = "any",
    children_order = "strict"
)]
pub struct List {
    #[xattribute(name = "id", optional, default)]
    pub id: ::core::option::Option<String>,
    #[xattribute(name = "itemType", optional, default)]
    pub item_type: ::core::option::Option<crate::xs::types::QName>,
    #[xvalue(default)]
    pub annotation: ::core::option::Option<crate::xs::Annotation>,
    #[xelement(
        name = "simpleType",
        namespace = "http://www.w3.org/2001/XMLSchema",
        group,
        optional
    )]
    pub simple_type: ::core::option::Option<
        ::std::boxed::Box<crate::xs::types::LocalSimpleType>,
    >,
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
#[xelement(
    name = "maxExclusive",
    namespace = "http://www.w3.org/2001/XMLSchema",
    allow_unknown_attributes = "any"
)]
pub struct MaxExclusive(#[xgroup] pub ::std::boxed::Box<crate::xs::types::Facet>);
impl ::core::convert::From<crate::xs::types::Facet> for MaxExclusive {
    fn from(value: crate::xs::types::Facet) -> Self {
        MaxExclusive(::std::boxed::Box::new(value))
    }
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
#[xelement(
    name = "maxInclusive",
    namespace = "http://www.w3.org/2001/XMLSchema",
    allow_unknown_attributes = "any"
)]
pub struct MaxInclusive(#[xgroup] pub ::std::boxed::Box<crate::xs::types::Facet>);
impl ::core::convert::From<crate::xs::types::Facet> for MaxInclusive {
    fn from(value: crate::xs::types::Facet) -> Self {
        MaxInclusive(::std::boxed::Box::new(value))
    }
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
#[xelement(
    name = "maxLength",
    namespace = "http://www.w3.org/2001/XMLSchema",
    allow_unknown_attributes = "any"
)]
pub struct MaxLength(#[xgroup] pub ::std::boxed::Box<crate::xs::types::NumFacet>);
impl ::core::convert::From<crate::xs::types::NumFacet> for MaxLength {
    fn from(value: crate::xs::types::NumFacet) -> Self {
        MaxLength(::std::boxed::Box::new(value))
    }
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
#[xelement(
    name = "minExclusive",
    namespace = "http://www.w3.org/2001/XMLSchema",
    allow_unknown_attributes = "any"
)]
pub struct MinExclusive(#[xgroup] pub ::std::boxed::Box<crate::xs::types::Facet>);
impl ::core::convert::From<crate::xs::types::Facet> for MinExclusive {
    fn from(value: crate::xs::types::Facet) -> Self {
        MinExclusive(::std::boxed::Box::new(value))
    }
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
#[xelement(
    name = "minInclusive",
    namespace = "http://www.w3.org/2001/XMLSchema",
    allow_unknown_attributes = "any"
)]
pub struct MinInclusive(#[xgroup] pub ::std::boxed::Box<crate::xs::types::Facet>);
impl ::core::convert::From<crate::xs::types::Facet> for MinInclusive {
    fn from(value: crate::xs::types::Facet) -> Self {
        MinInclusive(::std::boxed::Box::new(value))
    }
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
#[xelement(
    name = "minLength",
    namespace = "http://www.w3.org/2001/XMLSchema",
    allow_unknown_attributes = "any"
)]
pub struct MinLength(#[xgroup] pub ::std::boxed::Box<crate::xs::types::NumFacet>);
impl ::core::convert::From<crate::xs::types::NumFacet> for MinLength {
    fn from(value: crate::xs::types::NumFacet) -> Self {
        MinLength(::std::boxed::Box::new(value))
    }
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::bon::Builder,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
#[xelement(
    name = "notation",
    namespace = "http://www.w3.org/2001/XMLSchema",
    allow_unknown_attributes = "any",
    children_order = "strict"
)]
pub struct Notation {
    #[xattribute(name = "id", optional, default)]
    pub id: ::core::option::Option<String>,
    #[xattribute(name = "name")]
    pub name: ::xmlity::LocalName<'static>,
    #[xattribute(name = "public", optional, default)]
    pub public: ::core::option::Option<String>,
    #[xattribute(name = "system", optional, default)]
    pub system: ::core::option::Option<crate::xs::types::TargetNamespace>,
    #[xvalue(default)]
    pub annotation: ::core::option::Option<crate::xs::Annotation>,
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::bon::Builder,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
#[xelement(
    name = "openContent",
    namespace = "http://www.w3.org/2001/XMLSchema",
    allow_unknown_attributes = "any",
    children_order = "strict"
)]
pub struct OpenContent {
    #[xattribute(name = "id", optional, default)]
    pub id: ::core::option::Option<String>,
    #[xattribute(name = "mode", optional, default)]
    pub mode: ::core::option::Option<String>,
    #[xvalue(default)]
    pub annotation: ::core::option::Option<crate::xs::Annotation>,
    #[xelement(
        name = "any",
        namespace = "http://www.w3.org/2001/XMLSchema",
        group,
        optional
    )]
    pub any: ::core::option::Option<::std::boxed::Box<crate::xs::types::Wildcard>>,
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::bon::Builder,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
#[xelement(
    name = "override",
    namespace = "http://www.w3.org/2001/XMLSchema",
    allow_unknown_attributes = "any",
    children_order = "strict"
)]
pub struct Override {
    #[xattribute(name = "schemaLocation")]
    pub schema_location: crate::xs::types::TargetNamespace,
    #[xattribute(name = "id", optional, default)]
    pub id: ::core::option::Option<String>,
    #[xvalue(default)]
    pub annotation: ::core::option::Option<crate::xs::Annotation>,
    #[xvalue(default)]
    #[builder(default)]
    pub schema_top: ::std::vec::Vec<crate::xs::groups::SchemaTop>,
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::bon::Builder,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
#[xelement(
    name = "pattern",
    namespace = "http://www.w3.org/2001/XMLSchema",
    allow_unknown_attributes = "any",
    children_order = "strict"
)]
pub struct Pattern {
    #[xattribute(name = "id", optional, default)]
    pub id: ::core::option::Option<String>,
    #[xattribute(name = "value")]
    pub value: String,
    #[xvalue(default)]
    pub annotation: ::core::option::Option<crate::xs::Annotation>,
}
pub mod redefine_items {
    impl ::core::convert::From<crate::xs::Annotation> for Redefine {
        fn from(value: crate::xs::Annotation) -> Self {
            Redefine::Annotation(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xs::groups::Redefinable> for Redefine {
        fn from(value: crate::xs::groups::Redefinable) -> Self {
            Redefine::Redefinable(::std::boxed::Box::new(value))
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub enum Redefine {
        Annotation(::std::boxed::Box<crate::xs::Annotation>),
        Redefinable(::std::boxed::Box<crate::xs::groups::Redefinable>),
    }
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::bon::Builder,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
#[xelement(
    name = "redefine",
    namespace = "http://www.w3.org/2001/XMLSchema",
    allow_unknown_attributes = "any"
)]
pub struct Redefine {
    #[xattribute(name = "schemaLocation")]
    pub schema_location: crate::xs::types::TargetNamespace,
    #[xattribute(name = "id", optional, default)]
    pub id: ::core::option::Option<String>,
    #[xvalue(default)]
    #[builder(default)]
    pub redefine: ::std::vec::Vec<redefine_items::Redefine>,
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::bon::Builder,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
#[xelement(
    name = "restriction",
    namespace = "http://www.w3.org/2001/XMLSchema",
    allow_unknown_attributes = "any",
    children_order = "strict"
)]
pub struct Restriction {
    #[xattribute(name = "id", optional, default)]
    pub id: ::core::option::Option<String>,
    #[xattribute(name = "base", optional, default)]
    pub base: ::core::option::Option<crate::xs::types::QName>,
    #[xvalue(default)]
    pub annotation: ::core::option::Option<crate::xs::Annotation>,
    pub simple_restriction_model: crate::xs::groups::SimpleRestrictionModel,
}
pub mod schema_items {
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::bon::Builder,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xvalue(order = "strict")]
    pub struct Child1 {
        pub default_open_content: crate::xs::DefaultOpenContent,
        #[xvalue(default)]
        #[builder(default)]
        pub annotation: ::std::vec::Vec<crate::xs::Annotation>,
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::bon::Builder,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xvalue(order = "strict")]
    pub struct Child2 {
        pub schema_top: crate::xs::groups::SchemaTop,
        #[xvalue(default)]
        #[builder(default)]
        pub annotation: ::std::vec::Vec<crate::xs::Annotation>,
    }
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::bon::Builder,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
#[xelement(
    name = "schema",
    namespace = "http://www.w3.org/2001/XMLSchema",
    allow_unknown_attributes = "any",
    children_order = "strict"
)]
pub struct Schema {
    #[xattribute(name = "targetNamespace", optional, default)]
    pub target_namespace: ::core::option::Option<crate::xs::types::TargetNamespace>,
    #[xattribute(name = "version", optional, default)]
    pub version: ::core::option::Option<String>,
    #[xattribute(name = "finalDefault", optional, default)]
    pub final_default: ::core::option::Option<crate::xs::types::FullDerivationSetType>,
    #[xattribute(name = "blockDefault", optional, default)]
    pub block_default: ::core::option::Option<crate::xs::types::BlockSetType>,
    #[xattribute(name = "attributeFormDefault", optional, default)]
    pub attribute_form_default: ::core::option::Option<crate::xs::types::FormChoiceType>,
    #[xattribute(name = "elementFormDefault", optional, default)]
    pub element_form_default: ::core::option::Option<crate::xs::types::FormChoiceType>,
    #[xattribute(name = "defaultAttributes", optional, default)]
    pub default_attributes: ::core::option::Option<crate::xs::types::QName>,
    #[xattribute(name = "xpathDefaultNamespace", optional, default)]
    pub xpath_default_namespace: ::core::option::Option<
        crate::xs::types::XpathDefaultNamespaceType,
    >,
    #[xattribute(name = "id", optional, default)]
    pub id: ::core::option::Option<String>,
    #[xattribute(deferred = true, optional, default)]
    pub attribute_9: ::core::option::Option<crate::xml::Lang>,
    #[xvalue(default)]
    #[builder(default)]
    pub composition: ::std::vec::Vec<crate::xs::groups::Composition>,
    #[xvalue(default)]
    pub child_1: ::core::option::Option<schema_items::Child1>,
    #[xvalue(default)]
    #[builder(default)]
    pub child_2: ::std::vec::Vec<schema_items::Child2>,
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::bon::Builder,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
#[xelement(
    name = "selector",
    namespace = "http://www.w3.org/2001/XMLSchema",
    allow_unknown_attributes = "any",
    children_order = "strict"
)]
pub struct Selector {
    #[xattribute(name = "id", optional, default)]
    pub id: ::core::option::Option<String>,
    #[xattribute(name = "xpath")]
    pub xpath: String,
    #[xattribute(name = "xpathDefaultNamespace", optional, default)]
    pub xpath_default_namespace: ::core::option::Option<
        crate::xs::types::XpathDefaultNamespaceType,
    >,
    #[xvalue(default)]
    pub annotation: ::core::option::Option<crate::xs::Annotation>,
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
#[xelement(
    name = "sequence",
    namespace = "http://www.w3.org/2001/XMLSchema",
    allow_unknown_attributes = "any"
)]
pub struct Sequence(#[xgroup] pub ::std::boxed::Box<crate::xs::types::ExplicitGroup>);
impl ::core::convert::From<crate::xs::types::ExplicitGroup> for Sequence {
    fn from(value: crate::xs::types::ExplicitGroup) -> Self {
        Sequence(::std::boxed::Box::new(value))
    }
}
pub mod simple_content_items {
    impl ::core::convert::From<crate::xs::types::SimpleRestrictionType> for Child1 {
        fn from(value: crate::xs::types::SimpleRestrictionType) -> Self {
            Child1::Restriction(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xs::types::SimpleExtensionType> for Child1 {
        fn from(value: crate::xs::types::SimpleExtensionType) -> Self {
            Child1::Extension(::std::boxed::Box::new(value))
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub enum Child1 {
        #[xelement(
            name = "restriction",
            namespace = "http://www.w3.org/2001/XMLSchema",
            allow_unknown_attributes = "any"
        )]
        Restriction(
            #[xgroup]
            ::std::boxed::Box<crate::xs::types::SimpleRestrictionType>,
        ),
        #[xelement(
            name = "extension",
            namespace = "http://www.w3.org/2001/XMLSchema",
            allow_unknown_attributes = "any"
        )]
        Extension(#[xgroup] ::std::boxed::Box<crate::xs::types::SimpleExtensionType>),
    }
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::bon::Builder,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
#[xelement(
    name = "simpleContent",
    namespace = "http://www.w3.org/2001/XMLSchema",
    allow_unknown_attributes = "any",
    children_order = "strict"
)]
pub struct SimpleContent {
    #[xattribute(name = "id", optional, default)]
    pub id: ::core::option::Option<String>,
    #[xvalue(default)]
    pub annotation: ::core::option::Option<crate::xs::Annotation>,
    pub child_1: simple_content_items::Child1,
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
#[xelement(
    name = "simpleType",
    namespace = "http://www.w3.org/2001/XMLSchema",
    allow_unknown_attributes = "any"
)]
pub struct SimpleType(
    #[xgroup]
    pub ::std::boxed::Box<crate::xs::types::TopLevelSimpleType>,
);
impl ::core::convert::From<crate::xs::types::TopLevelSimpleType> for SimpleType {
    fn from(value: crate::xs::types::TopLevelSimpleType) -> Self {
        SimpleType(::std::boxed::Box::new(value))
    }
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::bon::Builder,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
#[xelement(
    name = "totalDigits",
    namespace = "http://www.w3.org/2001/XMLSchema",
    allow_unknown_attributes = "any",
    children_order = "strict"
)]
pub struct TotalDigits {
    #[xattribute(name = "id", optional, default)]
    pub id: ::core::option::Option<String>,
    #[xattribute(name = "value")]
    pub value: usize,
    #[xattribute(name = "fixed", optional, default)]
    pub fixed: ::core::option::Option<bool>,
    #[xvalue(default)]
    pub annotation: ::core::option::Option<crate::xs::Annotation>,
}
pub mod union_items {
    impl ::core::convert::From<crate::xs::types::LocalSimpleType> for SimpleType {
        fn from(value: crate::xs::types::LocalSimpleType) -> Self {
            SimpleType(::std::boxed::Box::new(value))
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xelement(
        name = "simpleType",
        namespace = "http://www.w3.org/2001/XMLSchema",
        allow_unknown_attributes = "any"
    )]
    pub struct SimpleType(
        #[xgroup]
        pub ::std::boxed::Box<crate::xs::types::LocalSimpleType>,
    );
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::bon::Builder,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
#[xelement(
    name = "union",
    namespace = "http://www.w3.org/2001/XMLSchema",
    allow_unknown_attributes = "any",
    children_order = "strict"
)]
pub struct Union {
    #[xattribute(name = "id", optional, default)]
    pub id: ::core::option::Option<String>,
    #[xattribute(name = "memberTypes", optional, default)]
    pub member_types: ::core::option::Option<String>,
    #[xvalue(default)]
    pub annotation: ::core::option::Option<crate::xs::Annotation>,
    #[xvalue(default)]
    #[builder(default)]
    pub simple_type: ::std::vec::Vec<union_items::SimpleType>,
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
#[xelement(
    name = "unique",
    namespace = "http://www.w3.org/2001/XMLSchema",
    allow_unknown_attributes = "any"
)]
pub struct Unique(#[xgroup] pub ::std::boxed::Box<crate::xs::types::Keybase>);
impl ::core::convert::From<crate::xs::types::Keybase> for Unique {
    fn from(value: crate::xs::types::Keybase) -> Self {
        Unique(::std::boxed::Box::new(value))
    }
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::bon::Builder,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
#[xelement(
    name = "whiteSpace",
    namespace = "http://www.w3.org/2001/XMLSchema",
    allow_unknown_attributes = "any",
    children_order = "strict"
)]
pub struct WhiteSpace {
    #[xattribute(name = "id", optional, default)]
    pub id: ::core::option::Option<String>,
    #[xattribute(name = "value")]
    pub value: String,
    #[xattribute(name = "fixed", optional, default)]
    pub fixed: ::core::option::Option<bool>,
    #[xvalue(default)]
    pub annotation: ::core::option::Option<crate::xs::Annotation>,
}
