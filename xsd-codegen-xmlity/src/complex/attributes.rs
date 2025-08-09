use std::sync::Arc;

use super::{ComplexContext, ComplexToTypeTemplate, Scope, ToTypeTemplateData};
use crate::complex::ComplexToTypeTemplateExt;
use crate::misc::{dedup_field_idents, TypeReference};
use crate::naming_strategies::{IndexedNamingStrategy, WrappingNamingStrategy};
use crate::templates::element_record::{ElementField, ElementFieldGroup};
use crate::{
    simple::{simple_type::SimpleTypeRootHandler, SimpleContext, SimpleToTypeTemplate},
    templates::element_record::ElementFieldAttribute,
    Result, ToIdentTypesExt,
};
use quote::format_ident;
use syn::parse_quote;
use xmlity::ExpandedName;
use xsd_fragments::fragments::complex::{self as cx, AttributeUse};
use xsd_fragments::fragments::FragmentIdx;

#[derive(Debug)]
pub struct AnyAttributesHandler {
    pub any_attributes_ident: String,
    pub any_attributes_type: syn::Type,
}

impl ComplexToTypeTemplate<cx::AnyAttributeFragment> for AnyAttributesHandler {
    type TypeTemplate = (syn::Ident, ElementField);

    fn to_type_template<C: ComplexContext, S: Scope>(
        &self,
        _context: &C,
        _scope: &mut S,
        _item: &cx::AnyAttributeFragment,
    ) -> Result<ToTypeTemplateData<Self::TypeTemplate>> {
        let ident = format_ident!("{}", self.any_attributes_ident);

        let any_attributes = (
            ident.to_field_ident(),
            ElementField::Group(ElementFieldGroup {
                ty: TypeReference::new_static(self.any_attributes_type.clone()),
            }),
        );

        Ok(ToTypeTemplateData {
            ident: Some(ident),
            template: any_attributes,
        })
    }
}

#[derive(Debug)]
pub struct AttributeDeclarationsHandler {
    pub attribute_declaration_handler: Arc<AttributeDeclarationHandler>,
    pub any_attributes_handler: Arc<AnyAttributesHandler>,
    pub suggested_attribute_type_naming: IndexedNamingStrategy,
    pub default_attribute_ident_naming: IndexedNamingStrategy,
}

impl ComplexToTypeTemplate<cx::AttributeDeclarationsFragment> for AttributeDeclarationsHandler {
    type TypeTemplate = Vec<(syn::Ident, ElementField)>;

    fn to_type_template<C: ComplexContext, S: Scope>(
        &self,
        context: &C,
        scope: &mut S,
        item: &cx::AttributeDeclarationsFragment,
    ) -> Result<ToTypeTemplateData<Self::TypeTemplate>> {
        let attributes = item
            .declarations
            .iter()
            .enumerate()
            .map(|(i, a)| {
                let sub_context =
                    context.sub_context(self.suggested_attribute_type_naming.ident_for_index(i));

                self.attribute_declaration_handler
                    .to_type_template(&sub_context, scope, a)
                    .map(|a| {
                        (
                            a.ident.map(|a| a.to_field_ident()).unwrap_or_else(|| {
                                self.default_attribute_ident_naming.ident_for_index(i)
                            }),
                            ElementField::Attribute(a.template),
                        )
                    })
            })
            .collect::<Result<Vec<_>>>()?;

        let any_attributes = item
            .any_attribute
            .as_ref()
            .map(|id| {
                self.any_attributes_handler
                    .resolve_type_template(context, scope, id)
            })
            .transpose()?
            .map(|a| a.template);

        let attributes = dedup_field_idents(attributes.into_iter().chain(any_attributes));

        Ok(ToTypeTemplateData {
            ident: None,
            template: attributes,
        })
    }
}

#[derive(Debug)]
pub struct LocalAttributeHandler {
    pub simple_type_handler: Arc<SimpleTypeRootHandler>,
    pub value_type_naming: WrappingNamingStrategy,
}

impl ComplexToTypeTemplate<FragmentIdx<cx::LocalAttributeFragment>> for LocalAttributeHandler {
    type TypeTemplate = ElementFieldAttribute;

    fn to_type_template<C: ComplexContext, S: Scope>(
        &self,
        context: &C,
        scope: &mut S,
        fragment_idx: &FragmentIdx<cx::LocalAttributeFragment>,
    ) -> Result<ToTypeTemplateData<Self::TypeTemplate>> {
        let item = context.get_fragment(fragment_idx).unwrap();

        let optional = match item.use_.unwrap_or_default() {
            AttributeUse::Optional => true,
            AttributeUse::Required => false,
            AttributeUse::Prohibited => return Err(crate::Error::prohibited_attributes()),
        };

        let (ident, template) = match &item.type_mode {
            cx::LocalAttributeFragmentTypeMode::Declared(local) => {
                let name = ExpandedName::new(local.name.clone(), None);
                let ident = local.name.to_item_ident();

                let simple_context = context
                    .simple_context()
                    .sub_context(self.value_type_naming.wrap_ident(&ident));

                let ty = self
                    .simple_type_handler
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
                let ty = context
                    .resolve_named_attribute(&fragment_idx.namespace_idx(), &reference.ref_)?;

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
    pub local_attribute_handler: Arc<LocalAttributeHandler>,
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
                .to_type_template(context, scope, fragment_idx),
            cx::AttributeDeclarationId::AttributeGroupRef(_fragment_idx) => {
                Err(crate::Error::UnsupportedFragment {
                    fragment: "AttributeGroupRef".to_string(),
                })
            }
        }
    }
}

#[derive(Debug)]
pub struct TopLevelAttributeHandler {
    pub simple_type_handler: Arc<SimpleTypeRootHandler>,
}

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

        let ty = self
            .simple_type_handler
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
    use std::str::FromStr;

    use pretty_assertions::assert_eq;

    use syn::{parse_quote, Item};
    use url::Url;
    use xmlity::{LocalName, XmlNamespace};
    use xsd::{ns, xs, xsn};
    use xsd_fragments::XmlnsContext;

    use crate::Generator;

    #[test]
    fn simple_attribute() {
        const TEST_NAMESPACE: XmlNamespace<'static> =
            XmlNamespace::new_dangerous("http://example.com");
        let test_location = Url::from_str("http://example.com/test.xsd").unwrap();

        let attribute = xs::types::TopLevelAttribute::builder()
            .name(LocalName::new_dangerous("SimpleAttribute"))
            .type_(xs::types::QName(xsn::STRING.clone()))
            .any_attributes(ns::AnyAttributes::default())
            .build()
            .into();

        let mut ctx = XmlnsContext::new();
        let (ns_id, ns) = ctx.init_namespace(test_location.clone(), TEST_NAMESPACE.into());

        let sequence = ns
            .import_top_level_attribute(&attribute)
            .unwrap()
            .into_owned();

        let mut generator = Generator::new(&ctx);

        generator.bind_types(crate::binds::StdXsdTypes);

        let (type_, actual_items) = generator.generate_attribute(&ns_id, &sequence).unwrap();

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
