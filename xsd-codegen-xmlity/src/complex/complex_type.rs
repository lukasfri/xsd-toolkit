use crate::{
    complex::dedup_field_idents,
    templates::{
        self,
        element_record::{ElementField, ElementFieldType},
        group_record::GroupRecord,
        value_record::ItemFieldItem,
    },
    Result, ToIdentTypesExt,
};

use quote::format_ident;
use xsd_type_compiler::fragments::complex::{self as cx};

use super::{groups::TypeDefParticleTemplate, ComplexContext, Scope, ComplexToTypeTemplate, ToTypeTemplateData};

impl ComplexToTypeTemplate for cx::AttributeDeclarationsFragment {
    type TypeTemplate = Vec<(syn::Ident, ElementField)>;

    fn to_type_template<C: ComplexContext, S: Scope>(
        &self,
        context: &C,
        scope: &mut S,
    ) -> Result<ToTypeTemplateData<Self::TypeTemplate>> {
        let attributes = self
            .declarations
            .iter()
            .enumerate()
            .map(|(i, a)| {
                context
                    .sub_context(format_ident!("Attribute{i}"))
                    .resolve_fragment(a, scope)
                    .map(|a| {
                        (
                            a.ident
                                .map(|a| a.to_field_ident())
                                .unwrap_or_else(|| format_ident!("attribute_{i}")),
                            ElementField::Attribute(a.template),
                        )
                    })
            })
            .collect::<Result<Vec<_>>>()?;

        let attributes = dedup_field_idents(attributes);

        Ok(ToTypeTemplateData {
            ident: None,
            template: attributes,
        })
    }
}

fn dedup_attribute_field_idents<T, E>(
    existing_fields: &[(syn::Ident, E)],
    attribute_fields: impl IntoIterator<Item = (syn::Ident, T)>,
) -> Vec<(syn::Ident, T)> {
    attribute_fields
        .into_iter()
        .map(|(mut ident, value)| {
            if existing_fields
                .iter()
                .any(|(existing_ident, _)| existing_ident == &ident)
            {
                const ATTRIBUTE_SUFFIX: &str = "attribute";
                ident = if ident.to_string().ends_with("_") {
                    format_ident!("{ident}{ATTRIBUTE_SUFFIX}")
                } else {
                    format_ident!("{ident}_{ATTRIBUTE_SUFFIX}")
                };
            }

            (ident, value)
        })
        .collect()
}

impl ComplexToTypeTemplate for cx::RestrictionFragment {
    type TypeTemplate = templates::group_record::GroupRecord;

