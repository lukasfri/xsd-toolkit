use crate::complex;

use crate::complex::AllTypeFragment;
use crate::complex::ChoiceTypeFragment;
use crate::complex::ComplexContentFragment;
use crate::complex::ComplexTypeFragment;
use crate::complex::FragmentId as ComplexFragmentId;
use crate::complex::SequenceTypeFragment;
use crate::complex::ANY_TYPE_EXPANDED_NAME;
use crate::transformers::Context;
use crate::transformers::XmlnsContextTransformer;
use crate::ComplexTypeIdent;

/// Expands restriction and extension fragments to their base fragments, with the modifications applied.
pub struct ExpandBasedFragments;

trait Restrictable {
    fn apply_restriction(&mut self, child: &Self, ctx: &mut Context<'_>);
}

impl Restrictable for SequenceTypeFragment {
    fn apply_restriction(&mut self, child: &Self, ctx: &mut Context<'_>) {
        self.fragments.iter_mut().for_each(|a| todo!());
    }
}

impl Restrictable for ChoiceTypeFragment {
    fn apply_restriction(&mut self, child: &Self, ctx: &mut Context<'_>) {
        self.fragments.iter_mut().for_each(|a| todo!());
    }
}

impl Restrictable for AllTypeFragment {
    fn apply_restriction(&mut self, child: &Self, ctx: &mut Context<'_>) {
        self.fragments.iter_mut().for_each(|a| todo!());
    }
}

trait Extendable {
    fn apply_extension(&mut self, child: &Self, ctx: &mut Context<'_>);
}

impl Extendable for SequenceTypeFragment {
    fn apply_extension(&mut self, child: &Self, ctx: &mut Context<'_>) {
        self.fragments.iter_mut().for_each(|a| todo!());
    }
}

impl Extendable for ChoiceTypeFragment {
    fn apply_extension(&mut self, child: &Self, ctx: &mut Context<'_>) {
        self.fragments.iter_mut().for_each(|a| todo!());
    }
}

impl Extendable for AllTypeFragment {
    fn apply_extension(&mut self, child: &Self, ctx: &mut Context<'_>) {
        self.fragments.iter_mut().for_each(|a| todo!());
    }
}

impl ExpandBasedFragments {
    fn expand_restriction(
        ctx: &mut Context<'_>,
        fragment_idx: &ComplexFragmentId,
        base: &ComplexTypeIdent,
        child_content_fragment_id: Option<&ComplexFragmentId>,
    ) {
        let base_fragment = ctx.get_complex_fragment_from_ident(base).unwrap();

        let mut transformed_fragment = base_fragment.clone();

        let ComplexTypeFragment::ComplexContent(root) = &mut transformed_fragment else {
            panic!("Expected base fragment to be a root");
        };

        let restriction = match root {
            ComplexContentFragment::Extension(_) => {
                panic!("This shouldn't be possible. Any parent extension should've been turned into a restriction.")
            }
            ComplexContentFragment::Restriction(restriction) => restriction,
        };
        let complex::Restriction {
            base,
            content_fragment: content_fragment_id,
        } = restriction;

        debug_assert!(*base == ComplexTypeIdent::Named(ANY_TYPE_EXPANDED_NAME.clone()));

        let content_fragment_id = content_fragment_id.as_mut().unwrap();
        let child_content_fragment_id = child_content_fragment_id.unwrap();

        let mut content_fragment = ctx
            .get_complex_fragment(content_fragment_id)
            .unwrap()
            .clone();
        let child_content_fragment = ctx
            .get_complex_fragment(child_content_fragment_id)
            .unwrap()
            .clone();

        match (&mut content_fragment, &child_content_fragment) {
            (ComplexTypeFragment::Sequence(seq), ComplexTypeFragment::Sequence(child_seq)) => {
                seq.apply_restriction(child_seq, ctx);
            }
            (ComplexTypeFragment::All(all), ComplexTypeFragment::All(child_all)) => {
                all.apply_restriction(child_all, ctx);
            }
            (ComplexTypeFragment::Choice(choice), ComplexTypeFragment::Choice(child_choice)) => {
                choice.apply_restriction(child_choice, ctx);
            }
            _ => {
                panic!("Expected both fragments to be the same type and of sequence, all or choice")
            }
        }

        *ctx.get_complex_fragment_mut(content_fragment_id).unwrap() = content_fragment;

        *ctx.get_complex_fragment_mut(fragment_idx).unwrap() = transformed_fragment;
    }

