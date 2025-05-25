pub mod attributes;
pub mod complex_type;
pub mod elements;
pub mod groups;

use crate::{misc::TypeReference, Result};

use syn::{parse_quote, Ident};
use xmlity::{ExpandedName, LocalName};
use xsd::schema::MaxOccursValue;
use xsd_type_compiler::complex::{ComplexTypeFragmentCompiler, FragmentAccess, FragmentIdx};

pub trait Context: crate::simple::Context {
    fn resolve_fragment<F>(&self, fragment: &F) -> Result<ToTypeTemplateData<F::TypeTemplate>>
    where
        F: ToTypeTemplate;

    fn resolve_fragment_id<F>(
        &self,
        fragment_id: &FragmentIdx<F>,
    ) -> Result<ToTypeTemplateData<F::TypeTemplate>>
    where
        F: ToTypeTemplate,
        ComplexTypeFragmentCompiler: FragmentAccess<F>;

    fn resolve_named_type(&self, name: &ExpandedName<'_>) -> Result<syn::Type>;

    fn to_expanded_name(&self, name: LocalName<'static>) -> ExpandedName<'static>;
}

pub struct ToTypeTemplateData<T> {
    pub ident: Option<Ident>,
    pub template: T,
}

pub trait ToTypeTemplate {
    type TypeTemplate;

    fn to_type_template<C: Context>(
        &self,
        context: &C,
    ) -> Result<ToTypeTemplateData<Self::TypeTemplate>>;
}

fn min_max_occurs_type(
    min_occurs: usize,
    max_occurs: MaxOccursValue,
    type_: TypeReference<'_>,
) -> TypeReference<'_> {
    match (min_occurs, max_occurs) {
        (1, MaxOccursValue::Bounded(1)) => type_,
        (0, MaxOccursValue::Bounded(1)) => TypeReference::new(|path| {
            let type_ = type_.into_type(path);
            parse_quote!(Option<#type_>)
        }),
        (_, _) => TypeReference::new(|path| {
            let type_ = type_.into_type(path);
            parse_quote!(Vec<#type_>)
        }),
    }
}
