use quote::format_ident;
use std::fmt::Debug;
use std::str::FromStr;
use syn::parse_quote;

use super::RestrictionBuilder;
use crate::misc::TypeReference;
use crate::templates;
use crate::ToIdentTypesExt;
use inflector::Inflector;
use xsd_type_compiler::fragments::simple as sm;

pub trait StringBaseValue: FromStr<Err: Debug> {
    fn to_pattern(&self) -> syn::Pat;

    fn to_pattern_value(value: &syn::Expr) -> syn::Expr;

    fn to_value_expr(&self) -> syn::Expr;

    fn repr_type() -> syn::Type;

    fn value_ident(&self) -> syn::Ident;
}

impl StringBaseValue for String {
    fn to_pattern(&self) -> syn::Pat {
        let val_string = proc_macro2::Literal::string(self.as_str());

        parse_quote!(#val_string)
    }

    fn to_pattern_value(value: &syn::Expr) -> syn::Expr {
        // In code: String::from("value")
        parse_quote!(::std::string::String::as_str(&#value))
    }

    fn to_value_expr(&self) -> syn::Expr {
        // In code: String::from("value")
        parse_quote!(::std::string::String::from(#self))
    }

    fn repr_type() -> syn::Type {
        // In code: String
        parse_quote!(::std::string::String)
    }

    fn value_ident(&self) -> syn::Ident {
        const MAX_IDENT_LENGTH: usize = 24;

        let a = self
            .to_pascal_case()
            .chars()
            .take(MAX_IDENT_LENGTH)
            .collect::<String>()
            .replace('-', "_")
            .replace(' ', "_")
            .replace('.', "_");

        format_ident!("{}", a)
    }
}

pub struct StringRestrictionBuilder<T: StringBaseValue> {
    _marker: std::marker::PhantomData<T>,
}

impl<T: StringBaseValue> StringRestrictionBuilder<T> {
    pub fn new() -> Self {
        Self {
            _marker: std::marker::PhantomData,
        }
    }
}

impl<C: crate::simple::SimpleContext, S: crate::Scope, T: StringBaseValue> RestrictionBuilder<C, S>
    for StringRestrictionBuilder<T>
{
    fn build(
        &self,
        context: &C,
        scope: &mut S,
        facets: &[&sm::FacetFragment],
    ) -> crate::Result<crate::ToTypeTemplateData<TypeReference<'static>>> {
        let ident = context.suggested_ident().clone();

        let enum_with_ident = format_ident!("{}_with", ident.to_path_ident());
        let error_ident = format_ident!("{}ParseError", ident.to_item_ident());

        let mut length = None;
        let mut min_length = None;
        let mut max_length = None;
        let mut enumerations = Vec::new();

        for facet in facets {
            match facet {
                sm::FacetFragment::Enumeration { value } => {
                    enumerations.push(value.0.as_str());
                }
                sm::FacetFragment::Length { value } => {
                    length = Some(value);
                }
                sm::FacetFragment::MinLength { value } => {
                    min_length = Some(value);
                }
                sm::FacetFragment::MaxLength { value } => {
                    max_length = Some(value);
                }
                sm::FacetFragment::Pattern { value } => todo!(),
                sm::FacetFragment::Assertion { test } => todo!(),
                sm::FacetFragment::MinExclusive { .. }
                | sm::FacetFragment::MinInclusive { .. }
                | sm::FacetFragment::MaxExclusive { .. }
                | sm::FacetFragment::MaxInclusive { .. }
                | sm::FacetFragment::TotalDigits { .. }
                | sm::FacetFragment::FractionDigits { .. }
                | sm::FacetFragment::WhiteSpace { .. }
                | sm::FacetFragment::ExplicitTimezone { .. } => {
                    todo!("Unsupported facet: {:?}", facet)
                }
            }
        }

        if enumerations.is_empty() {
            let struct_def = templates::wrapper_struct::WrapperStruct {
                struct_ident: ident.to_item_ident(),
                repr_type: T::repr_type(),
                repr: false,
                enum_with_mod: enum_with_ident,
            };

            let struct_item = struct_def.to_struct();
            let (err, try_from_impl) = struct_def.try_from_impl(&error_ident);
            let into_impl = struct_def.into_impl();
            let with_mod = struct_def.with_mod();
            let enum_ty = scope.add_item(struct_item)?;
            scope.add_item(with_mod)?;
            scope.add_item(err)?;
            scope.add_raw_items([try_from_impl, into_impl]);

            Ok(crate::ToTypeTemplateData {
                ident: Some(ident),
                template: enum_ty,
            })
        } else {
            // If there are enumerations, we create an enum type
            let enumerations = enumerations
                .into_iter()
                .map(|s| T::from_str(s).expect("Failed to parse enumeration value"))
                .collect::<Vec<_>>();

            let enumerations = enumerations
                .iter()
                .map(|v| (v.value_ident(), v.to_pattern(), v.to_value_expr()))
                .collect();

            let enum_def = templates::specific_enum::SpecificEnum {
                enum_ident: ident.to_item_ident(),
                repr_type: T::repr_type(),
                enumerations,
                repr: false,
                enum_with_mod: enum_with_ident,
                value_to_pattern: |a| T::to_pattern_value(a),
            };

            let enum_item = enum_def.to_enum();
            let (err_items, try_from_impl) = enum_def.try_from_impl(&error_ident);
            let into_impl = enum_def.into_impl();
            let with_mod = enum_def.with_mod();
            let enum_ty = scope.add_item(enum_item)?;
            scope.add_item(with_mod)?;
            scope.add_raw_items(err_items);
            scope.add_raw_items([try_from_impl, into_impl]);

            Ok(crate::ToTypeTemplateData {
                ident: Some(ident),
                template: enum_ty,
            })
        }
    }
}
