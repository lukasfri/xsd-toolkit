//! This module provides the core functionality for transforming and managing XSD fragments.
//!
//! It includes two main transformer types: [`XmlnsContextTransformer`] and [`XmlnsLocalTransformer`], which are used to transform XSDs on both a set-wide and a local level.

pub mod context;
pub use context::{XmlnsContextTransformer, XmlnsContextTransformerContext};
pub mod local;
pub use local::{XmlnsLocalTransformer, XmlnsLocalTransformerContext};

/// The [`TransformChange`] enum is used to indicate whether a transformation has changed the context or not.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransformChange {
    /// Indicates that changes were made during the transformation.
    Changed,
    /// Indicates that no changes were made during the transformation.
    Unchanged,
}
impl TransformChange {
    /// Creates a new [`TransformChange`] instance indicating no changes.
    pub const fn new() -> Self {
        Self::Unchanged
    }
}

impl Default for TransformChange {
    fn default() -> Self {
        Self::new()
    }
}

impl From<bool> for TransformChange {
    fn from(value: bool) -> Self {
        if value {
            Self::Changed
        } else {
            Self::Unchanged
        }
    }
}

impl From<TransformChange> for bool {
    fn from(value: TransformChange) -> Self {
        value == TransformChange::Changed
    }
}

impl std::ops::BitOr for TransformChange {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        if self == Self::Changed || rhs == Self::Changed {
            Self::Changed
        } else {
            Self::Unchanged
        }
    }
}

impl std::ops::BitOrAssign for TransformChange {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = *self | rhs;
    }
}

impl FromIterator<TransformChange> for TransformChange {
    fn from_iter<T: IntoIterator<Item = TransformChange>>(iter: T) -> Self {
        let mut changed = Self::new();

        for item in iter {
            changed |= item;
        }

        changed
    }
}
