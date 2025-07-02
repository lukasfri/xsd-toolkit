use crate::{
    finish_mod,
    misc::{common_name, dedup_field_idents, TypeReference, COMMON_NAME_MIN_LENGTH},
    templates::{
        self,
        choice::{self, ChoiceVariantType},
        element_record::ElementRecord,
        value_record::{ItemField, ItemFieldItem, ItemFieldType, ItemRecord},
        ItemOrder,
    },
    GeneratorScope, Result, ToIdentTypesExt,
};

use quote::format_ident;
use syn::{parse_quote, Item, Path};
use xsd_type_compiler::fragments::complex::{self as cx, AllNNI};

use super::{
    elements::LocalElementFragmentTemplate, ComplexContext, ComplexToTypeTemplate, Scope,
    ToTypeTemplateData,
};

impl ComplexToTypeTemplate for cx::AllFragment {
    type TypeTemplate = ItemOrTemplate<ItemRecord>;

    fn to_type_template<C: ComplexContext, S: Scope>(
        &self,
        context: &C,
        scope: &mut S,
    ) -> Result<ToTypeTemplateData<Self::TypeTemplate>> {
        let mut sub_scope = GeneratorScope::new(scope.augmenter());

        // Struct with strict order
        let fields = self
            .fragments
            .iter()
            .enumerate()
            .map(|(i, fragment_id)| {
                let suggested_ident = format_ident!("Child{i}");
                let res = context
                    .sub_context(suggested_ident.clone())
                    .resolve_fragment(fragment_id, &mut sub_scope)?;

                let ident = res.ident.unwrap_or(suggested_ident);

                let item = res.template.into_item(
                    context,
                    &mut sub_scope,
                    &ident.to_item_ident(),
                    None,
                )?;

                Ok(((ident.to_field_ident(), item), ident))
            })
            .collect::<Result<Vec<_>>>()?;

        let (fields, names) = fields.into_iter().unzip::<_, _, Vec<_>, Vec<_>>();

        let common_name = common_name(names.iter().map(|a| a.to_string()), COMMON_NAME_MIN_LENGTH);

        let ident = common_name
            .map(|a| format_ident!("{a}"))
            .unwrap_or_else(|| context.suggested_ident().clone())
            .to_item_ident();

        let fields = dedup_field_idents(fields);

        let mut template = templates::value_record::ItemRecord {
            children_order: ItemOrder::None,
            fields: ItemFieldType::Named(fields),
        };

        template.force_empty_if_empty();

        let sub_scope_items = sub_scope.finish();

        let mod_name = format_ident!("{}_items", ident.to_path_ident());

        let mod_path: syn::Path = parse_quote!(#mod_name);

        let min_occurs = self.min_occurs.unwrap_or(1);
        let max_occurs = self.max_occurs.unwrap_or_default();

        let template = ItemOrTemplate::new(
            template,
            |template| syn::Item::Struct(template.to_struct(&ident, Some(&mod_path))),
            min_occurs,
            max_occurs,
            scope,
        )?;

        if matches!(template, ItemOrTemplate::Template(_)) {
            scope.add_raw_items(sub_scope_items);
        } else {
            let _mod_ref = finish_mod(&mod_name, sub_scope_items)
                .map(|a| scope.add_item(a))
                .transpose()?;
        }

        Ok(ToTypeTemplateData {
            ident: Some(ident),
            template,
        })
    }
}

pub enum ItemOrTemplate<T> {
    Template(T),
    Item(templates::value_record::ItemFieldItem),
}

impl<T> ItemOrTemplate<T> {
    pub fn new<S: Scope>(
        template: T,
        template_to_item: impl FnOnce(T) -> syn::Item,
        min_occurs: usize,
        max_occurs: AllNNI,
        scope: &mut S,
    ) -> Result<Self> {
        match (min_occurs, max_occurs) {
            (1, AllNNI::Bounded(1)) => Ok(Self::Template(template)),
            (min, max) => {
                let ty = scope.add_item(template_to_item(template))?;

                match (min, max) {
                    (0, AllNNI::Bounded(1)) => Ok(Self::Item(ItemFieldItem {
                        ty: ty.wrap(TypeReference::option_wrapper),
                        default: true,
                    })),
                    (1, AllNNI::Bounded(1)) => unreachable!(),
                    _ => Ok(Self::Item(ItemFieldItem {
                        ty: ty.wrap(TypeReference::vec_non_boxed_wrapper),
                        default: true,
                    })),
                }
            }
        }
    }
}

impl ComplexToTypeTemplate for cx::SequenceFragment {
    type TypeTemplate = ItemOrTemplate<ItemRecord>;

