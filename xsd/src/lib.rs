//! This crate provides a wrapper around the XML Schema (XSD) definitions
//! from the `xmlity_ns_xs` crate, allowing for easier access and manipulation of
//! XML Schema components such as elements, attributes, and types.
//!
//! It includes methods to retrieve top-level elements, attributes, and types,
//! as well as to navigate through schema compositions like includes and imports.
//!
//! Furthermore, it provides you with a convenient way to load an XML schema set
//! from a URL tree through the [`set`] module.
use std::ops::Deref;

use xmlity::XmlNamespace;
pub use xmlity_ns_xs as xs;
/// XSD schema names and common type references.
pub mod xsn;
pub use xmlity_ns as ns;
mod link;
pub use link::UrlExt;
pub mod set;

/// Wrapper around an XML Schema definition.
#[derive(Debug, Clone, PartialEq)]
pub struct XmlSchema {
    /// The underlying XSD schema.
    pub underlying_schema: xs::Schema,
}

impl XmlSchema {
    /// Creates a new [`XmlSchema`] wrapper.
    pub fn new(underlying_schema: xs::Schema) -> Self {
        Self { underlying_schema }
    }

    /// Returns the schema definition.
    pub fn schema(&self) -> &xs::schema_items::Schema {
        match &self.underlying_schema {
            xmlity_ns_xs::Schema::Schema(schema) => schema,
            xmlity_ns_xs::Schema::Dynamic(_) => {
                panic!("Expected a schema, but found a substitution group:",)
            }
        }
    }

    /// Returns the target namespace of the schema.
    pub fn namespace(&self) -> Option<&xmlity::XmlNamespace> {
        self.schema()
            .target_namespace
            .as_deref()
            .map(XmlNamespace::new)
            .transpose()
            .expect("Failed to parse namespace")
    }

    /// Returns an iterator over schema compositions (imports, includes, etc.).
    pub fn compositions(&self) -> impl Iterator<Item = &xs::groups::Composition> {
        self.schema().composition.iter()
    }

    /// Returns an iterator over schema includes.
    pub fn includes(&self) -> impl Iterator<Item = &xs::Include> + use<'_> {
        self.compositions().filter_map(|c| match c {
            xs::groups::Composition::Include(include) => Some(include.deref()),
            _ => None,
        })
    }

    /// Returns an iterator over schema imports.
    pub fn imports(&self) -> impl Iterator<Item = &xs::Import> + use<'_> {
        self.compositions().filter_map(|c| match c {
            xs::groups::Composition::Import(import) => Some(import.deref()),
            _ => None,
        })
    }

    /// Returns an iterator over all top-level schema components.
    pub fn schema_tops(&self) -> impl Iterator<Item = &xs::groups::SchemaTop> {
        self.schema().child_2.iter().map(|a| &a.schema_top)
    }

    /// Returns an iterator over top-level element declarations.
    pub fn top_level_elements(&self) -> impl Iterator<Item = &xs::Element> {
        self.schema_tops().filter_map(|top| {
            if let xs::groups::SchemaTop::Element(element) = top {
                Some(element.deref())
            } else {
                None
            }
        })
    }

    /// Returns an iterator over top-level attribute declarations.
    pub fn top_level_attributes(&self) -> impl Iterator<Item = &xs::Attribute> {
        self.schema_tops().filter_map(|top| {
            if let xs::groups::SchemaTop::Attribute(attribute) = top {
                Some(attribute.deref())
            } else {
                None
            }
        })
    }

    /// Returns an iterator over redefinable schema components.
    pub fn redefinable(&self) -> impl Iterator<Item = &xs::groups::Redefinable> {
        self.schema_tops().filter_map(|top| {
            if let xs::groups::SchemaTop::Redefinable(redefinable) = top {
                Some(redefinable.deref())
            } else {
                None
            }
        })
    }

    /// Returns an iterator over top-level simple type definitions.
    pub fn top_level_simple_types(&self) -> impl Iterator<Item = &xs::SimpleType> {
        self.redefinable().filter_map(|re| {
            if let xs::groups::Redefinable::SimpleType(simple_type) = re {
                Some(simple_type.deref())
            } else {
                None
            }
        })
    }

    /// Returns an iterator over top-level complex type definitions.
    pub fn top_level_complex_types(&self) -> impl Iterator<Item = &xs::ComplexType> {
        self.redefinable().filter_map(|re| {
            if let xs::groups::Redefinable::ComplexType(complex_type) = re {
                Some(complex_type.deref())
            } else {
                None
            }
        })
    }
}
