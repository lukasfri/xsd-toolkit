use std::iter;

use quote::quote;
use syn::{parse_quote, Field, Ident, ItemStruct};
use xmlity::ExpandedName;

use crate::{
    misc::TypeReference,
    templates::{
        element_record::{self, ElementField},
        value_record,
    },
};

use super::{group_record::GroupRecord, FieldMode, ItemOrder};

#[derive(Debug)]
pub struct ItemFieldItem {
    pub ty: TypeReference<'static>,
    pub default: bool,
}

impl ItemFieldItem {
    pub fn option_attributes(&self) -> impl Iterator<Item = syn::Meta> {
        let default_option: Option<syn::Meta> = if self.default {
            Some(parse_quote! { default })
        } else {
            None
        };

        default_option.into_iter()
    }

    pub fn value_attr(&self) -> Option<syn::Attribute> {
        let options = self.option_attributes().collect::<Vec<_>>();

        if options.is_empty() {
            None
        } else {
            Some(parse_quote!(#[xvalue(#(#options),*)]))
        }
    }

    pub fn to_field(
        &self,
        ident: Option<&Ident>,
        mode: FieldMode,
        path: Option<&syn::Path>,
    ) -> syn::Field {
        let ident_prefix = ident.map(|ident| quote!(#ident: )).unwrap_or_default();

        let vis = mode.to_vis();
        let ty = self.ty.to_type(path);
        let value_attr = self.value_attr();

        parse_quote!(
            #value_attr
            #vis #ident_prefix #ty
        )
    }

    pub fn into_group_record(self, ident: Option<Ident>) -> GroupRecord {
        GroupRecord::new_single_field(ident, ElementField::Item(self))
    }

    pub fn into_item_record(self, ident: Option<Ident>) -> ItemRecord {
        ItemRecord::new_single_field(ident, ItemField::Item(self))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SingleChildMode {
    Item,
    Group,
}

#[derive(Debug)]
pub struct ItemFieldElement {
    pub name: ExpandedName<'static>,
    pub ty: TypeReference<'static>,
    pub child_mode: SingleChildMode,
    pub optional: bool,
    pub default: bool,
}

impl ItemFieldElement {
    pub fn option_attributes(&self) -> impl Iterator<Item = syn::Meta> {
        let name = self.name.local_name().to_string();
        let name_option: syn::Meta = parse_quote! { name = #name };
        let namespace_option: Option<syn::Meta> =
            self.name.namespace().map(ToString::to_string).map(|ns| {
                parse_quote! { namespace = #ns }
            });

        let group_option: Option<syn::Meta> = if self.child_mode == SingleChildMode::Group {
            Some(parse_quote! { group })
        } else {
            None
        };

        let optional_option: Option<syn::Meta> = if self.optional {
            Some(parse_quote! { optional })
        } else {
            None
        };

        let default_option: Option<syn::Meta> = if self.default {
            Some(parse_quote! { default })
        } else {
            None
        };

        iter::once(name_option)
            .chain(namespace_option)
            .chain(group_option)
            .chain(optional_option)
            .chain(default_option)
    }

    pub fn element_attr(&self) -> syn::Attribute {
        let options = self.option_attributes();
        parse_quote!(#[xelement(#(#options),*)])
    }

    pub fn to_field(
        &self,
        ident: Option<&Ident>,
        mode: FieldMode,
        path: Option<&syn::Path>,
    ) -> syn::Field {
        let ident_prefix = ident.map(|ident| quote!(#ident: )).unwrap_or_default();

        let vis = mode.to_vis();
        let ty = self.ty.to_type(path);
        let element_attr = self.element_attr();

        parse_quote!(
            #element_attr
            #vis #ident_prefix #ty
        )
    }
}

#[derive(Debug)]
pub enum ItemField {
    Item(ItemFieldItem),
    Element(ItemFieldElement),
}

impl ItemField {
    fn to_field<'a>(
        &self,
        ident: Option<&'a Ident>,
        mode: FieldMode,
        path: Option<&'a syn::Path>,
    ) -> Field {
        match self {
            ItemField::Item(item) => item.to_field(ident, mode, path),
            ItemField::Element(element) => element.to_field(ident, mode, path),
        }
    }
}

#[derive(Debug)]
pub enum ItemFieldType {
    Named(Vec<(Ident, ItemField)>),
    Unnamed(Vec<ItemField>),
    Empty,
}

impl ItemFieldType {
    pub fn len(&self) -> usize {
        match self {
            Self::Named(items) => items.len(),
            Self::Unnamed(items) => items.len(),
            Self::Empty => 0,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn first(&self) -> Option<&ItemField> {
        match self {
            Self::Named(items) => items.first().map(|(_, field)| field),
            Self::Unnamed(items) => items.first(),
            Self::Empty => None,
        }
    }

    pub fn to_element_fields(self) -> element_record::ElementFieldType {
        match self {
            Self::Named(items) => element_record::ElementFieldType::Named(
                items
                    .into_iter()
                    .map(|(ident, field)| (ident, field.into()))
                    .collect(),
            ),
            Self::Unnamed(items) => element_record::ElementFieldType::Unnamed(
                items.into_iter().map(|field| field.into()).collect(),
            ),
            Self::Empty => element_record::ElementFieldType::Empty,
        }
    }

    pub fn to_item_fields(self) -> value_record::ItemFieldType {
        match self {
            Self::Named(items) => value_record::ItemFieldType::Named(items),
            Self::Unnamed(items) => value_record::ItemFieldType::Unnamed(items),
            Self::Empty => value_record::ItemFieldType::Empty,
        }
    }
}

#[derive(Debug)]
pub struct ItemRecord {
    pub children_order: ItemOrder,
    pub fields: ItemFieldType,
}

impl ItemRecord {
    pub fn new_empty() -> Self {
        Self {
            children_order: ItemOrder::None,
            fields: ItemFieldType::Empty,
        }
    }

    pub fn new_single_field(ident: Option<Ident>, field: ItemField) -> Self {
        let fields = match ident {
            Some(ident) => ItemFieldType::Named(vec![(ident, field)]),
            None => ItemFieldType::Unnamed(vec![field]),
        };

        Self {
            children_order: ItemOrder::None,
            fields,
        }
    }

    pub fn force_empty_if_empty(&mut self) {
        if self.fields.is_empty() {
            self.fields = ItemFieldType::Empty;
        }
    }

    fn fields<'a>(
        &'a self,
        mode: FieldMode,
        path: Option<&'a syn::Path>,
    ) -> impl Iterator<Item = Field> + use<'a> {
        // self.fields.iter().map(move |(ident, field)| {
        //     Self::field(&self.field_type, ident.as_ref(), field, mode, path)
        // })

        match &self.fields {
            ItemFieldType::Named(items) => items
                .iter()
                .map(move |(ident, field)| field.to_field(Some(ident), mode, path))
                .collect::<Vec<_>>()
                .into_iter(),
            ItemFieldType::Unnamed(items) => items
                .iter()
                .map(move |field| field.to_field(None, mode, path))
                .collect::<Vec<_>>()
                .into_iter(),
            ItemFieldType::Empty => vec![].into_iter(),
        }
    }

    pub fn option_attributes(&self) -> impl Iterator<Item = syn::MetaNameValue> {
        let children_order_attr = match self.children_order {
            ItemOrder::None => None,
            order => {
                let order = order.to_item_value();

                Some(parse_quote! { order = #order })
            }
        };

        children_order_attr.into_iter()
    }

    fn value_attr(&self) -> Option<syn::Attribute> {
        let options = self.option_attributes().collect::<Vec<_>>();
        if options.is_empty() {
            None
        } else {
            Some(parse_quote!(#[xvalue(#(#options),*)]))
        }
    }

    pub fn to_struct(&self, ident: &Ident, path: Option<&syn::Path>) -> ItemStruct {
        let fields = self.fields(FieldMode::Struct, path).collect::<Vec<_>>();

        let derive_attr = super::derive_attribute([
            parse_quote!(::core::fmt::Debug),
            parse_quote!(::xmlity::Serialize),
            parse_quote!(::xmlity::Deserialize),
        ]);
        let value_attr = self.value_attr();

        match &self.fields {
            ItemFieldType::Empty => {
                parse_quote!(
                  #derive_attr
                  #value_attr
                  pub struct #ident;
                )
            }
            ItemFieldType::Named(_) => parse_quote!(
              #derive_attr
              #value_attr
              pub struct #ident {
                #(#fields,)*
              }
            ),
            ItemFieldType::Unnamed(_) => {
                parse_quote!(
                  #derive_attr
                  #value_attr
                  pub struct #ident (
                    #(#fields),*
                  );
                )
            }
        }
    }

    pub fn to_variant(&self, ident: &Ident, path: Option<&syn::Path>) -> syn::Variant {
        let fields = self.fields(FieldMode::Variant, path);

        let value_attr = self.value_attr();

        match &self.fields {
            ItemFieldType::Empty => {
                parse_quote!(
                    #value_attr
                    #ident
                )
            }
            ItemFieldType::Named(_) => parse_quote!(
                #value_attr
                #ident {
                  #(#fields,)*
                }
            ),
            ItemFieldType::Unnamed(_) => {
                parse_quote!(
                  #value_attr
                  #ident (
                    #(#fields),*
                  )
                )
            }
        }
    }

    pub fn into_group_record(self) -> GroupRecord {
        GroupRecord {
            attribute_order: ItemOrder::None,
            children_order: self.children_order,
            fields: self.fields.to_element_fields(),
        }
    }

    pub fn into_item_record(self) -> ItemRecord {
        ItemRecord {
            children_order: self.children_order,
            fields: self.fields.to_item_fields(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quote::format_ident;
    use syn::{parse_quote, ItemStruct};

    #[test]
    fn generate_empty_element() {
        let record = ItemRecord::new_empty();

        let ident = format_ident!("Test");

        let actual_item = record.to_struct(&ident, None);

        let expected_item: ItemStruct = parse_quote!(
            #[derive(::core::fmt::Debug, ::xmlity::Serialize, ::xmlity::Deserialize)]
            pub struct Test;
        );

        assert_eq!(expected_item, actual_item);
    }

    #[test]
    fn generate_empty_element_with_single_child_named() {
        let record = ItemRecord::new_single_field(
            Some(format_ident!("a")),
            ItemField::Item(ItemFieldItem {
                ty: TypeReference::new_prefixed_type(parse_quote!(Child)),
                default: false,
            }),
        );

        let ident = format_ident!("Test");

        let actual_item = record.to_struct(&ident, None);

        let expected_item: ItemStruct = parse_quote!(
            #[derive(::core::fmt::Debug, ::xmlity::Serialize, ::xmlity::Deserialize)]
            pub struct Test {
                pub a: Child,
            }
        );

        assert_eq!(expected_item, actual_item);
    }

    #[test]
    fn generate_empty_element_with_single_child_unnamed() {
        let record = ItemRecord::new_single_field(
            None,
            ItemField::Item(ItemFieldItem {
                ty: TypeReference::new_prefixed_type(parse_quote!(Child)),
                default: false,
            }),
        );

        let ident = format_ident!("Test");

        let actual_item = record.to_struct(&ident, None);

        let expected_item: ItemStruct = parse_quote!(
            #[derive(::core::fmt::Debug, ::xmlity::Serialize, ::xmlity::Deserialize)]
            pub struct Test(pub Child);
        );

        assert_eq!(expected_item, actual_item);
    }
}
