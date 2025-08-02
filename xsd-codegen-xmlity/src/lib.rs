pub mod augments;
pub mod binds;
mod complex;
pub mod misc;
mod naming_strategies;
mod simple;
pub mod templates;
mod transformer;
pub use transformer::{CodegenTransformer, Error as CodegenTransformerError};
mod generator;
pub use generator::{Generator, GeneratorContext, GeneratorScope};

use inflector::Inflector;
use misc::TypeReference;
use quote::format_ident;
use syn::Ident;
use xmlity::{ExpandedName, LocalName, XmlNamespace};

use crate::augments::ItemAugmentation;

#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    MissingNamespace {
        namespace: XmlNamespace<'static>,
    },
    NoNamespace,
    MissingElement {
        name: ExpandedName<'static>,
    },
    MissingAttribute {
        name: ExpandedName<'static>,
    },
    MissingGroup {
        name: ExpandedName<'static>,
    },
    MissingType {
        name: ExpandedName<'static>,
    },
    UnsupportedFragment {
        fragment: String,
    },
    UnsupportedSimpleBase {
        base: Option<ExpandedName<'static>>,
    },
    HandlerDoesNotExist {
        origin: &'static str,
        handler: &'static str,
    },
    FragmentNotFound {
        fragment_type: String,
    },
    UnboundNamespace {
        namespace: Option<XmlNamespace<'static>>,
        item_name: Option<String>,
    },
    UnsupportedItemType {
        item_type: String,
    },
    UnsupportedSchemaFeature(UnsupportedSchemaFeature),
}

/// Represents various XML Schema features that are not yet supported
/// by the fragmented XML schema code generation.
///
/// This enum categorizes unsupported XML Schema features that
/// the XSD toolkit cannot currently process. When the code generator encounters
/// one of these features, it should return an `Error::UnsupportedSchemaFeature`
/// with the appropriate variant.
///
/// # Future Work
///
/// As features are implemented, their corresponding variants should be removed
/// from this enum. This provides a clear roadmap of what remains to be implemented
/// for full XML Schema compliance.
///
/// It is however not expected for this enum to ever be empty, as there will always be
/// some features that need to be processed into another form for the code generation to work.
#[derive(Debug, Clone, PartialEq)]
pub enum UnsupportedSchemaFeature {
    /// Prohibited attributes are not supported in attribute declarations
    ProhibitedAttributes,
    /// Simple content restrictions are not supported
    SimpleContentRestriction,
    /// Mixed content models are not supported
    MixedContent,
    /// Dynamic facets are not supported in simple types
    DynamicFacets,
}

impl UnsupportedSchemaFeature {
    /// Get a human-readable description of the unsupported feature
    pub fn description(&self) -> &'static str {
        match self {
            UnsupportedSchemaFeature::ProhibitedAttributes => {
                "Prohibited attributes are not supported in attribute declarations. They should be removed during transformation."
            }
            UnsupportedSchemaFeature::SimpleContentRestriction => {
                "Simple content restrictions are not supported."
            }
            UnsupportedSchemaFeature::MixedContent => "Mixed content models are not supported.",
            UnsupportedSchemaFeature::DynamicFacets => {
                "Dynamic facets are not supported in simple types."
            }
        }
    }
}

impl std::fmt::Display for UnsupportedSchemaFeature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::MissingNamespace { namespace } => {
                write!(f, "Missing namespace: {}", namespace)
            }
            Error::NoNamespace => write!(f, "No namespace specified"),
            Error::MissingElement { name } => write!(f, "Missing element: {}", name),
            Error::MissingAttribute { name } => write!(f, "Missing attribute: {}", name),
            Error::MissingGroup { name } => write!(f, "Missing group: {}", name),
            Error::MissingType { name } => write!(f, "Missing type: {}", name),
            Error::UnsupportedFragment { fragment } => {
                write!(f, "Unsupported fragment: {}", fragment)
            }
            Error::UnsupportedSimpleBase { base } => {
                write!(f, "Unsupported simple base: {:?}", base)
            }
            Error::HandlerDoesNotExist { origin, handler } => {
                write!(f, "Handler '{}' does not exist in '{}'", handler, origin)
            }
            Error::FragmentNotFound { fragment_type } => {
                write!(f, "Fragment not found: {}", fragment_type)
            }
            Error::UnboundNamespace {
                namespace,
                item_name,
            } => {
                write!(
                    f,
                    "Unbound namespace: {:?} for item: {:?}",
                    namespace, item_name
                )
            }
            Error::UnsupportedItemType { item_type } => {
                write!(f, "Unsupported item type: {}", item_type)
            }
            Error::UnsupportedSchemaFeature(feature) => {
                write!(f, "Unsupported schema feature: {}", feature)
            }
        }
    }
}

