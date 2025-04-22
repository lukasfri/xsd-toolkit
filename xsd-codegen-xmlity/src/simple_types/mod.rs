use proc_macro2::Span;
use syn::{parse_quote, Ident, Item, ItemType, Variant};
use xsd_simple_type_compiler::{
    CompiledSimpleType, LocalSimpleTypeCompiler, SimpleTypeCompiler, WithMetadata,
};

use crate::{Context, Result, TypeGenerator, TypesOutput};

pub struct CompiledSimpleTypeGenerator;
impl TypeGenerator for CompiledSimpleTypeGenerator {
    type Input = CompiledSimpleType;

    fn generate_type(&self, input: &Self::Input, context: &Context) -> Result<TypesOutput> {
        let derive_traits = context.derive_traits.as_slice();

        match input {
            CompiledSimpleType::Literal {
                literal:
                    WithMetadata {
                        value,
                        recommended_name,
                        ..
                    },
                ..
            } => {
                let ident = Ident::new(recommended_name, Span::call_site());

                let output = TypesOutput::new(
                    parse_quote!(
                        #[derive(#(#derive_traits,)* Serialize, Deserialize)]
                        pub enum #ident {
                            #[xvalue(value = #value)]
                            #ident,
                        }
                    ),
                    ident,
                );

                Ok(output)
            }
            CompiledSimpleType::Type(qname_ref) => {
                todo!("Need to resolve dynamic locations of types")
            }
            // context.resolve_type(&qname_ref.value),
            CompiledSimpleType::Options(WithMetadata {
                value,
                recommended_name,
                ..
            }) => {
                let ident = Ident::new(recommended_name, Span::call_site());

                let mut variants = Vec::<Variant>::new();

                let mod_name = TypesOutput::to_mod_name(&ident);

                let mut related = vec![];
                for (i, a) in value.iter().enumerate() {
                    if let CompiledSimpleType::Literal {
                        literal:
                            WithMetadata {
                                value,
                                recommended_name,
                                ..
                            },
                        ..
                    } = a
                    {
                        let variant_ident = Ident::new(recommended_name, Span::call_site());

                        variants.push(parse_quote! {
                            #[xvalue(value = #value)]
                            #variant_ident(#mod_name::#variant_ident)
                        });
                        continue;
                    }
                    let option = CompiledSimpleTypeGenerator
                        .generate_type(a, &context.sub_context(i.to_string()))?;

                    related.extend(option.related_mod().map(Item::Mod));
                    related.extend(option.item);

                    let variant_ident = option.target_ident;

                    variants.push(parse_quote!(#variant_ident(#mod_name::#variant_ident)));
                }

                let mut output = TypesOutput::new(
                    parse_quote!(
                        #[derive(#(#derive_traits,)* Serialize, Deserialize)]
                        pub enum #ident {
                            #(#variants),*
                        }
                    ),
                    ident,
                );

                output.related = related;

                Ok(output)
            }
            CompiledSimpleType::List(WithMetadata {
                value,
                recommended_name,
                ..
            }) => {
                let ident = Ident::new(recommended_name, Span::call_site());

                let types_output = CompiledSimpleTypeGenerator.generate_type(value, context)?;

                let mod_name = TypesOutput::to_mod_name(&ident);

                let target_ident = &types_output.target_ident;

                let type_: ItemType =
                    parse_quote!(pub type #ident = xsd_root::List::<#mod_name::#target_ident>;);

                let mut output = TypesOutput::new(type_.into(), ident);

                output.related = Vec::from_iter(types_output.related_mod().map(Item::Mod));

                Ok(output)
            }
        }
    }
}

pub struct LocalSimpleTypeGenerator;
impl TypeGenerator for LocalSimpleTypeGenerator {
    type Input = xsd::schema::LocalSimpleType;

