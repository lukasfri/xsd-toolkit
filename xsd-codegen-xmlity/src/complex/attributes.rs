use std::any::type_name;
use std::sync::Weak;

use super::{ComplexContext, ComplexToTypeTemplate, Scope, ToTypeTemplateData};
use crate::complex::ComplexToTypeTemplateExt;
use crate::{
    misc::WeakExt,
    simple::{simple_type::SimpleTypeRootHandler, SimpleContext, SimpleToTypeTemplate},
    templates::element_record::ElementFieldAttribute,
    Result, ToIdentTypesExt,
};
use quote::format_ident;
use syn::parse_quote;
use xmlity::ExpandedName;
use xsd_fragments::fragments::complex::{self as cx, AttributeUse};

#[derive(Debug)]
pub struct LocalAttributeHandler {
    pub simple_type_handler: Weak<SimpleTypeRootHandler>,
}

impl ComplexToTypeTemplate<cx::LocalAttributeFragment> for LocalAttributeHandler {
    type TypeTemplate = ElementFieldAttribute;

    fn to_type_template<C: ComplexContext, S: Scope>(
        &self,
        context: &C,
        scope: &mut S,
        item: &cx::LocalAttributeFragment,
    ) -> Result<ToTypeTemplateData<Self::TypeTemplate>> {
        let optional = match item.use_.unwrap_or_default() {
            AttributeUse::Optional => true,
            AttributeUse::Required => false,
            AttributeUse::Prohibited => panic!("prohibited attributes are not supported"),
        };

        let (ident, template) = match &item.type_mode {
            cx::LocalAttributeFragmentTypeMode::Declared(local) => {
                let name = ExpandedName::new(local.name.clone(), None);
                let ident = local.name.to_item_ident();

                let simple_context = context
                    .simple_context()
                    .sub_context(format_ident!("{}Value", ident));

                let ty = self
                    .simple_type_handler
                    .upgrade_or_else(|| crate::Error::HandlerDoesNotExist {
                        origin: type_name::<Self>(),
                        handler: type_name::<SimpleTypeRootHandler>(),
                    })?
                    .to_type_template(&simple_context, scope, &local.type_)?
                    .template;

                let ty = ty.wrap_if(optional, |a| parse_quote!(::core::option::Option<#a>));

                let template = ElementFieldAttribute {
                    name: Some(name),
                    ty,
                    deferred: false,
                    optional,
                    default: false,
                };

                (Some(ident), template)
            }
            cx::LocalAttributeFragmentTypeMode::Reference(reference) => {
                let ident = reference.ref_.local_name().to_item_ident();
                let ty = context.resolve_named_attribute(&reference.ref_)?;

                let ty = ty.wrap_if(optional, |a| parse_quote!(::core::option::Option<#a>));

                let template = ElementFieldAttribute {
                    name: None,
                    ty,
                    deferred: true,
                    optional,
                    default: false,
                };

                (Some(ident), template)
            }
        };

        Ok(ToTypeTemplateData { ident, template })
    }
}

#[derive(Debug)]
pub struct AttributeDeclarationHandler {
    pub local_attribute_handler: Weak<LocalAttributeHandler>,
}

impl ComplexToTypeTemplate<cx::AttributeDeclarationId> for AttributeDeclarationHandler {
    type TypeTemplate = ElementFieldAttribute;

    fn to_type_template<C: ComplexContext, S: Scope>(
        &self,
        context: &C,
        scope: &mut S,
        item: &cx::AttributeDeclarationId,
    ) -> Result<ToTypeTemplateData<Self::TypeTemplate>> {
        match item {
            cx::AttributeDeclarationId::Attribute(fragment_idx) => self
                .local_attribute_handler
                .upgrade_or_else(|| crate::Error::HandlerDoesNotExist {
                    origin: type_name::<Self>(),
                    handler: type_name::<LocalAttributeHandler>(),
                })?
                .resolve_type_template(context, scope, fragment_idx),
            cx::AttributeDeclarationId::AttributeGroupRef(_fragment_idx) => {
                Err(crate::Error::UnsupportedFragment {
                    fragment: "AttributeGroupRef".to_string(),
                })
            }
        }
    }
}

#[derive(Debug)]
pub struct TopLevelAttributeHandler;

impl ComplexToTypeTemplate<cx::TopLevelAttributeFragment> for TopLevelAttributeHandler {
    type TypeTemplate = ElementFieldAttribute;

    fn to_type_template<C: ComplexContext, S: Scope>(
        &self,
        context: &C,
        scope: &mut S,
        item: &cx::TopLevelAttributeFragment,
    ) -> Result<ToTypeTemplateData<Self::TypeTemplate>> {
        let name = ExpandedName::new(
            item.name.clone(),
            Some(context.namespace().clone().into_owned()),
        );
        let ident = item.name.to_item_ident();

        let ty = SimpleTypeRootHandler
            .to_type_template(context.simple_context(), scope, &item.type_)?
            .template;

        let template = ElementFieldAttribute {
            name: Some(name),
            ty,
            deferred: false,
            default: false,
            optional: false,
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
    use xmlity::{LocalName, XmlNamespace};
    use xsd::{ns, xs, xsn};
    use xsd_fragments::XmlnsContext;

    use crate::Generator;

    #[test]
    fn simple_attribute() {
        const TEST_NAMESPACE: XmlNamespace<'static> =
            XmlNamespace::new_dangerous("http://example.com");

        let attribute = xs::types::TopLevelAttribute::builder()
            .name(LocalName::new_dangerous("SimpleAttribute"))
            .type_(xs::types::QName(xsn::STRING.clone()))
            .any_attributes(ns::AnyAttributes::default())
            .build()
            .into();

        let mut ctx = XmlnsContext::new();
        let ns = ctx.init_namespace(TEST_NAMESPACE);

        let sequence = ns
            .import_top_level_attribute(&attribute)
            .unwrap()
            .into_owned();

        let mut generator = Generator::new(&ctx);

        generator.bind_types(crate::binds::StdXsdTypes);

        let (type_, actual_items) = generator.generate_attribute(&sequence).unwrap();

        #[rustfmt::skip]
        let expected_items: Vec<Item> = vec![
            parse_quote!(
                #[derive(::core::fmt::Debug, ::xmlity::SerializeAttribute, ::xmlity::Deserialize)]
                #[xattribute(name = "SimpleAttribute", namespace = "http://example.com")]
                pub struct SimpleAttribute(pub String);
            )
        ];

        assert_eq!(expected_items, actual_items);

        assert_eq!(type_.into_type(None), parse_quote!(SimpleAttribute));
    }
}
