use crate::{
    misc::TypeReference,
    simple as simple_gen,
    templates::{
        self,
        choice::Choice,
        element_record::{ElementField, ElementFieldAttribute},
        group_record::GroupRecord,
        value_record::{ItemField, ItemFieldItem, ItemRecord},
        FieldType, ItemOrder,
    },
    Result,
};

use std::collections::HashMap;

use quote::format_ident;
use syn::{parse_quote, Ident, Item};
use xmlity::{ExpandedName, LocalName, XmlNamespace};
use xsd::schema::MaxOccursValue;
use xsd_type_compiler::complex::{
    self as cx, ComplexTypeFragmentCompiler, FragmentAccess, FragmentIdx,
};

pub struct ComplexTypeFragmentGenerator<'a> {
    pub context: &'a xsd_type_compiler::complex::ComplexTypeFragmentCompiler,
    pub bound_namespaces: &'a HashMap<XmlNamespace<'static>, Ident>,
    pub simple_generator: &'a simple_gen::SimpleGeneratedNamespaceTypes,
}

impl ComplexTypeFragmentGenerator<'_> {
    pub fn generate(&self) -> Result<Vec<Item>> {
        Ok(vec![])
    }

    pub fn get_named_type(&self, name: &ExpandedName<'_>) -> Result<Option<syn::TypePath>> {
        todo!()
    }
}

pub struct FragmentResolveInfo {
    pub possible_ident: String,
}

pub struct ResolveFragmentData<T> {
    pub ident: Option<Ident>,
    pub type_: T,
}

pub trait Context: crate::simple::Context {
    fn resolve_fragment<F>(&self, fragment: &F) -> Result<ToTypeTemplateData<F::TypeTemplate>>
    where
        F: ToTypeTemplate;

    fn resolve_fragment_id<F>(
        &self,
        fragment_id: &FragmentIdx<F>,
    ) -> Result<ToTypeTemplateData<F::TypeTemplate>>
    where
        F: ToTypeTemplate,
        ComplexTypeFragmentCompiler: FragmentAccess<F>;

    fn resolve_named_type(&self, name: &ExpandedName<'_>) -> Result<syn::Type>;

    fn to_expanded_name(&self, name: LocalName<'static>) -> ExpandedName<'static>;
}

pub struct ToTypeTemplateData<T> {
    pub ident: Option<Ident>,
    pub template: T,
}

pub trait ToTypeTemplate {
    type TypeTemplate;

    fn to_type_template<C: Context>(
        &self,
        context: &C,
    ) -> Result<ToTypeTemplateData<Self::TypeTemplate>>;
}

impl ToTypeTemplate for cx::ElementTypeContentId {
    type TypeTemplate = templates::group_record::GroupRecord;

    fn to_type_template<C: Context>(
        &self,
        context: &C,
    ) -> Result<ToTypeTemplateData<Self::TypeTemplate>> {
        todo!()
    }
}

impl ToTypeTemplate for cx::LocalElementTypeFragment {
    type TypeTemplate = templates::element_record::ElementRecord;

    fn to_type_template<C: Context>(
        &self,
        context: &C,
    ) -> Result<ToTypeTemplateData<Self::TypeTemplate>> {
        let name = context.to_expanded_name(self.name.clone());
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
                    ident: None,
                    template,
                })
            }
            xsd_type_compiler::NamedOrAnonymous::Anonymous(anonymous) => {
                let sub_type = context.resolve_fragment(anonymous)?;

                let template = sub_type.template.into_element_record(name);

                Ok(ToTypeTemplateData {
                    ident: None,
                    template,
                })
            }
        }
    }
}

impl ToTypeTemplate for cx::ReferenceElementTypeFragment {
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

impl ToTypeTemplate for cx::ElementTypeFragment {
    type TypeTemplate = ItemField;