    fn expand_extension(
        ctx: &mut Context<'_>,
        fragment_id: &ComplexFragmentId,
        base: &ComplexTypeIdent,
        child_content_fragment_id: Option<&ComplexFragmentId>,
    ) {
        let base_fragment = ctx.get_complex_fragment_from_ident(base).unwrap();

        let mut transformed_fragment = base_fragment.clone();

        let ComplexTypeFragment::ComplexContent(root) = &mut transformed_fragment else {
            panic!("Expected base fragment to be a root");
        };

        let restriction = match root {
            ComplexContentFragment::Extension(_) => {
                panic!("This shouldn't be possible. Any parent extension should've been turned into a restriction.")
            }
            ComplexContentFragment::Restriction(restriction) => restriction,
        };
        let complex::Restriction {
            base,
            content_fragment: content_fragment_id,
        } = restriction;

        debug_assert!(*base == ComplexTypeIdent::Named(ANY_TYPE_EXPANDED_NAME.clone()));

        let content_fragment_id = content_fragment_id.as_mut().unwrap();
        let child_content_fragment_id = child_content_fragment_id.unwrap();

        let mut content_fragment = ctx
            .get_complex_fragment(content_fragment_id)
            .unwrap()
            .clone();
        let child_content_fragment = ctx
            .get_complex_fragment(child_content_fragment_id)
            .unwrap()
            .clone();

        match (&mut content_fragment, &child_content_fragment) {
            (ComplexTypeFragment::Sequence(seq), ComplexTypeFragment::Sequence(child_seq)) => {
                seq.apply_extension(child_seq, ctx);
            }
            (ComplexTypeFragment::All(all), ComplexTypeFragment::All(child_all)) => {
                all.apply_extension(child_all, ctx);
            }
            (ComplexTypeFragment::Choice(choice), ComplexTypeFragment::Choice(child_choice)) => {
                choice.apply_extension(child_choice, ctx);
            }
            _ => {
                panic!("Expected both fragments to be the same type and of sequence, all or choice")
            }
        }

        *ctx.get_complex_fragment_mut(content_fragment_id).unwrap() = content_fragment;

        *ctx.get_complex_fragment_mut(fragment_id).unwrap() = transformed_fragment;
    }
}

impl XmlnsContextTransformer for ExpandBasedFragments {
    fn transform(self, mut ctx: Context<'_>) {
        for fragment_idx in ctx.iter_complex_fragment_ids() {
            let fragment = ctx.get_complex_fragment(&fragment_idx).unwrap();

            match fragment {
                ComplexTypeFragment::ComplexContent(ComplexContentFragment::Restriction(
                    complex::Restriction {
                        base,
                        content_fragment: child_content_fragment,
                    },
                )) => {
                    if base == &ComplexTypeIdent::Named(ANY_TYPE_EXPANDED_NAME.clone()) {
                        continue;
                    }

                    let base = base.clone();
                    let child_content_fragment = child_content_fragment.clone();
                    Self::expand_restriction(
                        &mut ctx,
                        &fragment_idx,
                        &base,
                        child_content_fragment.as_ref(),
                    )
                }
                ComplexTypeFragment::ComplexContent(ComplexContentFragment::Extension(
                    complex::Extension {
                        base,
                        content_fragment: child_content_fragment,
                    },
                )) => {
                    let base = base.clone();
                    let child_content_fragment = child_content_fragment.clone();
                    Self::expand_extension(
                        &mut ctx,
                        &fragment_idx,
                        &base,
                        child_content_fragment.as_ref(),
                    )
                }

                _ => continue,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use xmlity::{ExpandedName, LocalName, XmlNamespace};
    use xsd::schema::{
        Abstract, Base, ChoiceType, ComplexContent, ComplexContentContent, ComplexRestrictionType,
        ComplexTypeModel, ExtensionType, GroupTypeContent, LocalElement, MaxOccurs, MaxOccursValue,
        MinOccurs, QName, Ref, SequenceType, TopLevelComplexType, Type, TypeDefParticle,
    };

    use crate::{
        complex::{transformers::ExpandBasedFragments, ANY_TYPE_EXPANDED_NAME},
        transformers::XmlnsContextTransformer,
        CompiledNamespace, XmlnsContext,
    };

    #[test]
    fn test_expand_based_fragments() {
        let mut xmlns_context = XmlnsContext::new();

        let mut compiled_namespace =
            CompiledNamespace::new(XmlNamespace::new_dangerous("http://localhost"));

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
                    base: Base(QName(annotated)),
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

        let element = compiled_namespace
            .add_top_level_complex_type(&element)
            .into_owned();

        xmlns_context.add_namespace(compiled_namespace);

        ExpandBasedFragments.transform(super::Context {
            xmlns_context: &mut xmlns_context,
            namespace: &XmlNamespace::new_dangerous("http://localhost"),
        });

        // let top_level_element = xmlns_context
        //     .namespaces
        //     .get(element.namespace().unwrap())
        //     .unwrap()
        //     .top_level_types
        //     .get(element.local_name());

        let fragments = xmlns_context
            .namespaces
            .get(element.namespace().unwrap())
            .unwrap();

        let fragments = &fragments.complex_type.fragments;

        println!("{fragments:#?}");
    }
}
