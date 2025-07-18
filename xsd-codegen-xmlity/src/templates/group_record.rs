use syn::{parse_quote, Field, Ident, ItemStruct};
use xmlity::ExpandedName;

use crate::templates::element_record::{AllowUnknown, ElementFieldType};

use super::{
    element_record::{ElementField, ElementRecord},
    FieldMode, FieldType, ItemOrder,
};

#[derive(Debug)]
pub struct GroupRecord {
    pub attribute_order: ItemOrder,
    pub children_order: ItemOrder,
    pub fields: ElementFieldType,
}

impl GroupRecord {
    pub fn new_single_field(ident: Option<Ident>, field: ElementField) -> Self {
        let fields = match ident {
            Some(ident) => ElementFieldType::Named(vec![(ident, field)]),
            None => ElementFieldType::Unnamed(vec![field]),
        };

        Self {
            attribute_order: ItemOrder::None,
            children_order: ItemOrder::None,
            fields,
        }
    }

    pub fn new_empty() -> Self {
        Self {
            attribute_order: ItemOrder::None,
            children_order: ItemOrder::None,
            fields: ElementFieldType::Empty,
        }
    }

    pub fn force_empty_if_empty(&mut self) {
        if self.fields.is_empty() {
            self.fields = ElementFieldType::Empty;
        }
    }

    fn fields<'a>(
        &'a self,
        mode: FieldMode,
        path: Option<&'a syn::Path>,
    ) -> impl Iterator<Item = Field> + use<'a> {
        match &self.fields {
            ElementFieldType::Named(fields) => fields
                .iter()
                .map(move |(ident, field)| {
                    field.to_field(&FieldType::Named, Some(ident), mode, path)
                })
                .collect::<Vec<_>>()
                .into_iter(),
            ElementFieldType::Unnamed(fields) => fields
                .iter()
                .map(move |field| field.to_field(&FieldType::Unnamed, None, mode, path))
                .collect::<Vec<_>>()
                .into_iter(),
            ElementFieldType::Empty => vec![].into_iter(),
        }
        // self.fields
        //     .iter()
        //     .map(move |(ident, field)| field.to_field(&self.field_type, ident.as_ref(), mode, path))
    }

    pub fn option_attributes(&self) -> impl Iterator<Item = syn::MetaNameValue> {
        let attribute_order_attr = match self.attribute_order {
            ItemOrder::None => None,
            order => {
                let order = order.to_group_value();

                Some(parse_quote! { attribute_order = #order })
            }
        };

        let children_order_attr = match self.children_order {
            ItemOrder::None => None,
            order => {
                let order = order.to_group_value();

                Some(parse_quote! { children_order = #order })
            }
        };

        attribute_order_attr.into_iter().chain(children_order_attr)
    }

    fn group_attr(&self) -> Option<syn::Attribute> {
        let options = self.option_attributes().collect::<Vec<_>>();
        if options.is_empty() {
            None
        } else {
            Some(parse_quote!(#[xgroup(#(#options),*)]))
        }
    }

    pub fn to_struct(&self, ident: &Ident, path: Option<&syn::Path>) -> ItemStruct {
        let fields = self.fields(FieldMode::Struct, path).collect::<Vec<_>>();
        let derive_attr = super::derive_attribute([
            parse_quote!(::core::fmt::Debug),
            parse_quote!(::xmlity::SerializationGroup),
            parse_quote!(::xmlity::DeserializationGroup),
        ]);
        let group_attr = self.group_attr();

        match &self.fields {
            ElementFieldType::Named(_) => {
                parse_quote!(
                  #derive_attr
                  #group_attr
                  pub struct #ident {
                    #(#fields,)*
                  }
                )
            }
            ElementFieldType::Unnamed(_) => {
                parse_quote!(
                  #derive_attr
                  #group_attr
                  pub struct #ident (
                    #(#fields),*
                  );
                )
            }
            ElementFieldType::Empty => {
                parse_quote!(
                  #derive_attr
                  #group_attr
                  pub struct #ident;
                )
            }
        }
    }

    pub fn into_element_record(self, name: ExpandedName<'static>) -> ElementRecord {
        ElementRecord {
            name,
            attribute_order: self.attribute_order,
            children_order: self.children_order,
            fields: self.fields,
            allow_unknown_attributes: AllowUnknown::Any,
            allow_unknown_children: AllowUnknown::AtEnd,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{misc::TypeReference, templates::value_record::ItemFieldItem};
    use pretty_assertions::assert_eq;

    use super::*;
    use quote::format_ident;
    use syn::{parse_quote, ItemStruct};

    #[test]
    fn generate_empty_element() {
        let record = GroupRecord::new_empty();

        let ident = format_ident!("Test");

        let actual_item = record.to_struct(&ident, None);

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
            fields: ElementFieldType::Named(vec![(
                format_ident!("a"),
                ElementField::Item(ItemFieldItem {
                    ty: TypeReference::new_static(parse_quote!(Child)),
                    default: false,
                }),
            )]),
        };

        let ident = format_ident!("Test");

        let actual_item = record.to_struct(&ident, None);

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
            fields: ElementFieldType::Unnamed(vec![ElementField::Item(ItemFieldItem {
                ty: TypeReference::new_static(parse_quote!(Child)),
                default: false,
            })]),
        };

        let ident = format_ident!("Test");

        let actual_item = record.to_struct(&ident, None);

        #[rustfmt::skip]
        let expected_item: ItemStruct = parse_quote!(
            #[derive(::core::fmt::Debug, ::xmlity::SerializationGroup, ::xmlity::DeserializationGroup)]
            pub struct Test(pub Child);
        );

        assert_eq!(expected_item, actual_item);
    }
}