    fn to_type_template<C: Context>(
        &self,
        context: &C,
    ) -> Result<ToTypeTemplateData<Self::TypeTemplate>> {
        match self {
            cx::ElementTypeFragment::Local(local) => {
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
            cx::ElementTypeFragment::Reference(reference) => {
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

impl ToTypeTemplate for cx::GroupTypeContentId {
    type TypeTemplate = templates::value_record::ItemField;

    fn to_type_template<C: Context>(
        &self,
        context: &C,
    ) -> Result<ToTypeTemplateData<Self::TypeTemplate>> {
        match self {
            cx::GroupTypeContentId::Element(fragment_idx) => {
                let fragment: &cx::ElementTypeFragment = todo!();

                // let template = context.resolve_fragment(fragment)?;
                todo!()
            }
            cx::GroupTypeContentId::Group(fragment_idx) => todo!(),
            cx::GroupTypeContentId::All(fragment_idx) => todo!(),
            cx::GroupTypeContentId::Choice(fragment_idx) => todo!(),
            cx::GroupTypeContentId::Sequence(fragment_idx) => todo!(),
            cx::GroupTypeContentId::Any(fragment_idx) => todo!(),
        }
    }
}

impl ToTypeTemplate for cx::SequenceFragment {
    type TypeTemplate = templates::value_record::ItemRecord;

    fn to_type_template<C: Context>(
        &self,
        context: &C,
    ) -> Result<ToTypeTemplateData<Self::TypeTemplate>> {
        // Struct with strict order
        let fields = self
            .fragments
            .iter()
            .enumerate()
            .map(|(i, fragment_id)| {
                let res = context.resolve_fragment::<_>(fragment_id)?;

                Ok((
                    Some(res.ident.unwrap_or_else(|| format_ident!("field_{}", i))),
                    res.template,
                ))
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

fn min_max_occurs_type(
    min_occurs: usize,
    max_occurs: MaxOccursValue,
    type_: TypeReference<'_>,
) -> TypeReference<'_> {
    match (min_occurs, max_occurs) {
        (1, MaxOccursValue::Bounded(1)) => type_,
        (0, MaxOccursValue::Bounded(1)) => TypeReference::new(|path| {
            let type_ = type_.into_type(path);
            parse_quote!(Option<#type_>)
        }),
        (_, _) => TypeReference::new(|path| {
            let type_ = type_.into_type(path);
            parse_quote!(Vec<#type_>)
        }),
    }
}

impl ToTypeTemplate for cx::LocalAttributeFragment {
    type TypeTemplate = templates::element_record::ElementFieldAttribute;

    fn to_type_template<C: Context>(
        &self,
        context: &C,
    ) -> Result<ToTypeTemplateData<Self::TypeTemplate>> {
        match self {
            cx::LocalAttributeFragment::Local(local) => {
                let name = ExpandedName::new(local.name.clone(), None);
                let ident = format_ident!("{}", local.name.to_string());

                //TODO: handle attribute type
                let ty = parse_quote!(String);

                let template = ElementFieldAttribute {
                    name: Some(name),
                    ty,
                    deferred: false,
                };

                Ok(ToTypeTemplateData {
                    ident: Some(ident),
                    template,
                })
            }
            cx::LocalAttributeFragment::Reference(reference) => {
                let ident = format_ident!("todo");

                let ty = context.resolve_named_type(&reference.name)?;

                let template = ElementFieldAttribute {
                    name: None,
                    ty,
                    deferred: true,
                };

                Ok(ToTypeTemplateData {
                    ident: Some(ident),
                    template,
                })
            }
        }
    }
}

impl ToTypeTemplate for cx::AttributeDeclarationId {
    type TypeTemplate = templates::element_record::ElementFieldAttribute;

    fn to_type_template<C: Context>(
        &self,
        context: &C,
    ) -> Result<ToTypeTemplateData<Self::TypeTemplate>> {
        match self {
            cx::AttributeDeclarationId::Attribute(fragment_idx) => {
                context.resolve_fragment_id(fragment_idx)
            }
            cx::AttributeDeclarationId::AttributeGroupRef(fragment_idx) => todo!(),
        }
    }
}

pub enum TypeDefParticleTemplate {
    Record(ItemRecord),
    Choice(Choice),
}

impl ToTypeTemplate for cx::TypeDefParticleId {
    type TypeTemplate = TypeDefParticleTemplate;

    fn to_type_template<C: Context>(
        &self,
        context: &C,
    ) -> Result<ToTypeTemplateData<Self::TypeTemplate>> {
        match self {
            cx::TypeDefParticleId::Group(fragment_idx) => todo!(),
            cx::TypeDefParticleId::All(fragment_idx) => todo!(),
            cx::TypeDefParticleId::Sequence(fragment_idx) => {
                let sequence = context.resolve_fragment_id(fragment_idx)?;

                Ok(ToTypeTemplateData {
                    ident: None,
                    template: TypeDefParticleTemplate::Record(sequence.template),
                })
            }
            cx::TypeDefParticleId::Choice(fragment_idx) => todo!(),
        }
    }
}

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
                    TypeDefParticleTemplate::Choice(choice) => todo!(),
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
            cx::ComplexContentChildId::Extension(fragment_idx) => {
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
            cx::ComplexTypeModelId::SimpleContent(fragment_idx) => todo!(),
            cx::ComplexTypeModelId::ComplexContent(fragment_idx) => {
                context.resolve_fragment_id(fragment_idx)
            }
            cx::ComplexTypeModelId::Other { particle } => todo!(),
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
                                    .build()
                                    .into(),
                                xs::LocalAttribute::builder()
                                    .name(xs::Name(LocalName::new_dangerous("b")))
                                    .type_(xs::Type(xs::QName(string_expanded_name.clone())))
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

        let generator = Generator::new(&context);

        let (type_, actual_items) = generator.generate_top_level_type(&sequence).unwrap();

        #[rustfmt::skip]
        let expected_items: Vec<Item> = vec![
            parse_quote!(
                #[derive(::core::fmt::Debug, ::xmlity::SerializationGroup, ::xmlity::DeserializationGroup)]
                #[xgroup(children_order = "strict")]
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
