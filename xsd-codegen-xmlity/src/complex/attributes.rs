use crate::{
    templates::{
        self,
        element_record::{ElementField, ElementFieldAttribute},
        group_record::GroupRecord,
        FieldType,
    },
    Result,
};

use quote::format_ident;
use syn::parse_quote;
use xmlity::ExpandedName;
use xsd_type_compiler::complex::{self as cx};

use super::{Context, ToTypeTemplate, ToTypeTemplateData, TypeDefParticleTemplate};

impl ToTypeTemplate for cx::LocalAttributeFragment {
    type TypeTemplate = ElementFieldAttribute;

    fn to_type_template<C: Context>(
        &self,
        context: &C,
    ) -> Result<ToTypeTemplateData<Self::TypeTemplate>> {
        match self {
            cx::LocalAttributeFragment::Declared(local) => {
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
    type TypeTemplate = ElementFieldAttribute;

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

impl ToTypeTemplate for cx::TopLevelAttributeFragment {
    type TypeTemplate = ElementFieldAttribute;

    fn to_type_template<C: Context>(
        &self,
        context: &C,
    ) -> Result<ToTypeTemplateData<Self::TypeTemplate>> {
        let name = ExpandedName::new(
            self.name.clone(),
            Some(context.namespace().clone().into_owned()),
        );
        let ident = format_ident!("{}", self.name.to_string());

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
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use syn::{parse_quote, Item};
    use xmlity::{ExpandedName, LocalName, XmlNamespace};
    use xsd::schema as xs;
    use xsd_type_compiler::{CompiledNamespace, XmlnsContext};

    use crate::Generator;

    #[test]
    fn simple_attribute() {
        let string_expanded_name = ExpandedName::new(
            LocalName::new_dangerous("string"),
            XmlNamespace::XMLNS.into(),
        );

        let attribute = xs::TopLevelAttribute::builder()
            .name(xs::Name(LocalName::new_dangerous("SimpleAttribute")))
            .type_(xs::Type(xs::QName(string_expanded_name.clone())))
            .build();

        let namespace = XmlNamespace::new_dangerous("http://example.com");
        let namespace_lit = namespace.as_str();

        let mut compiled_namespace = CompiledNamespace::new(namespace.clone());

        let sequence = compiled_namespace
            .import_top_level_attribute(&attribute)
            .unwrap()
            .into_owned();

        let mut context = XmlnsContext::new();
        context.add_namespace(compiled_namespace);

        let generator = Generator::new(&context);

        let (type_, actual_items) = generator.generate_top_level_attribute(&sequence).unwrap();

        #[rustfmt::skip]
        let expected_items: Vec<Item> = vec![
            parse_quote!(
                #[derive(::core::fmt::Debug, ::xmlity::SerializeAttribute, ::xmlity::Deserialize)]
                #[xattribute(name = "SimpleAttribute", namespace = #namespace_lit)]
                pub struct SimpleAttribute(pub String);
            )
        ];

        assert_eq!(expected_items, actual_items);

        assert_eq!(type_.into_type(None), parse_quote!(SimpleAttribute));
    }
}
