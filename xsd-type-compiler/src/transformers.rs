use crate::complex::{self, ComplexTypeRootFragment, FragmentAccess};
use crate::simple::{FragmentId as SimpleFragmentId, SimpleTypeFragment};
use crate::{CompiledNamespace, ComplexTypeIdent};

use xmlity::ExpandedName;

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

/// This transformer type is only capable of doing local transformations within a namespace, and cannot access other namespaces.
///
/// It is useful for things like expanding extension fragments, or resolving local references, but not for things like resolving global references.
pub trait XmlnsLocalTransformer {
    type Error: std::fmt::Debug;

    /// Returns true if the context was changed.
    fn transform(
        self,
        context: XmlnsLocalTransformerContext<'_>,
    ) -> Result<TransformChange, Self::Error>;
}

pub struct XmlnsLocalTransformerContext<'a> {
    // pub namespace: &'a XmlNamespace<'static>,
    pub namespace: &'a mut CompiledNamespace,
    // pub xmlns_context: &'a mut crate::XmlnsContext,
}

impl XmlnsLocalTransformerContext<'_> {
    // pub fn context(&self) -> &crate::XmlnsContext {
    //     self.xmlns_context
    // }

    // pub fn context_mut(&mut self) -> &mut crate::XmlnsContext {
    //     self.xmlns_context
    // }

    pub fn current_namespace(&self) -> &crate::CompiledNamespace {
        &self.namespace
    }

    pub fn current_namespace_mut(&mut self) -> &mut crate::CompiledNamespace {
        self.namespace
    }

    pub fn iter_complex_fragment_ids<F: 'static>(&self) -> Vec<complex::FragmentIdx<F>>
    where
        complex::ComplexTypeFragmentCompiler: complex::FragmentAccess<F>,
    {
        self.current_namespace().complex_type.iter_fragment_ids()
    }

    pub fn get_complex_fragment<F>(&self, fragment_idx: &complex::FragmentIdx<F>) -> Option<&F>
    where
        complex::ComplexTypeFragmentCompiler: complex::FragmentAccess<F>,
    {
        self.current_namespace()
            .complex_type
            .get_fragment(fragment_idx)
    }

    pub fn get_complex_fragment_mut<F>(
        &mut self,
        fragment_idx: &complex::FragmentIdx<F>,
    ) -> Option<&mut F>
    where
        complex::ComplexTypeFragmentCompiler: complex::FragmentAccess<F>,
    {
        self.current_namespace_mut()
            .complex_type
            .get_fragment_mut(fragment_idx)
    }

    pub fn iter_simple_fragment_ids(&self) -> impl Iterator<Item = SimpleFragmentId> {
        self.current_namespace()
            .complex_type
            .simple_type_fragments
            .iter_fragment_ids()
    }

    pub fn get_simple_fragment(
        &self,
        fragment_idx: &SimpleFragmentId,
    ) -> Option<&SimpleTypeFragment> {
        self.current_namespace()
            .complex_type
            .simple_type_fragments
            .get_fragment(fragment_idx)
    }

    pub fn simple_fragment_mut(
        &mut self,
        fragment_idx: &SimpleFragmentId,
    ) -> Option<&mut SimpleTypeFragment> {
        self.current_namespace_mut()
            .complex_type
            .simple_type_fragments
            .get_fragment_mut(fragment_idx)
    }

    pub fn get_named_type<'a>(
        &'a self,
        name: &'a ExpandedName<'_>,
    ) -> Option<&'a crate::TopLevelType> {
        self.current_namespace()
            .top_level_types
            .get(name.local_name())
    }

    pub fn get_named_attribute_group<'a>(
        &'a self,
        name: &'a ExpandedName<'_>,
    ) -> Option<&'a crate::TopLevelAttributeGroup> {
        self.current_namespace()
            .top_level_attribute_groups
            .get(name.local_name())
    }

    pub fn get_complex_fragment_from_ident<'a, F>(
        &'a self,
        name: &'a ComplexTypeIdent,
    ) -> Option<&'a ComplexTypeRootFragment>
    where
        complex::ComplexTypeFragmentCompiler: complex::FragmentAccess<F>,
    {
        let fragment_id = match name {
            ComplexTypeIdent::Named(expanded_name) => {
                match self.get_named_type(expanded_name).unwrap() {
                    crate::TopLevelType::Complex(complex) => &complex.root_fragment,
                    crate::TopLevelType::Simple(_) => unreachable!(),
                }
            }
            ComplexTypeIdent::Anonymous(fragment_id) => fragment_id,
        };

        self.get_complex_fragment::<ComplexTypeRootFragment>(fragment_id)
    }
}