    fn to_type_template<C: ComplexContext, S: Scope>(
        &self,
        context: &C,
        scope: &mut S,
    ) -> Result<ToTypeTemplateData<Self::TypeTemplate>> {
        let mut sub_scope = GeneratorScope::new(scope.augmenter());

        // Struct with strict order
        let fields = self
            .fragments
            .iter()
            .enumerate()
            .map(|(i, fragment_id)| {
                let suggested_ident = format_ident!("Child{i}");
                let res = context
                    .sub_context(suggested_ident.clone())
                    .resolve_fragment(fragment_id, &mut sub_scope)?;

                let ident = res.ident.unwrap_or(suggested_ident);

                let item = res.template.into_item(
                    context,
                    &mut sub_scope,
                    &ident.to_item_ident(),
                    None,
                )?;

                Ok(((ident.to_field_ident(), item), ident))
            })
            .collect::<Result<Vec<_>>>()?;

        let (fields, names) = fields.into_iter().unzip::<_, _, Vec<_>, Vec<_>>();

        let common_name = common_name(names.iter().map(|a| a.to_string()), COMMON_NAME_MIN_LENGTH);

        let ident = common_name
            .map(|a| format_ident!("{a}"))
            .unwrap_or_else(|| context.suggested_ident().clone())
            .to_item_ident();

        let fields = dedup_field_idents(fields);

        let mut template = templates::value_record::ItemRecord {
            children_order: ItemOrder::Strict,
            fields: ItemFieldType::Named(fields),
        };

        template.force_empty_if_empty();

        let sub_scope_items = sub_scope.finish();

        let mod_name = format_ident!("{}_items", ident.to_path_ident());

        let mod_path: syn::Path = parse_quote!(#mod_name);

        let min_occurs = self.min_occurs.unwrap_or(1);
        let max_occurs = self.max_occurs.unwrap_or_default();

        let template = ItemOrTemplate::new(
            template,
            |template| syn::Item::Struct(template.to_struct(&ident, Some(&mod_path))),
            min_occurs,
            max_occurs,
            scope,
        )?;

        if matches!(template, ItemOrTemplate::Template(_)) {
            scope.add_raw_items(sub_scope_items);
        } else {
            let _mod_ref = finish_mod(&mod_name, sub_scope_items)
                .map(|a| scope.add_item(a))
                .transpose()?;
        }

        Ok(ToTypeTemplateData {
            ident: Some(ident),
            template,
        })
    }
}

impl ComplexToTypeTemplate for cx::ChoiceFragment {
    type TypeTemplate = ItemOrTemplate<templates::choice::Choice>;

