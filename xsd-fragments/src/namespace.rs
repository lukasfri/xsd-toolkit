use std::collections::{BTreeMap, VecDeque};

use xmlity::{ExpandedName, LocalName, XmlNamespace};
use xsd::xs::{self};

use crate::{
    fragments::{
        self,
        complex::{
            ComplexFragmentEquivalent, ComplexTypeRootFragment, RedefinableId,
            TopLevelAttributeGroupFragment, TopLevelGroupFragment,
        },
        simple::{SimpleFragmentEquivalent, SimpleTypeRootFragment},
        Context, FragmentIdx, FragmentedXsdDocumentIdx,
    },
    Error,
};

/// Represents a compiled namespace, which contains all the fragments for that namespace.
#[derive(Debug)]
pub struct FragmentedXsdDocument {
    pub namespace: Option<XmlNamespace<'static>>,
    pub imports: BTreeMap<XmlNamespace<'static>, FragmentedXsdDocumentIdx>,
    pub redefines: VecDeque<Redefine>,
    pub includes: VecDeque<Include>,
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

impl FragmentedXsdDocument {
    /// Creates a new [`CompiledNamespace`] with the given namespace and namespace index.
    pub fn new(
        namespace_idx: FragmentedXsdDocumentIdx,
        namespace: Option<XmlNamespace<'static>>,
    ) -> Self {
        let simple_type_compiler =
            fragments::simple::SimpleTypeFragmentCompiler::new(namespace_idx);
        let complex_type_compiler =
            fragments::complex::ComplexTypeFragmentCompiler::new_with_simple_compiler(
                namespace_idx,
                simple_type_compiler,
            );

        Self {
            namespace,
            imports: BTreeMap::new(),
            redefines: VecDeque::new(),
            includes: VecDeque::new(),
            complex_type_compiler,
            top_level_types: BTreeMap::new(),
            top_level_elements: BTreeMap::new(),
            top_level_attributes: BTreeMap::new(),
            top_level_groups: BTreeMap::new(),
            top_level_attribute_groups: BTreeMap::new(),
        }
    }

    /// Creates a copy of the namespace with a new namespace index.
    pub fn clone_with_namespace(ref_: &Self, namespace_idx: FragmentedXsdDocumentIdx) -> Self {
        let mut complex_type_compiler = ref_.complex_type_compiler.clone();
        complex_type_compiler.namespace_idx = namespace_idx;
        complex_type_compiler.simple_type_compiler.namespace_idx = namespace_idx;

        let mut top_level_types = ref_.top_level_types.clone();
        top_level_types.iter_mut().for_each(|(_, v)| match v {
            TopLevelType::Simple(top_level_simple_type) => {
                top_level_simple_type.root_fragment = FragmentIdx::new(
                    namespace_idx,
                    top_level_simple_type.root_fragment.local_idx(),
                )
            }
            TopLevelType::Complex(top_level_complex_type) => {
                top_level_complex_type.root_fragment = FragmentIdx::new(
                    namespace_idx,
                    top_level_complex_type.root_fragment.local_idx(),
                )
            }
        });

        let mut top_level_elements = ref_.top_level_elements.clone();
        top_level_elements.iter_mut().for_each(|(_, v)| {
            v.root_fragment = FragmentIdx::new(namespace_idx, v.root_fragment.local_idx());
        });

        let mut top_level_attributes = ref_.top_level_attributes.clone();
        top_level_attributes.iter_mut().for_each(|(_, v)| {
            v.root_fragment = FragmentIdx::new(namespace_idx, v.root_fragment.local_idx());
        });

        let mut top_level_groups = ref_.top_level_groups.clone();
        top_level_groups.iter_mut().for_each(|(_, v)| {
            v.root_fragment = FragmentIdx::new(namespace_idx, v.root_fragment.local_idx());
        });

        let mut top_level_attribute_groups = ref_.top_level_attribute_groups.clone();
        top_level_attribute_groups.iter_mut().for_each(|(_, v)| {
            v.root_fragment = FragmentIdx::new(namespace_idx, v.root_fragment.local_idx());
        });

        let mut redefines = ref_.redefines.clone();
        redefines.iter_mut().for_each(|v| {
            v.root_fragment = FragmentIdx::new(namespace_idx, v.root_fragment.local_idx());
        });

        let mut includes = ref_.includes.clone();
        includes.iter_mut().for_each(|v| {
            v.root_fragment = FragmentIdx::new(namespace_idx, v.root_fragment.local_idx());
        });

        Self {
            namespace: ref_.namespace.clone(),
            imports: ref_.imports.clone(),
            complex_type_compiler,
            top_level_types,
            top_level_elements,
            top_level_attributes,
            top_level_groups,
            top_level_attribute_groups,
            redefines,
            includes,
        }
    }

