use std::{ sync::{Arc}};

use crate::{
    complex::{attributes::{ AttributeDeclarationsHandler}, groups::TypeDefParticleIdHandler, ComplexToTypeTemplateExt}, naming_strategies::WrappingNamingStrategy, templates::{
        self,
        element_record::{ElementField,  ElementFieldType},
        group_record::GroupRecord,
        value_record::ItemFieldItem,
    }, Result, ToIdentTypesExt
};

use quote::format_ident;
use syn::parse_quote;
use xsd_fragments::fragments::complex::{self as cx};

use super::{groups::TypeDefParticleTemplate, ComplexContext, Scope, ComplexToTypeTemplate, ToTypeTemplateData};

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
                let sub_context = 
                context
                .sub_context(self.content_type_naming.wrap_ident(context.suggested_ident()));

                self.type_def_particle_handler
                    .to_type_template(&sub_context, scope, &a)
            })
            .transpose()?;

        let mut template = template.map(|a| {
            let ident = a.ident.unwrap_or_else(|| format_ident!("{}", self.default_particle_ident));

                match a.template {
                    TypeDefParticleTemplate::Record(item_record) => {
                        item_record.into_group_record()
                    }
                    TypeDefParticleTemplate::Choice(item) => {
                        let item = item.to_enum(&ident, None);

                        let ty = scope.add_item(item).unwrap();

                        GroupRecord::new_single_field(
                            Some(ident.to_field_ident()),
                            ElementField::Item(ItemFieldItem {
                                ty,
                                default: false,
                                default_with: None,
                            })
                        )
                    }
                    TypeDefParticleTemplate::Item(item) => GroupRecord::new_single_field(
                        Some(ident.to_field_ident()),
                        ElementField::Item(item),
                    ),
                }
            })
            .unwrap_or_else(GroupRecord::new_empty);

        let attributes = self.attribute_declarations_handler
            .resolve_type_template(context, scope, &item.attribute_declarations)?;

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

impl ComplexToTypeTemplate<cx::SimpleExtensionFragment> for SimpleExtensionFragmentHandler {
    type TypeTemplate = templates::group_record::GroupRecord;

