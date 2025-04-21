pub enum CompiledSimpleType {
    Literal(String),
    Type(QNameRef),
    Options(Vec<CompiledSimpleType>),
    List(Box<CompiledSimpleType>),
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

        for facet in input.facets.iter() {
            match facet {
                xsd::schema::Facet::Enumeration(enumeration) => {
                    let value = enumeration.value.as_str();

                    variants.push(CompiledSimpleType::Literal(value.to_owned()));
                }
                xsd::schema::Facet::MinExclusive(_) => {}
                _ => {}
            }
        }

        Ok(CompiledSimpleType::Options(variants))
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
        let type_ = match (input.item_type.as_ref(), input.simple_type.as_ref()) {
            (Some(item_type), None) => CompiledSimpleType::Type(item_type.clone()),

            (None, Some(simple_type)) => {
                LocalSimpleTypeCompiler.compile_simple_type(simple_type, context)?
            }
            _ => todo!(),
        };

        Ok(CompiledSimpleType::List(Box::new(type_)))
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
            variants.extend(member_type.iter().cloned().map(CompiledSimpleType::Type));
        }

        variants.extend(
            input
                .simple_types
                .iter()
                .map(|simple_type| {
                    LocalSimpleTypeCompiler.compile_simple_type(simple_type, context)
                })
                .collect::<Result<Vec<_>>>()?,
        );

        Ok(CompiledSimpleType::Options(variants))
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
    use super::*;

    #[test]
    fn it_works() {}
}
