use crate::{
    templates::{self, element_record::ElementField, group_record::GroupRecord, FieldType},
    Result,
};

use quote::format_ident;
use xsd_type_compiler::complex::{self as cx};

use super::{groups::TypeDefParticleTemplate, Context, ToTypeTemplate, ToTypeTemplateData};

impl ToTypeTemplate for cx::RestrictionFragment {
    type TypeTemplate = templates::group_record::GroupRecord;

    fn to_type_template<C: Context>(
        &self,
        context: &C,
    ) -> Result<ToTypeTemplateData<Self::TypeTemplate>> {
        let mut template = self
            .content_fragment
            .map(|a| {
                context.resolve_fragment(&a).map(|a| match a.template {
                    TypeDefParticleTemplate::Record(item_record) => item_record.into_group_record(),
                    TypeDefParticleTemplate::Choice(_choice) => todo!(),
                })
            })
            .transpose()?
            .unwrap_or_else(|| GroupRecord::new_empty(FieldType::Named));

        let attributes = self
            .attribute_declarations
            .iter()
            .enumerate()
            .map(|(i, a)| {
                context.resolve_fragment(a).map(|a| {
                    (
                        Some(a.ident.unwrap_or_else(|| format_ident!("attribute_{i}"))),
                        ElementField::Attribute(a.template),
                    )
                })
            });

        template.fields = attributes
            .chain(template.fields.into_iter().map(Ok))
            .collect::<Result<Vec<_>>>()?;

        Ok(ToTypeTemplateData {
            ident: None,
            template,
        })
    }
}

impl ToTypeTemplate for cx::ComplexContentFragment {
    type TypeTemplate = templates::group_record::GroupRecord;

    fn to_type_template<C: Context>(
        &self,
        context: &C,
    ) -> Result<ToTypeTemplateData<Self::TypeTemplate>> {
        match &self.content_fragment {
            cx::ComplexContentChildId::Extension(_fragment_idx) => {
                panic!("TODO: Should give error. Extension not supported")
            }
            cx::ComplexContentChildId::Restriction(fragment_idx) => {
                context.resolve_fragment_id(fragment_idx)
            }
        }
    }
}

impl ToTypeTemplate for cx::ComplexTypeModelId {
    type TypeTemplate = templates::group_record::GroupRecord;

    fn to_type_template<C: Context>(
        &self,
        context: &C,
    ) -> Result<ToTypeTemplateData<Self::TypeTemplate>> {
        match self {
            cx::ComplexTypeModelId::SimpleContent(_fragment_idx) => todo!(),
            cx::ComplexTypeModelId::ComplexContent(fragment_idx) => {
                context.resolve_fragment_id(fragment_idx)
            }
            cx::ComplexTypeModelId::Other { particle: _ } => todo!(),
        }
    }
}

impl ToTypeTemplate for cx::ComplexTypeRootFragment {
    type TypeTemplate = templates::group_record::GroupRecord;

    fn to_type_template<C: Context>(
        &self,
        context: &C,
    ) -> Result<ToTypeTemplateData<Self::TypeTemplate>> {
        let mut fragment = context.resolve_fragment(&self.content)?;

        let name_ident = self
            .name
            .as_ref()
            .map(ToString::to_string)
            .map(|a| format_ident!("{a}"));

        fragment.ident = name_ident.or(fragment.ident);

        Ok(fragment)
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use syn::{parse_quote, Item};
    use xmlity::{ExpandedName, LocalName, XmlNamespace};
    use xsd::schema as xs;
    use xsd_type_compiler::{complex::ANY_TYPE_EXPANDED_NAME, CompiledNamespace, XmlnsContext};

    use crate::Generator;

    #[test]
    fn empty_sequence_complex_type() {
        let sequence = xs::TopLevelComplexType::builder()
            .name(LocalName::new_dangerous("SimpleSequence"))
            .content(
                xs::ComplexContent::builder()
                    .content(
                        xs::ComplexRestrictionType::builder()
                            .base(xs::Base(xs::QName(ANY_TYPE_EXPANDED_NAME.clone())))
                            .particle(xs::SequenceType::builder().content(vec![]).build().into())
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

        let generator = Generator::new(&context);

        let (type_, actual_items) = generator.generate_top_level_type(&sequence).unwrap();

        #[rustfmt::skip]
        let expected_items: Vec<Item> = vec![
            parse_quote!(
                #[derive(::core::fmt::Debug, ::xmlity::SerializationGroup, ::xmlity::DeserializationGroup)]
                #[xgroup(children_order = "strict")]
                pub struct SimpleSequence;
            )
        ];

        assert_eq!(expected_items, actual_items);

        assert_eq!(type_.into_type(None), parse_quote!(SimpleSequence));
    }

    #[test]
    fn two_child_sequence_complex_type() {
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
                                        xs::LocalElement::builder()
                                            .name(xs::Name(LocalName::new_dangerous("a")))
                                            .type_(xs::Type(xs::QName(
                                                integer_expanded_name.clone(),
                                            )))
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

        generator.bind_type(integer_expanded_name, parse_quote!(i32));
        generator.bind_type(string_expanded_name, parse_quote!(String));

        let (type_, actual_items) = generator.generate_top_level_type(&sequence).unwrap();

        #[rustfmt::skip]
        let expected_items: Vec<Item> = vec![
            parse_quote!(
                #[derive(::core::fmt::Debug, ::xmlity::SerializationGroup, ::xmlity::DeserializationGroup)]
                #[xgroup(children_order = "strict")]
                pub struct SimpleSequence {
                    #[xelement(name = "a", namespace = "http://example.com")]
                    pub a: i32,
                    #[xelement(name = "b", namespace = "http://example.com")]
                    pub b: String,
                }
            )
        ];

        assert_eq!(expected_items, actual_items);

        assert_eq!(type_.into_type(None), parse_quote!(SimpleSequence));
    }

    #[test]
    fn two_attribute_sequence_complex_type() {
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
                            .particle(xs::SequenceType::builder().content(vec![]).build().into())
                            .attributes(vec![
                                xs::LocalAttribute::builder()
                                    .name(xs::Name(LocalName::new_dangerous("a")))
                                    .type_(xs::Type(xs::QName(integer_expanded_name.clone())))
                                    .use_(xs::AttrUse(xs::AttributeUseType::Required))
                                    .build()
                                    .into(),
                                xs::LocalAttribute::builder()
                                    .name(xs::Name(LocalName::new_dangerous("b")))
                                    .type_(xs::Type(xs::QName(string_expanded_name.clone())))
                                    .use_(xs::AttrUse(xs::AttributeUseType::Optional))
                                    .build()
                                    .into(),
                            ])
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

        generator.bind_type(string_expanded_name, parse_quote!(String));
        generator.bind_type(integer_expanded_name, parse_quote!(i32));

        let (type_, actual_items) = generator.generate_top_level_type(&sequence).unwrap();

        #[rustfmt::skip]
        let expected_items: Vec<Item> = vec![
            parse_quote!(
                #[derive(::core::fmt::Debug, ::xmlity::SerializationGroup, ::xmlity::DeserializationGroup)]
                #[xgroup(children_order = "strict")]
                pub struct SimpleSequence {
                    #[xattribute(name = "a")]
                    pub a: i32,
                    #[xattribute(name = "b", optional, default)]
                    pub b: Option<String>,
                }
            )
        ];

        assert_eq!(expected_items, actual_items);

        assert_eq!(type_.into_type(None), parse_quote!(SimpleSequence));
    }
}
