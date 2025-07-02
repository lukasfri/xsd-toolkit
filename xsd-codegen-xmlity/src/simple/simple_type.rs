use crate::misc::common_name;
use crate::misc::dedup_field_idents;
use crate::misc::COMMON_NAME_MIN_LENGTH;
use crate::simple::SimpleContext;
use crate::templates::choice::ChoiceVariantType;
use crate::templates::value_record::ItemField;
use crate::templates::value_record::ItemFieldItem;
use crate::templates::value_record::ItemRecord;
use crate::GeneratorScope;
use crate::Result;
use crate::ToIdentTypesExt;
use crate::TypeType;
use crate::{misc::TypeReference, simple::SimpleToTypeTemplate, templates, ToTypeTemplateData};
use quote::format_ident;
use quote::ToTokens;
use std::ops::Deref;
use std::sync::LazyLock;
use syn::parse_quote;
use xsd_type_compiler::fragments::FragmentIdx;
use xsd_type_compiler::{fragments::simple as sm, NamedOrAnonymous};

impl SimpleToTypeTemplate for sm::ListFragment {
    type TypeTemplate = TypeReference<'static>;

    fn to_type_template<C: super::SimpleContext, S: crate::Scope>(
        &self,
        context: &C,
        scope: &mut S,
    ) -> crate::Result<crate::ToTypeTemplateData<Self::TypeTemplate>> {
        let ty = context.resolve_fragment(&self.item_type, scope)?;

        Ok(crate::ToTypeTemplateData {
            ident: None,
            template: ty.template.wrap(|ty: syn::Type| -> syn::Type {
                parse_quote!(
                    crate::xs::types::List<#ty>
                )
            }),
        })
    }
}

impl SimpleToTypeTemplate for sm::UnionFragment {
    type TypeTemplate = templates::choice::Choice;

    fn to_type_template<C: super::SimpleContext, S: crate::Scope>(
        &self,
        context: &C,
        scope: &mut S,
    ) -> crate::Result<crate::ToTypeTemplateData<Self::TypeTemplate>> {
        let mut sub_scope = GeneratorScope::new(scope.augmenter());

        // Struct with strict order
        let member_type_variants = self
            .member_types
            .iter()
            .enumerate()
            .map(|(i, name)| {
                let suggested_ident = format_ident!("Variant{i}");
                let res = context.resolve_named_type(name)?;

                assert_eq!(
                    res.ty_type,
                    TypeType::Simple,
                    "Member type of union must be simple"
                );

                let ident = suggested_ident;

                //TODO: Get name from the name of the type
                Ok(((ident.to_variant_ident(), res.ty), ident))
            })
            .collect::<Result<Vec<_>>>()?;

        let simple_type_variants = self
            .simple_types
            .iter()
            .enumerate()
            .map(|(i, fragment_id)| {
                let suggested_ident = format_ident!("Variant{i}");
                let res = context
                    .sub_context(suggested_ident.clone())
                    .resolve_fragment_id(fragment_id, &mut sub_scope)?;

                let ident = res.ident.unwrap_or_else(|| suggested_ident);

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

impl SimpleToTypeTemplate for sm::SimpleDerivation {
    type TypeTemplate = TypeReference<'static>;

    fn to_type_template<C: super::SimpleContext, S: crate::Scope>(
        &self,
        context: &C,
        scope: &mut S,
    ) -> crate::Result<crate::ToTypeTemplateData<Self::TypeTemplate>> {
        match self {
            sm::SimpleDerivation::Restriction(fragment_idx) => {
                context.resolve_fragment_id(fragment_idx, scope)
            }
            sm::SimpleDerivation::List(fragment_idx) => {
                context.resolve_fragment_id(fragment_idx, scope)
            }
            sm::SimpleDerivation::Union(fragment_idx) => {
                let ident = context.suggested_ident();
                let res = context.resolve_fragment_id(fragment_idx, scope)?;

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

impl SimpleToTypeTemplate for sm::SimpleTypeRootFragment {
    type TypeTemplate = TypeReference<'static>;

    fn to_type_template<C: super::SimpleContext, S: crate::Scope>(
        &self,
        context: &C,
        scope: &mut S,
    ) -> crate::Result<crate::ToTypeTemplateData<Self::TypeTemplate>> {
        self.simple_derivation.to_type_template(context, scope)
    }
}

impl SimpleToTypeTemplate for NamedOrAnonymous<FragmentIdx<sm::SimpleTypeRootFragment>> {
    type TypeTemplate = TypeReference<'static>;

    fn to_type_template<C: super::SimpleContext, S: crate::Scope>(
        &self,
        context: &C,
        scope: &mut S,
    ) -> crate::Result<crate::ToTypeTemplateData<Self::TypeTemplate>> {
        match self {
            NamedOrAnonymous::Named(name) => {
                let bound_type = context.resolve_named_type(name)?;

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
                context.resolve_fragment_id(fragment_idx, scope)
            }
        }
    }
}

impl SimpleToTypeTemplate for Option<NamedOrAnonymous<FragmentIdx<sm::SimpleTypeRootFragment>>> {
    type TypeTemplate = TypeReference<'static>;

    fn to_type_template<C: super::SimpleContext, S: crate::Scope>(
        &self,
        context: &C,
        scope: &mut S,
    ) -> crate::Result<crate::ToTypeTemplateData<Self::TypeTemplate>> {
        static SIMPLE_ANY_TYPE_NAMED: LazyLock<
            NamedOrAnonymous<FragmentIdx<sm::SimpleTypeRootFragment>>,
        > = LazyLock::new(|| NamedOrAnonymous::Named(xsd::xsn::SIMPLE_ANY_TYPE.clone()));

        self.as_ref()
            .unwrap_or_else(|| SIMPLE_ANY_TYPE_NAMED.deref())
            .to_type_template(context, scope)
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
}
