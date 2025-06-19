use std::iter;

use proc_macro2::Span;
use quote::{quote, ToTokens};
use syn::{parse_quote, Field, Ident, ItemStruct};
use xmlity::ExpandedName;

use crate::misc::TypeReference;

use super::{
    value_record::{ItemFieldElement, SingleChildMode},
    FieldMode, FieldType, ItemOrder,
};

#[derive(Debug, Default, PartialEq)]
pub enum AllowUnknown {
    Any,
    #[default]
    AtEnd,
    None,
}

impl AllowUnknown {
    pub fn to_item_value(&self) -> syn::LitStr {
        match self {
            AllowUnknown::Any => syn::LitStr::new("any", Span::call_site()),
            AllowUnknown::AtEnd => syn::LitStr::new("at_end", Span::call_site()),
            AllowUnknown::None => syn::LitStr::new("none", Span::call_site()),
        }
    }
}

impl ToTokens for AllowUnknown {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.to_item_value().to_tokens(tokens)
    }
}

#[derive(Debug)]
pub struct ElementFieldAttribute {
    pub name: Option<ExpandedName<'static>>,
    pub ty: TypeReference<'static>,
    pub deferred: bool,
    pub optional: bool,
    pub default: bool,
}

impl ElementFieldAttribute {
    pub fn option_attributes(&self) -> impl Iterator<Item = syn::Meta> {
        let name_option: Option<syn::Meta> = self
            .name
            .as_ref()
            .map(|en| en.local_name().to_string())
            .map(|n| parse_quote! { name = #n });

        let namespace_option: Option<syn::Meta> = self
            .name
            .as_ref()
            .and_then(|en| en.namespace())
            .map(ToString::to_string)
            .map(|ns| parse_quote! { namespace = #ns });

        let deferred_option: Option<syn::Meta> = if self.deferred {
            Some(parse_quote! { deferred = true })
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

        name_option
            .into_iter()
            .chain(namespace_option)
            .chain(deferred_option)
            .chain(optional_option)
            .chain(default_option)
    }

    fn attribute_attr(&self) -> syn::Attribute {
        let options = self.option_attributes();
        parse_quote!(#[xattribute(#(#options),*)])
    }

    pub fn to_struct(&self, ident: &Ident, path: Option<&syn::Path>) -> ItemStruct {
        let derive_attr = super::derive_attribute([
            parse_quote!(::core::fmt::Debug),
            parse_quote!(::xmlity::SerializeAttribute),
            parse_quote!(::xmlity::Deserialize),
        ]);
        let element_attr = self.attribute_attr();

        let ty = self.ty.to_type(path);

        parse_quote!(
          #derive_attr
          #element_attr
          pub struct #ident(pub #ty);
        )
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
        let attribute_attr = self.attribute_attr();

        parse_quote!(
            #attribute_attr
            #vis #ident_prefix #ty
        )
    }
}

#[derive(Debug)]
pub struct ElementFieldGroup {
    pub ty: TypeReference<'static>,
}

impl ElementFieldGroup {
    pub fn option_attributes(&self) -> impl Iterator<Item = syn::MetaNameValue> {
        iter::empty()
    }

    pub fn group_attr(&self) -> syn::Attribute {
        let options = self.option_attributes().collect::<Vec<_>>();

        if options.is_empty() {
            parse_quote!(#[xgroup])
        } else {
            parse_quote!(#[xgroup(#(#options),*)])
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
        let group_attr = self.group_attr();

        parse_quote!(
            #group_attr
            #vis #ident_prefix #ty
        )
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
        path: Option<&'a syn::Path>,
    ) -> Field {
        match field_type {
            FieldType::Named => assert!(ident.is_some()),
            FieldType::Unnamed => assert!(ident.is_none()),
        }

        match self {
            ElementField::Attribute(attribute) => attribute.to_field(ident, mode, path),
            ElementField::Item(item) => item.to_field(ident, mode, path),
            ElementField::Element(element) => element.to_field(ident, mode, path),
            ElementField::Group(group) => group.to_field(ident, mode, path),
        }
    }
}

#[derive(Debug)]
pub enum ElementFieldType {
    Named(Vec<(Ident, ElementField)>),
    Unnamed(Vec<ElementField>),
    Empty,
}

impl ElementFieldType {
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

    pub fn first(&self) -> Option<&ElementField> {
        match self {
            Self::Named(items) => items.first().map(|(_, field)| field),
            Self::Unnamed(items) => items.first(),
            Self::Empty => None,
        }
    }

    pub fn prefix_fields(&mut self, new: Self) {
        match self {
            Self::Named(items) => match new {
                Self::Named(new_items) => {
                    items.splice(0..0, new_items);
                }
                Self::Unnamed(_) => panic!("cannot prefix named fields with unnamed fields"),
                Self::Empty => (),
            },
            Self::Unnamed(items) => match new {
                Self::Named(_) => panic!("cannot prefix unnamed fields with named fields"),
                Self::Unnamed(new_items) => {
                    items.splice(0..0, new_items);
                }
                Self::Empty => (),
            },
            Self::Empty => *self = new,
        }
    }
}

#[derive(Debug)]
pub struct ElementRecord {
    pub name: ExpandedName<'static>,
    pub attribute_order: ItemOrder,
    pub allow_unknown_attributes: AllowUnknown,
    pub children_order: ItemOrder,
    pub allow_unknown_children: AllowUnknown,
    pub fields: ElementFieldType,
}

impl ElementRecord {
    pub fn new_empty(name: ExpandedName<'static>) -> Self {
        Self {
            name,
            attribute_order: ItemOrder::None,
            children_order: ItemOrder::None,
            fields: ElementFieldType::Empty,
            allow_unknown_attributes: AllowUnknown::Any,
            allow_unknown_children: AllowUnknown::default(),
        }
    }

    pub fn new_single_field(
        name: ExpandedName<'static>,
        field_ident: Option<syn::Ident>,
        field: ElementField,
    ) -> Self {
        Self {
            name,
            attribute_order: ItemOrder::None,
            children_order: ItemOrder::None,
            fields: match field_ident {
                Some(ident) => ElementFieldType::Named(vec![(ident, field)]),
                None => ElementFieldType::Unnamed(vec![field]),
            },
            allow_unknown_attributes: AllowUnknown::Any,
            allow_unknown_children: AllowUnknown::default(),
        }
    }

    fn fields<'a>(
        &'a self,
        mode: FieldMode,
        path: Option<&'a syn::Path>,
    ) -> impl Iterator<Item = Field> + use<'a> {
        match &self.fields {
            ElementFieldType::Named(items) => items
                .iter()
                .map(move |(ident, field)| {
                    field.to_field(&FieldType::Named, Some(ident), mode, path)
                })
                .collect::<Vec<_>>()
                .into_iter(),
            ElementFieldType::Unnamed(items) => items
                .iter()
                .map(move |field| field.to_field(&FieldType::Unnamed, None, mode, path))
                .collect::<Vec<_>>()
                .into_iter(),
            ElementFieldType::Empty => vec![].into_iter(),
        }
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
                let order = order.to_item_value();

                Some(parse_quote! { attribute_order = #order })
            }
        };

        let allow_unknown_attributes_option: Option<syn::MetaNameValue> =
            Some(&self.allow_unknown_attributes)
                .filter(|a| **a != AllowUnknown::default())
                .map(|allow_unknown| parse_quote!(allow_unknown_attributes = #allow_unknown));

        let children_order_attr = match self.children_order {
            ItemOrder::None => None,
            order => {
                let order = order.to_item_value();

                Some(parse_quote! { children_order = #order })
            }
        };

        let allow_unknown_children_option: Option<syn::MetaNameValue> =
            Some(&self.allow_unknown_children)
                .filter(|a| **a != AllowUnknown::default())
                .map(|allow_unknown| parse_quote!(allow_unknown_children = #allow_unknown));

        iter::once(name_option)
            .chain(namespace_option)
            .chain(attribute_order_attr)
            .chain(allow_unknown_attributes_option)
            .chain(children_order_attr)
            .chain(allow_unknown_children_option)
    }

    fn element_attr(&self) -> syn::Attribute {
        let options = self.option_attributes();
        parse_quote!(#[xelement(#(#options),*)])
    }

    pub fn to_struct(&self, ident: &Ident, path: Option<&syn::Path>) -> ItemStruct {
        let fields = self.fields(FieldMode::Struct, path).collect::<Vec<_>>();

        let derive_attr = super::derive_attribute([
            parse_quote!(::core::fmt::Debug),
            parse_quote!(::xmlity::Serialize),
            parse_quote!(::xmlity::Deserialize),
        ]);
        let element_attr = self.element_attr();

        match &self.fields {
            ElementFieldType::Empty => {
                parse_quote!(
                  #derive_attr
                  #element_attr
                  pub struct #ident;
                )
            }
            ElementFieldType::Named(_) => parse_quote!(
              #derive_attr
              #element_attr
              pub struct #ident {
                #(#fields,)*
              }
            ),
            ElementFieldType::Unnamed(_) => {
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

    pub fn to_variant(&self, ident: &Ident, path: Option<&syn::Path>) -> syn::Variant {
        let fields = self.fields(FieldMode::Variant, path);

        let element_attr = self.element_attr();

        match &self.fields {
            ElementFieldType::Empty => {
                parse_quote!(
                    #element_attr
                    #ident
                )
            }
            ElementFieldType::Named(_) => parse_quote!(
                #element_attr
                #ident {
                  #(#fields,)*
                }
            ),
            ElementFieldType::Unnamed(_) => {
                parse_quote!(
                  #element_attr
                  #ident (
                    #(#fields),*
                  )
                )
            }
        }
    }

    pub fn try_into_compact_item_field(self, optional: bool) -> Result<ItemFieldElement, Self> {
        // Requirements:

        // #1: There must be only one or zero fields.
        if self.fields.len() > 1 {
            return Err(self);
        }

        // #2: The field must be a simple child or a group.
        let child = self.fields.first();
        let (ty, child_mode) = match child {
            Some(ElementField::Item(child)) => (child.ty.clone(), SingleChildMode::Item),
            Some(ElementField::Group(group)) => (group.ty.clone(), SingleChildMode::Group),
            None => {
                let empty = TypeReference::new_static(parse_quote! { () });
                (empty, SingleChildMode::Item)
            }
            _ => return Err(self),
        };

        Ok(ItemFieldElement {
            name: self.name,
            ty,
            child_mode,
            optional,
            default: false,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::templates::value_record::ItemFieldItem;
    use pretty_assertions::assert_eq;

    use super::*;
    use quote::format_ident;
    use syn::parse_quote;
    use xmlity::{ExpandedName, LocalName};

    #[test]
    fn generate_empty_element() {
        let record =
            ElementRecord::new_empty(ExpandedName::new(LocalName::new_dangerous("test"), None));

        let ident = format_ident!("Test");

        let actual_item = record.to_struct(&ident, None);

        let actual = prettyplease::unparse(&syn::File {
            shebang: None,
            attrs: Vec::new(),
            items: vec![actual_item.into()],
        });

        #[rustfmt::skip]
        let expected: syn::File = parse_quote!(
            #[derive(::core::fmt::Debug, ::xmlity::Serialize, ::xmlity::Deserialize)]
            #[xelement(name = "test", allow_unknown_attributes = "any")]
            pub struct Test;
        );

        let expected = prettyplease::unparse(&expected);

        assert_eq!(actual, expected);
    }

    #[test]
    fn generate_element_with_single_child_named() {
        let record = ElementRecord {
            name: ExpandedName::new(LocalName::new_dangerous("test"), None),
            attribute_order: ItemOrder::None,
            children_order: ItemOrder::None,
            fields: ElementFieldType::Named(vec![(
                format_ident!("a"),
                ElementField::Item(ItemFieldItem {
                    ty: TypeReference::new_static(parse_quote!(Child)),
                    default: false,
                }),
            )]),
            allow_unknown_attributes: AllowUnknown::Any,
            allow_unknown_children: AllowUnknown::AtEnd,
        };

        let ident = format_ident!("Test");

        let actual_item = record.to_struct(&ident, None);

        let actual = prettyplease::unparse(&syn::File {
            shebang: None,
            attrs: Vec::new(),
            items: vec![actual_item.into()],
        });

        #[rustfmt::skip]
        let expected: syn::File = parse_quote!(
            #[derive(::core::fmt::Debug, ::xmlity::Serialize, ::xmlity::Deserialize)]
            #[xelement(name = "test", allow_unknown_attributes = "any")]
            pub struct Test {
                pub a: Child,
            }
        );

        let expected = prettyplease::unparse(&expected);

        assert_eq!(actual, expected);
    }

    #[test]
    fn generate_element_with_single_child_unnamed() {
        let record = ElementRecord {
            name: ExpandedName::new(LocalName::new_dangerous("test"), None),
            attribute_order: ItemOrder::None,
            children_order: ItemOrder::None,
            fields: ElementFieldType::Unnamed(vec![ElementField::Item(ItemFieldItem {
                ty: TypeReference::new_static(parse_quote!(Child)),
                default: false,
            })]),
            allow_unknown_attributes: AllowUnknown::Any,
            allow_unknown_children: AllowUnknown::AtEnd,
        };

        let ident = format_ident!("Test");

        let actual_item = record.to_struct(&ident, None);

        let actual = prettyplease::unparse(&syn::File {
            shebang: None,
            attrs: Vec::new(),
            items: vec![actual_item.into()],
        });

        #[rustfmt::skip]
        let expected: syn::File = parse_quote!(
            #[derive(::core::fmt::Debug, ::xmlity::Serialize, ::xmlity::Deserialize)]
            #[xelement(name = "test", allow_unknown_attributes = "any")]
            pub struct Test(pub Child);
        );

        let expected = prettyplease::unparse(&expected);

        assert_eq!(actual, expected);
    }

    #[test]
    fn generate_element_with_single_attribute_named() {
        let record = ElementRecord {
            name: ExpandedName::new(LocalName::new_dangerous("test"), None),
            attribute_order: ItemOrder::None,
            children_order: ItemOrder::None,
            fields: ElementFieldType::Named(vec![(
                format_ident!("a"),
                ElementField::Attribute(ElementFieldAttribute {
                    name: Some(ExpandedName::new(LocalName::new_dangerous("a"), None)),
                    ty: TypeReference::new_static(parse_quote!(::std::string::String)),
                    deferred: false,
                    optional: false,
                    default: false,
                }),
            )]),
            allow_unknown_attributes: AllowUnknown::Any,
            allow_unknown_children: AllowUnknown::AtEnd,
        };

        let ident = format_ident!("Test");

        let actual_item = record.to_struct(&ident, None);

        let actual = prettyplease::unparse(&syn::File {
            shebang: None,
            attrs: Vec::new(),
            items: vec![actual_item.into()],
        });

        #[rustfmt::skip]
        let expected: syn::File = parse_quote!(
            #[derive(::core::fmt::Debug, ::xmlity::Serialize, ::xmlity::Deserialize)]
            #[xelement(name = "test", allow_unknown_attributes = "any")]
            pub struct Test {
                #[xattribute(name = "a")]
                pub a: ::std::string::String,
            }
        );

        let expected = prettyplease::unparse(&expected);

        assert_eq!(actual, expected);
    }
}
