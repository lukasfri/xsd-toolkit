use std::collections::BTreeMap;
use std::ops::Deref;

use transformers::TransformChange;
use xmlity::{ExpandedName, LocalName, XmlNamespace};
use xsd::xs;

use crate::fragments::{transformers, FragmentIdx};
pub mod fragments;
use crate::fragments::complex::ComplexFragmentEquivalent;
use crate::fragments::simple::SimpleFragmentEquivalent;

#[derive(Debug)]
pub enum Error {
    ImportOfExistingEntity,
}

#[derive(Debug)]
pub struct XmlnsContext {
    pub namespaces: BTreeMap<XmlNamespace<'static>, CompiledNamespace>,
}

impl XmlnsContext {
    pub fn new() -> Self {
        Self {
            namespaces: BTreeMap::new(),
        }
    }

    pub fn add_namespace(&mut self, namespace: CompiledNamespace) {
        self.namespaces
            .insert(namespace.namespace.clone(), namespace);
    }

    pub fn transform<T: transformers::XmlnsLocalTransformer>(
        &mut self,
        namespace: &XmlNamespace<'static>,
        transformer: T,
    ) -> Result<TransformChange, T::Error> {
        self.namespaces
            .get_mut(namespace)
            .unwrap()
            .transform(transformer)
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
    pub complex_type: fragments::complex::ComplexTypeFragmentCompiler,
    pub top_level_types: BTreeMap<LocalName<'static>, TopLevelType>,
    pub top_level_elements: BTreeMap<LocalName<'static>, TopLevelElement>,
    pub top_level_attributes: BTreeMap<LocalName<'static>, TopLevelAttribute>,
    pub top_level_groups: BTreeMap<LocalName<'static>, TopLevelGroup>,
    pub top_level_attribute_groups: BTreeMap<LocalName<'static>, TopLevelAttributeGroup>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum NamedOrAnonymous<T> {
    Named(ExpandedName<'static>),
    Anonymous(T),
}

impl CompiledNamespace {
    pub fn new(namespace: XmlNamespace<'static>) -> Self {
        let simple_type_compiler =
            fragments::simple::SimpleTypeFragmentCompiler::new(namespace.clone());
        let complex_type_compiler = fragments::complex::ComplexTypeFragmentCompiler::new(
            namespace.clone(),
            simple_type_compiler,
        );

        Self {
            namespace,
            complex_type: complex_type_compiler,
            top_level_types: BTreeMap::new(),
            top_level_elements: BTreeMap::new(),
            top_level_attributes: BTreeMap::new(),
            top_level_groups: BTreeMap::new(),
            top_level_attribute_groups: BTreeMap::new(),
        }
    }

    pub fn transform<T: transformers::XmlnsLocalTransformer>(
        &mut self,
        transformer: T,
    ) -> Result<TransformChange, T::Error> {
        transformer.transform(transformers::XmlnsLocalTransformerContext { namespace: self })
    }

    pub fn from_schema(schema: &xsd::XmlSchema) -> Result<Self, Error> {
        let mut this = Self::new(schema.underlying_schema.target_namespace.clone().unwrap().0);

        use xs::groups::{Redefinable, SchemaTop};

        for redefine in schema.schema_tops() {
            match redefine {
                SchemaTop::Redefinable(redefineable) => match redefineable.deref() {
                    Redefinable::SimpleType(simple_type) => {
                        this.import_top_level_simple_type(simple_type)?;
                    }
                    Redefinable::ComplexType(complex_type) => {
                        this.import_top_level_complex_type(complex_type)?;
                    }
                    Redefinable::Group(group) => {
                        this.import_top_level_group(group)?;
                    }
                    Redefinable::AttributeGroup(attribute_group) => {
                        this.import_top_level_attribute_group(attribute_group)?;
                    }
                },
                SchemaTop::Element(element) => {
                    this.import_top_level_element(element)?;
                }
                SchemaTop::Attribute(attribute) => {
                    this.import_top_level_attribute(attribute)?;
                }
                SchemaTop::Notation(_) => {}
            }
        }

        Ok(this)
    }

    pub fn to_schema(&self) -> xsd::XmlSchema {
        todo!()
    }

    pub fn import_top_level_simple_type(
        &mut self,
        simple_type: &xs::SimpleType,
    ) -> Result<ExpandedName<'_>, Error> {
        let name = simple_type.0.name.clone();

        if self.top_level_types.contains_key(&name) {
            return Err(Error::ImportOfExistingEntity);
        }

        let root_fragment = simple_type.0.to_simple_fragments(&mut self.complex_type);
        let type_ = TopLevelType::Simple(TopLevelSimpleType { root_fragment });
        self.top_level_types.insert(name.clone(), type_);

        let name = ExpandedName::new(name, Some(self.namespace.as_ref()));

        Ok(name)
    }

    pub fn import_top_level_complex_type(
        &mut self,
        complex_type: &xs::ComplexType,
    ) -> Result<ExpandedName<'_>, Error> {
        let name = complex_type.0.name.clone();

        if self.top_level_types.contains_key(&name) {
            return Err(Error::ImportOfExistingEntity);
        }

        let root_fragment = complex_type.0.to_complex_fragments(&mut self.complex_type);

        let type_ = TopLevelType::Complex(TopLevelComplexType { root_fragment });
        self.top_level_types.insert(name.clone(), type_);

        let name = ExpandedName::new(name, Some(self.namespace.as_ref()));

        Ok(name)
    }

