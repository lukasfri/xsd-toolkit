pub mod attributes;
pub mod complex_type;
pub mod elements;
pub mod groups;

use crate::{
    misc::TypeReference, simple::SimpleContext, BoundType, Result, Scope, ToTypeTemplateData,
};

use syn::Ident;
use xmlity::{ExpandedName, ExpandedNameBuf, LocalName, LocalNameBuf, XmlNamespace};
use xsd_fragments::fragments::{
    complex::{AllNNI, ComplexTypeFragmentCompiler},
    FragmentAccess, FragmentIdx, FragmentedXsdDocumentIdx,
};

pub trait ComplexContext {
    type SimpleContext: SimpleContext;
    type SubContext: ComplexContext;

    fn simple_context(&self) -> &Self::SimpleContext;

    fn sub_context(&self, suggested_ident: Ident) -> Self::SubContext;

    fn suggested_ident(&self) -> &Ident;

    fn namespace(&self) -> &XmlNamespace;

    fn to_expanded_name(&self, name: LocalNameBuf) -> ExpandedNameBuf;

    fn get_fragment<F>(&self, fragment: &FragmentIdx<F>) -> Result<&F>
    where
        ComplexTypeFragmentCompiler: FragmentAccess<F>;

    fn resolve_type_template<I, H: ComplexToTypeTemplate<I>, S: Scope>(
        &self,
        fragment_id: &xsd_fragments::fragments::FragmentIdx<I>,
        scope: &mut S,
        handler: &H,
    ) -> Result<ToTypeTemplateData<H::TypeTemplate>>
    where
        ComplexTypeFragmentCompiler: FragmentAccess<I>;

    fn resolve_named_type(
        &self,
        key: &FragmentedXsdDocumentIdx,
        name: &ExpandedName<'_>,
    ) -> Result<BoundType>;

    fn resolve_named_element(
        &self,
        key: &FragmentedXsdDocumentIdx,
        name: &ExpandedName<'_>,
    ) -> Result<TypeReference<'static>>;

    fn resolve_named_attribute(
        &self,
        key: &FragmentedXsdDocumentIdx,
        name: &ExpandedName<'_>,
    ) -> Result<TypeReference<'static>>;

    fn resolve_named_group(
        &self,
        key: &FragmentedXsdDocumentIdx,
        name: &ExpandedName<'_>,
    ) -> Result<TypeReference<'static>>;

    fn substitution_group_members(
        &self,
        name: &ExpandedName<'_>,
    ) -> Result<impl Iterator<Item = (FragmentedXsdDocumentIdx, ExpandedName<'_>)>>;
}

pub trait ComplexToTypeTemplate<I> {
    type TypeTemplate;

    fn to_type_template<C: ComplexContext, S: Scope>(
        &self,
        context: &C,
        scope: &mut S,
        item: &I,
    ) -> Result<ToTypeTemplateData<Self::TypeTemplate>>;
}

pub trait ComplexToTypeTemplateExt<I>: ComplexToTypeTemplate<I> {
    fn resolve_type_template<C: ComplexContext, S: Scope>(
        &self,
        context: &C,
        scope: &mut S,
        fragment_id: &xsd_fragments::fragments::FragmentIdx<I>,
    ) -> Result<ToTypeTemplateData<Self::TypeTemplate>>
    where
        ComplexTypeFragmentCompiler: FragmentAccess<I>;
}

impl<I, T: ComplexToTypeTemplate<I>> ComplexToTypeTemplateExt<I> for T {
    fn resolve_type_template<C: ComplexContext, S: Scope>(
        &self,
        context: &C,
        scope: &mut S,
        fragment_id: &xsd_fragments::fragments::FragmentIdx<I>,
    ) -> Result<ToTypeTemplateData<Self::TypeTemplate>>
    where
        ComplexTypeFragmentCompiler: FragmentAccess<I>,
    {
        context.resolve_type_template(fragment_id, scope, self)
    }
}

fn min_max_occurs_type(
    min_occurs: usize,
    max_occurs: AllNNI,
    type_: TypeReference<'_>,
) -> (TypeReference<'_>, bool) {
    match (min_occurs, max_occurs) {
        (1, AllNNI::Bounded(1)) => (type_, false),
        (0, AllNNI::Bounded(1)) => (type_.wrap(TypeReference::option_wrapper), true),
        (_, _) => (type_.wrap(TypeReference::vec_non_boxed_wrapper), true),
    }
}
