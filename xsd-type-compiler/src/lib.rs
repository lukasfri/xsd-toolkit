use std::collections::HashMap;

use complex::ToComplexFragments;
use simple::ToSimpleFragments;
use xmlity::{ExpandedName, LocalName, XmlNamespace};
use xsd::schema;
pub mod transformers;

pub mod complex;
mod misc;
pub mod simple;

pub struct XmlnsContext {
    pub namespaces: HashMap<XmlNamespace<'static>, CompiledNamespace>,
}

impl XmlnsContext {
    pub fn new() -> Self {
        Self {
            namespaces: HashMap::new(),
        }
    }

    pub fn add_namespace(&mut self, namespace: CompiledNamespace) {
        self.namespaces
            .insert(namespace.namespace.clone(), namespace);
    }

    pub fn transform(
        &mut self,
        namespace: &XmlNamespace<'static>,
        transformer: impl transformers::XmlnsContextTransformer,
    ) {
        transformer.transform(transformers::Context {
            namespace,
            xmlns_context: self,
        });
    }
}

impl Default for XmlnsContext {
    fn default() -> Self {
        Self::new()
    }
}

pub struct CompiledNamespace {
    pub namespace: XmlNamespace<'static>,
    pub complex_type: complex::ComplexTypeFragmentCompiler,
    pub top_level_types: HashMap<LocalName<'static>, TopLevelType>,
    pub top_level_elements: HashMap<LocalName<'static>, TopLevelElement>,
    pub top_level_attributes: HashMap<LocalName<'static>, TopLevelAttribute>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ComplexTypeIdent {
    Named(ExpandedName<'static>),
    Anonymous(complex::FragmentId),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SimpleTypeIdent {
    Named(ExpandedName<'static>),
    Anonymous(simple::FragmentId),
}

impl From<ExpandedName<'static>> for ComplexTypeIdent {
    fn from(value: ExpandedName<'static>) -> Self {
        Self::Named(value)
    }
}

impl CompiledNamespace {
    pub fn new(namespace: XmlNamespace<'static>) -> Self {
        let simple_type_compiler = simple::SimpleTypeFragmentCompiler::new(namespace.clone());
        let complex_type_compiler =
            complex::ComplexTypeFragmentCompiler::new(namespace.clone(), simple_type_compiler);

        Self {
            namespace,
            complex_type: complex_type_compiler,
            top_level_types: HashMap::new(),
            top_level_elements: HashMap::new(),
            top_level_attributes: HashMap::new(),
        }
    }

    pub fn from_schema(schema: &xsd::XmlSchema) -> Self {
        let mut this = Self::new(schema.underlying_schema.target_namespace.0.clone());

        for element in schema.top_level_elements() {
            this.add_top_level_element(element);
        }

        this
    }

    pub fn add_top_level_simple_type(
        &mut self,
        type_: &xsd::schema::TopLevelSimpleType,
    ) -> ExpandedName<'_> {
        let name = type_.name.clone();
        let root_fragment = type_.content.to_simple_fragments(&mut self.complex_type);
        let type_ = TopLevelType::Simple(TopLevelSimpleType { root_fragment });
        self.top_level_types.insert(name.clone(), type_);

        ExpandedName::new(name, Some(self.namespace.as_ref()))
    }

    pub fn add_top_level_complex_type(
        &mut self,
        type_: &xsd::schema::TopLevelComplexType,
    ) -> ExpandedName<'_> {
        let name = type_.name.clone();
        let root_fragment = type_.content.to_complex_fragments(&mut self.complex_type);
        let type_ = TopLevelType::Complex(TopLevelComplexType { root_fragment });
        self.top_level_types.insert(name.clone(), type_);

        ExpandedName::new(name, Some(self.namespace.as_ref()))
    }

    pub fn add_top_level_element(
        &mut self,
        element: &xsd::schema::TopLevelElement,
    ) -> ExpandedName<'_> {
        let name = element.name.0.clone();

        let content_type = if let Some(type_) = element.type_.as_ref() {
            ComplexTypeIdent::Named(type_.0 .0.clone())
        } else {
            let type_choice = element.type_choice.as_ref().unwrap();
            match type_choice {
                schema::ElementTypeContent::SimpleType(_) => todo!(),
                schema::ElementTypeContent::ComplexType(local_complex_type) => {
                    ComplexTypeIdent::Anonymous(
                        local_complex_type
                            .content
                            .to_complex_fragments(&mut self.complex_type),
                    )
                }
            }
        };

        self.top_level_elements
            .insert(name.clone(), TopLevelElement { content_type });

        ExpandedName::new(name, Some(self.namespace.as_ref()))
    }

    pub fn add_top_level_attribute(
        &mut self,
        attribute: &xsd::schema::TopLevelAttribute,
    ) -> ExpandedName<'_> {
        let name = attribute.name.0.clone();

        let content_type = if let Some(type_) = attribute.type_.as_ref() {
            SimpleTypeIdent::Named(type_.0 .0.clone())
        } else {
            let simple_type = attribute.simple_type.as_ref().unwrap();
            let type_fragment = simple_type.to_simple_fragments(&mut self.complex_type);

            SimpleTypeIdent::Anonymous(type_fragment)
        };

        self.top_level_attributes
            .insert(name.clone(), TopLevelAttribute { content_type });

        ExpandedName::new(name, Some(self.namespace.as_ref()))
    }
}

#[derive(Debug)]
pub struct TopLevelSimpleType {
    pub root_fragment: simple::FragmentId,
}

#[derive(Debug)]
pub struct TopLevelComplexType {
    pub root_fragment: complex::FragmentId,
}

#[derive(Debug)]
pub enum TopLevelType {
    Simple(TopLevelSimpleType),
    Complex(TopLevelComplexType),
}

#[derive(Debug)]
pub struct TopLevelElement {
    pub content_type: ComplexTypeIdent,
}

#[derive(Debug)]
pub struct TopLevelAttribute {
    pub content_type: SimpleTypeIdent,
}
