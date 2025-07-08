pub mod complex;
pub mod simple;

use xsd_fragments::fragments::{complex as cx, simple as sm, FragmentIdx};
use xsd_fragments::fragments::{FragmentAccess, NamespaceIdx};
use xsd_fragments::CompiledNamespace;

use xmlity::{ExpandedName, LocalName, XmlNamespace};

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
        self.namespace
    }

    pub fn current_namespace_mut(&mut self) -> &mut crate::CompiledNamespace {
        self.namespace
    }

    pub fn iter_complex_fragment_ids<F: 'static>(&self) -> Vec<FragmentIdx<F>>
    where
        cx::ComplexTypeFragmentCompiler: FragmentAccess<F>,
    {
        self.current_namespace().complex_type.iter_fragment_ids()
    }

    pub fn get_complex_fragment<F>(&self, fragment_idx: &FragmentIdx<F>) -> Option<&F>
    where
        cx::ComplexTypeFragmentCompiler: FragmentAccess<F>,
    {
        self.current_namespace()
            .complex_type
            .get_fragment(fragment_idx)
    }

    pub fn get_complex_fragment_mut<F>(&mut self, fragment_idx: &FragmentIdx<F>) -> Option<&mut F>
    where
        cx::ComplexTypeFragmentCompiler: FragmentAccess<F>,
    {
        self.current_namespace_mut()
            .complex_type
            .get_fragment_mut(fragment_idx)
    }

    pub fn iter_simple_fragment_ids<F: 'static>(&self) -> Vec<FragmentIdx<F>>
    where
        sm::SimpleTypeFragmentCompiler: FragmentAccess<F>,
    {
        self.current_namespace()
            .complex_type
            .simple_type_fragments
            .iter_fragment_ids()
    }

    pub fn get_simple_fragment<F>(&self, fragment_idx: &FragmentIdx<F>) -> Option<&F>
    where
        sm::SimpleTypeFragmentCompiler: FragmentAccess<F>,
    {
        self.current_namespace()
            .complex_type
            .simple_type_fragments
            .get_fragment(fragment_idx)
    }

    pub fn get_simple_fragment_mut<F>(&mut self, fragment_idx: &FragmentIdx<F>) -> Option<&mut F>
    where
        sm::SimpleTypeFragmentCompiler: FragmentAccess<F>,
    {
        self.current_namespace_mut()
            .complex_type
            .simple_type_fragments
            .get_fragment_mut(fragment_idx)
    }

    pub fn get_named_type<'a>(
        &'a self,
        name: &'a LocalName<'_>,
    ) -> Option<&'a xsd_fragments::TopLevelType> {
        self.current_namespace().top_level_types.get(name)
    }

    pub fn get_named_attribute_group<'a>(
        &'a self,
        name: &'a LocalName<'_>,
    ) -> Option<&'a xsd_fragments::TopLevelAttributeGroup> {
        self.current_namespace()
            .top_level_attribute_groups
            .get(name)
    }
}

pub trait XmlnsContextTransformer {
    type Error: std::fmt::Debug;

    /// Returns true if the context was changed.
    fn transform(
        self,
        context: XmlnsContextTransformerContext<'_>,
    ) -> Result<TransformChange, Self::Error>;
}

pub struct XmlnsContextTransformerContext<'a> {
    pub xmlns_context: &'a mut xsd_fragments::XmlnsContext,
}

