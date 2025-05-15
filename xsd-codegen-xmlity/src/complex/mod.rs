use crate::{
    misc::{GeneratedFragment, TypeReference},
    simple as simple_gen,
    types::{ItemOrder, Items, TypeChoice, TypeItem, TypeRecord},
    NamedTypeClass, Result,
};

use std::collections::HashMap;

use quote::format_ident;
use syn::{parse_quote, Ident, Item};
use xmlity::{ExpandedName, XmlNamespace};
use xsd::schema::MaxOccursValue;
use xsd_type_compiler::{
    complex::{
        self, AllTypeFragment, AttributeTypeFragment, ChoiceTypeFragment, ComplexContentFragment,
        ComplexTypeFragment, ElementTypeFragment, GroupRefTypeFragment, LocalElementTypeFragment,
        ReferenceElementTypeFragment, SequenceTypeFragment, SimpleContentFragment,
        SimpleTypeTypeFragment, ANY_TYPE_EXPANDED_NAME,
    },
    ComplexTypeIdent,
};

pub struct ComplexTypeFragmentGenerator<'a> {
    pub context: &'a xsd_type_compiler::complex::ComplexTypeFragmentCompiler,
    pub bound_namespaces: &'a HashMap<XmlNamespace<'static>, Ident>,
    pub simple_generator: &'a simple_gen::SimpleGeneratedNamespaceTypes,
}

impl ComplexTypeFragmentGenerator<'_> {
    pub fn generate(&self) -> Result<Vec<Item>> {
        Ok(vec![])
    }

    pub fn get_named_type(&self, name: &ExpandedName<'_>) -> Result<Option<syn::TypePath>> {
        todo!()
    }
}

pub trait Context: crate::simple::Context {
    fn get_complex_fragment(
        &self,
        fragment_id: &complex::FragmentId,
    ) -> Result<Option<GeneratedFragment>>;

    fn get_complex_type_ident(
        &self,
        fragment_id: &ComplexTypeIdent,
    ) -> Result<Option<GeneratedFragment>> {
        match fragment_id {
            ComplexTypeIdent::Named(name) => {
                self.get_named_type(name, NamedTypeClass::Type)
                    .map(|type_| {
                        type_.map(|type_| {
                            let type_ = TypeReference::new_static(type_);

                            GeneratedFragment {
                                type_: crate::types::TypeRecord::Item(TypeItem::new(type_)),
                            }
                        })
                    })
            }
            ComplexTypeIdent::Anonymous(fr) => self.get_complex_fragment(fr),
        }
    }
}

pub trait ComplexTypeToRustType {
    fn generate_complex_rust_types<C: Context>(&self, context: &C) -> Result<GeneratedFragment>;
}

impl ComplexTypeToRustType for ComplexTypeFragment {
    fn generate_complex_rust_types<C: Context>(&self, context: &C) -> Result<GeneratedFragment> {
        use ComplexTypeFragment as F;
        match self {
            F::SimpleContent(simple_content) => simple_content.generate_complex_rust_types(context),
            F::ComplexContent(complex_content) => {
                complex_content.generate_complex_rust_types(context)
            }
            F::GroupRef(group_ref) => group_ref.generate_complex_rust_types(context),
            F::All(all) => all.generate_complex_rust_types(context),
            F::Choice(choice) => choice.generate_complex_rust_types(context),
            F::Sequence(sequence) => sequence.generate_complex_rust_types(context),
            F::Element(element_type_fragment) => {
                element_type_fragment.generate_complex_rust_types(context)
            }
            F::Attribute(attribute) => attribute.generate_complex_rust_types(context),
            F::SimpleType(simple_type) => simple_type.generate_complex_rust_types(context),
        }
    }
}

impl ComplexTypeToRustType for SimpleContentFragment {
    fn generate_complex_rust_types<C: Context>(&self, context: &C) -> Result<GeneratedFragment> {
        todo!()
    }
}

impl ComplexTypeToRustType for ComplexContentFragment {
    fn generate_complex_rust_types<C: Context>(&self, context: &C) -> Result<GeneratedFragment> {
        let restriction = match self {
            ComplexContentFragment::Extension(_extension) => return Err(panic!("A")),
            ComplexContentFragment::Restriction(restriction) => restriction,
        };

        if restriction.base != ComplexTypeIdent::Named(ANY_TYPE_EXPANDED_NAME.clone()) {
            return Err(panic!("A"));
        }

        let Some(content_fragment) = &restriction.content_fragment else {
            return Err(panic!("B"));
        };

        let GeneratedFragment { type_ } = context
            .get_complex_fragment(content_fragment)?
            .ok_or_else(|| panic!("C"))?;

        // let fields = item.into_fields(format_ident!("content"));

        Ok(GeneratedFragment { type_ })
    }
}

