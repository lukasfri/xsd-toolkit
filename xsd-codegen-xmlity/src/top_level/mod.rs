use std::ops::Deref;

use proc_macro2::Span;
use syn::{parse_quote, Ident, Item, ItemEnum};
use xsd::schema::ElementTypeContent;
use xsd_simple_type_compiler::{SimpleTypeCompiler, TopLevelSimpleTypeCompiler};

use crate::{
    annotation_to_doc_attribute, complex_types::other::LocalComplexTypeGenerator, rustify_name,
    simple_types::CompiledSimpleTypeGenerator, Context, Result, TypeGenerator, TypesOutput,
};

pub struct TopLevelElementGenerator;
impl TypeGenerator for TopLevelElementGenerator {
    type Input = xsd::schema::TopLevelElement;

    fn generate_type(&self, input: &Self::Input, context: &Context) -> Result<TypesOutput> {
        let derive_traits = context.derive_traits.as_slice();
        let namespace_meta = &context.namespace_expr;

        let id = input.name.as_ref().unwrap();
        let id = id.0.as_str();
        let rust_name = rustify_name(id);
        let ident = Ident::new(&rust_name, Span::call_site());

        let mod_ident = Ident::new(&format!("{rust_name}_items"), Span::call_site());

        let mut local_items: Vec<Item> = Vec::new();

        let doc_attribute = input
            .annotation
            .as_ref()
            .and_then(annotation_to_doc_attribute);

        let content_type = match (&input.type_choice, &input.type_, &input.abstract_) {
            (Some(ElementTypeContent::SimpleType(_simple_local_type)), None, None) => todo!(),
            (Some(ElementTypeContent::ComplexType(local_complex_type)), None, None) => {
                let context = context.sub_context(format!("{rust_name}Content"));

                let output = LocalComplexTypeGenerator
                    .generate_type(local_complex_type.deref(), &context)?;

                local_items.extend(output.related_mod().map(Item::Mod));
                let target_ident = &output.target_ident;
                let path_to_target: syn::TypePath = parse_quote!(#mod_ident::#target_ident);
                local_items.extend(output.item);

                path_to_target.into()
            }
            (None, Some(type_), None) => context.resolve_type(type_).type_,
            (None, None, Some(true)) => {
                let substituted_elements = context
                    .schema
                    .expect("Expected schema to be present")
                    .schema_top
                    .iter()
                    .filter_map(|a| match a {
                        xsd::schema::SchemaTop::Element(top_level_element) => {
                            Some(top_level_element)
                        }
                        _ => None,
                    })
                    .filter(|a| {
                        let qname = xsd::schema::QNameRef {
                            name: id.to_owned(),
                            namespace: context
                                .schema
                                .as_ref()
                                .expect("Expected schema to be present")
                                .target_namespace
                                .clone()
                                .map(xsd::schema::Namespace),
                        };
                        a.substitution_group.contains(&qname)
                    });

                let substituted_elements = substituted_elements
                    .filter_map(|a| a.name.as_ref().map(|a| a.0.as_str()))
                    .map(rustify_name)
                    .map(|a| Ident::new(&a, Span::call_site()))
                    .collect::<Vec<_>>();

                let abstract_enum: ItemEnum = parse_quote!(
                    #doc_attribute
                    #[derive(#(#derive_traits,)* Serialize, Deserialize)]
                    pub enum #ident {
                        #(#substituted_elements(#substituted_elements)),*
                    }
                );

                return Ok(TypesOutput::new(Item::Enum(abstract_enum), ident));
            }
            _ => todo!("Handle this case: {:?}", id),
        };
        let element = parse_quote!(
            #doc_attribute
            #[derive(#(#derive_traits,)* Serialize, Deserialize)]
            #[xelement(name = #id, #namespace_meta)]
            pub struct #ident(#[xgroup] #content_type);
        );

        let mut output = TypesOutput::new(Item::Struct(element), ident);

        output.related = local_items;

        Ok(output)
    }
}

pub struct TopLevelAttributeGenerator;
impl TypeGenerator for TopLevelAttributeGenerator {
    type Input = xsd::schema::TopLevelAttribute;

    fn generate_type(&self, input: &Self::Input, context: &Context) -> Result<TypesOutput> {
        let derive_traits = context.derive_traits.as_slice();
        let namespace_meta = &context.namespace_expr;

        let id = input.name.as_ref().unwrap();
        let id = id.0.as_str();
        let ident = Ident::new(rustify_name(id).as_str(), Span::call_site());

        let doc_attribute = input
            .annotation
            .as_ref()
            .and_then(annotation_to_doc_attribute);

        let attribute = parse_quote!(
            #doc_attribute
            #[derive(#(#derive_traits,)* SerializeAttribute, Deserialize)]
            #[xattribute(name = #id, #namespace_meta)]
            pub struct #ident(pub String);
        );

        let output = TypesOutput::new(Item::Struct(attribute), ident);

        Ok(output)
    }
}

pub struct TopLevelSimpleTypeGenerator;
impl TypeGenerator for TopLevelSimpleTypeGenerator {
    type Input = xsd::schema::TopLevelSimpleType;

    fn generate_type(&self, input: &Self::Input, context: &Context) -> Result<TypesOutput> {
        let id = input.name.as_ref().unwrap();
        let id = id.0.as_str();
        let _rust_name = rustify_name(id);

        let _doc_attribute = input
            .annotation
            .as_ref()
            .and_then(annotation_to_doc_attribute);

        //TODO: Add support for final attribute and add docs to generated types
        // SimpleDerivationTypeGenerator.generate_type(&input.content, &context.sub_context(rust_name))

        let compiled = TopLevelSimpleTypeCompiler
            .compile_simple_type(input, &xsd_simple_type_compiler::Context)
            .unwrap();

        CompiledSimpleTypeGenerator.generate_type(&compiled, context)
    }
}
