use crate::{BoundType, Result, Scope, ToTypeTemplateData};

use syn::Ident;
use xmlity::{ExpandedName, LocalName, XmlNamespace};
use xsd_type_compiler::fragments::{
    simple::SimpleTypeFragmentCompiler, FragmentAccess, FragmentIdx,
};

pub mod groups;
pub mod simple_type;

pub trait SimpleContext {
    type SubContext: SimpleContext;

    fn namespace(&self) -> &XmlNamespace<'_>;

    fn sub_context(&self, suggested_ident: Ident) -> Self::SubContext;

    fn suggested_ident(&self) -> &Ident;

    fn resolve_fragment<F: SimpleToTypeTemplate, S: Scope>(
        &self,
        fragment: &F,
        scope: &mut S,
    ) -> Result<ToTypeTemplateData<F::TypeTemplate>>;

    fn resolve_fragment_id<F: SimpleToTypeTemplate, S: Scope>(
        &self,
        fragment_id: &FragmentIdx<F>,
        scope: &mut S,
    ) -> Result<ToTypeTemplateData<F::TypeTemplate>>
    where
        SimpleTypeFragmentCompiler: FragmentAccess<F>;

    fn resolve_named_type(&self, name: &ExpandedName<'_>) -> Result<BoundType>;

    fn to_expanded_name(&self, name: LocalName<'static>) -> ExpandedName<'static>;
}

pub trait SimpleToTypeTemplate {
    type TypeTemplate;

    fn to_type_template<C: SimpleContext, S: Scope>(
        &self,
        context: &C,
        scope: &mut S,
    ) -> Result<ToTypeTemplateData<Self::TypeTemplate>>;
}

#[cfg(test)]
mod tests {}
