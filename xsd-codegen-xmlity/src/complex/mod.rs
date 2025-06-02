pub mod attributes;
pub mod complex_type;
pub mod elements;
pub mod groups;

use crate::{misc::TypeReference, Result, TypeType};

use syn::{parse_quote, Ident};
use xmlity::{ExpandedName, LocalName, XmlNamespace};
use xsd::schema::MaxOccursValue;
use xsd_type_compiler::complex::{ComplexTypeFragmentCompiler, FragmentAccess, FragmentIdx};

pub trait Context {
    type SubContext: Context;

    fn namespace(&self) -> &XmlNamespace<'_>;

    fn sub_context<'i>(&self, suggested_ident: Ident) -> Self::SubContext;

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

    fn resolve_named_type(
        &self,
        name: &ExpandedName<'_>,
    ) -> Result<(TypeReference<'static>, TypeType)>;

    fn resolve_named_group(&self, name: &ExpandedName<'_>) -> Result<TypeReference<'static>>;

    fn resolve_named_element(&self, name: &ExpandedName<'_>) -> Result<TypeReference<'static>>;

    fn resolve_named_attribute(&self, name: &ExpandedName<'_>) -> Result<TypeReference<'static>>;

    fn to_expanded_name(&self, name: LocalName<'static>) -> ExpandedName<'static>;
}

pub trait Scope {
    fn add_item<I: Into<syn::Item>>(&mut self, item: I) -> Result<TypeReference<'static>>;
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
    max_occurs: MaxOccursValue,
    type_: TypeReference<'_>,
) -> (TypeReference<'_>, bool) {
    match (min_occurs, max_occurs) {
        (1, MaxOccursValue::Bounded(1)) => (type_, false),
        (0, MaxOccursValue::Bounded(1)) => (type_.wrap(|ty| parse_quote!(Option<#ty>)), true),
        (_, _) => (type_.wrap(|ty| parse_quote!(Vec<#ty>)), true),
    }
}
