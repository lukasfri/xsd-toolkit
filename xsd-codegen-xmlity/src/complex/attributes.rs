use crate::{
    misc::TypeReference, templates::element_record::ElementFieldAttribute, Result, ToIdentTypesExt,
    TypeType,
};

use quote::ToTokens;
use syn::parse_quote;
use xmlity::ExpandedName;
use xsd_type_compiler::{
    complex::{self as cx, AttributeUse},
    NamedOrAnonymous,
};

use super::{Context, Scope, ToTypeTemplate, ToTypeTemplateData};

impl ToTypeTemplate for cx::LocalAttributeFragment {
    type TypeTemplate = ElementFieldAttribute;

    fn to_type_template<C: Context, S: Scope>(
        &self,
        context: &C,
        _scope: &mut S,
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

                let ty = match local.type_.as_ref() {
                    Some(NamedOrAnonymous::Named(name)) => {
                        let bound_type = context.resolve_named_type(name)?;
                        assert_eq!(
                            bound_type.ty_type,
                            TypeType::Simple,
                            "{} is not a simple type, but is used as an attribute value",
                            bound_type.ty.to_type(None).to_token_stream()
                        );

                        bound_type.ty.clone()
                    }
                    Some(NamedOrAnonymous::Anonymous(_)) => {
                        //TODO
                        TypeReference::new_static(parse_quote!(String))
                    }
                    None => TypeReference::new_static(parse_quote!(())),
                };

                let ty = ty.wrap_if(optional, |a| parse_quote!(::core::option::Option<#a>));

                let template = ElementFieldAttribute {
                    name: Some(name),
                    ty,
                    deferred: false,
                    optional,
                    default: optional,
                };

                (Some(ident), template)
            }
            cx::LocalAttributeFragmentTypeMode::Reference(reference) => {
                let ty = context.resolve_named_attribute(&reference.name)?;

                let ty = ty.wrap_if(optional, |a| parse_quote!(::core::option::Option<#a>));

                let template = ElementFieldAttribute {
                    name: None,
                    ty,
                    deferred: true,
                    optional,
                    default: optional,
                };

                (None, template)
            }
        };

        Ok(ToTypeTemplateData { ident, template })
    }
}

impl ToTypeTemplate for cx::AttributeDeclarationId {
    type TypeTemplate = ElementFieldAttribute;

    fn to_type_template<C: Context, S: Scope>(
        &self,
        context: &C,
        scope: &mut S,
    ) -> Result<ToTypeTemplateData<Self::TypeTemplate>> {
        match self {
            cx::AttributeDeclarationId::Attribute(fragment_idx) => {
                context.resolve_fragment_id(fragment_idx, scope)
            }
            cx::AttributeDeclarationId::AttributeGroupRef(_fragment_idx) => todo!(),
        }
    }
}

impl ToTypeTemplate for cx::TopLevelAttributeFragment {
    type TypeTemplate = ElementFieldAttribute;

    fn to_type_template<C: Context, S: Scope>(
        &self,
        context: &C,
        _scope: &mut S,
    ) -> Result<ToTypeTemplateData<Self::TypeTemplate>> {
        let name = ExpandedName::new(
            self.name.clone(),
            Some(context.namespace().clone().into_owned()),
        );
        let ident = self.name.to_item_ident();

        let ty = match self.type_.as_ref() {
            Some(NamedOrAnonymous::Named(name)) => {
                let bound_type = context.resolve_named_type(name)?;

                assert_eq!(
                    bound_type.ty_type,
                    TypeType::Simple,
                    "{} is not a simple type, but is used as an attribute value",
                    bound_type.ty.to_type(None).to_token_stream()
                );

                bound_type.ty.clone()
            }
            Some(NamedOrAnonymous::Anonymous(_)) => todo!(),
            None => TypeReference::new_static(parse_quote!(())),
        };

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
    use xmlity::{ExpandedName, LocalName, XmlNamespace};
    use xsd::schema as xs;
    use xsd_type_compiler::{CompiledNamespace, XmlnsContext};

    use crate::Generator;

    #[test]
    fn simple_attribute() {
        let attribute = xs::TopLevelAttribute::builder()
            .name(xs::NameAttr(LocalName::new_dangerous("SimpleAttribute")))
            .type_(xs::Type(xs::QName(ExpandedName::new(
                LocalName::new_dangerous("string"),
                XmlNamespace::XS.into(),
            ))))
            .build();

        let namespace = XmlNamespace::new_dangerous("http://example.com");

        let mut compiled_namespace = CompiledNamespace::new(namespace.clone());

        let sequence = compiled_namespace
            .import_top_level_attribute(&attribute)
            .unwrap()
            .into_owned();

        let mut context = XmlnsContext::new();
        context.add_namespace(compiled_namespace);

        let mut generator = Generator::new(&context);

        generator.bind_types(crate::binds::StdXsdTypes);

        let (type_, actual_items) = generator.generate_top_level_attribute(&sequence).unwrap();

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
