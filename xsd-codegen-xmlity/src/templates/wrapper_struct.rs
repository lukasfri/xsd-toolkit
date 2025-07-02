use syn::{parse_quote, Ident};

use crate::templates::specific_enum::TryFromIntoWithMod;

pub struct TryFromStruct<'a> {
    pub repr_type: &'a syn::Type,
}

impl TryFromStruct<'_> {
    pub fn to_error_type(&self, ident: &'_ Ident) -> syn::ItemEnum {
        parse_quote!(
            #[derive(::core::fmt::Debug)]
            pub enum #ident {
            }
        )
    }

    // pub fn to_impl(&self, enum_type: &syn::Type, error_path: &syn::Type) -> syn::ItemImpl {
    //     let repr_type = &self.repr_type;
    //     parse_quote!(
    //         impl ::core::convert::TryFrom<#repr_type> for #enum_type {
    //             type Error = #error_path;

    //             fn try_from(value: #repr_type) -> ::core::result::Result<Self, Self::Error> {
    //                 Ok(Self(value))
    //             }
    //         }
    //     )
    // }
}

pub struct StructInto<'a> {
    pub repr_type: &'a syn::Type,
}

impl<'a> StructInto<'a> {
    pub fn to_impl(self, enum_type: &syn::Type) -> syn::ItemImpl {
        let repr_type = &self.repr_type;
        parse_quote!(
            impl ::core::convert::From<#enum_type> for #repr_type {
                fn from(value: #enum_type) -> Self {
                    value.0
                }
            }
        )
    }
}

pub struct WrapperStruct {
    pub struct_ident: Ident,
    pub repr_type: syn::Type,
    pub enum_with_mod: syn::Ident,
    pub repr: bool,
}