    fn generate_type(&self, input: &Self::Input, context: &Context) -> Result<TypesOutput> {
        let compiled = LocalSimpleTypeCompiler
            .compile_simple_type(input, &xsd_simple_type_compiler::Context)
            .unwrap();

        CompiledSimpleTypeGenerator.generate_type(&compiled, context)
    }
}

// pub struct LocalRestrictionGenerator;
// impl TypeGenerator for LocalRestrictionGenerator {
//     type Input = xsd::schema::LocalRestriction;

//     fn generate_type(&self, input: &Self::Input, context: &Context) -> Result<TypesOutput> {
//         let derive_traits = context.derive_traits.as_slice();

//         let name = context
//             .name_context
//             .as_ref()
//             .expect("must have a name from the parent");
//         let rust_name = rustify_name(name);

//         let mut variants: Vec<Variant> = vec![];

//         if let Some(_simple_type) = input.simple_type.as_ref() {
//             //TODO: handle simple type
//         }

//         for facet in input.facets.iter() {
//             match facet {
//                 xsd::schema::Facet::Enumeration(enumeration) => {
//                     let value = enumeration.value.as_str();
//                     let rust_name = rustify_name(value);
//                     let target_ident = Ident::new(rust_name.as_str(), Span::call_site());
//                     variants.push(parse_quote!(
//                         #[xvalue(value = #value)]
//                         #target_ident
//                     ));
//                 }
//                 xsd::schema::Facet::MinExclusive(_) => {}
//                 _ => {}
//             }
//         }

//         let ident = Ident::new(&rust_name, Span::call_site());
//         let simple_type = parse_quote!(
//             #[derive(#(#derive_traits,)* Serialize, Deserialize)]
//             pub enum #ident {
//                 #(#variants),*
//             }
//         );

//         let output = TypesOutput::new(Item::Enum(simple_type), ident);

//         Ok(output)
//     }
// }

// pub struct ListGenerator;
// impl TypeGenerator for ListGenerator {
//     type Input = xsd::schema::List;

//     fn generate_type(&self, input: &Self::Input, context: &Context) -> Result<TypesOutput> {
//         let derive_traits = context.derive_traits.as_slice();

//         let doc_attribute = input
//             .annotation
//             .as_ref()
//             .and_then(annotation_to_doc_attribute);

//         let mut items = Vec::new();

//         let (ident, rust_type) = match (input.item_type.as_ref(), input.simple_type.as_ref()) {
//             (Some(item_type), None) => {
//                 let resolved_type = context.resolve_type(item_type);

//                 let list_name = format!("{}List", resolved_type.ident);
//                 let ident = Ident::new(&list_name, Span::call_site());

//                 (ident, resolved_type.type_)
//             }
//             (None, Some(simple_type)) => {
//                 let name = context
//                     .name_context
//                     .as_ref()
//                     .expect("must have a name from the parent");
//                 let rust_name = rustify_name(name);
//                 let ident = Ident::new(&rust_name, Span::call_site());

//                 let simple_type = LocalSimpleTypeGenerator.generate_type(
//                     simple_type,
//                     &context.sub_context(format!("{rust_name}Content")),
//                 )?;

//                 items.extend(simple_type.related_mod().map(Item::Mod));
//                 items.extend([simple_type.target]);

//                 let mod_name = TypesOutput::to_mod_name(&ident);

//                 let sub_type_ident = simple_type.target_ident;
//                 (ident, Type::Path(parse_quote!(#mod_name::#sub_type_ident)))
//             }
//             _ => todo!(),
//         };

//         let simple_type = parse_quote!(
//             #doc_attribute
//             #[derive(#(#derive_traits,)* Serialize, Deserialize)]
//             pub struct #ident(std::vec::Vec<#rust_type>);
//         );

//         let mut output = TypesOutput::new(Item::Struct(simple_type), ident);

//         output.related = items;

//         Ok(output)
//     }
// }

// pub struct UnionGenerator;
// impl TypeGenerator for UnionGenerator {
//     type Input = xsd::schema::Union;

//     fn generate_type(&self, input: &Self::Input, context: &Context) -> Result<TypesOutput> {
//         let derive_traits = context.derive_traits.as_slice();

//         let name = context
//             .name_context
//             .as_ref()
//             .expect("must have a name from the parent");
//         let rust_name = rustify_name(name);

//         let mod_ident = Ident::new(&format!("{rust_name}_items"), Span::call_site());

//         let doc_attribute = input
//             .annotation
//             .as_ref()
//             .and_then(annotation_to_doc_attribute);

//         let mut sub_items: Vec<Item> = vec![];

//         let mut variants: Vec<Variant> = vec![];

//         if let Some(member_type) = input.member_types.as_ref() {
//             let member_types = member_type
//                 .iter()
//                 .map(|a| context.resolve_type(a))
//                 .collect::<Vec<ResolvedType>>();

//             variants.extend(
//                 member_types
//                     .into_iter()
//                     .map(|ResolvedType { type_, ident }| parse_quote!(#ident(#type_))),
//             );
//         }

//         for (i, simple_type) in input.simple_types.iter().enumerate() {
//             let type_output = LocalSimpleTypeGenerator
//                 .generate_type(simple_type, &context.sub_context(format!("SimpleType{i}")))?;

//             match &type_output.target {
//                 syn::Item::Enum(enum_) => {
//                     enum_.variants.iter().for_each(|v| variants.push(v.clone()));
//                     sub_items.extend(type_output.related.iter().cloned());
//                 }
//                 syn::Item::Struct(ItemStruct { ident, fields, .. }) => {
//                     variants.push(Variant {
//                         attrs: vec![],
//                         ident: ident.clone(),
//                         fields: fields.clone(),
//                         discriminant: None,
//                     });
//                     sub_items.extend(type_output.related.iter().cloned());
//                 }
//                 _ => {
//                     sub_items.extend(type_output.related_mod().map(Item::Mod));
//                     sub_items.extend([type_output.target]);
//                     let target_ident = type_output.target_ident;

//                     let type_: syn::Type = parse_quote!(#mod_ident::#target_ident);
//                     variants.push(parse_quote!(#target_ident(#type_)));
//                 }
//             };
//         }

//         let ident = Ident::new(&rust_name, Span::call_site());

//         let simple_type = parse_quote!(
//             #doc_attribute
//             #[derive(#(#derive_traits,)* Serialize, Deserialize)]
//             pub enum #ident {
//                 #(#variants),*
//             }
//         );

//         let mut output = TypesOutput::new(Item::Enum(simple_type), ident);

//         output.related = sub_items;

//         Ok(output)
//     }
// }

// pub struct LocalSimpleTypeGenerator;
// impl TypeGenerator for LocalSimpleTypeGenerator {
//     type Input = xsd::schema::LocalSimpleType;

//     fn generate_type(&self, input: &Self::Input, context: &Context) -> Result<TypesOutput> {
//         let _doc_attribute = input
//             .annotation
//             .as_ref()
//             .and_then(annotation_to_doc_attribute);

//         //TODO: Add support for final attribute and add docs to generated types
//         SimpleDerivationTypeGenerator.generate_type(&input.content, context)
//     }
// }

// pub struct SimpleDerivationTypeGenerator;
// impl TypeGenerator for SimpleDerivationTypeGenerator {
//     type Input = xsd::schema::SimpleDerivation;

//     fn generate_type(&self, input: &Self::Input, context: &Context) -> Result<TypesOutput> {
//         match input {
//             xsd::schema::SimpleDerivation::Restriction(restriction) => {
//                 LocalRestrictionGenerator.generate_type(restriction, context)
//             }
//             xsd::schema::SimpleDerivation::List(list) => {
//                 ListGenerator.generate_type(list.as_ref(), context)
//             }
//             xsd::schema::SimpleDerivation::Union(union) => {
//                 UnionGenerator.generate_type(union, context)
//             }
//         }
//     }
// }

#[cfg(test)]
mod tests {
    use syn::{parse_quote, File, Item};
    use xsd::schema::Namespace;
    use xsd_simple_type_compiler::ListCompiler;
    use xsd_xmlity::schema;