    pub fn merge_with(&mut self, other: &FragmentedXsdDocument) -> Result<(), Error> {
        if other
            .namespace
            .as_ref()
            .is_some_and(|ns| Some(ns) != self.namespace.as_ref())
        {
            todo!(
                "Handle error for merging namespaces with different namespaces: {:?} and {:?}",
                self.namespace,
                other.namespace
            );
        }

        self.imports.extend(other.imports.clone());
        self.complex_type_compiler
            .merge_with(&other.complex_type_compiler)?;

        for (name, top_level_type) in &other.top_level_types {
            self.top_level_types
                .entry(name.clone())
                .or_insert_with(|| top_level_type.clone());
        }

        for (name, top_level_element) in &other.top_level_elements {
            self.top_level_elements
                .entry(name.clone())
                .or_insert_with(|| top_level_element.clone());
        }

        for (name, top_level_attribute) in &other.top_level_attributes {
            self.top_level_attributes
                .entry(name.clone())
                .or_insert_with(|| top_level_attribute.clone());
        }

        for (name, top_level_group) in &other.top_level_groups {
            self.top_level_groups
                .entry(name.clone())
                .or_insert_with(|| top_level_group.clone());
        }

        for (name, top_level_attribute_group) in &other.top_level_attribute_groups {
            self.top_level_attribute_groups
                .entry(name.clone())
                .or_insert_with(|| top_level_attribute_group.clone());
        }

        Ok(())
    }

    /// Imports a redefineable element into the namespace.
    pub fn import_redefineable(
        &mut self,
        redefineable: &xs::groups::Redefinable,
    ) -> Result<RedefinableId, Error> {
        use xs::groups::Redefinable;

        match redefineable {
            Redefinable::SimpleType(simple_type) => self
                .import_simple_type(simple_type)
                .map(RedefinableId::SimpleType),
            Redefinable::ComplexType(complex_type) => self
                .import_complex_type(complex_type)
                .map(RedefinableId::ComplexType),
            Redefinable::Group(group) => self.import_group(group).map(RedefinableId::Group),
            Redefinable::AttributeGroup(attribute_group) => self
                .import_attribute_group(attribute_group)
                .map(RedefinableId::AttributeGroup),
        }
    }

    pub fn import_top_level_redefineable(
        &mut self,
        redefineable: &xs::groups::Redefinable,
    ) -> Result<(), Error> {
        use xs::groups::Redefinable;

        match redefineable {
            Redefinable::SimpleType(simple_type) => {
                self.import_top_level_simple_type(simple_type).map(|_| ())
            }
            Redefinable::ComplexType(complex_type) => {
                self.import_top_level_complex_type(complex_type).map(|_| ())
            }
            Redefinable::Group(group) => self.import_top_level_group(group).map(|_| ()),
            Redefinable::AttributeGroup(attribute_group) => self
                .import_top_level_attribute_group(attribute_group)
                .map(|_| ()),
        }
    }

    /// Imports a schema into the namespace.
    pub fn import_schema(&mut self, schema: &xs::Schema) -> Result<(), Error> {
        let xs::Schema::Schema(schema) = schema else {
            panic!("Expected a schema, but found: {:?}", schema);
        };

        schema
            .child_2
            .iter()
            .map(|a| &a.schema_top)
            .try_for_each(|schema_top| self.import_schema_top(schema_top))
    }

    pub fn export_schema(&self) -> Result<xs::schema_items::Schema, Error> {
        todo!()
    }

    /// Imports a schema top into the namespace.
    pub fn import_schema_top(&mut self, schema_top: &xs::groups::SchemaTop) -> Result<(), Error> {
        use xs::groups::SchemaTop;

        match schema_top {
            SchemaTop::Redefinable(redefineable) => {
                self.import_top_level_redefineable(redefineable)
            }
            SchemaTop::Element(element) => self.import_top_level_element(element).map(|_| ()),
            SchemaTop::Attribute(attribute) => {
                self.import_top_level_attribute(attribute).map(|_| ())
            }
            SchemaTop::Notation(_) => Ok(()),
        }
    }

    // /// Imports a redefine into the namespace.
    // pub fn import_redefine(&mut self, schema: &xs::redefine_items::Redefine) -> Result<(), Error> {
    //     schema
    //         .redefine_content
    //         .iter()
    //         .try_for_each(|schema_top| match schema_top {
    //             xs::redefine_items::RedefineContent::Redefinable(redefineable) => {
    //                 self.import_redefineable(redefineable).map(|_| ())
    //             }
    //             _ => Ok(()),
    //         })
    // }

    /// Imports a schema into the namespace.
    pub fn import_override(&mut self, schema: &xs::override_items::Override) -> Result<(), Error> {
        schema
            .schema_top
            .iter()
            .try_for_each(|schema_top| self.import_schema_top(schema_top))
    }

