use std::collections::BTreeMap;

use xmlity::{ExpandedName, LocalName, XmlNamespace};
use xsd::xs;

use crate::{
    fragments::{
        self, complex::ComplexFragmentEquivalent, simple::SimpleFragmentEquivalent, FragmentIdx,
        NamespaceIdx,
    },
    Error,
};

/// Represents a compiled namespace, which contains all the fragments for that namespace.
#[derive(Debug)]
pub struct CompiledNamespace {
    /// The XML namespace associated with this compiled namespace.
    pub namespace: XmlNamespace<'static>,
    /// The [`ComplexTypeFragmentCompiler`] for complex types, which also contains a [`SimpleTypeFragmentCompiler`].
    pub complex_type_compiler: fragments::complex::ComplexTypeFragmentCompiler,
    /// A map of top-level types, which can be either simple or complex.
    pub top_level_types: BTreeMap<LocalName<'static>, TopLevelType>,
    /// A map of top-level elements.
    pub top_level_elements: BTreeMap<LocalName<'static>, TopLevelElement>,
    /// A map of top-level attributes.
    pub top_level_attributes: BTreeMap<LocalName<'static>, TopLevelAttribute>,
    /// A map of top-level groups.
    pub top_level_groups: BTreeMap<LocalName<'static>, TopLevelGroup>,
    /// A map of top-level attribute groups.
    pub top_level_attribute_groups: BTreeMap<LocalName<'static>, TopLevelAttributeGroup>,
}

impl CompiledNamespace {
    /// Creates a new [`CompiledNamespace`] with the given namespace and namespace index.
    pub fn new(namespace: XmlNamespace<'static>, namespace_idx: NamespaceIdx) -> Self {
        let simple_type_compiler =
            fragments::simple::SimpleTypeFragmentCompiler::new(namespace.clone(), namespace_idx);
        let complex_type_compiler =
            fragments::complex::ComplexTypeFragmentCompiler::new_with_simple_compiler(
                namespace.clone(),
                namespace_idx,
                simple_type_compiler,
            );

        Self {
            namespace,
            complex_type_compiler,
            top_level_types: BTreeMap::new(),
            top_level_elements: BTreeMap::new(),
            top_level_attributes: BTreeMap::new(),
            top_level_groups: BTreeMap::new(),
            top_level_attribute_groups: BTreeMap::new(),
        }
    }

    /// Imports a redefineable element into the namespace.
    pub fn import_redefineable(
        &mut self,
        redefineable: &xs::groups::Redefinable,
    ) -> Result<(), Error> {
        use xs::groups::Redefinable;

        match redefineable {
            Redefinable::SimpleType(simple_type) => {
                self.import_top_level_simple_type(simple_type)?;
            }
            Redefinable::ComplexType(complex_type) => {
                self.import_top_level_complex_type(complex_type)?;
            }
            Redefinable::Group(group) => {
                self.import_top_level_group(group)?;
            }
            Redefinable::AttributeGroup(attribute_group) => {
                self.import_top_level_attribute_group(attribute_group)?;
            }
        }

        Ok(())
    }

    /// Imports a schema into the namespace.
    pub fn import_schema(&mut self, schema: &xsd::XmlSchema) -> Result<(), Error> {
        use xs::groups::SchemaTop;

        for schema_top in schema.schema_tops() {
            match schema_top {
                SchemaTop::Redefinable(redefineable) => self.import_redefineable(redefineable)?,
                SchemaTop::Element(element) => {
                    self.import_top_level_element(element)?;
                }
                SchemaTop::Attribute(attribute) => {
                    self.import_top_level_attribute(attribute)?;
                }
                SchemaTop::Notation(_) => {}
            }
        }

        Ok(())
    }

    /// Imports a top-level simple type from the XSD namespace.
    pub fn import_top_level_simple_type(
        &mut self,
        simple_type: &xs::SimpleType,
    ) -> Result<ExpandedName<'_>, Error> {
        let simple_type = match simple_type {
            xs::SimpleType::SimpleType(simple_type) => simple_type,
            _ => panic!("Expected a simple type, but found: {:?}", simple_type),
        };

        let local_name = simple_type.name.clone();
        let name = ExpandedName::new(local_name.clone(), Some(self.namespace.as_ref()));

        if self.top_level_types.contains_key(&local_name) {
            return Ok(name);
        }

        let root_fragment = simple_type.to_simple_fragments(&mut self.complex_type_compiler)?;

        let value = TopLevelType::Simple(TopLevelSimpleType { root_fragment });
        self.top_level_types.insert(local_name, value);

