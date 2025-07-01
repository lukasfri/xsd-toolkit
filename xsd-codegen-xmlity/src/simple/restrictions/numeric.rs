use crate::templates;
use crate::ToIdentTypesExt;
use crate::{misc::TypeReference, simple::restrictions::RestrictionBuilder};
use quote::format_ident;
use std::fmt::Debug;
use std::str::FromStr;
use syn::parse_quote;
use xsd_type_compiler::fragments::simple as sm;

pub trait NumericBaseValue: FromStr<Err: Debug> {
    fn to_pattern(&self) -> syn::Pat;

    fn to_pattern_value(value: &syn::Expr) -> syn::Expr {
        value.clone()
    }

    fn to_value_expr(&self) -> syn::Expr {
        let pattern = self.to_pattern();

        parse_quote!(#pattern)
    }

    fn repr_type() -> syn::Type;

    fn value_ident(&self) -> syn::Ident;

    fn is_repr() -> bool {
        // Used to determine if #[repr(...)] can be applied to the type
        false
    }

    fn supports_fraction_digits() -> bool {
        false
    }
}

impl NumericBaseValue for rust_decimal::Decimal {
    fn to_pattern(&self) -> syn::Pat {
        let mantissa = self.mantissa();
        let scale = self.scale();

        let mantissa = proc_macro2::Literal::i128_suffixed(mantissa);
        let scale = proc_macro2::Literal::u32_suffixed(scale);

        parse_quote!((#mantissa, #scale))
    }

    fn to_pattern_value(value: &syn::Expr) -> syn::Expr {
        // In code: (::rust_decimal::Decimal::mantissa(&#value), ::rust_decimal::Decimal::scale(&#value))
        parse_quote!((
            ::rust_decimal::Decimal::mantissa(&#value),
            ::rust_decimal::Decimal::scale(&#value)
        ))
    }

    fn to_value_expr(&self) -> syn::Expr {
        let mantissa = self.mantissa();
        let scale = self.scale();

        let mantissa = proc_macro2::Literal::i128_suffixed(mantissa);
        let scale = proc_macro2::Literal::u32_suffixed(scale);

        parse_quote!(::rust_decimal::Decimal::from_i128_with_scale(#mantissa, #scale))
    }

    fn repr_type() -> syn::Type {
        // In code: ::rust_decimal::Decimal
        parse_quote!(::rust_decimal::Decimal)
    }

    fn value_ident(&self) -> syn::Ident {
        // In code: D123456
        format_ident!("D{}", self.to_string().replace('.', "_"))
    }

    fn is_repr() -> bool {
        true
    }
}

impl NumericBaseValue for f32 {
    fn to_pattern(&self) -> syn::Pat {
        // In code: 123.456f32
        let val_string = proc_macro2::Literal::f32_suffixed(*self);

        parse_quote!(#val_string)
    }

    fn repr_type() -> syn::Type {
        // In code: f32
        parse_quote!(f32)
    }

    fn value_ident(&self) -> syn::Ident {
        let prefix = if *self >= 0.0 { "P" } else { "N" };
        format_ident!(
            "{}{}",
            prefix,
            self.to_string().replace('-', "").replace('.', "_")
        )
    }

    fn is_repr() -> bool {
        true
    }
}

impl NumericBaseValue for f64 {
    fn to_pattern(&self) -> syn::Pat {
        // In code: 123.456f64
        let val_string = proc_macro2::Literal::f64_suffixed(*self);

        parse_quote!(#val_string)
    }

    fn repr_type() -> syn::Type {
        // In code: f64
        parse_quote!(f64)
    }

    fn value_ident(&self) -> syn::Ident {
        let prefix = if *self >= 0.0 { "P" } else { "N" };
        format_ident!(
            "{}{}",
            prefix,
            self.to_string().replace('-', "").replace('.', "_")
        )
    }

    fn is_repr() -> bool {
        true
    }
}

impl NumericBaseValue for usize {
    fn to_pattern(&self) -> syn::Pat {
        // In code: 123usize
        let val_string = proc_macro2::Literal::usize_suffixed(*self);

        parse_quote!(#val_string)
    }

    fn repr_type() -> syn::Type {
        // In code: usize
        parse_quote!(usize)
    }

    fn value_ident(&self) -> syn::Ident {
        // In code: U123
        format_ident!("U{}", self.to_string())
    }

    fn is_repr() -> bool {
        true
    }
}

impl NumericBaseValue for isize {
    fn to_pattern(&self) -> syn::Pat {
        // In code: 123isize
        let val_string = proc_macro2::Literal::isize_suffixed(*self);

        parse_quote!(#val_string)
    }

    fn repr_type() -> syn::Type {
        // In code: isize
        parse_quote!(isize)
    }

    fn value_ident(&self) -> syn::Ident {
        let prefix = if *self >= 0 { "P" } else { "N" };
        format_ident!("{}{}", prefix, self.to_string())
    }

    fn is_repr() -> bool {
        true
    }
}

impl NumericBaseValue for u64 {
    fn to_pattern(&self) -> syn::Pat {
        // In code: 123u64
        let val_string = proc_macro2::Literal::u64_suffixed(*self);

        parse_quote!(#val_string)
    }

    fn repr_type() -> syn::Type {
        // In code: u64
        parse_quote!(u64)
    }

    fn value_ident(&self) -> syn::Ident {
        // In code: U123
        format_ident!("U{}", self.to_string())
    }

    fn is_repr() -> bool {
        true
    }
}

impl NumericBaseValue for u32 {
    fn to_pattern(&self) -> syn::Pat {
        // In code: 123u32
        let val_string = proc_macro2::Literal::u32_suffixed(*self);

        parse_quote!(#val_string)
    }

    fn repr_type() -> syn::Type {
        // In code: u32
        parse_quote!(u32)
    }

    fn value_ident(&self) -> syn::Ident {
        // In code: U123
        format_ident!("U{}", self.to_string())
    }

    fn is_repr() -> bool {
        true
    }
}

impl NumericBaseValue for u16 {
    fn to_pattern(&self) -> syn::Pat {
        // In code: 123u16
        let val_string = proc_macro2::Literal::u16_suffixed(*self);

        parse_quote!(#val_string)
    }

    fn repr_type() -> syn::Type {
        // In code: u16
        parse_quote!(u16)
    }

    fn value_ident(&self) -> syn::Ident {
        // In code: U123
        format_ident!("U{}", self.to_string())
    }

    fn is_repr() -> bool {
        true
    }
}

impl NumericBaseValue for u8 {
    fn to_pattern(&self) -> syn::Pat {
        // In code: 123u8
        let val_string = proc_macro2::Literal::u8_suffixed(*self);

        parse_quote!(#val_string)
    }

    fn repr_type() -> syn::Type {
        // In code: u8
        parse_quote!(u8)
    }

    fn value_ident(&self) -> syn::Ident {
        // In code: U123
        format_ident!("U{}", self.to_string())
    }

    fn is_repr() -> bool {
        true
    }
}

impl NumericBaseValue for i64 {
    fn to_pattern(&self) -> syn::Pat {
        // In code: 123i64
        let val_string = proc_macro2::Literal::i64_suffixed(*self);

        parse_quote!(#val_string)
    }

    fn repr_type() -> syn::Type {
        // In code: i64
        parse_quote!(i64)
    }

    fn value_ident(&self) -> syn::Ident {
        let prefix = if *self >= 0 { "P" } else { "N" };
        format_ident!("{}{}", prefix, self.to_string().replace('-', ""))
    }

    fn is_repr() -> bool {
        true
    }
}

impl NumericBaseValue for i32 {
    fn to_pattern(&self) -> syn::Pat {
        // In code: 123i32
        let val_string = proc_macro2::Literal::i32_suffixed(*self);

        parse_quote!(#val_string)
    }

    fn repr_type() -> syn::Type {
        // In code: i32
        parse_quote!(i32)
    }

    fn value_ident(&self) -> syn::Ident {
        let prefix = if *self >= 0 { "P" } else { "N" };
        format_ident!("{}{}", prefix, self.to_string().replace('-', ""))
    }

    fn is_repr() -> bool {
        true
    }
}

impl NumericBaseValue for i16 {
    fn to_pattern(&self) -> syn::Pat {
        // In code: 123i16
        let val_string = proc_macro2::Literal::i16_suffixed(*self);

        parse_quote!(#val_string)
    }

    fn repr_type() -> syn::Type {
        // In code: i16
        parse_quote!(i16)
    }

    fn value_ident(&self) -> syn::Ident {
        let prefix = if *self >= 0 { "P" } else { "N" };
        format_ident!("{}{}", prefix, self.to_string().replace('-', ""))
    }

    fn is_repr() -> bool {
        true
    }
}

impl NumericBaseValue for i8 {
    fn to_pattern(&self) -> syn::Pat {
        // In code: 123i8
        let val_string = proc_macro2::Literal::i8_suffixed(*self);

        parse_quote!(#val_string)
    }

    fn repr_type() -> syn::Type {
        // In code: i8
        parse_quote!(i8)
    }

    fn value_ident(&self) -> syn::Ident {
        let prefix = if *self >= 0 { "P" } else { "N" };
        format_ident!("{}{}", prefix, self.to_string().replace('-', ""))
    }

    fn is_repr() -> bool {
        true
    }
}

impl NumericBaseValue for std::num::NonZeroUsize {
    fn to_pattern(&self) -> syn::Pat {
        // In code: NonZeroUsize::new(123).unwrap()
        let val_string = proc_macro2::Literal::usize_suffixed(self.get());

        parse_quote!(::core::num::NonZeroUsize::new(#val_string).unwrap())
    }

    fn repr_type() -> syn::Type {
        // In code: NonZeroUsize
        parse_quote!(::core::num::NonZeroUsize)
    }

    fn value_ident(&self) -> syn::Ident {
        // In code: U123
        format_ident!("U{}", self.get())
    }
}

pub struct NumericRestrictionBuilder<T: NumericBaseValue> {
    _marker: std::marker::PhantomData<T>,
}

impl<T: NumericBaseValue> NumericRestrictionBuilder<T> {
    pub fn new() -> Self {
        Self {
            _marker: std::marker::PhantomData,
        }
    }
}

impl<C: crate::simple::SimpleContext, S: crate::Scope, T: NumericBaseValue> RestrictionBuilder<C, S>
    for NumericRestrictionBuilder<T>
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

        let mut min_inclusive = None;
        let mut min_exclusive = None;
        let mut max_inclusive = None;
        let mut max_exclusive = None;
        let mut enumerations = Vec::new();

        for facet in facets {
            match facet {
                sm::FacetFragment::MinExclusive { value } => {
                    min_exclusive = Some(value.0.trim());
                }
                sm::FacetFragment::MinInclusive { value } => {
                    min_inclusive = Some(value.0.trim());
                }
                sm::FacetFragment::MaxExclusive { value } => {
                    max_exclusive = Some(value.0.trim());
                }
                sm::FacetFragment::MaxInclusive { value } => {
                    max_inclusive = Some(value.0.trim());
                }
                sm::FacetFragment::Enumeration { value } => {
                    enumerations.push(value.0.trim());
                }
                sm::FacetFragment::TotalDigits { value } => todo!(),
                sm::FacetFragment::FractionDigits { value } => {
                    if T::supports_fraction_digits() {
                        assert_eq!(
                            *value, 0,
                            "FractionDigits facet is not supported for numeric types"
                        );
                    }
                }
                sm::FacetFragment::Pattern { value } => todo!(),
                sm::FacetFragment::Assertion { test } => todo!(),
                sm::FacetFragment::WhiteSpace { .. }
                | sm::FacetFragment::ExplicitTimezone { .. }
                | sm::FacetFragment::Length { .. }
                | sm::FacetFragment::MinLength { .. }
                | sm::FacetFragment::MaxLength { .. } => {
                    todo!("Unsupported facet: {:?}", facet)
                }
            }
        }

        if enumerations.is_empty() {
            // Becoming a wrapping struct for the base type
            let min_inclusive =
                min_inclusive.map(|s| T::from_str(s).expect("Failed to parse min inclusive value"));
            let min_exclusive =
                min_exclusive.map(|s| T::from_str(s).expect("Failed to parse min exclusive value"));
            let max_inclusive =
                max_inclusive.map(|s| T::from_str(s).expect("Failed to parse max inclusive value"));
            let max_exclusive =
                max_exclusive.map(|s| T::from_str(s).expect("Failed to parse max exclusive value"));

            let struct_def = templates::wrapper_struct::WrapperStruct {
                struct_ident: ident.to_item_ident(),
                repr_type: T::repr_type(),
                repr: T::is_repr(),
                enum_with_mod: enum_with_ident,
            };

            let struct_item = struct_def.to_struct();
            let err = struct_def.try_from_impl(&error_ident);
            let into_impl = struct_def.into_impl();
            let with_mod = struct_def.with_mod();
            let enum_ty = scope.add_item(struct_item)?;
            scope.add_item(with_mod)?;
            scope.add_item(err)?;
            scope.add_raw_items([into_impl]);

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
                repr: T::is_repr(),
                enum_with_mod: format_ident!("test_enum_with"),
                value_to_pattern: |a| T::to_pattern_value(a),
            };

            let enum_item = enum_def.to_enum();
            let (err_items, try_from_impl) =
                enum_def.try_from_impl(&format_ident!("TestEnumParseError"));
            let into_impl = enum_def.into_impl();
            let with_mod = enum_def.with_mod();
            let enum_ty = scope.add_item(enum_item)?;
            scope.add_item(with_mod)?;
            scope.add_raw_items(err_items);
            scope.add_raw_items([try_from_impl, into_impl]);

            Ok(crate::ToTypeTemplateData {
                ident: None,
                template: enum_ty,
            })
        }
    }
}
