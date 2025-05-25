use crate::{
    templates::{
        self,
        choice::Choice,
        value_record::{ItemField, ItemRecord},
        FieldType, ItemOrder,
    },
    Result,
};

use quote::format_ident;
use xsd_type_compiler::complex as cx;

use super::{Context, ToTypeTemplate, ToTypeTemplateData};

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
            cx::GroupTypeContentId::Sequence(fragment_idx) => {
                let record = context.resolve_fragment_id(fragment_idx)?;

                let item = todo!();

                Ok(ToTypeTemplateData {
                    ident: record.ident,
                    template: ItemField::Item(item),
                })
            }
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
