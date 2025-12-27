use std::{fmt::Debug, sync::Arc};

use crate::{
    complex::{complex_type::ComplexTypeRootHandler, ComplexToTypeTemplateExt},
    misc::TypeReference,
    simple::{simple_type::SimpleTypeRootHandler, SimpleContext},
    templates::{
        choice,
        element_record::{
            AllowUnknown, ElementField, ElementFieldGroup, ElementFieldType, ElementRecord,
        },
        group_record::GroupRecord,
        value_record::{self, ItemFieldItem},
        ItemOrder,
    },
    Result, ToIdentTypesExt, TypeType,
};

use quote::format_ident;
use syn::Type;
use xsd_fragments::fragments::{complex as cx, FragmentIdx, FragmentedXsdDocumentIdx};

use super::{ComplexContext, ComplexToTypeTemplate, Scope, ToTypeTemplateData};

#[derive(Debug)]
pub struct ElementTypeContentIdHandler {
    pub simple_type_root_handler: Arc<SimpleTypeRootHandler>,
    pub complex_type_root_handler: Arc<ComplexTypeRootHandler>,
}

impl ComplexToTypeTemplate<cx::ElementTypeContentId> for ElementTypeContentIdHandler {
    type TypeTemplate = GroupRecord;

    fn to_type_template<C: ComplexContext, S: Scope>(
        &self,
        context: &C,
        scope: &mut S,
        item: &cx::ElementTypeContentId,
    ) -> Result<ToTypeTemplateData<Self::TypeTemplate>> {
        match item {
            cx::ElementTypeContentId::SimpleType(fragment_id) => context
                .simple_context()
                .resolve_type_template(fragment_id, scope, &*self.simple_type_root_handler)
                .map(|sub_type| ToTypeTemplateData {
                    ident: None,
                    template: GroupRecord::new_single_field(
                        None,
                        ElementField::Item(ItemFieldItem {
                            ty: sub_type.template,
                            default: false,
                            default_with: None,
                        }),
                    ),
                }),
            cx::ElementTypeContentId::ComplexType(fragment_idx) => self
                .complex_type_root_handler
                .resolve_type_template(context, scope, fragment_idx),
        }
    }
}

fn type_to_element_field(
    ty: TypeReference<'static>,
    ty_type: TypeType,
    default: bool,
    default_with: Option<syn::Path>,
) -> ElementField {
    match ty_type {
        TypeType::Simple => ElementField::Item(ItemFieldItem {
            ty,
            default,
            default_with,
        }),
        TypeType::Complex => ElementField::Group(ElementFieldGroup { ty }),
    }
}

#[derive(Debug)]
pub struct DeclaredElementHandler {
    pub element_type_content_handler: Arc<ElementTypeContentIdHandler>,
    pub allow_unknown_attributes: AllowUnknown,
    pub allow_unknown_children: AllowUnknown,
}

impl ComplexToTypeTemplate<(FragmentedXsdDocumentIdx, &cx::DeclaredElementFragment)>
    for DeclaredElementHandler
{
    type TypeTemplate = ElementRecord;

    fn to_type_template<C: ComplexContext, S: Scope>(
        &self,
        context: &C,
        scope: &mut S,
        (namespace_idx, item): &(FragmentedXsdDocumentIdx, &cx::DeclaredElementFragment),
    ) -> Result<ToTypeTemplateData<Self::TypeTemplate>> {
        let name = context.to_expanded_name(item.name.clone());
        let ident = item.name.to_item_ident();

        match &item.type_ {
            xsd_fragments::NamedOrAnonymous::Named(expanded_name) => {
                let bound_type =
                    context.resolve_named_type(namespace_idx, &expanded_name.as_ref())?;

                let field = type_to_element_field(bound_type.ty, bound_type.ty_type, false, None);

                let template = ElementRecord::new_single_field(name, None, field);

                Ok(ToTypeTemplateData {
                    ident: Some(ident),
                    template,
                })
            }
            xsd_fragments::NamedOrAnonymous::Anonymous(anonymous) => {
                let sub_type = self
                    .element_type_content_handler
                    .to_type_template(context, scope, anonymous)?;

                let mut template = sub_type.template.into_element_record(name);
                template.allow_unknown_attributes = self.allow_unknown_attributes;
                template.allow_unknown_children = self.allow_unknown_children;

                Ok(ToTypeTemplateData {
                    ident: Some(ident),
                    template,
                })
            }
        }
    }
}

#[derive(Debug)]
pub struct ReferenceElementHandler;

