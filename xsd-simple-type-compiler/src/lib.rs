#[derive(Debug, Clone, PartialEq)]
pub struct WithMetadata<T> {
    pub value: T,
    pub recommended_name: String,
    pub documentation: Option<String>,
}

impl<T> WithMetadata<T> {
    pub fn new<U: Into<String>>(value: T, recommmended_name: U) -> Self {
        Self {
            value,
            recommended_name: recommmended_name.into(),
            documentation: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum CompiledSimpleType {
    Literal {
        literal: WithMetadata<String>,
        base_type: Box<CompiledSimpleType>,
    },
    Type(WithMetadata<QNameRef>),
    Options(WithMetadata<Vec<CompiledSimpleType>>),
    List(WithMetadata<Box<CompiledSimpleType>>),
}

#[derive(Debug, thiserror::Error)]
pub enum Error {}

pub type Result<T> = std::result::Result<T, Error>;

pub struct Context;

pub trait SimpleTypeCompiler {
    type Input;

    fn compile_simple_type(
        &self,
        input: &Self::Input,
        context: &Context,
    ) -> Result<CompiledSimpleType>;
}

use xsd::schema::QNameRef;

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

pub struct LocalRestrictionCompiler;
impl SimpleTypeCompiler for LocalRestrictionCompiler {
    type Input = xsd::schema::LocalRestriction;

    fn compile_simple_type(
        &self,
        input: &Self::Input,
        _context: &Context,
    ) -> Result<CompiledSimpleType> {
        let mut variants: Vec<CompiledSimpleType> = vec![];

        if let Some(_simple_type) = input.simple_type.as_ref() {
            //TODO: handle simple type
        }

        for facet in input.facets.iter().rev() {
            match facet {
                xsd::schema::Facet::Enumeration(enumeration) => {
                    let value = enumeration.value.as_str();

                    variants.push(CompiledSimpleType::Literal {
                        literal: WithMetadata::new(value.to_owned(), rustify_name(value)),
                        base_type: Box::new(CompiledSimpleType::Type(WithMetadata::new(
                            input.base.clone(),
                            rustify_name(&input.base.name),
                        ))),
                    });
                }
                xsd::schema::Facet::MinExclusive(_) => {}
                _ => {}
            }
        }

        if variants.len() == 1 {
            Ok(variants.pop().expect("Checked"))
        } else {
            Ok(CompiledSimpleType::Options(WithMetadata::new(
                variants, "Union",
            )))
        }
    }
}

pub struct ListCompiler;
impl SimpleTypeCompiler for ListCompiler {
    type Input = xsd::schema::List;

    fn compile_simple_type(
        &self,
        input: &Self::Input,
        context: &Context,
    ) -> Result<CompiledSimpleType> {
        let (name, type_) = match (input.item_type.as_ref(), input.simple_type.as_ref()) {
            (Some(item_type), None) => (
                item_type.name.clone(),
                CompiledSimpleType::Type(WithMetadata::new(
                    item_type.clone(),
                    format!("{}List", rustify_name(&item_type.name)),
                )),
            ),

            (None, Some(simple_type)) => (
                simple_type
                    .id
                    .as_ref()
                    .map(|a| a.0.clone())
                    .unwrap_or("SimpleType".to_owned()),
                LocalSimpleTypeCompiler.compile_simple_type(simple_type, context)?,
            ),
            _ => todo!(),
        };

        Ok(CompiledSimpleType::List(WithMetadata::new(
            Box::new(type_),
            format!("{name}List"),
        )))
    }
}

pub struct UnionCompiler;
impl SimpleTypeCompiler for UnionCompiler {
    type Input = xsd::schema::Union;

    fn compile_simple_type(
        &self,
        input: &Self::Input,
        context: &Context,
    ) -> Result<CompiledSimpleType> {
        let mut variants: Vec<CompiledSimpleType> = vec![];

        if let Some(member_type) = input.member_types.as_ref() {
            variants.extend(member_type.iter().cloned().map(|value| {
                let name = value.name.clone();
                CompiledSimpleType::Type(WithMetadata::new(value, name))
            }));
        }

        variants.extend(
            input
                .simple_types
                .iter()
                .map(|simple_type| {
                    LocalSimpleTypeCompiler.compile_simple_type(simple_type, context)
                })
                .collect::<Result<Vec<_>>>()?
                .into_iter()
                .flat_map(|a| match a {
                    CompiledSimpleType::Options(compiled_simple_types) => {
                        compiled_simple_types.value
                    }
                    a => vec![a],
                }),
        );

        Ok(CompiledSimpleType::Options(WithMetadata::new(
            variants, "Union",
        )))
    }
}

pub struct LocalSimpleTypeCompiler;
impl SimpleTypeCompiler for LocalSimpleTypeCompiler {
    type Input = xsd::schema::LocalSimpleType;

    fn compile_simple_type(
        &self,
        input: &Self::Input,
        context: &Context,
    ) -> Result<CompiledSimpleType> {
        //TODO: Add support for final attribute and add docs to generated types
        SimpleDerivationTypeCompiler.compile_simple_type(&input.content, context)
    }
}

pub struct TopLevelSimpleTypeCompiler;
impl SimpleTypeCompiler for TopLevelSimpleTypeCompiler {
    type Input = xsd::schema::TopLevelSimpleType;

    fn compile_simple_type(
        &self,
        input: &Self::Input,
        context: &Context,
    ) -> Result<CompiledSimpleType> {
        //TODO: Add support for final attribute and add docs to generated types
        SimpleDerivationTypeCompiler.compile_simple_type(&input.content, context)
    }
}

pub struct SimpleDerivationTypeCompiler;
impl SimpleTypeCompiler for SimpleDerivationTypeCompiler {
    type Input = xsd::schema::SimpleDerivation;

    fn compile_simple_type(
        &self,
        input: &Self::Input,
        context: &Context,
    ) -> Result<CompiledSimpleType> {
        match input {
            xsd::schema::SimpleDerivation::Restriction(restriction) => {
                LocalRestrictionCompiler.compile_simple_type(restriction, context)
            }
            xsd::schema::SimpleDerivation::List(list) => {
                ListCompiler.compile_simple_type(list.as_ref(), context)
            }
            xsd::schema::SimpleDerivation::Union(union) => {
                UnionCompiler.compile_simple_type(union, context)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ListCompiler;
    use xsd::schema::Namespace;
    use xsd_xmlity::schema;

    use super::*;
    use pretty_assertions::assert_eq;

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
        let context = crate::Context {};

        let actual = LocalSimpleTypeCompiler
            .compile_simple_type(&result, &context)
            .unwrap();

        let expected = CompiledSimpleType::Options(WithMetadata::new(
            vec![
                CompiledSimpleType::Type(WithMetadata::new(
                    QNameRef {
                        namespace: Some(Namespace("http://www.w3.org/2001/XMLSchema".to_owned())),
                        name: "QName".to_owned(),
                    },
                    "QName",
                )),
                CompiledSimpleType::Literal {
                    literal: WithMetadata::new("##definedSibling".to_owned(), "DefinedSibling"),
                    base_type: Box::new(CompiledSimpleType::Type(WithMetadata::new(
                        QNameRef {
                            namespace: Some(Namespace(
                                "http://www.w3.org/2001/XMLSchema".to_owned(),
                            )),
                            name: "token".to_owned(),
                        },
                        "Token",
                    ))),
                },
                CompiledSimpleType::Literal {
                    literal: WithMetadata::new("##defined".to_owned(), "Defined"),
                    base_type: Box::new(CompiledSimpleType::Type(WithMetadata::new(
                        QNameRef {
                            namespace: Some(Namespace(
                                "http://www.w3.org/2001/XMLSchema".to_owned(),
                            )),
                            name: "token".to_owned(),
                        },
                        "Token",
                    ))),
                },
            ],
            "Union",
        ));

        assert_eq!(actual, expected);
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

        let context = crate::Context {};

        let actual = TopLevelSimpleTypeCompiler
            .compile_simple_type(&result, &context)
            .unwrap();

        let expected = CompiledSimpleType::Options(WithMetadata::new(
            vec![
                CompiledSimpleType::Literal {
                    literal: WithMetadata::new("#all".to_owned(), "All"),
                    base_type: Box::new(CompiledSimpleType::Type(WithMetadata::new(
                        QNameRef {
                            namespace: Some(Namespace(
                                "http://www.w3.org/2001/XMLSchema".to_owned(),
                            )),
                            name: "token".to_owned(),
                        },
                        "Token",
                    ))),
                },
                CompiledSimpleType::List(WithMetadata::new(
                    Box::new(CompiledSimpleType::Options(WithMetadata::new(
                        vec![
                            CompiledSimpleType::Literal {
                                literal: WithMetadata::new(
                                    "substitution".to_owned(),
                                    "Substitution",
                                ),
                                base_type: Box::new(CompiledSimpleType::Type(WithMetadata::new(
                                    QNameRef {
                                        namespace: Some(Namespace(
                                            "http://www.w3.org/2001/XMLSchema".to_owned(),
                                        )),
                                        name: "derivationControl".to_owned(),
                                    },
                                    "DerivationControl",
                                ))),
                            },
                            CompiledSimpleType::Literal {
                                literal: WithMetadata::new("restriction".to_owned(), "Restriction"),
                                base_type: Box::new(CompiledSimpleType::Type(WithMetadata::new(
                                    QNameRef {
                                        namespace: Some(Namespace(
                                            "http://www.w3.org/2001/XMLSchema".to_owned(),
                                        )),
                                        name: "derivationControl".to_owned(),
                                    },
                                    "DerivationControl",
                                ))),
                            },
                            CompiledSimpleType::Literal {
                                literal: WithMetadata::new("extension".to_owned(), "Extension"),
                                base_type: Box::new(CompiledSimpleType::Type(WithMetadata::new(
                                    QNameRef {
                                        namespace: Some(Namespace(
                                            "http://www.w3.org/2001/XMLSchema".to_owned(),
                                        )),
                                        name: "derivationControl".to_owned(),
                                    },
                                    "DerivationControl",
                                ))),
                            },
                        ],
                        "Union",
                    ))),
                    "SimpleTypeList",
                )),
            ],
            "Union",
        ));

        assert_eq!(actual, expected);
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

        let context = crate::Context {};

        let actual = ListCompiler.compile_simple_type(&result, &context).unwrap();

        let expected = CompiledSimpleType::List(WithMetadata::new(
            Box::new(CompiledSimpleType::Options(WithMetadata::new(
                vec![
                    CompiledSimpleType::Type(WithMetadata::new(
                        QNameRef {
                            namespace: Some(Namespace(
                                "http://www.w3.org/2001/XMLSchema".to_owned(),
                            )),
                            name: "QName".to_owned(),
                        },
                        "QName",
                    )),
                    CompiledSimpleType::Literal {
                        literal: WithMetadata::new("##definedSibling".to_owned(), "DefinedSibling"),
                        base_type: Box::new(CompiledSimpleType::Type(WithMetadata::new(
                            QNameRef {
                                namespace: Some(Namespace(
                                    "http://www.w3.org/2001/XMLSchema".to_owned(),
                                )),
                                name: "token".to_owned(),
                            },
                            "Token",
                        ))),
                    },
                    CompiledSimpleType::Literal {
                        literal: WithMetadata::new("##defined".to_owned(), "Defined"),
                        base_type: Box::new(CompiledSimpleType::Type(WithMetadata::new(
                            QNameRef {
                                namespace: Some(Namespace(
                                    "http://www.w3.org/2001/XMLSchema".to_owned(),
                                )),
                                name: "token".to_owned(),
                            },
                            "Token",
                        ))),
                    },
                ],
                "Union",
            ))),
            "SimpleTypeList",
        ));

        assert_eq!(actual, expected);
    }
}