    fn to_type_template<C: ComplexContext, S: Scope>(
        &self,
        context: &C,
        scope: &mut S,
    ) -> Result<ToTypeTemplateData<Self::TypeTemplate>> {
        let mut sub_scope = GeneratorScope::new(scope.augmenter());

        let variants = self
            .fragments
            .iter()
            .enumerate()
            .map(|(i, fragment_id)| {
                let suggested_ident = format_ident!("Variant{i}");
                let res = context
                    .sub_context(suggested_ident.clone())
                    .resolve_fragment(fragment_id, &mut sub_scope)?;

                let ident = res.ident.unwrap_or(suggested_ident);

                Ok(((ident.to_variant_ident(), res.template), ident))
            })
            .collect::<Result<Vec<_>>>()?;

        let (variants, names) = variants.into_iter().unzip::<_, _, Vec<_>, Vec<_>>();

        let common_name = common_name(names.iter().map(|a| a.to_string()), COMMON_NAME_MIN_LENGTH);

        let ident = common_name
            .map(|a| format_ident!("{a}"))
            .unwrap_or_else(|| context.suggested_ident().clone())
            .to_item_ident();

        let mod_name = format_ident!("{}_variants", ident.to_path_ident());

        let mod_path: syn::Path = parse_quote!(#mod_name);

        let variants = variants
            .into_iter()
            .map(|(ident, variant)| {
                let variant =
                    variant.into_variant(context, &mut sub_scope, &ident, Some(&mod_path))?;

                Ok((ident, variant))
            })
            .collect::<Result<Vec<_>>>()?;

        let variants = dedup_field_idents(variants);

        let template = templates::choice::Choice { variants };

        let _mod_ref = sub_scope
            .finish_mod(&mod_name)
            .map(|a| scope.add_item(a))
            .transpose()?;

        let min_occurs = self.min_occurs.unwrap_or(1);
        let max_occurs = self.max_occurs.unwrap_or(AllNNI::Bounded(1));

        let template = if min_occurs != 1 || max_occurs != AllNNI::Bounded(1) {
            let item = template.to_enum(&ident, None);

            let ty = scope.add_item(item)?;

            let (ty, optional) = super::min_max_occurs_type(min_occurs, max_occurs, ty);

            ItemOrTemplate::Item(ItemFieldItem {
                ty,
                default: optional,
            })
        } else {
            ItemOrTemplate::Template(template)
        };

        Ok(ToTypeTemplateData {
            ident: Some(ident),
            template,
        })
    }
}

impl ComplexToTypeTemplate for cx::AnyFragment {
    type TypeTemplate = TypeReference<'static>;

    fn to_type_template<C: ComplexContext, S: Scope>(
        &self,
        context: &C,
        _scope: &mut S,
    ) -> Result<ToTypeTemplateData<Self::TypeTemplate>> {
        Ok(ToTypeTemplateData {
            ident: Some(context.suggested_ident().clone()),
            template: TypeReference::new_static(parse_quote!(::xmlity::XmlValue)),
        })
    }
}

impl ComplexToTypeTemplate for cx::GroupRefFragment {
    type TypeTemplate = ItemFieldItem;

    fn to_type_template<C: ComplexContext, S: Scope>(
        &self,
        context: &C,
        _scope: &mut S,
    ) -> Result<ToTypeTemplateData<Self::TypeTemplate>> {
        let min_occurs = self.min_occurs.unwrap_or(1);
        let max_occurs = self.max_occurs.unwrap_or_default();

        //TODO: Should not wrap in box if we also wrap it in Vec.
        let ty = context.resolve_named_group(&self.ref_)?;

        let (ty, optional) = super::min_max_occurs_type(min_occurs, max_occurs, ty);

        let template = ItemFieldItem {
            ty,
            default: optional,
        };

        let ident = self.ref_.local_name().to_item_ident();

        Ok(ToTypeTemplateData {
            ident: Some(ident),
            template,
        })
    }
}

#[derive(Debug)]
pub enum GroupTypeContentTemplate {
    // Record(ItemRecord),
    ElementRecord {
        record: ElementRecord,
        min_occurs: usize,
        max_occurs: AllNNI,
    },
    Item(ItemFieldItem),
}

impl GroupTypeContentTemplate {
    fn into_variant<C: ComplexContext, S: Scope>(
        self,
        _context: &C,
        scope: &mut S,
        ident: &syn::Ident,
        path: Option<&syn::Path>,
    ) -> Result<choice::ChoiceVariantType> {
        match self {
            GroupTypeContentTemplate::ElementRecord {
                record,
                min_occurs,
                max_occurs,
            } => {
                if min_occurs == 1 && max_occurs == AllNNI::Bounded(1) {
                    Ok(ChoiceVariantType::Element(record))
                } else {
                    let ty = scope.add_item(record.to_struct(ident, path))?;

                    let ty = ty.wrap(TypeReference::box_non_boxed_wrapper);

                    let (ty, default) = super::min_max_occurs_type(min_occurs, max_occurs, ty);

                    Ok(ChoiceVariantType::Item(ItemRecord::new_single_field(
                        None,
                        ItemField::Item(ItemFieldItem { ty, default }),
                    )))
                }
            }
            GroupTypeContentTemplate::Item(mut item) => {
                if let Some(path) = path {
                    item.ty = item
                        .ty
                        .wrap(TypeReference::box_non_boxed_wrapper)
                        .prefix(path.clone());
                }

                Ok(ChoiceVariantType::Item(ItemRecord::new_single_field(
                    None,
                    ItemField::Item(item),
                )))
            }
        }
    }