    pub fn import_simple_type(
        &mut self,
        simple_type: &xs::SimpleType,
    ) -> Result<FragmentIdx<SimpleTypeRootFragment>, Error> {
        let simple_type = match simple_type {
            xs::SimpleType::SimpleType(simple_type) => simple_type,
            _ => panic!("Expected a simple type, but found: {:?}", simple_type),
        };

        simple_type
            .to_simple_fragments(
                self.complex_type_compiler.as_mut(),
                &Context {
                    default_namespace: self.namespace.as_ref(),
                },
            )
            .map_err(Error::SimpleFragmentError)
    }

    /// Imports a top-level simple type from the XSD namespace.
    pub fn import_top_level_simple_type(
        &mut self,
        simple_type: &xs::SimpleType,
    ) -> Result<ExpandedName<'_>, Error> {
        let root_fragment = self.import_simple_type(simple_type)?;

        let simple_type = match simple_type {
            xs::SimpleType::SimpleType(simple_type) => simple_type,
            _ => panic!("Expected a simple type, but found: {:?}", simple_type),
        };

        let local_name = simple_type.name.clone();
        let name = ExpandedName::new(local_name.clone(), self.namespace.clone());

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
            self.complex_type_compiler.as_ref(),
            fragment_id,
        )?;

        Ok(Some(xs::SimpleType::from(type_)))
    }

    pub fn import_complex_type(
        &mut self,
        complex_type: &xs::ComplexType,
    ) -> Result<FragmentIdx<ComplexTypeRootFragment>, Error> {
        let complex_type = match complex_type {
            xs::ComplexType::ComplexType(complex_type) => complex_type,
            _ => panic!("Expected a complex type, but found: {:?}", complex_type),
        };

        complex_type
            .to_complex_fragments(
                &mut self.complex_type_compiler,
                &Context {
                    default_namespace: self.namespace.as_ref(),
                },
            )
            .map_err(Error::ComplexFragmentError)
    }

    /// Imports a top-level complex type from the XSD namespace.
    pub fn import_top_level_complex_type(
        &mut self,
        complex_type: &xs::ComplexType,
    ) -> Result<ExpandedName<'_>, Error> {
        let root_fragment = self.import_complex_type(complex_type)?;

        let complex_type = match complex_type {
            xs::ComplexType::ComplexType(complex_type) => complex_type,
            _ => panic!("Expected a complex type, but found: {:?}", complex_type),
        };

        let local_name = complex_type.name.clone();
        let name = ExpandedName::new(local_name.clone(), self.namespace.clone());

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
        let name = ExpandedName::new(local_name.clone(), self.namespace.clone());

        let root_fragment = element.to_complex_fragments(
            &mut self.complex_type_compiler,
            &Context {
                default_namespace: self.namespace.as_ref(),
            },
        )?;

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
        let name = ExpandedName::new(local_name.clone(), self.namespace.clone());

        let root_fragment = attribute.to_complex_fragments(
            &mut self.complex_type_compiler,
            &Context {
                default_namespace: self.namespace.as_ref(),
            },
        )?;

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

    fn import_group(
        &mut self,
        group: &xs::Group,
    ) -> Result<FragmentIdx<TopLevelGroupFragment>, Error> {
        let group = match group {
            xs::Group::Group(group) => group,
            _ => {
                panic!("Expected a group, but found: {:?}", group);
            }
        };

        let root_fragment = group.to_complex_fragments(
            &mut self.complex_type_compiler,
            &Context {
                default_namespace: self.namespace.as_ref(),
            },
        )?;

        Ok(root_fragment)
    }

    /// Imports a top-level group from the XSD namespace.
    pub fn import_top_level_group(&mut self, group: &xs::Group) -> Result<ExpandedName<'_>, Error> {
        let root_fragment = self.import_group(group)?;

        let group = match group {
            xs::Group::Group(group) => group,
            _ => {
                panic!("Expected a group, but found: {:?}", group);
            }
        };

        let local_name = group.name.clone();
        let name = ExpandedName::new(local_name.clone(), self.namespace.clone());

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

    fn import_attribute_group(
        &mut self,
        attribute_group: &xs::AttributeGroup,
    ) -> Result<FragmentIdx<TopLevelAttributeGroupFragment>, Error> {
        let attribute_group = match attribute_group {
            xs::AttributeGroup::AttributeGroup(attribute_group) => attribute_group,
            _ => {
                panic!(
                    "Expected an attribute group, but found: {:?}",
                    attribute_group
                );
            }
        };

        let root_fragment = attribute_group.to_complex_fragments(
            &mut self.complex_type_compiler,
            &Context {
                default_namespace: self.namespace.as_ref(),
            },
        )?;

        Ok(root_fragment)
    }

    /// Imports a top-level attribute group from the XSD namespace.
    pub fn import_top_level_attribute_group(
        &mut self,
        attribute_group: &xs::AttributeGroup,
    ) -> Result<ExpandedName<'_>, Error> {
        let root_fragment = self.import_attribute_group(attribute_group)?;

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
        let name = ExpandedName::new(local_name.clone(), self.namespace.clone());

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
