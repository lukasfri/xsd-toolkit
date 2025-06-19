use syn::{parse_quote, Ident};

use super::{element_record::ElementRecord, value_record::ItemRecord};

#[derive(Debug)]
pub enum ChoiceVariantType {
    Element(ElementRecord),
    Item(ItemRecord),
}

impl ChoiceVariantType {
    pub fn to_variant(&self, ident: &Ident, path: Option<&syn::Path>) -> syn::Variant {
        match &self {
            ChoiceVariantType::Element(element_record) => element_record.to_variant(ident, path),
            ChoiceVariantType::Item(item) => item.to_variant(ident, path),
        }
    }
}

#[derive(Debug)]
pub struct Choice {
    pub variants: Vec<(Ident, ChoiceVariantType)>,
}

impl Choice {
    pub fn variants<'a>(
        &'a self,
        path: Option<&'a syn::Path>,
    ) -> impl Iterator<Item = syn::Variant> + use<'a> {
        self.variants
            .iter()
            .map(move |(ident, v)| v.to_variant(ident, path))
    }

    pub fn to_enum(&self, ident: &Ident, path: Option<&syn::Path>) -> syn::ItemEnum {
        let variants = self.variants(path);

        let derive_attr: syn::Attribute =
            parse_quote!(#[derive(::core::fmt::Debug, ::xmlity::Serialize, ::xmlity::Deserialize)]);

        parse_quote!(
          #derive_attr
          pub enum #ident {
            #(#variants,)*
          }
        )
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::{
        misc::TypeReference,
        templates::{
            element_record::{AllowUnknown, ElementField, ElementFieldAttribute, ElementFieldType},
            ItemOrder,
        },
    };

    use super::*;
    use quote::format_ident;
    use syn::{parse_quote, ItemEnum};
    use xmlity::{ExpandedName, LocalName};

    #[test]
    fn generate_no_variant_choice() {
        let record = Choice {
            variants: Vec::new(),
        };

        let ident = format_ident!("Test");

        let actual_item = record.to_enum(&ident, None);

        let expected_item: ItemEnum = parse_quote!(
            #[derive(::core::fmt::Debug, ::xmlity::Serialize, ::xmlity::Deserialize)]
            pub enum Test {}
        );

        assert_eq!(expected_item, actual_item);
    }

    #[test]
    fn generate_element_no_field_variant_choice() {
        let record = Choice {
            variants: vec![(
                format_ident!("A"),
                ChoiceVariantType::Element(ElementRecord {
                    name: ExpandedName::new(LocalName::new_dangerous("a"), None),
                    attribute_order: ItemOrder::None,
                    children_order: ItemOrder::None,
                    fields: ElementFieldType::Empty,
                    allow_unknown_attributes: AllowUnknown::Any,
                    allow_unknown_children: AllowUnknown::AtEnd,
                }),
            )],
        };

        let ident = format_ident!("Test");

        let actual_item = record.to_enum(&ident, None);

        let expected_item: ItemEnum = parse_quote!(
            #[derive(::core::fmt::Debug, ::xmlity::Serialize, ::xmlity::Deserialize)]
            pub enum Test {
                #[xelement(name = "a")]
                A,
            }
        );

        assert_eq!(expected_item, actual_item);
    }

    #[test]
    fn generate_element_variant_choice() {
        let record = Choice {
            variants: vec![(
                format_ident!("A"),
                ChoiceVariantType::Element(ElementRecord {
                    name: ExpandedName::new(LocalName::new_dangerous("a"), None),
                    attribute_order: ItemOrder::None,
                    children_order: ItemOrder::None,
                    fields: ElementFieldType::Named(vec![(
                        format_ident!("for_"),
                        ElementField::Attribute(ElementFieldAttribute {
                            name: Some(ExpandedName::new(LocalName::new_dangerous("for"), None)),
                            ty: TypeReference::new_static(parse_quote!(::std::string::String)),
                            deferred: false,
                            optional: false,
                            default: false,
                        }),
                    )]),
                    allow_unknown_attributes: AllowUnknown::Any,
                    allow_unknown_children: AllowUnknown::AtEnd,
                }),
            )],
        };

        let ident = format_ident!("Test");

        let actual_item = record.to_enum(&ident, None);

        let expected_item: ItemEnum = parse_quote!(
            #[derive(::core::fmt::Debug, ::xmlity::Serialize, ::xmlity::Deserialize)]
            pub enum Test {
                #[xelement(name = "a")]
                A {
                    #[xattribute(name = "for")]
                    for_: ::std::string::String,
                },
            }
        );

        assert_eq!(expected_item, actual_item);
    }
}
