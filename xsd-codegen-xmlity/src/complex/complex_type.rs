use std::sync::Arc;

use crate::{
    complex::{
        attributes::AttributeDeclarationsHandler, groups::TypeDefParticleIdHandler,
        ComplexToTypeTemplateExt,
    },
    naming_strategies::WrappingNamingStrategy,
    templates::{
        self,
        element_record::{ElementField, ElementFieldType},
        group_record::GroupRecord,
        value_record::ItemFieldItem,
    },
    Result, ToIdentTypesExt,
};

use quote::format_ident;
use syn::parse_quote;
use xsd_fragments::fragments::{
    complex::{self as cx},
    FragmentIdx,
};

use super::{
    groups::TypeDefParticleTemplate, ComplexContext, ComplexToTypeTemplate, Scope,
    ToTypeTemplateData,
};

fn dedup_attribute_field_idents<T, E>(
    existing_fields: &[(syn::Ident, E)],
    attribute_fields: impl IntoIterator<Item = (syn::Ident, T)>,
    attribute_suffix_naming: &WrappingNamingStrategy,
) -> Vec<(syn::Ident, T)> {
    attribute_fields
        .into_iter()
        .map(|(ident, value)| {
            if existing_fields
                .iter()
                .any(|(existing_ident, _)| existing_ident == &ident)
            {
                let new_ident = attribute_suffix_naming.wrap_ident(&ident);
                (new_ident, value)
            } else {
                (ident, value)
            }
        })
        .collect()
}

#[derive(Debug)]
pub struct RestrictionHandler {
    pub attribute_declarations_handler: Arc<AttributeDeclarationsHandler>,
    pub type_def_particle_handler: Arc<TypeDefParticleIdHandler>,
    pub default_particle_ident: String,
    pub content_type_naming: WrappingNamingStrategy,
    pub attribute_suffix_naming: WrappingNamingStrategy,
}

impl ComplexToTypeTemplate<cx::RestrictionFragment> for RestrictionHandler {
    type TypeTemplate = templates::group_record::GroupRecord;

    fn to_type_template<C: ComplexContext, S: Scope>(
        &self,
        context: &C,
        scope: &mut S,
        item: &cx::RestrictionFragment,
    ) -> Result<ToTypeTemplateData<Self::TypeTemplate>> {
        let template = item
            .content_fragment
            .map(|a| {
                let sub_context = context.sub_context(
                    self.content_type_naming
                        .wrap_ident(context.suggested_ident()),
                );

                self.type_def_particle_handler
                    .to_type_template(&sub_context, scope, &a)
            })
            .transpose()?;

        let mut template = template
            .map(|a| {
                let ident = a
                    .ident
                    .unwrap_or_else(|| format_ident!("{}", self.default_particle_ident));

                match a.template {
                    TypeDefParticleTemplate::Record(item_record) => item_record.into_group_record(),
                    TypeDefParticleTemplate::Choice(item) => {
                        let item = item.to_enum(&ident, None);

                        let ty = scope.add_item(item).unwrap();

                        GroupRecord::new_single_field(
                            Some(ident.to_field_ident()),
                            ElementField::Item(ItemFieldItem {
                                ty,
                                default: false,
                                default_with: None,
                            }),
                        )
                    }
                    TypeDefParticleTemplate::Item(item) => GroupRecord::new_single_field(
                        Some(ident.to_field_ident()),
                        ElementField::Item(item),
                    ),
                }
            })
            .unwrap_or_else(GroupRecord::new_empty);

        let attributes = self.attribute_declarations_handler.resolve_type_template(
            context,
            scope,
            &item.attribute_declarations,
        )?;

        let attribute_fields = dedup_attribute_field_idents(
            match &template.fields {
                ElementFieldType::Named(items) => items.as_slice(),
                ElementFieldType::Empty => &[],
                ElementFieldType::Unnamed(_) => {
                    unreachable!("Should only be named fields or empty")
                }
            },
            attributes.template,
            &self.attribute_suffix_naming,
        );

        template
            .fields
            .prefix_fields(ElementFieldType::Named(attribute_fields));

        template.force_empty_if_empty();

        Ok(ToTypeTemplateData {
            ident: None,
            template,
        })
    }
}

#[derive(Debug)]
pub struct SimpleExtensionFragmentHandler {
    pub attribute_declarations_handler: Arc<AttributeDeclarationsHandler>,
    pub content_field_ident: String,
    pub attribute_suffix_naming: WrappingNamingStrategy,
}

impl ComplexToTypeTemplate<FragmentIdx<cx::SimpleExtensionFragment>>
    for SimpleExtensionFragmentHandler
{
    type TypeTemplate = templates::group_record::GroupRecord;

    fn to_type_template<C: ComplexContext, S: Scope>(
        &self,
        context: &C,
        scope: &mut S,

        fragment_idx: &FragmentIdx<cx::SimpleExtensionFragment>,
    ) -> Result<ToTypeTemplateData<Self::TypeTemplate>> {
        let item = context.get_fragment(fragment_idx)?;
        let simple_type = context.resolve_named_type(&fragment_idx.namespace_idx(), &item.base)?;

        if simple_type.ty_type != crate::TypeType::Simple {
            return Err(crate::Error::UnsupportedFragment {
                fragment: "SimpleExtensionFragment with non-simple type as base".to_string(),
            });
        }

        let mut template = GroupRecord::new_single_field(
            Some(format_ident!("{}", self.content_field_ident)),
            ElementField::Item(ItemFieldItem {
                ty: simple_type.ty,
                default: false,
                // Todo: This should only be added to certain simple types that allow empty strings
                default_with: Some(parse_quote!(::xmlity_ns::empty_str_default)),
            }),
        );

        let attributes = self.attribute_declarations_handler.resolve_type_template(
            context,
            scope,
            &item.attribute_declarations,
        )?;

        let attribute_fields = dedup_attribute_field_idents(
            match &template.fields {
                ElementFieldType::Named(items) => items,
                ElementFieldType::Empty => &[],
                ElementFieldType::Unnamed(_) => {
                    unreachable!("Should only be named fields or empty")
                }
            },
            attributes.template,
            &self.attribute_suffix_naming,
        );

        template
            .fields
            .prefix_fields(ElementFieldType::Named(attribute_fields));

        template.force_empty_if_empty();

        Ok(ToTypeTemplateData {
            ident: None,
            template,
        })
    }
}