    pub fn export_top_level_complex_type(
        &self,
        complex_type: &LocalName<'_>,
    ) -> Result<Option<xs::ComplexType>, Error> {
        let Some(TopLevelType::Complex(top_level_complex_type)) =
            self.top_level_types.get(complex_type)
        else {
            return Ok(None);
        };

        let fragment_id = &top_level_complex_type.root_fragment;

        let complex_type =
            xs::types::TopLevelComplexType::from_complex_fragments(&self.complex_type, fragment_id)
                .unwrap();

        Ok(Some(xs::ComplexType(complex_type.into())))
    }

    pub fn import_top_level_element(
        &mut self,
        element: &xs::Element,
    ) -> Result<ExpandedName<'_>, Error> {
        let name = element.0.name.clone();

        if self.top_level_elements.contains_key(&name) {
            return Err(Error::ImportOfExistingEntity);
        }

        let root_fragment = element.0.to_complex_fragments(&mut self.complex_type);

        self.top_level_elements
            .insert(name.clone(), TopLevelElement { root_fragment });

        let name = ExpandedName::new(name, Some(self.namespace.as_ref()));

        Ok(name)
    }

    pub fn export_top_level_element(
        &self,
        element: &LocalName<'_>,
    ) -> Result<Option<xs::Element>, Error> {
        let Some(top_level_element) = self.top_level_elements.get(element) else {
            return Ok(None);
        };

        let fragment_id = &top_level_element.root_fragment;

        let element =
            xs::types::TopLevelElement::from_complex_fragments(&self.complex_type, fragment_id)
                .unwrap();

        Ok(Some(xs::Element(element.into())))
    }

    pub fn import_top_level_attribute(
        &mut self,
        attribute: &xs::Attribute,
    ) -> Result<ExpandedName<'_>, Error> {
        let name = attribute.0.name.clone();

        if self.top_level_attributes.contains_key(&name) {
            return Err(Error::ImportOfExistingEntity);
        }

        let root_fragment = attribute.0.to_complex_fragments(&mut self.complex_type);

        self.top_level_attributes
            .insert(name.clone(), TopLevelAttribute { root_fragment });

        let name = ExpandedName::new(name, Some(self.namespace.as_ref()));

        Ok(name)
    }

    pub fn import_top_level_group(&mut self, group: &xs::Group) -> Result<ExpandedName<'_>, Error> {
        let name = group.0.name.clone();

        if self.top_level_groups.contains_key(&name) {
            return Err(Error::ImportOfExistingEntity);
        }

        let root_fragment = group.0.to_complex_fragments(&mut self.complex_type);
        let type_ = TopLevelGroup { root_fragment };
        self.top_level_groups.insert(name.clone(), type_);

        let name = ExpandedName::new(name, Some(self.namespace.as_ref()));

        Ok(name)
    }

    pub fn import_top_level_attribute_group(
        &mut self,
        attribute_group: &xs::AttributeGroup,
    ) -> Result<ExpandedName<'_>, Error> {
        let name = attribute_group.0.name.clone();

        if self.top_level_groups.contains_key(&name) {
            return Err(Error::ImportOfExistingEntity);
        }

        let root_fragment = attribute_group
            .0
            .to_complex_fragments(&mut self.complex_type);
        let type_ = TopLevelAttributeGroup { root_fragment };
        self.top_level_attribute_groups.insert(name.clone(), type_);

        let name = ExpandedName::new(name, Some(self.namespace.as_ref()));

        Ok(name)
    }
}

#[derive(Debug)]
pub struct TopLevelSimpleType {
    pub root_fragment: fragments::FragmentIdx<fragments::simple::SimpleTypeRootFragment>,
}

#[derive(Debug)]
pub struct TopLevelComplexType {
    pub root_fragment: FragmentIdx<fragments::complex::ComplexTypeRootFragment>,
}

#[derive(Debug)]
pub enum TopLevelType {
    Simple(TopLevelSimpleType),
    Complex(TopLevelComplexType),
}

#[derive(Debug)]
pub struct TopLevelElement {
    pub root_fragment: FragmentIdx<fragments::complex::TopLevelElementFragment>,
}

#[derive(Debug)]
pub struct TopLevelAttribute {
    pub root_fragment: FragmentIdx<fragments::complex::TopLevelAttributeFragment>,
}

#[derive(Debug)]
pub struct TopLevelGroup {
    pub root_fragment: FragmentIdx<fragments::complex::TopLevelGroupFragment>,
}

#[derive(Debug)]
pub struct TopLevelAttributeGroup {
    pub root_fragment: FragmentIdx<fragments::complex::TopLevelAttributeGroupFragment>,
}