impl WrapperStruct {
    pub fn option_attributes(&self) -> impl Iterator<Item = syn::Meta> {
        let enum_with_mod = &self.enum_with_mod;
        Some(parse_quote! { with = #enum_with_mod }).into_iter()
    }

    fn value_attr(&self) -> syn::Attribute {
        let options = self.option_attributes();
        parse_quote!(#[xvalue(#(#options),*)])
    }

    pub fn to_struct(&self) -> syn::ItemStruct {
        let enum_ident = &self.struct_ident;
        let repr_type = &self.repr_type;
        let attr = self.value_attr();
        let repr_attr = self
            .repr
            .then(|| -> syn::Attribute { parse_quote!(#[repr(transparent)]) });

        parse_quote!(
            #[derive(::core::fmt::Debug, ::xmlity::Serialize, ::xmlity::Deserialize)]
            #attr
            #repr_attr
            pub struct #enum_ident(pub #repr_type);
        )
    }

    pub fn try_from_impl(&self, error_ident: &Ident) -> syn::ItemEnum {
        let impl_ = TryFromStruct {
            repr_type: &self.repr_type,
        };

        impl_.to_error_type(error_ident)

        // let enum_ident = &self.struct_ident;

        // (
        //     impl_.to_error_type(error_ident),
        //     impl_.to_impl(&parse_quote!(#enum_ident), &parse_quote!(#error_ident)),
        // )
    }

    pub fn into_impl(&self) -> syn::ItemImpl {
        let enum_ident = &self.struct_ident;

        StructInto {
            repr_type: &self.repr_type,
        }
        .to_impl(&parse_quote!(#enum_ident))
    }

    pub fn with_mod(&self) -> syn::ItemMod {
        let struct_ident = &self.struct_ident;

        TryFromIntoWithMod {
            repr_type: &self.repr_type,
            destination_type: &parse_quote!(super::#struct_ident),
            mod_name: &self.enum_with_mod,
        }
        .with_mod()
    }
}

// #[cfg(test)]
// mod tests {
//     use pretty_assertions::assert_eq;

//     use super::*;
//     use quote::format_ident;
//     use syn::{parse_quote, ItemEnum};
//     use xmlity::LocalName;

//     fn item_to_string<T: Into<syn::Item>>(item: T) -> String {
//         let item: syn::Item = item.into();
//         let file = syn::File {
//             shebang: None,
//             attrs: Vec::new(),
//             items: vec![item],
//         };
//         prettyplease::unparse(&file)
//     }

//     #[test]
//     fn generate_no_variant_choice() {
//         let record = Choice {
//             repr_type: parse_quote!(i32),
//             variants: Vec::new(),
//         };

//         todo!()
//         // let ident = format_ident!("Test");

//         // let actual_item = record.to_enum(&ident,
//         //     &parse_quote!(deserialize_with),
//         //     &parse_quote!(serialize_with),
//         // );

//         // let expected_item: ItemEnum = parse_quote!(
//         //     #[derive(::core::fmt::Debug, ::xmlity::Serialize, ::xmlity::Deserialize)]
//         //     #[repr(i32)]
//         //     pub enum Test {}
//         // );

//         // assert_eq!(expected_item, actual_item);
//     }

//     #[test]
//     fn generate_one_value_choice() {
//         let record = Choice {
//             repr_type: parse_quote!(i32),
//             variants: vec![(
//                 format_ident!("A"),
//                 ChoiceVariantType {
//                     value_expr: parse_quote!(1),
//                 },
//             )],
//         };

//         let ident = format_ident!("Test");
//         let deserialize_with_ident = format_ident!("deserialize_with");
//         let serialize_with_ident = format_ident!("serialize_with");

//         let actual_deserialize_fn =
//             record.deserialize_with_fn(&deserialize_with_ident, &parse_quote!(#ident), |_| {
//                 Vec::new()
//             });

//         // Signature: fn<'de, D>(D) -> Result<T, D::Error>
//         let expected_deserialize_fn: syn::ItemFn = parse_quote!(
//             pub fn deserialize_with<'de, D>(
//                 deserializer: D,
//             ) -> ::core::result::Result<Test, D::Error>
//             where
//                 D: ::xmlity::Deserializer<'de>,
//             {
//                 let text: ::std::string::String = ::xmlity::Deserialize::deserialize(deserializer)?;

//                 let value: i32 = text.parse().map_err(|_| {
//                     ::xmlity::de::Error::custom(format!("Failed to parse value for Test: {}", text))
//                 })?;

//                 match value {
//                     1 => Ok(Test::A),
//                     _ => Err(::xmlity::de::Error::custom(format!(
//                         "Unexpected value for Test: {}",
//                         value
//                     ))),
//                 }
//             }
//         );

//         assert_eq!(
//             item_to_string(expected_deserialize_fn),
//             item_to_string(actual_deserialize_fn)
//         );

//         let actual_serialize_fn =
//             record.serialize_with_fn(&serialize_with_ident, &parse_quote!(#ident));

//         // Signature: fn<S>(&T, S) -> Result<S::Ok, S::Error> where S: Serializer
//         let expected_serialize_fn: syn::ItemFn = parse_quote!(
//             pub fn serialize_with<S>(
//                 &value: &Test,
//                 serializer: S,
//             ) -> ::core::result::Result<S::Ok, S::Error>
//             where
//                 S: ::xmlity::Serializer,
//             {
//                 let value: i32 = match value {
//                     Test::A => 1,
//                 };

//                 ::xmlity::Serialize::serialize(
//                     ::std::string::ToString::to_string(&value),
//                     serializer,
//                 )
//             }
//         );

//         assert_eq!(
//             item_to_string(expected_serialize_fn),
//             item_to_string(actual_serialize_fn)
//         );

//         let actual_item = record.to_enum(
//             &ident,
//             &parse_quote!(#deserialize_with_ident),
//             &parse_quote!(#serialize_with_ident),
//         );

//         let expected_item: ItemEnum = parse_quote!(
//             #[derive(::core::fmt::Debug, ::xmlity::Serialize, ::xmlity::Deserialize)]
//             #[xvalue(deserialize_with = #deserialize_with_ident, serialize_with = #serialize_with_ident)]
//             pub enum Test {
//                 A,
//             }
//         );

//         assert_eq!(item_to_string(expected_item), item_to_string(actual_item));
//     }
// }