impl std::error::Error for Error {}

impl Error {
    /// Create an error for an unsupported schema feature
    pub fn unsupported_feature(feature: UnsupportedSchemaFeature) -> Self {
        Error::UnsupportedSchemaFeature(feature)
    }

    /// Create an error for prohibited attributes
    pub fn prohibited_attributes() -> Self {
        Error::UnsupportedSchemaFeature(UnsupportedSchemaFeature::ProhibitedAttributes)
    }

    /// Create an error for simple content restriction
    pub fn simple_content_restriction() -> Self {
        Error::UnsupportedSchemaFeature(UnsupportedSchemaFeature::SimpleContentRestriction)
    }

    /// Create an error for mixed content
    pub fn mixed_content() -> Self {
        Error::UnsupportedSchemaFeature(UnsupportedSchemaFeature::MixedContent)
    }

    /// Create an error for dynamic facets
    pub fn dynamic_facets() -> Self {
        Error::UnsupportedSchemaFeature(UnsupportedSchemaFeature::DynamicFacets)
    }
}
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TypeType {
    Simple,
    Complex,
}

pub trait ToIdentTypesExt {
    fn to_item_ident(&self) -> Ident;
    fn to_field_ident(&self) -> Ident;
    fn to_variant_ident(&self) -> Ident;
    fn to_path_ident(&self) -> Ident;
}

impl ToIdentTypesExt for LocalName<'_> {
    fn to_item_ident(&self) -> Ident {
        format_ident!(
            "{}",
            misc::unkeywordify(self.to_string().to_pascal_case().as_str())
        )
    }
    fn to_field_ident(&self) -> Ident {
        format_ident!(
            "{}",
            misc::unkeywordify(self.to_string().to_snake_case().as_str())
        )
    }

    fn to_variant_ident(&self) -> Ident {
        format_ident!(
            "{}",
            misc::unkeywordify(self.to_string().to_pascal_case().as_str())
        )
    }

    fn to_path_ident(&self) -> Ident {
        format_ident!(
            "{}",
            misc::unkeywordify(self.to_string().to_snake_case().as_str())
        )
    }
}

impl ToIdentTypesExt for Ident {
    fn to_item_ident(&self) -> Ident {
        format_ident!(
            "{}",
            misc::unkeywordify(self.to_string().to_pascal_case().as_str())
        )
    }

    fn to_field_ident(&self) -> Ident {
        format_ident!(
            "{}",
            misc::unkeywordify(self.to_string().to_snake_case().as_str())
        )
    }

    fn to_variant_ident(&self) -> Ident {
        format_ident!(
            "{}",
            misc::unkeywordify(self.to_string().to_pascal_case().as_str())
        )
    }

    fn to_path_ident(&self) -> Ident {
        format_ident!(
            "{}",
            misc::unkeywordify(self.to_string().to_snake_case().as_str())
        )
    }
}

pub trait Scope {
    fn add_item<I: Into<syn::Item>>(&mut self, item: I) -> Result<TypeReference<'static>>;

    fn add_raw_items<I: IntoIterator<Item = J>, J: Into<syn::Item>>(&mut self, items: I);

    fn augmenter(&self) -> &dyn ItemAugmentation;
}

#[derive(Debug, Clone)]
pub struct BoundType {
    pub ty: TypeReference<'static>,
    pub ty_type: TypeType,
    pub serialize_with: Option<syn::Path>,
    pub deserialize_with: Option<syn::Path>,
}

pub struct ToTypeTemplateData<T> {
    pub ident: Option<Ident>,
    pub template: T,
}
