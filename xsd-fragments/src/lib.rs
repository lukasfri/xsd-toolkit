use std::collections::BTreeMap;
use std::ops::Deref;

use xmlity::{ExpandedName, LocalName, XmlNamespace};
use xsd::xs;

use crate::fragments::{FragmentIdx, NamespaceIdx};
pub mod fragments;
use crate::fragments::complex::ComplexFragmentEquivalent;
use crate::fragments::simple::SimpleFragmentEquivalent;

#[derive(Debug)]
pub enum Error {
    ImportOfExistingEntity,
    NonExistentXmlNamespace { namespace: XmlNamespace<'static> },
}

#[derive(Debug)]
pub struct XmlnsContext {
    pub namespaces: BTreeMap<NamespaceIdx, CompiledNamespace>,
    pub namespace_idxs: BTreeMap<XmlNamespace<'static>, NamespaceIdx>,
    namespace_id_count: usize,
}

impl XmlnsContext {
    pub fn new() -> Self {
        Self {
            namespaces: BTreeMap::new(),
            namespace_idxs: BTreeMap::new(),
            namespace_id_count: 0,
        }
    }

    fn generate_fragment_id(&mut self) -> NamespaceIdx {
        let fragment_id = NamespaceIdx::new(self.namespace_id_count);
        self.namespace_id_count += 1;
        fragment_id
    }

    pub fn init_namespace(&mut self, namespace: XmlNamespace<'static>) -> &mut CompiledNamespace {
        let namespace_idx = self.generate_fragment_id();
        self.namespace_idxs.insert(namespace.clone(), namespace_idx);

        let namespace = CompiledNamespace::new(namespace, namespace_idx);

        self.namespaces.insert(namespace_idx, namespace);

        self.namespaces
            .get_mut(&namespace_idx)
            .expect("Just inserted namespace")
    }

    pub fn get_namespace(&self, namespace: &XmlNamespace<'_>) -> Option<&CompiledNamespace> {
        let Some(namespace_idx) = self.namespace_idxs.get(namespace) else {
            return None;
        };

        self.namespaces.get(namespace_idx)
    }

    pub fn get_namespace_mut(
        &mut self,
        namespace: &XmlNamespace<'_>,
    ) -> Option<&mut CompiledNamespace> {
        let Some(namespace_idx) = self.namespace_idxs.get(namespace) else {
            return None;
        };

        self.namespaces.get_mut(namespace_idx)
    }

    pub fn import_redefine(&mut self, redefine: &xs::Redefine) -> Result<(), Error> {
        use xs::redefine_items::RedefineContent;
        let redefine = match redefine {
            xs::Redefine::Redefine(redefine) => redefine,
            _ => panic!("Expected a redefine, but found: {:?}", redefine),
        };

        let namespace = &redefine.schema_location.0;

        let compiled_namespace =
            self.get_namespace_mut(namespace)
                .ok_or(Error::NonExistentXmlNamespace {
                    namespace: namespace.clone(),
                })?;

        redefine
            .redefine_content
            .iter()
            .filter_map(|r| match r {
                RedefineContent::Annotation(_) => None,
                RedefineContent::Redefinable(redefinable) => Some(redefinable.deref()),
            })
            .map(|r| compiled_namespace.import_redefineable(r))
            .collect::<Result<(), Error>>()
    }

    pub fn import_namespace_map(
        &mut self,
        map: &xsd_namespace_map::XmlNamespaceMap,
    ) -> Result<(), Error> {
        map.locations
            .iter()
            .filter_map(|(_, location)| location.as_ref())
            .map(|location| self.import_schema(&location.schema))
            .collect::<Result<(), Error>>()
    }

    pub fn import_schema(&mut self, schema: &xsd::XmlSchema) -> Result<(), Error> {
        use xs::groups::Composition;
        let namespace = schema.namespace();

        schema
            .compositions()
            .map(|c| match c {
                Composition::Include(include) => Ok(()),
                Composition::Import(import) => Ok(()),
                Composition::Redefine(redefine) => self.import_redefine(redefine),
                Composition::Override(_) => todo!(),
                Composition::Annotation(annotation) => Ok(()),
            })
            .collect::<Result<(), Error>>()?;

        let compiled_namespace = if let Some(compiled_namespace) = self.get_namespace_mut(namespace)
        {
            compiled_namespace
        } else {
            self.init_namespace(namespace.clone())
        };

        compiled_namespace.import_schema(schema)
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
    pub fn new(namespace: XmlNamespace<'static>, namespace_idx: NamespaceIdx) -> Self {
        let simple_type_compiler =
            fragments::simple::SimpleTypeFragmentCompiler::new(namespace.clone(), namespace_idx);
        let complex_type_compiler = fragments::complex::ComplexTypeFragmentCompiler::new(
            namespace.clone(),
            namespace_idx,
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

    pub fn import_include(&mut self, include: &xs::Include) -> Result<(), Error> {
        let include = match include {
            xs::Include::Include(include) => include,
            _ => panic!("Expected an include, but found: {:?}", include),
        };

        let include_namespace = &include.schema_location.0;
        todo!()
    }

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

        let root_fragment = simple_type.to_simple_fragments(&mut self.complex_type);
        let type_ = TopLevelType::Simple(TopLevelSimpleType { root_fragment });
        self.top_level_types.insert(local_name.clone(), type_);

        Ok(name)
    }

    pub fn export_top_level_simple_type(
        &self,
        name: &LocalName<'_>,
    ) -> Result<Option<xs::SimpleType>, Error> {
        let Some(TopLevelType::Simple(type_)) = self.top_level_types.get(name) else {
            return Ok(None);
        };

        let fragment_id = &type_.root_fragment;

        let type_ =
            xs::types::TopLevelSimpleType::from_simple_fragments(&self.complex_type, fragment_id)
                .unwrap();

        Ok(Some(xs::SimpleType::from(type_)))
    }

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

        let root_fragment = complex_type.to_complex_fragments(&mut self.complex_type);

        let type_ = TopLevelType::Complex(TopLevelComplexType { root_fragment });
        self.top_level_types.insert(local_name.clone(), type_);

        Ok(name)
    }

    pub fn export_top_level_complex_type(
        &self,
        name: &LocalName<'_>,
    ) -> Result<Option<xs::ComplexType>, Error> {
        let Some(TopLevelType::Complex(type_)) = self.top_level_types.get(name) else {
            return Ok(None);
        };

        let fragment_id = &type_.root_fragment;

        let type_ =
            xs::types::TopLevelComplexType::from_complex_fragments(&self.complex_type, fragment_id)
                .unwrap();

        Ok(Some(xs::ComplexType::from(type_)))
    }

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

        let root_fragment = element.to_complex_fragments(&mut self.complex_type);

        self.top_level_elements
            .insert(local_name.clone(), TopLevelElement { root_fragment });

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

        Ok(Some(xs::Element::from(element)))
    }

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

        let root_fragment = attribute.to_complex_fragments(&mut self.complex_type);

        self.top_level_attributes
            .insert(local_name.clone(), TopLevelAttribute { root_fragment });

        Ok(name)
    }

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

        let root_fragment = group.to_complex_fragments(&mut self.complex_type);
        let type_ = TopLevelGroup { root_fragment };
        self.top_level_groups.insert(local_name.clone(), type_);

        Ok(name)
    }

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

        let root_fragment = attribute_group.to_complex_fragments(&mut self.complex_type);
        let type_ = TopLevelAttributeGroup { root_fragment };
        self.top_level_attribute_groups
            .insert(local_name.clone(), type_);

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