impl ComplexToTypeTemplate<(FragmentedXsdDocumentIdx, &cx::ReferenceElementFragment)>
    for ReferenceElementHandler
{
    type TypeTemplate = ItemFieldItem;

    fn to_type_template<C: ComplexContext, S: Scope>(
        &self,
        context: &C,
        _scope: &mut S,
        (namespace_idx, item): &(FragmentedXsdDocumentIdx, &cx::ReferenceElementFragment),
    ) -> Result<ToTypeTemplateData<Self::TypeTemplate>> {
        let ty = context.resolve_named_element(namespace_idx, &item.ref_.as_ref())?;

        let template = ItemFieldItem {
            ty,
            default: false,
            default_with: None,
        };

        Ok(ToTypeTemplateData {
            ident: Some(item.ref_.local_name().to_item_ident()),
            template,
        })
    }
}

pub enum LocalElementFragmentTemplate {
    ElementRecord {
        template: ElementRecord,
        min_occurs: usize,
        max_occurs: cx::AllNNI,
    },
    Item(ItemFieldItem),
}

#[derive(Debug)]
pub struct LocalElementHandler {
    pub declared_element_handler: Arc<DeclaredElementHandler>,
    pub reference_element_handler: Arc<ReferenceElementHandler>,
}

impl ComplexToTypeTemplate<FragmentIdx<cx::LocalElementFragment>> for LocalElementHandler {
    type TypeTemplate = LocalElementFragmentTemplate;

    fn to_type_template<C: ComplexContext, S: Scope>(
        &self,
        context: &C,
        scope: &mut S,
        fragment_idx: &FragmentIdx<cx::LocalElementFragment>,
    ) -> Result<ToTypeTemplateData<Self::TypeTemplate>> {
        let item = context.get_fragment(fragment_idx)?;

        let min_occurs = item.min_occurs.unwrap_or(1);
        let max_occurs = item.max_occurs.unwrap_or(cx::AllNNI::Bounded(1));

        match &item.type_ {
            cx::LocalElementFragmentType::Local(local) => {
                let local = self.declared_element_handler.to_type_template(
                    context,
                    scope,
                    &(fragment_idx.namespace_idx(), local),
                )?;

                Ok(ToTypeTemplateData {
                    ident: local.ident,
                    template: LocalElementFragmentTemplate::ElementRecord {
                        template: local.template,
                        min_occurs,
                        max_occurs,
                    },
                })
            }
            cx::LocalElementFragmentType::Reference(reference) => {
                let reference = self.reference_element_handler.to_type_template(
                    context,
                    scope,
                    &(fragment_idx.namespace_idx(), reference),
                )?;

                let (ty, optional) =
                    super::min_max_occurs_type(min_occurs, max_occurs, reference.template.ty);

                let template = LocalElementFragmentTemplate::Item(ItemFieldItem {
                    ty,
                    default: optional,
                    default_with: None,
                });

                Ok(ToTypeTemplateData {
                    ident: reference.ident,
                    template,
                })
            }
        }
    }
}

pub enum TopLevelElementTemplate {
    ElementRecord(ElementRecord),
    Choice(choice::Choice),
}

impl TopLevelElementTemplate {
    pub fn to_item(&self, item_name: &syn::Ident, path: Option<&syn::Path>) -> syn::Item {
        match self {
            TopLevelElementTemplate::ElementRecord(element_record) => {
                element_record.to_struct(item_name, path).into()
            }
            TopLevelElementTemplate::Choice(choice) => choice.to_enum(item_name, path).into(),
        }
    }
}

pub struct TopLevelElementHandler {
    pub dynamic_substitute_group: bool,
    pub standalone_element_type: bool,
    pub element_type_content_handler: Arc<ElementTypeContentIdHandler>,
    pub dynamic_variant_ident: String,
    pub substitution_group_wrapper: Arc<dyn Fn(Type) -> Type + 'static>,
    pub allow_unknown_attributes: AllowUnknown,
    pub allow_unknown_children: AllowUnknown,
}

impl Debug for TopLevelElementHandler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TopLevelElementHandler")
            .field("dynamic_substitute_group", &self.dynamic_substitute_group)
            .field("standalone_element_type", &self.standalone_element_type)
            .field("dynamic_variant_ident", &self.dynamic_variant_ident)
            .field("substitution_group", &"<Fn(Type) -> Type>")
            .finish()
    }
}

impl ComplexToTypeTemplate<FragmentIdx<cx::TopLevelElementFragment>> for TopLevelElementHandler {
    type TypeTemplate = TopLevelElementTemplate;

