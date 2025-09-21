use quote::format_ident;
use syn::{parse_quote, Ident, Stmt};

pub enum BoundEdge {
    Inclusive,
    Exclusive,
}

// pub struct OrderedFilter<T: ToTokens> {
//     pub repr_type: syn::Type,
//     /// TODO: Review
//     /// This is relevant for partially ordered types such as `f32`, where `a < b` does not imply `!(a > b)`.
//     allow_non_ordered: bool,
//     pub min: Option<(BoundEdge, T)>,
//     pub max: Option<(BoundEdge, T)>,
// }

// impl<T: ToTokens> OrderedFilter<T> {
//     pub fn new(repr_type: syn::Type) -> Self {
//         Self {
//             repr_type,
//             allow_non_ordered: false,
//             min: None,
//             max: None,
//         }
//     }

//     pub fn allow_non_ordered(mut self, allow: bool) -> Self {
//         self.allow_non_ordered = allow;
//         self
//     }

//     pub fn with_min(mut self, edge: BoundEdge, value: T) -> Self {
//         self.min = Some((edge, value));
//         self
//     }

//     pub fn with_max(mut self, edge: BoundEdge, value: T) -> Self {
//         self.max = Some((edge, value));
//         self
//     }

//     pub fn check_stmts(&self, error_path: &syn::Type, value_expr: &syn::Expr) -> Vec<syn::ExprIf> {
//         // TODO: Handle non-ordered types
//         let min: Option<syn::ExprIf> = self.min.as_ref().map(|(edge, value)| {
//             let min_error_variant = match edge {
//                 BoundEdge::Inclusive => format_ident!("MinInclusive"),
//                 BoundEdge::Exclusive => format_ident!("MinExclusive"),
//             };

//             let condition: syn::Expr = match edge {
//                 BoundEdge::Inclusive => parse_quote!(#value_expr < #value),
//                 BoundEdge::Exclusive => parse_quote!(#value_expr <= #value),
//             };

//             parse_quote!(
//                 if #condition {
//                     return Err(#error_path::#min_error_variant {
//                         limit: #value,
//                         value: #value_expr,
//                     });
//                 }
//             )
//         });

//         let max: Option<syn::ExprIf> = self.max.as_ref().map(|(edge, value)| {
//             let max_error_variant = match edge {
//                 BoundEdge::Inclusive => format_ident!("MaxInclusive"),
//                 BoundEdge::Exclusive => format_ident!("MaxExclusive"),
//             };

//             let condition: syn::Expr = match edge {
//                 BoundEdge::Inclusive => parse_quote!(#value_expr > #value),
//                 BoundEdge::Exclusive => parse_quote!(#value_expr >= #value),
//             };

//             parse_quote!(
//                 if #condition {
//                     return Err(#error_path::#max_error_variant {
//                         limit: #value,
//                         value: #value_expr,
//                     });
//                 }
//             )
//         });

//         min.into_iter().chain(max.into_iter()).collect()
//     }

//     pub fn error_variants(&self) -> Vec<syn::Variant> {
//         let mut variants = Vec::<syn::Variant>::new();
//         let repr_type = &self.repr_type;

//         if let Some((edge, _)) = &self.min {
//             let ident = match edge {
//                 BoundEdge::Inclusive => format_ident!("MinInclusive"),
//                 BoundEdge::Exclusive => format_ident!("MinExclusive"),
//             };
//             variants.push(parse_quote!(
//                 #ident {
//                     limit: #repr_type,
//                     value: #repr_type,
//                 }
//             ));
//         }

//         if let Some((edge, _)) = &self.max {
//             let ident = match edge {
//                 BoundEdge::Inclusive => format_ident!("MaxInclusive"),
//                 BoundEdge::Exclusive => format_ident!("MaxExclusive"),
//             };
//             variants.push(parse_quote!(
//                 #ident {
//                     limit: #repr_type,
//                     value: #repr_type,
//                 }
//             ));
//         }