    fn to_type_template<C: ComplexContext, S: Scope>(
        &self,
        context: &C,
        scope: &mut S,

        item: &cx::SimpleExtensionFragment,
    ) -> Result<ToTypeTemplateData<Self::TypeTemplate>> {
        let simple_type = context.resolve_named_type(&item.base)?;

        if simple_type.ty_type != crate::TypeType::Simple {
            return Err(crate::Error::UnsupportedFragment {
                fragment: "SimpleExtensionFragment with non-simple type as base".to_string(),
            });
        }

        let mut template = GroupRecord::new_single_field(Some(format_ident!("{}", self.content_field_ident)), ElementField::Item(ItemFieldItem {
            ty: simple_type.ty,
            default: false,
            // Todo: This should only be added to certain simple types that allow empty strings
            default_with: Some(parse_quote!(::xmlity_ns::empty_str_default)),
        }));


        let attributes = self.attribute_declarations_handler
            .resolve_type_template(context, scope, &item.attribute_declarations)?;

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
            cx::SimpleContentChildId::Extension(fragment_idx) => {
                self.simple_extension_handler
                    .resolve_type_template(context, scope, &fragment_idx)
            },
            cx::SimpleContentChildId::Restriction(_) => {
                Err(crate::Error::simple_content_restriction())
            },
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
                    fragment: " ComplexContent Extension".to_string(),
                })
            }
            cx::ComplexContentChildId::Restriction(fragment_idx) => {
                self.restriction_fragment_handler
                    .resolve_type_template(context, scope, fragment_idx)
            }
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
            cx::ComplexTypeModelId::SimpleContent(fragment_idx) => {
                self.simple_content_handler
                    .resolve_type_template(context, scope, fragment_idx)
            },
            cx::ComplexTypeModelId::ComplexContent(fragment_idx) => {
                self.complex_content_handler
                    .resolve_type_template(context, scope, fragment_idx)
            }
            cx::ComplexTypeModelId::Other {
                particle,
                attr_decls,
            } => {
                let (ident, mut template) = particle
                    .as_ref()
                    .map(|particle| {
                        let sub_context = context
                            .sub_context(self.other_content_type_naming.wrap_ident(context.suggested_ident()));

                        self.type_def_particle_handler
                            .to_type_template(&sub_context, scope, particle)
                            .map(|a| {
                                (
                                    a.ident,
                                    a.template
                                        .into_group_record(Some(format_ident!("{}", self.other_content_field_ident))),
                                )
                            })
                    })
                    .unwrap_or_else(|| Ok((None, GroupRecord::new_empty())))?;

                let attributes = self.attribute_declarations_handler
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

impl ComplexToTypeTemplate <cx::ComplexTypeRootFragment> for ComplexTypeRootHandler {
    type TypeTemplate = GroupRecord;

    fn to_type_template<C: ComplexContext, S: Scope>(
        &self,
        context: &C,
        scope: &mut S,
        item: &cx::ComplexTypeRootFragment,
    ) -> Result<ToTypeTemplateData<Self::TypeTemplate>> {
        let mut fragment = self.complex_type_model_handler
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

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use syn::parse_quote;
    use xmlity::{LocalName, XmlNamespace};
    use xsd::ns;
    use xsd::xs;
    use xsd::xsn;
    use xsd_fragments::{ XmlnsContext};

    use crate::Generator;

    #[test]
    fn empty_sequence_complex_type() {
        const TEST_NAMESPACE: XmlNamespace<'static> =
            XmlNamespace::new_dangerous("http://example.com");
            
        let sequence = xs::types::TopLevelComplexType::builder()
            .name(LocalName::new_dangerous("SimpleSequence"))
            .complex_type_model(Box::new(
                xs::ComplexContent::from(xs::complex_content_items::ComplexContent::builder()
                    .child_1(
                        xs::types::ComplexRestrictionType::builder()
                            .base(xs::types::QName(xsn::ANY_TYPE.clone()))
                            .child_1(
                                xs::types::complex_restriction_type_items::Child1::builder()
                                .type_def_particle(Box::new(
                                    xs::Sequence::from(xs::types::ExplicitGroup::builder()
                                        .nested_particle(vec![])
                                        .any_attributes(ns::AnyAttributes::default())
                                        .build()
                                    ).into()
                                ))
                                .build())
                            .attr_decls(xs::groups::AttrDecls::builder().build().into())
                            .assertions(xs::groups::Assertions::builder().build().into())
                            .any_attributes(ns::AnyAttributes::default())
                            .build()
                            .into(),
                    )
                    .build())
                    .into()
                ),
            )
            .any_attributes(ns::AnyAttributes::default())
            .build()
            .into();

        let mut ctx = XmlnsContext::new();
        let ns = ctx.init_namespace(TEST_NAMESPACE);
        
        let sequence = ns
            .import_top_level_complex_type(&sequence)
            .unwrap()
            .into_owned();

        let generator = Generator::new(&ctx);

        let (type_, actual_items) = generator.generate_type(&sequence).unwrap();

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
        const TEST_NAMESPACE: XmlNamespace<'static> =
            XmlNamespace::new_dangerous("http://example.com");

        let sequence = xs::types::TopLevelComplexType::builder()
            .name(LocalName::new_dangerous("SimpleSequence"))
            .complex_type_model(
                Box::new(
                xs::ComplexContent::from(xs::complex_content_items::ComplexContent::builder()
                    .child_1(
                        xs::types::ComplexRestrictionType::builder()
                            .base(xs::types::QName(xsn::ANY_TYPE.clone()))
                            .child_1(
                                xs::types::complex_restriction_type_items::Child1::builder()
                                .type_def_particle(
                                    Box::new(
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
                                .into())).build(),
                            )
                            .attr_decls(xs::groups::AttrDecls::builder().build().into())
                            .assertions(xs::groups::Assertions::builder().build().into())
                            .any_attributes(ns::AnyAttributes::default())
                            .build()
                            .into(),
                    )
                    .build())
                    .into()
                ),
            )
            .any_attributes(ns::AnyAttributes::default())
            .build()
            .into();



        let mut ctx = XmlnsContext::new();
        let ns = ctx.init_namespace(TEST_NAMESPACE);

        let sequence = ns
            .import_top_level_complex_type(&sequence)
            .unwrap()
            .into_owned();

        let mut generator = Generator::new(&ctx);

        generator.bind_types(crate::binds::StdXsdTypes);

        let (type_, actual_items) = generator.generate_type(&sequence).unwrap();

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
        const TEST_NAMESPACE: XmlNamespace<'static> =
            XmlNamespace::new_dangerous("http://example.com");

        let sequence = xs::types::TopLevelComplexType::builder()
            .name(LocalName::new_dangerous("SimpleSequence"))
            .complex_type_model(
                Box::new(
                xs::ComplexContent::from(xs::complex_content_items::ComplexContent::builder()
                    .child_1(
                        xs::types::ComplexRestrictionType::builder()
                            .base(xs::types::QName(xsn::ANY_TYPE.clone()))
                            .child_1(
                                xs::types::complex_restriction_type_items::Child1::builder()
                                .type_def_particle(
                                    Box::new(
                                    xs::Sequence::from(
                                        xs::types::ExplicitGroup::builder()
                                            .nested_particle(vec![])
                                            .any_attributes(ns::AnyAttributes::default())
                                            .build()
                                    ).into())
                                )
                                .build()
                            )
                            .attr_decls(
                                xs::groups::AttrDecls::builder()
                                    .attribute(vec![
                                        xs::types::Attribute::builder()
                                            .name(LocalName::new_dangerous("a"))
                                            .type_(xs::types::QName(xsn::INTEGER.clone()))
                                            .use_(xs::types::attribute_items::UseValue::Required)
                                            .build()
                                            .into(),
                                        xs::types::Attribute::builder()
                                            .name(LocalName::new_dangerous("b"))
                                            .type_(xs::types::QName(xsn::STRING.clone()))
                                            .use_(xs::types::attribute_items::UseValue::Optional)
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
                    .into()
                ),
            )
            .any_attributes(ns::AnyAttributes::default())
            .build()
            .into();


        let mut ctx = XmlnsContext::new();
        let ns = ctx.init_namespace(TEST_NAMESPACE);

        let sequence = ns
            .import_top_level_complex_type(&sequence)
            .unwrap()
            .into_owned();


        let mut generator = Generator::new(&ctx);

        generator.bind_types(crate::binds::StdXsdTypes);

        let (type_, actual_items) = generator.generate_type(&sequence).unwrap();

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
                #[xattribute(name = "b", optional)]
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
        const TEST_NAMESPACE: XmlNamespace<'static> =
            XmlNamespace::new_dangerous("http://example.com");

        let sequence = xs::types::TopLevelComplexType::builder()
            .name(LocalName::new_dangerous("SimpleSequence"))
            .complex_type_model(
                Box::new(
                xs::ComplexContent::from(xs::complex_content_items::ComplexContent::builder()
                    .child_1(
                        xs::types::ComplexRestrictionType::builder()
                            .base(xs::types::QName(xsn::ANY_TYPE.clone()))
                            .child_1(
                                xs::types::complex_restriction_type_items::Child1::builder()
                                .type_def_particle(
                                    Box::new(
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
                                                    .build()
                                            ).into(),
                                            xs::types::LocalElement::builder()
                                                .name(LocalName::new_dangerous("c"))
                                                .type_attribute(xs::types::QName(xsn::STRING.clone()))
                                                .any_attributes(ns::AnyAttributes::default())
                                                .build()
                                                .into(),
                                        ])
                                        .any_attributes(ns::AnyAttributes::default())
                                        .build()
                                ).into()))
                                .build()
                            )
                            .attr_decls(xs::groups::AttrDecls::builder().build().into())
                            .assertions(xs::groups::Assertions::builder().build().into())
                            .any_attributes(ns::AnyAttributes::default())
                            .build()
                            .into(),
                    )
                    .build())
                    .into()
            ),
            )
            .any_attributes(ns::AnyAttributes::default())
            .build()
            .into();


        let mut ctx = XmlnsContext::new();
        let ns = ctx.init_namespace(TEST_NAMESPACE);

        let sequence = ns
            .import_top_level_complex_type(&sequence)
            .unwrap()
            .into_owned();

        let mut generator = Generator::new(&ctx);

        generator.bind_types(crate::binds::StdXsdTypes);

        let (type_, actual_items) = generator.generate_type(&sequence).unwrap();

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
        const TEST_NAMESPACE: XmlNamespace<'static> =
            XmlNamespace::new_dangerous("http://example.com");

        let sequence = xs::types::TopLevelComplexType::builder()
            .name(LocalName::new_dangerous("SimpleSequence"))
            .complex_type_model(
                Box::new(
                xs::ComplexContent::from(xs::complex_content_items::ComplexContent::builder()
                    .child_1(
                        xs::types::ComplexRestrictionType::builder()
                            .base(xs::types::QName(xsn::ANY_TYPE.clone()))
                            .child_1(
                                xs::types::complex_restriction_type_items::Child1::builder()
                                .type_def_particle(
                                    Box::new(
                                xs::Sequence::from(
                                xs::types::ExplicitGroup::builder()
                                    .nested_particle(vec![
                                        xs::Sequence::from(xs::types::ExplicitGroup::builder()
                                            .nested_particle(vec![
                                                xs::Sequence::from(xs::types::ExplicitGroup::builder()
                                                    .nested_particle(vec![
                                                        xs::types::LocalElement::builder()
                                                            .name(LocalName::new_dangerous("a"))
                                                            .type_attribute(xs::types::QName(
                                                                xsn::INTEGER.clone(),
                                                            ))
                                                            .any_attributes(ns::AnyAttributes::default())
                                                            .build()
                                                            .into()
                                                    ])
                                                    .any_attributes(ns::AnyAttributes::default())
                                                    .build())
                                                .into(),
                                                xs::types::LocalElement::builder()
                                                    .name(LocalName::new_dangerous("b"))
                                                    .type_attribute(xs::types::QName(xsn::STRING.clone()))
                                                    .any_attributes(ns::AnyAttributes::default())
                                                    .build()
                                                    .into(),
                                            ])
                                            .any_attributes(ns::AnyAttributes::default())
                                            .build()).into(),
                                        xs::types::LocalElement::builder()
                                            .name(LocalName::new_dangerous("c"))
                                            .type_attribute(xs::types::QName(xsn::STRING.clone()))
                                            .any_attributes(ns::AnyAttributes::default())
                                            .build()
                                            .into(),
                                    ])
                                    .any_attributes(ns::AnyAttributes::default())
                                    .build()
                            ).into())).build())
                            .attr_decls(xs::groups::AttrDecls::builder().build().into())
                            .assertions(xs::groups::Assertions::builder().build().into())
                            .any_attributes(ns::AnyAttributes::default())
                            .build()
                            .into(),
                    )
                    .build())
                    .into()
                ),
            )
            .any_attributes(ns::AnyAttributes::default())
            .build()
            .into();


        let mut ctx = XmlnsContext::new();
        let ns = ctx.init_namespace(TEST_NAMESPACE);

        let sequence = ns
            .import_top_level_complex_type(&sequence)
            .unwrap()
            .into_owned();

        let mut generator = Generator::new(&ctx);

        generator.bind_types(crate::binds::StdXsdTypes);

        let (type_, actual_items) = generator.generate_type(&sequence).unwrap();

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