        Ok(name)
    }

    /// Exports a top-level simple type from the XSD namespace.
    pub fn export_top_level_simple_type(
        &self,
        name: &LocalName<'_>,
    ) -> Result<Option<xs::SimpleType>, Error> {
        let Some(TopLevelType::Simple(type_)) = self.top_level_types.get(name) else {
            return Ok(None);
        };

        let fragment_id = &type_.root_fragment;

        let type_ = xs::types::TopLevelSimpleType::from_simple_fragments(
            &self.complex_type_compiler,
            fragment_id,
        )?;

        Ok(Some(xs::SimpleType::from(type_)))
    }

    /// Imports a top-level complex type from the XSD namespace.
    pub fn import_top_level_complex_type(
        &mut self,
        complex_type: &xs::ComplexType,
    ) -> Result<ExpandedName<'_>, Error> {
        let complex_type = match complex_type {
            xs::ComplexType::ComplexType(complex_type) => complex_type,
            _ => panic!("Expected a complex type, but found: {:?}", complex_type),
        };

        let local_name = complex_type.name.clone();
        let name = ExpandedName::new(local_name.clone(), Some(self.namespace.as_ref()));

        if self.top_level_types.contains_key(&local_name) {
            return Ok(name);
        }

        let root_fragment = complex_type.to_complex_fragments(&mut self.complex_type_compiler)?;

        let value = TopLevelType::Complex(TopLevelComplexType { root_fragment });
        self.top_level_types.insert(local_name, value);

        Ok(name)
    }

    /// Exports a top-level complex type from the XSD namespace.
    pub fn export_top_level_complex_type(
        &self,
        name: &LocalName<'_>,
    ) -> Result<Option<xs::ComplexType>, Error> {
        let Some(TopLevelType::Complex(type_)) = self.top_level_types.get(name) else {
            return Ok(None);
        };

        let fragment_id = &type_.root_fragment;

        let type_ = xs::types::TopLevelComplexType::from_complex_fragments(
            &self.complex_type_compiler,
            fragment_id,
        )?;

        Ok(Some(xs::ComplexType::from(type_)))
    }

    /// Imports a top-level element from the XSD namespace.
    pub fn import_top_level_element(
        &mut self,
        element: &xs::Element,
    ) -> Result<ExpandedName<'_>, Error> {
        let element = match element {
            xs::Element::Element(element) => element,
            _ => panic!("Expected an element, but found: {:?}", element),
        };

        let local_name = element.name.clone();
        let name = ExpandedName::new(local_name.clone(), Some(self.namespace.as_ref()));

        if self.top_level_elements.contains_key(&local_name) {
            return Ok(name);
        }

        let root_fragment = element.to_complex_fragments(&mut self.complex_type_compiler)?;

        let value = TopLevelElement { root_fragment };
        self.top_level_elements.insert(local_name, value);

        Ok(name)
    }

    /// Exports a top-level element from the XSD namespace.
    pub fn export_top_level_element(
        &self,
        element: &LocalName<'_>,
    ) -> Result<Option<xs::Element>, Error> {
        let Some(top_level_element) = self.top_level_elements.get(element) else {
            return Ok(None);
        };

        let fragment_id = &top_level_element.root_fragment;

        let element = xs::types::TopLevelElement::from_complex_fragments(
            &self.complex_type_compiler,
            fragment_id,
        )?;

        Ok(Some(xs::Element::from(element)))
    }

    /// Imports a top-level attribute from the XSD namespace.
    pub fn import_top_level_attribute(
        &mut self,
        attribute: &xs::Attribute,
    ) -> Result<ExpandedName<'_>, Error> {
        let attribute = match attribute {
            xs::Attribute::Attribute(attribute) => attribute,
            _ => panic!("Expected an attribute, but found: {:?}", attribute),
        };

        let local_name = attribute.name.clone();
        let name = ExpandedName::new(local_name.clone(), Some(self.namespace.as_ref()));

        if self.top_level_attributes.contains_key(&local_name) {
            return Ok(name);
        }

        let root_fragment = attribute.to_complex_fragments(&mut self.complex_type_compiler)?;

        let value = TopLevelAttribute { root_fragment };
        self.top_level_attributes.insert(local_name, value);

        Ok(name)
    }

    /// Exports a top-level attribute from the XSD namespace.
    pub fn export_top_level_attribute(
        &self,
        attribute: &LocalName<'_>,
    ) -> Result<Option<xs::Attribute>, Error> {
        let Some(top_level_attribute) = self.top_level_attributes.get(attribute) else {
            return Ok(None);
        };

        let fragment_id = &top_level_attribute.root_fragment;

        let attribute = xs::types::TopLevelAttribute::from_complex_fragments(
            &self.complex_type_compiler,
            fragment_id,
        )?;

        Ok(Some(xs::Attribute::from(attribute)))
    }

    /// Imports a top-level group from the XSD namespace.
    pub fn import_top_level_group(&mut self, group: &xs::Group) -> Result<ExpandedName<'_>, Error> {
        let group = match group {
            xs::Group::Group(group) => group,
            _ => {
                panic!("Expected a group, but found: {:?}", group);
            }
        };

        let local_name = group.name.clone();
        let name = ExpandedName::new(local_name.clone(), Some(self.namespace.as_ref()));

        if self.top_level_groups.contains_key(&local_name) {
            return Ok(name);
        }

        let root_fragment = group.to_complex_fragments(&mut self.complex_type_compiler)?;

        let value = TopLevelGroup { root_fragment };
        self.top_level_groups.insert(local_name.clone(), value);

        Ok(name)
    }

    /// Exports a top-level group from the XSD namespace.
    pub fn export_top_level_group(
        &self,
        group: &LocalName<'_>,
    ) -> Result<Option<xs::Group>, Error> {
        let Some(top_level_group) = self.top_level_groups.get(group) else {
            return Ok(None);
        };

        let fragment_id = &top_level_group.root_fragment;

        let group = xs::types::NamedGroup::from_complex_fragments(
            &self.complex_type_compiler,
            fragment_id,
        )?;

        Ok(Some(xs::Group::from(group)))
    }

    /// Imports a top-level attribute group from the XSD namespace.
    pub fn import_top_level_attribute_group(
        &mut self,
        attribute_group: &xs::AttributeGroup,
    ) -> Result<ExpandedName<'_>, Error> {
        let attribute_group = match attribute_group {
            xs::AttributeGroup::AttributeGroup(attribute_group) => attribute_group,
            _ => {
                panic!(
                    "Expected an attribute group, but found: {:?}",
                    attribute_group
                );
            }
        };

        let local_name = attribute_group.name.clone();
        let name = ExpandedName::new(local_name.clone(), Some(self.namespace.as_ref()));

        if self.top_level_groups.contains_key(&local_name) {
            return Ok(name);
        }

        let root_fragment =
            attribute_group.to_complex_fragments(&mut self.complex_type_compiler)?;

        let value = TopLevelAttributeGroup { root_fragment };
        self.top_level_attribute_groups.insert(local_name, value);

        Ok(name)
    }

    /// Exports a top-level attribute group from the XSD namespace.
    pub fn export_top_level_attribute_group(
        &self,
        attribute_group: &LocalName<'_>,
    ) -> Result<Option<xs::AttributeGroup>, Error> {
        let Some(top_level_attribute_group) = self.top_level_attribute_groups.get(attribute_group)
        else {
            return Ok(None);
        };

        let fragment_id = &top_level_attribute_group.root_fragment;

        let attribute_group = xs::types::NamedAttributeGroup::from_complex_fragments(
            &self.complex_type_compiler,
            fragment_id,
        )?;

        Ok(Some(xs::AttributeGroup::from(attribute_group)))
    }
}

