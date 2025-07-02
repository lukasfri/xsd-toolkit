use syn::Item;

use crate::{augments::ItemAugmentation, misc::unbox_type};

#[derive(Debug)]
pub struct StructFromAugmentation {}

impl StructFromAugmentation {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        StructFromAugmentation {}
    }
}

impl ItemAugmentation for StructFromAugmentation {
    fn augment_struct(&self, item: &mut syn::ItemStruct) -> Vec<Item> {
        let item_ident = &item.ident;

        let fields = match &item.fields {
            syn::Fields::Unnamed(fields_unnamed) => fields_unnamed,
            syn::Fields::Named(_) | syn::Fields::Unit => return Vec::new(),
        };

        if fields.unnamed.len() != 1 {
            return Vec::new();
        }

        let field = fields.unnamed.first().unwrap();

        let syn::Type::Path(field_ty) = &field.ty else {
            return Vec::new();
        };

        // Then, if field_ty is a boxed type, we want to create a From impl for the inner type instead of the boxed type.
        let impl_: syn::ItemImpl = if let Some(field_ty) = unbox_type(field_ty) {
            syn::parse_quote! {
                impl ::core::convert::From<#field_ty> for #item_ident {
                    fn from(value: #field_ty) -> Self {
                        #item_ident(::std::boxed::Box::new(value))
                    }
                }
            }
        } else {
            syn::parse_quote! {
                impl ::core::convert::From<#field_ty> for #item_ident {
                    fn from(value: #field_ty) -> Self {
                        #item_ident(value)
                    }
                }
            }
        };

        vec![impl_.into()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn struct_augmentation() {
        let mut item: syn::ItemStruct = syn::parse_quote! {
            #[derive(Debug, Clone)]
            pub struct A(String);
        };
        let original = item.clone();
        let augmentation = StructFromAugmentation::new();
        let augment_items = augmentation.augment_struct(&mut item);

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
            impl ::core::convert::From<String> for A {
                fn from(value: String) -> Self {
                    A(value)
                }
            }
        };

        assert_eq!(
            prettyplease::unparse(&actual),
            prettyplease::unparse(&expected)
        );
    }

    #[test]
    fn struct_boxed_augmentation() {
        let mut item: syn::ItemStruct = syn::parse_quote! {
            #[derive(Debug, Clone)]
            pub struct A(Box<String>);
        };
        let original = item.clone();
        let augmentation = StructFromAugmentation::new();
        let augment_items = augmentation.augment_struct(&mut item);
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
            impl ::core::convert::From<String> for A {
                fn from(value: String) -> Self {
                    A(::std::boxed::Box::new(value))
                }
            }
        };

        assert_eq!(
            prettyplease::unparse(&actual),
            prettyplease::unparse(&expected)
        );
    }
}
