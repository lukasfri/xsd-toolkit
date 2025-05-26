use crate::{
    misc::TypeReference,
    templates::{
        self, element_record::ElementRecord, value_record::ItemFieldItem, FieldType, ItemOrder,
    },
    Result, TypeType,
};

use quote::format_ident;
use xsd::schema::MaxOccursValue;
use xsd_type_compiler::complex::{self as cx};

use super::{Context, Scope, ToTypeTemplate, ToTypeTemplateData};

impl ToTypeTemplate for cx::ElementTypeContentId {
    type TypeTemplate = templates::group_record::GroupRecord;

    fn to_type_template<C: Context, S: Scope>(
        &self,
        context: &C,
        scope: &mut S,
    ) -> Result<ToTypeTemplateData<Self::TypeTemplate>> {
        match self {
            cx::ElementTypeContentId::SimpleType(_fragment_id) => todo!(),
            cx::ElementTypeContentId::ComplexType(fragment_idx) => {
                context.resolve_fragment_id(fragment_idx, scope)
            }
        }
    }
}

impl ToTypeTemplate for cx::DeclaredElementFragment {
    type TypeTemplate = templates::element_record::ElementRecord;

    fn to_type_template<C: Context, S: Scope>(
        &self,
        context: &C,
        scope: &mut S,
    ) -> Result<ToTypeTemplateData<Self::TypeTemplate>> {
        let name = context.to_expanded_name(self.name.clone());
        let ident = format_ident!("{}", self.name.to_string());

        match &self.type_ {
            xsd_type_compiler::NamedOrAnonymous::Named(expanded_name) => {
                let (ty, ty_type) = context.resolve_named_type(expanded_name)?;

                let ty = TypeReference::new_static(ty);

                let field = match ty_type {
                    TypeType::Simple => {
                        templates::element_record::ElementField::Item(ItemFieldItem { ty })
                    }
                    TypeType::Complex => templates::element_record::ElementField::Group(
                        templates::element_record::ElementFieldGroup { ty },
                    ),
                };

                let template = templates::element_record::ElementRecord {
                    name,
                    attribute_order: ItemOrder::None,
                    children_order: ItemOrder::None,
                    field_type: FieldType::Unnamed,
                    fields: vec![(None, field)],
                };

                Ok(ToTypeTemplateData {
                    ident: Some(ident),
                    template,
                })
            }
            xsd_type_compiler::NamedOrAnonymous::Anonymous(anonymous) => {
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

impl ToTypeTemplate for cx::ReferenceElementFragment {
    type TypeTemplate = ItemFieldItem;

    fn to_type_template<C: Context, S: Scope>(
        &self,
        context: &C,
        _scope: &mut S,
    ) -> Result<ToTypeTemplateData<Self::TypeTemplate>> {
        let element = context.resolve_named_element(&self.name)?;
        let ty = TypeReference::new_static(element);

        let template = ItemFieldItem { ty };

        Ok(ToTypeTemplateData {
            ident: Some(format_ident!("{}", self.name.local_name().to_string())),
            template,
        })
    }
}

pub enum LocalElementFragmentTemplate {
    ElementRecord {
        template: ElementRecord,
        min_occurs: usize,
        max_occurs: MaxOccursValue,
    },
    Item(ItemFieldItem),
}

impl ToTypeTemplate for cx::LocalElementFragment {
    type TypeTemplate = LocalElementFragmentTemplate;

    fn to_type_template<C: Context, S: Scope>(
        &self,
        context: &C,
        scope: &mut S,
    ) -> Result<ToTypeTemplateData<Self::TypeTemplate>> {
        let min_occurs = self.min_occurs.map(|a| a.0).unwrap_or(1);
        let max_occurs = self.max_occurs.unwrap_or(MaxOccursValue::Bounded(1));

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

                let ty = super::min_max_occurs_type(min_occurs, max_occurs, reference.template.ty);

                let template = LocalElementFragmentTemplate::Item(ItemFieldItem { ty });

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

    fn to_type_template<C: Context, S: Scope>(
        &self,
        context: &C,
        scope: &mut S,
    ) -> Result<ToTypeTemplateData<Self::TypeTemplate>> {
        let name = context.to_expanded_name(self.name.clone());
        let ident = format_ident!("{}", self.name.to_string());
        match &self.type_ {
            xsd_type_compiler::NamedOrAnonymous::Named(expanded_name) => {
                let (ty, ty_type) = context.resolve_named_type(expanded_name)?;

                let ty = TypeReference::new_static(ty);

                let field = match ty_type {
                    TypeType::Simple => {
                        templates::element_record::ElementField::Item(ItemFieldItem { ty })
                    }
                    TypeType::Complex => templates::element_record::ElementField::Group(
                        templates::element_record::ElementFieldGroup { ty },
                    ),
                };

                let template = templates::element_record::ElementRecord {
                    name,
                    attribute_order: ItemOrder::None,
                    children_order: ItemOrder::None,
                    field_type: FieldType::Unnamed,
                    fields: vec![(None, field)],
                };

                Ok(ToTypeTemplateData {
                    ident: Some(ident),
                    template,
                })
            }
            xsd_type_compiler::NamedOrAnonymous::Anonymous(anonymous) => {
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

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use syn::{parse_quote, Item};
    use xmlity::{ExpandedName, LocalName, XmlNamespace};
    use xsd::schema as xs;
    use xsd_type_compiler::{complex::ANY_TYPE_EXPANDED_NAME, CompiledNamespace, XmlnsContext};

    use crate::{Generator, TypeType};

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
                #[xelement(name = "SimpleSequence", namespace = "http://example.com", children_order = "strict")]
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

        let mut compiled_namespace = CompiledNamespace::new(namespace.clone());

        let sequence = compiled_namespace
            .import_top_level_element(&sequence)
            .unwrap()
            .into_owned();

        let mut context = XmlnsContext::new();
        context.add_namespace(compiled_namespace);

        let mut generator = Generator::new(&context);

        generator.bind_type(integer_expanded_name, parse_quote!(i32), TypeType::Simple);
        generator.bind_type(string_expanded_name, parse_quote!(String), TypeType::Simple);

        let (type_, actual_items) = generator.generate_top_level_element(&sequence).unwrap();

        #[rustfmt::skip]
        let expected_items: Vec<Item> = vec![
            parse_quote!(
                #[derive(::core::fmt::Debug, ::xmlity::Serialize, ::xmlity::Deserialize)]
                #[xelement(name = "SimpleSequence", namespace = "http://example.com", children_order = "strict")]
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
                                            .use_(xs::AttrUse(xs::AttributeUseType::Required))
                                            .build()
                                            .into(),
                                        xs::LocalAttribute::builder()
                                            .name(xs::Name(LocalName::new_dangerous("b")))
                                            .type_(xs::Type(xs::QName(
                                                string_expanded_name.clone(),
                                            )))
                                            .use_(xs::AttrUse(xs::AttributeUseType::Required))
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
            .import_top_level_element(&sequence)
            .unwrap()
            .into_owned();

        let mut context = XmlnsContext::new();
        context.add_namespace(compiled_namespace);

        let mut generator = Generator::new(&context);

        generator.bind_type(string_expanded_name, parse_quote!(String), TypeType::Simple);
        generator.bind_type(integer_expanded_name, parse_quote!(i32), TypeType::Simple);

        let (type_, actual_items) = generator.generate_top_level_element(&sequence).unwrap();

        #[rustfmt::skip]
        let expected_items: Vec<Item> = vec![
            parse_quote!(
                #[derive(::core::fmt::Debug, ::xmlity::Serialize, ::xmlity::Deserialize)]
                #[xelement(name = "SimpleSequence", namespace = "http://example.com", children_order = "strict")]
                pub struct SimpleSequence {
                    #[xattribute(name = "a")]
                    pub a: i32,
                    #[xattribute(name = "b")]
                    pub b: String,
                }
            )
        ];

        assert_eq!(expected_items, actual_items);

        assert_eq!(type_.into_type(None), parse_quote!(SimpleSequence));
    }

    #[test]
    fn two_sequence_deep_element() {
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
                                                xs::SequenceType::builder()
                                                    .content(vec![
                                                        xs::LocalElement::builder()
                                                            .name(xs::Name(
                                                                LocalName::new_dangerous("a"),
                                                            ))
                                                            .type_(xs::Type(xs::QName(
                                                                integer_expanded_name.clone(),
                                                            )))
                                                            .build()
                                                            .into(),
                                                        xs::LocalElement::builder()
                                                            .name(xs::Name(
                                                                LocalName::new_dangerous("b"),
                                                            ))
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
                    .build()
                    .into(),
            )
            .build();

        let namespace = XmlNamespace::new_dangerous("http://example.com");

        let mut compiled_namespace = CompiledNamespace::new(namespace.clone());

        let sequence = compiled_namespace
            .import_top_level_element(&sequence)
            .unwrap()
            .into_owned();

        let mut context = XmlnsContext::new();
        context.add_namespace(compiled_namespace);

        let mut generator = Generator::new(&context);

        generator.bind_type(string_expanded_name, parse_quote!(String), TypeType::Simple);
        generator.bind_type(integer_expanded_name, parse_quote!(i32), TypeType::Simple);

        let (type_, actual_items) = generator.generate_top_level_element(&sequence).unwrap();

        #[rustfmt::skip]
        let expected_items: Vec<Item> = vec![
            parse_quote!(
                pub mod SimpleSequence_items {
                    #[derive(::core::fmt::Debug, ::xmlity::Serialize, ::xmlity::Deserialize)]
                    #[xvalue(children_order = "strict")]
                    pub struct SimpleSequenceChild {
                        #[xelement(name = "a", namespace = "http://example.com")]
                        pub a: i32,
                        #[xelement(name = "b", namespace = "http://example.com")]
                        pub b: String,
                    }
                }
            ),
            parse_quote!(
                #[derive(::core::fmt::Debug, ::xmlity::Serialize, ::xmlity::Deserialize)]
                #[xelement(name = "SimpleSequence", namespace = "http://example.com", children_order = "strict")]
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

    #[test]
    fn two_attribute_two_children_sequence_element() {
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
                                    .attributes(vec![
                                        xs::LocalAttribute::builder()
                                            .name(xs::Name(LocalName::new_dangerous("c")))
                                            .type_(xs::Type(xs::QName(
                                                integer_expanded_name.clone(),
                                            )))
                                            .use_(xs::AttrUse(xs::AttributeUseType::Required))
                                            .build()
                                            .into(),
                                        xs::LocalAttribute::builder()
                                            .name(xs::Name(LocalName::new_dangerous("d")))
                                            .type_(xs::Type(xs::QName(
                                                string_expanded_name.clone(),
                                            )))
                                            .use_(xs::AttrUse(xs::AttributeUseType::Required))
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
            .import_top_level_element(&sequence)
            .unwrap()
            .into_owned();

        let mut context = XmlnsContext::new();
        context.add_namespace(compiled_namespace);

        let mut generator = Generator::new(&context);

        generator.bind_type(string_expanded_name, parse_quote!(String), TypeType::Simple);
        generator.bind_type(integer_expanded_name, parse_quote!(i32), TypeType::Simple);

        let (type_, actual_items) = generator.generate_top_level_element(&sequence).unwrap();

        #[rustfmt::skip]
        let expected_items: Vec<Item> = vec![
            parse_quote!(
                #[derive(::core::fmt::Debug, ::xmlity::Serialize, ::xmlity::Deserialize)]
                #[xelement(name = "SimpleSequence", namespace = "http://example.com", children_order = "strict")]
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
            )
        ];

        assert_eq!(expected_items, actual_items);

        assert_eq!(type_.into_type(None), parse_quote!(SimpleSequence));
    }
}