/// Represents a top-level simple type in the XSD namespace.
#[derive(Debug)]
pub struct TopLevelSimpleType {
    /// The root fragment of the simple type.
    pub root_fragment: fragments::FragmentIdx<fragments::simple::SimpleTypeRootFragment>,
}

/// Represents a top-level complex type in the XSD namespace.
#[derive(Debug)]
pub struct TopLevelComplexType {
    /// The root fragment of the complex type.
    pub root_fragment: FragmentIdx<fragments::complex::ComplexTypeRootFragment>,
}

/// Represents a top-level type in the XSD namespace, which can be either simple or complex.
#[derive(Debug)]
pub enum TopLevelType {
    /// A simple type.
    Simple(TopLevelSimpleType),
    /// A complex type.
    Complex(TopLevelComplexType),
}

/// Represents a top-level element in the XSD namespace.
#[derive(Debug)]
pub struct TopLevelElement {
    /// The root fragment of the complex type.
    pub root_fragment: FragmentIdx<fragments::complex::TopLevelElementFragment>,
}

/// Represents a top-level attribute in the XSD namespace.
#[derive(Debug)]
pub struct TopLevelAttribute {
    /// The root fragment of the complex type.
    pub root_fragment: FragmentIdx<fragments::complex::TopLevelAttributeFragment>,
}

/// Represents a top-level group in the XSD namespace.
#[derive(Debug)]
pub struct TopLevelGroup {
    /// The root fragment of the complex type.
    pub root_fragment: FragmentIdx<fragments::complex::TopLevelGroupFragment>,
}

/// Represents a top-level attribute group in the XSD namespace.
#[derive(Debug)]
pub struct TopLevelAttributeGroup {
    /// The root fragment of the complex type.
    pub root_fragment: FragmentIdx<fragments::complex::TopLevelAttributeGroupFragment>,
}
