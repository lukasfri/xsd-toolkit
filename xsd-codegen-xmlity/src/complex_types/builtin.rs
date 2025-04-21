#![allow(non_camel_case_types)]

use proc_macro2::Span;
use syn::{parse_quote, Ident, Item};

use crate::{Context, Result, TypeGenerator, TypesOutput};

pub struct stringGenerator;
impl TypeGenerator for stringGenerator {
    type Input = xsd::builtin::string;
    fn generate_type(&self, input: &Self::Input, _context: &Context) -> Result<TypesOutput> {
        let ident = Ident::new("string", Span::call_site());

        let output = TypesOutput::new(
            Item::Type(parse_quote!(
                pub type #ident = String;
            )),
            ident,
        );

        Ok(output)
    }
}

pub struct booleanGenerator;
impl TypeGenerator for booleanGenerator {
    type Input = xsd::builtin::boolean;
    fn generate_type(&self, input: &Self::Input, _context: &Context) -> Result<TypesOutput> {
        let ident = Ident::new("boolean", Span::call_site());

        let output = TypesOutput::new(
            Item::Type(parse_quote!(
                pub type #ident = bool;
            )),
            ident,
        );

        Ok(output)
    }
}

pub struct decimalGenerator;
impl TypeGenerator for decimalGenerator {
    type Input = xsd::builtin::decimal;

    fn generate_type(&self, input: &Self::Input, _context: &Context) -> Result<TypesOutput> {
        let ident = Ident::new("decimal", Span::call_site());

        let output = TypesOutput::new(
            Item::Type(parse_quote!(
                pub type #ident = f64;
            )),
            ident,
        );

        Ok(output)
    }
}

pub struct floatGenerator;
impl TypeGenerator for floatGenerator {
    type Input = xsd::builtin::float;

    fn generate_type(&self, input: &Self::Input, _context: &Context) -> Result<TypesOutput> {
        let ident = Ident::new("float", Span::call_site());

        let output = TypesOutput::new(
            Item::Type(parse_quote!(
                pub type #ident = f32;
            )),
            ident,
        );

        Ok(output)
    }
}