//         variants
//     }
// }

pub struct TryFromEnum<
    'a,
    T: IntoIterator<Item = (&'a Ident, &'a syn::Pat)>,
    F: FnOnce(&syn::Expr) -> syn::Expr,
> {
    pub repr_type: &'a syn::Type,
    pub enumerations: T,
    pub value_to_pattern: F,
}

impl<'a, T: IntoIterator<Item = (&'a Ident, &'a syn::Pat)>, F: FnOnce(&syn::Expr) -> syn::Expr>
    TryFromEnum<'a, T, F>
{
    pub fn to_error_type(&self, ident: &'_ Ident) -> Vec<syn::Item> {
        let repr_type = &self.repr_type;

        let enum_: syn::ItemEnum = parse_quote!(
            #[derive(::core::fmt::Debug)]
            pub enum #ident {
                NonExistent {
                    value: #repr_type,
                },
            }
        );

        let error_display: syn::ItemImpl = parse_quote!(
            impl ::core::fmt::Display for #ident {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::result::Result<(), ::core::fmt::Error> {
                    match self {
                        #ident::NonExistent { value } => {
                            write!(f, "Value '{value:?}' does not exist in the enumeration")
                        }
                    }
                }
            }
        );

        vec![enum_.into(), error_display.into()]
    }

    pub fn to_impl(self, enum_type: &syn::Type, error_path: &syn::Type) -> syn::ItemImpl {
        let repr_type = &self.repr_type;
        let arms = self
            .enumerations
            .into_iter()
            .map(|(ident, value)| -> syn::Arm { parse_quote!(#value => Ok(#enum_type::#ident)) });

        let value_expr = parse_quote!(value);
        let value_pattern_expr = (self.value_to_pattern)(&value_expr);

        parse_quote!(
            impl ::core::convert::TryFrom<#repr_type> for #enum_type {
                type Error = #error_path;

                fn try_from(value: #repr_type) -> ::core::result::Result<Self, Self::Error> {
                    match #value_pattern_expr {
                        #(#arms,)*
                        _ => Err(#error_path::NonExistent {
                            value,
                        }),
                    }
                }
            }
        )
    }
}

pub struct EnumInto<'a, T: IntoIterator<Item = (&'a Ident, &'a syn::Expr)>> {
    pub repr_type: &'a syn::Type,
    pub enumerations: T,
}

