//! This module provides the [`XmlnsLocalTransformer`] trait with implementations in the [complex] and [simple] modules.

/// Complex type local transformers.
pub mod complex;
/// Simple type local transformers.
pub mod simple;

use xmlity::{LocalName, XmlNamespace};

use crate::{
    fragments::{complex as cx, simple as sm, FragmentAccess, FragmentIdx, NamespaceIdx},
    transformers::TransformChange,
    CompiledNamespace,
};

/// This transformer type is only capable of doing local transformations within a namespace, and cannot access other namespaces.
///
/// It is useful for things like expanding extension fragments, or resolving local references, but not for things like resolving global references.
pub trait XmlnsLocalTransformer {
    /// Error type for [`XmlnsLocalTransformer`] failures.
    type Error: std::fmt::Debug;

    /// Returns true if the context was changed.
    fn transform(
        self,
        context: XmlnsLocalTransformerContext<'_>,
    ) -> Result<TransformChange, Self::Error>;
}

/// Context for local transformers, providing access to a single namespace.
#[derive(Debug)]
pub struct XmlnsLocalTransformerContext<'a> {
    /// The namespace being transformed.
    pub namespace: &'a mut CompiledNamespace,
}

impl XmlnsLocalTransformerContext<'_> {
    /// Gets the current namespace.
    pub fn current_namespace(&self) -> &crate::CompiledNamespace {
        self.namespace
    }

    /// Gets the current namespace mutably.
    pub fn current_namespace_mut(&mut self) -> &mut crate::CompiledNamespace {
        self.namespace
    }

    /// Returns an iterator over all complex fragment IDs of a specific type in this namespace.
    pub fn iter_complex_fragment_ids<F: 'static>(&self) -> impl Iterator<Item = FragmentIdx<F>> + '_
    where
        cx::ComplexTypeFragmentCompiler: FragmentAccess<F>,
    {
        self.current_namespace()
            .complex_type_compiler
            .iter_fragment_ids()
            .into_iter()
    }

    /// Gets a complex fragment by its ID.
    pub fn get_complex_fragment<F>(&self, fragment_idx: &FragmentIdx<F>) -> Option<&F>
    where
        cx::ComplexTypeFragmentCompiler: FragmentAccess<F>,
    {
        self.current_namespace()
            .complex_type_compiler
            .get_fragment(fragment_idx)
    }

    /// Gets a mutable complex fragment by its ID.
    pub fn get_complex_fragment_mut<F>(&mut self, fragment_idx: &FragmentIdx<F>) -> Option<&mut F>
    where
        cx::ComplexTypeFragmentCompiler: FragmentAccess<F>,
    {
        self.current_namespace_mut()
            .complex_type_compiler
            .get_fragment_mut(fragment_idx)
    }

    /// Returns all simple fragment IDs of a specific type in this namespace.
    pub fn iter_simple_fragment_ids<F: 'static>(&self) -> Vec<FragmentIdx<F>>
    where
        sm::SimpleTypeFragmentCompiler: FragmentAccess<F>,
    {
        self.current_namespace()
            .complex_type_compiler
            .simple_type_compiler
            .iter_fragment_ids()
    }

    /// Gets a simple fragment by its ID.
    pub fn get_simple_fragment<F>(&self, fragment_idx: &FragmentIdx<F>) -> Option<&F>
    where
        sm::SimpleTypeFragmentCompiler: FragmentAccess<F>,
    {
        self.current_namespace()
            .complex_type_compiler
            .simple_type_compiler
            .get_fragment(fragment_idx)
    }

    /// Gets a mutable simple fragment by its ID.
    pub fn get_simple_fragment_mut<F>(&mut self, fragment_idx: &FragmentIdx<F>) -> Option<&mut F>
    where
        sm::SimpleTypeFragmentCompiler: FragmentAccess<F>,
    {
        self.current_namespace_mut()
            .complex_type_compiler
            .simple_type_compiler
            .get_fragment_mut(fragment_idx)
    }

    /// Gets a named type by its local name within this namespace.
    pub fn get_named_type<'a>(
        &'a self,
        name: &'a LocalName<'_>,
    ) -> Option<&'a crate::TopLevelType> {
        self.current_namespace().top_level_types.get(name)
    }

    /// Gets a named attribute group by its local name within this namespace.
    pub fn get_named_attribute_group<'a>(
        &'a self,
        name: &'a LocalName<'_>,
    ) -> Option<&'a crate::TopLevelAttributeGroup> {
        self.current_namespace()
            .top_level_attribute_groups
            .get(name)
    }
}

impl crate::XmlnsContext {
    /// Applies a local transformer to a specific namespace.
    pub fn local_transform<T: XmlnsLocalTransformer>(
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

    /// Applies a local transformer to a namespace by its ID.
    pub fn local_transform_id<T: XmlnsLocalTransformer>(
        &mut self,
        namespace: &NamespaceIdx,
        transformer: T,
    ) -> Result<TransformChange, T::Error> {
        self.namespaces
            .get_mut(namespace)
            .unwrap()
            .transform(transformer)
    }

    /// Applies a local transformer to all namespaces.
    pub fn local_transform_all<T: XmlnsLocalTransformer + Clone>(
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
}

impl crate::CompiledNamespace {
    /// Applies a local transformer to this namespace.
    pub fn transform<T: XmlnsLocalTransformer>(
        &mut self,
        transformer: T,
    ) -> Result<TransformChange, T::Error> {
        transformer.transform(XmlnsLocalTransformerContext { namespace: self })
    }
}
