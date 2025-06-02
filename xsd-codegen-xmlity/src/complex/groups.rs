use crate::{
    misc::TypeReference,
    templates::{
        self,
        choice::{self, ChoiceVariantType},
        element_record::ElementRecord,
        value_record::{ItemField, ItemFieldItem, ItemRecord},
        FieldType, ItemOrder,
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
    type TypeTemplate = templates::value_record::ItemRecord;

    fn to_type_template<C: Context, S: Scope>(
        &self,
        context: &C,
        scope: &mut S,
    ) -> Result<ToTypeTemplateData<Self::TypeTemplate>> {
        // Struct with strict order
        let fields = self
            .fragments
            .iter()
            .enumerate()
            .map(|(i, fragment_id)| {
                let res = context
                    .sub_context(format_ident!("Child{i}"))
                    .resolve_fragment(fragment_id, scope)?;
                let ident = res
                    .ident
                    .map(|a| a.to_field_ident())
                    .unwrap_or_else(|| format_ident!("child_{}", i));
                let item = res.template.to_item(context, scope, &ident, None)?;

                Ok((Some(ident), item))
            })
            .collect::<Result<Vec<_>>>()?;

        let template = templates::value_record::ItemRecord {
            children_order: ItemOrder::None,
            field_type: FieldType::Named,
            fields,
        };

        Ok(ToTypeTemplateData {
            ident: None,
            template,
        })
    }
}

impl ToTypeTemplate for cx::SequenceFragment {
    type TypeTemplate = templates::value_record::ItemRecord;

    fn to_type_template<C: Context, S: Scope>(
        &self,
        context: &C,
        scope: &mut S,
    ) -> Result<ToTypeTemplateData<Self::TypeTemplate>> {
        // Struct with strict order
        let fields = self
            .fragments
            .iter()
            .enumerate()
            .map(|(i, fragment_id)| {
                let res = context
                    .sub_context(format_ident!("Child{i}"))
                    .resolve_fragment(fragment_id, scope)?;
                let ident = res
                    .ident
                    .map(|a| a.to_field_ident())
                    .unwrap_or_else(|| format_ident!("child_{}", i));
                let item = res.template.to_item(context, scope, &ident, None)?;

                Ok((Some(ident), item))
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

impl ToTypeTemplate for cx::ChoiceFragment {
    type TypeTemplate = ItemFieldItem;

    fn to_type_template<C: Context, S: Scope>(
        &self,
        context: &C,
        scope: &mut S,
    ) -> Result<ToTypeTemplateData<Self::TypeTemplate>> {
        let ident = context.suggested_ident();

        let mod_name = format_ident!("{}_items", ident.to_path_ident());

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

                let variant = res
                    .template
                    .to_variant(context, &mut sub_scope, &ident, None)?;

                Ok((ident, variant))
            })
            .collect::<Result<Vec<_>>>()?;

        let template = templates::choice::Choice { variants };

        let _mod_ref = sub_scope
            .finish_mod(&mod_name)
            .map(|a| scope.add_item(a))
            .transpose()?;

        let item = template.to_enum(ident, Some(&parse_quote!(#mod_name)));

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
        _context: &C,
        _scope: &mut S,
    ) -> Result<ToTypeTemplateData<Self::TypeTemplate>> {
        todo!()
    }
}

impl ToTypeTemplate for cx::GroupRefFragment {
    type TypeTemplate = TypeReference<'static>;

    fn to_type_template<C: Context, S: Scope>(
        &self,
        context: &C,
        _scope: &mut S,
    ) -> Result<ToTypeTemplateData<Self::TypeTemplate>> {
        let ty = context.resolve_named_group(&self.ref_)?;

        Ok(ToTypeTemplateData {
            ident: Some(self.ref_.local_name().to_item_ident()),
            template: ty,
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
    fn to_variant<C: Context, S: Scope>(
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

                    let (ty, optional) = super::min_max_occurs_type(min_occurs, max_occurs, ty);

                    Ok(ChoiceVariantType::Item(ItemRecord::new_single_field(
                        None,
                        ItemField::Item(ItemFieldItem {
                            ty,

                            default: optional,
                        }),
                    )))
                }
            }
            GroupTypeContentTemplate::Item(item) => Ok(ChoiceVariantType::Item(
                ItemRecord::new_single_field(None, ItemField::Item(item)),
            )),
        }
    }

    fn to_item<C: Context, S: Scope>(
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
                            .wrap_if(optional, |ty| parse_quote! { Option<#ty> });
                        field_element.optional = optional;
                        field_element.default = optional;

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
            GroupTypeContentTemplate::Item(item) => Ok(ItemField::Item(item)),
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
        match self {
            cx::GroupTypeContentId::Element(fragment_idx) => context
                .resolve_fragment_id(fragment_idx, scope)
                .map(|res| ToTypeTemplateData {
                    ident: res.ident,
                    template: res.template.into(),
                }),
            cx::GroupTypeContentId::Group(fragment_idx) => context
                .resolve_fragment_id(fragment_idx, scope)
                .map(|res| ToTypeTemplateData {
                    ident: res.ident,
                    template: GroupTypeContentTemplate::Item(ItemFieldItem {
                        ty: res.template,
                        default: false,
                    }),
                }),
            cx::GroupTypeContentId::All(fragment_idx) => {
                let mut sub_scope = GeneratorScope::new();
                let record = context.resolve_fragment_id(fragment_idx, &mut sub_scope)?;

                let ident = record
                    .ident
                    .as_ref()
                    .unwrap_or_else(|| context.suggested_ident());

                let mod_name = format_ident!("{}_items", ident.to_path_ident());

                let _mod_ref = sub_scope
                    .finish_mod(&mod_name)
                    .map(|a| scope.add_item(a))
                    .transpose()?;

                let mod_path = parse_quote!(#mod_name);

                let item = record.template.to_struct(ident, Some(&mod_path));

                let item = scope.add_item(item)?;

                let template = GroupTypeContentTemplate::Item(ItemFieldItem {
                    ty: item,
                    default: false,
                });

                Ok(ToTypeTemplateData {
                    ident: None,
                    template,
                })
            }
            cx::GroupTypeContentId::Choice(fragment_idx) => {
                let record = context.resolve_fragment_id(fragment_idx, scope)?;

                let template = GroupTypeContentTemplate::Item(record.template);

                Ok(ToTypeTemplateData {
                    ident: None,
                    template,
                })
            }
            cx::GroupTypeContentId::Sequence(fragment_idx) => {
                let mut sub_scope = GeneratorScope::new();
                let record = context.resolve_fragment_id(fragment_idx, &mut sub_scope)?;

                let ident = record
                    .ident
                    .as_ref()
                    .unwrap_or_else(|| context.suggested_ident());

                let mod_name = format_ident!("{}_items", ident.to_path_ident());

                let _mod_ref = sub_scope
                    .finish_mod(&mod_name)
                    .map(|a| scope.add_item(a))
                    .transpose()?;

                let mod_path = parse_quote!(#mod_name);

                let item = record.template.to_struct(ident, Some(&mod_path));

                let ty = scope.add_item(item)?;

                let template = GroupTypeContentTemplate::Item(ItemFieldItem { ty, default: false });

                Ok(ToTypeTemplateData {
                    ident: None,
                    template,
                })
            }
            cx::GroupTypeContentId::Any(fragment_idx) => {
                let mut sub_scope = GeneratorScope::new();
                let any = context.resolve_fragment_id(fragment_idx, &mut sub_scope)?;

                let ty = any.template;

                Ok(ToTypeTemplateData {
                    ident: any.ident,
                    template: GroupTypeContentTemplate::Item(ItemFieldItem { ty, default: false }),
                })
            }
        }
    }
}

pub enum TypeDefParticleTemplate {
    Record(ItemRecord),
    Item(ItemFieldItem),
}

impl ToTypeTemplate for cx::TypeDefParticleId {
    type TypeTemplate = TypeDefParticleTemplate;

    fn to_type_template<C: Context, S: Scope>(
        &self,
        context: &C,
        scope: &mut S,
    ) -> Result<ToTypeTemplateData<Self::TypeTemplate>> {
        match self {
            cx::TypeDefParticleId::Group(_) => panic!("TODO"),
            cx::TypeDefParticleId::All(fragment_idx) => {
                let all = context.resolve_fragment_id(fragment_idx, scope)?;

                Ok(ToTypeTemplateData {
                    ident: all.ident,
                    template: TypeDefParticleTemplate::Record(all.template),
                })
            }
            cx::TypeDefParticleId::Sequence(fragment_idx) => {
                let sequence = context.resolve_fragment_id(fragment_idx, scope)?;

                Ok(ToTypeTemplateData {
                    ident: sequence.ident,
                    template: TypeDefParticleTemplate::Record(sequence.template),
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

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use syn::{parse_quote, Item};
    use xmlity::{LocalName, XmlNamespace};
    use xsd::schema as xs;
    use xsd::schema_names as xsn;
    use xsd_type_compiler::{CompiledNamespace, XmlnsContext};

    use crate::Generator;

    #[test]
    fn three_choice_sequence_deep_element() {
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

        #[rustfmt::skip]
        let expected_items: Vec<Item> = vec![
            parse_quote!(
                pub mod simple_sequence_items {
                    pub mod child_0_items {
                        #[derive(::core::fmt::Debug, ::xmlity::Serialize, ::xmlity::Deserialize)]
                        #[xvalue(children_order = "strict")]
                        pub struct Variant0 {
                            #[xelement(name = "a", namespace = "http://example.com")]
                            pub a: i32,
                        }
                    }

                    #[derive(::core::fmt::Debug, ::xmlity::Serialize, ::xmlity::Deserialize)]
                    pub enum Child0 {
                        Variant0(child_0_items::Variant0),
                        #[xelement(name = "b", namespace = "http://example.com")]
                        B(String),
                    }
                }
            ),
            parse_quote!(
                #[derive(::core::fmt::Debug, ::xmlity::SerializationGroup, ::xmlity::DeserializationGroup)]
                #[xgroup(children_order = "strict")]
                pub struct SimpleSequence {
                    pub child_0: simple_sequence_items::Child0,
                    #[xelement(name = "c", namespace = "http://example.com")]
                    pub c: String,
                }
            )
        ];

        assert_eq!(expected_items, actual_items);

        assert_eq!(type_.into_type(None), parse_quote!(SimpleSequence));
    }
}
