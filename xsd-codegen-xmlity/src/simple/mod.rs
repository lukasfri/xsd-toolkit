use crate::{BoundType, Result, Scope, ToTypeTemplateData};

use syn::Ident;
use xmlity::{ExpandedName, LocalName, XmlNamespace};
use xsd_fragments::fragments::{
    simple::SimpleTypeFragmentCompiler, FragmentAccess, FragmentIdx, LocalNamespaceIdx,
};

pub mod restrictions;
pub mod simple_type;

pub trait SimpleContext {
    type SubContext: SimpleContext;

    fn sub_context(&self, suggested_ident: Ident) -> Self::SubContext;

    fn suggested_ident(&self) -> &Ident;

    fn namespace(&self) -> &XmlNamespace<'_>;

    fn namespace_idx(&self) -> &LocalNamespaceIdx;

    fn to_expanded_name(&self, name: LocalName<'static>) -> ExpandedName<'static>;

    fn get_fragment<F>(&self, fragment: &FragmentIdx<F>) -> Result<&F>
    where
        SimpleTypeFragmentCompiler: FragmentAccess<F>;

    fn resolve_type_template<I, H: SimpleToTypeTemplate<I>, S: Scope>(
        &self,
        fragment_id: &FragmentIdx<I>,
        scope: &mut S,
        handler: &H,
    ) -> Result<ToTypeTemplateData<H::TypeTemplate>>
    where
        SimpleTypeFragmentCompiler: FragmentAccess<I>;

    fn resolve_named_type(
        &self,
        key: &LocalNamespaceIdx,
        name: &ExpandedName<'_>,
    ) -> Result<BoundType>;
}

pub trait SimpleToTypeTemplate<I> {
    type TypeTemplate;

    fn to_type_template<C: SimpleContext, S: Scope>(
        &self,
        context: &C,
        scope: &mut S,
        item: &I,
    ) -> Result<ToTypeTemplateData<Self::TypeTemplate>>;
}

#[cfg(test)]
mod tests {}