    fn into_item<C: ComplexContext, S: Scope>(
        self,
        _context: &C,
        scope: &mut S,
        ident: &syn::Ident,
        path: Option<&syn::Path>,
    ) -> Result<ItemField> {
        match self {
            GroupTypeContentTemplate::ElementRecord {
                record,
                min_occurs,
                max_occurs,
            } => {
                let record = if max_occurs == AllNNI::Bounded(1) {
                    let optional = min_occurs == 0;
                    let res = record.try_into_compact_item_field(optional);

                    match res {
                        Ok(mut field_element) => {
                            field_element.ty = field_element
                                .ty
                                .wrap_if(optional, TypeReference::option_wrapper);
                            field_element.optional = optional;

                            return Ok(ItemField::Element(field_element));
                        }
                        Err(item) => item,
                    }
                } else {
                    record
                };

                let ty = scope.add_item(record.to_struct(ident, path))?;

                let (ty, optional) = super::min_max_occurs_type(min_occurs, max_occurs, ty);

                Ok(ItemField::Item(ItemFieldItem {
                    ty,
                    default: optional,
                }))
            }
            GroupTypeContentTemplate::Item(mut item) => {
                if let Some(path) = path {
                    item.ty = item.ty.prefix(path.clone());
                }

                Ok(ItemField::Item(item))
            }
        }
    }
}

impl From<LocalElementFragmentTemplate> for GroupTypeContentTemplate {
    fn from(value: LocalElementFragmentTemplate) -> Self {
        match value {
            LocalElementFragmentTemplate::ElementRecord {
                max_occurs,
                min_occurs,
                template,
            } => GroupTypeContentTemplate::ElementRecord {
                record: template,
                min_occurs,
                max_occurs,
            },
            LocalElementFragmentTemplate::Item(item) => GroupTypeContentTemplate::Item(item),
        }
    }
}

impl ComplexToTypeTemplate for cx::NestedParticleId {
    type TypeTemplate = GroupTypeContentTemplate;

