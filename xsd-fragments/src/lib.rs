//! This crate provides a way to represent XSD fragments in a way that is easy to work with.
//! It is used to compile XSD schemas into a set of fragments that can be used to
//! generate code or to validate XML documents.
//!
//! The main entry point is the [`XmlnsContext`] struct, which is used to manage namespaces and their fragments.
//! The [`CompiledNamespace`] struct represents a compiled namespace, which contains all the fragments
//! for that namespace.
//! The fragments are represented by the [`FragmentIdx`] type, which is a wrapper around a usize that
//! allows for easy indexing of the fragments.
//! The fragments are divided into two main categories: simple and complex, matching the two sides of XSD.
//!
//! (Top level complex types)[`fragments::complex::ComplexTypeRootFragment`] and (top level simple types)[`fragments::simple::SimpleTypeRootFragment`] are represented by the same data structure as their
//! local counterparts when in fragment form due to ease of use when doing transformations and generation.

use xmlity::{ExpandedName, XmlNamespace};

pub mod fragments;

mod context;
mod namespace;
pub mod transformers;
pub use context::{FragmentedXsdDocumentKey, XmlnsContext};
pub use namespace::{
    FragmentedXsdDocument, TopLevelAttribute, TopLevelAttributeGroup, TopLevelComplexType,
    TopLevelElement, TopLevelGroup, TopLevelSimpleType, TopLevelType,
};

/// Error type used for when importing, exporting and otherwise interacting with namespaces.
#[derive(Debug, derive_more::derive::From, derive_more::derive::Display)]
pub enum Error {
    /// Tried to import an existing entity.
    #[display("Tried to import an existing entity")]
    ImportOfExistingEntity,
    /// Tried to import a namespace that does not exist.
    #[display("Tried to import a namespace that does not exist")]
    NonExistentXmlNamespace {
        /// The namespace that was attempted to be accessed.
        namespace: XmlNamespace<'static>,
    },
    /// Tried to use an undefined namespace.
    #[display("Tried to use an undefined namespace")]
    UndefinedNamespace,
    /// Tried to use an undefined fragment.
    #[display("Error when processing complex fragments")]
    ComplexFragmentError(fragments::complex::Error),
    /// Tried to use an undefined simple fragment.
    #[display("Error when processing simple fragments")]
    SimpleFragmentError(fragments::simple::Error),
}

/// A type that can be either a named or an anonymous type.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum NamedOrAnonymous<T> {
    /// A named type, represented by an [`ExpandedName`].
    Named(ExpandedName<'static>),
    /// An anonymous type, represented by a value of type `T`.
    Anonymous(T),
}
