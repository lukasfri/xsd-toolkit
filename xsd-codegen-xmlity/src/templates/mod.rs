pub mod element_record;
use proc_macro2::Span;
use syn::{LitStr, Token};

pub mod choice;

pub mod group_record;
pub mod value_record;

#[derive(Debug, Clone, Copy)]
pub enum ItemOrder {
    Strict,
    None,
}

impl ItemOrder {
    pub fn to_group_value(&self) -> syn::LitStr {
        match self {
            ItemOrder::Strict => LitStr::new("strict", Span::call_site()),
            ItemOrder::None => LitStr::new("none", Span::call_site()),
        }
    }

    pub fn to_item_value(&self) -> syn::LitStr {
        match self {
            ItemOrder::Strict => LitStr::new("loose", Span::call_site()),
            ItemOrder::None => LitStr::new("none", Span::call_site()),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum FieldType {
    Named,
    Unnamed,
}

#[derive(Debug, Clone, Copy)]
pub enum FieldMode {
    Struct,
    Variant,
}

impl FieldMode {
    pub fn to_vis(&self) -> syn::Visibility {
        match self {
            FieldMode::Struct => syn::Visibility::Public(<Token![pub]>::default()),
            FieldMode::Variant => syn::Visibility::Inherited,
        }
    }
}

fn derive_attribute(items: impl IntoIterator<Item = syn::Path>) -> syn::Attribute {
    let items = items.into_iter();
    syn::parse_quote!(#[derive(#(#items),*)])
}
