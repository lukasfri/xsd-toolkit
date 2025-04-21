use proc_macro2::Span;
use syn::{parse_quote, Ident, Item};

use crate::{
    annotation_to_doc_attribute, rustify_name, Context, Result, TypeGenerator, TypesOutput,
};

pub struct LocalComplexTypeGenerator;
impl TypeGenerator for LocalComplexTypeGenerator {
    type Input = xsd::schema::LocalComplexType;

    fn generate_type(&self, input: &Self::Input, context: &Context) -> Result<TypesOutput> {
        let derive_traits = context.derive_traits.as_slice();

        let name = context
            .name_context
            .as_ref()
            .expect("LocalComplexType must have a name from the parent element");
        let ident = Ident::new(rustify_name(name).as_str(), Span::call_site());

        let doc_attribute = input
            .annotation
            .as_ref()
            .and_then(annotation_to_doc_attribute);

        let complex_type = parse_quote!(
            #doc_attribute
            #[derive(#(#derive_traits,)* SerializationGroup, DeserializationGroup)]
            #[xgroup]
            pub struct #ident {

            }
        );

        let output = TypesOutput::new(Item::Struct(complex_type), ident);

        Ok(output)
    }
}

pub struct TopLevelComplexTypeGenerator;
impl TypeGenerator for TopLevelComplexTypeGenerator {
    type Input = xsd::schema::TopLevelComplexType;

    fn generate_type(&self, input: &Self::Input, context: &Context) -> Result<TypesOutput> {
        let derive_traits = context.derive_traits.as_slice();

        let id = input.name.as_ref().unwrap();
        let id = id.0.as_str();
        let ident = Ident::new(rustify_name(id).as_str(), Span::call_site());

        let doc_attribute = input
            .annotation
            .as_ref()
            .and_then(annotation_to_doc_attribute);

        let complex_type = parse_quote!(
            #doc_attribute
            #[derive(#(#derive_traits,)* SerializationGroup, DeserializationGroup)]
            #[xgroup]
            pub struct #ident {

            }
        );

        let output = TypesOutput::new(Item::Struct(complex_type), ident);

        Ok(output)
    }
}

pub struct GroupTypeGenerator;
impl TypeGenerator for GroupTypeGenerator {
    type Input = xsd::schema::GroupType;
    fn generate_type(&self, input: &Self::Input, context: &Context) -> Result<TypesOutput> {
        let derive_traits = context.derive_traits.as_slice();

        let id = input.name.as_ref().unwrap();
        let id = id.0.as_str();
        let ident = Ident::new(rustify_name(id).as_str(), Span::call_site());

        let doc_attribute = input
            .annotation
            .as_ref()
            .and_then(annotation_to_doc_attribute);

        let group = Item::Struct(parse_quote!(
            #doc_attribute
            #[derive(#(#derive_traits,)* SerializationGroup, DeserializationGroup)]
            #[xgroup]
            pub struct #ident {

            }
        ));

        let output = TypesOutput::new(group, ident);

        Ok(output)
    }
}

pub struct AttributeGroupTypeGenerator;
impl TypeGenerator for AttributeGroupTypeGenerator {
    type Input = xsd::schema::AttributeGroupType;
    fn generate_type(&self, input: &Self::Input, context: &Context) -> Result<TypesOutput> {
        let derive_traits = context.derive_traits.as_slice();

        let id = input.name.as_ref().unwrap();
        let id = id.0.as_str();
        let ident = Ident::new(rustify_name(id).as_str(), Span::call_site());

        let doc_attribute = input
            .annotation
            .as_ref()
            .and_then(annotation_to_doc_attribute);

        let attribute_group = parse_quote!(
            #doc_attribute
            #[derive(#(#derive_traits,)* SerializationGroup, DeserializationGroup)]
            #[xgroup]
            pub struct #ident {

            }
        );

        let output = TypesOutput::new(Item::Struct(attribute_group), ident);

        Ok(output)
    }
}
