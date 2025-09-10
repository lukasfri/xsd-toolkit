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

    pub const fn mark_changed(&mut self) {
        *self = Self::Changed;
    }

    pub const fn is_changed(&self) -> bool {
        matches!(self, Self::Changed)
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
        iter.into_iter().fold(Self::new(), |acc, item| acc | item)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_transform_change() {
        let change1 = TransformChange::Changed;
        let change2 = TransformChange::Unchanged;

        assert!(change1 | change2 == TransformChange::Changed);
        assert!(change1 | change1 == TransformChange::Changed);
        assert!(change2 | change2 == TransformChange::Unchanged);
    }

    #[test]
    fn test_transform_change_from_bool() {
        assert!(TransformChange::from(true) == TransformChange::Changed);
        assert!(TransformChange::from(false) == TransformChange::Unchanged);
    }

    #[test]
    fn test_transform_change_from_iter() {
        let changes: Vec<TransformChange> =
            vec![TransformChange::Changed, TransformChange::Unchanged];
        let combined_change: TransformChange = changes.into_iter().collect();
        assert!(combined_change == TransformChange::Changed);
    }
}
