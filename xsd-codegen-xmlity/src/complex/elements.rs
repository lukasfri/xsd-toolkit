use crate::{
    misc::TypeReference,
    templates::{
        self,
        element_record::{AllowUnknown, ElementFieldType, ElementRecord},
        group_record::GroupRecord,
        value_record::ItemFieldItem,
        ItemOrder,
    },
    Result, ToIdentTypesExt, TypeType,
};

use xsd::schema::MaxOccursValue;
use xsd_type_compiler::complex::{self as cx};

use super::{Context, Scope, ToTypeTemplate, ToTypeTemplateData};

impl ToTypeTemplate for cx::ElementTypeContentId {
    type TypeTemplate = GroupRecord;

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

fn type_to_element_field(
    ty: TypeReference<'static>,
    ty_type: TypeType,
    default: bool,
) -> templates::element_record::ElementField {
    match ty_type {
        TypeType::Simple => {
            templates::element_record::ElementField::Item(ItemFieldItem { ty, default })
        }
        TypeType::Complex => templates::element_record::ElementField::Group(
            templates::element_record::ElementFieldGroup { ty },
        ),
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
        let ident = self.name.to_item_ident();

        match &self.type_ {
            xsd_type_compiler::NamedOrAnonymous::Named(expanded_name) => {
                let bound_type = context.resolve_named_type(expanded_name)?;

                let field = type_to_element_field(bound_type.ty, bound_type.ty_type, false);

                let template =
                    templates::element_record::ElementRecord::new_single_field(name, None, field);

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

impl ToTypeTemplate for cx::TopLevelElementFragment {
    type TypeTemplate = templates::element_record::ElementRecord;

    fn to_type_template<C: Context, S: Scope>(
        &self,
        context: &C,
        scope: &mut S,
    ) -> Result<ToTypeTemplateData<Self::TypeTemplate>> {
        let name = context.to_expanded_name(self.name.clone());
        let ident = self.name.to_item_ident();

        let type_ = self.type_.as_ref();

        let template = match type_ {
            Some(xsd_type_compiler::NamedOrAnonymous::Named(expanded_name)) => {
                let bound_type = context.resolve_named_type(expanded_name)?;

                let field = type_to_element_field(bound_type.ty, bound_type.ty_type, false);

                templates::element_record::ElementRecord {
                    name,
                    attribute_order: ItemOrder::None,
                    children_order: ItemOrder::None,
                    fields: ElementFieldType::Unnamed(vec![field]),
                    allow_unknown_attributes: AllowUnknown::Any,
                    allow_unknown_children: AllowUnknown::AtEnd,
                }
            }
            Some(xsd_type_compiler::NamedOrAnonymous::Anonymous(anonymous)) => {
                let sub_type = context.resolve_fragment(anonymous, scope)?;

                sub_type.template.into_element_record(name)
            }
            None => ElementRecord::new_empty(name),
        };

        Ok(ToTypeTemplateData {
            ident: Some(ident),
            template,
        })
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use syn::parse_quote;
    use xmlity::{ExpandedName, LocalName, XmlNamespace};
    use xsd::schema as xs;
    use xsd::schema_names as xsn;
    use xsd_type_compiler::{CompiledNamespace, XmlnsContext};

    use crate::misc::TypeReference;
    use crate::BoundType;
    use crate::{Generator, TypeType};

    #[test]
    fn empty_sequence_element() {
        let sequence = xs::types::TopLevelElement::builder()
            .name(LocalName::new_dangerous("SimpleSequence"))
            .child_1(
                xs::LocalComplexType::builder()
                    .content(
                        xs::ComplexContent::builder()
                            .content(
                                xs::ComplexRestrictionType::builder()
                                    .base(xs::QName(xsn::ANY_TYPE.clone()))
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
            .build()
            .into();

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

        let actual = prettyplease::unparse(&syn::File {
            shebang: None,
            attrs: Vec::new(),
            items: actual_items,
        });

        #[rustfmt::skip]
        let expected: syn::File = parse_quote!(
            #[derive(::core::fmt::Debug, ::xmlity::Serialize, ::xmlity::Deserialize)]
            #[xelement(name = "SimpleSequence", namespace = "http://example.com", allow_unknown_attributes = "any", children_order = "strict")]
            pub struct SimpleSequence;
        );

        let expected = prettyplease::unparse(&expected);

        assert_eq!(actual, expected);

        assert_eq!(type_.into_type(None), parse_quote!(SimpleSequence));
    }

    #[test]
    fn two_child_sequence_element() {
        let sequence = xs::TopLevelElement(
            xs::types::TopLevelElement::builder()
                .name(LocalName::new_dangerous("SimpleSequence"))
                .child_1(
                    xs::LocalComplexType::builder()
                        .content(
                            xs::ComplexContent::builder()
                                .content(
                                    xs::ComplexRestrictionType::builder()
                                        .base(xs::QName(xsn::ANY_TYPE.clone()))
                                        .particle(
                                            xs::SequenceType::builder()
                                                .content(vec![
                                                    xs::LocalElement::builder()
                                                        .name(LocalName::new_dangerous("a"))
                                                        .type_(xs::QName(xsn::INTEGER.clone()))
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
                .build(),
        );

        let namespace = XmlNamespace::new_dangerous("http://example.com");

        let mut compiled_namespace = CompiledNamespace::new(namespace.clone());

        let sequence = compiled_namespace
            .import_top_level_element(&sequence)
            .unwrap()
            .into_owned();

        let mut context = XmlnsContext::new();
        context.add_namespace(compiled_namespace);

        let mut generator = Generator::new(&context);

        generator.bind_types(crate::binds::StdXsdTypes);

        let (type_, actual_items) = generator.generate_top_level_element(&sequence).unwrap();

        let actual = prettyplease::unparse(&syn::File {
            shebang: None,
            attrs: Vec::new(),
            items: actual_items.clone(),
        });

        #[rustfmt::skip]
        let expected: syn::File = parse_quote!(
            #[derive(::core::fmt::Debug, ::xmlity::Serialize, ::xmlity::Deserialize)]
            #[xelement(name = "SimpleSequence", namespace = "http://example.com", allow_unknown_attributes = "any", children_order = "strict")]
            pub struct SimpleSequence {
                #[xelement(name = "a", namespace = "http://example.com")]
                pub a: i32,
                #[xelement(name = "b", namespace = "http://example.com")]
                pub b: String,
            }
        );

        let expected = prettyplease::unparse(&expected);

        assert_eq!(actual, expected);

        assert_eq!(type_.into_type(None), parse_quote!(SimpleSequence));
    }

    #[test]
    fn two_attribute_sequence_element() {
        let sequence = xs::TopLevelElement(
            xs::types::TopLevelElement::builder()
                .name(LocalName::new_dangerous("SimpleSequence"))
                .child_1(
                    xs::LocalComplexType::builder()
                        .content(
                            xs::ComplexContent::builder()
                                .content(
                                    xs::ComplexRestrictionType::builder()
                                        .base(xs::QName(xsn::ANY_TYPE.clone()))
                                        .particle(
                                            xs::SequenceType::builder()
                                                .content(vec![])
                                                .build()
                                                .into(),
                                        )
                                        .attr_decls(
                                            xs::AttrDecls::builder()
                                                .declarations(vec![
                                                    xs::LocalAttribute::builder()
                                                        .name(LocalName::new_dangerous("a"))
                                                        .type_(xs::QName(xsn::INTEGER.clone()))
                                                        .use_(xs::AttributeUseType::Required)
                                                        .build()
                                                        .into(),
                                                    xs::LocalAttribute::builder()
                                                        .name(LocalName::new_dangerous("b"))
                                                        .type_(xs::QName(xsn::STRING.clone()))
                                                        .use_(xs::AttributeUseType::Required)
                                                        .build()
                                                        .into(),
                                                ])
                                                .build(),
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
                .build(),
        );

        let namespace = XmlNamespace::new_dangerous("http://example.com");

        let mut compiled_namespace = CompiledNamespace::new(namespace.clone());

        let sequence = compiled_namespace
            .import_top_level_element(&sequence)
            .unwrap()
            .into_owned();

        let mut context = XmlnsContext::new();
        context.add_namespace(compiled_namespace);

        let mut generator = Generator::new(&context);

        generator.bind_types(crate::binds::StdXsdTypes);

        let (type_, actual_items) = generator.generate_top_level_element(&sequence).unwrap();

        let actual = prettyplease::unparse(&syn::File {
            shebang: None,
            attrs: Vec::new(),
            items: actual_items.clone(),
        });

        #[rustfmt::skip]
        let expected: syn::File = parse_quote!(
            #[derive(::core::fmt::Debug, ::xmlity::Serialize, ::xmlity::Deserialize)]
            #[xelement(name = "SimpleSequence", namespace = "http://example.com", allow_unknown_attributes = "any", children_order = "strict")]
            pub struct SimpleSequence {
                #[xattribute(name = "a")]
                pub a: i32,
                #[xattribute(name = "b")]
                pub b: String,
            }
        );

        let expected = prettyplease::unparse(&expected);

        assert_eq!(actual, expected);

        assert_eq!(type_.into_type(None), parse_quote!(SimpleSequence));
    }

    #[test]
    fn two_sequence_deep_element() {
        let sequence = xs::types::TopLevelElement::builder()
            .name(LocalName::new_dangerous("SimpleSequence"))
            .child_1(
                xs::LocalComplexType::builder()
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
                    .build()
                    .into(),
            )
            .build()
            .into();

        let namespace = XmlNamespace::new_dangerous("http://example.com");

        let mut compiled_namespace = CompiledNamespace::new(namespace.clone());

        let sequence = compiled_namespace
            .import_top_level_element(&sequence)
            .unwrap()
            .into_owned();

        let mut context = XmlnsContext::new();
        context.add_namespace(compiled_namespace);

        let mut generator = Generator::new(&context);

        generator.bind_types(crate::binds::StdXsdTypes);

        let (type_, actual_items) = generator.generate_top_level_element(&sequence).unwrap();

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
            
            #[derive(::core::fmt::Debug, ::xmlity::Serialize, ::xmlity::Deserialize)]
            #[xelement(name = "SimpleSequence", namespace = "http://example.com", allow_unknown_attributes = "any", children_order = "strict")]
            pub struct SimpleSequence {
                pub child_0: simple_sequence_items::Child0,
                #[xelement(name = "c", namespace = "http://example.com")]
                pub c: String,

            }
        );

        let expected = prettyplease::unparse(&expected);

        assert_eq!(actual, expected);

        assert_eq!(type_.into_type(None), parse_quote!(SimpleSequence));
    }

    #[test]
    fn two_attribute_two_children_sequence_element() {
        let sequence = xs::types::TopLevelElement::builder()
            .name(LocalName::new_dangerous("SimpleSequence"))
            .child_1(
                xs::LocalComplexType::builder()
                    .content(
                        xs::ComplexContent::builder()
                            .content(
                                xs::ComplexRestrictionType::builder()
                                    .base(xs::QName(xsn::ANY_TYPE.clone()))
                                    .particle(
                                        xs::SequenceType::builder()
                                            .content(vec![
                                                xs::LocalElement::builder()
                                                    .name(LocalName::new_dangerous("a"))
                                                    .type_(xs::QName(xsn::INTEGER.clone()))
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
                                    )
                                    .attr_decls(
                                        xs::AttrDecls::builder()
                                            .declarations(vec![
                                                xs::LocalAttribute::builder()
                                                    .name(LocalName::new_dangerous("c"))
                                                    .type_(xs::QName(xsn::INTEGER.clone()))
                                                    .use_(xs::AttributeUseType::Required)
                                                    .build()
                                                    .into(),
                                                xs::LocalAttribute::builder()
                                                    .name(LocalName::new_dangerous("d"))
                                                    .type_(xs::QName(xsn::STRING.clone()))
                                                    .use_(xs::AttributeUseType::Required)
                                                    .build()
                                                    .into(),
                                            ])
                                            .build(),
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
            .build()
            .into();

        let namespace = XmlNamespace::new_dangerous("http://example.com");

        let mut compiled_namespace = CompiledNamespace::new(namespace.clone());

        let sequence = compiled_namespace
            .import_top_level_element(&sequence)
            .unwrap()
            .into_owned();

        let mut context = XmlnsContext::new();
        context.add_namespace(compiled_namespace);

        let mut generator = Generator::new(&context);

        generator.bind_types(crate::binds::StdXsdTypes);

        let (type_, actual_items) = generator.generate_top_level_element(&sequence).unwrap();

        let actual = prettyplease::unparse(&syn::File {
            shebang: None,
            attrs: Vec::new(),
            items: actual_items.clone(),
        });

        #[rustfmt::skip]
        let expected: syn::File = parse_quote!(
            #[derive(::core::fmt::Debug, ::xmlity::Serialize, ::xmlity::Deserialize)]
            #[xelement(name = "SimpleSequence", namespace = "http://example.com", allow_unknown_attributes = "any", children_order = "strict")]
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
        );

        let expected = prettyplease::unparse(&expected);

        assert_eq!(actual, expected);

        assert_eq!(type_.into_type(None), parse_quote!(SimpleSequence));
    }

    #[test]
    fn complex_reference_type_local_element() {
        let child_type_expanded_name = ExpandedName::new(
            LocalName::new_dangerous("childType"),
            XmlNamespace::XS.into(),
        );

        let sequence = xs::types::TopLevelElement::builder()
            .name(LocalName::new_dangerous("SimpleSequence"))
            .child_1(
                xs::LocalComplexType::builder()
                    .content(
                        xs::ComplexContent::builder()
                            .content(
                                xs::ComplexRestrictionType::builder()
                                    .base(xs::QName(xsn::ANY_TYPE.clone()))
                                    .particle(
                                        xs::SequenceType::builder()
                                            .content(vec![xs::LocalElement::builder()
                                                .name(LocalName::new_dangerous("a"))
                                                .type_(xs::QName(child_type_expanded_name.clone()))
                                                .build()
                                                .into()])
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
            .build()
            .into();

        let namespace = XmlNamespace::new_dangerous("http://example.com");

        let mut compiled_namespace = CompiledNamespace::new(namespace.clone());

        let sequence = compiled_namespace
            .import_top_level_element(&sequence)
            .unwrap()
            .into_owned();

        let mut context = XmlnsContext::new();
        context.add_namespace(compiled_namespace);

        let mut generator = Generator::new(&context);

        generator.bind_type(
            child_type_expanded_name,
            BoundType {
                ty: TypeReference::new_static(parse_quote!(types::ChildType)),
                ty_type: TypeType::Complex,
                serialize_with: None,
                deserialize_with: None,
            },
        );

        let (type_, actual_items) = generator.generate_top_level_element(&sequence).unwrap();

        let actual = prettyplease::unparse(&syn::File {
            shebang: None,
            attrs: Vec::new(),
            items: actual_items,
        });

        #[rustfmt::skip]
        let expected: syn::File = parse_quote!(
            #[derive(::core::fmt::Debug, ::xmlity::Serialize, ::xmlity::Deserialize)]
            #[xelement(name = "SimpleSequence", namespace = "http://example.com", allow_unknown_attributes = "any", children_order = "strict")]
            pub struct SimpleSequence {
                #[xelement(name = "a", namespace = "http://example.com", group)]
                pub a: types::ChildType,
            }
        );

        let expected = prettyplease::unparse(&expected);

        assert_eq!(actual, expected);
        assert_eq!(type_.into_type(None), parse_quote!(SimpleSequence));
    }

    #[test]
    fn simple_reference_type_top_level_element() {
        let sequence = xs::types::TopLevelElement::builder()
            .name(LocalName::new_dangerous("SimpleSequence"))
            .type_(xs::QName(xsn::STRING.clone()))
            .build()
            .into();

        let namespace = XmlNamespace::new_dangerous("http://example.com");

        let mut compiled_namespace = CompiledNamespace::new(namespace.clone());

        let sequence = compiled_namespace
            .import_top_level_element(&sequence)
            .unwrap()
            .into_owned();

        let mut context = XmlnsContext::new();
        context.add_namespace(compiled_namespace);

        let mut generator = Generator::new(&context);

        generator.bind_types(crate::binds::StdXsdTypes);

        let (type_, actual_items) = generator.generate_top_level_element(&sequence).unwrap();

        let actual = prettyplease::unparse(&syn::File {
            shebang: None,
            attrs: Vec::new(),
            items: actual_items.clone(),
        });

        #[rustfmt::skip]
        let expected: syn::File = parse_quote!(
            #[derive(::core::fmt::Debug, ::xmlity::Serialize, ::xmlity::Deserialize)]
            #[xelement(name = "SimpleSequence", namespace = "http://example.com", allow_unknown_attributes = "any")]
            pub struct SimpleSequence(pub String);
        );

        let expected = prettyplease::unparse(&expected);

        assert_eq!(actual, expected);

        assert_eq!(type_.into_type(None), parse_quote!(SimpleSequence));
    }

    #[test]
    fn complex_reference_type_top_level_element() {
        let child_type_expanded_name = ExpandedName::new(
            LocalName::new_dangerous("childType"),
            XmlNamespace::XS.into(),
        );

        let sequence = xs::types::TopLevelElement::builder()
            .name(LocalName::new_dangerous("SimpleSequence"))
            .type_(xs::QName(child_type_expanded_name.clone()))
            .build()
            .into();

        let namespace = XmlNamespace::new_dangerous("http://example.com");

        let mut compiled_namespace = CompiledNamespace::new(namespace.clone());

        let sequence = compiled_namespace
            .import_top_level_element(&sequence)
            .unwrap()
            .into_owned();

        let mut context = XmlnsContext::new();
        context.add_namespace(compiled_namespace);

        let mut generator = Generator::new(&context);

        generator.bind_type(
            child_type_expanded_name,
            BoundType {
                ty: TypeReference::new_static(parse_quote!(types::ChildType)),
                ty_type: TypeType::Complex,
                serialize_with: None,
                deserialize_with: None,
            },
        );

        let (type_, actual_items) = generator.generate_top_level_element(&sequence).unwrap();

        let actual = prettyplease::unparse(&syn::File {
            shebang: None,
            attrs: Vec::new(),
            items: actual_items.clone(),
        });

        #[rustfmt::skip]
        let expected: syn::File = parse_quote!(
            #[derive(::core::fmt::Debug, ::xmlity::Serialize, ::xmlity::Deserialize)]
            #[xelement(name = "SimpleSequence", namespace = "http://example.com", allow_unknown_attributes = "any")]
            pub struct SimpleSequence(#[xgroup] pub types::ChildType);
        );

        let expected = prettyplease::unparse(&expected);

        assert_eq!(actual, expected);

        assert_eq!(type_.into_type(None), parse_quote!(SimpleSequence));
    }

    #[test]
    fn element_ref_element() {
        let child_element_expanded_name = ExpandedName::new(
            LocalName::new_dangerous("ChildElement"),
            XmlNamespace::XS.into(),
        );

        let sequence = xs::types::TopLevelElement::builder()
            .name(LocalName::new_dangerous("SimpleSequence"))
            .child_1(
                xs::LocalComplexType::builder()
                    .content(
                        xs::ComplexContent::builder()
                            .content(
                                xs::ComplexRestrictionType::builder()
                                    .base(xs::QName(xsn::ANY_TYPE.clone()))
                                    .particle(
                                        xs::SequenceType::builder()
                                            .content(vec![xs::LocalElement::builder()
                                                .ref_(xs::QName(
                                                    child_element_expanded_name.clone(),
                                                ))
                                                .build()
                                                .into()])
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
            .build()
            .into();

        let namespace = XmlNamespace::new_dangerous("http://example.com");

        let mut compiled_namespace = CompiledNamespace::new(namespace.clone());

        let sequence = compiled_namespace
            .import_top_level_element(&sequence)
            .unwrap()
            .into_owned();

        let mut context = XmlnsContext::new();
        context.add_namespace(compiled_namespace);

        let mut generator = Generator::new(&context);

        generator.bind_element(
            child_element_expanded_name,
            TypeReference::new_static(parse_quote!(types::ChildElement)),
        );

        let (type_, actual_items) = generator.generate_top_level_element(&sequence).unwrap();

        let actual = prettyplease::unparse(&syn::File {
            shebang: None,
            attrs: Vec::new(),
            items: actual_items.clone(),
        });

        #[rustfmt::skip]
        let expected: syn::File = parse_quote!(
            #[derive(::core::fmt::Debug, ::xmlity::Serialize, ::xmlity::Deserialize)]
            #[xelement(name = "SimpleSequence", namespace = "http://example.com", allow_unknown_attributes = "any", children_order = "strict")]
            pub struct SimpleSequence {
                pub child_element: types::ChildElement,
            }
        );

        let expected = prettyplease::unparse(&expected);

        assert_eq!(actual, expected);

        assert_eq!(type_.into_type(None), parse_quote!(SimpleSequence));
    }
}
