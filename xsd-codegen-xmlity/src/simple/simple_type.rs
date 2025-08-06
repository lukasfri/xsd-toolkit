use crate::generator::GeneratorScope;
use crate::misc::common_name;
use crate::misc::dedup_field_idents;
use crate::misc::WeakExt;
use crate::misc::COMMON_NAME_MIN_LENGTH;
use crate::simple::restrictions::RestrictionHandler;
use crate::simple::SimpleContext;
use crate::templates::choice::ChoiceVariantType;
use crate::templates::value_record::ItemField;
use crate::templates::value_record::ItemFieldItem;
use crate::templates::value_record::ItemRecord;
use crate::Result;
use crate::ToIdentTypesExt;
use crate::TypeType;
use crate::{misc::TypeReference, simple::SimpleToTypeTemplate, templates, ToTypeTemplateData};
use quote::format_ident;
use quote::ToTokens;
use std::any::type_name;
use std::fmt::Debug;
use std::ops::Deref;
use std::sync::Arc;
use std::sync::LazyLock;
use std::sync::Weak;
use syn::parse_quote;
use syn::Type;
use xsd_fragments::fragments::FragmentIdx;
use xsd_fragments::{fragments::simple as sm, NamedOrAnonymous};

pub struct ListHandler {
    pub simple_type_handler: Weak<SimpleTypeRootHandler>,
    pub list_wrapper: Arc<dyn Fn(Type) -> Type + 'static>,
}

impl Debug for ListHandler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ListHandler")
            .field("list_wrapper", &"&<dyn Fn(Type) -> Type + 'static>")
            .finish()
    }
}

impl SimpleToTypeTemplate<sm::ListFragment> for ListHandler {
    type TypeTemplate = TypeReference<'static>;

    fn to_type_template<C: super::SimpleContext, S: crate::Scope>(
        &self,
        context: &C,
        scope: &mut S,
        item: &sm::ListFragment,
    ) -> crate::Result<crate::ToTypeTemplateData<Self::TypeTemplate>> {
        let simple_type_handler =
            self.simple_type_handler
                .upgrade_or_else(|| crate::Error::HandlerDoesNotExist {
                    origin: type_name::<Self>(),
                    handler: type_name::<SimpleTypeRootHandler>(),
                })?;

        let ty = simple_type_handler.to_type_template(context, scope, &item.item_type)?;

        let list_wrapper = self.list_wrapper.clone();

        Ok(crate::ToTypeTemplateData {
            ident: None,
            template: ty
                .template
                .wrap(move |ty: syn::Type| -> syn::Type { list_wrapper(ty) }),
        })
    }
}

#[derive(Debug)]
pub struct UnionHandler {
    pub simple_type_handler: Weak<SimpleTypeRootHandler>,
}

impl SimpleToTypeTemplate<sm::UnionFragment> for UnionHandler {
    type TypeTemplate = templates::choice::Choice;

    fn to_type_template<C: super::SimpleContext, S: crate::Scope>(
        &self,
        context: &C,
        scope: &mut S,
        item: &sm::UnionFragment,
    ) -> crate::Result<crate::ToTypeTemplateData<Self::TypeTemplate>> {
        let mut sub_scope = GeneratorScope::new(scope.augmenter());

        let simple_type_handler =
            self.simple_type_handler
                .upgrade_or_else(|| crate::Error::HandlerDoesNotExist {
                    origin: type_name::<Self>(),
                    handler: type_name::<SimpleTypeRootHandler>(),
                })?;

        // Struct with strict order
        let member_type_variants = item
            .member_types
            .iter()
            .map(|name| {
                let res = context.resolve_named_type(context.namespace_idx(), name)?;

                assert_eq!(
                    res.ty_type,
                    TypeType::Simple,
                    "Member type of union must be simple"
                );

                let ident = name.local_name().to_item_ident();

                Ok(((ident.to_variant_ident(), res.ty), ident))
            })
            .collect::<Result<Vec<_>>>()?;

        let simple_type_variants = item
            .simple_types
            .iter()
            .enumerate()
            .map(|(i, fragment_id)| {
                let suggested_ident = format_ident!("Variant{i}");
                let res = context
                    .sub_context(suggested_ident.clone())
                    .resolve_type_template(
                        fragment_id,
                        &mut sub_scope,
                        simple_type_handler.deref(),
                    )?;

                let ident = res.ident.unwrap_or(suggested_ident);

                Ok(((ident.to_variant_ident(), res.template), ident))
            })
            .collect::<Result<Vec<_>>>()?;

        let variants = member_type_variants
            .into_iter()
            .chain(simple_type_variants)
            .collect::<Vec<_>>();

        let (variants, names) = variants.into_iter().unzip::<_, _, Vec<_>, Vec<_>>();

        let common_name = common_name(names.iter().map(|a| a.to_string()), COMMON_NAME_MIN_LENGTH);

        let ident = common_name
            .map(|a| format_ident!("{a}"))
            .unwrap_or_else(|| context.suggested_ident().clone())
            .to_item_ident();

        let mod_name = format_ident!("{}_variants", ident.to_path_ident());

        let mod_path: syn::Path = parse_quote!(#mod_name);

        let variants = variants
            .into_iter()
            .map(|(ident, mut variant)| {
                variant = variant
                    .wrap(TypeReference::box_non_boxed_wrapper)
                    .prefix(mod_path.clone());

                let variant = ChoiceVariantType::Item(ItemRecord::new_single_field(
                    None,
                    ItemField::Item(ItemFieldItem {
                        ty: variant.clone(),
                        default: false,
                        default_with: None,
                    }),
                ));

                Ok((ident, variant))
            })
            .collect::<Result<Vec<_>>>()?;

        let variants = dedup_field_idents(variants);

        let template = templates::choice::Choice { variants };

        let _mod_ref = sub_scope
            .finish_mod(&mod_name)
            .map(|a| scope.add_item(a))
            .transpose()?;

        Ok(ToTypeTemplateData {
            ident: Some(ident),
            template,
        })
    }
}

#[derive(Debug)]
pub struct SimpleDerivationHandler {
    pub restriction_handler: RestrictionHandler,
    pub list_handler: ListHandler,
    pub union_handler: UnionHandler,
}

impl SimpleToTypeTemplate<sm::SimpleDerivation> for SimpleDerivationHandler {
    type TypeTemplate = TypeReference<'static>;