    fn to_type_template<C: ComplexContext, S: Scope>(
        &self,
        context: &C,
        scope: &mut S,
    ) -> Result<ToTypeTemplateData<Self::TypeTemplate>> {
        let template = match self {
            cx::NestedParticleId::Element(fragment_idx) => context
                .resolve_fragment_id(fragment_idx, scope)
                .map(|res| ToTypeTemplateData {
                    ident: res.ident,
                    template: res.template.into(),
                })?,
            cx::NestedParticleId::Group(fragment_idx) => context
                .resolve_fragment_id(fragment_idx, scope)
                .map(|res| ToTypeTemplateData {
                    ident: res.ident,
                    template: GroupTypeContentTemplate::Item(res.template),
                })?,
            // cx::NestedParticleId::All(fragment_idx) => {
            //     let record = context.resolve_fragment_id(fragment_idx, scope)?;

            //     let ident = record
            //         .ident
            //         .unwrap_or_else(|| context.suggested_ident().clone());

            //     let item = match record.template {
            //         ItemOrTemplate::Template(record) => {
            //             let item = record.to_struct(&ident, None);

            //             let ty = scope.add_item(item)?;

            //             ItemFieldItem { ty, default: false }
            //         }
            //         ItemOrTemplate::Item(item) => item,
            //     };

            //     let template = GroupTypeContentTemplate::Item(item);

            //     ToTypeTemplateData {
            //         ident: Some(ident),
            //         template,
            //     }
            // }
            cx::NestedParticleId::Choice(fragment_idx) => {
                let record = context.resolve_fragment_id(fragment_idx, scope)?;

                let ident = record
                    .ident
                    .unwrap_or_else(|| context.suggested_ident().clone());

                let item = match record.template {
                    ItemOrTemplate::Template(record) => {
                        let item = record.to_enum(&ident, None);

                        let ty = scope.add_item(item)?;

                        ItemFieldItem { ty, default: false }
                    }
                    ItemOrTemplate::Item(item) => item,
                };

                let template = GroupTypeContentTemplate::Item(item);

                ToTypeTemplateData {
                    ident: Some(ident),
                    template,
                }
            }
            cx::NestedParticleId::Sequence(fragment_idx) => {
                let mut sub_scope = GeneratorScope::new(scope.augmenter());

                let mod_name = format_ident!("{}_items", context.suggested_ident().to_path_ident());

                let mod_path: syn::Path = parse_quote!(#mod_name);

                let record = context.resolve_fragment_id(fragment_idx, &mut sub_scope)?;

                let ident = record
                    .ident
                    .unwrap_or_else(|| context.suggested_ident().clone());

                let item = match record.template {
                    ItemOrTemplate::Template(record) => {
                        let item = record.to_struct(&ident, Some(&mod_path));

                        sub_scope
                            .finish_mod(&mod_name)
                            .map(|a| scope.add_item(a))
                            .transpose()?;

                        let ty = scope.add_item(item)?;

                        ItemFieldItem { ty, default: false }
                    }
                    ItemOrTemplate::Item(item) => {
                        scope.add_raw_items(sub_scope.finish());

                        item
                    }
                };

                let template = GroupTypeContentTemplate::Item(item);

                ToTypeTemplateData {
                    ident: Some(ident),
                    template,
                }
            }
            cx::NestedParticleId::Any(fragment_idx) => {
                let template = context.resolve_fragment_id(fragment_idx, scope)?;

                let ty = template.template;

                ToTypeTemplateData {
                    ident: template.ident,
                    template: GroupTypeContentTemplate::Item(ItemFieldItem { ty, default: false }),
                }
            }
        };

        Ok(template)
    }
}

#[derive(Debug)]
pub enum TypeDefParticleTemplate {
    Record(ItemRecord),
    Choice(templates::choice::Choice),
    Item(ItemFieldItem),
}

impl TypeDefParticleTemplate {
    pub fn into_group_record(
        self,
        ident: Option<syn::Ident>,
    ) -> templates::group_record::GroupRecord {
        match self {
            TypeDefParticleTemplate::Record(item_record) => item_record.into_group_record(),
            TypeDefParticleTemplate::Choice(_) => todo!(),
            TypeDefParticleTemplate::Item(item_field_item) => {
                item_field_item.into_group_record(ident)
            }
        }
    }
}

impl ComplexToTypeTemplate for cx::TypeDefParticleId {
    type TypeTemplate = TypeDefParticleTemplate;

    fn to_type_template<C: ComplexContext, S: Scope>(
        &self,
        context: &C,
        scope: &mut S,
    ) -> Result<ToTypeTemplateData<Self::TypeTemplate>> {
        match self {
            cx::TypeDefParticleId::Group(group) => {
                let group = context.resolve_fragment_id(group, scope)?;

                let template = TypeDefParticleTemplate::Item(group.template);

                Ok(ToTypeTemplateData {
                    ident: group.ident,
                    template,
                })
            }
            cx::TypeDefParticleId::All(fragment_idx) => {
                let all = context.resolve_fragment_id(fragment_idx, scope)?;

                let template = match all.template {
                    ItemOrTemplate::Template(record) => TypeDefParticleTemplate::Record(record),
                    ItemOrTemplate::Item(item) => TypeDefParticleTemplate::Item(item),
                };

                Ok(ToTypeTemplateData {
                    ident: all.ident,
                    template,
                })
            }
            cx::TypeDefParticleId::Sequence(fragment_idx) => {
                let sequence = context.resolve_fragment_id(fragment_idx, scope)?;

                let template = match sequence.template {
                    ItemOrTemplate::Template(record) => TypeDefParticleTemplate::Record(record),
                    ItemOrTemplate::Item(item) => TypeDefParticleTemplate::Item(item),
                };

                Ok(ToTypeTemplateData {
                    ident: sequence.ident,
                    template,
                })
            }
            cx::TypeDefParticleId::Choice(fragment_idx) => {
                let choice = context.resolve_fragment_id(fragment_idx, scope)?;

                let ident = choice
                    .ident
                    .as_ref()
                    .unwrap_or_else(|| context.suggested_ident());

                let template = match choice.template {
                    ItemOrTemplate::Template(record) => TypeDefParticleTemplate::Choice(record),
                    ItemOrTemplate::Item(item) => TypeDefParticleTemplate::Item(item),
                };

                Ok(ToTypeTemplateData {
                    ident: Some(ident.clone()),
                    template,
                })
            }
        }
    }
}

impl ComplexToTypeTemplate for cx::NamedGroupTypeContentId {
    type TypeTemplate = TypeDefParticleTemplate;

