pub mod complex;
pub mod simple;

use xmlity::ExpandedName;

use crate::{
    fragments::{complex as cx, simple as sm, FragmentAccess, FragmentIdx, NamespaceIdx},
    transformers::TransformChange,
};

pub trait XmlnsContextTransformer {
    type Error: std::fmt::Debug;

    /// Returns true if the context was changed.
    fn transform(
        self,
        context: XmlnsContextTransformerContext<'_>,
    ) -> Result<TransformChange, Self::Error>;
}

#[derive(Debug)]
pub struct XmlnsContextTransformerContext<'a> {
    pub xmlns_context: &'a mut crate::XmlnsContext,
}

impl XmlnsContextTransformerContext<'_> {
    fn get_namespace(&self, namespace_idx: &NamespaceIdx) -> Option<&crate::CompiledNamespace> {
        self.xmlns_context.namespaces.get(namespace_idx)
    }

    fn get_namespace_mut(
        &mut self,
        namespace_idx: &NamespaceIdx,
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
    ) -> Option<&'a crate::TopLevelType> {
        self.xmlns_context
            .get_namespace(name.namespace()?)?
            .top_level_types
            .get(name.local_name())
    }

    pub fn get_named_attribute_group<'a>(
        &'a self,
        name: &'a ExpandedName<'_>,
    ) -> Option<&'a crate::TopLevelAttributeGroup> {
        self.xmlns_context
            .get_namespace(name.namespace()?)?
            .top_level_attribute_groups
            .get(name.local_name())
    }
}

impl crate::XmlnsContext {
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
