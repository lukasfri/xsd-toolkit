use std::iter;

use quote::quote;
use syn::{parse_quote, Field, Ident, ItemStruct};
use xmlity::ExpandedName;

use super::{group_record::GroupRecord, FieldMode, FieldType, ItemOrder};

#[derive(Debug)]
pub struct ItemFieldItem {
    pub ty: syn::Type,
}

impl ItemFieldItem {
    pub fn to_field(&self, ident: Option<&Ident>, mode: FieldMode) -> syn::Field {
        let ident_prefix = ident.map(|ident| quote!(#ident: )).unwrap_or_default();

        let vis = mode.to_vis();
        let ty = &self.ty;

        parse_quote!(
            #vis #ident_prefix #ty
        )
    }
}

#[derive(Debug)]
pub enum SingleChildMode {
    Item,
    Group,
}

#[derive(Debug)]
pub struct ItemFieldElement {
    pub name: ExpandedName<'static>,
    pub ty: syn::Type,
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
            .chain(optional_option)
            .chain(default_option)
    }

    pub fn element_attr(&self) -> syn::Attribute {
        let options = self.option_attributes();
        parse_quote!(#[xelement(#(#options),*)])
    }

    pub fn to_field(&self, ident: Option<&Ident>, mode: FieldMode) -> syn::Field {
        let ident_prefix = ident.map(|ident| quote!(#ident: )).unwrap_or_default();

        let vis = mode.to_vis();
        let ty = &self.ty;
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

#[derive(Debug)]
pub struct ItemRecord {
    pub children_order: ItemOrder,
    pub field_type: FieldType,
    pub fields: Vec<(Option<Ident>, ItemField)>,
}

impl ItemRecord {
    fn field<'a>(
        field_type: &FieldType,
        ident: Option<&'a Ident>,
        field: &'a ItemField,
        mode: FieldMode,
    ) -> Field {
        match field_type {
            FieldType::Named => assert!(ident.is_some()),
            FieldType::Unnamed => assert!(ident.is_none()),
        }

        match field {
            ItemField::Item(item) => item.to_field(ident, mode),
            ItemField::Element(element) => element.to_field(ident, mode),
        }
    }

    fn fields<'a>(&'a self, mode: FieldMode) -> impl Iterator<Item = Field> + use<'a> {
        self.fields
            .iter()
            .map(move |(ident, field)| Self::field(&self.field_type, ident.as_ref(), field, mode))
    }

    pub fn option_attributes(&self) -> impl Iterator<Item = syn::MetaNameValue> {
        let children_order_attr = match self.children_order {
            ItemOrder::None => None,
            order => {
                let order = order.to_order_value();

                Some(parse_quote! { children_order = #order })
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

    pub fn to_struct(&self, ident: &Ident) -> ItemStruct {
        let fields = self.fields(FieldMode::Struct).collect::<Vec<_>>();

        let derive_attr = super::derive_attribute([
            parse_quote!(::core::fmt::Debug),
            parse_quote!(::xmlity::Serialize),
            parse_quote!(::xmlity::Deserialize),
        ]);
        let value_attr = self.value_attr();

        match self.field_type {
            _ if self.fields.is_empty() => {
                parse_quote!(
                  #derive_attr
                  #value_attr
                  pub struct #ident;
                )
            }
            FieldType::Named => parse_quote!(
              #derive_attr
              #value_attr
              pub struct #ident {
                #(#fields,)*
              }
            ),
            FieldType::Unnamed => {
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

    pub fn to_variant(&self, ident: &Ident) -> syn::Variant {
        let fields = self.fields(FieldMode::Variant);

        let value_attr = self.value_attr();

        match self.field_type {
            _ if self.fields.is_empty() => {
                parse_quote!(
                    #value_attr
                    #ident
                )
            }
            FieldType::Named => parse_quote!(
                #value_attr
                #ident {
                  #(#fields),*
                }
            ),
            FieldType::Unnamed => {
                parse_quote!(
                  #value_attr
                  #ident (
                    #(#fields,)*
                  )
                )
            }
        }
    }

    pub fn into_group_record(self) -> GroupRecord {
        GroupRecord {
            attribute_order: ItemOrder::None,
            children_order: self.children_order,
            field_type: self.field_type,
            fields: self
                .fields
                .into_iter()
                .map(|(ident, field)| (ident, field.into()))
                .collect(),
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
        let record = ItemRecord {
            children_order: ItemOrder::None,
            field_type: FieldType::Named,
            fields: Vec::new(),
        };

        let ident = format_ident!("Test");

        let actual_item = record.to_struct(&ident);

        let expected_item: ItemStruct = parse_quote!(
            #[derive(::core::fmt::Debug, ::xmlity::Serialize, ::xmlity::Deserialize)]
            pub struct Test;
        );

        assert_eq!(expected_item, actual_item);
    }

    #[test]
    fn generate_empty_element_with_single_child_named() {
        let record = ItemRecord {
            children_order: ItemOrder::None,
            field_type: FieldType::Named,
            fields: vec![(
                Some(format_ident!("a")),
                ItemField::Item(ItemFieldItem {
                    ty: parse_quote!(Child),
                }),
            )],
        };

        let ident = format_ident!("Test");

        let actual_item = record.to_struct(&ident);

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
        let record = ItemRecord {
            children_order: ItemOrder::None,
            field_type: FieldType::Unnamed,
            fields: vec![(
                None,
                ItemField::Item(ItemFieldItem {
                    ty: parse_quote!(Child),
                }),
            )],
        };

        let ident = format_ident!("Test");

        let actual_item = record.to_struct(&ident);

        let expected_item: ItemStruct = parse_quote!(
            #[derive(::core::fmt::Debug, ::xmlity::Serialize, ::xmlity::Deserialize)]
            pub struct Test(pub Child);
        );

        assert_eq!(expected_item, actual_item);
    }
}