    fn to_type_template<C: ComplexContext, S: Scope>(
        &self,
        context: &C,
        scope: &mut S,
    ) -> Result<ToTypeTemplateData<Self::TypeTemplate>> {
        match self {
            cx::NamedGroupTypeContentId::All(fragment_idx) => {
                let all = context.resolve_fragment_id(fragment_idx, scope)?;

                let template = match all.template {
                    ItemOrTemplate::Template(record) => TypeDefParticleTemplate::Record(record),
                    ItemOrTemplate::Item(item) => TypeDefParticleTemplate::Item(item),
                };

                Ok(ToTypeTemplateData {
                    ident: all.ident,
                    template,
                })
            }
            cx::NamedGroupTypeContentId::Sequence(fragment_idx) => {
                let sequence = context.resolve_fragment_id(fragment_idx, scope)?;

                let template = match sequence.template {
                    ItemOrTemplate::Template(record) => TypeDefParticleTemplate::Record(record),
                    ItemOrTemplate::Item(item) => TypeDefParticleTemplate::Item(item),
                };

                Ok(ToTypeTemplateData {
                    ident: sequence.ident,
                    template,
                })
            }
            cx::NamedGroupTypeContentId::Choice(fragment_idx) => {
                let choice = context.resolve_fragment_id(fragment_idx, scope)?;

                let template = match choice.template {
                    ItemOrTemplate::Template(record) => TypeDefParticleTemplate::Choice(record),
                    ItemOrTemplate::Item(item) => TypeDefParticleTemplate::Item(item),
                };

                Ok(ToTypeTemplateData {
                    ident: choice.ident,
                    template,
                })
            }
        }
    }
}

pub enum TopLevelGroupTemplate {
    ItemRecord(ItemRecord),
    Choice(choice::Choice),
}

impl TopLevelGroupTemplate {
    pub fn to_item(&self, ident: &syn::Ident, path: Option<&Path>) -> Item {
        match self {
            TopLevelGroupTemplate::ItemRecord(item_record) => {
                Item::Struct(item_record.to_struct(ident, path))
            }
            TopLevelGroupTemplate::Choice(choice) => Item::Enum(choice.to_enum(ident, path)),
        }
    }
}

impl ComplexToTypeTemplate for cx::TopLevelGroupFragment {
    type TypeTemplate = TopLevelGroupTemplate;

