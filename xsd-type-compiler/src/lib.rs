use std::collections::HashMap;

use complex::ComplexFragmentEquivalent;
use simple::ToSimpleFragments;
use transformers::TransformChange;
use xmlity::{ExpandedName, LocalName, XmlNamespace};
pub mod transformers;

pub mod complex;
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

    pub fn transform<T: transformers::XmlnsContextTransformer>(
        &mut self,
        namespace: &XmlNamespace<'static>,
        transformer: T,
    ) -> Result<TransformChange, T::Error> {
        transformer.transform(transformers::Context {
            namespace,
            xmlns_context: self,
        })
    }
}

impl Default for XmlnsContext {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
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
    Anonymous(complex::FragmentIdx<complex::ComplexTypeRootFragment>),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum NamedOrAnonymous<T> {
    Named(ExpandedName<'static>),
    Anonymous(T),
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

    pub fn from_schema(schema: &xsd::XmlSchema) -> Result<Self, ()> {
        let mut this = Self::new(schema.underlying_schema.target_namespace.0.clone());

        for simple_type in schema.top_level_simple_types() {
            this.import_top_level_simple_type(simple_type)?;
        }

        for complex_type in schema.top_level_complex_types() {
            this.import_top_level_complex_type(complex_type)?;
        }

        for attribute in schema.top_level_attributes() {
            this.import_top_level_attribute(attribute)?;
        }

        for element in schema.top_level_elements() {
            this.import_top_level_element(element)?;
        }

        Ok(this)
    }

    pub fn import_top_level_simple_type(
        &mut self,
        type_: &xsd::schema::TopLevelSimpleType,
    ) -> Result<ExpandedName<'_>, ()> {
        let name = type_.name.clone();
        if self.top_level_types.contains_key(&name) {
            return Err(());
        }

        let root_fragment = type_.content.to_simple_fragments(&mut self.complex_type);
        let type_ = TopLevelType::Simple(TopLevelSimpleType { root_fragment });
        self.top_level_types.insert(name.clone(), type_);

        let name = ExpandedName::new(name, Some(self.namespace.as_ref()));

        Ok(name)
    }

    pub fn import_top_level_complex_type(
        &mut self,
        type_: &xsd::schema::TopLevelComplexType,
    ) -> Result<ExpandedName<'_>, ()> {
        let name = type_.name.clone();
        if self.top_level_types.contains_key(&name) {
            return Err(());
        }

        let root_fragment = type_.to_complex_fragments(&mut self.complex_type);

        let type_ = TopLevelType::Complex(TopLevelComplexType { root_fragment });
        self.top_level_types.insert(name.clone(), type_);

        let name = ExpandedName::new(name, Some(self.namespace.as_ref()));

        Ok(name)
    }

    pub fn export_top_level_complex_type(
        &self,
        type_: &LocalName<'_>,
    ) -> Result<Option<xsd::schema::TopLevelComplexType>, ()> {
        let Some(TopLevelType::Complex(top_level_complex_type)) = self.top_level_types.get(type_)
        else {
            return Ok(None);
        };

        let fragment_id = &top_level_complex_type.root_fragment;

        let top_level = xsd::schema::TopLevelComplexType::from_complex_fragments(
            &self.complex_type,
            fragment_id,
        )
        .unwrap();

        Ok(Some(top_level))
    }

    pub fn import_top_level_element(
        &mut self,
        element: &xsd::schema::TopLevelElement,
    ) -> Result<ExpandedName<'_>, ()> {
        // let name = element.name.0.clone();
        // if self.top_level_elements.contains_key(&name) {
        //     return Err(());
        // }

        // let content_type = if let Some(type_) = element.type_.as_ref() {
        //     ComplexTypeIdent::Named(type_.0 .0.clone())
        // } else {
        //     let type_choice = element.type_choice.as_ref().unwrap();
        //     match type_choice {
        //         schema::ElementTypeContent::SimpleType(_) => todo!(),
        //         schema::ElementTypeContent::ComplexType(local_complex_type) => {
        //             ComplexTypeIdent::Anonymous(
        //                 local_complex_type.to_complex_fragments(&mut self.complex_type),
        //             )
        //         }
        //     }
        // };

        let name = element.0.name.clone();
        if self.top_level_elements.contains_key(&name) {
            return Err(());
        }

        let root_fragment = element.to_complex_fragments(&mut self.complex_type);

        self.top_level_elements
            .insert(name.clone(), TopLevelElement { root_fragment });

        let name = ExpandedName::new(name, Some(self.namespace.as_ref()));

        Ok(name)
    }

    pub fn import_top_level_attribute(
        &mut self,
        attribute: &xsd::schema::TopLevelAttribute,
    ) -> Result<ExpandedName<'_>, ()> {
        let name = attribute.name.0.clone();
        if self.top_level_attributes.contains_key(&name) {
            return Err(());
        }

        let root_fragment = attribute.to_complex_fragments(&mut self.complex_type);

        self.top_level_attributes
            .insert(name.clone(), TopLevelAttribute { root_fragment });

        let name = ExpandedName::new(name, Some(self.namespace.as_ref()));

        Ok(name)
    }
}

#[derive(Debug)]
pub struct TopLevelSimpleType {
    pub root_fragment: simple::FragmentId,
}

#[derive(Debug)]
pub struct TopLevelComplexType {
    pub root_fragment: complex::FragmentIdx<complex::ComplexTypeRootFragment>,
}

#[derive(Debug)]
pub enum TopLevelType {
    Simple(TopLevelSimpleType),
    Complex(TopLevelComplexType),
}

#[derive(Debug)]
pub struct TopLevelElement {
    pub root_fragment: complex::FragmentIdx<complex::TopLevelElementFragment>,
}

#[derive(Debug)]
pub struct TopLevelAttribute {
    pub root_fragment: complex::FragmentIdx<complex::TopLevelAttributeFragment>,
}
