use std::sync::Arc;

use crate::{
    complex::{complex_type::ComplexTypeRootHandler, ComplexToTypeTemplateExt},
    misc::{unbox_type, TypeReference},
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

use syn::parse_quote;
use xsd_fragments::fragments::complex as cx;

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
}

impl ComplexToTypeTemplate<cx::DeclaredElementFragment> for DeclaredElementHandler {
    type TypeTemplate = ElementRecord;

    fn to_type_template<C: ComplexContext, S: Scope>(
        &self,
        context: &C,
        scope: &mut S,
        item: &cx::DeclaredElementFragment,
    ) -> Result<ToTypeTemplateData<Self::TypeTemplate>> {
        let name = context.to_expanded_name(item.name.clone());
        let ident = item.name.to_item_ident();

        match &item.type_ {
            xsd_fragments::NamedOrAnonymous::Named(expanded_name) => {
                let bound_type = context.resolve_named_type(expanded_name)?;

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

                let template = sub_type.template.into_element_record(name);

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

impl ComplexToTypeTemplate<cx::ReferenceElementFragment> for ReferenceElementHandler {
    type TypeTemplate = ItemFieldItem;

    fn to_type_template<C: ComplexContext, S: Scope>(
        &self,
        context: &C,
        _scope: &mut S,
        item: &cx::ReferenceElementFragment,
    ) -> Result<ToTypeTemplateData<Self::TypeTemplate>> {
        let ty = context.resolve_named_element(&item.ref_)?;

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

impl ComplexToTypeTemplate<cx::LocalElementFragment> for LocalElementHandler {
    type TypeTemplate = LocalElementFragmentTemplate;

    fn to_type_template<C: ComplexContext, S: Scope>(
        &self,
        context: &C,
        scope: &mut S,
        item: &cx::LocalElementFragment,
    ) -> Result<ToTypeTemplateData<Self::TypeTemplate>> {
        let min_occurs = item.min_occurs.unwrap_or(1);
        let max_occurs = item.max_occurs.unwrap_or(cx::AllNNI::Bounded(1));

        match &item.type_ {
            cx::LocalElementFragmentType::Local(local) => {
                let local = self
                    .declared_element_handler
                    .to_type_template(context, scope, local)?;

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
                let reference = self
                    .reference_element_handler
                    .to_type_template(context, scope, reference)?;

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

#[derive(Debug)]
pub struct TopLevelElementHandler {
    pub dynamic_substitute_group: bool,
    pub standalone_element_type: bool,
    pub element_type_content_handler: Arc<ElementTypeContentIdHandler>,
    pub dynamic_variant_ident: syn::Ident,
}

impl ComplexToTypeTemplate<cx::TopLevelElementFragment> for TopLevelElementHandler {
    type TypeTemplate = TopLevelElementTemplate;

    fn to_type_template<C: ComplexContext, S: Scope>(
        &self,
        context: &C,
        scope: &mut S,
        item: &cx::TopLevelElementFragment,
    ) -> Result<ToTypeTemplateData<Self::TypeTemplate>> {
        let name = context.to_expanded_name(item.name.clone());
        let ident = item.name.to_item_ident();

        let type_ = item.type_.as_ref();
        let self_type = context.resolve_named_element(&name)?;

        let mut substitution_choices = Vec::new();

        substitution_choices.extend(
            context
                .substitution_group_members(&name)?
                .map(|a| {
                    let element_type = context.resolve_named_element(&a)?;
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
            let substitute_group_ty = self_type.wrap(|ty| {
                let ty = match ty {
                    syn::Type::Path(ty) => unbox_type(&ty).unwrap_or(syn::Type::Path(ty)),
                    _ => ty,
                };

                parse_quote!(::xmlity_ns::SubstitutionGroup<#ty>)
            });

            substitution_choices.push((
                self.dynamic_variant_ident.clone(),
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

        let element_record = (!item.abstract_)
            .then(|| match type_ {
                Some(xsd_fragments::NamedOrAnonymous::Named(expanded_name)) => {
                    let bound_type = context.resolve_named_type(expanded_name)?;

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

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use syn::parse_quote;
    use xmlity::{ExpandedName, LocalName, XmlNamespace};
    use xsd::ns;
    use xsd::xs;
    use xsd::xsn;
    use xsd_fragments::XmlnsContext;

    use crate::misc::TypeReference;
    use crate::BoundType;
    use crate::{Generator, TypeType};

    #[test]
    fn empty_sequence_element() {
        const TEST_NAMESPACE: XmlNamespace<'static> =
            XmlNamespace::new_dangerous("http://example.com");

        let sequence = xs::types::TopLevelElement::builder()
            .name(LocalName::new_dangerous("SimpleSequence"))
            .type_(
                xs::types::LocalComplexType::builder()
                    .complex_type_model(Box::new(
                        xs::ComplexContent::from(xs::complex_content_items::ComplexContent::builder()
                            .child_1(
                                xs::types::ComplexRestrictionType::builder()
                                    .base(xs::types::QName(xsn::ANY_TYPE.clone()))
                                    .child_1(
                                        xs::types::complex_restriction_type_items::Child1::builder(
                                        )
                                        .type_def_particle(Box::new(
                                            xs::Sequence::from(
                                                xs::types::ExplicitGroup::builder()
                                                    .nested_particle(vec![])
                                                    .any_attributes(ns::AnyAttributes::default())
                                                    .build()
                                            )
                                            .into(),
                                        ))
                                        .build()
                                        .into(),
                                    )
                                    .attr_decls(xs::groups::AttrDecls::builder().build().into())
                                    .assertions(xs::groups::Assertions::builder().build().into())
                                    .any_attributes(ns::AnyAttributes::default())
                                    .build()
                                    .into(),
                            )
                            .build())
                            .into(),
                    ))
                    .any_attributes(ns::AnyAttributes::default())
                    .build()
                    .into(),
            )
            .any_attributes(ns::AnyAttributes::default())
            .build()
            .into();

        let mut ctx = XmlnsContext::new();
        let ns = ctx.init_namespace(TEST_NAMESPACE);

        let sequence = ns.import_top_level_element(&sequence).unwrap().into_owned();

        let mut generator = Generator::new(&ctx);

        generator.bind_namespace(TEST_NAMESPACE, parse_quote!(test_ns));

        let (type_, actual_items) = generator.generate_element(&sequence).unwrap();

        let actual = prettyplease::unparse(&syn::File {
            shebang: None,
            attrs: Vec::new(),
            items: actual_items,
        });

        #[rustfmt::skip]
        let expected: syn::File = parse_quote!(
            pub mod simple_sequence_items {
                #[derive(
                    ::core::fmt::Debug,
                    ::xmlity::SerializationGroup,
                    ::xmlity::DeserializationGroup
                )]
                #[xgroup(children_order = "strict")]
                pub struct SimpleSequence;
            }

            #[derive(::core::fmt::Debug, ::xmlity::Serialize, ::xmlity::Deserialize)]
            pub enum SimpleSequence {
                #[xelement(name = "SimpleSequence", namespace = "http://example.com", allow_unknown_attributes = "any")]
                SimpleSequence(#[xgroup] simple_sequence_items::SimpleSequence),
                Dynamic(
                    ::xmlity_ns::SubstitutionGroup<test_ns::SimpleSequence>,
                ),
            }
        );

        let expected = prettyplease::unparse(&expected);

        assert_eq!(actual, expected);

        assert_eq!(type_.into_type(None), parse_quote!(SimpleSequence));
    }

    #[test]
    fn two_child_sequence_element() {
        const TEST_NAMESPACE: XmlNamespace<'static> =
            XmlNamespace::new_dangerous("http://example.com");

        let sequence = xs::types::TopLevelElement::builder()
            .name(LocalName::new_dangerous("SimpleSequence"))
            .type_(
                xs::types::LocalComplexType::builder()
                    .complex_type_model(Box::new(
                        xs::ComplexContent::from(xs::complex_content_items::ComplexContent::builder()
                            .child_1(
                                xs::types::ComplexRestrictionType::builder()
                                    .base(xs::types::QName(xsn::ANY_TYPE.clone()))
                                    .child_1(
                                        xs::types::complex_restriction_type_items::Child1::builder(
                                        )
                                        .type_def_particle(Box::new(
                                            xs::Sequence::from(
                                                xs::types::ExplicitGroup::builder()
                                                    .nested_particle(vec![
                                                        xs::types::LocalElement::builder()
                                                            .name(LocalName::new_dangerous("a"))
                                                            .type_attribute(xs::types::QName(
                                                                xsn::INTEGER.clone(),
                                                            ))
                                                            .any_attributes(ns::AnyAttributes::default())
                                                            .build()
                                                            .into(),
                                                        xs::types::LocalElement::builder()
                                                            .name(LocalName::new_dangerous("b"))
                                                            .type_attribute(xs::types::QName(
                                                                xsn::STRING.clone(),
                                                            ))
                                                            .any_attributes(ns::AnyAttributes::default())
                                                            .build()
                                                            .into(),
                                                    ])
                                                    .any_attributes(ns::AnyAttributes::default())
                                                    .build()
                                            )
                                            .into(),
                                        ))
                                        .build()
                                        .into(),
                                    )
                                    .attr_decls(xs::groups::AttrDecls::builder().build().into())
                                    .assertions(xs::groups::Assertions::builder().build().into())
                                    .any_attributes(ns::AnyAttributes::default())
                                    .build()
                                    .into(),
                            )
                            .build())
                            .into(),
                    ))
                    .any_attributes(ns::AnyAttributes::default())
                    .build()
                    .into(),
            )
            .any_attributes(ns::AnyAttributes::default())
            .build()
            .into();

        let mut ctx = XmlnsContext::new();
        let ns = ctx.init_namespace(TEST_NAMESPACE);

        let sequence = ns.import_top_level_element(&sequence).unwrap().into_owned();

        let mut generator = Generator::new(&ctx);

        generator.bind_namespace(TEST_NAMESPACE, parse_quote!(test_ns));

        generator.bind_types(crate::binds::StdXsdTypes);

        let (type_, actual_items) = generator.generate_element(&sequence).unwrap();

        let actual = prettyplease::unparse(&syn::File {
            shebang: None,
            attrs: Vec::new(),
            items: actual_items.clone(),
        });

        #[rustfmt::skip]
        let expected: syn::File = parse_quote!(
            pub mod simple_sequence_items {
                #[derive(
                    ::core::fmt::Debug,
                    ::xmlity::SerializationGroup,
                    ::xmlity::DeserializationGroup
                )]
                #[xgroup(children_order = "strict")]
                pub struct SimpleSequence {
                    #[xelement(name = "a", namespace = "http://example.com")]
                    pub a: i32,
                    #[xelement(name = "b", namespace = "http://example.com")]
                    pub b: String,
                }
            }

            #[derive(::core::fmt::Debug, ::xmlity::Serialize, ::xmlity::Deserialize)]
            pub enum SimpleSequence {
                #[xelement(
                    name = "SimpleSequence",
                    namespace = "http://example.com",
                    allow_unknown_attributes = "any"
                )]
                SimpleSequence(#[xgroup] simple_sequence_items::SimpleSequence),
                Dynamic(
                    ::xmlity_ns::SubstitutionGroup<test_ns::SimpleSequence>,
                ),
            }
        );

        let expected = prettyplease::unparse(&expected);

        assert_eq!(actual, expected);

        assert_eq!(type_.into_type(None), parse_quote!(SimpleSequence));
    }

    #[test]
    fn two_attribute_sequence_element() {
        const TEST_NAMESPACE: XmlNamespace<'static> =
            XmlNamespace::new_dangerous("http://example.com");

        let sequence = xs::Element::from(
            xs::types::TopLevelElement::builder()
                .name(LocalName::new_dangerous("SimpleSequence"))
                .type_(
                    xs::types::LocalComplexType::builder()
                        .complex_type_model(Box::new(
                            xs::ComplexContent::from(xs::complex_content_items::ComplexContent::builder()
                                .child_1(
                                    xs::types::ComplexRestrictionType::builder()
                                        .base(xs::types::QName(xsn::ANY_TYPE.clone()))
                                        .child_1(
                                            xs::types::complex_restriction_type_items::Child1::builder()
                                            .type_def_particle(Box::new(xs::Sequence::from(xs::types::ExplicitGroup::builder()
                                                .nested_particle(vec![])
                                                .any_attributes(ns::AnyAttributes::default())
                                                .build()).into()))
                                                .build()
                                                .into()
                                        )
                                        .attr_decls(
                                            xs::groups::AttrDecls::builder()
                                                .attribute(vec![
                                                    xs::types::Attribute::builder()
                                                        .name(LocalName::new_dangerous("a"))
                                                        .type_(xs::types::QName(
                                                            xsn::INTEGER.clone(),
                                                        ))
                                                        .use_(xs::types::attribute_items::UseValue::Required)
                                                        .build()
                                                        .into(),
                                                    xs::types::Attribute::builder()
                                                        .name(LocalName::new_dangerous("b"))
                                                        .type_(xs::types::QName(
                                                            xsn::STRING.clone(),
                                                        ))
                                                        .use_(xs::types::attribute_items::UseValue::Required)
                                                        .build()
                                                        .into(),
                                                ])
                                                .build().into(),
                                        )
                                        .assertions(xs::groups::Assertions::builder().build().into())
                                        .any_attributes(ns::AnyAttributes::default())
                                        .build()
                                        .into(),
                                )
                                .build())
                                .into(),
                        ))
                        .any_attributes(ns::AnyAttributes::default())
                        .build()
                        .into(),
                )
                .any_attributes(ns::AnyAttributes::default())
                .build(),
        );

        let mut ctx = XmlnsContext::new();
        let ns = ctx.init_namespace(TEST_NAMESPACE);

        let sequence = ns.import_top_level_element(&sequence).unwrap().into_owned();

        let mut generator = Generator::new(&ctx);

        generator.bind_namespace(TEST_NAMESPACE, parse_quote!(test_ns));

        generator.bind_types(crate::binds::StdXsdTypes);

        let (type_, actual_items) = generator.generate_element(&sequence).unwrap();

        let actual = prettyplease::unparse(&syn::File {
            shebang: None,
            attrs: Vec::new(),
            items: actual_items.clone(),
        });

        #[rustfmt::skip]
        let expected: syn::File = parse_quote!(
            pub mod simple_sequence_items {
                #[derive(
                    ::core::fmt::Debug,
                    ::xmlity::SerializationGroup,
                    ::xmlity::DeserializationGroup
                )]
                #[xgroup(children_order = "strict")]
                pub struct SimpleSequence {
                    #[xattribute(name = "a")]
                    pub a: i32,
                    #[xattribute(name = "b")]
                    pub b: String,
                }
            }

            #[derive(::core::fmt::Debug, ::xmlity::Serialize, ::xmlity::Deserialize)]
            pub enum SimpleSequence {
                #[xelement(
                    name = "SimpleSequence",
                    namespace = "http://example.com",
                    allow_unknown_attributes = "any"
                )]
                SimpleSequence(#[xgroup] simple_sequence_items::SimpleSequence),
                Dynamic(
                    ::xmlity_ns::SubstitutionGroup<test_ns::SimpleSequence>,
                ),
            }
        );

        let expected = prettyplease::unparse(&expected);

        assert_eq!(actual, expected);

        assert_eq!(type_.into_type(None), parse_quote!(SimpleSequence));
    }

    #[test]
    fn two_sequence_deep_element() {
        const TEST_NAMESPACE: XmlNamespace<'static> =
            XmlNamespace::new_dangerous("http://example.com");

        let sequence = xs::types::TopLevelElement::builder()
            .name(LocalName::new_dangerous("SimpleSequence"))
            .type_(
                xs::types::LocalComplexType::builder()
                    .complex_type_model(Box::new(
                        xs::ComplexContent::from(xs::complex_content_items::ComplexContent::builder()
                            .child_1(
                                xs::types::ComplexRestrictionType::builder()
                                    .base(xs::types::QName(xsn::ANY_TYPE.clone()))
                                    .child_1(
                                        xs::types::complex_restriction_type_items::Child1::builder(
                                        )
                                        .type_def_particle(Box::new(
                                            xs::Sequence::from(
                                                xs::types::ExplicitGroup::builder()
                                                    .nested_particle(vec![
                                                        xs::Sequence::from(
                                                            xs::types::ExplicitGroup::builder()
                                                                .nested_particle(vec![
                                                                    xs::types::LocalElement::builder()
                                                                        .name(LocalName::new_dangerous("a"))
                                                                        .type_attribute(xs::types::QName(
                                                                            xsn::INTEGER.clone(),
                                                                        ))
                                                                        .any_attributes(ns::AnyAttributes::default())
                                                                        .build()
                                                                        .into(),
                                                                    xs::types::LocalElement::builder()
                                                                        .name(LocalName::new_dangerous("b"))
                                                                        .type_attribute(xs::types::QName(
                                                                            xsn::STRING.clone(),
                                                                        ))
                                                                        .any_attributes(ns::AnyAttributes::default())
                                                                        .build()
                                                                        .into(),
                                                                ])
                                                                .any_attributes(ns::AnyAttributes::default())
                                                            .build(),
                                                        )
                                                        .into(),
                                                        xs::types::LocalElement::builder()
                                                            .name(LocalName::new_dangerous("c"))
                                                            .type_attribute(xs::types::QName(
                                                                xsn::STRING.clone(),
                                                            ))
                                                            .any_attributes(ns::AnyAttributes::default())
                                                            .build()
                                                            .into(),
                                                    ])
                                                    .any_attributes(ns::AnyAttributes::default())
                                                    .build(),
                                            )
                                            .into(),
                                        ))
                                        .build()
                                        .into(),
                                    )
                                    .attr_decls(xs::groups::AttrDecls::builder().build().into())
                                    .assertions(xs::groups::Assertions::builder().build().into())
                                    .any_attributes(ns::AnyAttributes::default())
                                    .build()
                                    .into(),
                            )
                            .build())
                            .into(),
                    ))
                    .any_attributes(ns::AnyAttributes::default())
                    .build()
                    .into(),
            )
            .any_attributes(ns::AnyAttributes::default())
            .build()
            .into();

        let mut ctx = XmlnsContext::new();
        let ns = ctx.init_namespace(TEST_NAMESPACE);

        let sequence = ns.import_top_level_element(&sequence).unwrap().into_owned();

        let mut generator = Generator::new(&ctx);

        generator.bind_namespace(TEST_NAMESPACE, parse_quote!(test_ns));

        generator.bind_types(crate::binds::StdXsdTypes);

        let (type_, actual_items) = generator.generate_element(&sequence).unwrap();

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
                    #[xelement(name = "a", namespace = "http://example.com")]
                    pub a: i32,
                    #[xelement(name = "b", namespace = "http://example.com")]
                    pub b: String,
                }

                #[derive(
                    ::core::fmt::Debug,
                    ::xmlity::SerializationGroup,
                    ::xmlity::DeserializationGroup
                )]
                #[xgroup(children_order = "strict")]
                pub struct SimpleSequence {
                    pub child_0: Child0,
                    #[xelement(name = "c", namespace = "http://example.com")]
                    pub c: String,
                }
            }
            
            #[derive(::core::fmt::Debug, ::xmlity::Serialize, ::xmlity::Deserialize)]
            pub enum SimpleSequence {
                #[xelement(
                    name = "SimpleSequence",
                    namespace = "http://example.com",
                    allow_unknown_attributes = "any"
                )]
                SimpleSequence(#[xgroup] simple_sequence_items::SimpleSequence),
                Dynamic(
                    ::xmlity_ns::SubstitutionGroup<test_ns::SimpleSequence>,
                ),
            }
        );

        let expected = prettyplease::unparse(&expected);

        assert_eq!(actual, expected);

        assert_eq!(type_.into_type(None), parse_quote!(SimpleSequence));
    }

    #[test]
    fn two_attribute_two_children_sequence_element() {
        const TEST_NAMESPACE: XmlNamespace<'static> =
            XmlNamespace::new_dangerous("http://example.com");

        let sequence = xs::types::TopLevelElement::builder()
            .name(LocalName::new_dangerous("SimpleSequence"))
            .type_(
                xs::types::LocalComplexType::builder()
                    .complex_type_model(Box::new(
                        xs::ComplexContent::from(xs::complex_content_items::ComplexContent::builder()
                            .child_1(
                                xs::types::ComplexRestrictionType::builder()
                                    .base(xs::types::QName(xsn::ANY_TYPE.clone()))
                                    .child_1(
                                        xs::types::complex_restriction_type_items::Child1::builder(
                                        )
                                        .type_def_particle(Box::new(
                                            xs::Sequence::from(
                                                xs::types::ExplicitGroup::builder()
                                                    .nested_particle(vec![
                                                        xs::types::LocalElement::builder()
                                                            .name(LocalName::new_dangerous("a"))
                                                            .type_attribute(xs::types::QName(
                                                                xsn::INTEGER.clone(),
                                                            ))
                                                            .any_attributes(ns::AnyAttributes::default())
                                                            .build()
                                                            .into(),
                                                        xs::types::LocalElement::builder()
                                                            .name(LocalName::new_dangerous("b"))
                                                            .type_attribute(xs::types::QName(
                                                                xsn::STRING.clone(),
                                                            ))
                                                            .any_attributes(ns::AnyAttributes::default())
                                                            .build()
                                                            .into(),
                                                    ])
                                                    .any_attributes(ns::AnyAttributes::default())
                                                    .build(),
                                            )
                                            .into(),
                                        ))
                                        .build()
                                        .into(),
                                    )
                                    .attr_decls(
                                        xs::groups::AttrDecls::builder()
                                            .attribute(vec![
                                                xs::types::Attribute::builder()
                                                    .name(LocalName::new_dangerous("c"))
                                                    .type_(xs::types::QName(xsn::INTEGER.clone()))
                                                    .use_(xs::types::attribute_items::UseValue::Required)
                                                    .build()
                                                    .into(),
                                                xs::types::Attribute::builder()
                                                    .name(LocalName::new_dangerous("d"))
                                                    .type_(xs::types::QName(xsn::STRING.clone()))
                                                    .use_(xs::types::attribute_items::UseValue::Required)
                                                    .build()
                                                    .into(),
                                            ])
                                            .build()
                                            .into(),
                                    )
                                    .assertions(xs::groups::Assertions::builder().build().into())
                                    .any_attributes(ns::AnyAttributes::default())
                                    .build()
                                    .into(),
                            )
                            .build())
                            .into(),
                    ))
                    .any_attributes(ns::AnyAttributes::default())
                    .build()
                    .into(),
            )
            .any_attributes(ns::AnyAttributes::default())
            .build()
            .into();

        let mut ctx = XmlnsContext::new();
        let ns = ctx.init_namespace(TEST_NAMESPACE);

        let sequence = ns.import_top_level_element(&sequence).unwrap().into_owned();

        let mut generator = Generator::new(&ctx);

        generator.bind_namespace(TEST_NAMESPACE, parse_quote!(test_ns));

        generator.bind_types(crate::binds::StdXsdTypes);

        let (type_, actual_items) = generator.generate_element(&sequence).unwrap();

        let actual = prettyplease::unparse(&syn::File {
            shebang: None,
            attrs: Vec::new(),
            items: actual_items.clone(),
        });

        #[rustfmt::skip]
        let expected: syn::File = parse_quote!(
            pub mod simple_sequence_items {
                #[derive(
                    ::core::fmt::Debug,
                    ::xmlity::SerializationGroup,
                    ::xmlity::DeserializationGroup
                )]
                #[xgroup(children_order = "strict")]
                pub struct SimpleSequence {
                    #[xattribute(name = "c")]
                    pub c: i32,
                    #[xattribute(name = "d")]
                    pub d: String,
                    #[xelement(name = "a", namespace = "http://example.com")]
                    pub a: i32,
                    #[xelement(name = "b", namespace = "http://example.com")]
                    pub b: String,
                }
            }

            #[derive(::core::fmt::Debug, ::xmlity::Serialize, ::xmlity::Deserialize)]
            pub enum SimpleSequence {
                #[xelement(
                    name = "SimpleSequence",
                    namespace = "http://example.com",
                    allow_unknown_attributes = "any"
                )]
                SimpleSequence(#[xgroup] simple_sequence_items::SimpleSequence),
                Dynamic(
                    ::xmlity_ns::SubstitutionGroup<test_ns::SimpleSequence>,
                ),
            }
        );

        let expected = prettyplease::unparse(&expected);

        assert_eq!(actual, expected);

        assert_eq!(type_.into_type(None), parse_quote!(SimpleSequence));
    }

    #[test]
    fn complex_reference_type_local_element() {
        const TEST_NAMESPACE: XmlNamespace<'static> =
            XmlNamespace::new_dangerous("http://example.com");

        let child_type_expanded_name = ExpandedName::new(
            LocalName::new_dangerous("childType"),
            XmlNamespace::XS.into(),
        );

        let sequence = xs::types::TopLevelElement::builder()
            .name(LocalName::new_dangerous("SimpleSequence"))
            .type_(
                xs::types::LocalComplexType::builder()
                    .complex_type_model(Box::new(
                        xs::ComplexContent::from(xs::complex_content_items::ComplexContent::builder()
                            .child_1(
                                xs::types::ComplexRestrictionType::builder()
                                    .base(xs::types::QName(xsn::ANY_TYPE.clone()))
                                    .child_1(
                                        xs::types::complex_restriction_type_items::Child1::builder(
                                        )
                                        .type_def_particle(Box::new(
                                            xs::Sequence::from(
                                                xs::types::ExplicitGroup::builder()
                                                    .nested_particle(vec![
                                                        xs::types::LocalElement::builder()
                                                            .name(LocalName::new_dangerous("a"))
                                                            .type_attribute(xs::types::QName(
                                                                child_type_expanded_name.clone(),
                                                            ))
                                                            .any_attributes(ns::AnyAttributes::default())
                                                            .build()
                                                            .into(),
                                                    ])
                                                    .any_attributes(ns::AnyAttributes::default())
                                                    .build(),
                                            )
                                            .into(),
                                        ))
                                        .build()
                                        .into(),
                                    )
                                    .attr_decls(xs::groups::AttrDecls::builder().build().into())
                                    .assertions(xs::groups::Assertions::builder().build().into())
                                    .any_attributes(ns::AnyAttributes::default())
                                    .build()
                                    .into(),
                            )
                            .build())
                            .into(),
                    ))
                    .any_attributes(ns::AnyAttributes::default())
                    .build()
                    .into(),
            )
            .any_attributes(ns::AnyAttributes::default())
            .build()
            .into();

        let mut ctx = XmlnsContext::new();
        let ns = ctx.init_namespace(TEST_NAMESPACE);

        let sequence = ns.import_top_level_element(&sequence).unwrap().into_owned();

        let mut generator = Generator::new(&ctx);

        generator.bind_namespace(TEST_NAMESPACE, parse_quote!(test_ns));

        generator.bind_type(
            child_type_expanded_name,
            BoundType {
                ty: TypeReference::new_static(parse_quote!(types::ChildType)),
                ty_type: TypeType::Complex,
                serialize_with: None,
                deserialize_with: None,
            },
        );

        let (type_, actual_items) = generator.generate_element(&sequence).unwrap();

        let actual = prettyplease::unparse(&syn::File {
            shebang: None,
            attrs: Vec::new(),
            items: actual_items,
        });

        #[rustfmt::skip]
        let expected: syn::File = parse_quote!(
            pub mod simple_sequence_items {
                #[derive(
                    ::core::fmt::Debug,
                    ::xmlity::SerializationGroup,
                    ::xmlity::DeserializationGroup
                )]
                #[xgroup(children_order = "strict")]
                pub struct SimpleSequence {
                    #[xelement(name = "a", namespace = "http://example.com", group)]
                    pub a: types::ChildType,
                }
            }

            #[derive(::core::fmt::Debug, ::xmlity::Serialize, ::xmlity::Deserialize)]
            pub enum SimpleSequence {
                #[xelement(
                    name = "SimpleSequence",
                    namespace = "http://example.com",
                    allow_unknown_attributes = "any"
                )]
                SimpleSequence(#[xgroup] simple_sequence_items::SimpleSequence),
                Dynamic(
                    ::xmlity_ns::SubstitutionGroup<test_ns::SimpleSequence>,
                ),
            }
        );

        let expected = prettyplease::unparse(&expected);

        assert_eq!(actual, expected);
        assert_eq!(type_.into_type(None), parse_quote!(SimpleSequence));
    }

    #[test]
    fn simple_reference_type_top_level_element() {
        const TEST_NAMESPACE: XmlNamespace<'static> =
            XmlNamespace::new_dangerous("http://example.com");

        let sequence = xs::types::TopLevelElement::builder()
            .name(LocalName::new_dangerous("SimpleSequence"))
            .type_attribute(xs::types::QName(xsn::STRING.clone()))
            .any_attributes(ns::AnyAttributes::default())
            .build()
            .into();

        let mut ctx = XmlnsContext::new();
        let ns = ctx.init_namespace(TEST_NAMESPACE);

        let sequence = ns.import_top_level_element(&sequence).unwrap().into_owned();

        let mut generator = Generator::new(&ctx);

        generator.bind_namespace(TEST_NAMESPACE, parse_quote!(test_ns));

        generator.bind_types(crate::binds::StdXsdTypes);

        let (type_, actual_items) = generator.generate_element(&sequence).unwrap();

        let actual = prettyplease::unparse(&syn::File {
            shebang: None,
            attrs: Vec::new(),
            items: actual_items.clone(),
        });

        #[rustfmt::skip]
        let expected: syn::File = parse_quote!(
            #[derive(::core::fmt::Debug, ::xmlity::Serialize, ::xmlity::Deserialize)]
            pub enum SimpleSequence {
                #[xelement(name = "SimpleSequence", namespace = "http://example.com", allow_unknown_attributes = "any")]
                SimpleSequence(String),
                Dynamic(
                    ::xmlity_ns::SubstitutionGroup<test_ns::SimpleSequence>,
                ),
            }
        );

        let expected = prettyplease::unparse(&expected);

        assert_eq!(actual, expected);

        assert_eq!(type_.into_type(None), parse_quote!(SimpleSequence));
    }

    #[test]
    fn complex_reference_type_top_level_element() {
        const TEST_NAMESPACE: XmlNamespace<'static> =
            XmlNamespace::new_dangerous("http://example.com");

        let child_type_expanded_name = ExpandedName::new(
            LocalName::new_dangerous("childType"),
            XmlNamespace::XS.into(),
        );

        let sequence = xs::types::TopLevelElement::builder()
            .name(LocalName::new_dangerous("SimpleSequence"))
            .type_attribute(xs::types::QName(child_type_expanded_name.clone()))
            .any_attributes(ns::AnyAttributes::default())
            .build()
            .into();

        let mut ctx = XmlnsContext::new();
        let ns = ctx.init_namespace(TEST_NAMESPACE);

        let sequence = ns.import_top_level_element(&sequence).unwrap().into_owned();

        let mut generator = Generator::new(&ctx);

        generator.bind_namespace(TEST_NAMESPACE, parse_quote!(test_ns));

        generator.bind_type(
            child_type_expanded_name,
            BoundType {
                ty: TypeReference::new_static(parse_quote!(types::ChildType)),
                ty_type: TypeType::Complex,
                serialize_with: None,
                deserialize_with: None,
            },
        );

        let (type_, actual_items) = generator.generate_element(&sequence).unwrap();

        let actual = prettyplease::unparse(&syn::File {
            shebang: None,
            attrs: Vec::new(),
            items: actual_items.clone(),
        });

        #[rustfmt::skip]
        let expected: syn::File = parse_quote!(
            #[derive(::core::fmt::Debug, ::xmlity::Serialize, ::xmlity::Deserialize)]
            pub enum SimpleSequence {
                #[xelement(name = "SimpleSequence", namespace = "http://example.com", allow_unknown_attributes = "any")]
                SimpleSequence(#[xgroup] types::ChildType),
                Dynamic(
                    ::xmlity_ns::SubstitutionGroup<test_ns::SimpleSequence>,
                ),
            }
        );

        let expected = prettyplease::unparse(&expected);

        assert_eq!(actual, expected);

        assert_eq!(type_.into_type(None), parse_quote!(SimpleSequence));
    }

    #[test]
    fn element_ref_element() {
        const TEST_NAMESPACE: XmlNamespace<'static> =
            XmlNamespace::new_dangerous("http://example.com");

        let child_element_expanded_name = ExpandedName::new(
            LocalName::new_dangerous("ChildElement"),
            XmlNamespace::XS.into(),
        );

        let sequence = xs::types::TopLevelElement::builder()
            .name(LocalName::new_dangerous("SimpleSequence"))
            .type_(
                xs::types::LocalComplexType::builder()
                    .complex_type_model(Box::new(
                        xs::ComplexContent::from(xs::complex_content_items::ComplexContent::builder()
                            .child_1(
                                xs::types::ComplexRestrictionType::builder()
                                    .base(xs::types::QName(xsn::ANY_TYPE.clone()))
                                    .child_1(
                                        xs::types::complex_restriction_type_items::Child1::builder(
                                        )
                                        .type_def_particle(Box::new(
                                            xs::Sequence::from(
                                                xs::types::ExplicitGroup::builder()
                                                    .nested_particle(vec![
                                                        xs::types::LocalElement::builder()
                                                            .ref_(xs::types::QName(
                                                                child_element_expanded_name.clone(),
                                                            ))
                                                            .any_attributes(ns::AnyAttributes::default())
                                                            .build()
                                                            .into(),
                                                    ])
                                                    .any_attributes(ns::AnyAttributes::default())
                                                    .build(),
                                            )
                                            .into(),
                                        ))
                                        .build()
                                        .into(),
                                    )
                                    .attr_decls(xs::groups::AttrDecls::builder().build().into())
                                    .assertions(xs::groups::Assertions::builder().build().into())
                                    .any_attributes(ns::AnyAttributes::default())
                                    .build()
                                    .into(),
                            )
                            .build())
                            .into(),
                    ))
                    .any_attributes(ns::AnyAttributes::default())
                    .build()
                    .into(),
            )
            .any_attributes(ns::AnyAttributes::default())
            .build()
            .into();

        let mut ctx = XmlnsContext::new();
        let ns = ctx.init_namespace(TEST_NAMESPACE);

        let sequence = ns.import_top_level_element(&sequence).unwrap().into_owned();

        let mut generator = Generator::new(&ctx);

        generator.bind_namespace(TEST_NAMESPACE, parse_quote!(test_ns));

        generator.bind_element(
            child_element_expanded_name,
            TypeReference::new_static(parse_quote!(types::ChildElement)),
        );

        let (type_, actual_items) = generator.generate_element(&sequence).unwrap();

        let actual = prettyplease::unparse(&syn::File {
            shebang: None,
            attrs: Vec::new(),
            items: actual_items.clone(),
        });

        #[rustfmt::skip]
        let expected: syn::File = parse_quote!(
            pub mod simple_sequence_items {
                #[derive(
                    ::core::fmt::Debug,
                    ::xmlity::SerializationGroup,
                    ::xmlity::DeserializationGroup
                )]
                #[xgroup(children_order = "strict")]
                pub struct SimpleSequence {
                    pub child_element: types::ChildElement,
                }
            }

            #[derive(::core::fmt::Debug, ::xmlity::Serialize, ::xmlity::Deserialize)]
            pub enum SimpleSequence {
                #[xelement(
                    name = "SimpleSequence",
                    namespace = "http://example.com",
                    allow_unknown_attributes = "any"
                )]
                SimpleSequence(#[xgroup] simple_sequence_items::SimpleSequence),
                Dynamic(
                    ::xmlity_ns::SubstitutionGroup<test_ns::SimpleSequence>,
                ),
            }
        );

        let expected = prettyplease::unparse(&expected);

        assert_eq!(actual, expected);

        assert_eq!(type_.into_type(None), parse_quote!(SimpleSequence));
    }
}
