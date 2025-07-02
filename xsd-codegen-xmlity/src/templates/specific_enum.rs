use quote::format_ident;
use syn::{parse_quote, Ident};

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
                            write!(f, "Value '{:?}' does not exist in the enumeration", value)
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

pub struct TryFromDeserializeWith<'a> {
    pub repr_type: &'a syn::Type,
    pub final_type: &'a syn::Type,
}

impl TryFromDeserializeWith<'_> {
    pub fn deserialize_with_fn(
        &self,
        fn_ident: &Ident,
        text_checks: impl for<'a> FnOnce(&'a syn::Expr) -> Vec<syn::Stmt>,
    ) -> syn::ItemFn {
        let repr_type = &self.repr_type;
        let final_type = &self.final_type;

        let text_checks = text_checks(&parse_quote!(::std::string::String::as_str(&text)));

        parse_quote!(
            pub fn #fn_ident<'de, D>(deserializer: D) -> ::core::result::Result<#final_type, D::Error>
            where
                D: ::xmlity::Deserializer<'de>,
            {
                let text: ::std::string::String = ::xmlity::Deserialize::deserialize(deserializer)?;

                #(#text_checks)*

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

pub struct TryFromIntoWithMod<'a> {
    pub repr_type: &'a syn::Type,
    pub destination_type: &'a syn::Type,
    pub mod_name: &'a Ident,
}

impl<'a> TryFromIntoWithMod<'a> {
    fn deserialize_with_fn(
        &self,
        fn_ident: &Ident,
        text_checks: impl for<'b> FnOnce(&'b syn::Expr) -> Vec<syn::Stmt>,
    ) -> syn::ItemFn {
        TryFromDeserializeWith {
            repr_type: self.repr_type,
            final_type: self.destination_type,
        }
        .deserialize_with_fn(fn_ident, text_checks)
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
        let deserialize = self.deserialize_with_fn(&deserialize_ident, |_| Vec::new());

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

pub struct SpecificEnum<T: Fn(&syn::Expr) -> syn::Expr> {
    pub enum_ident: Ident,
    pub repr_type: syn::Type,
    pub enumerations: Vec<(Ident, syn::Pat, syn::Expr)>,
    pub enum_with_mod: syn::Ident,
    pub repr: bool,
    pub value_to_pattern: T,
}

impl<T: Fn(&syn::Expr) -> syn::Expr> SpecificEnum<T> {
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
        }
        .with_mod()
    }
}

// #[cfg(test)]
// mod tests {
//     use pretty_assertions::assert_eq;

//     use super::*;
//     use quote::format_ident;
//     use syn::{parse_quote, ItemEnum};
//     use xmlity::LocalName;

//     fn item_to_string<T: Into<syn::Item>>(item: T) -> String {
//         let item: syn::Item = item.into();
//         let file = syn::File {
//             shebang: None,
//             attrs: Vec::new(),
//             items: vec![item],
//         };
//         prettyplease::unparse(&file)
//     }

//     #[test]
//     fn generate_no_variant_choice() {
//         let record = Choice {
//             repr_type: parse_quote!(i32),
//             variants: Vec::new(),
//         };

//         todo!()
//         // let ident = format_ident!("Test");

//         // let actual_item = record.to_enum(&ident,
//         //     &parse_quote!(deserialize_with),
//         //     &parse_quote!(serialize_with),
//         // );

//         // let expected_item: ItemEnum = parse_quote!(
//         //     #[derive(::core::fmt::Debug, ::xmlity::Serialize, ::xmlity::Deserialize)]
//         //     #[repr(i32)]
//         //     pub enum Test {}
//         // );

//         // assert_eq!(expected_item, actual_item);
//     }

//     #[test]
//     fn generate_one_value_choice() {
//         let record = Choice {
//             repr_type: parse_quote!(i32),
//             variants: vec![(
//                 format_ident!("A"),
//                 ChoiceVariantType {
//                     value_expr: parse_quote!(1),
//                 },
//             )],
//         };

//         let ident = format_ident!("Test");
//         let deserialize_with_ident = format_ident!("deserialize_with");
//         let serialize_with_ident = format_ident!("serialize_with");

//         let actual_deserialize_fn =
//             record.deserialize_with_fn(&deserialize_with_ident, &parse_quote!(#ident), |_| {
//                 Vec::new()
//             });

//         // Signature: fn<'de, D>(D) -> Result<T, D::Error>
//         let expected_deserialize_fn: syn::ItemFn = parse_quote!(
//             pub fn deserialize_with<'de, D>(
//                 deserializer: D,
//             ) -> ::core::result::Result<Test, D::Error>
//             where
//                 D: ::xmlity::Deserializer<'de>,
//             {
//                 let text: ::std::string::String = ::xmlity::Deserialize::deserialize(deserializer)?;

//                 let value: i32 = text.parse().map_err(|_| {
//                     ::xmlity::de::Error::custom(format!("Failed to parse value for Test: {}", text))
//                 })?;

//                 match value {
//                     1 => Ok(Test::A),
//                     _ => Err(::xmlity::de::Error::custom(format!(
//                         "Unexpected value for Test: {}",
//                         value
//                     ))),
//                 }
//             }
//         );

//         assert_eq!(
//             item_to_string(expected_deserialize_fn),
//             item_to_string(actual_deserialize_fn)
//         );

//         let actual_serialize_fn =
//             record.serialize_with_fn(&serialize_with_ident, &parse_quote!(#ident));

//         // Signature: fn<S>(&T, S) -> Result<S::Ok, S::Error> where S: Serializer
//         let expected_serialize_fn: syn::ItemFn = parse_quote!(
//             pub fn serialize_with<S>(
//                 &value: &Test,
//                 serializer: S,
//             ) -> ::core::result::Result<S::Ok, S::Error>
//             where
//                 S: ::xmlity::Serializer,
//             {
//                 let value: i32 = match value {
//                     Test::A => 1,
//                 };

//                 ::xmlity::Serialize::serialize(
//                     ::std::string::ToString::to_string(&value),
//                     serializer,
//                 )
//             }
//         );

//         assert_eq!(
//             item_to_string(expected_serialize_fn),
//             item_to_string(actual_serialize_fn)
//         );

//         let actual_item = record.to_enum(
//             &ident,
//             &parse_quote!(#deserialize_with_ident),
//             &parse_quote!(#serialize_with_ident),
//         );

//         let expected_item: ItemEnum = parse_quote!(
//             #[derive(::core::fmt::Debug, ::xmlity::Serialize, ::xmlity::Deserialize)]
//             #[xvalue(deserialize_with = #deserialize_with_ident, serialize_with = #serialize_with_ident)]
//             pub enum Test {
//                 A,
//             }
//         );

//         assert_eq!(item_to_string(expected_item), item_to_string(actual_item));
//     }
// }
