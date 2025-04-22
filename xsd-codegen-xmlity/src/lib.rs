use core::str;

use complex_types::other::{
    AttributeGroupTypeGenerator, GroupTypeGenerator, TopLevelComplexTypeGenerator,
};
use quote::ToTokens;
use syn::{parse_quote, Item, ItemConst, ItemMod, ItemUse};
use top_level::{
    TopLevelAttributeGenerator, TopLevelElementGenerator, TopLevelSimpleTypeGenerator,
};
use xmlity::XmlValue;
use xsd::schema::{
    Annotation, AnnotationContent, Namespace, QNameRef, Redefineable, Schema, SchemaTop, Source,
};
pub mod complex_types;
pub mod simple_types;
pub mod top_level;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Error when interacting with simple type compiler: {0}")]
    CompiledSimpleType(#[from] xsd_simple_type_compiler::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Copy)]
pub enum NamespaceExpr<'a> {
    Const(&'a str),
    Path(&'a syn::Expr),
}

impl NamespaceExpr<'_> {
    pub fn into_meta_name_value(&self) -> syn::MetaNameValue {
        match self {
            NamespaceExpr::Const(_) => parse_quote!(namespace = #self),
            NamespaceExpr::Path(expr) => parse_quote!(namespace_expr = #expr),
        }
    }
}

impl ToTokens for NamespaceExpr<'_> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.into_meta_name_value().to_tokens(tokens);
    }
}

pub struct Context<'a> {
    pub namespace: Option<Namespace>,
    pub namespace_expr: NamespaceExpr<'a>,
    pub derive_traits: Vec<syn::Path>,
    pub name_context: Option<String>,
    pub schema: Option<&'a Schema>,
    pub can_change_name: bool,
}

pub enum XmlType {
    Element,
    Attribute,
    SimpleType,
    ComplexType,
}

#[derive(Clone)]
pub struct ResolvedType {
    pub type_: syn::Type,
    pub ident: syn::Ident,
}

impl Context<'_> {
    pub fn sub_context<T: Into<String>>(&self, name_context: T) -> Context {
        Context {
            namespace: self.namespace.clone(),
            namespace_expr: self.namespace_expr,
            derive_traits: self.derive_traits.clone(),
            name_context: Some(name_context.into()),
            schema: self.schema,
            can_change_name: false,
        }
    }

    pub fn with_can_change_name(mut self, can_change_name: bool) -> Self {
        self.can_change_name = can_change_name;
        self
    }

    pub fn resolve_type(&self, qname: &QNameRef) -> ResolvedType {
        if qname.namespace == self.namespace {
            let name = &qname.name;
            let rust_name = rustify_name(name);
            let rust_name = syn::Ident::new(&rust_name, proc_macro2::Span::call_site());

            ResolvedType {
                type_: parse_quote!(xsd_root::types::#rust_name),
                ident: rust_name,
            }
        } else {
            ResolvedType {
                type_: parse_quote!(todo!()),
                ident: parse_quote!(todo!()),
            }
        }
    }
}

pub struct TypesOutput {
    pub target_ident: syn::Ident,
    pub item: Option<syn::Item>,
    pub related: Vec<syn::Item>,
}

impl ToTokens for TypesOutput {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.related_mod().to_tokens(tokens);
        self.item.to_tokens(tokens);
    }
}

impl TypesOutput {
    pub fn new(target: syn::Item, target_ident: syn::Ident) -> Self {
        Self {
            target_ident,
            item: Some(target),
            related: Vec::new(),
        }
    }

    pub fn to_mod_name(data_ident: &syn::Ident) -> syn::Ident {
        fn snake_caseify(ident: &str) -> String {
            let mut snake_case = String::new();
            let mut chars = ident.chars();
            if let Some(first) = chars.next() {
                snake_case.push(first.to_ascii_lowercase());
            }
            for c in chars {
                if c.is_uppercase() {
                    snake_case.push('_');
                    snake_case.push(c.to_ascii_lowercase());
                } else {
                    snake_case.push(c);
                }
            }
            snake_case
        }

        let mod_ident = snake_caseify(&data_ident.to_string());
        syn::Ident::new(&mod_ident, proc_macro2::Span::call_site())
    }