impl ComplexTypeToRustType for GroupRefTypeFragment {
    fn generate_complex_rust_types<C: Context>(&self, context: &C) -> Result<GeneratedFragment> {
        //TODO: Should have been resolved
        Err(panic!("GroupRefTypeFragment"))
    }
}

impl ComplexTypeToRustType for AllTypeFragment {
    fn generate_complex_rust_types<C: Context>(&self, context: &C) -> Result<GeneratedFragment> {
        let items = self
            .fragments
            .iter()
            .map(|fragment| {
                context
                    .get_complex_fragment(fragment)?
                    .ok_or_else(|| panic!("{fragment:?}"))
            })
            .collect::<Result<Vec<_>>>()?;

        let items = items
            .into_iter()
            .enumerate()
            .map(|(i, a)| (format_ident!("item{i}"), a.type_));

        let type_ = TypeRecord::Items(Items {
            order: ItemOrder::None,
            items: items.collect(),
        });

        Ok(GeneratedFragment { type_ })
    }
}

impl ComplexTypeToRustType for ChoiceTypeFragment {
    fn generate_complex_rust_types<C: Context>(&self, context: &C) -> Result<GeneratedFragment> {
        let items = self
            .fragments
            .iter()
            .map(|fragment| {
                context
                    .get_complex_fragment(fragment)?
                    .ok_or_else(|| panic!("{fragment:?}"))
            })
            .collect::<Result<Vec<_>>>()?;

        let variants = items
            .into_iter()
            .enumerate()
            .map(|(i, fragment)| (format_ident!("Variant{}", i), fragment.type_))
            .collect::<Vec<_>>();

        let type_ = TypeRecord::Choice(TypeChoice { variants });

        Ok(GeneratedFragment { type_ })
    }
}

impl ComplexTypeToRustType for SequenceTypeFragment {
    fn generate_complex_rust_types<C: Context>(&self, context: &C) -> Result<GeneratedFragment> {
        // Struct with strict order
        let items = self
            .fragments
            .iter()
            .map(|fragment| {
                context
                    .get_complex_fragment(fragment)?
                    .ok_or_else(|| panic!("{fragment:?}"))
            })
            .collect::<Result<Vec<_>>>()?;

        let items = items
            .into_iter()
            .enumerate()
            .map(|(i, a)| (format_ident!("item{i}"), a.type_));

        let type_ = TypeRecord::Items(Items {
            order: ItemOrder::Strict,
            items: items.collect(),
        });

        Ok(GeneratedFragment { type_ })
    }
}

impl ComplexTypeToRustType for ElementTypeFragment {
    fn generate_complex_rust_types<C: Context>(&self, context: &C) -> Result<GeneratedFragment> {
        match self {
            ElementTypeFragment::Local(local_element_type_fragment) => {
                local_element_type_fragment.generate_complex_rust_types(context)
            }
            ElementTypeFragment::Reference(reference_element_type_fragment) => {
                reference_element_type_fragment.generate_complex_rust_types(context)
            }
        }
    }
}

