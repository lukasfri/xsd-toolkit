pub mod attributes;
pub mod complex_type;
pub mod elements;
pub mod groups;

use crate::{augments::ItemAugmentation, misc::TypeReference, BoundType, Result};

use quote::format_ident;
use syn::Ident;
use xmlity::{ExpandedName, LocalName, XmlNamespace};
use xsd::xs::types::AllNNI;
use xsd_type_compiler::complex::{ComplexTypeFragmentCompiler, FragmentAccess, FragmentIdx};

fn dedup_field_idents<T>(
    fields: impl IntoIterator<Item = (syn::Ident, T)>,
) -> Vec<(syn::Ident, T)> {
    //This function in case of multiple fields having the same ident (ex. "field") should name them field_0, field_1, etc. Order must be preserved.
    let mut seen_idents = std::collections::HashSet::new();
    let mut deduped_fields = Vec::new();

    for (ident, value) in fields {
        let mut new_ident = ident.clone();
        let mut counter = 0;

        while seen_idents.contains(&new_ident) {
            new_ident = format_ident!("{}_{}", ident, counter.to_string());
            counter += 1;
        }

        seen_idents.insert(new_ident.clone());
        deduped_fields.push((new_ident, value));
    }

    deduped_fields
}

pub trait Context {
    type SubContext: Context;

    fn namespace(&self) -> &XmlNamespace<'_>;

    fn sub_context(&self, suggested_ident: Ident) -> Self::SubContext;

    fn suggested_ident(&self) -> &Ident;

    fn resolve_fragment<F: ToTypeTemplate, S: Scope>(
        &self,
        fragment: &F,
        scope: &mut S,
    ) -> Result<ToTypeTemplateData<F::TypeTemplate>>;

    fn resolve_fragment_id<F: ToTypeTemplate, S: Scope>(
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

    fn to_expanded_name(&self, name: LocalName<'static>) -> ExpandedName<'static>;
}

pub trait Scope {
    fn add_item<I: Into<syn::Item>>(&mut self, item: I) -> Result<TypeReference<'static>>;

    fn add_raw_items<I: IntoIterator<Item = J>, J: Into<syn::Item>>(&mut self, items: I);

    fn augmenter(&self) -> &dyn ItemAugmentation;
}

pub struct ToTypeTemplateData<T> {
    pub ident: Option<Ident>,
    pub template: T,
}

pub trait ToTypeTemplate {
    type TypeTemplate;

    fn to_type_template<C: Context, S: Scope>(
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
