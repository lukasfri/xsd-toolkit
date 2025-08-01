use syn::{parse_quote, Ident, Item, ItemMod};

use crate::{
    augments::{self, ItemAugmentationExt},
    misc::{finish_mod, TypeReference},
    Error, Result, Scope,
};

#[derive(Debug)]
pub struct GeneratorScope<'a> {
    items: Vec<Item>,
    augmentation: &'a dyn augments::ItemAugmentation,
}

impl Scope for GeneratorScope<'_> {
    fn add_item<I: Into<Item>>(&mut self, item: I) -> Result<TypeReference<'static>> {
        let mut item: Item = item.into();

        self.items.extend(self.augmentation.augment_item(&mut item));

        let ident = match &item {
            Item::Struct(item) => &item.ident,
            Item::Enum(item) => &item.ident,
            Item::Mod(item) => &item.ident,
            _ => {
                return Err(Error::UnsupportedItemType {
                    item_type: format!("{:?}", std::mem::discriminant(&item)),
                })
            }
        };

        let ref_ = TypeReference::new_prefixed_type(parse_quote!(#ident));

        self.items.push(item);

        Ok(ref_)
    }

    fn add_raw_items<I: IntoIterator<Item = J>, J: Into<syn::Item>>(&mut self, items: I) {
        self.items.extend(items.into_iter().map(Into::into));
    }

    fn augmenter(&self) -> &dyn augments::ItemAugmentation {
        self.augmentation
    }
}

impl<'a> GeneratorScope<'a> {
    pub fn new(augmentation: &'a dyn augments::ItemAugmentation) -> Self {
        Self {
            items: Vec::new(),
            augmentation,
        }
    }

    pub fn finish(self) -> Vec<Item> {
        self.items
    }

    pub fn finish_mod(self, mod_name: &Ident) -> Option<ItemMod> {
        finish_mod(mod_name, self.items)
    }
}