    fn to_type_template<C: ComplexContext, S: Scope>(
        &self,
        context: &C,
        scope: &mut S,
        fragment_idx: &FragmentIdx<cx::TopLevelElementFragment>,
    ) -> Result<ToTypeTemplateData<Self::TypeTemplate>> {
        let item = context.get_fragment(fragment_idx)?;

        let name = context.to_expanded_name(item.name.clone());
        let ident = item.name.to_item_ident();

        let type_ = item.type_.as_ref();
        let self_type =
            context.resolve_named_element(&fragment_idx.namespace_idx(), &name.as_ref())?;

        let mut substitution_choices = Vec::new();

        substitution_choices.extend(
            context
                .substitution_group_members(&name.as_ref())?
                .map(|(id, a)| {
                    let element_type = context.resolve_named_element(&id, &a)?;
                    Ok((
                        a.local_name().to_variant_ident(),
                        choice::ChoiceVariantType::Item(
                            value_record::ItemRecord::new_single_field(
                                None,
                                value_record::ItemField::Item(ItemFieldItem {
                                    ty: element_type,
                                    default: false,
                                    default_with: None,
                                }),
                            ),
                        ),
                    ))
                })
                .collect::<Result<Vec<_>>>()?,
        );

        if self.dynamic_substitute_group {
            let substitute_group = self.substitution_group_wrapper.clone();
            let substitute_group_ty = self_type.wrap(move |ty| (substitute_group)(ty));

            substitution_choices.push((
                format_ident!("{}", self.dynamic_variant_ident),
                choice::ChoiceVariantType::Item(value_record::ItemRecord::new_single_field(
                    None,
                    value_record::ItemField::Item(ItemFieldItem {
                        ty: substitute_group_ty,
                        default: false,
                        default_with: None,
                    }),
                )),
            ));
        }

        let mut element_record = (!item.abstract_)
            .then(|| match type_ {
                Some(xsd_fragments::NamedOrAnonymous::Named(expanded_name)) => {
                    let bound_type = context.resolve_named_type(
                        &fragment_idx.namespace_idx(),
                        &expanded_name.as_ref(),
                    )?;

                    let field =
                        type_to_element_field(bound_type.ty, bound_type.ty_type, false, None);

                    Ok(ElementRecord {
                        name,
                        attribute_order: ItemOrder::None,
                        children_order: ItemOrder::None,
                        fields: ElementFieldType::Unnamed(vec![field]),
                        allow_unknown_attributes: AllowUnknown::Any,
                        allow_unknown_children: AllowUnknown::AtEnd,
                    })
                }
                Some(xsd_fragments::NamedOrAnonymous::Anonymous(anonymous)) => {
                    let sub_type = self
                        .element_type_content_handler
                        .to_type_template(context, scope, anonymous)?;

                    if self.standalone_element_type {
                        let type_ = sub_type.template.to_struct(&ident, None);

                        let ty = scope.add_item(type_)?;

                        Ok(ElementRecord {
                            name,
                            attribute_order: ItemOrder::None,
                            children_order: ItemOrder::None,
                            fields: ElementFieldType::Unnamed(vec![ElementField::Group(
                                ElementFieldGroup { ty },
                            )]),
                            allow_unknown_attributes: AllowUnknown::Any,
                            allow_unknown_children: AllowUnknown::AtEnd,
                        })
                    } else {
                        Ok(sub_type.template.into_element_record(name))
                    }
                }
                None => Ok(ElementRecord::new_empty(name)),
            })
            .transpose()?;

        if let Some(element_record) = &mut element_record {
            element_record.allow_unknown_attributes = self.allow_unknown_attributes;
            element_record.allow_unknown_children = self.allow_unknown_children;
        }

        if !substitution_choices.is_empty() || element_record.is_none() {
            let element_variant = element_record.map(|element_record| {
                (
                    ident.to_item_ident(),
                    choice::ChoiceVariantType::Element(element_record),
                )
            });

            let choice = choice::Choice {
                variants: element_variant
                    .into_iter()
                    .chain(substitution_choices)
                    .collect(),
            };

            Ok(ToTypeTemplateData {
                ident: Some(ident),
                template: TopLevelElementTemplate::Choice(choice),
            })
        } else {
            Ok(ToTypeTemplateData {
                ident: Some(ident),
                template: TopLevelElementTemplate::ElementRecord(
                    element_record.expect(
                        "Element record should be present due to condition in if statement",
                    ),
                ),
            })
        }
    }
}