    fn to_type_template<C: ComplexContext, S: Scope>(
        &self,
        context: &C,
        scope: &mut S,
    ) -> Result<ToTypeTemplateData<Self::TypeTemplate>> {
        let ident = self.name.as_ref().to_item_ident();

        let fragment = context
            .sub_context(ident.clone())
            .resolve_fragment(&self.content, scope)?;

        let template = match fragment.template {
            TypeDefParticleTemplate::Record(item_record) => {
                TopLevelGroupTemplate::ItemRecord(item_record.into_item_record())
            }
            TypeDefParticleTemplate::Choice(choice) => TopLevelGroupTemplate::Choice(choice),
            TypeDefParticleTemplate::Item(item_field_item) => {
                TopLevelGroupTemplate::ItemRecord(item_field_item.into_item_record(None))
            }
        };

        Ok(ToTypeTemplateData {
            ident: Some(ident),
            template,
        })
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use syn::parse_quote;
    use xmlity::{LocalName, XmlNamespace};
    use xsd::xs;
    use xsd::xsn;
    use xsd_type_compiler::{CompiledNamespace, XmlnsContext};

    use crate::Generator;

    #[test]
    fn three_choice_sequence_deep_top_level_type() {
        let sequence = xs::types::TopLevelComplexType::builder()
            .name(LocalName::new_dangerous("SimpleSequence"))
            .complex_type_model(
                Box::new(
                xs::ComplexContent::builder()
                    .child_1(
                        xs::types::ComplexRestrictionType::builder()
                            .base(xs::types::QName(xsn::ANY_TYPE.clone()))
                            .child_1(
                                xs::types::complex_restriction_type_items::Child1::builder()
                                .type_def_particle(
                                    Box::new(
                                xs::Sequence(
                                    xs::types::ExplicitGroup::builder()
                                        .nested_particle(vec![
                                            xs::Choice(xs::types::ExplicitGroup::builder()
                                                .nested_particle(vec![
                                                    xs::Sequence(xs::types::ExplicitGroup::builder()
                                                        .nested_particle(vec![xs::types::LocalElement::builder()
                                                            .name(LocalName::new_dangerous("a"))
                                                            .type_attribute(xs::types::QName(
                                                                xsn::INTEGER.clone(),
                                                            ))
                                                            .build()
                                                            .into()])
                                                        .build()
                                                        .into()).into(),
                                                    xs::types::LocalElement::builder()
                                                        .name(LocalName::new_dangerous("b"))
                                                        .type_attribute(xs::types::QName(
                                                            xsn::STRING.clone(),
                                                        ))
                                                        .build()
                                                        .into(),
                                                ])
                                                .build()
                                                .into()).into(),
                                            xs::types::LocalElement::builder()
                                                .name(LocalName::new_dangerous("c"))
                                                .type_attribute(xs::types::QName(xsn::STRING.clone()))
                                                .build()
                                                .into(),
                                        ])
                                        .build()
                                        .into(),
                                )
                                .into())
                                ).build()
                                .into(),
                            )
                                    .attr_decls(xs::groups::AttrDecls::builder().build().into())
                                    .assertions(xs::groups::Assertions::builder().build().into())
                            .build()
                            .into(),
                    )
                    .build()
                    .into(),
                )
            )
            .build()
            .into();

        let namespace = XmlNamespace::new_dangerous("http://example.com");

        let mut compiled_namespace = CompiledNamespace::new(namespace.clone());

        let sequence = compiled_namespace
            .import_top_level_complex_type(&sequence)
            .unwrap()
            .into_owned();

        let mut context = XmlnsContext::new();
        context.add_namespace(compiled_namespace);

        let mut generator = Generator::new(&context);

        generator.bind_types(crate::binds::StdXsdTypes);

        let (type_, actual_items) = generator.generate_top_level_type(&sequence).unwrap();

        let actual = prettyplease::unparse(&syn::File {
            shebang: None,
            attrs: Vec::new(),
            items: actual_items.clone(),
        });

        #[rustfmt::skip]
        let expected: syn::File = parse_quote!(
            pub mod simple_sequence_items {
                pub mod child_0_variants {
                    #[derive(::core::fmt::Debug, ::xmlity::Serialize, ::xmlity::Deserialize)]
                    #[xvalue(order = "strict")]
                    pub struct A {
                        #[xelement(name = "a", namespace = "http://example.com")]
                        pub a: i32,
                    }
                }

                #[derive(::core::fmt::Debug, ::xmlity::Serialize, ::xmlity::Deserialize)]
                pub enum Child0 {
                    A(::std::boxed::Box<child_0_variants::A>),
                    #[xelement(
                        name = "b",
                        namespace = "http://example.com",
                        allow_unknown_attributes = "any"
                    )]
                    B(String),
                }
            }

            #[derive(::core::fmt::Debug, ::xmlity::SerializationGroup, ::xmlity::DeserializationGroup)]
            #[xgroup(children_order = "strict")]
            pub struct SimpleSequence {
                pub child_0: simple_sequence_items::Child0,
                #[xelement(name = "c", namespace = "http://example.com")]
                pub c: String,
            }
        );

        let expected = prettyplease::unparse(&expected);

        assert_eq!(actual, expected);

