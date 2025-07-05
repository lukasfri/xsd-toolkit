use std::fmt::Debug;

use dyn_clone::DynClone;
use quote::{format_ident, ToTokens};
use syn::{Path, Type, TypePath};

pub trait TypeReferenceTrait: FnOnce(Option<&Path>) -> Type + DynClone {}

dyn_clone::clone_trait_object!(TypeReferenceTrait);

impl<F> TypeReferenceTrait for F where F: FnOnce(Option<&Path>) -> Type + DynClone {}

/// This is a wrapper around a function that takes an optional path and returns a type. It's used for types, which are very often relative to the current module. This allows us to defer the final formulation of the type until the location of the type is known.
///
/// For example, if a type A refers to a type B in a different module, the type reference for B will be a function that takes the path to the module containing A and returns the type B.
#[derive(Clone)]
pub struct TypeReference<'a>(Box<dyn TypeReferenceTrait + 'a>);

impl Debug for TypeReference<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TypeReference").finish()
    }
}

impl<'a> TypeReference<'a> {
    pub fn new<F>(f: F) -> Self
    where
        F: TypeReferenceTrait + 'a,
    {
        Self(Box::new(f))
    }

    pub fn new_prefixed_type(type_: TypePath) -> Self {
        Self::new(move |path| {
            if let Some(path) = path {
                syn::parse_quote! { #path::#type_ }
            } else {
                syn::parse_quote! { #type_ }
            }
        })
    }

    pub fn prefix(self, prefix: Path) -> Self {
        TypeReference(Box::new(move |path: Option<&Path>| {
            if let Some(path) = path {
                self.to_type(Some(&syn::parse_quote! { #path::#prefix }))
            } else {
                self.to_type(Some(&prefix))
            }
        }))
    }

    pub fn new_static(type_: Type) -> Self {
        Self::new(move |_| type_)
    }

    pub fn to_type(&self, path: Option<&Path>) -> Type {
        (self.0.clone())(path)
    }

    pub fn into_type(self, path: Option<&Path>) -> Type {
        (self.0)(path)
    }

    pub fn wrap<F>(self, wrapper: F) -> Self
    where
        F: FnOnce(Type) -> Type + Clone + 'static,
    {
        Self::new(move |path| wrapper(self.to_type(path)))
    }

    pub fn option_wrapper(ty: Type) -> Type {
        syn::parse_quote! { ::core::option::Option<#ty> }
    }

    pub fn vec_wrapper(ty: Type) -> Type {
        syn::parse_quote! { ::std::vec::Vec<#ty> }
    }

    pub fn vec_non_boxed_wrapper(ty: Type) -> Type {
        let ty = match &ty {
            Type::Path(path) => Some(path),
            _ => None,
        }
        .and_then(unbox_type)
        .unwrap_or(ty);

        Self::vec_wrapper(ty)
    }

    pub fn box_wrapper(ty: Type) -> Type {
        syn::parse_quote! { ::std::boxed::Box<#ty> }
    }

    pub fn box_non_boxed_wrapper(ty: Type) -> Type {
        if match &ty {
            Type::Path(path) => Some(path),
            _ => None,
        }
        .and_then(unbox_type)
        .is_some()
        {
            ty
        } else {
            Self::box_wrapper(ty)
        }
    }

    pub fn wrap_if<F>(self, condition: bool, wrapper: F) -> Self
    where
        F: FnOnce(Type) -> Type + Clone + 'static,
    {
        if condition {
            self.wrap(wrapper)
        } else {
            self
        }
    }
}

pub fn unkeywordify(ident: &str) -> String {
    match ident {
        "type" => "type_".to_string(),
        "ref" => "ref_".to_string(),
        "match" => "match_".to_string(),
        "enum" => "enum_".to_string(),
        "self" => "self_".to_string(),
        "super" => "super_".to_string(),
        "crate" => "crate_".to_string(),
        "extern" => "extern_".to_string(),
        "use" => "use_".to_string(),
        "where" => "where_".to_string(),
        "as" => "as_".to_string(),
        "async" => "async_".to_string(),
        "await" => "await_".to_string(),
        "dyn" => "dyn_".to_string(),
        "union" => "union_".to_string(),
        "static" => "static_".to_string(),
        "const" => "const_".to_string(),
        "fn" => "fn_".to_string(),
        "for" => "for_".to_string(),
        "if" => "if_".to_string(),
        "else" => "else_".to_string(),
        "loop" => "loop_".to_string(),
        "while" => "while_".to_string(),
        "break" => "break_".to_string(),
        "continue" => "continue_".to_string(),
        "return" => "return_".to_string(),
        "in" => "in_".to_string(),
        "let" => "let_".to_string(),
        "impl" => "impl_".to_string(),
        "trait" => "trait_".to_string(),
        "struct" => "struct_".to_string(),
        "override" => "override_".to_string(),
        "abstract" => "abstract_".to_string(),
        "final" => "final_".to_string(),
        _ => ident.to_string(),
    }
}

pub fn rustify_name(ident: &str) -> String {
    let ident = unkeywordify(ident).replace("-", "_").replace('#', "");

    //Capitalize_first_letter
    ident
        .chars()
        .enumerate()
        .map(|(i, c)| if i == 0 { c.to_ascii_uppercase() } else { c })
        .collect::<String>()
}

// Returns the type that is boxed, if any.
// Examples:
// - Box<Foo> -> Foo
// - Box<dyn Foo> -> dyn Foo
// - Foo -> None
// - ::std::boxed::Box<Foo> -> Foo
// - ::std::boxed::Box<dyn Foo> -> dyn Foo
pub fn unbox_type(ty: &syn::TypePath) -> Option<syn::Type> {
    let mut segments = ty.path.segments.iter();

    let first = segments.next().unwrap();
    if first.ident == "Box" {
        let argument = match &first.arguments {
            syn::PathArguments::AngleBracketed(arguments) => arguments,
            syn::PathArguments::Parenthesized(_) | syn::PathArguments::None => return None,
        };

        Some(syn::parse2(argument.args.to_token_stream()).unwrap())
    } else if first.ident == "std" {
        let _boxed = segments.next().filter(|a| a.ident == "boxed")?;

        let box_ = segments.next().filter(|a| a.ident == "Box")?;

        let argument = match &box_.arguments {
            syn::PathArguments::AngleBracketed(arguments) => arguments,
            syn::PathArguments::Parenthesized(_) | syn::PathArguments::None => return None,
        };

        Some(syn::parse2(argument.args.to_token_stream()).unwrap())
    } else {
        None
    }
}

pub fn unvec_type(ty: &syn::TypePath) -> Option<syn::Type> {
    let mut segments = ty.path.segments.iter();

    let first = segments.next().unwrap();
    if first.ident == "Vec" {
        let argument = match &first.arguments {
            syn::PathArguments::AngleBracketed(arguments) => arguments,
            syn::PathArguments::Parenthesized(_) | syn::PathArguments::None => return None,
        };

        Some(syn::parse2(argument.args.to_token_stream()).unwrap())
    } else if first.ident == "std" {
        let _vec = segments.next().filter(|a| a.ident == "vec")?;

        let vec_ = segments.next().filter(|a| a.ident == "Vec")?;

        let argument = match &vec_.arguments {
            syn::PathArguments::AngleBracketed(arguments) => arguments,
            syn::PathArguments::Parenthesized(_) | syn::PathArguments::None => return None,
        };

        Some(syn::parse2(argument.args.to_token_stream()).unwrap())
    } else {
        None
    }
}

pub const COMMON_NAME_MIN_LENGTH: usize = 4;

// Tries to find a common name between a list of strings.
// ["SimpleType", "ComplexType"] becomes Some("Type")
// ["One", "Two"] becomes None
pub fn common_name<I: IntoIterator<Item = T>, T: AsRef<str>>(
    names: I,
    min_length: usize,
) -> Option<String> {
    let names: Vec<String> = names.into_iter().map(|s| s.as_ref().to_string()).collect();
    if names.is_empty() {
        return None;
    }
    if names.len() == 1 {
        return Some(names[0].clone());
    }

    let min_len = names.iter().map(|s| s.len()).min().unwrap();
    let first = &names[0];

    for len in (min_length.max(1)..=min_len).rev() {
        let mut candidates = Vec::new();

        for start in 0..=(first.len() - len) {
            let substring = &first[start..start + len];
            if names.iter().skip(1).all(|s| s.contains(substring)) {
                candidates.push(substring.to_string());
            }
        }

        // First find if there is a type if a capital letter start, otherwise, pick the last.
        let candidate_index = candidates
            .iter()
            .position(|a| a.chars().next().unwrap().is_ascii_uppercase())
            .or_else(|| (!candidates.is_empty()).then(|| candidates.len() - 1));

        if let Some(candidate_index) = candidate_index {
            return candidates.into_iter().nth(candidate_index);
        }
    }

    None
}

pub fn dedup_field_idents<T>(
    fields: impl IntoIterator<Item = (syn::Ident, T)>,
) -> Vec<(syn::Ident, T)> {
    //This function in case of multiple fields having the same ident (ex. "field") should name them field_0, field_1, etc. Order must be preserved.
    let mut seen_idents = std::collections::HashSet::new();
    let mut deduped_fields = Vec::new();

    for (ident, value) in fields {
        let mut new_ident = ident.clone();
        let mut counter = 0;

        while seen_idents.contains(&new_ident) {
            new_ident = format_ident!("{}_{}", ident, counter.to_string());
            counter += 1;
        }

        seen_idents.insert(new_ident.clone());
        deduped_fields.push((new_ident, value));
    }

    deduped_fields
}

#[cfg(test)]
mod tests {
    use syn::parse_quote;

    use super::*;

    #[test]
    fn unbox() {
        assert_eq!(
            unbox_type(&parse_quote!(::std::boxed::Box<i32>)),
            Option::<syn::Type>::Some(parse_quote!(i32))
        );
        assert_eq!(
            unbox_type(&parse_quote!(Box<i32>)),
            Option::<syn::Type>::Some(parse_quote!(i32))
        );
        assert_eq!(unbox_type(&parse_quote!(i32)), None);
    }

    #[test]
    fn unvec() {
        assert_eq!(
            unvec_type(&parse_quote!(::std::vec::Vec<documentation_items::Child0>)),
            Option::<syn::Type>::Some(parse_quote!(documentation_items::Child0))
        );
    }

    #[test]
    fn common_name() {
        assert_eq!(
            super::common_name(["SimpleType", "ComplexType"], 3),
            Some("Type".to_string())
        );
        assert_eq!(
            super::common_name(["SimpleType", "ComplexType"], 4),
            Some("Type".to_string())
        );
        assert_eq!(super::common_name(["SimpleType", "ComplexType"], 5), None);

        assert_eq!(super::common_name(["One", "Two"], 1), None);
    }
}
