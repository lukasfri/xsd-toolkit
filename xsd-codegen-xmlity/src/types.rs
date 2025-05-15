//! This module provides an API for creating the structures demanded by the XMLity derive API.

use std::iter;

use proc_macro2::Span;
use quote::format_ident;
use syn::{parse_quote, Field, Ident, Item, LitStr, Path, Token};
use xmlity::ExpandedName;

use crate::misc::TypeReference;

#[derive(Debug)]
pub struct TypeChoice {
    pub variants: Vec<(Ident, TypeRecord)>,
}

impl TypeChoice {
    pub fn into_enum(
        &self,
        ident: &Ident,
        path: Option<&Path>,
    ) -> (TypeReference<'static>, syn::ItemEnum, Vec<Item>) {
        let (variants, associated_items): (Vec<_>, Vec<_>) = self
            .variants
            .iter()
            .map(|(variant_ident, variant_type)| variant_type.into_variant(variant_ident, path))
            .unzip();

        let associated_items: Vec<_> = associated_items.into_iter().flatten().collect();

        let enum_ = parse_quote!(
            #[derive(Debug, xmlity::Serialize, xmlity::Deserialize)]
            pub enum #ident {
              #(#variants),*
            }
        );

        let type_ = TypeReference::new_prefix(parse_quote!(#ident));

        (type_, enum_, associated_items)
    }
}

#[derive(Debug)]
pub struct Attribute {}

#[derive(Debug)]
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

pub enum FieldMode {
    Struct,
    Enum,
}

#[derive(Debug)]
pub struct Items {
    pub order: ItemOrder,
    pub items: Vec<(Ident, TypeRecord)>,
}

impl Items {
    fn fields<'a>(
        &'a self,
        mode: FieldMode,
        path: Option<&'a Path>,
    ) -> impl Iterator<Item = (Field, Vec<Item>)> + use<'a> {
        let vis = match mode {
            FieldMode::Struct => syn::Visibility::Public(<Token![pub]>::default()),
            FieldMode::Enum => syn::Visibility::Inherited,
        };

        self.items.iter().enumerate().zip(iter::repeat(vis)).map(
            move |((i, (field_ident, item)), vis)| {
                let ident = format_ident!("Item{i}");

                let (type_, items) = item.into_type(&ident, None);
                let type_ = type_.into_type(path);

                let field = parse_quote!(
                  #vis #field_ident: #type_
                );

                (field, items)
            },
        )
    }

    pub fn into_variant(&self, ident: &Ident, path: Option<&Path>) -> (syn::Variant, Vec<Item>) {
        let (fields, associated_types): (Vec<_>, Vec<_>) =
            self.fields(FieldMode::Enum, path).unzip();
        let children_order = self.order.to_order_value();

        let variant = parse_quote!(
          #[xvalue(children_order = #children_order)]
          #ident {
            #(#fields),*
          }
        );

        (variant, associated_types.into_iter().flatten().collect())
    }

    pub fn into_struct(
        &self,
        ident: &Ident,
        path: Option<&Path>,
    ) -> (TypeReference<'static>, syn::ItemStruct, Vec<Item>) {
        let (fields, associated_types): (Vec<_>, Vec<_>) =
            self.fields(FieldMode::Struct, path).unzip();
        let children_order = self.order.to_order_value();

        let struct_ = parse_quote!(
          #[derive(Debug, xmlity::Serialize, xmlity::Deserialize)]
          #[xvalue(children_order = #children_order)]
          pub struct #ident {
            #(#fields),*
          }
        );
        let type_ = TypeReference::new_prefix(parse_quote!(#ident));

        (
            type_,
            struct_,
            associated_types.into_iter().flatten().collect(),
        )
    }
}

#[derive(Debug)]
pub struct ElementRecord {
    pub name: ExpandedName<'static>,
    pub attribute_order: ItemOrder,
    pub attributes: Vec<(Ident, Attribute)>,
    pub children: Items,
}

impl ElementRecord {
    fn fields<'a>(
        &'a self,
        mode: FieldMode,
        path: Option<&'a Path>,
    ) -> impl Iterator<Item = (Field, Vec<Item>)> + use<'a> {
        let vis = match mode {
            FieldMode::Struct => syn::Visibility::Public(<Token![pub]>::default()),
            FieldMode::Enum => syn::Visibility::Inherited,
        };

        let attributes =
            self.attributes
                .iter()
                .zip(iter::repeat(vis))
                .map(|((field_ident, _attr), vis)| {
                    todo!();
                    let type_: syn::Type = parse_quote!(i32);

                    let field = parse_quote!(
                      #vis #field_ident: #type_
                    );

                    (field, vec![])
                });

        let children = self.children.fields(mode, path);

        attributes.chain(children)
    }

    pub fn into_variant(&self, ident: &Ident, path: Option<&Path>) -> (syn::Variant, Vec<Item>) {
        let attribute_order = self.attribute_order.to_order_value();
        let children_order = self.children.order.to_order_value();

        let (fields, associated_types): (Vec<_>, Vec<_>) =
            self.fields(FieldMode::Enum, path).unzip();

        let name = self.name.local_name().to_string();
        let namespace = self.name.namespace().unwrap().to_string();

        let type_ = parse_quote!(
          #[xelement(name = #name, namespace = #namespace, attribute_order = #attribute_order, children_order = #children_order)]
          #ident {
            #(#fields),*
          }
        );

        (type_, associated_types.into_iter().flatten().collect())
    }

    pub fn into_struct(
        &self,
        ident: &Ident,
        path: Option<&Path>,
    ) -> (TypeReference<'static>, syn::ItemStruct, Vec<Item>) {
        let attribute_order = self.attribute_order.to_order_value();
        let children_order = self.children.order.to_order_value();

        let (fields, associated_types): (Vec<_>, Vec<_>) =
            self.fields(FieldMode::Struct, path).unzip();

        let name = self.name.local_name().to_string();
        let namespace = self.name.namespace().unwrap().to_string();

        let struct_def = parse_quote!(
          #[derive(Debug, xmlity::Serialize, xmlity::Deserialize)]
          #[xelement(name = #name, namespace = #namespace,attribute_order = #attribute_order, children_order = #children_order)]
          pub struct #ident {
            #(#fields),*
          }
        );

        (
            TypeReference::new_prefix(parse_quote!(#ident)),
            struct_def,
            associated_types.into_iter().flatten().collect(),
        )
    }
}

