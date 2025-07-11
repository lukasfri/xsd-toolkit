use crate::{BoundType, Result, Scope, ToTypeTemplateData};

use syn::Ident;
use xmlity::{ExpandedName, LocalName, XmlNamespace};
use xsd_fragments::fragments::{simple::SimpleTypeFragmentCompiler, FragmentAccess, FragmentIdx};

pub mod restrictions;
pub mod simple_type;

pub trait SimpleContext {
    type SubContext: SimpleContext;

    fn sub_context(&self, suggested_ident: Ident) -> Self::SubContext;

    fn suggested_ident(&self) -> &Ident;

    fn namespace(&self) -> &XmlNamespace<'_>;

    fn to_expanded_name(&self, name: LocalName<'static>) -> ExpandedName<'static>;

    fn resolve_fragment<F: SimpleToTypeTemplate, S: Scope>(
        &self,
        fragment: &F,
        scope: &mut S,
    ) -> Result<ToTypeTemplateData<F::TypeTemplate>>;

    fn get_fragment<F, S: Scope>(&self, fragment: &FragmentIdx<F>, scope: &mut S) -> Result<&F>
    where
        SimpleTypeFragmentCompiler: FragmentAccess<F>;

    fn resolve_fragment_id<F: SimpleToTypeTemplate, S: Scope>(
        &self,
        fragment_id: &FragmentIdx<F>,
        scope: &mut S,
    ) -> Result<ToTypeTemplateData<F::TypeTemplate>>
    where
        SimpleTypeFragmentCompiler: FragmentAccess<F>;

    fn resolve_named_type(&self, name: &ExpandedName<'_>) -> Result<BoundType>;
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
