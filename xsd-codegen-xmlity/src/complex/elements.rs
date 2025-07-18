use crate::{
    misc::{unbox_type, TypeReference},
    simple::SimpleContext,
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
use syn::parse_quote;
use xsd_fragments::fragments::complex::{self as cx, AllNNI};

use super::{ComplexContext, ComplexToTypeTemplate, Scope, ToTypeTemplateData};

impl ComplexToTypeTemplate for cx::ElementTypeContentId {
    type TypeTemplate = GroupRecord;

    fn to_type_template<C: ComplexContext, S: Scope>(
        &self,
        context: &C,
        scope: &mut S,
    ) -> Result<ToTypeTemplateData<Self::TypeTemplate>> {
        match self {
            cx::ElementTypeContentId::SimpleType(_fragment_id) => context
                .simple_context()
                .resolve_fragment_id(_fragment_id, scope)
                .map(|sub_type| ToTypeTemplateData {
                    ident: None,
                    template: GroupRecord::new_single_field(
                        None,
                        ElementField::Item(ItemFieldItem {
                            ty: sub_type.template,
                            default: false,
                        }),
                    ),
                }),
            cx::ElementTypeContentId::ComplexType(fragment_idx) => {
                context.resolve_fragment_id(fragment_idx, scope)
            }
        }
    }
}

fn type_to_element_field(
    ty: TypeReference<'static>,
    ty_type: TypeType,
    default: bool,
) -> ElementField {
    match ty_type {
        TypeType::Simple => ElementField::Item(ItemFieldItem { ty, default }),
        TypeType::Complex => ElementField::Group(ElementFieldGroup { ty }),
    }
}

impl ComplexToTypeTemplate for cx::DeclaredElementFragment {
    type TypeTemplate = ElementRecord;

    fn to_type_template<C: ComplexContext, S: Scope>(
        &self,
        context: &C,
        scope: &mut S,
    ) -> Result<ToTypeTemplateData<Self::TypeTemplate>> {
        let name = context.to_expanded_name(self.name.clone());
        let ident = self.name.to_item_ident();

        match &self.type_ {
            xsd_fragments::NamedOrAnonymous::Named(expanded_name) => {
                let bound_type = context.resolve_named_type(expanded_name)?;

                let field = type_to_element_field(bound_type.ty, bound_type.ty_type, false);

                let template = ElementRecord::new_single_field(name, None, field);

                Ok(ToTypeTemplateData {
                    ident: Some(ident),
                    template,
                })
            }
            xsd_fragments::NamedOrAnonymous::Anonymous(anonymous) => {
                let sub_type = context.resolve_fragment(anonymous, scope)?;

                let template = sub_type.template.into_element_record(name);

                Ok(ToTypeTemplateData {
                    ident: Some(ident),
                    template,
                })
            }
        }
    }
}

impl ComplexToTypeTemplate for cx::ReferenceElementFragment {
    type TypeTemplate = ItemFieldItem;

    fn to_type_template<C: ComplexContext, S: Scope>(
        &self,
        context: &C,
        _scope: &mut S,
    ) -> Result<ToTypeTemplateData<Self::TypeTemplate>> {
        let ty = context.resolve_named_element(&self.name)?;

        let template = ItemFieldItem { ty, default: false };

        Ok(ToTypeTemplateData {
            ident: Some(self.name.local_name().to_item_ident()),
            template,
        })
    }
}

pub enum LocalElementFragmentTemplate {
    ElementRecord {
        template: ElementRecord,
        min_occurs: usize,
        max_occurs: AllNNI,
    },
    Item(ItemFieldItem),
}

impl ComplexToTypeTemplate for cx::LocalElementFragment {
    type TypeTemplate = LocalElementFragmentTemplate;

    fn to_type_template<C: ComplexContext, S: Scope>(
        &self,
        context: &C,
        scope: &mut S,
    ) -> Result<ToTypeTemplateData<Self::TypeTemplate>> {
        let min_occurs = self.min_occurs.unwrap_or(1);
        let max_occurs = self.max_occurs.unwrap_or(AllNNI::Bounded(1));

        match &self.type_ {
            cx::LocalElementFragmentType::Local(local) => {
                let local = context.resolve_fragment(local, scope)?;

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
                let reference = context.resolve_fragment(reference, scope)?;

                let (ty, optional) =
                    super::min_max_occurs_type(min_occurs, max_occurs, reference.template.ty);

                let template = LocalElementFragmentTemplate::Item(ItemFieldItem {
                    ty,
                    default: optional,
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

impl ComplexToTypeTemplate for cx::TopLevelElementFragment {
    type TypeTemplate = TopLevelElementTemplate;

    fn to_type_template<C: ComplexContext, S: Scope>(
        &self,
        context: &C,
        scope: &mut S,
    ) -> Result<ToTypeTemplateData<Self::TypeTemplate>> {
        let name = context.to_expanded_name(self.name.clone());
        let ident = self.name.to_item_ident();

        let type_ = self.type_.as_ref();
        let self_type = context.resolve_named_element(&name)?;

        let substitute_group_setting = true;
        let element_type_standalone = substitute_group_setting;

        let element_record = match type_ {
            Some(xsd_fragments::NamedOrAnonymous::Named(expanded_name)) => {
                let bound_type = context.resolve_named_type(expanded_name)?;

                let field = type_to_element_field(bound_type.ty, bound_type.ty_type, false);

                ElementRecord {
                    name,
                    attribute_order: ItemOrder::None,
                    children_order: ItemOrder::None,
                    fields: ElementFieldType::Unnamed(vec![field]),
                    allow_unknown_attributes: AllowUnknown::Any,
                    allow_unknown_children: AllowUnknown::AtEnd,
                }
            }
            Some(xsd_fragments::NamedOrAnonymous::Anonymous(anonymous)) => {
                let sub_type = context.resolve_fragment(anonymous, scope)?;

                if element_type_standalone {
                    let type_ = sub_type.template.to_struct(&ident, None);

                    let ty = scope.add_item(type_)?;

                    ElementRecord {
                        name,
                        attribute_order: ItemOrder::None,
                        children_order: ItemOrder::None,
                        fields: ElementFieldType::Unnamed(vec![ElementField::Group(
                            ElementFieldGroup { ty },
                        )]),
                        allow_unknown_attributes: AllowUnknown::Any,
                        allow_unknown_children: AllowUnknown::AtEnd,
                    }
                } else {
                    sub_type.template.into_element_record(name)
                }
            }
            None => ElementRecord::new_empty(name),
        };

        if substitute_group_setting {
            let substitute_group_ty = self_type.wrap(|ty| {
                let ty = match ty {
                    syn::Type::Path(ty) => unbox_type(&ty).unwrap_or(syn::Type::Path(ty)),
                    _ => ty,
                };

                parse_quote!(::xmlity_ns::SubstitutionGroup<#ty>)
            });

            let choice = choice::Choice {
                variants: vec![
                    (
                        ident.to_item_ident(),
                        choice::ChoiceVariantType::Element(element_record),
                    ),
                    (
                        format_ident!("SubstitutionGroup"),
                        choice::ChoiceVariantType::Item(
                            value_record::ItemRecord::new_single_field(
                                None,
                                value_record::ItemField::Item(ItemFieldItem {
                                    ty: substitute_group_ty,
                                    default: false,
                                }),
                            ),
                        ),
                    ),
                ],
            };

            Ok(ToTypeTemplateData {
                ident: Some(ident),
                template: TopLevelElementTemplate::Choice(choice),
            })
        } else {
            Ok(ToTypeTemplateData {
                ident: Some(ident),
                template: TopLevelElementTemplate::ElementRecord(element_record),
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use syn::parse_quote;
    use xmlity::{ExpandedName, LocalName, XmlNamespace};
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
                                                    .build()
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
                            .build())
                            .into(),
                    ))
                    .build()
                    .into(),
            )
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
                SubstitutionGroup(
                    ::xmlity_ns::SubstitutionGroup<::std::boxed::Box<test_ns::SimpleSequence>>,
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
                                                    .build()
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
                            .build())
                            .into(),
                    ))
                    .build()
                    .into(),
            )
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
                SubstitutionGroup(
                    ::xmlity_ns::SubstitutionGroup<::std::boxed::Box<test_ns::SimpleSequence>>,
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
                                                        // .use_(xs::types::AttributeUseType::Required)
                                                        .build()
                                                        .into(),
                                                    xs::types::Attribute::builder()
                                                        .name(LocalName::new_dangerous("b"))
                                                        .type_(xs::types::QName(
                                                            xsn::STRING.clone(),
                                                        ))
                                                        .use_(xs::types::attribute_items::UseValue::Required)
                                                        // .use_(xs::types::AttributeUseType::Required)
                                                        .build()
                                                        .into(),
                                                ])
                                                .build().into(),
                                        )
                                        .assertions(xs::groups::Assertions::builder().build().into())
                                        .build()
                                        .into(),
                                )
                                .build())
                                .into(),
                        ))
                        .build()
                        .into(),
                )
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
                SubstitutionGroup(
                    ::xmlity_ns::SubstitutionGroup<::std::boxed::Box<test_ns::SimpleSequence>>,
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
                                                                .build(),
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
                                                    .build(),
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
                            .build())
                            .into(),
                    ))
                    .build()
                    .into(),
            )
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
                SubstitutionGroup(
                    ::xmlity_ns::SubstitutionGroup<::std::boxed::Box<test_ns::SimpleSequence>>,
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
                                                    // .use_(xs::AttributeUseType::Required)
                                                    .use_(xs::types::attribute_items::UseValue::Required)
                                                    .build()
                                                    .into(),
                                                xs::types::Attribute::builder()
                                                    .name(LocalName::new_dangerous("d"))
                                                    .type_(xs::types::QName(xsn::STRING.clone()))
                                                    // .use_(xs::AttributeUseType::Required)
                                                    .use_(xs::types::attribute_items::UseValue::Required)
                                                    .build()
                                                    .into(),
                                            ])
                                            .build()
                                            .into(),
                                    )
                                    .assertions(xs::groups::Assertions::builder().build().into())
                                    .build()
                                    .into(),
                            )
                            .build())
                            .into(),
                    ))
                    .build()
                    .into(),
            )
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
                SubstitutionGroup(
                    ::xmlity_ns::SubstitutionGroup<::std::boxed::Box<test_ns::SimpleSequence>>,
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
                                                            .build()
                                                            .into(),
                                                    ])
                                                    .build(),
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
                            .build())
                            .into(),
                    ))
                    .build()
                    .into(),
            )
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
                SubstitutionGroup(
                    ::xmlity_ns::SubstitutionGroup<::std::boxed::Box<test_ns::SimpleSequence>>,
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
                SubstitutionGroup(
                    ::xmlity_ns::SubstitutionGroup<::std::boxed::Box<test_ns::SimpleSequence>>,
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
                SubstitutionGroup(
                    ::xmlity_ns::SubstitutionGroup<::std::boxed::Box<test_ns::SimpleSequence>>,
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
                                                            .build()
                                                            .into(),
                                                    ])
                                                    .build(),
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
                            .build())
                            .into(),
                    ))
                    .build()
                    .into(),
            )
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
                SubstitutionGroup(
                    ::xmlity_ns::SubstitutionGroup<::std::boxed::Box<test_ns::SimpleSequence>>,
                ),
            }
        );

        let expected = prettyplease::unparse(&expected);

        assert_eq!(actual, expected);

        assert_eq!(type_.into_type(None), parse_quote!(SimpleSequence));
    }
}