    fn to_type_template<C: super::SimpleContext, S: crate::Scope>(
        &self,
        context: &C,
        scope: &mut S,
        item: &sm::SimpleDerivation,
    ) -> crate::Result<crate::ToTypeTemplateData<Self::TypeTemplate>> {
        match item {
            sm::SimpleDerivation::Restriction(fragment_idx) => {
                context.resolve_type_template(fragment_idx, scope, &self.restriction_handler)
            }
            sm::SimpleDerivation::List(fragment_idx) => {
                context.resolve_type_template(fragment_idx, scope, &self.list_handler)
            }
            sm::SimpleDerivation::Union(fragment_idx) => {
                let ident = context.suggested_ident();
                let res =
                    context.resolve_type_template(fragment_idx, scope, &self.union_handler)?;

                let enum_ = res.template.to_enum(&ident.to_item_ident(), None);

                let ty = scope.add_item(enum_)?;

                Ok(crate::ToTypeTemplateData {
                    ident: Some(ident.to_item_ident()),
                    template: ty,
                })
            }
        }
    }
}

#[derive(Debug)]
pub struct SimpleTypeRootHandler {
    pub simple_derivation_handler: SimpleDerivationHandler,
}

impl SimpleToTypeTemplate<sm::SimpleTypeRootFragment> for SimpleTypeRootHandler {
    type TypeTemplate = TypeReference<'static>;

    fn to_type_template<C: super::SimpleContext, S: crate::Scope>(
        &self,
        context: &C,
        scope: &mut S,
        item: &sm::SimpleTypeRootFragment,
    ) -> crate::Result<crate::ToTypeTemplateData<Self::TypeTemplate>> {
        self.simple_derivation_handler
            .to_type_template(context, scope, &item.simple_derivation)
    }
}

impl SimpleToTypeTemplate<NamedOrAnonymous<FragmentIdx<sm::SimpleTypeRootFragment>>>
    for SimpleTypeRootHandler
{
    type TypeTemplate = TypeReference<'static>;

    fn to_type_template<C: super::SimpleContext, S: crate::Scope>(
        &self,
        context: &C,
        scope: &mut S,
        item: &NamedOrAnonymous<FragmentIdx<sm::SimpleTypeRootFragment>>,
    ) -> crate::Result<crate::ToTypeTemplateData<Self::TypeTemplate>> {
        match item {
            NamedOrAnonymous::Named(name) => {
                let bound_type = context.resolve_named_type(context.namespace_idx(), name)?;

                assert_eq!(
                    bound_type.ty_type,
                    crate::TypeType::Simple,
                    "{} is not a simple type, but is in a simple reference",
                    bound_type.ty.to_type(None).to_token_stream()
                );

                Ok(crate::ToTypeTemplateData {
                    ident: None,
                    template: bound_type.ty,
                })
            }
            NamedOrAnonymous::Anonymous(fragment_idx) => {
                context.resolve_type_template(fragment_idx, scope, self)
            }
        }
    }
}

impl SimpleToTypeTemplate<Option<NamedOrAnonymous<FragmentIdx<sm::SimpleTypeRootFragment>>>>
    for SimpleTypeRootHandler
{
    type TypeTemplate = TypeReference<'static>;

    fn to_type_template<C: super::SimpleContext, S: crate::Scope>(
        &self,
        context: &C,
        scope: &mut S,
        item: &Option<NamedOrAnonymous<FragmentIdx<sm::SimpleTypeRootFragment>>>,
    ) -> crate::Result<crate::ToTypeTemplateData<Self::TypeTemplate>> {
        static SIMPLE_ANY_TYPE_NAMED: LazyLock<
            NamedOrAnonymous<FragmentIdx<sm::SimpleTypeRootFragment>>,
        > = LazyLock::new(|| NamedOrAnonymous::Named(xsd::xsn::SIMPLE_ANY_TYPE.clone()));

        self.to_type_template(
            context,
            scope,
            item.as_ref().unwrap_or_else(|| &*SIMPLE_ANY_TYPE_NAMED),
        )
    }
}
