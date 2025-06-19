use crate::{
    complex::dedup_field_idents,
    misc::TypeReference,
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
use syn::parse_quote;
use xsd::schema::MaxOccursValue;
use xsd_type_compiler::complex as cx;

use super::{
    elements::LocalElementFragmentTemplate, Context, Scope, ToTypeTemplate, ToTypeTemplateData,
};

impl ToTypeTemplate for cx::AllFragment {
    type TypeTemplate = ItemOrTemplate<ItemRecord>;

    fn to_type_template<C: Context, S: Scope>(
        &self,
        context: &C,
        scope: &mut S,
    ) -> Result<ToTypeTemplateData<Self::TypeTemplate>> {
        let min_occurs = self.min_occurs.map(|a| a.0).unwrap_or(1);
        let max_occurs = self.max_occurs.unwrap_or_default();

        let mut sub_scope = GeneratorScope::new();

        let mod_name = format_ident!("{}_items", context.suggested_ident().to_path_ident());

        let mod_path: syn::Path = parse_quote!(#mod_name);

        // Struct with strict order
        let fields = self
            .fragments
            .iter()
            .enumerate()
            .map(|(i, fragment_id)| {
                let res = context
                    .sub_context(format_ident!("Child{i}"))
                    .resolve_fragment(fragment_id, &mut sub_scope)?;
                let ident = res
                    .ident
                    .map(|a| a.to_field_ident())
                    .unwrap_or_else(|| format_ident!("child_{}", i));
                let item = res
                    .template
                    .into_item(context, &mut sub_scope, &ident, None)?;

                Ok((ident, item))
            })
            .collect::<Result<Vec<_>>>()?;

        let fields = dedup_field_idents(fields);

        let mut template = templates::value_record::ItemRecord {
            children_order: ItemOrder::None,
            fields: ItemFieldType::Named(fields),
        };

        template.force_empty_if_empty();

        let template = ItemOrTemplate::new(
            template,
            |template| {
                syn::Item::Struct(template.to_struct(context.suggested_ident(), Some(&mod_path)))
            },
            min_occurs,
            max_occurs,
            scope,
        )?;

        if matches!(template, ItemOrTemplate::Record(_)) {
            scope.add_items(sub_scope.finish());
        } else {
            let _mod_ref = sub_scope
                .finish_mod(&mod_name)
                .map(|a| scope.add_item(a))
                .transpose()?;
        }

        Ok(ToTypeTemplateData {
            ident: None,
            template,
        })
    }
}

pub enum ItemOrTemplate<T> {
    Record(T),
    Item(templates::value_record::ItemFieldItem),
}

impl<T> ItemOrTemplate<T> {
    pub fn new<S: Scope>(
        template: T,
        template_to_item: impl FnOnce(T) -> syn::Item,
        min_occurs: usize,
        max_occurs: MaxOccursValue,
        scope: &mut S,
    ) -> Result<Self> {
        match (min_occurs, max_occurs) {
            (1, MaxOccursValue::Bounded(1)) => Ok(Self::Record(template)),
            (min, max) => {
                let ty = scope.add_item(template_to_item(template))?;

                match (min, max) {
                    (0, MaxOccursValue::Bounded(1)) => Ok(Self::Item(ItemFieldItem {
                        ty: ty.wrap(TypeReference::option_wrapper),
                        default: true,
                    })),
                    (1, MaxOccursValue::Bounded(1)) => unreachable!(),
                    _ => Ok(Self::Item(ItemFieldItem {
                        ty: ty.wrap(TypeReference::vec_wrapper),
                        default: true,
                    })),
                }
            }
        }
    }
}

impl ToTypeTemplate for cx::SequenceFragment {
    type TypeTemplate = ItemOrTemplate<ItemRecord>;

    fn to_type_template<C: Context, S: Scope>(
        &self,
        context: &C,
        scope: &mut S,
    ) -> Result<ToTypeTemplateData<Self::TypeTemplate>> {
        let min_occurs = self.min_occurs.map(|a| a.0).unwrap_or(1);
        let max_occurs = self.max_occurs.unwrap_or_default();

        let mut sub_scope = GeneratorScope::new();

        let mod_name = format_ident!("{}_items", context.suggested_ident().to_path_ident());

        let mod_path: syn::Path = parse_quote!(#mod_name);

        // Struct with strict order
        let fields = self
            .fragments
            .iter()
            .enumerate()
            .map(|(i, fragment_id)| {
                let res = context
                    .sub_context(format_ident!("Child{i}"))
                    .resolve_fragment(fragment_id, &mut sub_scope)?;
                let ident = res
                    .ident
                    .map(|a| a.to_field_ident())
                    .unwrap_or_else(|| format_ident!("child_{}", i));
                let item = res
                    .template
                    .into_item(context, &mut sub_scope, &ident, None)?;

                Ok((ident, item))
            })
            .collect::<Result<Vec<_>>>()?;

        let fields = dedup_field_idents(fields);

        let mut template = templates::value_record::ItemRecord {
            children_order: ItemOrder::Strict,
            fields: ItemFieldType::Named(fields),
        };

        template.force_empty_if_empty();

        let template = ItemOrTemplate::new(
            template,
            |template| {
                syn::Item::Struct(template.to_struct(context.suggested_ident(), Some(&mod_path)))
            },
            min_occurs,
            max_occurs,
            scope,
        )?;

        if matches!(template, ItemOrTemplate::Record(_)) {
            scope.add_items(sub_scope.finish());
        } else {
            let _mod_ref = sub_scope
                .finish_mod(&mod_name)
                .map(|a| scope.add_item(a))
                .transpose()?;
        }

        Ok(ToTypeTemplateData {
            ident: None,
            template,
        })
    }
}

impl ToTypeTemplate for cx::ChoiceFragment {
    type TypeTemplate = ItemFieldItem;

    fn to_type_template<C: Context, S: Scope>(
        &self,
        context: &C,
        scope: &mut S,
    ) -> Result<ToTypeTemplateData<Self::TypeTemplate>> {
        let ident = context.suggested_ident();

        let mod_name = format_ident!("{}_variants", ident.to_path_ident());

        let mod_path: syn::Path = parse_quote!(#mod_name);

        let mut sub_scope = GeneratorScope::new();

        // Struct with strict order
        let variants = self
            .fragments
            .iter()
            .enumerate()
            .map(|(i, fragment_id)| {
                let res = context
                    .sub_context(format_ident!("Variant{i}"))
                    .resolve_fragment(fragment_id, &mut sub_scope)?;
                let ident = res
                    .ident
                    .map(|a| a.to_variant_ident())
                    .unwrap_or_else(|| format_ident!("Variant{}", i));

                let variant =
                    res.template
                        .into_variant(context, &mut sub_scope, &ident, Some(&mod_path))?;

                Ok((ident, variant))
            })
            .collect::<Result<Vec<_>>>()?;

        let variants = dedup_field_idents(variants);

        let template = templates::choice::Choice { variants };

        let _mod_ref = sub_scope
            .finish_mod(&mod_name)
            .map(|a| scope.add_item(a))
            .transpose()?;

        let item = template.to_enum(ident, None);

        let ty = scope.add_item(item)?;

        let min_occurs = self.min_occurs.map(|a| a.0).unwrap_or(1);
        let max_occurs = self.max_occurs.unwrap_or(MaxOccursValue::Bounded(1));

        let (ty, optional) = super::min_max_occurs_type(min_occurs, max_occurs, ty);

        Ok(ToTypeTemplateData {
            ident: None,
            template: ItemFieldItem {
                ty,
                default: optional,
            },
        })
    }
}

impl ToTypeTemplate for cx::AnyFragment {
    type TypeTemplate = TypeReference<'static>;

    fn to_type_template<C: Context, S: Scope>(
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

impl ToTypeTemplate for cx::GroupRefFragment {
    type TypeTemplate = ItemFieldItem;

    fn to_type_template<C: Context, S: Scope>(
        &self,
        context: &C,
        _scope: &mut S,
    ) -> Result<ToTypeTemplateData<Self::TypeTemplate>> {
        let min_occurs = self.min_occurs.map(|a| a.0).unwrap_or(1);
        let max_occurs = self.max_occurs.unwrap_or_default();

        //TODO: Should not wrap in box if we also wrap it in Vec.
        let ty = context.resolve_named_group(&self.ref_)?;

        let (ty, optional) = super::min_max_occurs_type(min_occurs, max_occurs, ty);

        let template = ItemFieldItem {
            ty,
            default: optional,
        };

        Ok(ToTypeTemplateData {
            ident: Some(self.ref_.local_name().to_item_ident()),
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
        max_occurs: MaxOccursValue,
    },
    Item(ItemFieldItem),
}

impl GroupTypeContentTemplate {
    fn into_variant<C: Context, S: Scope>(
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
                if min_occurs == 1 && max_occurs == MaxOccursValue::Bounded(1) {
                    Ok(ChoiceVariantType::Element(record))
                } else {
                    let ty = scope.add_item(record.to_struct(ident, path))?;

                    let ty = ty.wrap(TypeReference::box_wrapper);

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
                        .wrap(TypeReference::box_wrapper)
                        .prefix(path.clone());
                }

                Ok(ChoiceVariantType::Item(ItemRecord::new_single_field(
                    None,
                    ItemField::Item(item),
                )))
            }
        }
    }

    fn into_item<C: Context, S: Scope>(
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
                let optional = min_occurs == 0;
                let res = record.try_into_compact_item_field(optional);

                let record = match res {
                    Ok(mut field_element) => {
                        field_element.ty = field_element
                            .ty
                            .wrap_if(optional, TypeReference::option_wrapper);
                        field_element.optional = optional;

                        return Ok(ItemField::Element(field_element));
                    }
                    Err(item) => item,
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

impl ToTypeTemplate for cx::GroupTypeContentId {
    type TypeTemplate = GroupTypeContentTemplate;

    fn to_type_template<C: Context, S: Scope>(
        &self,
        context: &C,
        scope: &mut S,
    ) -> Result<ToTypeTemplateData<Self::TypeTemplate>> {
        let template = match self {
            cx::GroupTypeContentId::Element(fragment_idx) => context
                .resolve_fragment_id(fragment_idx, scope)
                .map(|res| ToTypeTemplateData {
                    ident: res.ident,
                    template: res.template.into(),
                })?,
            cx::GroupTypeContentId::Group(fragment_idx) => context
                .resolve_fragment_id(fragment_idx, scope)
                .map(|res| ToTypeTemplateData {
                    ident: res.ident,
                    template: GroupTypeContentTemplate::Item(res.template),
                })?,
            cx::GroupTypeContentId::All(fragment_idx) => {
                let record = context.resolve_fragment_id(fragment_idx, scope)?;

                let ident = record
                    .ident
                    .as_ref()
                    .unwrap_or_else(|| context.suggested_ident());

                let item = match record.template {
                    ItemOrTemplate::Record(record) => {
                        let item = record.to_struct(ident, None);

                        let ty = scope.add_item(item)?;

                        ItemFieldItem { ty, default: false }
                    }
                    ItemOrTemplate::Item(item) => item,
                };

                let template = GroupTypeContentTemplate::Item(item);

                ToTypeTemplateData {
                    ident: None,
                    template,
                }
            }
            cx::GroupTypeContentId::Choice(fragment_idx) => {
                let record = context.resolve_fragment_id(fragment_idx, scope)?;

                let template = GroupTypeContentTemplate::Item(record.template);

                ToTypeTemplateData {
                    ident: None,
                    template,
                }
            }
            cx::GroupTypeContentId::Sequence(fragment_idx) => {
                let mut sub_scope = GeneratorScope::new();

                let mod_name = format_ident!("{}_items", context.suggested_ident().to_path_ident());

                let mod_path: syn::Path = parse_quote!(#mod_name);

                let record = context.resolve_fragment_id(fragment_idx, &mut sub_scope)?;

                let ident = record
                    .ident
                    .as_ref()
                    .unwrap_or_else(|| context.suggested_ident());

                let item = match record.template {
                    ItemOrTemplate::Record(record) => {
                        let item = record.to_struct(ident, Some(&mod_path));

                        sub_scope
                            .finish_mod(&mod_name)
                            .map(|a| scope.add_item(a))
                            .transpose()?;

                        let ty = scope.add_item(item)?;

                        ItemFieldItem { ty, default: false }
                    }
                    ItemOrTemplate::Item(item) => {
                        scope.add_items(sub_scope.finish());

                        item
                    }
                };

                let template = GroupTypeContentTemplate::Item(item);

                ToTypeTemplateData {
                    ident: None,
                    template,
                }
            }
            cx::GroupTypeContentId::Any(fragment_idx) => {
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
    Item(ItemFieldItem),
}

impl TypeDefParticleTemplate {
    pub fn into_group_record(
        self,
        ident: Option<syn::Ident>,
    ) -> templates::group_record::GroupRecord {
        match self {
            TypeDefParticleTemplate::Record(item_record) => item_record.into_group_record(),
            TypeDefParticleTemplate::Item(item_field_item) => {
                item_field_item.into_group_record(ident)
            }
        }
    }
    pub fn into_item_record(
        self,
        ident: Option<syn::Ident>,
    ) -> templates::value_record::ItemRecord {
        match self {
            TypeDefParticleTemplate::Record(item_record) => item_record.into_item_record(),
            TypeDefParticleTemplate::Item(item_field_item) => {
                item_field_item.into_item_record(ident)
            }
        }
    }
}

impl ToTypeTemplate for cx::TypeDefParticleId {
    type TypeTemplate = TypeDefParticleTemplate;

    fn to_type_template<C: Context, S: Scope>(
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

                Ok(ToTypeTemplateData {
                    ident: all.ident,
                    template: match all.template {
                        ItemOrTemplate::Record(record) => TypeDefParticleTemplate::Record(record),
                        ItemOrTemplate::Item(item) => TypeDefParticleTemplate::Item(item),
                    },
                })
            }
            cx::TypeDefParticleId::Sequence(fragment_idx) => {
                let sequence = context.resolve_fragment_id(fragment_idx, scope)?;

                Ok(ToTypeTemplateData {
                    ident: sequence.ident,
                    template: match sequence.template {
                        ItemOrTemplate::Record(record) => TypeDefParticleTemplate::Record(record),
                        ItemOrTemplate::Item(item) => TypeDefParticleTemplate::Item(item),
                    },
                })
            }
            cx::TypeDefParticleId::Choice(fragment_idx) => {
                let choice = context.resolve_fragment_id(fragment_idx, scope)?;

                let ident = choice
                    .ident
                    .as_ref()
                    .unwrap_or_else(|| context.suggested_ident());

                let template = TypeDefParticleTemplate::Item(choice.template);

                Ok(ToTypeTemplateData {
                    ident: Some(ident.clone()),
                    template,
                })
            }
        }
    }
}

impl ToTypeTemplate for cx::NamedGroupTypeContentId {
    type TypeTemplate = TypeDefParticleTemplate;

    fn to_type_template<C: Context, S: Scope>(
        &self,
        context: &C,
        scope: &mut S,
    ) -> Result<ToTypeTemplateData<Self::TypeTemplate>> {
        match self {
            cx::NamedGroupTypeContentId::All(fragment_idx) => {
                let all = context.resolve_fragment_id(fragment_idx, scope)?;

                Ok(ToTypeTemplateData {
                    ident: all.ident,
                    template: match all.template {
                        ItemOrTemplate::Record(record) => TypeDefParticleTemplate::Record(record),
                        ItemOrTemplate::Item(item) => TypeDefParticleTemplate::Item(item),
                    },
                })
            }
            cx::NamedGroupTypeContentId::Sequence(fragment_idx) => {
                let sequence = context.resolve_fragment_id(fragment_idx, scope)?;

                Ok(ToTypeTemplateData {
                    ident: sequence.ident,
                    template: match sequence.template {
                        ItemOrTemplate::Record(record) => TypeDefParticleTemplate::Record(record),
                        ItemOrTemplate::Item(item) => TypeDefParticleTemplate::Item(item),
                    },
                })
            }
            cx::NamedGroupTypeContentId::Choice(fragment_idx) => {
                let choice = context.resolve_fragment_id(fragment_idx, scope)?;

                let ident = choice
                    .ident
                    .as_ref()
                    .unwrap_or_else(|| context.suggested_ident());

                let template = TypeDefParticleTemplate::Item(choice.template);

                Ok(ToTypeTemplateData {
                    ident: Some(ident.clone()),
                    template,
                })
            }
        }
    }
}

impl ToTypeTemplate for cx::TopLevelGroupFragment {
    type TypeTemplate = ItemRecord;

    fn to_type_template<C: Context, S: Scope>(
        &self,
        context: &C,
        scope: &mut S,
    ) -> Result<ToTypeTemplateData<Self::TypeTemplate>> {
        let name_ident = self.name.as_ref().to_item_ident();

        let fragment = context.resolve_fragment(&self.content, scope)?;

        Ok(ToTypeTemplateData {
            ident: Some(name_ident),
            template: fragment.template.into_item_record(None),
        })
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use syn::parse_quote;
    use xmlity::{LocalName, XmlNamespace};
    use xsd::schema as xs;
    use xsd::schema_names as xsn;
    use xsd_type_compiler::{CompiledNamespace, XmlnsContext};

    use crate::Generator;

    #[test]
    fn three_choice_sequence_deep_top_level_type() {
        let sequence = xs::TopLevelComplexType::builder()
            .name(LocalName::new_dangerous("SimpleSequence"))
            .content(
                xs::ComplexContent::builder()
                    .content(
                        xs::ComplexRestrictionType::builder()
                            .base(xs::QName(xsn::ANY_TYPE.clone()))
                            .particle(
                                xs::SequenceType::builder()
                                    .content(vec![
                                        xs::ChoiceType::builder()
                                            .content(vec![
                                                xs::SequenceType::builder()
                                                    .content(vec![xs::LocalElement::builder()
                                                        .name(LocalName::new_dangerous("a"))
                                                        .type_(xs::QName(xsn::INTEGER.clone()))
                                                        .build()
                                                        .into()])
                                                    .build()
                                                    .into(),
                                                xs::LocalElement::builder()
                                                    .name(LocalName::new_dangerous("b"))
                                                    .type_(xs::QName(xsn::STRING.clone()))
                                                    .build()
                                                    .into(),
                                            ])
                                            .build()
                                            .into(),
                                        xs::LocalElement::builder()
                                            .name(LocalName::new_dangerous("c"))
                                            .type_(xs::QName(xsn::STRING.clone()))
                                            .build()
                                            .into(),
                                    ])
                                    .build()
                                    .into(),
                            )
                            .build()
                            .into(),
                    )
                    .build()
                    .into(),
            )
            .build();

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
                    pub struct Variant0 {
                        #[xelement(name = "a", namespace = "http://example.com")]
                        pub a: i32,
                    }
                }

                #[derive(::core::fmt::Debug, ::xmlity::Serialize, ::xmlity::Deserialize)]
                pub enum Child0 {
                    Variant0(::std::boxed::Box<child_0_variants::Variant0>),
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
        let sequence = xs::TopLevelComplexType::builder()
            .name(LocalName::new_dangerous("SimpleSequence"))
            .content(
                xs::ComplexContent::builder()
                    .content(
                        xs::ComplexRestrictionType::builder()
                            .base(xs::QName(xsn::ANY_TYPE.clone()))
                            .particle(
                                xs::SequenceType::builder()
                                    .content(vec![
                                        xs::SequenceType::builder()
                                            .content(vec![
                                                xs::LocalElement::builder()
                                                    .name(LocalName::new_dangerous("a"))
                                                    .type_(xs::QName(xsn::INTEGER.clone()))
                                                    .min_occurs(xs::MinOccurs(0))
                                                    .build()
                                                    .into(),
                                                xs::LocalElement::builder()
                                                    .name(LocalName::new_dangerous("b"))
                                                    .type_(xs::QName(xsn::STRING.clone()))
                                                    .build()
                                                    .into(),
                                            ])
                                            .min_occurs(xs::MinOccurs(0))
                                            .build()
                                            .into(),
                                        xs::LocalElement::builder()
                                            .name(LocalName::new_dangerous("c"))
                                            .type_(xs::QName(xsn::STRING.clone()))
                                            .build()
                                            .into(),
                                    ])
                                    .build()
                                    .into(),
                            )
                            .build()
                            .into(),
                    )
                    .build()
                    .into(),
            )
            .build();

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
