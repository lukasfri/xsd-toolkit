use crate::{
    templates::{
        self,
        value_record::{ItemField, ItemFieldItem},
        FieldType, ItemOrder,
    },
    Result,
};

use quote::format_ident;
use xsd_type_compiler::complex::{self as cx};

use super::{Context, ToTypeTemplate, ToTypeTemplateData};

impl ToTypeTemplate for cx::ElementTypeContentId {
    type TypeTemplate = templates::group_record::GroupRecord;

    fn to_type_template<C: Context>(
        &self,
        context: &C,
    ) -> Result<ToTypeTemplateData<Self::TypeTemplate>> {
        match self {
            cx::ElementTypeContentId::SimpleType(fragment_id) => todo!(),
            cx::ElementTypeContentId::ComplexType(fragment_idx) => {
                context.resolve_fragment_id(fragment_idx)
            }
        }
    }
}

impl ToTypeTemplate for cx::DeclaredElementFragment {
    type TypeTemplate = templates::element_record::ElementRecord;

    fn to_type_template<C: Context>(
        &self,
        context: &C,
    ) -> Result<ToTypeTemplateData<Self::TypeTemplate>> {
        let name = context.to_expanded_name(self.name.clone());
        let ident = format_ident!("{}", self.name.to_string());
        match &self.type_ {
            xsd_type_compiler::NamedOrAnonymous::Named(expanded_name) => {
                let group_type = context.resolve_named_type(expanded_name)?;

                let template = templates::element_record::ElementRecord {
                    name,
                    attribute_order: ItemOrder::None,
                    children_order: ItemOrder::None,
                    field_type: FieldType::Unnamed,
                    fields: vec![(
                        None,
                        templates::element_record::ElementField::Group(
                            templates::element_record::ElementFieldGroup { ty: group_type },
                        ),
                    )],
                };

                Ok(ToTypeTemplateData {
                    ident: Some(ident),
                    template,
                })
            }
            xsd_type_compiler::NamedOrAnonymous::Anonymous(anonymous) => {
                let sub_type = context.resolve_fragment(anonymous)?;

                let template = sub_type.template.into_element_record(name);

                Ok(ToTypeTemplateData {
                    ident: Some(ident),
                    template,
                })
            }
        }
    }
}

impl ToTypeTemplate for cx::ReferenceElementFragment {
    type TypeTemplate = ItemFieldItem;

    fn to_type_template<C: Context>(
        &self,
        context: &C,
    ) -> Result<ToTypeTemplateData<Self::TypeTemplate>> {
        let ident = todo!();

        let ty = todo!();

        let template = ItemFieldItem { ty };

        Ok(ToTypeTemplateData { ident, template })
    }
}

impl ToTypeTemplate for cx::LocalElementFragment {
    type TypeTemplate = ItemField;

    fn to_type_template<C: Context>(
        &self,
        context: &C,
    ) -> Result<ToTypeTemplateData<Self::TypeTemplate>> {
        match self {
            cx::LocalElementFragment::Local(local) => {
                let local = context.resolve_fragment(local)?;

                let template = local
                    .template
                    .try_into_compact_item_field()
                    .map(ItemField::Element)
                    .unwrap_or_else(|element| {
                        let ty = todo!();

                        ItemField::Item(ItemFieldItem { ty })
                    });

                Ok(ToTypeTemplateData {
                    ident: local.ident,
                    template,
                })
            }
            cx::LocalElementFragment::Reference(reference) => {
                let reference = context.resolve_fragment(reference)?;

                let template = ItemField::Item(reference.template);

                Ok(ToTypeTemplateData {
                    ident: reference.ident,
                    template,
                })
            }
        }
    }
}

impl ToTypeTemplate for cx::TopLevelElementFragment {
    type TypeTemplate = templates::element_record::ElementRecord;

