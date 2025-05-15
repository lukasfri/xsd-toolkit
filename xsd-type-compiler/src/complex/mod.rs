//! This module contains the logic for compiling complex types into fragments.
//!
//! It is naturally dependent on the simple type compiler, as complex types can contain simple types.

pub mod transformers;

use std::{
    collections::{BTreeMap, VecDeque},
    sync::LazyLock,
};

use crate::{
    simple::{SimpleTypeFragmentCompiler, ToSimpleFragments},
    ComplexTypeIdent, SimpleTypeIdent,
};
use xmlity::{ExpandedName, LocalName, XmlNamespace};
use xsd::schema::{
    AllType, Any, ChoiceType, ElementTypeContent, GroupRef, GroupTypeContent, LocalElement,
    MaxOccursValue, MinOccurs, SequenceType, TypeDefParticle,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FragmentIdx(usize);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FragmentId(pub XmlNamespace<'static>, pub FragmentIdx);

pub struct SimpleContent {}

pub struct ComplexContent {}

pub enum Content {
    Simple(SimpleContent),
    Complex(ComplexContent),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Extension {
    pub base: ComplexTypeIdent,
    pub content_fragment: Option<FragmentId>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Restriction {
    pub base: ComplexTypeIdent,
    pub content_fragment: Option<FragmentId>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SimpleContentFragment {}

#[derive(Debug, Clone, PartialEq)]
pub enum ComplexContentFragment {
    Extension(Extension),
    Restriction(Restriction),
}

#[derive(Debug, Clone, PartialEq)]
pub struct LocalElementTypeFragment {
    pub name: LocalName<'static>,
    pub type_: ComplexTypeIdent,
    pub min_occurs: Option<MinOccurs>,
    pub max_occurs: Option<MaxOccursValue>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ReferenceElementTypeFragment {
    pub name: ExpandedName<'static>,
    pub min_occurs: Option<MinOccurs>,
    pub max_occurs: Option<MaxOccursValue>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ElementTypeFragment {
    Local(LocalElementTypeFragment),
    Reference(ReferenceElementTypeFragment),
}

#[derive(Debug, Clone, PartialEq)]
pub struct GroupRefTypeFragment {
    pub ref_: ComplexTypeIdent,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AllTypeFragment {
    pub fragments: VecDeque<FragmentId>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ChoiceTypeFragment {
    pub fragments: VecDeque<FragmentId>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SequenceTypeFragment {
    pub fragments: VecDeque<FragmentId>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AttributeTypeFragment {
    pub name: LocalName<'static>,
    pub type_: SimpleTypeIdent,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SimpleTypeTypeFragment {
    pub type_: SimpleTypeIdent,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ComplexTypeFragment {
    SimpleContent(SimpleContentFragment),
    ComplexContent(ComplexContentFragment),
    GroupRef(GroupRefTypeFragment),
    All(AllTypeFragment),
    Choice(ChoiceTypeFragment),
    Sequence(SequenceTypeFragment),
    Element(ElementTypeFragment),
    Attribute(AttributeTypeFragment),
    SimpleType(SimpleTypeTypeFragment),
}

#[derive(Debug, Clone)]
pub struct ComplexTypeFragmentCompiler {
    fragment_id_count: usize,
    pub namespace: XmlNamespace<'static>,
    pub fragments: BTreeMap<FragmentIdx, ComplexTypeFragment>,
    pub simple_type_fragments: SimpleTypeFragmentCompiler,
}

impl ComplexTypeFragmentCompiler {
    pub fn new(
        namespace: XmlNamespace<'static>,
        simple_type_fragments: SimpleTypeFragmentCompiler,
    ) -> Self {
        Self {
            fragment_id_count: 0,
            fragments: BTreeMap::new(),
            namespace,
            simple_type_fragments,
        }
    }

    fn generate_fragment_id(&mut self) -> FragmentId {
        let fragment_id = FragmentId(self.namespace.clone(), FragmentIdx(self.fragment_id_count));
        self.fragment_id_count += 1;
        fragment_id
    }

    pub fn push_fragment(&mut self, fragment: ComplexTypeFragment) -> FragmentId {
        let fragment_id = self.generate_fragment_id();

        self.fragments.insert(fragment_id.1, fragment);

        fragment_id
    }

    pub fn get_fragment(&self, idx: &FragmentId) -> Option<&ComplexTypeFragment> {
        if self.namespace != idx.0 {
            return None;
        }

        self.fragments.get(&idx.1)
    }

    pub fn get_fragment_mut(&mut self, idx: &FragmentId) -> Option<&mut ComplexTypeFragment> {
        if self.namespace != idx.0 {
            return None;
        }

        self.fragments.get_mut(&idx.1)
    }

    pub fn iter_fragment_ids(&self) -> impl Iterator<Item = FragmentId> {
        self.fragments
            .keys()
            .map(|idx| FragmentId(self.namespace.clone(), *idx))
            .collect::<Vec<_>>()
            .into_iter()
    }
}

impl AsMut<SimpleTypeFragmentCompiler> for ComplexTypeFragmentCompiler {
    fn as_mut(&mut self) -> &mut SimpleTypeFragmentCompiler {
        &mut self.simple_type_fragments
    }
}

impl AsMut<ComplexTypeFragmentCompiler> for ComplexTypeFragmentCompiler {
    fn as_mut(&mut self) -> &mut ComplexTypeFragmentCompiler {
        self
    }
}

pub trait ToComplexFragments {
    fn to_complex_fragments<T: AsMut<ComplexTypeFragmentCompiler>>(
        &self,
        compiler: T,
    ) -> FragmentId;
}

impl ToComplexFragments for ElementTypeContent {
    fn to_complex_fragments<T: AsMut<ComplexTypeFragmentCompiler>>(
        &self,
        mut compiler: T,
    ) -> FragmentId {
        let mut compiler = compiler.as_mut();

        match self {
            ElementTypeContent::SimpleType(local_simple_type) => {
                let simple_type_fragment = local_simple_type.to_simple_fragments(&mut compiler);

                compiler.push_fragment(ComplexTypeFragment::SimpleType(SimpleTypeTypeFragment {
                    type_: SimpleTypeIdent::Anonymous(simple_type_fragment),
                }))
            }
            ElementTypeContent::ComplexType(local_complex_type) => {
                local_complex_type.to_complex_fragments(compiler)
            }
        }
    }
}

impl ToComplexFragments for LocalElement {
    fn to_complex_fragments<T: AsMut<ComplexTypeFragmentCompiler>>(
        &self,
        mut compiler: T,
    ) -> FragmentId {
        let mut compiler = compiler.as_mut();

        let max_occurs = self.max_occurs.as_ref().map(|a| a.0);
        let min_occurs = self.min_occurs.clone();

        if let Some(ref_) = self.ref_.as_ref() {
            compiler.push_fragment(ComplexTypeFragment::Element(
                ElementTypeFragment::Reference(ReferenceElementTypeFragment {
                    name: ref_.0 .0.clone(),
                    max_occurs,
                    min_occurs,
                }),
            ))
        } else {
            let name = self
                .name
                .as_ref()
                .map(|a| a.0.clone())
                .expect("If ref is none, type_choice should be Some");

            let type_: ComplexTypeIdent = if let Some(type_) = self.type_.as_ref() {
                ComplexTypeIdent::Named(type_.0 .0.clone())
            } else {
                let type_choice = self
                    .type_choice
                    .as_ref()
                    .expect("If ref is none and type is none, type_choice should be Some");

                let content_type = type_choice.to_complex_fragments(&mut compiler);

                ComplexTypeIdent::Anonymous(content_type)
            };

            compiler.push_fragment(ComplexTypeFragment::Element(ElementTypeFragment::Local(
                LocalElementTypeFragment {
                    name,
                    type_,
                    max_occurs,
                    min_occurs,
                },
            )))
        }
    }
}

impl ToComplexFragments for GroupRef {
    fn to_complex_fragments<T: AsMut<ComplexTypeFragmentCompiler>>(
        &self,
        mut compiler: T,
    ) -> FragmentId {
        let compiler = compiler.as_mut();

        compiler.push_fragment(ComplexTypeFragment::GroupRef(GroupRefTypeFragment {
            ref_: ComplexTypeIdent::from(self.ref_.0 .0.clone()),
        }))
    }
}

impl ToComplexFragments for Any {
    fn to_complex_fragments<T: AsMut<ComplexTypeFragmentCompiler>>(
        &self,
        mut compiler: T,
    ) -> FragmentId {
        let _compiler = compiler.as_mut();

        todo!()
    }
}

impl ToComplexFragments for GroupTypeContent {
    fn to_complex_fragments<T: AsMut<ComplexTypeFragmentCompiler>>(
        &self,
        mut compiler: T,
    ) -> FragmentId {
        let compiler = compiler.as_mut();

        match self {
            GroupTypeContent::Element(local_element) => {
                local_element.to_complex_fragments(compiler)
            }
            GroupTypeContent::Group(group_type) => group_type.to_complex_fragments(compiler),
            GroupTypeContent::All(all_type) => all_type.to_complex_fragments(compiler),
            GroupTypeContent::Choice(choice_type) => choice_type.to_complex_fragments(compiler),
            GroupTypeContent::Sequence(sequence_type) => {
                sequence_type.to_complex_fragments(compiler)
            }
            GroupTypeContent::Any(any) => any.to_complex_fragments(compiler),
        }
    }
}

impl ToComplexFragments for AllType {
    fn to_complex_fragments<T: AsMut<ComplexTypeFragmentCompiler>>(
        &self,
        mut compiler: T,
    ) -> FragmentId {
        let mut compiler = compiler.as_mut();

        let all = ComplexTypeFragment::All(AllTypeFragment {
            fragments: self
                .content
                .iter()
                .map(|content| content.to_complex_fragments(&mut compiler))
                .collect(),
        });

        compiler.push_fragment(all)
    }
}

impl ToComplexFragments for ChoiceType {
    fn to_complex_fragments<T: AsMut<ComplexTypeFragmentCompiler>>(
        &self,
        mut compiler: T,
    ) -> FragmentId {
        let mut compiler = compiler.as_mut();

        let all = ComplexTypeFragment::Choice(ChoiceTypeFragment {
            fragments: self
                .content
                .iter()
                .map(|content| content.to_complex_fragments(&mut compiler))
                .collect(),
        });

        compiler.push_fragment(all)
    }
}

impl ToComplexFragments for SequenceType {
    fn to_complex_fragments<T: AsMut<ComplexTypeFragmentCompiler>>(
        &self,
        mut compiler: T,
    ) -> FragmentId {
        let mut compiler = compiler.as_mut();

        let all = ComplexTypeFragment::Sequence(SequenceTypeFragment {
            fragments: self
                .content
                .iter()
                .map(|content| content.to_complex_fragments(&mut compiler))
                .collect(),
        });

        compiler.push_fragment(all)
    }
}

impl ToComplexFragments for TypeDefParticle {
    fn to_complex_fragments<T: AsMut<ComplexTypeFragmentCompiler>>(
        &self,
        mut compiler: T,
    ) -> FragmentId {
        let compiler = compiler.as_mut();

        match self {
            xsd::schema::TypeDefParticle::Group(group_ref) => {
                let type_ident = ComplexTypeIdent::from(group_ref.ref_.0 .0.clone());

                compiler.push_fragment(ComplexTypeFragment::GroupRef(GroupRefTypeFragment {
                    ref_: type_ident,
                }))
            }
            xsd::schema::TypeDefParticle::All(all) => all.to_complex_fragments(compiler),
            xsd::schema::TypeDefParticle::Choice(choice) => choice.to_complex_fragments(compiler),
            xsd::schema::TypeDefParticle::Sequence(sequence) => {
                sequence.to_complex_fragments(compiler)
            }
        }
    }
}

impl ToComplexFragments for xsd::schema::ExtensionType {
    /// This method expects all references to already be defined.
    fn to_complex_fragments<T: AsMut<ComplexTypeFragmentCompiler>>(
        &self,
        mut compiler: T,
    ) -> FragmentId {
        let mut compiler = compiler.as_mut();

        let base = ComplexTypeIdent::from(self.base.0 .0.clone());

        let content_fragment = self
            .particle
            .as_ref()
            .map(|content| content.to_complex_fragments(&mut compiler));

        let root_fragment = ComplexContentFragment::Extension(Extension {
            base,
            content_fragment,
        });

        compiler.push_fragment(ComplexTypeFragment::ComplexContent(root_fragment))
    }
}

impl ToComplexFragments for xsd::schema::ComplexRestrictionType {
    fn to_complex_fragments<T: AsMut<ComplexTypeFragmentCompiler>>(
        &self,
        mut compiler: T,
    ) -> FragmentId {
        let mut compiler = compiler.as_mut();

        let base = ComplexTypeIdent::from(self.base.0 .0.clone());

        let content_fragment = self
            .particle
            .as_ref()
            .map(|particle| particle.to_complex_fragments(&mut compiler));

        let root_fragment = ComplexContentFragment::Restriction(Restriction {
            base,
            content_fragment,
        });

        compiler.push_fragment(ComplexTypeFragment::ComplexContent(root_fragment))
    }
}

impl ToComplexFragments for xsd::schema::ComplexTypeModel {
    /// This method expects all references to already be defined.
    fn to_complex_fragments<T: AsMut<ComplexTypeFragmentCompiler>>(
        &self,
        mut compiler: T,
    ) -> FragmentId {
        let compiler = compiler.as_mut();

        match self {
            xsd::schema::ComplexTypeModel::SimpleContent(_simple_content) => {
                todo!()
            }
            xsd::schema::ComplexTypeModel::ComplexContent(complex_content) => {
                match &complex_content.content {
                    xsd::schema::ComplexContentContent::Extension(extension) => {
                        extension.to_complex_fragments(compiler)
                    }
                    xsd::schema::ComplexContentContent::Restriction(restriction) => {
                        restriction.to_complex_fragments(compiler)
                    }
                }
            }
        }
    }
}

impl ToComplexFragments for xsd::schema::TopLevelComplexType {
    fn to_complex_fragments<T: AsMut<ComplexTypeFragmentCompiler>>(
        &self,
        mut compiler: T,
    ) -> FragmentId {
        let compiler = compiler.as_mut();

        self.content.to_complex_fragments(compiler)
    }
}

impl ToComplexFragments for xsd::schema::LocalComplexType {
    fn to_complex_fragments<T: AsMut<ComplexTypeFragmentCompiler>>(
        &self,
        mut compiler: T,
    ) -> FragmentId {
        let compiler = compiler.as_mut();

        self.content.to_complex_fragments(compiler)
    }
}

impl ToComplexFragments for xsd::schema::NamedGroupTypeContent {
    fn to_complex_fragments<T: AsMut<ComplexTypeFragmentCompiler>>(
        &self,
        mut compiler: T,
    ) -> FragmentId {
        let compiler = compiler.as_mut();

        match self {
            xsd::schema::NamedGroupTypeContent::All(all_type) => {
                all_type.to_complex_fragments(compiler)
            }
            xsd::schema::NamedGroupTypeContent::Choice(choice_type) => {
                choice_type.to_complex_fragments(compiler)
            }
            xsd::schema::NamedGroupTypeContent::Sequence(sequence_type) => {
                sequence_type.to_complex_fragments(compiler)
            }
        }
    }
}

pub static ANY_TYPE_EXPANDED_NAME: LazyLock<ExpandedName<'static>> = LazyLock::new(|| {
    ExpandedName::new(
        LocalName::new("anyType").unwrap(),
        Some(XmlNamespace::XMLNS),
    )
});

#[cfg(test)]
mod tests {
    use xsd::schema::{
        Abstract, Base, ComplexContent, ComplexContentContent, ComplexRestrictionType,
        ComplexTypeModel, ExtensionType, GroupTypeContent, LocalElement, MaxOccurs, QName, Ref,
        SequenceType, TopLevelComplexType, Type,
    };

    use super::*;

    /// <xs:element name="complexContent" id="complexContent">
    ///     <xs:annotation>
    ///       <xs:documentation
    ///            source="../structures/structures.html#element-complexContent"/>
    ///     </xs:annotation>
    ///     <xs:complexType>
    ///       <xs:complexContent>
    ///         <xs:extension base="xs:annotated">
    ///           <xs:choice>
    ///             <xs:element name="restriction" type="xs:complexRestrictionType"/>
    ///             <xs:element name="extension" type="xs:extensionType"/>
    ///           </xs:choice>
    ///           <xs:attribute name="mixed" type="xs:boolean">
    ///             <xs:annotation>
    ///               <xs:documentation>
    ///        Overrides any setting on complexType parent.</xs:documentation>
    ///             </xs:annotation>
    ///           </xs:attribute>
    ///         </xs:extension>
    ///       </xs:complexContent>
    ///     </xs:complexType>
    ///   </xs:element>
    ///
    /// This should become a top level element with a reference to an anonymous complex type
    struct _Todo;

    /// <xs:complexType>
    ///   <xs:complexContent>
    ///     <xs:extension base="xs:annotated">
    ///       <xs:choice>
    ///         <xs:element name="restriction" type="xs:complexRestrictionType"/>
    ///         <xs:element name="extension" type="xs:extensionType"/>
    ///       </xs:choice>
    ///       <xs:attribute name="mixed" type="xs:boolean">
    ///         <xs:annotation>
    ///           <xs:documentation>
    ///             Overrides any setting on complexType parent.
    ///           </xs:documentation>
    ///         </xs:annotation>
    ///       </xs:attribute>
    ///     </xs:extension>
    ///   </xs:complexContent>
    /// </xs:complexType>
    struct _Todo2;

    // //Flatten nested choices
    // #[test]
    // fn flatten_nested_choices() {
    //     let mut fragments = ComplexTypeFragmentCollection::new(
    //         XmlNamespace::new_dangerous("http://localhost"),
    //         SimpleTypeFragmentCollection {},
    //     );

    //     let element1 = ComplexTypeFragment::Element(ElementTypeFragment::Reference {
    //         name: ExpandedName::new(
    //             LocalName::new("element1").unwrap(),
    //             Some(XmlNamespace::XMLNS),
    //         ),
    //         min_occurs: Some(MinOccurs(1)),
    //         max_occurs: None,
    //     });
    //     let element1 = fragments.push_fragment(element1);

    //     let element2 = ComplexTypeFragment::Element(ElementTypeFragment::Reference {
    //         name: ExpandedName::new(
    //             LocalName::new("element2").unwrap(),
    //             Some(XmlNamespace::XMLNS),
    //         ),
    //         min_occurs: Some(MinOccurs(1)),
    //         max_occurs: None,
    //     });
    //     let element2 = fragments.push_fragment(element2);

    //     let choice1 = ComplexTypeFragment::Choice {
    //         fragments: vec_deque![element1.clone()],
    //     };
    //     let choice1 = fragments.push_fragment(choice1);

    //     let choice2 = ComplexTypeFragment::Choice {
    //         fragments: vec_deque![choice1, element2.clone()],
    //     };
    //     let choice2 = fragments.push_fragment(choice2);

    //     FlattenNestedChoices.transform(&mut fragments);

    //     let expected = ComplexTypeFragment::Choice {
    //         fragments: vec_deque![element1, element2],
    //     };

    //     let actual = fragments.get_fragment(&choice2).unwrap();

    //     assert_eq!(*actual, expected);
    // }

    // // Flatten nested sequences
    // // This can only be done if the sequence does not by itself have a minOccurs or maxOccurs that would affect how the fragments are nested
    // #[test]
    // fn flatten_nested_sequences() {
    //     let mut fragments = ComplexTypeFragmentCollection::new(
    //         XmlNamespace::new_dangerous("http://localhost"),
    //         SimpleTypeFragmentCollection {},
    //     );

    //     let element1 = ComplexTypeFragment::Element(ElementTypeFragment::Reference {
    //         name: ExpandedName::new(
    //             LocalName::new("element1").unwrap(),
    //             Some(XmlNamespace::XMLNS),
    //         ),
    //         min_occurs: Some(MinOccurs(1)),
    //         max_occurs: None,
    //     });
    //     let element1 = fragments.push_fragment(element1);
    //     let element2 = ComplexTypeFragment::Element(ElementTypeFragment::Reference {
    //         name: ExpandedName::new(
    //             LocalName::new("element2").unwrap(),
    //             Some(XmlNamespace::XMLNS),
    //         ),
    //         min_occurs: Some(MinOccurs(1)),
    //         max_occurs: None,
    //     });
    //     let element2 = fragments.push_fragment(element2);
    //     let sequence1 = ComplexTypeFragment::Sequence {
    //         fragments: vec_deque![element1.clone()],
    //     };
    //     let sequence1 = fragments.push_fragment(sequence1);
    //     let sequence2 = ComplexTypeFragment::Sequence {
    //         fragments: vec_deque![sequence1, element2.clone()],
    //     };
    //     let sequence2 = fragments.push_fragment(sequence2);

    //     FlattenNestedSequences.transform(&mut fragments);

    //     let actual = fragments.get_fragment(&sequence2).unwrap();
    //     let expected = ComplexTypeFragment::Sequence {
    //         fragments: vec_deque![element1, element2],
    //     };

    //     assert_eq!(*actual, expected);
    // }

    // // Expand extensions
    // #[test]
    // fn expand_extensions() {
    //     let mut fragments = ComplexTypeFragmentCollection::new(
    //         XmlNamespace::new_dangerous("http://localhost"),
    //         SimpleTypeFragmentCollection {},
    //     );

    //     let element1 = ComplexTypeFragment::Element(ElementTypeFragment::Reference {
    //         name: ExpandedName::new(
    //             LocalName::new("element1").unwrap(),
    //             Some(XmlNamespace::XMLNS),
    //         ),
    //         min_occurs: Some(MinOccurs(1)),
    //         max_occurs: None,
    //     });

    //     let element1 = fragments.push_fragment(element1);

    //     let element2 = ComplexTypeFragment::Element(ElementTypeFragment::Reference {
    //         name: ExpandedName::new(
    //             LocalName::new("element2").unwrap(),
    //             Some(XmlNamespace::XMLNS),
    //         ),
    //         min_occurs: Some(MinOccurs(1)),
    //         max_occurs: None,
    //     });

    //     let element2 = fragments.push_fragment(element2);

    //     let base = ComplexTypeFragment::ComplexContent(ComplexContentFragment::Restriction {
    //         base: TypeIdent(TypeIdent::External(any_type_expanded_name())),
    //         content_fragment: vec_deque![element1.clone()],
    //     });

    //     let base_type_ident = TypeIdent(TypeIdent::Local(LocalTypeIdent::Named(
    //         LocalName::new("base").unwrap(),
    //     )));
    //     let base = fragments.push_fragment(base);
    //     fragments
    //         .fragment_names
    //         .insert(base_type_ident.clone(), base);

    //     let derivative = ComplexTypeFragment::ComplexContent(ComplexContentFragment::Extension {
    //         base: base_type_ident,
    //         content_fragment: vec_deque![element2.clone()],
    //     });

    //     let derivative = fragments.push_fragment(derivative);

    //     ExpandExtensions.transform(&mut fragments);

    //     let expected = ComplexTypeFragment::ComplexContent(ComplexContentFragment::Restriction {
    //         base: TypeIdent(TypeIdent::External(any_type_expanded_name())),
    //         content_fragment: vec_deque![element1, element2],
    //     });

    //     let actual = fragments.get_fragment(&derivative).unwrap();

    //     assert_eq!(*actual, expected);
    // }

    /// # Chain of extensions and restrictions test
    ///
    /// PS: This is not a direct copy from XMLSchema. "annotated" is not a restriction of "anyType" but an extension of "openAttrs",
    /// however for this example we've deleted all attributes and changed the base to "anyType" to make it a restriction.
    /// Groups ("identityConstraint") have also been removed.
    ///
    /// ## "annotated"
    /// ```xml
    /// <xs:complexType name="annotated">
    ///     <xs:complexContent>
    ///         <xs:restriction base="xs:anyType">
    ///             <xs:sequence>
    ///                 <xs:element ref="xs:annotation" minOccurs="0"/>
    ///             </xs:sequence>
    ///         </xs:restriction>
    ///     </xs:complexContent>
    /// </xs:complexType>
    /// ```
    ///
    /// ## "element"
    /// ```xml
    /// <xs:complexType name="element" abstract="true">
    ///     <xs:complexContent>
    ///         <xs:extension base="xs:annotated">
    ///             <xs:sequence>
    ///                 <xs:choice minOccurs="0">
    ///                     <xs:element name="simpleType" type="xs:localSimpleType"/>
    ///                     <xs:element name="complexType" type="xs:localComplexType"/>
    ///                 </xs:choice>
    ///                 <xs:element name="alternative" type="xs:altType"
    ///                         minOccurs="0" maxOccurs="unbounded"/>
    ///             </xs:sequence>
    ///         </xs:extension>
    ///     </xs:complexContent>
    /// </xs:complexType>
    /// ```
    ///
    /// ## "topLevelElement"
    /// ```xml
    /// <xs:complexType name="topLevelElement">
    ///     <xs:complexContent>
    ///         <xs:restriction base="xs:element">
    ///             <xs:sequence>
    ///                 <xs:element ref="xs:annotation" minOccurs="0"/>
    ///                 <xs:choice minOccurs="0">
    ///                     <xs:element name="simpleType" type="xs:localSimpleType"/>
    ///                     <xs:element name="complexType" type="xs:localComplexType"/>
    ///                 </xs:choice>
    ///                 <xs:element name="alternative" type="xs:altType"
    ///                     minOccurs="0" maxOccurs="unbounded"/>
    ///             </xs:sequence>
    ///         </xs:restriction>
    ///     </xs:complexContent>
    /// </xs:complexType>
    /// ```
    // #[test]
    // fn test_expand_extensions_restrictions() {
    //     let mut fragments = ComplexTypeFragmentCollection::new(
    //         XmlNamespace::new_dangerous("http://localhost"),
    //         SimpleTypeFragmentCollection {},
    //     );

    //     let annotation_element_name = ExpandedName::new(
    //         LocalName::new("annotation").unwrap(),
    //         Some(XmlNamespace::XMLNS),
    //     );
    //     let annotation_element = ComplexTypeFragment::Element(ElementTypeFragment::Reference {
    //         name: annotation_element_name.clone(),
    //         min_occurs: Some(MinOccurs(0)),
    //         max_occurs: None,
    //     });

    //     let annotation_element = fragments.push_fragment(annotation_element);

    //     let annotated_name =
    //         TypeIdent(LocalTypeIdent::Named(LocalName::new("annotated").unwrap()).into());
    //     let annotated = ComplexTypeFragment::ComplexContent(ComplexContentFragment::Restriction {
    //         base: TypeIdent(TypeIdent::External(any_type_expanded_name())),
    //         content_fragment: vec_deque![annotation_element,],
    //     });

    //     let annotated = fragments.push_fragment(annotated);
    //     fragments.fragment_names.insert(annotated_name, annotated);

    //     let simple_type_element_type_name =
    //         TypeIdent(TypeIdent::External(ExpandedName::new(
    //             LocalName::new("localSimpleType").unwrap(),
    //             Some(XmlNamespace::XMLNS),
    //         )));
    //     let simple_type_element_type =
    //         ComplexTypeFragment::ComplexContent(ComplexContentFragment::Restriction {
    //             base: TypeIdent(TypeIdent::External(any_type_expanded_name())),
    //             content_fragment: vec_deque![],
    //         });

    //     let simple_type_element_type = fragments.push_fragment(simple_type_element_type);
    //     fragments
    //         .fragment_names
    //         .insert(simple_type_element_type_name, simple_type_element_type);

    //     let simple_type_element_type_name =
    //         TypeIdent(TypeIdent::External(ExpandedName::new(
    //             LocalName::new("localComplexType").unwrap(),
    //             Some(XmlNamespace::XMLNS),
    //         )));
    //     let simple_type_element_type =
    //         ComplexTypeFragment::ComplexContent(ComplexContentFragment::Restriction {
    //             base: TypeIdent(TypeIdent::External(any_type_expanded_name())),
    //             content_fragment: vec_deque![],
    //         });

    //     let simple_type_element_type = fragments.push_fragment(simple_type_element_type);
    //     fragments
    //         .fragment_names
    //         .insert(simple_type_element_type_name, simple_type_element_type);

    //     let simple_type_element_type_name =
    //         TypeIdent(TypeIdent::External(ExpandedName::new(
    //             LocalName::new("altType").unwrap(),
    //             Some(XmlNamespace::XMLNS),
    //         )));
    //     let simple_type_element_type =
    //         ComplexTypeFragment::ComplexContent(ComplexContentFragment::Restriction {
    //             base: TypeIdent(TypeIdent::External(any_type_expanded_name())),
    //             content_fragment: vec_deque![],
    //         });

    //     let simple_type_element_type = fragments.push_fragment(simple_type_element_type);
    //     fragments
    //         .fragment_names
    //         .insert(simple_type_element_type_name, simple_type_element_type);

    //     let element = ComplexTypeFragment::ComplexContent(ComplexContentFragment::Extension {
    //         base: TypeIdent(TypeIdent::External(annotation_element_name)),
    //         content_fragment: vec_deque![],
    //     });
    // }

    #[test]
    fn convert_annotated_to_fragments() {
        let namespace = XmlNamespace::new_dangerous("http://localhost");

        let simple_type_compiler = SimpleTypeFragmentCompiler::new(namespace.clone());
        let mut fragment_compiler =
            ComplexTypeFragmentCompiler::new(namespace.clone(), simple_type_compiler);

        let id = TopLevelComplexType {
            id: None,
            name: LocalName::new_dangerous("annotated"),
            mixed: None,
            abstract_: None,
            final_: None,
            block: None,
            default_attributes_apply: None,
            annotation: None,
            content: ComplexTypeModel::ComplexContent(ComplexContent {
                id: None,
                mixed: None,
                annotation: None,
                content: ComplexContentContent::Restriction(Box::new(ComplexRestrictionType {
                    id: None,
                    base: Base(QName(ANY_TYPE_EXPANDED_NAME.clone())),
                    annotation: None,
                    simple_type: None,
                    content: None,
                    particle: Some(TypeDefParticle::Sequence(SequenceType {
                        id: None,
                        name: None,
                        ref_: None,
                        min_occurs: None,
                        max_occurs: None,
                        content: vec![GroupTypeContent::Element(Box::new(LocalElement {
                            ref_: Some(Ref(QName(ExpandedName::new(
                                LocalName::new_dangerous("annotation"),
                                Some(XmlNamespace::XMLNS),
                            )))),
                            id: None,
                            name: None,
                            type_: None,
                            min_occurs: Some(MinOccurs(0)),
                            max_occurs: None,
                            default: None,
                            fixed: None,
                            nillable: None,
                            block: None,
                            form: None,
                            target_namespace: None,
                            annotation: None,
                            type_choice: None,
                            alternatives: Vec::new(),
                            identity_constraints: Vec::new(),
                        }))],
                    })),
                })),
            }),
        }
        .to_complex_fragments(&mut fragment_compiler);

        assert_eq!(id, FragmentId(namespace, FragmentIdx(2)));
        assert_eq!(fragment_compiler.fragments.len(), 3);

        assert!(matches!(
            fragment_compiler.fragments[&FragmentIdx(0)],
            ComplexTypeFragment::Element(_)
        ));
        assert!(matches!(
            fragment_compiler.fragments[&FragmentIdx(1)],
            ComplexTypeFragment::Sequence { .. }
        ));
        assert!(matches!(
            fragment_compiler.fragments[&FragmentIdx(2)],
            ComplexTypeFragment::ComplexContent(_)
        ));

        println!("{:#?}", fragment_compiler);
    }

    #[test]
    fn convert_element_to_fragments() {
        let namespace = XmlNamespace::new_dangerous("http://localhost");

        let simple_type_compiler = SimpleTypeFragmentCompiler::new(namespace.clone());
        let mut fragment_compiler =
            ComplexTypeFragmentCompiler::new(namespace.clone(), simple_type_compiler);

        let annotated_name = LocalName::new_dangerous("annotated");
        let annotated_expanded_name = ExpandedName::new(
            annotated_name.clone(),
            Some(fragment_compiler.namespace.clone()),
        );

        let annotated = TopLevelComplexType {
            id: None,
            name: annotated_name,
            mixed: None,
            abstract_: None,
            final_: None,
            block: None,
            default_attributes_apply: None,
            annotation: None,
            content: ComplexTypeModel::ComplexContent(ComplexContent {
                id: None,
                mixed: None,
                annotation: None,
                content: ComplexContentContent::Restriction(Box::new(ComplexRestrictionType {
                    id: None,
                    base: Base(QName(ANY_TYPE_EXPANDED_NAME.clone())),
                    annotation: None,
                    simple_type: None,
                    content: None,
                    particle: Some(TypeDefParticle::Sequence(SequenceType {
                        id: None,
                        name: None,
                        ref_: None,
                        min_occurs: None,
                        max_occurs: None,
                        content: vec![GroupTypeContent::Element(Box::new(LocalElement {
                            ref_: Some(Ref(QName(ExpandedName::new(
                                LocalName::new_dangerous("annotation"),
                                Some(XmlNamespace::XMLNS),
                            )))),
                            id: None,
                            name: None,
                            type_: None,
                            min_occurs: Some(MinOccurs(0)),
                            max_occurs: None,
                            default: None,
                            fixed: None,
                            nillable: None,
                            block: None,
                            form: None,
                            target_namespace: None,
                            annotation: None,
                            type_choice: None,
                            alternatives: Vec::new(),
                            identity_constraints: Vec::new(),
                        }))],
                    })),
                })),
            }),
        }
        .to_complex_fragments(&mut fragment_compiler);

        assert_eq!(fragment_compiler.fragments.len(), 3);

        assert_eq!(annotated, FragmentId(namespace.clone(), FragmentIdx(2)));

        assert!(matches!(
            fragment_compiler.fragments[&FragmentIdx(0)],
            ComplexTypeFragment::Element(_)
        ));
        assert!(matches!(
            fragment_compiler.fragments[&FragmentIdx(1)],
            ComplexTypeFragment::Sequence { .. }
        ));
        assert!(matches!(
            fragment_compiler.fragments[&FragmentIdx(2)],
            ComplexTypeFragment::ComplexContent(_)
        ));

        // ## "element"
        // ```xml
        // <xs:complexType name="element" abstract="true">
        //     <xs:complexContent>
        //         <xs:extension base="xs:annotated">
        //             <xs:sequence>
        //                 <xs:choice minOccurs="0">
        //                     <xs:element name="simpleType" type="xs:localSimpleType"/>
        //                     <xs:element name="complexType" type="xs:localComplexType"/>
        //                 </xs:choice>
        //                 <xs:element name="alternative" type="xs:altType"
        //                         minOccurs="0" maxOccurs="unbounded"/>
        //             </xs:sequence>
        //         </xs:extension>
        //     </xs:complexContent>
        // </xs:complexType>
        // ```
        let element_id = TopLevelComplexType {
            id: None,
            name: LocalName::new_dangerous("element"),
            mixed: None,
            abstract_: Some(Abstract(true)),
            final_: None,
            block: None,
            default_attributes_apply: None,
            annotation: None,
            content: ComplexTypeModel::ComplexContent(ComplexContent {
                id: None,
                mixed: None,
                annotation: None,
                content: ComplexContentContent::Extension(Box::new(ExtensionType {
                    id: None,
                    base: Base(QName(annotated_expanded_name)),
                    annotation: None,
                    content: None,
                    particle: Some(TypeDefParticle::Sequence(SequenceType {
                        id: None,
                        name: None,
                        ref_: None,
                        min_occurs: None,
                        max_occurs: None,
                        content: vec![
                            GroupTypeContent::Choice(Box::new(ChoiceType {
                                id: None,
                                name: None,
                                ref_: None,
                                min_occurs: Some(MinOccurs(0)),
                                max_occurs: None,
                                content: vec![
                                    GroupTypeContent::Element(Box::new(LocalElement {
                                        id: None,
                                        name: Some(xsd::schema::Name(LocalName::new_dangerous(
                                            "simpleType",
                                        ))),
                                        ref_: None,
                                        type_: Some(Type(QName(ExpandedName::new(
                                            LocalName::new_dangerous("localSimpleType"),
                                            Some(XmlNamespace::XMLNS),
                                        )))),
                                        min_occurs: Some(MinOccurs(0)),
                                        max_occurs: None,
                                        default: None,
                                        fixed: None,
                                        nillable: None,
                                        block: None,
                                        form: None,
                                        target_namespace: None,
                                        annotation: None,
                                        type_choice: None,
                                        alternatives: Vec::new(),
                                        identity_constraints: Vec::new(),
                                    })),
                                    GroupTypeContent::Element(Box::new(LocalElement {
                                        id: None,
                                        name: Some(xsd::schema::Name(LocalName::new_dangerous(
                                            "complexType",
                                        ))),
                                        ref_: None,
                                        type_: Some(Type(QName(ExpandedName::new(
                                            LocalName::new_dangerous("localComplexType"),
                                            Some(XmlNamespace::XMLNS),
                                        )))),
                                        min_occurs: Some(MinOccurs(0)),
                                        max_occurs: None,
                                        default: None,
                                        fixed: None,
                                        nillable: None,
                                        block: None,
                                        form: None,
                                        target_namespace: None,
                                        annotation: None,
                                        type_choice: None,
                                        alternatives: Vec::new(),
                                        identity_constraints: Vec::new(),
                                    })),
                                ],
                            })),
                            GroupTypeContent::Element(Box::new(LocalElement {
                                id: None,
                                name: Some(xsd::schema::Name(LocalName::new_dangerous(
                                    "complexType",
                                ))),
                                ref_: None,
                                type_: Some(Type(QName(ExpandedName::new(
                                    LocalName::new_dangerous("altType"),
                                    Some(XmlNamespace::XMLNS),
                                )))),
                                min_occurs: Some(MinOccurs(0)),
                                max_occurs: Some(MaxOccurs(MaxOccursValue::Unbounded)),
                                default: None,
                                fixed: None,
                                nillable: None,
                                block: None,
                                form: None,
                                target_namespace: None,
                                annotation: None,
                                type_choice: None,
                                alternatives: Vec::new(),
                                identity_constraints: Vec::new(),
                            })),
                        ],
                    })),
                })),
            }),
        }
        .to_complex_fragments(&mut fragment_compiler);

        assert_eq!(fragment_compiler.fragments.len(), 9);

        assert_eq!(element_id, FragmentId(namespace.clone(), FragmentIdx(8)));

        assert!(matches!(
            fragment_compiler.fragments[&FragmentIdx(3)],
            ComplexTypeFragment::Element(_)
        ));
        assert!(matches!(
            fragment_compiler.fragments[&FragmentIdx(4)],
            ComplexTypeFragment::Element(_)
        ));
        assert!(matches!(
            fragment_compiler.fragments[&FragmentIdx(5)],
            ComplexTypeFragment::Choice { .. }
        ));
        assert!(matches!(
            fragment_compiler.fragments[&FragmentIdx(6)],
            ComplexTypeFragment::Element(_)
        ));
        assert!(matches!(
            fragment_compiler.fragments[&FragmentIdx(7)],
            ComplexTypeFragment::Sequence { .. }
        ));
        assert!(matches!(
            fragment_compiler.fragments[&FragmentIdx(8)],
            ComplexTypeFragment::ComplexContent(_)
        ));

        println!("{:#?}", fragment_compiler);
    }
}
