use crate::{
    simple::SimpleContext, templates::element_record::ElementFieldAttribute, Result,
    ToIdentTypesExt,
};
use quote::format_ident;
use syn::parse_quote;
use xmlity::ExpandedName;
use xsd_fragments::fragments::complex::{self as cx, AttributeUse};

use super::{ComplexContext, ComplexToTypeTemplate, Scope, ToTypeTemplateData};

impl ComplexToTypeTemplate for cx::LocalAttributeFragment {
    type TypeTemplate = ElementFieldAttribute;

    fn to_type_template<C: ComplexContext, S: Scope>(
        &self,
        context: &C,
        scope: &mut S,
    ) -> Result<ToTypeTemplateData<Self::TypeTemplate>> {
        let optional = match self.use_.unwrap_or_default() {
            AttributeUse::Optional => true,
            AttributeUse::Required => false,
            AttributeUse::Prohibited => panic!("prohibited attributes are not supported"),
        };

        let (ident, template) = match &self.type_mode {
            cx::LocalAttributeFragmentTypeMode::Declared(local) => {
                let name = ExpandedName::new(local.name.clone(), None);
                let ident = local.name.to_item_ident();

                let ty = context
                    .simple_context()
                    .sub_context(format_ident!("{}Value", ident))
                    .resolve_fragment(&local.type_, scope)?
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
                let ident = reference.name.local_name().to_item_ident();
                let ty = context.resolve_named_attribute(&reference.name)?;

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

impl ComplexToTypeTemplate for cx::AttributeDeclarationId {
    type TypeTemplate = ElementFieldAttribute;

    fn to_type_template<C: ComplexContext, S: Scope>(
        &self,
        context: &C,
        scope: &mut S,
    ) -> Result<ToTypeTemplateData<Self::TypeTemplate>> {
        match self {
            cx::AttributeDeclarationId::Attribute(fragment_idx) => {
                context.resolve_fragment_id(fragment_idx, scope)
            }
            cx::AttributeDeclarationId::AttributeGroupRef(_fragment_idx) => {
                Err(crate::Error::UnsupportedFragment {
                    fragment: "AttributeGroupRef".to_string(),
                })
            }
        }
    }
}

impl ComplexToTypeTemplate for cx::TopLevelAttributeFragment {
    type TypeTemplate = ElementFieldAttribute;

    fn to_type_template<C: ComplexContext, S: Scope>(
        &self,
        context: &C,
        scope: &mut S,
    ) -> Result<ToTypeTemplateData<Self::TypeTemplate>> {
        let name = ExpandedName::new(
            self.name.clone(),
            Some(context.namespace().clone().into_owned()),
        );
        let ident = self.name.to_item_ident();

        let ty = context
            .simple_context()
            .resolve_fragment(&self.type_, scope)?
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
    use xsd::{xs, xsn};
    use xsd_fragments::XmlnsContext;

    use crate::Generator;

    #[test]
    fn simple_attribute() {
        const TEST_NAMESPACE: XmlNamespace<'static> =
            XmlNamespace::new_dangerous("http://example.com");

        let attribute = xs::types::TopLevelAttribute::builder()
            .name(LocalName::new_dangerous("SimpleAttribute"))
            .type_(xs::types::QName(xsn::STRING.clone()))
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
