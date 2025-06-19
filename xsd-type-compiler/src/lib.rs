use std::collections::BTreeMap;

use complex::ComplexFragmentEquivalent;
use simple::ToSimpleFragments;
use transformers::TransformChange;
use xmlity::{ExpandedName, LocalName, XmlNamespace};
use xsd::schema as xs;
pub mod transformers;

pub mod complex;
pub mod simple;

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
    pub complex_type: complex::ComplexTypeFragmentCompiler,
    pub top_level_types: BTreeMap<LocalName<'static>, TopLevelType>,
    pub top_level_elements: BTreeMap<LocalName<'static>, TopLevelElement>,
    pub top_level_attributes: BTreeMap<LocalName<'static>, TopLevelAttribute>,
    pub top_level_groups: BTreeMap<LocalName<'static>, TopLevelGroup>,
    pub top_level_attribute_groups: BTreeMap<LocalName<'static>, TopLevelAttributeGroup>,
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

    pub fn from_schema(schema: &xsd::XmlSchema) -> Result<Self, ()> {
        let mut this = Self::new(schema.underlying_schema.target_namespace.clone());

        for redefine in schema.schema_tops() {
            match redefine {
                xs::SchemaTop::Redefineable(redefineable) => match redefineable {
                    xs::Redefineable::SimpleType(simple_type) => {
                        this.import_top_level_simple_type(simple_type)?;
                    }
                    xs::Redefineable::ComplexType(complex_type) => {
                        this.import_top_level_complex_type(complex_type)?;
                    }
                    xs::Redefineable::Group(group) => {
                        this.import_top_level_group(group)?;
                    }
                    xs::Redefineable::AttributeGroup(attribute_group) => {
                        this.import_top_level_attribute_group(attribute_group)?;
                    }
                },
                xs::SchemaTop::Element(element) => {
                    this.import_top_level_element(element)?;
                }
                xs::SchemaTop::Attribute(attribute) => {
                    this.import_top_level_attribute(attribute)?;
                }
                xs::SchemaTop::Notation(_) => {}
                xs::SchemaTop::Annotation(_) => {}
            }
        }

        Ok(this)
    }

    pub fn to_schema(&self) -> xsd::XmlSchema {
        todo!()
    }

    pub fn import_top_level_simple_type(
        &mut self,
        type_: &xs::TopLevelSimpleType,
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
        type_: &xs::TopLevelComplexType,
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

    pub fn import_top_level_group(
        &mut self,
        type_: &xs::GroupType,
    ) -> Result<ExpandedName<'_>, ()> {
        let name = type_.name.clone();

        if self.top_level_groups.contains_key(&name) {
            return Err(());
        }

        let root_fragment = type_.to_complex_fragments(&mut self.complex_type);
        let type_ = TopLevelGroup { root_fragment };
        self.top_level_groups.insert(name.clone(), type_);

        let name = ExpandedName::new(name, Some(self.namespace.as_ref()));

        Ok(name)
    }

    pub fn import_top_level_attribute_group(
        &mut self,
        type_: &xs::AttributeGroupType,
    ) -> Result<ExpandedName<'_>, ()> {
        let name = type_.name.clone();

        if self.top_level_groups.contains_key(&name) {
            return Err(());
        }

        let root_fragment = type_.to_complex_fragments(&mut self.complex_type);
        let type_ = TopLevelAttributeGroup { root_fragment };
        self.top_level_attribute_groups.insert(name.clone(), type_);

        let name = ExpandedName::new(name, Some(self.namespace.as_ref()));

        Ok(name)
    }

    pub fn export_top_level_complex_type(
        &self,
        type_: &LocalName<'_>,
    ) -> Result<Option<xs::TopLevelComplexType>, ()> {
        let Some(TopLevelType::Complex(top_level_complex_type)) = self.top_level_types.get(type_)
        else {
            return Ok(None);
        };

        let fragment_id = &top_level_complex_type.root_fragment;

        let top_level =
            xs::TopLevelComplexType::from_complex_fragments(&self.complex_type, fragment_id)
                .unwrap();

        Ok(Some(top_level))
    }

    pub fn import_top_level_element(
        &mut self,
        element: &xs::TopLevelElement,
    ) -> Result<ExpandedName<'_>, ()> {
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

    pub fn export_top_level_element(
        &self,
        type_: &LocalName<'_>,
    ) -> Result<Option<xs::TopLevelElement>, ()> {
        let Some(top_level_element) = self.top_level_elements.get(type_) else {
            return Ok(None);
        };

        let fragment_id = &top_level_element.root_fragment;

        let top_level =
            xs::TopLevelElement::from_complex_fragments(&self.complex_type, fragment_id).unwrap();

        Ok(Some(top_level))
    }

    pub fn import_top_level_attribute(
        &mut self,
        attribute: &xs::TopLevelAttribute,
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

#[derive(Debug)]
pub struct TopLevelGroup {
    pub root_fragment: complex::FragmentIdx<complex::TopLevelGroupFragment>,
}

#[derive(Debug)]
pub struct TopLevelAttributeGroup {
    pub root_fragment: complex::FragmentIdx<complex::TopLevelAttributeGroupFragment>,
}
