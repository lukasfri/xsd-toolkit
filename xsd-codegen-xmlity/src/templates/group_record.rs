use syn::{parse_quote, Field, Ident, ItemStruct};
use xmlity::ExpandedName;

use super::{
    element_record::{ElementField, ElementRecord},
    FieldMode, FieldType, ItemOrder,
};

#[derive(Debug)]
pub struct GroupRecord {
    pub attribute_order: ItemOrder,
    pub children_order: ItemOrder,
    pub field_type: FieldType,
    pub fields: Vec<(Option<Ident>, ElementField)>,
}

impl GroupRecord {
    pub fn new_empty(field_type: FieldType) -> Self {
        Self {
            attribute_order: ItemOrder::None,
            children_order: ItemOrder::None,
            field_type,
            fields: Vec::new(),
        }
    }

    fn fields<'a>(&'a self, mode: FieldMode) -> impl Iterator<Item = Field> + use<'a> {
        self.fields
            .iter()
            .map(move |(ident, field)| field.to_field(&self.field_type, ident.as_ref(), mode))
    }

    pub fn option_attributes(&self) -> impl Iterator<Item = syn::MetaNameValue> {
        let attribute_order_attr = match self.attribute_order {
            ItemOrder::None => None,
            order => {
                let order = order.to_order_value();

                Some(parse_quote! { attribute_order = #order })
            }
        };

        let children_order_attr = match self.children_order {
            ItemOrder::None => None,
            order => {
                let order = order.to_order_value();

                Some(parse_quote! { children_order = #order })
            }
        };

        attribute_order_attr.into_iter().chain(children_order_attr)
    }

    fn group_attr(&self) -> Option<syn::Attribute> {
        let options = self.option_attributes().collect::<Vec<_>>();
        if options.is_empty() {
            return None;
        } else {
            Some(parse_quote!(#[xgroup(#(#options),*)]))
        }
    }

    pub fn into_struct(self, ident: &Ident) -> ItemStruct {
        let fields = self.fields(FieldMode::Struct).collect::<Vec<_>>();

        let derive_attr = super::derive_attribute([
            parse_quote!(::core::fmt::Debug),
            parse_quote!(::xmlity::SerializationGroup),
            parse_quote!(::xmlity::DeserializationGroup),
        ]);
        let group_attr = self.group_attr();

        match self.field_type {
            _ if self.fields.is_empty() => {
                parse_quote!(
                  #derive_attr
                  #group_attr
                  pub struct #ident;
                )
            }
            FieldType::Named => parse_quote!(
              #derive_attr
              #group_attr
              pub struct #ident {
                #(#fields,)*
              }
            ),
            FieldType::Unnamed => {
                parse_quote!(
                  #derive_attr
                  #group_attr
                  pub struct #ident (
                    #(#fields),*
                  );
                )
            }
        }
    }

    pub fn into_element_record(self, name: ExpandedName<'static>) -> ElementRecord {
        ElementRecord {
            name,
            attribute_order: self.attribute_order,
            children_order: self.children_order,
            field_type: self.field_type,
            fields: self.fields,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::templates::value_record::ItemFieldItem;
    use pretty_assertions::assert_eq;

    use super::*;
    use quote::format_ident;
    use syn::{parse_quote, ItemStruct};

    #[test]
    fn generate_empty_element() {
        let record = GroupRecord {
            attribute_order: ItemOrder::None,
            children_order: ItemOrder::None,
            field_type: FieldType::Named,
            fields: Vec::new(),
        };

        let ident = format_ident!("Test");

        let actual_item = record.into_struct(&ident);

        #[rustfmt::skip]
        let expected_item: ItemStruct = parse_quote!(
            #[derive(::core::fmt::Debug, ::xmlity::SerializationGroup, ::xmlity::DeserializationGroup)]
            pub struct Test;
        );

        assert_eq!(expected_item, actual_item);
    }

    #[test]
    fn generate_empty_element_with_single_child_named() {
        let record = GroupRecord {
            attribute_order: ItemOrder::None,
            children_order: ItemOrder::None,
            field_type: FieldType::Named,
            fields: vec![(
                Some(format_ident!("a")),
                ElementField::Item(ItemFieldItem {
                    ty: parse_quote!(Child),
                }),
            )],
        };

        let ident = format_ident!("Test");

        let actual_item = record.into_struct(&ident);

        #[rustfmt::skip]
        let expected_item: ItemStruct = parse_quote!(
            #[derive(::core::fmt::Debug, ::xmlity::SerializationGroup, ::xmlity::DeserializationGroup)]
            pub struct Test {
                pub a: Child,
            }
        );

        assert_eq!(expected_item, actual_item);
    }

    #[test]
    fn generate_empty_element_with_single_child_unnamed() {
        let record = GroupRecord {
            attribute_order: ItemOrder::None,
            children_order: ItemOrder::None,
            field_type: FieldType::Unnamed,
            fields: vec![(
                None,
                ElementField::Item(ItemFieldItem {
                    ty: parse_quote!(Child),
                }),
            )],
        };

        let ident = format_ident!("Test");

        let actual_item = record.into_struct(&ident);

        #[rustfmt::skip]
        let expected_item: ItemStruct = parse_quote!(
            #[derive(::core::fmt::Debug, ::xmlity::SerializationGroup, ::xmlity::DeserializationGroup)]
            pub struct Test(pub Child);
        );

        assert_eq!(expected_item, actual_item);
    }
}
