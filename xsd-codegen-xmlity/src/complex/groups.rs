use crate::{
    misc::TypeReference,
    templates::{
        self,
        choice::{self, ChoiceVariantType},
        element_record::ElementRecord,
        value_record::{ItemField, ItemFieldItem, ItemRecord},
        FieldType, ItemOrder,
    },
    GeneratorScope, Result,
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
                let res = context.resolve_fragment(fragment_id, scope)?;
                let ident = res.ident.unwrap_or_else(|| format_ident!("field_{}", i));
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
                let res = context.resolve_fragment(fragment_id, scope)?;
                let ident = res.ident.unwrap_or_else(|| format_ident!("field_{}", i));
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
    type TypeTemplate = templates::choice::Choice;

    fn to_type_template<C: Context, S: Scope>(
        &self,
        context: &C,
        scope: &mut S,
    ) -> Result<ToTypeTemplateData<Self::TypeTemplate>> {
        // Struct with strict order
        let variants = self
            .fragments
            .iter()
            .enumerate()
            .map(|(i, fragment_id)| {
                let res = context.resolve_fragment(fragment_id, scope)?;
                let ident = res.ident.unwrap_or_else(|| format_ident!("field_{}", i));

                let variant = res.template.to_variant(context, scope, &ident, None)?;

                Ok((ident, variant))
            })
            .collect::<Result<Vec<_>>>()?;

        let template = templates::choice::Choice { variants };

        Ok(ToTypeTemplateData {
            ident: None,
            template,
        })
    }
}

impl ToTypeTemplate for cx::AnyFragment {
    type TypeTemplate = TypeReference<'static>;

