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
