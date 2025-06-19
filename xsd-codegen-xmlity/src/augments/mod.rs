use std::fmt::Debug;

use syn::{Item, ItemEnum, ItemStruct};

mod bon;
pub use bon::BonAugmentation;
mod enum_from;
pub use enum_from::EnumFromAugmentation;

pub trait ItemAugmentation: Debug {
    fn augment_struct(&self, item: &mut ItemStruct) -> Vec<Item> {
        let _ = item;

        Vec::new()
    }

    fn augment_enum(&self, item: &mut ItemEnum) -> Vec<Item> {
        let _ = item;

        Vec::new()
    }
}

impl<T> ItemAugmentation for Option<T>
where
    T: ItemAugmentation,
{
    fn augment_struct(&self, item: &mut ItemStruct) -> Vec<Item> {
        match self {
            Some(augmentation) => augmentation.augment_struct(item),
            None => Vec::new(),
        }
    }

    fn augment_enum(&self, item: &mut ItemEnum) -> Vec<Item> {
        match self {
            Some(augmentation) => augmentation.augment_enum(item),
            None => Vec::new(),
        }
    }
}

pub trait ItemAugmentationExt: ItemAugmentation {
    fn augment_item(&self, item: &mut Item) -> Vec<Item> {
        match item {
            Item::Struct(item) => self.augment_struct(item),
            Item::Enum(item) => self.augment_enum(item),
            _ => Vec::new(),
        }
    }
}

impl<T: ItemAugmentation> ItemAugmentationExt for T {}

impl ItemAugmentation for &dyn ItemAugmentation {
    fn augment_struct(&self, item: &mut ItemStruct) -> Vec<Item> {
        (**self).augment_struct(item)
    }

    fn augment_enum(&self, item: &mut ItemEnum) -> Vec<Item> {
        (**self).augment_enum(item)
    }
}

impl ItemAugmentation for Box<dyn ItemAugmentation> {
    fn augment_struct(&self, item: &mut ItemStruct) -> Vec<Item> {
        (**self).augment_struct(item)
    }

    fn augment_enum(&self, item: &mut ItemEnum) -> Vec<Item> {
        (**self).augment_enum(item)
    }
}

impl<T: ItemAugmentation> ItemAugmentation for Vec<T> {
    fn augment_struct(&self, item: &mut ItemStruct) -> Vec<Item> {
        self.iter()
            .flat_map(|augmentation| augmentation.augment_struct(item))
            .collect::<Vec<_>>()
    }

    fn augment_enum(&self, item: &mut ItemEnum) -> Vec<Item> {
        self.iter()
            .flat_map(|augmentation| augmentation.augment_enum(item))
            .collect::<Vec<_>>()
    }
}

#[derive(Debug)]
pub struct NoopAugmentation {}

impl NoopAugmentation {
    pub fn new() -> Self {
        NoopAugmentation {}
    }
}

impl ItemAugmentation for NoopAugmentation {}
