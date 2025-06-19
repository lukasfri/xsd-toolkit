use std::{collections::HashMap, ops::AddAssign};

use syn::Item;

use crate::{augments::ItemAugmentation, misc::unbox_type};

#[derive(Debug)]
pub struct EnumFromAugmentation {}

impl EnumFromAugmentation {
    pub fn new() -> Self {
        EnumFromAugmentation {}
    }
}

impl ItemAugmentation for EnumFromAugmentation {
    fn augment_enum(&self, item: &mut syn::ItemEnum) -> Vec<Item> {
        let mut impls = Vec::new();

        // We first iterate through and get a collection of those types that have multiple variants associated with them. Those cannot be implemented.
        let mut ty_exists = HashMap::<syn::Type, usize>::new();
        for variant in item.variants.iter() {
            let fields = match &variant.fields {
                syn::Fields::Unnamed(fields_unnamed) => fields_unnamed,
                syn::Fields::Named(_) | syn::Fields::Unit => continue,
            };

            if fields.unnamed.len() != 1 {
                continue;
            }

            ty_exists
                .entry(fields.unnamed.first().unwrap().ty.clone())
                .or_default()
                .add_assign(1);
        }

        let item_ident = &item.ident;

        for variant in item.variants.iter() {
            let variant_ident = &variant.ident;

            let fields = match &variant.fields {
                syn::Fields::Unnamed(fields_unnamed) => fields_unnamed,
                syn::Fields::Named(_) | syn::Fields::Unit => continue,
            };

            if fields.unnamed.len() != 1 {
                continue;
            }

            let field = fields.unnamed.first().unwrap();

            if *ty_exists.get(&field.ty).unwrap() > 1 {
                continue;
            }

            let syn::Type::Path(field_ty) = &field.ty else {
                continue;
            };

            // Then, if field_ty is a boxed type, we want to create a From impl for the inner type instead of the boxed type.
            let impl_: syn::ItemImpl = if let Some(field_ty) = unbox_type(&field_ty) {
                syn::parse_quote! {
                    impl ::core::convert::From<#field_ty> for #item_ident {
                        fn from(value: #field_ty) -> Self {
                            #item_ident::#variant_ident(::std::boxed::Box::new(value))
                        }
                    }
                }
            } else {
                syn::parse_quote! {
                    impl ::core::convert::From<#field_ty> for #item_ident {
                        fn from(value: #field_ty) -> Self {
                            #item_ident::#variant_ident(value)
                        }
                    }
                }
            };

            impls.push(impl_.into());
        }

        impls
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use syn::parse_quote;

    #[test]
    fn enum_augmentation() {
        let mut item: syn::ItemEnum = syn::parse_quote! {
            #[derive(Debug, Clone)]
            pub enum Enum {
                A(String),
                B(i32),
            }
        };
        let original = item.clone();
        let augmentation = EnumFromAugmentation::new();
        let augment_items = augmentation.augment_enum(&mut item);

        assert_eq!(
            item, original,
            "Augmentation should not modify the original item"
        );

        let actual = syn::File {
            shebang: None,
            attrs: Vec::new(),
            items: augment_items,
        };

        let expected: syn::File = syn::parse_quote! {
            impl ::core::convert::From<String> for Enum {
                fn from(value: String) -> Self {
                    Enum::A(value)
                }
            }
            impl ::core::convert::From<i32> for Enum {
                fn from(value: i32) -> Self {
                    Enum::B(value)
                }
            }
        };

        assert_eq!(
            prettyplease::unparse(&actual),
            prettyplease::unparse(&expected)
        );
    }

    #[test]
    fn unbox() {
        assert_eq!(
            unbox_type(&parse_quote!(::std::boxed::Box<i32>)),
            Option::<syn::Type>::Some(parse_quote!(i32))
        );
        assert_eq!(
            unbox_type(&parse_quote!(Box<i32>)),
            Option::<syn::Type>::Some(parse_quote!(i32))
        );
        assert_eq!(unbox_type(&parse_quote!(i32)), None);
    }

    #[test]
    fn enum_boxed_augmentation() {
        let mut item: syn::ItemEnum = syn::parse_quote! {
            #[derive(Debug, Clone)]
            pub enum Enum {
                A(Box<String>),
                B(Box<i32>),
            }
        };
        let original = item.clone();
        let augmentation = EnumFromAugmentation::new();
        let augment_items = augmentation.augment_enum(&mut item);
        assert_eq!(
            item, original,
            "Augmentation should not modify the original item"
        );

        let actual = syn::File {
            shebang: None,
            attrs: Vec::new(),
            items: augment_items,
        };

        let expected: syn::File = syn::parse_quote! {
            impl ::core::convert::From<String> for Enum {
                fn from(value: String) -> Self {
                    Enum::A(::std::boxed::Box::new(value))
                }
            }

            impl ::core::convert::From<i32> for Enum {
                fn from(value: i32) -> Self {
                    Enum::B(::std::boxed::Box::new(value))
                }
            }
        };

        assert_eq!(
            prettyplease::unparse(&actual),
            prettyplease::unparse(&expected)
        );
    }

    #[test]
    fn enum_duplicate_variants() {
        let mut item: syn::ItemEnum = syn::parse_quote! {
            #[derive(Debug, Clone)]
            pub enum Enum {
                A(String),
                B(i32),
                C(i32),
            }
        };
        let original = item.clone();
        let augmentation = EnumFromAugmentation::new();
        let augment_items = augmentation.augment_enum(&mut item);
        assert_eq!(
            item, original,
            "Augmentation should not modify the original item"
        );

        let actual = syn::File {
            shebang: None,
            attrs: Vec::new(),
            items: augment_items,
        };

        let expected: syn::File = syn::parse_quote! {
            impl ::core::convert::From<String> for Enum {
                fn from(value: String) -> Self {
                    Enum::A(value)
                }
            }
        };

        assert_eq!(
            prettyplease::unparse(&actual),
            prettyplease::unparse(&expected)
        );
    }
}