#[derive(Debug)]
pub struct SimpleContentFragmentHandler {
    pub simple_extension_handler: Arc<SimpleExtensionFragmentHandler>,
}

impl ComplexToTypeTemplate<cx::SimpleContentFragment> for SimpleContentFragmentHandler {
    type TypeTemplate = templates::group_record::GroupRecord;

    fn to_type_template<C: ComplexContext, S: Scope>(
        &self,
        context: &C,
        scope: &mut S,
        item: &cx::SimpleContentFragment,
    ) -> Result<ToTypeTemplateData<Self::TypeTemplate>> {
        match item.content_fragment {
            cx::SimpleContentChildId::Extension(fragment_idx) => self
                .simple_extension_handler
                .to_type_template(context, scope, &fragment_idx),
            cx::SimpleContentChildId::Restriction(_) => {
                Err(crate::Error::simple_content_restriction())
            }
        }
    }
}

#[derive(Debug)]
pub struct ComplexContentHandler {
    pub restriction_fragment_handler: Arc<RestrictionHandler>,
}

impl ComplexToTypeTemplate<cx::ComplexContentFragment> for ComplexContentHandler {
    type TypeTemplate = templates::group_record::GroupRecord;

    fn to_type_template<C: ComplexContext, S: Scope>(
        &self,
        context: &C,
        scope: &mut S,
        item: &cx::ComplexContentFragment,
    ) -> Result<ToTypeTemplateData<Self::TypeTemplate>> {
        match &item.content_fragment {
            cx::ComplexContentChildId::Extension(_fragment_idx) => {
                Err(crate::Error::UnsupportedFragment {
                    fragment: "ComplexContent Extension".to_string(),
                })
            }
            cx::ComplexContentChildId::Restriction(fragment_idx) => self
                .restriction_fragment_handler
                .resolve_type_template(context, scope, fragment_idx),
        }
    }
}

#[derive(Debug)]
pub struct ComplexTypeModelHandler {
    pub simple_content_handler: Arc<SimpleContentFragmentHandler>,
    pub complex_content_handler: Arc<ComplexContentHandler>,
    pub attribute_declarations_handler: Arc<AttributeDeclarationsHandler>,
    pub type_def_particle_handler: Arc<TypeDefParticleIdHandler>,
    pub other_content_type_naming: WrappingNamingStrategy,
    pub other_content_field_ident: String,
}

impl ComplexToTypeTemplate<cx::ComplexTypeModelId> for ComplexTypeModelHandler {
    type TypeTemplate = GroupRecord;

    fn to_type_template<C: ComplexContext, S: Scope>(
        &self,
        context: &C,
        scope: &mut S,
        item: &cx::ComplexTypeModelId,
    ) -> Result<ToTypeTemplateData<Self::TypeTemplate>> {
        match item {
            cx::ComplexTypeModelId::SimpleContent(fragment_idx) => self
                .simple_content_handler
                .resolve_type_template(context, scope, fragment_idx),
            cx::ComplexTypeModelId::ComplexContent(fragment_idx) => self
                .complex_content_handler
                .resolve_type_template(context, scope, fragment_idx),
            cx::ComplexTypeModelId::Other {
                particle,
                attr_decls,
                ..
            } => {
                let (ident, mut template) = particle
                    .as_ref()
                    .map(|particle| {
                        let sub_context = context.sub_context(
                            self.other_content_type_naming
                                .wrap_ident(context.suggested_ident()),
                        );

                        self.type_def_particle_handler
                            .to_type_template(&sub_context, scope, particle)
                            .map(|a| {
                                (
                                    a.ident,
                                    a.template.into_group_record(Some(format_ident!(
                                        "{}",
                                        self.other_content_field_ident
                                    ))),
                                )
                            })
                    })
                    .unwrap_or_else(|| Ok((None, GroupRecord::new_empty())))?;

                let attributes = self
                    .attribute_declarations_handler
                    .resolve_type_template(context, scope, attr_decls)?;

                template
                    .fields
                    .prefix_fields(ElementFieldType::Named(attributes.template));

                Ok(ToTypeTemplateData { ident, template })
            }
        }
    }
}

#[derive(Debug)]
pub struct ComplexTypeRootHandler {
    pub complex_type_model_handler: Arc<ComplexTypeModelHandler>,
}

impl ComplexToTypeTemplate<cx::ComplexTypeRootFragment> for ComplexTypeRootHandler {
    type TypeTemplate = GroupRecord;

    fn to_type_template<C: ComplexContext, S: Scope>(
        &self,
        context: &C,
        scope: &mut S,
        item: &cx::ComplexTypeRootFragment,
    ) -> Result<ToTypeTemplateData<Self::TypeTemplate>> {
        let mut fragment =
            self.complex_type_model_handler
                .to_type_template(context, scope, &item.content)?;

        let name_ident = item
            .name
            .as_ref()
            .map(|a| a.to_item_ident())
            .unwrap_or_else(|| context.suggested_ident().clone());
        fragment.ident = Some(name_ident);

        Ok(fragment)
    }
}
