pub mod context;
pub use context::{XmlnsContextTransformer, XmlnsContextTransformerContext};
pub mod local;
pub use local::{XmlnsLocalTransformer, XmlnsLocalTransformerContext};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TransformChange {
    Changed,
    #[default]
    Unchanged,
}
impl TransformChange {
    fn new() -> Self {
        Self::Unchanged
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
