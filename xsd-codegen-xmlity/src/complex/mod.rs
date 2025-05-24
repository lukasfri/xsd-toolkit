pub mod attributes;
pub mod complex_type;
pub mod elements;

use crate::{
    misc::TypeReference,
    simple as simple_gen,
    templates::{
        self,
        choice::Choice,
        element_record::{ElementField, ElementFieldAttribute},
        group_record::GroupRecord,
        value_record::{ItemField, ItemFieldItem, ItemRecord},
        FieldType, ItemOrder,
    },
    Result,
};

use std::collections::HashMap;

use quote::format_ident;
use syn::{parse_quote, Ident, Item};
use xmlity::{ExpandedName, LocalName, XmlNamespace};
use xsd::schema::MaxOccursValue;
use xsd_type_compiler::complex::{
    self as cx, ComplexTypeFragmentCompiler, FragmentAccess, FragmentIdx,
};

pub struct ComplexTypeFragmentGenerator<'a> {
    pub context: &'a xsd_type_compiler::complex::ComplexTypeFragmentCompiler,
    pub bound_namespaces: &'a HashMap<XmlNamespace<'static>, Ident>,
    pub simple_generator: &'a simple_gen::SimpleGeneratedNamespaceTypes,
}

impl ComplexTypeFragmentGenerator<'_> {
    pub fn generate(&self) -> Result<Vec<Item>> {
        Ok(vec![])
    }

    pub fn get_named_type(&self, name: &ExpandedName<'_>) -> Result<Option<syn::TypePath>> {
        todo!()
    }
}

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

impl ToTypeTemplate for cx::GroupTypeContentId {
    type TypeTemplate = templates::value_record::ItemField;

    fn to_type_template<C: Context>(
        &self,
        context: &C,
    ) -> Result<ToTypeTemplateData<Self::TypeTemplate>> {
        match self {
            cx::GroupTypeContentId::Element(fragment_idx) => {
                context.resolve_fragment_id(fragment_idx)
            }
            cx::GroupTypeContentId::Group(fragment_idx) => todo!(),
            cx::GroupTypeContentId::All(fragment_idx) => todo!(),
            cx::GroupTypeContentId::Choice(fragment_idx) => todo!(),
            cx::GroupTypeContentId::Sequence(fragment_idx) => todo!(),
            cx::GroupTypeContentId::Any(fragment_idx) => todo!(),
        }
    }
}

impl ToTypeTemplate for cx::SequenceFragment {
    type TypeTemplate = templates::value_record::ItemRecord;

    fn to_type_template<C: Context>(
        &self,
        context: &C,
    ) -> Result<ToTypeTemplateData<Self::TypeTemplate>> {
        // Struct with strict order
        let fields = self
            .fragments
            .iter()
            .enumerate()
            .map(|(i, fragment_id)| {
                let res = context.resolve_fragment::<_>(fragment_id)?;

                Ok((
                    Some(res.ident.unwrap_or_else(|| format_ident!("field_{}", i))),
                    res.template,
                ))
            })
            .collect::<Result<Vec<_>>>()?;

        let template = templates::value_record::ItemRecord {
            children_order: ItemOrder::Strict,
            field_type: FieldType::Named,
            fields,
        };

        Ok(ToTypeTemplateData {
            ident: None,
            template,
        })
    }
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

pub enum TypeDefParticleTemplate {
    Record(ItemRecord),
    Choice(Choice),
}

impl ToTypeTemplate for cx::TypeDefParticleId {
    type TypeTemplate = TypeDefParticleTemplate;

    fn to_type_template<C: Context>(
        &self,
        context: &C,
    ) -> Result<ToTypeTemplateData<Self::TypeTemplate>> {
        match self {
            cx::TypeDefParticleId::Group(fragment_idx) => todo!(),
            cx::TypeDefParticleId::All(fragment_idx) => todo!(),
            cx::TypeDefParticleId::Sequence(fragment_idx) => {
                let sequence = context.resolve_fragment_id(fragment_idx)?;

                Ok(ToTypeTemplateData {
                    ident: None,
                    template: TypeDefParticleTemplate::Record(sequence.template),
                })
            }
            cx::TypeDefParticleId::Choice(fragment_idx) => todo!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use syn::{parse_quote, Item};
    use xmlity::{ExpandedName, LocalName, XmlNamespace};
    use xsd::schema as xs;
    use xsd_type_compiler::{complex::ANY_TYPE_EXPANDED_NAME, CompiledNamespace, XmlnsContext};

    use crate::Generator;
}