fn min_max_occurs_type(
    min_occurs: usize,
    max_occurs: MaxOccursValue,
    type_: TypeReference<'_>,
) -> TypeReference<'_> {
    match (min_occurs, max_occurs) {
        (1, MaxOccursValue::Bounded(1)) => type_,
        (0, MaxOccursValue::Bounded(1)) => TypeReference::new(|path| {
            let type_ = type_.into_type(path);
            parse_quote!(Option<#type_>)
        }),
        (_, _) => TypeReference::new(|path| {
            let type_ = type_.into_type(path);
            parse_quote!(Vec<#type_>)
        }),
    }
}

impl ComplexTypeToRustType for LocalElementTypeFragment {
    fn generate_complex_rust_types<C: Context>(&self, context: &C) -> Result<GeneratedFragment> {
        let generated_fragment = context.get_complex_type_ident(&self.type_)?.unwrap();

        let name = ExpandedName::new(
            self.name.clone(),
            Some(context.namespace().clone().into_owned()),
        );

        let mut element_record = match generated_fragment.type_ {
            TypeRecord::Items(items) => todo!(),
            TypeRecord::Element(element_record) => element_record,
            TypeRecord::Choice(type_choice) => todo!(),
            TypeRecord::Item(type_reference) => todo!(),
        };

        element_record.name = name;

        let min_occurs = self.min_occurs.as_ref().map(|a| a.0).unwrap_or(1);
        let max_occurs = self.max_occurs.unwrap_or(MaxOccursValue::Bounded(1));

        let (type_, struct_item, mut items) =
            element_record.into_struct(&format_ident!("Todo123"), None);
        items.push(Item::Struct(struct_item));

        let type_ = min_max_occurs_type(min_occurs, max_occurs, type_);

        Ok(GeneratedFragment {
            type_: TypeRecord::Item(TypeItem { items, type_ }),
        })
    }
}

impl ComplexTypeToRustType for ReferenceElementTypeFragment {
    fn generate_complex_rust_types<C: Context>(&self, context: &C) -> Result<GeneratedFragment> {
        let element_type = context
            .get_named_type(&self.name, NamedTypeClass::Element)?
            .ok_or_else(|| todo!("Element not found: {}", self.name.to_string()))?;

        let min_occurs = self.min_occurs.as_ref().map(|a| a.0).unwrap_or(1);
        let max_occurs = self.max_occurs.unwrap_or(MaxOccursValue::Bounded(1));

        let type_ = min_max_occurs_type(
            min_occurs,
            max_occurs,
            TypeReference::new_static(element_type),
        );

        Ok(GeneratedFragment {
            type_: TypeRecord::Item(TypeItem::new(type_)),
        })
    }
}

impl ComplexTypeToRustType for AttributeTypeFragment {
    fn generate_complex_rust_types<C: Context>(&self, context: &C) -> Result<GeneratedFragment> {
        todo!()
    }
}

impl ComplexTypeToRustType for SimpleTypeTypeFragment {
    fn generate_complex_rust_types<C: Context>(&self, context: &C) -> Result<GeneratedFragment> {
        match &self.type_ {
            xsd_type_compiler::SimpleTypeIdent::Named(expanded_name) => {
                let type_ = context
                    .get_named_type(expanded_name, crate::NamedTypeClass::Type)?
                    .ok_or_else(|| todo!())?;
                let type_ = TypeReference::new_static(type_);

                Ok(GeneratedFragment {
                    type_: TypeRecord::Item(TypeItem::new(type_)),
                })
            }
            xsd_type_compiler::SimpleTypeIdent::Anonymous(fragment_id) => {
                let a = context
                    .get_simple_fragment(fragment_id)?
                    .ok_or_else(|| todo!())?;

                Ok(a)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use quote::format_ident;
    use syn::parse_quote;
    use xmlity::{ExpandedName, LocalName, XmlNamespace};
    use xsd::schema::{
        Abstract, Base, ChoiceType, ComplexContent, ComplexContentContent, ComplexRestrictionType,
        ComplexTypeModel, ExtensionType, GroupTypeContent, LocalElement, MaxOccurs, MaxOccursValue,
        MinOccurs, QName, Ref, SequenceType, TopLevelComplexType, Type, TypeDefParticle,
    };
    use xsd_type_compiler::{
        complex::{transformers::ExpandBasedFragments, ANY_TYPE_EXPANDED_NAME},
        transformers::XmlnsContextTransformer,
        CompiledNamespace, XmlnsContext,
    };

    #[test]
    fn convert_annotated_to_fragments() {
        let namespace = XmlNamespace::new_dangerous("http://localhost");
        let mut compiled_namespace = CompiledNamespace::new(namespace.clone());

        let type_ = TopLevelComplexType {
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
        };

        let top_level_complex_type = compiled_namespace
            .add_top_level_complex_type(&type_)
            .into_owned();

        let context = XmlnsContext {
            namespaces: map_macro::hash_map! {
                namespace => compiled_namespace,
            },
        };

        let mut generator = crate::Generator::new(&context);

        generator
            .bound_namespaces
            .insert(XmlNamespace::XMLNS, format_ident!("xmlns"));

        let (_type_, items) = generator
            .generate_top_level_type(&top_level_complex_type)
            .unwrap();

        let file: syn::File = parse_quote! {
            #(#items)*

        };

        let res = prettyplease::unparse(&file);

        println!("{res}");
    }
    #[test]
    fn convert_element_to_fragments() {
        let namespace = XmlNamespace::new_dangerous("http://localhost");
        let mut compiled_namespace = CompiledNamespace::new(namespace.clone());

        let annotated = TopLevelComplexType {
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
        };

        let annotated = compiled_namespace
            .add_top_level_complex_type(&annotated)
            .into_owned();

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
        let element = TopLevelComplexType {
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
                    base: Base(QName(annotated.clone())),
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
        };

        let _element = compiled_namespace
            .add_top_level_complex_type(&element)
            .into_owned();

        let mut xmlns_context = XmlnsContext {
            namespaces: map_macro::hash_map! {
                namespace.clone() => compiled_namespace,
            },
        };

        xmlns_context.transform(&namespace, ExpandBasedFragments);

        let mut generator = crate::Generator::new(&xmlns_context);

        generator
            .bound_namespaces
            .insert(XmlNamespace::XMLNS, format_ident!("xmlns"));

        let items = generator.generate_namespace(&namespace).unwrap();

        let file: syn::File = parse_quote! {
            #(#items)*

        };

        let res = prettyplease::unparse(&file);

        println!("{res}");
    }
}