    pub fn mod_name(&self) -> syn::Ident {
        Self::to_mod_name(&self.target_ident)
    }

    pub fn related_mod(&self) -> Option<syn::ItemMod> {
        let related = &self.related;
        if related.is_empty() {
            return None;
        }

        let mod_ident = self.mod_name();

        Some(parse_quote!(
            pub mod #mod_ident {
                use super::xsd_root;
                use xmlity::{Serialize, Deserialize, DeserializationGroup, SerializationGroup};
                #(#related)*
            }
        ))
    }
}

pub trait TypeGenerator {
    type Input;

    fn generate_type(&self, input: &Self::Input, context: &Context) -> Result<TypesOutput>;
}

fn unkeywordify(ident: &str) -> String {
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
        _ => ident.to_string(),
    }
}

fn rustify_name(ident: &str) -> String {
    let ident = unkeywordify(ident).replace("-", "_").replace('#', "");

    //Capitalize_first_letter
    ident
        .chars()
        .enumerate()
        .map(|(i, c)| if i == 0 { c.to_ascii_uppercase() } else { c })
        .collect::<String>()
}

fn annotation_to_doc_string(annotation: &Annotation) -> Option<String> {
    let mut sources: Vec<Source> = vec![];
    let mut doc_comment: Option<String> = None;
    for documentation in annotation.content.iter() {
        let AnnotationContent::Documentation(documentation) = documentation else {
            continue;
        };
        if let Some(source) = &documentation.source {
            sources.push(source.clone());
        }
        let doc_string = documentation
            .any
            .iter()
            .filter_map(|any| match any {
                XmlValue::Text(text) => Some(str::from_utf8(&text.0).expect("Expected UTF-8")),
                _ => None,
            })
            .fold(String::new(), |acc, e| format!("{acc}\n{e}"));

        if doc_comment.is_none() {
            doc_comment = Some(doc_string);
        } else if let Some(doc_comment) = &mut doc_comment {
            doc_comment.push_str(&doc_string);
        }
    }

    let mut doc_comment = doc_comment?;
    doc_comment = doc_comment.trim().to_owned();

    if !sources.is_empty() {
        doc_comment.push_str("\n\nSources:\n");
        for source in sources.iter().map(|a| a.0.as_str()) {
            doc_comment.push_str(&format!("- {source}\n"));
        }

        doc_comment = doc_comment.trim().to_owned();
    }

    if doc_comment.is_empty() {
        None
    } else {
        Some(doc_comment)
    }
}