    fn to_type_template<C: ComplexContext, S: Scope>(
        &self,
        context: &C,
        scope: &mut S,
    ) -> Result<ToTypeTemplateData<Self::TypeTemplate>> {
        let mut template = self
            .content_fragment
            .map(|a| {
                context.resolve_fragment(&a, scope).map(|a| {
                    let ident = a.ident.unwrap_or_else(|| format_ident!("Particle"));

                    match a.template {
                        TypeDefParticleTemplate::Record(item_record) => {
                            item_record.into_group_record()
                        }
                        TypeDefParticleTemplate::Choice(item) => {
                            let item = item.to_enum(&ident, None);

                            let ty = scope.add_item(item).unwrap();

                            GroupRecord::new_single_field(
                                Some(ident.to_field_ident()),
                                ElementField::Item(ItemFieldItem { ty, default: false }),
                            )
                        }
                        TypeDefParticleTemplate::Item(item) => GroupRecord::new_single_field(
                            Some(ident.to_field_ident()),
                            ElementField::Item(item),
                        ),
                    }
                })
            })
            .transpose()?
            .unwrap_or_else(GroupRecord::new_empty);

        let attributes = context.resolve_fragment_id(&self.attribute_declarations, scope)?;

        let attribute_fields = dedup_attribute_field_idents(
            match &template.fields {
                ElementFieldType::Named(items) => items,
                ElementFieldType::Empty => &[],
                ElementFieldType::Unnamed(_) => {
                    unreachable!("Should only be named fields or empty")
                }
            },
            attributes.template,
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

impl ComplexToTypeTemplate for cx::ComplexContentFragment {
    type TypeTemplate = templates::group_record::GroupRecord;

    fn to_type_template<C: ComplexContext, S: Scope>(
        &self,
        context: &C,
        scope: &mut S,
    ) -> Result<ToTypeTemplateData<Self::TypeTemplate>> {
        match &self.content_fragment {
            cx::ComplexContentChildId::Extension(_fragment_idx) => {
                panic!(
                    "TODO: Should give error. Extension not supported. Ident: {}",
                    context.suggested_ident()
                )
            }
            cx::ComplexContentChildId::Restriction(fragment_idx) => {
                context.resolve_fragment_id(fragment_idx, scope)
            }
        }
    }
}

impl ComplexToTypeTemplate for cx::ComplexTypeModelId {
    type TypeTemplate = GroupRecord;

    fn to_type_template<C: ComplexContext, S: Scope>(
        &self,
        context: &C,
        scope: &mut S,
    ) -> Result<ToTypeTemplateData<Self::TypeTemplate>> {
        match self {
            cx::ComplexTypeModelId::SimpleContent(_fragment_idx) => todo!(),
            cx::ComplexTypeModelId::ComplexContent(fragment_idx) => {
                context.resolve_fragment_id(fragment_idx, scope)
            }
            cx::ComplexTypeModelId::Other {
                particle,
                attr_decls,
            } => {
                let (ident, mut template) = particle
                    .as_ref()
                    .map(|particle| {
                        context.resolve_fragment(particle, scope).map(|a| {
                            (
                                a.ident,
                                a.template
                                    .into_group_record(Some(format_ident!("particle"))),
                            )
                        })
                    })
                    .unwrap_or_else(|| Ok((None, GroupRecord::new_empty())))?;

                let attributes = context.resolve_fragment_id(attr_decls, scope)?;

                template
                    .fields
                    .prefix_fields(ElementFieldType::Named(attributes.template));

                Ok(ToTypeTemplateData { ident, template })
            }
        }
    }
}

impl ComplexToTypeTemplate for cx::ComplexTypeRootFragment {
    type TypeTemplate = GroupRecord;

    fn to_type_template<C: ComplexContext, S: Scope>(
        &self,
        context: &C,
        scope: &mut S,
    ) -> Result<ToTypeTemplateData<Self::TypeTemplate>> {
        let mut fragment = context.resolve_fragment(&self.content, scope)?;

        let name_ident = self
            .name
            .as_ref()
            .map(|a| a.to_item_ident())
            .unwrap_or_else(|| context.suggested_ident().clone());
        fragment.ident = Some(name_ident);

        Ok(fragment)
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
    fn empty_sequence_complex_type() {
        let sequence = xs::types::TopLevelComplexType::builder()
            .name(LocalName::new_dangerous("SimpleSequence"))
            .complex_type_model(Box::new(
                xs::ComplexContent::builder()
                    .child_1(
                        xs::types::ComplexRestrictionType::builder()
                            .base(xs::types::QName(xsn::ANY_TYPE.clone()))
                            .child_1(
                                xs::types::complex_restriction_type_items::Child1::builder()
                                .type_def_particle(Box::new(
                                    xs::Sequence(xs::types::ExplicitGroup::builder()
                                        .nested_particle(vec![])
                                        .build()
                                        .into()
                                    ).into()
                                ))
                                .build()
                                .into())
                            .attr_decls(xs::groups::AttrDecls::builder().build().into())
                            .assertions(xs::groups::Assertions::builder().build().into())
                            .build()
                            .into(),
                    )
                    .build()
                    .into()
                ),
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

        let generator = Generator::new(&context);

        let (type_, actual_items) = generator.generate_top_level_type(&sequence).unwrap();

        let actual = prettyplease::unparse(&syn::File {
            shebang: None,
            attrs: Vec::new(),
            items: actual_items,
        });

        #[rustfmt::skip]
        let expected: syn::File = parse_quote!(
            #[derive(::core::fmt::Debug, ::xmlity::SerializationGroup, ::xmlity::DeserializationGroup)]
            #[xgroup(children_order = "strict")]
            pub struct SimpleSequence;
        );

        let expected = prettyplease::unparse(&expected);

        assert_eq!(actual, expected);

        assert_eq!(
            type_.ty.into_type(None),
            parse_quote!(::std::boxed::Box<SimpleSequence>)
        );
    }

    #[test]
    fn two_child_sequence_complex_type() {
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
                                        .into(),
                                )
                                .into())).build().into(),
                            )
                            .attr_decls(xs::groups::AttrDecls::builder().build().into())
                            .assertions(xs::groups::Assertions::builder().build().into())
                            .build()
                            .into(),
                    )
                    .build()
                    .into()
                ),
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
            #[derive(::core::fmt::Debug, ::xmlity::SerializationGroup, ::xmlity::DeserializationGroup)]
            #[xgroup(children_order = "strict")]
            pub struct SimpleSequence {
                #[xelement(name = "a", namespace = "http://example.com")]
                pub a: i32,
                #[xelement(name = "b", namespace = "http://example.com")]
                pub b: String,
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
    fn two_attribute_sequence_complex_type() {
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
                                    xs::types::ExplicitGroup::builder().nested_particle(vec![]).build().into()).into())
                                )
                                .build()
                                .into()
                            )
                            .attr_decls(
                                xs::groups::AttrDecls::builder()
                                    .attribute(vec![
                                        xs::types::Attribute::builder()
                                            .name(LocalName::new_dangerous("a"))
                                            .type_(xs::types::QName(xsn::INTEGER.clone()))
                                            // .use_(xs::AttributeUseType::Required)
                                            .use_("required".to_string())
                                            .build()
                                            .into(),
                                        xs::types::Attribute::builder()
                                            .name(LocalName::new_dangerous("b"))
                                            .type_(xs::types::QName(xsn::STRING.clone()))
                                            // .use_(xs::AttributeUseType::Optional)
                                            .use_("optional".to_string())
                                            .build()
                                            .into(),
                                    ])
                                    .build().into(),
                            )
                            .assertions(xs::groups::Assertions::builder().build().into())
                            .build()
                            .into(),
                    )
                    .build()
                    .into()
                ),
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
            #[derive(::core::fmt::Debug, ::xmlity::SerializationGroup, ::xmlity::DeserializationGroup)]
            #[xgroup(children_order = "strict")]
            pub struct SimpleSequence {
                #[xattribute(name = "a")]
                pub a: i32,
                #[xattribute(name = "b", optional, default)]
                pub b: ::core::option::Option<String>,
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
    fn two_sequence_deep_element() {
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
                                            xs::Sequence(
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
                                                    .into(),
                                            ).into(),
                                            xs::types::LocalElement::builder()
                                                .name(LocalName::new_dangerous("c"))
                                                .type_attribute(xs::types::QName(xsn::STRING.clone()))
                                                .build()
                                                .into(),
                                        ])
                                        .build()
                                        .into(),
                                ).into()))
                                .build()
                                .into()
                            )
                            .attr_decls(xs::groups::AttrDecls::builder().build().into())
                            .assertions(xs::groups::Assertions::builder().build().into())
                            .build()
                            .into(),
                    )
                    .build()
                    .into()
            ),
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
                #[derive(::core::fmt::Debug, ::xmlity::Serialize, ::xmlity::Deserialize)]
                #[xvalue(order = "strict")]
                pub struct Child0 {
                    #[xelement(name = "a", namespace = "http://example.com")]
                    pub a: i32,
                    #[xelement(name = "b", namespace = "http://example.com")]
                    pub b: String,
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
    fn three_sequence_deep_element() {
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
                                        xs::Sequence(xs::types::ExplicitGroup::builder()
                                            .nested_particle(vec![
                                                xs::Sequence(xs::types::ExplicitGroup::builder()
                                                    .nested_particle(vec![
                                                        xs::types::LocalElement::builder()
                                                            .name(LocalName::new_dangerous("a"))
                                                            .type_attribute(xs::types::QName(
                                                                xsn::INTEGER.clone(),
                                                            ))
                                                            .build()
                                                            .into()
                                                    ])
                                                    .build()
                                                    .into())
                                                .into(),
                                                xs::types::LocalElement::builder()
                                                    .name(LocalName::new_dangerous("b"))
                                                    .type_attribute(xs::types::QName(xsn::STRING.clone()))
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
                            ).into())).build().into())
                            .attr_decls(xs::groups::AttrDecls::builder().build().into())
                            .assertions(xs::groups::Assertions::builder().build().into())
                            .build()
                            .into(),
                    )
                    .build()
                    .into()
                ),
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
                pub mod child_0_items {
                    #[derive(::core::fmt::Debug, ::xmlity::Serialize, ::xmlity::Deserialize)]
                    #[xvalue(order = "strict")]
                    pub struct A {
                        #[xelement(name = "a", namespace = "http://example.com")]
                        pub a: i32,
                    }
                }

                #[derive(::core::fmt::Debug, ::xmlity::Serialize, ::xmlity::Deserialize)]
                #[xvalue(order = "strict")]
                pub struct Child0 {
                    pub a: child_0_items::A,
                    #[xelement(name = "b", namespace = "http://example.com")]
                    pub b: String,
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
}