    fn to_type_template<C: Context, S: Scope>(
        &self,
        context: &C,
        scope: &mut S,
    ) -> Result<ToTypeTemplateData<Self::TypeTemplate>> {
        todo!()
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
        context: &C,
        scope: &mut S,
        ident: &syn::Ident,
        path: Option<&syn::Path>,
    ) -> Result<choice::ChoiceVariantType> {
        match self {
            // GroupTypeContentTemplate::Record(item_record) => {
            //     Ok(ChoiceVariantType::Item(item_record))
            // }
            GroupTypeContentTemplate::ElementRecord {
                record,
                min_occurs,
                max_occurs,
            } => {
                if min_occurs == 1 && max_occurs == MaxOccursValue::Bounded(1) {
                    Ok(ChoiceVariantType::Element(record))
                } else {
                    let ty = scope.add_item(record.to_struct(ident, path))?;

                    let ty = super::min_max_occurs_type(min_occurs, max_occurs, ty);

                    Ok(ChoiceVariantType::Item(ItemRecord::new_single_field(
                        None,
                        ItemField::Item(ItemFieldItem { ty }),
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
            // GroupTypeContentTemplate::Record(record) => scope
            //     .add_item(record.to_struct(ident, path))
            //     .map(|ty| ItemField::Item(ItemFieldItem { ty })),
            GroupTypeContentTemplate::ElementRecord {
                record,
                min_occurs,
                max_occurs,
            } => {
                let optional = min_occurs == 0;
                let res = record.try_into_compact_item_field(optional, optional);

                let record = match res {
                    Ok(mut field_element) => {
                        field_element.ty = field_element
                            .ty
                            .wrap_if(optional, |ty| parse_quote! { Option<#ty> });

                        return Ok(ItemField::Element(field_element));
                    }
                    Err(item) => item,
                };

                let ty = scope.add_item(record.to_struct(ident, path))?;

                let ty = super::min_max_occurs_type(min_occurs, max_occurs, ty);

                Ok(ItemField::Item(ItemFieldItem { ty }))
            }
            GroupTypeContentTemplate::Item(item) => return Ok(ItemField::Item(item)),
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
            cx::GroupTypeContentId::Group(fragment_idx) => todo!(),
            cx::GroupTypeContentId::All(fragment_idx) => todo!(),
            cx::GroupTypeContentId::Choice(fragment_idx) => {
                let mut sub_scope = GeneratorScope::new();
                let record = context.resolve_fragment_id(fragment_idx, &mut sub_scope)?;

                let ident = record
                    .ident
                    .unwrap_or_else(|| format_ident!("SimpleSequenceChild"));

                let mod_name = format_ident!("{}_items", ident);

                let mod_ref = sub_scope
                    .finish_mod(&mod_name)
                    .map(|a| scope.add_item(a))
                    .transpose()?;

                let mod_path = parse_quote!(#mod_name);

                let item = record.template.to_enum(&ident, Some(&mod_path));

                let item = scope.add_item(item)?;

                let template = GroupTypeContentTemplate::Item(ItemFieldItem { ty: item });

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
                    .unwrap_or_else(|| format_ident!("SimpleSequenceChild"));

                let mod_name = format_ident!("{}_items", ident);

                let mod_ref = sub_scope
                    .finish_mod(&mod_name)
                    .map(|a| scope.add_item(a))
                    .transpose()?;

                let mod_path = parse_quote!(#mod_name);

                let item = record.template.to_struct(&ident, Some(&mod_path));

                let item = scope.add_item(item)?;

                let template = GroupTypeContentTemplate::Item(ItemFieldItem { ty: item });

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
                    template: GroupTypeContentTemplate::Item(ItemFieldItem { ty }),
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
                    ident: None,
                    template: TypeDefParticleTemplate::Record(all.template),
                })
            }
            cx::TypeDefParticleId::Sequence(fragment_idx) => {
                let sequence = context.resolve_fragment_id(fragment_idx, scope)?;

                Ok(ToTypeTemplateData {
                    ident: None,
                    template: TypeDefParticleTemplate::Record(sequence.template),
                })
            }
            cx::TypeDefParticleId::Choice(fragment_idx) => {
                let choice = context.resolve_fragment_id(fragment_idx, scope)?;

                let ident = choice
                    .ident
                    .unwrap_or_else(|| format_ident!("SimpleSequenceChild"));

                let choice = choice.template.to_enum(&ident, None);

                let ty = scope.add_item(choice)?;

                let template = TypeDefParticleTemplate::Item(ItemFieldItem { ty });

                Ok(ToTypeTemplateData {
                    ident: None,
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
    use xmlity::{ExpandedName, LocalName, XmlNamespace};
    use xsd::schema as xs;
    use xsd_type_compiler::{complex::ANY_TYPE_EXPANDED_NAME, CompiledNamespace, XmlnsContext};

    use crate::{Generator, TypeType};

    #[test]
    fn three_choice_sequence_deep_element() {
        let integer_expanded_name = ExpandedName::new(
            LocalName::new_dangerous("integer"),
            XmlNamespace::XMLNS.into(),
        );
        let string_expanded_name = ExpandedName::new(
            LocalName::new_dangerous("string"),
            XmlNamespace::XMLNS.into(),
        );

        let sequence = xs::TopLevelComplexType::builder()
            .name(LocalName::new_dangerous("SimpleSequence"))
            .content(
                xs::ComplexContent::builder()
                    .content(
                        xs::ComplexRestrictionType::builder()
                            .base(xs::Base(xs::QName(ANY_TYPE_EXPANDED_NAME.clone())))
                            .particle(
                                xs::SequenceType::builder()
                                    .content(vec![
                                        xs::ChoiceType::builder()
                                            .content(vec![
                                                xs::SequenceType::builder()
                                                    .content(vec![xs::LocalElement::builder()
                                                        .name(xs::Name(LocalName::new_dangerous(
                                                            "a",
                                                        )))
                                                        .type_(xs::Type(xs::QName(
                                                            integer_expanded_name.clone(),
                                                        )))
                                                        .build()
                                                        .into()])
                                                    .build()
                                                    .into(),
                                                xs::LocalElement::builder()
                                                    .name(xs::Name(LocalName::new_dangerous("b")))
                                                    .type_(xs::Type(xs::QName(
                                                        string_expanded_name.clone(),
                                                    )))
                                                    .build()
                                                    .into(),
                                            ])
                                            .build()
                                            .into(),
                                        xs::LocalElement::builder()
                                            .name(xs::Name(LocalName::new_dangerous("c")))
                                            .type_(xs::Type(xs::QName(
                                                string_expanded_name.clone(),
                                            )))
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

        generator.bind_type(string_expanded_name, parse_quote!(String), TypeType::Simple);
        generator.bind_type(integer_expanded_name, parse_quote!(i32), TypeType::Simple);

        let (type_, actual_items) = generator.generate_top_level_type(&sequence).unwrap();

        // let print_code = actual_items
        //     .iter()
        //     .map(|item| item.to_token_stream())
        //     .collect::<proc_macro2::TokenStream>();
        // println!(
        //     "{}",
        //     prettyplease::unparse(&syn::parse2(print_code).unwrap())
        // );

        #[rustfmt::skip]
        let expected_items: Vec<Item> = vec![
            parse_quote!(
                pub mod SimpleSequence_items {
                    pub mod SimpleSequenceChild_items {
                        #[derive(::core::fmt::Debug, ::xmlity::Serialize, ::xmlity::Deserialize)]
                        #[xvalue(children_order = "strict")]
                        pub struct SimpleSequenceChild {
                            #[xelement(name = "a", namespace = "http://example.com")]
                            pub a: i32,
                        }
                    }

                    #[derive(::core::fmt::Debug, ::xmlity::Serialize, ::xmlity::Deserialize)]
                    pub enum SimpleSequenceChild {
                        field_0(SimpleSequenceChild_items::SimpleSequenceChild),
                        #[xelement(name = "b", namespace = "http://example.com")]
                        b(String),
                    }
                }
            ),
            parse_quote!(
                #[derive(::core::fmt::Debug, ::xmlity::SerializationGroup, ::xmlity::DeserializationGroup)]
                #[xgroup(children_order = "strict")]
                pub struct SimpleSequence {
                    pub field_0: SimpleSequence_items::SimpleSequenceChild,
                    #[xelement(name = "c", namespace = "http://example.com")]
                    pub c: String,
                }
            )
        ];

        assert_eq!(expected_items, actual_items);

        assert_eq!(type_.into_type(None), parse_quote!(SimpleSequence));
    }
}