fn annotation_to_doc_attribute(annotation: &Annotation) -> Option<syn::Attribute> {
    let doc_comment = annotation_to_doc_string(annotation)?;

    Some(parse_quote!(#[doc = #doc_comment]))
}

#[non_exhaustive]
pub struct Generator {}

impl Generator {
    pub fn generate(schema: &Schema) -> Result<Vec<Item>> {
        let namespace = schema.target_namespace.as_deref().unwrap();

        let namespace_expr: ItemConst = parse_quote! {
            pub const NAMESPACE: xmlity::XmlNamespace = xmlity::XmlNamespace::new_dangerous(#namespace);
        };

        let namespace_path: syn::ExprPath = parse_quote!(super::NAMESPACE);
        let namespace_path = syn::Expr::Path(namespace_path);

        let derive_traits = vec![
            parse_quote!(Debug),
            // parse_quote!(Clone),
            // parse_quote!(Eq),
            parse_quote!(PartialEq),
        ];
        let context = Context {
            namespace: Some(Namespace(namespace.to_owned())),
            namespace_expr: NamespaceExpr::Path(&namespace_path),
            derive_traits,
            // Top level are expected to take their own names.
            name_context: None,
            schema: Some(schema),
            can_change_name: false,
        };
        let mut types: Vec<TypesOutput> = Vec::new();
        let mut groups: Vec<TypesOutput> = Vec::new();
        let mut attribute_groups: Vec<TypesOutput> = Vec::new();
        let mut elements: Vec<TypesOutput> = Vec::new();
        let mut attributes: Vec<TypesOutput> = Vec::new();

        for element in schema.schema_top.iter() {
            match element {
                SchemaTop::Redefineable(Redefineable::SimpleType(simple_type)) => {
                    let item = TopLevelSimpleTypeGenerator.generate_type(simple_type, &context)?;
                    types.push(item);
                }
                SchemaTop::Redefineable(Redefineable::ComplexType(complex_type)) => {
                    let item =
                        TopLevelComplexTypeGenerator.generate_type(complex_type, &context)?;
                    types.push(item);
                }
                SchemaTop::Redefineable(Redefineable::Group(group_type)) => {
                    let item = GroupTypeGenerator.generate_type(group_type, &context)?;
                    groups.push(item);
                }
                SchemaTop::Redefineable(Redefineable::AttributeGroup(attribute_group_type)) => {
                    let item = AttributeGroupTypeGenerator
                        .generate_type(attribute_group_type, &context)?;
                    attribute_groups.push(item);
                }

                SchemaTop::Element(element_type) => {
                    let item = TopLevelElementGenerator.generate_type(element_type, &context)?;
                    elements.push(item);
                }
                SchemaTop::Attribute(attribute_type) => {
                    let item =
                        TopLevelAttributeGenerator.generate_type(attribute_type, &context)?;
                    attributes.push(item);
                }
                SchemaTop::Notation(_notation) => {}
                SchemaTop::Annotation(_annotation) => {}
            }
        }

        let group_import_expression: ItemUse = parse_quote! {
            use xmlity::{Serialize, Deserialize, SerializationGroup, DeserializationGroup};
        };

        let value_import_expression: ItemUse = parse_quote! {
            use xmlity::{Serialize, Deserialize};
        };

        let attribute_import_expression: ItemUse = parse_quote! {
            use xmlity::{SerializeAttribute, Deserialize};
        };

        let xsd_root_mod: ItemMod = parse_quote! {
            mod xsd_root {
                pub use super::super::{attribute_groups, groups, types};
            }
        };

        let types: Option<ItemMod> = if !types.is_empty() {
            Some(parse_quote! {
                pub mod types {
                    #xsd_root_mod
                    #group_import_expression
                    #(#types)*
                }
            })
        } else {
            None
        };
        let groups: Option<ItemMod> = if !groups.is_empty() {
            Some(parse_quote! {
                pub mod groups {
                    #xsd_root_mod
                    #group_import_expression
                    #(#groups)*
                }
            })
        } else {
            None
        };

        let attribute_groups: Option<ItemMod> = if !attribute_groups.is_empty() {
            Some(parse_quote! {
                pub mod attribute_groups {
                    #xsd_root_mod
                    #group_import_expression
                    #(#attribute_groups)*
                }
            })
        } else {
            None
        };
        let elements: Option<ItemMod> = if !elements.is_empty() {
            Some(parse_quote! {
                pub mod elements {
                    #xsd_root_mod
                    #value_import_expression
                    #(#elements)*
                }
            })
        } else {
            None
        };
        let attributes: Option<ItemMod> = if !attributes.is_empty() {
            Some(parse_quote! {
                pub mod attributes {
                    #xsd_root_mod
                    #attribute_import_expression
                    #(#attributes)*
                }
            })
        } else {
            None
        };

        Ok(vec![
            Some(namespace_expr.into()),
            types.map(Into::into),
            groups.map(Into::into),
            attribute_groups.map(Into::into),
            elements.map(Into::into),
            attributes.map(Into::into),
        ]
        .into_iter()
        .flatten()
        .collect())
    }
}

#[cfg(test)]
mod tests {
    use xmlity::{types::value::XmlText, XmlValue};
    use xsd::schema::*;

    #[test]
    fn annotation_to_doc_comment() {
        let annotation = Annotation {
            id: None,
            content: vec![
                AnnotationContent::Documentation(
                    Documentation {
                        source: None,
                        lang: None,
                        any: vec![
                            XmlValue::Text(
                                XmlText::new(
                                    "An abstract element, representing facets in general.\nThe facets defined by this spec are substitutable for\nthis element, and implementation-defined facets should\nalso name this as a substitution-group head.\n",
                                ),
                            ),
                        ],
                    },
                ),
            ],
        };

        let actual = super::annotation_to_doc_string(&annotation).unwrap();
        let expected = "An abstract element, representing facets in general.\nThe facets defined by this spec are substitutable for\nthis element, and implementation-defined facets should\nalso name this as a substitution-group head.";

        assert_eq!(actual, expected);
    }
}