impl XmlnsContextTransformerContext<'_> {
    fn get_namespace(
        &self,
        namespace_idx: &crate::NamespaceIdx,
    ) -> Option<&crate::CompiledNamespace> {
        self.xmlns_context.namespaces.get(namespace_idx)
    }

    fn get_namespace_mut(
        &mut self,
        namespace_idx: &crate::NamespaceIdx,
    ) -> Option<&mut crate::CompiledNamespace> {
        self.xmlns_context.namespaces.get_mut(namespace_idx)
    }

    pub fn iter_complex_fragment_ids<F: 'static>(&self) -> impl Iterator<Item = FragmentIdx<F>> + '_
    where
        cx::ComplexTypeFragmentCompiler: FragmentAccess<F>,
    {
        self.xmlns_context
            .namespaces
            .iter()
            .flat_map(|(_, ns)| ns.complex_type.iter_fragment_ids())
    }

    pub fn get_complex_fragment<F>(&self, fragment_idx: &FragmentIdx<F>) -> Option<&F>
    where
        cx::ComplexTypeFragmentCompiler: FragmentAccess<F>,
    {
        self.get_namespace(&fragment_idx.namespace_idx())?
            .complex_type
            .get_fragment(fragment_idx)
    }

    pub fn get_complex_fragment_mut<F>(&mut self, fragment_idx: &FragmentIdx<F>) -> Option<&mut F>
    where
        cx::ComplexTypeFragmentCompiler: FragmentAccess<F>,
    {
        self.get_namespace_mut(&fragment_idx.namespace_idx())?
            .complex_type
            .get_fragment_mut(fragment_idx)
    }

    pub fn iter_simple_fragment_ids<F: 'static>(&self) -> impl Iterator<Item = FragmentIdx<F>> + '_
    where
        sm::SimpleTypeFragmentCompiler: FragmentAccess<F>,
    {
        self.xmlns_context
            .namespaces
            .iter()
            .flat_map(|(_, ns)| ns.complex_type.simple_type_fragments.iter_fragment_ids())
    }

    pub fn get_simple_fragment<F>(&self, fragment_idx: &FragmentIdx<F>) -> Option<&F>
    where
        sm::SimpleTypeFragmentCompiler: FragmentAccess<F>,
    {
        self.get_namespace(&fragment_idx.namespace_idx())?
            .complex_type
            .simple_type_fragments
            .get_fragment(fragment_idx)
    }

    pub fn get_simple_fragment_mut<F>(&mut self, fragment_idx: &FragmentIdx<F>) -> Option<&mut F>
    where
        sm::SimpleTypeFragmentCompiler: FragmentAccess<F>,
    {
        self.get_namespace_mut(&fragment_idx.namespace_idx())?
            .complex_type
            .simple_type_fragments
            .get_fragment_mut(fragment_idx)
    }

    pub fn get_named_type<'a>(
        &'a self,
        name: &'a ExpandedName<'_>,
    ) -> Option<&'a xsd_fragments::TopLevelType> {
        self.xmlns_context
            .get_namespace(name.namespace()?)?
            .top_level_types
            .get(name.local_name())
    }

    pub fn get_named_attribute_group<'a>(
        &'a self,
        name: &'a ExpandedName<'_>,
    ) -> Option<&'a xsd_fragments::TopLevelAttributeGroup> {
        self.xmlns_context
            .get_namespace(name.namespace()?)?
            .top_level_attribute_groups
            .get(name.local_name())
    }
}

pub trait XmlnsContextExt {
    fn local_transform<T: XmlnsLocalTransformer>(
        &mut self,
        namespace: &XmlNamespace<'_>,
        transformer: T,
    ) -> Result<TransformChange, T::Error>;

    fn local_transform_id<T: XmlnsLocalTransformer>(
        &mut self,
        namespace: &NamespaceIdx,
        transformer: T,
    ) -> Result<TransformChange, T::Error>;

    fn local_transform_all<T: XmlnsLocalTransformer + Clone>(
        &mut self,
        transformer: T,
    ) -> Result<TransformChange, T::Error>;

    fn context_transform<T: XmlnsContextTransformer>(
        &mut self,
        transformer: T,
    ) -> Result<TransformChange, T::Error>;
}

impl XmlnsContextExt for xsd_fragments::XmlnsContext {
    fn local_transform<T: XmlnsLocalTransformer>(
        &mut self,
        namespace: &XmlNamespace<'_>,
        transformer: T,
    ) -> Result<TransformChange, T::Error> {
        let Some(namespace) = self.namespace_idxs.get(namespace) else {
            return Ok(TransformChange::Unchanged);
        };

        let namespace = *namespace;

        self.local_transform_id(&namespace, transformer)
    }

    fn local_transform_id<T: XmlnsLocalTransformer>(
        &mut self,
        namespace: &NamespaceIdx,
        transformer: T,
    ) -> Result<TransformChange, T::Error> {
        self.namespaces
            .get_mut(namespace)
            .unwrap()
            .transform(transformer)
    }

    fn local_transform_all<T: XmlnsLocalTransformer + Clone>(
        &mut self,
        transformer: T,
    ) -> Result<TransformChange, T::Error> {
        self.namespaces.values_mut().try_fold(
            TransformChange::Unchanged,
            |total_change, namespace| {
                let change = namespace.transform(transformer.clone())?;
                Ok(total_change | change)
            },
        )
    }

    fn context_transform<T: XmlnsContextTransformer>(
        &mut self,
        transformer: T,
    ) -> Result<TransformChange, T::Error> {
        let context = XmlnsContextTransformerContext {
            xmlns_context: self,
        };

        transformer.transform(context)
    }
}

pub trait CompiledNamespaceExt {
    fn transform<T: XmlnsLocalTransformer>(
        &mut self,
        transformer: T,
    ) -> Result<TransformChange, T::Error>;
}

impl CompiledNamespaceExt for xsd_fragments::CompiledNamespace {
    fn transform<T: XmlnsLocalTransformer>(
        &mut self,
        transformer: T,
    ) -> Result<TransformChange, T::Error> {
        transformer.transform(XmlnsLocalTransformerContext { namespace: self })
    }
}