    use crate::top_level::TopLevelSimpleTypeGenerator;

    use super::*;

    #[test]
    fn local_simple_type() {
        let xml = r###"
        <xs:simpleType xmlns:xs="http://www.w3.org/2001/XMLSchema">
            <xs:union memberTypes="xs:QName">
                <xs:simpleType>
                    <xs:restriction base="xs:token">
                        <xs:enumeration value="##defined"/>
                        <xs:enumeration value="##definedSibling"/>
                    </xs:restriction>
                </xs:simpleType>
            </xs:union>
        </xs:simpleType>
        "###;

        let result: schema::LocalSimpleType = xmlity_quick_xml::from_str(xml).unwrap();
        let result = result.into();
        let context = crate::Context {
            namespace: Some(Namespace("http://www.w3.org/2001/XMLSchema".to_owned())),
            namespace_expr: crate::NamespaceExpr::Const("http://www.w3.org/2001/XMLSchema"),
            derive_traits: vec![
                parse_quote!(Debug),
                parse_quote!(Clone),
                parse_quote!(PartialEq),
            ],
            name_context: Some("Content".to_owned()),
            schema: None,
            can_change_name: false,
        };

        let result = LocalSimpleTypeGenerator
            .generate_type(&result, &context)
            .unwrap();

        let mut items = Vec::new();

        items.extend(result.related_mod().map(Item::Mod));
        items.extend(result.item);

        let file = File {
            shebang: None,
            attrs: vec![],
            items,
        };

        let output = prettyplease::unparse(&file);

        insta::assert_snapshot!(output, @r###"
        pub mod Content_items {
            #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
            pub enum SimpleType_0 {
                #[xvalue(value = "##defined")]
                Defined,
                #[xvalue(value = "##definedSibling")]
                DefinedSibling,
            }
        }
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
        pub enum Content {
            QName(xsd_root::types::QName),
            SimpleType_0(Content_items::SimpleType_0),
        }
        "###);
    }

    #[test]
    fn local_simple_type_block_set() {
        let xml = r###"
        <xs:simpleType xmlns:xs="http://www.w3.org/2001/XMLSchema" name="blockSet">
            <xs:annotation>
                <xs:documentation>
                A utility type, not for public use</xs:documentation>
                <xs:documentation>
                #all or (possibly empty) subset of {substitution, extension, restriction}
                </xs:documentation>
            </xs:annotation>
            <xs:union>
                <xs:simpleType>
                    <xs:restriction base="xs:token">
                        <xs:enumeration value="#all"/>
                    </xs:restriction>
                </xs:simpleType>
                <xs:simpleType>
                    <xs:list>
                        <xs:simpleType>
                            <xs:restriction base="xs:derivationControl">
                                <xs:enumeration value="extension"/>
                                <xs:enumeration value="restriction"/>
                                <xs:enumeration value="substitution"/>
                            </xs:restriction>
                        </xs:simpleType>
                    </xs:list>
                </xs:simpleType>
            </xs:union>
        </xs:simpleType>
        "###;

        let result: schema::TopLevelSimpleType = xmlity_quick_xml::from_str(xml).unwrap();
        let result = result.into();
        let context = crate::Context {
            namespace: Some(Namespace("http://www.w3.org/2001/XMLSchema".to_owned())),
            namespace_expr: crate::NamespaceExpr::Const("http://www.w3.org/2001/XMLSchema"),
            derive_traits: vec![
                parse_quote!(Debug),
                parse_quote!(Clone),
                parse_quote!(PartialEq),
            ],
            name_context: Some("Content".to_owned()),
            schema: None,
            can_change_name: false,
        };

        let result = TopLevelSimpleTypeGenerator
            .generate_type(&result, &context)
            .unwrap();

        let mut items = Vec::new();

        items.extend(result.related_mod().map(Item::Mod));
        items.extend(result.item);

        let file = File {
            shebang: None,
            attrs: vec![],
            items,
        };

        let output = prettyplease::unparse(&file);

        insta::assert_snapshot!(output, @r###"
        pub mod block_set {
            use super::xsd_root;
            use xmlity::{DeserializationGroup, Deserialize, SerializationGroup, Serialize};
            #[derive(Debug, PartialEq, Serialize, Deserialize)]
            pub enum SimpleType1Content {
                #[xvalue(value = "extension")]
                Extension,
                #[xvalue(value = "restriction")]
                Restriction,
                #[xvalue(value = "substitution")]
                Substitution,
            }
        }
        #[derive(Debug, PartialEq, Serialize, Deserialize)]
        pub enum BlockSet {
            #[xvalue(value = "#all")]
            All,
            SimpleType1(std::vec::Vec<block_set::SimpleType1Content>),
        }
        "###);
    }

    #[test]
    fn list_type() {
        let xml = r###"
        <xs:list xmlns:xs="http://www.w3.org/2001/XMLSchema">
            <xs:simpleType>
                <xs:union memberTypes="xs:QName">
                    <xs:simpleType>
                        <xs:restriction base="xs:token">
                            <xs:enumeration value="##defined"/>
                            <xs:enumeration value="##definedSibling"/>
                        </xs:restriction>
                    </xs:simpleType>
                </xs:union>
            </xs:simpleType>
        </xs:list>
        "###;

        let result: schema::List = xmlity_quick_xml::from_str(xml).unwrap();
        let result = result.into();
        println!("{:#?}", result);
        let context = crate::Context {
            namespace: Some(Namespace("http://www.w3.org/2001/XMLSchema".to_owned())),
            namespace_expr: crate::NamespaceExpr::Const("http://www.w3.org/2001/XMLSchema"),
            derive_traits: vec![
                parse_quote!(Debug),
                parse_quote!(Clone),
                parse_quote!(PartialEq),
            ],
            name_context: Some("TestList".to_owned()),
            schema: None,
            can_change_name: false,
        };

        let result = ListCompiler
            .compile_simple_type(&result, &xsd_simple_type_compiler::Context)
            .unwrap();
        let result = CompiledSimpleTypeGenerator
            .generate_type(&result, &context)
            .unwrap();

        let mut items = Vec::new();

        items.extend(result.related_mod().map(Item::Mod));
        items.extend(result.item);

        let file = File {
            shebang: None,
            attrs: vec![],
            items,
        };

        let output = prettyplease::unparse(&file);

        insta::assert_snapshot!(output, @r###"
        pub mod test_list {
            #[derive(Debug, Clone, PartialEq)]
            pub struct TestList(pub Vec<TestListItem>);
            #[derive(Debug, Clone, PartialEq)]
            pub enum TestListItem {
                Defined,
                DefinedSibling,
                Token(String),
            }
        }

        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
        pub struct TestList(std::vec::Vec<test_list::TestListItem>);
        "###);
    }
}
