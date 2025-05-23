use std::iter;

use quote::quote;
use syn::{parse_quote, Field, Ident, ItemStruct, Token};
use xmlity::ExpandedName;

use super::{
    value_record::{ItemFieldElement, SingleChildMode},
    FieldMode, FieldType, ItemOrder,
};

#[derive(Debug)]
pub struct ElementFieldAttribute {
    pub name: Option<ExpandedName<'static>>,
    pub ty: syn::Type,
    pub deferred: bool,
}

impl ElementFieldAttribute {
    pub fn option_attributes(&self) -> impl Iterator<Item = syn::MetaNameValue> {
        let name_option: Option<syn::MetaNameValue> = self
            .name
            .as_ref()
            .map(|en| en.local_name().to_string())
            .map(|n| parse_quote! { name = #n });

        let namespace_option: Option<syn::MetaNameValue> = self
            .name
            .as_ref()
            .and_then(|en| en.namespace())
            .map(ToString::to_string)
            .map(|ns| parse_quote! { namespace = #ns });

        name_option.into_iter().chain(namespace_option)
    }

    fn attribute_attr(&self) -> syn::Attribute {
        let options = self.option_attributes();
        parse_quote!(#[xattribute(#(#options),*)])
    }
}

#[derive(Debug)]
pub struct ElementFieldGroup {
    pub ty: syn::Type,
}

impl ElementFieldGroup {
    pub fn option_attributes(&self) -> impl Iterator<Item = syn::MetaNameValue> {
        iter::empty()
    }

    pub fn group_attr(&self) -> syn::Attribute {
        let options = self.option_attributes().collect::<Vec<_>>();

        if options.is_empty() {
            return parse_quote!(#[xgroup]);
        } else {
            parse_quote!(#[xgroup(#(#options),*)])
        }
    }
}

#[derive(Debug)]
pub enum ElementField {
    Attribute(ElementFieldAttribute),
    Item(super::value_record::ItemFieldItem),
    Element(super::value_record::ItemFieldElement),
    Group(ElementFieldGroup),
}

impl From<super::value_record::ItemField> for ElementField {
    fn from(value: super::value_record::ItemField) -> Self {
        match value {
            super::value_record::ItemField::Item(item) => Self::Item(item),
            super::value_record::ItemField::Element(element) => Self::Element(element),
        }
    }
}

impl ElementField {
    pub fn to_field<'a>(
        &self,
        field_type: &FieldType,
        ident: Option<&'a Ident>,
        mode: FieldMode,
    ) -> Field {
        match field_type {
            FieldType::Named => assert!(ident.is_some()),
            FieldType::Unnamed => assert!(ident.is_none()),
        }

        let vis = match mode {
            FieldMode::Struct => syn::Visibility::Public(<Token![pub]>::default()),
            FieldMode::Variant => syn::Visibility::Inherited,
        };
        let ident_prefix = ident.map(|ident| quote!(#ident: )).unwrap_or_default();

        match self {
            ElementField::Attribute(attribute) => {
                let ty = &attribute.ty;
                let attribute_attr = attribute.attribute_attr();

                parse_quote!(
                    #attribute_attr
                    #vis #ident_prefix #ty
                )
            }
            ElementField::Item(item) => {
                let ty = &item.ty;
                parse_quote!(
                    #vis #ident_prefix #ty
                )
            }
            ElementField::Element(element) => {
                let ty = &element.ty;
                let element_attr = element.element_attr();

                parse_quote!(
                    #element_attr
                    #vis #ident_prefix #ty
                )
            }
            ElementField::Group(group) => {
                let ty = &group.ty;
                let group_attr = group.group_attr();

                parse_quote!(
                    #group_attr
                    #vis #ident_prefix #ty
                )
            }
        }
    }
}

#[derive(Debug)]
pub struct ElementRecord {
    pub name: ExpandedName<'static>,
    pub attribute_order: ItemOrder,
    pub children_order: ItemOrder,
    pub field_type: FieldType,
    pub fields: Vec<(Option<Ident>, ElementField)>,
}

impl ElementRecord {
    fn fields<'a>(&'a self, mode: FieldMode) -> impl Iterator<Item = Field> + use<'a> {
        self.fields
            .iter()
            .map(move |(ident, field)| field.to_field(&self.field_type, ident.as_ref(), mode))
    }

    pub fn option_attributes(&self) -> impl Iterator<Item = syn::MetaNameValue> {
        let name = self.name.local_name().to_string();
        let name_option: syn::MetaNameValue = parse_quote! { name = #name };
        let namespace_option: Option<syn::MetaNameValue> =
            self.name.namespace().map(ToString::to_string).map(|ns| {
                parse_quote! { namespace = #ns }
            });

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

        iter::once(name_option)
            .chain(namespace_option)
            .chain(attribute_order_attr)
            .chain(children_order_attr)
    }

    fn element_attr(&self) -> syn::Attribute {
        let options = self.option_attributes();
        parse_quote!(#[xelement(#(#options),*)])
    }

    pub fn into_struct(self, ident: &Ident) -> ItemStruct {
        let fields = self.fields(FieldMode::Struct).collect::<Vec<_>>();

        let derive_attr = super::derive_attribute([
            parse_quote!(::core::fmt::Debug),
            parse_quote!(::xmlity::Serialize),
            parse_quote!(::xmlity::Deserialize),
        ]);
        let element_attr = self.element_attr();

        match self.field_type {
            _ if self.fields.is_empty() => {
                parse_quote!(
                  #derive_attr
                  #element_attr
                  pub struct #ident;
                )
            }
            FieldType::Named => parse_quote!(
              #derive_attr
              #element_attr
              pub struct #ident {
                #(#fields,)*
              }
            ),
            FieldType::Unnamed => {
                parse_quote!(
                  #derive_attr
                  #element_attr
                  pub struct #ident (
                    #(#fields),*
                  );
                )
            }
        }
    }

    pub fn into_variant(&self, ident: &Ident) -> syn::Variant {
        let fields = self.fields(FieldMode::Variant);

        let element_attr = self.element_attr();

        match self.field_type {
            _ if self.fields.is_empty() => {
                parse_quote!(
                    #element_attr
                    #ident
                )
            }
            FieldType::Named => parse_quote!(
                #element_attr
                #ident {
                  #(#fields,)*
                }
            ),
            FieldType::Unnamed => {
                parse_quote!(
                  #element_attr
                  #ident (
                    #(#fields),*
                  )
                )
            }
        }
    }

    pub fn try_into_compact_item_field(self) -> Result<ItemFieldElement, Self> {
        // Requirements:

        // #1: There must be only one or zero fields.
        if self.fields.len() > 1 {
            return Err(self);
        }

        // #2: The field must be a simple child or a group.
        let child = self.fields.first();
        let (ty, child_mode) = match child.map(|(_, field)| field) {
            Some(ElementField::Item(child)) => (child.ty.clone(), SingleChildMode::Item),
            Some(ElementField::Group(group)) => (group.ty.clone(), SingleChildMode::Group),
            None => {
                let empty = parse_quote! { () };
                (empty, SingleChildMode::Item)
            }
            _ => return Err(self),
        };

        Ok(ItemFieldElement {
            name: self.name,
            ty,
            child_mode,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::templates::value_record::ItemFieldItem;

    use super::*;
    use quote::format_ident;
    use syn::{parse_quote, ItemStruct};
    use xmlity::{ExpandedName, LocalName};

    #[test]
    fn generate_empty_element() {
        let record = ElementRecord {
            name: ExpandedName::new(LocalName::new_dangerous("test"), None),
            attribute_order: ItemOrder::None,
            children_order: ItemOrder::None,
            field_type: FieldType::Named,
            fields: Vec::new(),
        };

        let ident = format_ident!("Test");

        let actual_item = record.into_struct(&ident);

        let expected_item: ItemStruct = parse_quote!(
            #[derive(::core::fmt::Debug, ::xmlity::Serialize, ::xmlity::Deserialize)]
            #[xelement(name = "test")]
            pub struct Test;
        );

        assert_eq!(expected_item, actual_item);
    }

    #[test]
    fn generate_empty_element_with_single_child_named() {
        let record = ElementRecord {
            name: ExpandedName::new(LocalName::new_dangerous("test"), None),
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

        let expected_item: ItemStruct = parse_quote!(
            #[derive(::core::fmt::Debug, ::xmlity::Serialize, ::xmlity::Deserialize)]
            #[xelement(name = "test")]
            pub struct Test {
                pub a: Child,
            }
        );

        assert_eq!(expected_item, actual_item);
    }

    #[test]
    fn generate_empty_element_with_single_child_unnamed() {
        let record = ElementRecord {
            name: ExpandedName::new(LocalName::new_dangerous("test"), None),
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

        let expected_item: ItemStruct = parse_quote!(
            #[derive(::core::fmt::Debug, ::xmlity::Serialize, ::xmlity::Deserialize)]
            #[xelement(name = "test")]
            pub struct Test(pub Child);
        );

        assert_eq!(expected_item, actual_item);
    }
}
