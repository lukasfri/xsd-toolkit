//! Context-based transformers for XML Schema fragments.

/// Complex type transformation implementations.
pub mod complex;
/// Simple type transformation implementations.
pub mod simple;

use xmlity::{ExpandedName, XmlNamespace};

use crate::{
    fragments::{complex as cx, simple as sm, FragmentAccess, FragmentIdx, FragmentedXsdDocumentIdx},
    transformers::TransformChange,
};

/// Trait for transformers that operate on [`XmlnsContext`] instances.
pub trait XmlnsContextTransformer {
    /// Error type returned by the [`XmlnsContextTransformer`].
    type Error: std::fmt::Debug;

    /// Returns true if the context was changed.
    fn transform(
        self,
        context: XmlnsContextTransformerContext<'_>,
    ) -> Result<TransformChange, Self::Error>;
}

#[derive(Debug)]
/// Context for namespace-based transformations.
pub struct XmlnsContextTransformerContext<'a> {
    /// The XML namespace context being transformed.
    pub xmlns_context: &'a mut crate::XmlnsContext,
}

impl XmlnsContextTransformerContext<'_> {
    fn get_namespace(
        &self,
        namespace_idx: &FragmentedXsdDocumentIdx,
    ) -> Option<&crate::FragmentedXsdDocument> {
        self.xmlns_context.namespaces.get(namespace_idx)
    }

    fn get_namespace_mut(
        &mut self,
        namespace_idx: &FragmentedXsdDocumentIdx,
    ) -> Option<&mut crate::FragmentedXsdDocument> {
        self.xmlns_context.namespaces.get_mut(namespace_idx)
    }

    fn get_referenced_namespace(
        &self,
        namespace_idx: &FragmentedXsdDocumentIdx,
        reference_namespace: Option<&XmlNamespace<'_>>,
    ) -> Option<&crate::FragmentedXsdDocument> {
        let compiled_namespace = self.get_namespace(namespace_idx)?;

        if reference_namespace.is_some_and(|a| *a == compiled_namespace.namespace) {
            Some(compiled_namespace)
        } else {
            let referenced_ns = compiled_namespace
                .namespace_references
                .get(reference_namespace?)?;

            Some(self.get_namespace(referenced_ns)?)
        }
    }

    /// Returns an iterator over all complex fragment IDs of a specific type.
    pub fn iter_complex_fragment_ids<F: 'static>(&self) -> impl Iterator<Item = FragmentIdx<F>> + '_
    where
        cx::ComplexTypeFragmentCompiler: FragmentAccess<F>,
    {
        self.xmlns_context
            .namespaces
            .iter()
            .flat_map(|(_, ns)| ns.complex_type_compiler.iter_fragment_ids())
    }

    /// Gets a complex fragment by its ID.
    pub fn get_complex_fragment<F>(&self, fragment_idx: &FragmentIdx<F>) -> Option<&F>
    where
        cx::ComplexTypeFragmentCompiler: FragmentAccess<F>,
    {
        self.get_namespace(&fragment_idx.namespace_idx())?
            .complex_type_compiler
            .get_fragment(fragment_idx)
    }

    /// Gets a mutable complex fragment by its ID.
    pub fn get_complex_fragment_mut<F>(&mut self, fragment_idx: &FragmentIdx<F>) -> Option<&mut F>
    where
        cx::ComplexTypeFragmentCompiler: FragmentAccess<F>,
    {
        self.get_namespace_mut(&fragment_idx.namespace_idx())?
            .complex_type_compiler
            .get_fragment_mut(fragment_idx)
    }

    /// Returns an iterator over all simple fragment IDs of a specific type.
    pub fn iter_simple_fragment_ids<F: 'static>(&self) -> impl Iterator<Item = FragmentIdx<F>> + '_
    where
        sm::SimpleTypeFragmentCompiler: FragmentAccess<F>,
    {
        self.xmlns_context.namespaces.iter().flat_map(|(_, ns)| {
            ns.complex_type_compiler
                .simple_type_compiler
                .iter_fragment_ids()
        })
    }

    /// Gets a simple fragment by its ID.
    pub fn get_simple_fragment<F>(&self, fragment_idx: &FragmentIdx<F>) -> Option<&F>
    where
        sm::SimpleTypeFragmentCompiler: FragmentAccess<F>,
    {
        self.get_namespace(&fragment_idx.namespace_idx())?
            .complex_type_compiler
            .simple_type_compiler
            .get_fragment(fragment_idx)
    }

    /// Gets a mutable simple fragment by its ID.
    pub fn get_simple_fragment_mut<F>(&mut self, fragment_idx: &FragmentIdx<F>) -> Option<&mut F>
    where
        sm::SimpleTypeFragmentCompiler: FragmentAccess<F>,
    {
        self.get_namespace_mut(&fragment_idx.namespace_idx())?
            .complex_type_compiler
            .simple_type_compiler
            .get_fragment_mut(fragment_idx)
    }

    /// Gets a named type by its expanded name.
    pub fn get_named_type<'a>(
        &'a self,
        namespace_idx: &FragmentedXsdDocumentIdx,
        name: &'a ExpandedName<'_>,
    ) -> Option<&'a crate::TopLevelType> {
        let ns = self.get_referenced_namespace(namespace_idx, name.namespace())?;

        ns.top_level_types.get(name.local_name())
    }

    /// Gets a named attribute group by its expanded name.
    pub fn get_named_attribute_group<'a>(
        &'a self,
        namespace_idx: &FragmentedXsdDocumentIdx,
        name: &'a ExpandedName<'_>,
    ) -> Option<&'a crate::TopLevelAttributeGroup> {
        let ns = self.get_referenced_namespace(namespace_idx, name.namespace())?;

        ns.top_level_attribute_groups.get(name.local_name())
    }
}

impl crate::XmlnsContext {
    /// Applies a context transformer to this context.
    pub fn context_transform<T: XmlnsContextTransformer>(
        &mut self,
        transformer: T,
    ) -> Result<TransformChange, T::Error> {
        let context = XmlnsContextTransformerContext {
            xmlns_context: self,
        };

        transformer.transform(context)
    }
}