    fn to_type_template<C: Context>(
        &self,
        context: &C,
    ) -> Result<ToTypeTemplateData<Self::TypeTemplate>> {
        let name = context.to_expanded_name(self.name.clone());
        let ident = format_ident!("{}", self.name.to_string());
        match &self.type_ {
            xsd_type_compiler::NamedOrAnonymous::Named(expanded_name) => {
                let group_type = context.resolve_named_type(expanded_name)?;

                let template = templates::element_record::ElementRecord {
                    name,
                    attribute_order: ItemOrder::None,
                    children_order: ItemOrder::None,
                    field_type: FieldType::Unnamed,
                    fields: vec![(
                        None,
                        templates::element_record::ElementField::Group(
                            templates::element_record::ElementFieldGroup { ty: group_type },
                        ),
                    )],
                };

                Ok(ToTypeTemplateData {
                    ident: Some(ident),
                    template,
                })
            }
            xsd_type_compiler::NamedOrAnonymous::Anonymous(anonymous) => {
                let sub_type = context.resolve_fragment(anonymous)?;

                let template = sub_type.template.into_element_record(name);

                Ok(ToTypeTemplateData {
                    ident: Some(ident),
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

    use crate::Generator;

    #[test]
    fn empty_sequence_element() {
        let sequence = xs::TopLevelElement::builder()
            .name(xs::Name(LocalName::new_dangerous("SimpleSequence")))
            .type_choice(
                xs::LocalComplexType::builder()
                    .content(
                        xs::ComplexContent::builder()
                            .content(
                                xs::ComplexRestrictionType::builder()
                                    .base(xs::Base(xs::QName(ANY_TYPE_EXPANDED_NAME.clone())))
                                    .particle(
                                        xs::SequenceType::builder().content(vec![]).build().into(),
                                    )
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
        let namespace_lit = namespace.as_str();

        let mut compiled_namespace = CompiledNamespace::new(namespace.clone());

        let sequence = compiled_namespace
            .import_top_level_element(&sequence)
            .unwrap()
            .into_owned();

        let mut context = XmlnsContext::new();
        context.add_namespace(compiled_namespace);

        let generator = Generator::new(&context);

        let (type_, actual_items) = generator.generate_top_level_element(&sequence).unwrap();

        #[rustfmt::skip]
        let expected_items: Vec<Item> = vec![
            parse_quote!(
                #[derive(::core::fmt::Debug, ::xmlity::Serialize, ::xmlity::Deserialize)]
                #[xelement(name = "SimpleSequence", namespace = #namespace_lit, children_order = "strict")]
                pub struct SimpleSequence;
            )
        ];

        assert_eq!(expected_items, actual_items);

        assert_eq!(type_.into_type(None), parse_quote!(SimpleSequence));
    }

    #[test]
    fn two_child_sequence_element() {
        let integer_expanded_name = ExpandedName::new(
            LocalName::new_dangerous("integer"),
            XmlNamespace::XMLNS.into(),
        );
        let string_expanded_name = ExpandedName::new(
            LocalName::new_dangerous("string"),
            XmlNamespace::XMLNS.into(),
        );

        let sequence = xs::TopLevelElement::builder()
            .name(xs::Name(LocalName::new_dangerous("SimpleSequence")))
            .type_choice(
                xs::LocalComplexType::builder()
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
                    .build()
                    .into(),
            )
            .build();

        let namespace = XmlNamespace::new_dangerous("http://example.com");
        let namespace_lit = namespace.as_str();

        let mut compiled_namespace = CompiledNamespace::new(namespace.clone());

        let sequence = compiled_namespace
            .import_top_level_element(&sequence)
            .unwrap()
            .into_owned();

        let mut context = XmlnsContext::new();
        context.add_namespace(compiled_namespace);

        let mut generator = Generator::new(&context);

        generator.bind_type(integer_expanded_name, parse_quote!(i32));
        generator.bind_type(string_expanded_name, parse_quote!(String));

        let (type_, actual_items) = generator.generate_top_level_element(&sequence).unwrap();

        #[rustfmt::skip]
        let expected_items: Vec<Item> = vec![
            parse_quote!(
                #[derive(::core::fmt::Debug, ::xmlity::Serialize, ::xmlity::Deserialize)]
                #[xelement(name = "SimpleSequence", namespace = #namespace_lit, children_order = "strict")]
                pub struct SimpleSequence {
                    #[xelement(name = "a", namespace = #namespace_lit)]
                    pub a: i32,
                    #[xelement(name = "b", namespace = #namespace_lit)]
                    pub b: String,
                }
            )
        ];

        assert_eq!(expected_items, actual_items);

        assert_eq!(type_.into_type(None), parse_quote!(SimpleSequence));
    }

    #[test]
    fn two_attribute_sequence_element() {
        let integer_expanded_name = ExpandedName::new(
            LocalName::new_dangerous("integer"),
            XmlNamespace::XMLNS.into(),
        );
        let string_expanded_name = ExpandedName::new(
            LocalName::new_dangerous("string"),
            XmlNamespace::XMLNS.into(),
        );

        let sequence = xs::TopLevelElement::builder()
            .name(xs::Name(LocalName::new_dangerous("SimpleSequence")))
            .type_choice(
                xs::LocalComplexType::builder()
                    .content(
                        xs::ComplexContent::builder()
                            .content(
                                xs::ComplexRestrictionType::builder()
                                    .base(xs::Base(xs::QName(ANY_TYPE_EXPANDED_NAME.clone())))
                                    .particle(
                                        xs::SequenceType::builder().content(vec![]).build().into(),
                                    )
                                    .attributes(vec![
                                        xs::LocalAttribute::builder()
                                            .name(xs::Name(LocalName::new_dangerous("a")))
                                            .type_(xs::Type(xs::QName(
                                                integer_expanded_name.clone(),
                                            )))
                                            .build()
                                            .into(),
                                        xs::LocalAttribute::builder()
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
        let namespace_lit = namespace.as_str();

        let mut compiled_namespace = CompiledNamespace::new(namespace.clone());

        let sequence = compiled_namespace
            .import_top_level_element(&sequence)
            .unwrap()
            .into_owned();

        let mut context = XmlnsContext::new();
        context.add_namespace(compiled_namespace);

        let generator = Generator::new(&context);

        let (type_, actual_items) = generator.generate_top_level_element(&sequence).unwrap();

        #[rustfmt::skip]
        let expected_items: Vec<Item> = vec![
            parse_quote!(
                #[derive(::core::fmt::Debug, ::xmlity::Serialize, ::xmlity::Deserialize)]
                #[xelement(name = "SimpleSequence", namespace = #namespace_lit, children_order = "strict")]
                pub struct SimpleSequence {
                    #[xattribute(name = "a")]
                    pub a: String,
                    #[xattribute(name = "b")]
                    pub b: String,
                }
            )
        ];

        assert_eq!(expected_items, actual_items);

        assert_eq!(type_.into_type(None), parse_quote!(SimpleSequence));
    }
}