#[derive(Debug, Clone)]
pub struct TypeItem {
    pub items: Vec<Item>,
    pub type_: TypeReference<'static>,
}

impl TypeItem {
    pub fn new(type_: TypeReference<'static>) -> Self {
        Self {
            items: Vec::new(),
            type_,
        }
    }
}

#[derive(Debug)]
pub enum TypeRecord {
    Items(Items),
    Element(ElementRecord),
    Choice(TypeChoice),
    Item(TypeItem),
}

impl TypeRecord {
    pub fn into_variant(&self, ident: &Ident, path: Option<&Path>) -> (syn::Variant, Vec<Item>) {
        let (_type, associated_items): (syn::Type, Vec<Item>) = match self {
            TypeRecord::Items(items) => return items.into_variant(ident, path),
            TypeRecord::Element(element_record) => return element_record.into_variant(ident, path),
            TypeRecord::Choice(type_choice) => {
                let (type_, item, mut items) = type_choice.into_enum(ident, path);
                items.push(Item::Enum(item));

                (parse_quote!(#ident), items)
            }
            TypeRecord::Item(type_) => {
                let TypeItem { items, type_ } = type_.clone();

                (type_.into_type(path), items)
            }
        };

        let variant: syn::Variant = parse_quote!(
        #ident(#_type)
        );

        (variant, associated_items)
    }

    pub fn into_type(
        &self,
        ident: &Ident,
        path: Option<&Path>,
    ) -> (TypeReference<'static>, Vec<Item>) {
        match self {
            TypeRecord::Items(items) => {
                let (type_, item, mut associated_items) = items.into_struct(ident, path);
                associated_items.push(Item::Struct(item));
                (type_, associated_items)
            }
            TypeRecord::Element(element_record) => {
                let (type_, item, mut associated_items) = element_record.into_struct(ident, path);
                associated_items.push(Item::Struct(item));
                (type_, associated_items)
            }
            TypeRecord::Choice(type_choice) => {
                let (type_, item, mut associated_items) = type_choice.into_enum(ident, path);
                associated_items.push(Item::Enum(item));

                (type_, associated_items)
            }
            TypeRecord::Item(item) => (item.type_.clone(), item.items.clone()),
        }
    }
}

#[cfg(test)]
mod tests {
    use quote::format_ident;
    use syn::File;
    use xmlity::{LocalName, XmlNamespace};

    use super::*;

    #[test]
    fn test_1() {
        let type_ = ElementRecord {
            name: ExpandedName::new(
                LocalName::new_dangerous("test-localname"),
                Some(XmlNamespace::new_dangerous("http://localhost")),
            ),
            attribute_order: ItemOrder::None,
            attributes: vec![],
            children: Items {
                order: ItemOrder::Strict,
                items: vec![],
            },
        };

        let (_, struct_item, associated_types) =
            type_.into_struct(&format_ident!("TestLocalname"), None);

        let file: File = parse_quote!(
            #(#associated_types)*

            #struct_item
        );

        let code = prettyplease::unparse(&file);

        println!("{code}");
    }

    #[test]
    fn test_2() {
        let type_ = ElementRecord {
            name: ExpandedName::new(
                LocalName::new_dangerous("test-localname"),
                Some(XmlNamespace::new_dangerous("http://localhost")),
            ),
            attribute_order: ItemOrder::None,
            attributes: vec![],
            children: Items {
                order: ItemOrder::Strict,
                items: vec![
                    (
                        format_ident!("content"),
                        TypeRecord::Items(Items {
                            order: ItemOrder::None,
                            items: vec![],
                        }),
                    ),
                    (
                        format_ident!("other"),
                        TypeRecord::Choice(TypeChoice { variants: vec![] }),
                    ),
                ],
            },
        };

        let (_, item_struct, associated_types) =
            type_.into_struct(&format_ident!("TestLocalname"), None);

        let file: File = parse_quote!(
            #(#associated_types)*

            #item_struct
        );

        let code = prettyplease::unparse(&file);

        println!("{code}");
    }
}
