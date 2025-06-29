use quote::ToTokens;
use syn::{Item, ItemStruct};

use crate::augments::ItemAugmentation;

#[derive(Debug, Default)]
pub struct AdditionalDerives {
    pub structs: Vec<syn::Path>,
    pub enums: Vec<syn::Path>,
}

impl AdditionalDerives {
    pub fn augment_attributes(attrs: &mut Vec<syn::Attribute>, paths: &[syn::Path]) {
        let derive_attribute = attrs
            .iter_mut()
            .find(|attr| attr.path().is_ident("derive"))
            .unwrap();

        let list = match &mut derive_attribute.meta {
            syn::Meta::List(meta_list) => meta_list,
            _ => panic!("Expected a list"),
        };

        let existing_tokens = list.tokens.clone();

        // list.tokens.extend(quote::quote! {, ::bon::Builder });
        list.tokens.extend(
            paths
                .iter()
                .filter(|path| {
                    // Filter out paths that are already present in the derive list
                    !existing_tokens
                        .to_string()
                        .contains(&path.to_token_stream().to_string())
                })
                .map(|path| {
                    let path = path.to_token_stream();
                    quote::quote! {, #path }
                }),
        );
    }
}

impl ItemAugmentation for AdditionalDerives {
    fn augment_struct(&self, item: &mut ItemStruct) -> Vec<Item> {
        Self::augment_attributes(&mut item.attrs, &self.structs);

        Vec::new()
    }

    fn augment_enum(&self, item: &mut syn::ItemEnum) -> Vec<Item> {
        Self::augment_attributes(&mut item.attrs, &self.enums);

        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::augments::{additional_derives::AdditionalDerives, ItemAugmentation};

    #[test]
    fn bon_augmentation() {
        let mut item = syn::parse_quote! {
            #[derive(Debug, Clone)]
            pub struct Foo {
                bar: String,
                baz: Vec<String>,
            }
        };
        let augmentation = AdditionalDerives {
            structs: vec![syn::parse_quote! {PartialEq}],
            enums: vec![],
        };
        augmentation.augment_struct(&mut item);
        assert_eq!(
            item,
            syn::parse_quote! {
              #[derive(Debug, Clone, PartialEq)]
              pub struct Foo {
                  bar: String,
                  baz: Vec<String>,
              }
            }
        );
    }
}
