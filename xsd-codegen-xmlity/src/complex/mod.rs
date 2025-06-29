pub mod attributes;
pub mod complex_type;
pub mod elements;
pub mod groups;

use crate::{
    misc::TypeReference, simple::SimpleContext, BoundType, Result, Scope, ToTypeTemplateData,
};

use syn::Ident;
use xmlity::{ExpandedName, LocalName, XmlNamespace};
use xsd::xs::types::AllNNI;
use xsd_type_compiler::fragments::{
    complex::ComplexTypeFragmentCompiler, FragmentAccess, FragmentIdx,
};

pub trait ComplexContext {
    type SimpleContext: SimpleContext;
    type SubContext: ComplexContext;

    fn simple_context(&self) -> &Self::SimpleContext;

    fn sub_context(&self, suggested_ident: Ident) -> Self::SubContext;

    fn suggested_ident(&self) -> &Ident;

    fn namespace(&self) -> &XmlNamespace<'_>;

    fn to_expanded_name(&self, name: LocalName<'static>) -> ExpandedName<'static>;

    fn resolve_fragment<F: ComplexToTypeTemplate, S: Scope>(
        &self,
        fragment: &F,
        scope: &mut S,
    ) -> Result<ToTypeTemplateData<F::TypeTemplate>>;

    fn resolve_fragment_id<F: ComplexToTypeTemplate, S: Scope>(
        &self,
        fragment_id: &FragmentIdx<F>,
        scope: &mut S,
    ) -> Result<ToTypeTemplateData<F::TypeTemplate>>
    where
        ComplexTypeFragmentCompiler: FragmentAccess<F>;

    fn resolve_named_type(&self, name: &ExpandedName<'_>) -> Result<BoundType>;

    fn resolve_named_element(&self, name: &ExpandedName<'_>) -> Result<TypeReference<'static>>;

    fn resolve_named_attribute(&self, name: &ExpandedName<'_>) -> Result<TypeReference<'static>>;

    fn resolve_named_group(&self, name: &ExpandedName<'_>) -> Result<TypeReference<'static>>;
}

pub trait ComplexToTypeTemplate {
    type TypeTemplate;

    fn to_type_template<C: ComplexContext, S: Scope>(
        &self,
        context: &C,
        scope: &mut S,
    ) -> Result<ToTypeTemplateData<Self::TypeTemplate>>;
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