impl<'a, T: IntoIterator<Item = (&'a Ident, &'a syn::Expr)>> EnumInto<'a, T> {
    pub fn to_impl(self, enum_type: &syn::Type) -> syn::ItemImpl {
        let repr_type = &self.repr_type;
        let arms = self
            .enumerations
            .into_iter()
            .map(|(ident, value)| -> syn::Arm { parse_quote!(#enum_type::#ident => #value) });

        parse_quote!(
            impl ::core::convert::From<#enum_type> for #repr_type {
                fn from(value: #enum_type) -> Self {
                    match value {
                        #(#arms,)*
                    }
                }
            }
        )
    }
}

// pub struct NumericTryFrom<'a, T: ToTokens> {
//     pub repr_type: &'a syn::Type,
//     pub ordered_filter: OrderedFilter<T>,
// }

// impl<T: ToTokens> NumericTryFrom<'_, T> {
//     pub fn to_error_type(&self, ident: &'_ Ident) -> syn::ItemEnum {
//         let ordered_errors = self.ordered_filter.error_variants();

//         parse_quote!(
//             #[derive(::core::fmt::Debug)]
//             pub enum #ident {
//                 #(#ordered_errors,)*
//             }
//         )
//     }
//     pub fn to_impl(
//         &self,
//         type_: &syn::Type,
//         value_constructor: impl FnOnce(&syn::Expr) -> syn::Expr,
//     ) -> syn::ItemImpl {
//         let repr_type = &self.repr_type;

//         let ordered_filter_checks = &self
//             .ordered_filter
//             .check_stmts(&parse_quote!(Self::Error), &parse_quote!(value));

//         let value_constructor = value_constructor(&parse_quote!(value));

//         parse_quote!(
//             impl ::core::convert::TryFrom<#repr_type> for #type_ {
//                 type Error = ::xmlity::de::Error;

//                 fn try_from(value: #repr_type) -> ::core::result::Result<Self, Self::Error> {
//                     #(#ordered_filter_checks)*

//                     Ok(##value_constructor)
//                 }
//             }
//         )
//     }
// }

pub struct TryFromDeserializeWith<'a, F: Fn(&syn::Expr) -> Vec<Stmt>> {
    pub repr_type: &'a syn::Type,
    pub final_type: &'a syn::Type,
    pub text_checks: F,
    pub expr_if_empty: Option<&'a syn::Expr>,
}

pub fn text_checks_default(_: &syn::Expr) -> Vec<syn::Stmt> {
    vec![]
}

impl<F: Fn(&syn::Expr) -> Vec<Stmt>> TryFromDeserializeWith<'_, F> {
    pub fn deserialize_with_fn(&self, fn_ident: &Ident) -> syn::ItemFn {
        let repr_type = &self.repr_type;
        let final_type = &self.final_type;

        let text_checks = (self.text_checks)(&parse_quote!(::std::string::String::as_str(&text)));
        let expr_if_empty: Option<syn::ExprIf> = self.expr_if_empty.as_ref().map(|e| {
            parse_quote!(
                if text.is_empty() {
                    return #final_type::try_from(#e).map_err(::xmlity::de::Error::custom);
                }
            )
        });

        parse_quote!(
            pub fn #fn_ident<'de, D>(deserializer: D) -> ::core::result::Result<#final_type, D::Error>
            where
                D: ::xmlity::Deserializer<'de>,
            {
                let text: ::std::string::String = ::xmlity::Deserialize::deserialize(deserializer)?;

                #(#text_checks)*

                #expr_if_empty

                let value: #repr_type = text.parse().map_err(::xmlity::de::Error::custom)?;

                #final_type::try_from(value).map_err(::xmlity::de::Error::custom)
            }
        )
    }
}

pub struct IntoSerializeWith<'a> {
    pub origin_type: &'a syn::Type,
    pub repr_type: &'a syn::Type,
}

impl IntoSerializeWith<'_> {
    pub fn serialize_with_fn(&self, fn_ident: &Ident) -> syn::ItemFn {
        let origin_type = &self.origin_type;
        let repr_type = &self.repr_type;

        parse_quote!(
            pub fn #fn_ident<S>(value: &#origin_type, serializer: S) -> ::core::result::Result<S::Ok, S::Error>
            where
                S: ::xmlity::Serializer,
            {
                let value: #repr_type = ::core::clone::Clone::clone(value).into();

                ::xmlity::Serialize::serialize(::std::string::String::as_str(&::std::string::ToString::to_string(&value)), serializer)
            }
        )
    }
}

pub struct TryFromIntoWithMod<'a, F> {
    pub repr_type: &'a syn::Type,
    pub destination_type: &'a syn::Type,
    pub mod_name: &'a Ident,
    pub text_checks: F,
    pub expr_if_empty: Option<&'a syn::Expr>,
}

impl<'a, F: Fn(&syn::Expr) -> Vec<Stmt>> TryFromIntoWithMod<'a, F> {
    fn deserialize_with_fn(&self, fn_ident: &Ident) -> syn::ItemFn {
        TryFromDeserializeWith {
            repr_type: self.repr_type,
            final_type: self.destination_type,
            text_checks: &self.text_checks,
            expr_if_empty: self.expr_if_empty,
        }
        .deserialize_with_fn(fn_ident)
    }

