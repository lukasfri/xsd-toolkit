use quote::ToTokens;
use syn::{Item, ItemStruct};

use crate::augments::ItemAugmentation;

#[derive(Debug)]
pub struct BonAugmentation {}

impl BonAugmentation {
    pub fn new() -> Self {
        BonAugmentation {}
    }
}

impl ItemAugmentation for BonAugmentation {
    fn augment_struct(&self, item: &mut ItemStruct) -> Vec<Item> {
        // Bon only works on structs with named fields
        let fields = match &mut item.fields {
            syn::Fields::Named(fields_named) => fields_named,
            syn::Fields::Unnamed(_) | syn::Fields::Unit => return Vec::new(),
        };

        let derive_attribute = item
            .attrs
            .iter_mut()
            .find(|attr| attr.path().is_ident("derive"))
            .unwrap();

        let list = match &mut derive_attribute.meta {
            syn::Meta::List(meta_list) => meta_list,
            _ => panic!("Expected a list"),
        };

        list.tokens.extend(quote::quote! {, ::bon::Builder });

        for field in fields.named.iter_mut() {
            if let syn::Type::Path(type_path) = &field.ty {
                let ty = type_path.to_token_stream().to_string();

                if ty.starts_with("Vec")
                    || ty.starts_with("std::vec::Vec")
                    || ty.starts_with("::std::vec::Vec")
                {
                    field.attrs.push(syn::parse_quote! {
                        #[builder(default)]
                    });
                }
            }
        }

        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::augments::{BonAugmentation, ItemAugmentation};

    #[test]
    fn bon_augmentation() {
        let mut item = syn::parse_quote! {
            #[derive(Debug, Clone)]
            pub struct Foo {
                bar: String,
                baz: Vec<String>,
            }
        };
        let augmentation = BonAugmentation::new();
        augmentation.augment_struct(&mut item);
        assert_eq!(
            item,
            syn::parse_quote! {
                #[derive(Debug, Clone, ::bon::Builder)]
                pub struct Foo {
                    bar: String,
                    #[builder(default)]
                    baz: Vec<String>,
                }
            }
        );
    }
}
