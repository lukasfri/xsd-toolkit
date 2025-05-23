pub mod element_record;
use proc_macro2::Span;
use syn::LitStr;

pub mod choice;

pub mod group_record;
pub mod value_record;

#[derive(Debug, Clone, Copy)]
pub enum ItemOrder {
    Strict,
    None,
}

impl ItemOrder {
    pub fn to_order_value(&self) -> syn::LitStr {
        match self {
            ItemOrder::Strict => LitStr::new("strict", Span::call_site()),
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

fn derive_attribute(items: impl IntoIterator<Item = syn::Path>) -> syn::Attribute {
    let items = items.into_iter();
    syn::parse_quote!(#[derive(#(#items),*)])
}