    fn serialize_with_fn(&self, fn_ident: &Ident) -> syn::ItemFn {
        IntoSerializeWith {
            origin_type: self.destination_type,
            repr_type: self.repr_type,
        }
        .serialize_with_fn(fn_ident)
    }

    pub fn with_mod(&self) -> syn::ItemMod {
        let deserialize_ident = format_ident!("deserialize");
        let deserialize = self.deserialize_with_fn(&deserialize_ident);

        let serialize_ident = format_ident!("serialize");
        let serialize = self.serialize_with_fn(&serialize_ident);

        let enum_with_mod = &self.mod_name;
        parse_quote!(
            pub mod #enum_with_mod {
                #deserialize

                #serialize
            }
        )
    }
}

pub struct SpecificEnum<T: Fn(&syn::Expr) -> syn::Expr, F: Fn(&syn::Expr) -> Vec<Stmt>> {
    pub enum_ident: Ident,
    pub repr_type: syn::Type,
    pub enumerations: Vec<(Ident, syn::Pat, syn::Expr)>,
    pub enum_with_mod: syn::Ident,
    pub repr: bool,
    pub value_to_pattern: T,
    pub text_checks: F,
    pub expr_if_empty: Option<syn::Expr>,
}

impl<T: Fn(&syn::Expr) -> syn::Expr, F: Fn(&syn::Expr) -> Vec<Stmt>> SpecificEnum<T, F> {
    pub fn option_attributes(&self) -> impl Iterator<Item = syn::Meta> {
        let enum_with_mod = &self.enum_with_mod;
        Some(parse_quote! { with = #enum_with_mod }).into_iter()
    }

    fn value_attr(&self) -> syn::Attribute {
        let options = self.option_attributes();
        parse_quote!(#[xvalue(#(#options),*)])
    }

    pub fn to_enum(&self) -> syn::ItemEnum {
        let enum_ident = &self.enum_ident;
        let repr_type = &self.repr_type;
        let variants = self
            .enumerations
            .iter()
            .map(|(ident, _, value)| -> syn::Variant {
                if self.repr {
                    parse_quote!(#ident = #value)
                } else {
                    parse_quote!(#ident)
                }
            });
        let attr = self.value_attr();
        let repr_attr = self
            .repr
            .then(|| -> syn::Attribute { parse_quote!(#[repr(#repr_type)]) });

        parse_quote!(
            #[derive(::core::fmt::Debug, ::core::clone::Clone, ::core::marker::Copy, ::xmlity::Serialize, ::xmlity::Deserialize)]
            #attr
            #repr_attr
            pub enum #enum_ident {
                #(#variants,)*
            }
        )
    }

    pub fn try_from_impl(&self, error_ident: &Ident) -> (Vec<syn::Item>, syn::ItemImpl) {
        let impl_ = TryFromEnum {
            repr_type: &self.repr_type,
            enumerations: self.enumerations.iter().map(|(ident, pat, _)| (ident, pat)),
            value_to_pattern: &self.value_to_pattern,
        };

        let enum_ident = &self.enum_ident;

        (
            impl_.to_error_type(error_ident),
            impl_.to_impl(&parse_quote!(#enum_ident), &parse_quote!(#error_ident)),
        )
    }

    pub fn into_impl(&self) -> syn::ItemImpl {
        let enum_ident = &self.enum_ident;

        EnumInto {
            repr_type: &self.repr_type,
            enumerations: self
                .enumerations
                .iter()
                .map(|(ident, _, value)| (ident, value)),
        }
        .to_impl(&parse_quote!(#enum_ident))
    }

    pub fn with_mod(&self) -> syn::ItemMod {
        let enum_ident = &self.enum_ident;

        TryFromIntoWithMod {
            repr_type: &self.repr_type,
            destination_type: &parse_quote!(super::#enum_ident),
            mod_name: &self.enum_with_mod,
            text_checks: &self.text_checks,
            expr_if_empty: self.expr_if_empty.as_ref(),
        }
        .with_mod()
    }
}