        assert_eq!(
            type_.ty.into_type(None),
            parse_quote!(::std::boxed::Box<SimpleSequence>)
        );
    }

    #[test]
    fn two_sequence_deep_top_level_type() {
        let sequence =
            xs::types::TopLevelComplexType::builder()
                .name(LocalName::new_dangerous("SimpleSequence"))
                .complex_type_model(Box::new(
                    xs::ComplexContent::builder()
                        .child_1(
                            xs::types::ComplexRestrictionType::builder()
                                .base(xs::types::QName(xsn::ANY_TYPE.clone()))
                                .child_1(
                                    xs::types::complex_restriction_type_items::Child1::builder()
                                        .type_def_particle(Box::new(
                                            xs::Sequence(
                                                xs::types::ExplicitGroup::builder()
                                                    .nested_particle(vec![
                                                        xs::Sequence(
                                                            xs::types::ExplicitGroup::builder()
                                                                .nested_particle(vec![
                                                    xs::types::LocalElement::builder()
                                                        .name(LocalName::new_dangerous("a"))
                                                        .type_attribute(xs::types::QName(
                                                            xsn::INTEGER.clone(),
                                                        ))
                                                        .min_occurs(0)
                                                        .build()
                                                        .into(),
                                                    xs::types::LocalElement::builder()
                                                        .name(LocalName::new_dangerous("b"))
                                                        .type_attribute(xs::types::QName(
                                                            xsn::STRING.clone(),
                                                        ))
                                                        .build()
                                                        .into(),
                                                ])
                                                                .min_occurs(0)
                                                                .build()
                                                                .into(),
                                                        )
                                                        .into(),
                                                        xs::types::LocalElement::builder()
                                                            .name(LocalName::new_dangerous("c"))
                                                            .type_attribute(xs::types::QName(
                                                                xsn::STRING.clone(),
                                                            ))
                                                            .build()
                                                            .into(),
                                                    ])
                                                    .build()
                                                    .into(),
                                            )
                                            .into(),
                                        ))
                                        .build()
                                        .into(),
                                )
                                .attr_decls(xs::groups::AttrDecls::builder().build().into())
                                .assertions(xs::groups::Assertions::builder().build().into())
                                .build()
                                .into(),
                        )
                        .build()
                        .into(),
                ))
                .build()
                .into();

        let namespace = XmlNamespace::new_dangerous("http://example.com");

        let mut compiled_namespace = CompiledNamespace::new(namespace.clone());

        let sequence = compiled_namespace
            .import_top_level_complex_type(&sequence)
            .unwrap()
            .into_owned();

        let mut context = XmlnsContext::new();
        context.add_namespace(compiled_namespace);

        let mut generator = Generator::new(&context);

        generator.bind_types(crate::binds::StdXsdTypes);

        let (type_, actual_items) = generator.generate_top_level_type(&sequence).unwrap();

        let actual = prettyplease::unparse(&syn::File {
            shebang: None,
            attrs: Vec::new(),
            items: actual_items.clone(),
        });

        #[rustfmt::skip]
        let expected: syn::File = parse_quote!(
            pub mod simple_sequence_items {
                #[derive(::core::fmt::Debug, ::xmlity::Serialize, ::xmlity::Deserialize)]
                #[xvalue(order = "strict")]
                pub struct Child0 {
                    #[xelement(name = "a", namespace = "http://example.com", optional)]
                    pub a: ::core::option::Option<i32>,
                    #[xelement(name = "b", namespace = "http://example.com")]
                    pub b: String,
                }
            }
            
            #[derive(::core::fmt::Debug, ::xmlity::SerializationGroup, ::xmlity::DeserializationGroup)]
            #[xgroup(children_order = "strict")]
            pub struct SimpleSequence {
                #[xvalue(default)]
                pub child_0: ::core::option::Option<simple_sequence_items::Child0>,
                #[xelement(name = "c", namespace = "http://example.com")]
                pub c: String,
            }
        );

        let expected = prettyplease::unparse(&expected);

        assert_eq!(actual, expected);

        assert_eq!(
            type_.ty.into_type(None),
            parse_quote!(::std::boxed::Box<SimpleSequence>)
        );
    }
}
