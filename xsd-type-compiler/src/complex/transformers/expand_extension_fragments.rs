use std::collections::BTreeMap;
use std::collections::VecDeque;
use std::ops::Deref;

use xmlity::ExpandedName;

use crate::complex::AttributeDeclarationId;
use crate::complex::AttributeDeclarationsFragment;
use crate::complex::ComplexContentChildId;
use crate::complex::ComplexContentFragment;
use crate::complex::ComplexTypeModelId;
use crate::complex::ComplexTypeRootFragment;
use crate::complex::ExtensionFragment;
use crate::complex::FragmentAccess;
use crate::complex::FragmentIdx;
use crate::complex::LocalAttributeFragment;
use crate::complex::LocalAttributeFragmentTypeMode;
use crate::complex::RestrictionFragment;
use crate::complex::SequenceFragment;
use crate::transformers::TransformChange;
use crate::transformers::TransformerContext;
use crate::transformers::XmlnsContextTransformer;
use crate::TopLevelType;
use xsd::schema_names as xsn;

/// Expands restriction and extension fragments to their base fragments, with the modifications applied.
#[non_exhaustive]
pub struct ExpandExtensionFragments {}

impl ExpandExtensionFragments {
    pub fn new() -> Self {
        Self {}
    }

    fn expand_attribute(
        ctx: &mut TransformerContext<'_>,
        attribute: &FragmentIdx<LocalAttributeFragment>,
        base_attribute: &FragmentIdx<LocalAttributeFragment>,
    ) -> Result<(), ()> {
        let base_attribute = ctx.get_complex_fragment(base_attribute).unwrap().clone();
        let attribute = ctx.get_complex_fragment_mut(attribute).unwrap();

        let decl_base_attribute = match base_attribute.type_mode {
            LocalAttributeFragmentTypeMode::Declared(local) => local,
            _ => todo!(),
        };
        let decl_attribute = match &mut attribute.type_mode {
            LocalAttributeFragmentTypeMode::Declared(local) => local,
            _ => todo!(),
        };

        if attribute.use_.is_none() {
            attribute.use_ = base_attribute.use_;
        }
        if decl_attribute.type_.is_none() {
            decl_attribute.type_ = decl_base_attribute.type_;
        }

        Ok(())
    }

    fn expand_expanded_attributes<'a>(
        ctx: &mut TransformerContext<'_>,
        child_attributes: FragmentIdx<AttributeDeclarationsFragment>,
        base_attributes: FragmentIdx<AttributeDeclarationsFragment>,
    ) -> Result<FragmentIdx<AttributeDeclarationsFragment>, ()> {
        fn resolve_attr_name(
            ctx: &TransformerContext,
            a: &FragmentIdx<LocalAttributeFragment>,
        ) -> ExpandedName<'static> {
            let fragment = ctx.get_complex_fragment(a).unwrap();
            match &fragment.type_mode {
                LocalAttributeFragmentTypeMode::Declared(local) => {
                    ExpandedName::new(local.name.clone(), None)
                }
                LocalAttributeFragmentTypeMode::Reference(ref_) => ref_.name.clone(),
            }
        }

        let base_attribute_fragment = ctx.get_complex_fragment(&base_attributes).unwrap().clone();
        let child_attribute_fragment = ctx.get_complex_fragment(&child_attributes).unwrap().clone();

        let resolved_base_attributes = base_attribute_fragment
            .declarations
            .iter()
            .map(|a| match a {
                AttributeDeclarationId::Attribute(a) => (a.clone(), resolve_attr_name(&ctx, a)),
                AttributeDeclarationId::AttributeGroupRef(_) => todo!(),
            })
            .collect::<BTreeMap<_, _>>();
        let resolved_child_attributes = child_attribute_fragment
            .declarations
            .iter()
            .map(|a| match a {
                AttributeDeclarationId::Attribute(a) => (a.clone(), resolve_attr_name(&ctx, a)),
                AttributeDeclarationId::AttributeGroupRef(_) => todo!(),
            })
            .collect::<BTreeMap<_, _>>();

        let mut new_attribute_declarations = VecDeque::new();

        for base_attribute in base_attribute_fragment.declarations.iter() {
            let AttributeDeclarationId::Attribute(base_attribute) = base_attribute else {
                todo!()
            };

            let base_attribute_name = resolved_base_attributes.get(base_attribute).unwrap();

            let Some((matching_child_attribute, _)) = resolved_child_attributes
                .iter()
                .find(|(_, name)| *name == base_attribute_name)
            else {
                new_attribute_declarations
                    .push_back(AttributeDeclarationId::Attribute(*base_attribute));
                continue;
            };

            Self::expand_attribute(ctx, matching_child_attribute, base_attribute)?;

            new_attribute_declarations
                .push_back(AttributeDeclarationId::Attribute(*matching_child_attribute));
        }

        // Now we iterate through children attributes and only add those that have not been added yet because they were in the base.
        for child_attribute in child_attribute_fragment.declarations.iter() {
            let AttributeDeclarationId::Attribute(child_attribute) = child_attribute else {
                todo!()
            };

            let child_attribute_name = resolved_child_attributes.get(child_attribute).unwrap();

            if resolved_base_attributes
                .iter()
                .any(|(_, name)| name == child_attribute_name)
            {
                continue;
            }

            new_attribute_declarations
                .push_back(AttributeDeclarationId::Attribute(*child_attribute));
        }

        let child_attribute_fragment = ctx.get_complex_fragment_mut(&child_attributes).unwrap();
        child_attribute_fragment.declarations = new_attribute_declarations;

        Ok(child_attributes)
    }

    fn expand_extension(
        ctx: &mut TransformerContext<'_>,
        child_complex_content_fragment_idx: &FragmentIdx<ComplexContentFragment>,
    ) -> Result<TransformChange, ()> {
        let child_complex_content_fragment = ctx
            .get_complex_fragment(child_complex_content_fragment_idx)
            .unwrap();
        let child_fragment_idx = match child_complex_content_fragment.content_fragment {
            ComplexContentChildId::Extension(fragment_idx) => fragment_idx,
            ComplexContentChildId::Restriction(_) => {
                return Ok(TransformChange::Unchanged);
            }
        };

        let child_fragment = ctx.get_complex_fragment(&child_fragment_idx).unwrap();

        let base = child_fragment.base.clone();

        if &base == xsn::ANY_TYPE.deref() {
            return Ok(TransformChange::Unchanged);
        }

        let base_fragment = ctx.get_named_type(&base).unwrap();

        let base_fragment = match base_fragment {
            TopLevelType::Complex(complex) => complex,
            TopLevelType::Simple(_) => {
                todo!("Error - cannot expand complex content of simple type.")
            }
        };

        let base_root_fragment = ctx
            .get_complex_fragment::<ComplexTypeRootFragment>(&base_fragment.root_fragment)
            .unwrap();

        let (base_content_content_fragment_id, base_content_base, base_attributes) =
            match base_root_fragment.content {
                ComplexTypeModelId::Other {
                    particle,
                    attr_decls,
                    ..
                } => (particle, xsn::ANY_TYPE.clone(), attr_decls),
                ComplexTypeModelId::SimpleContent(_) => {
                    //TODO
                    return Ok(TransformChange::Unchanged);
                }
                ComplexTypeModelId::ComplexContent(base_complex_content_id) => {
                    let base_content_fragment =
                        ctx.get_complex_fragment(&base_complex_content_id).unwrap();

                    // Checks if base content is either a restriction of xs:anyType or an extension. If it is a non-anyType restriction, we cannot expand it since it could create a type that is not a valid derivative of the base's base type.

                    match base_content_fragment.content_fragment {
                        ComplexContentChildId::Extension(fragment_idx) => {
                            let base_extension_fragment = ctx
                                .get_complex_fragment::<ExtensionFragment>(&fragment_idx)
                                .unwrap();
                            (
                                base_extension_fragment.content_fragment.clone(),
                                base_extension_fragment.base.clone(),
                                base_extension_fragment.attribute_declarations.clone(),
                            )
                        }
                        ComplexContentChildId::Restriction(fragment_idx) => {
                            let base_restriction_fragment = ctx
                                .get_complex_fragment::<RestrictionFragment>(&fragment_idx)
                                .unwrap();

                            if &base_restriction_fragment.base != xsn::ANY_TYPE.deref() {
                                todo!("Error - cannot expand complex content of restriction type.")
                            }

                            (
                                base_restriction_fragment.content_fragment.clone(),
                                base_restriction_fragment.base.clone(),
                                base_restriction_fragment.attribute_declarations.clone(),
                            )
                        }
                    }
                }
            };

        let child_attributes = child_fragment.attribute_declarations.clone();

        let new_content_fragment = child_fragment
            .content_fragment
            .map(|child_content_content_fragment_id| {
                let Some(base_content_content_fragment_id) = base_content_content_fragment_id
                else {
                    return child_content_content_fragment_id;
                };

                let new_content_fragment = SequenceFragment {
                    max_occurs: None,
                    min_occurs: None,
                    fragments: {
                        let mut fragments = VecDeque::with_capacity(2);
                        fragments.push_back(base_content_content_fragment_id.into());
                        fragments.push_back(child_content_content_fragment_id.into());
                        fragments
                    },
                };

                let compiler = &mut ctx.current_namespace_mut().complex_type;

                let new_content_fragment = compiler.push_fragment(new_content_fragment);

                new_content_fragment.into()
            })
            .or(base_content_content_fragment_id);

        let new_attribute_declarations =
            Self::expand_expanded_attributes(ctx, child_attributes, base_attributes)?;

        let new_child_content = RestrictionFragment {
            base: base_content_base,
            content_fragment: new_content_fragment,
            attribute_declarations: new_attribute_declarations,
        };

        let compiler = &mut ctx.current_namespace_mut().complex_type;

        let new_child_content = compiler.push_fragment(new_child_content);

        compiler
            .get_fragment_mut(&child_complex_content_fragment_idx)
            .unwrap()
            .content_fragment = ComplexContentChildId::Restriction(new_child_content);

        Ok(TransformChange::Changed)
    }
}

impl XmlnsContextTransformer for ExpandExtensionFragments {
    type Error = ();

    fn transform(self, mut ctx: TransformerContext<'_>) -> Result<TransformChange, ()> {
        ctx.iter_complex_fragment_ids()
            .into_iter()
            .map(|f| Self::expand_extension(&mut ctx, &f))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use xmlity::{ExpandedName, LocalName, XmlNamespace};
    use xsd::schema::{self as xs, MaxOccurs, MaxOccursValue};
    use xsd::schema_names as xsn;

    use crate::{
        complex::transformers::ExpandExtensionFragments, transformers::TransformChange,
        CompiledNamespace, XmlnsContext,
    };

    #[test]
    fn basic_child_only_expand_extension() {
        let test_namespace = XmlNamespace::new_dangerous("http://localhost");

        let parent_seq = xs::SequenceType::builder()
            .content(vec![
                xs::LocalElement::new_ref_typed(
                    LocalName::new_dangerous("number"),
                    ExpandedName::new(LocalName::new_dangerous("integer"), XmlNamespace::XS.into()),
                )
                .into(),
                xs::LocalElement::new_ref_typed(
                    LocalName::new_dangerous("name"),
                    ExpandedName::new(LocalName::new_dangerous("string"), XmlNamespace::XS.into()),
                )
                .into(),
            ])
            .build();

        // ```xml
        // <xs:complexType name="ProductType">
        //   <xs:complexContent>
        //     <xs:restriction base="xs:anyType">
        //       <xs:sequence>
        //         <xs:element name="number" type="xs:integer"/>
        //         <xs:element name="name" type="xs:string"/>
        //       </xs:sequence>
        //     </xs:restriction>
        //   </xs:complexContent>
        // </xs:complexType>
        // ```
        let product_type = xs::TopLevelComplexType::builder()
            .name(LocalName::new_dangerous("ProductType"))
            .content(
                xs::ComplexContent::builder()
                    .content(
                        xs::ComplexRestrictionType::builder()
                            .base(xs::QName(xsn::ANY_TYPE.clone()))
                            .particle(parent_seq.clone().into())
                            .build()
                            .into(),
                    )
                    .build()
                    .into(),
            )
            .build();

        let child_choice = xs::ChoiceType::builder()
            .max_occurs(MaxOccurs(MaxOccursValue::Unbounded))
            .content(vec![
                xs::LocalElement::new_ref_typed(
                    LocalName::new_dangerous("size"),
                    ExpandedName::new(LocalName::new_dangerous("integer"), XmlNamespace::XS.into()),
                )
                .into(),
                xs::LocalElement::new_ref_typed(
                    LocalName::new_dangerous("color"),
                    ExpandedName::new(LocalName::new_dangerous("string"), XmlNamespace::XS.into()),
                )
                .into(),
            ])
            .build();

        // <xs:complexType name="ShirtType">
        //   <xs:complexContent>
        //     <xs:extension base="ProductType">
        //       <xs:choice maxOccurs="unbounded">
        //         <xs:element name="size" type="xs:integer"/>
        //         <xs:element name="color" type="xs:string"/>
        //       </xs:choice>
        //     </xs:extension>
        //   </xs:complexContent>
        // </xs:complexType>
        let derived_shirt_type = xs::TopLevelComplexType::builder()
            .name(LocalName::new_dangerous("ShirtType"))
            .content(xs::ComplexTypeModel::ComplexContent(
                xs::ComplexContent::builder()
                    .content(
                        xs::ExtensionType::builder()
                            .base(xs::QName(ExpandedName::new(
                                LocalName::new_dangerous("ProductType"),
                                Some(test_namespace.clone()),
                            )))
                            .particle(child_choice.clone().into())
                            .build()
                            .into(),
                    )
                    .build(),
            ))
            .build();

        // ```xml
        // <xs:complexType name="ShirtType">
        //   <xs:complexContent>
        //     <xs:restriction base="xs:anyType">
        //       <xs:sequence>
        //         <xs:sequence>
        //           <xs:element name="number" type="xs:integer"/>
        //           <xs:element name="name" type="xs:string"/>
        //         </xs:sequence>
        //         <xs:choice maxOccurs="unbounded">
        //           <xs:element name="size" type="xs:integer"/>
        //           <xs:element name="color" type="xs:string"/>
        //         </xs:choice>
        //       </xs:sequence>
        //     </xs:restriction>
        //   </xs:complexContent>
        // </xs:complexType>
        // ```
        let expected_flattened_shirt_type = xs::TopLevelComplexType::builder()
            .name(LocalName::new_dangerous("ShirtType"))
            .content(xs::ComplexTypeModel::ComplexContent(
                xs::ComplexContent::builder()
                    .content(
                        xs::ComplexRestrictionType::builder()
                            .base(xs::QName(xsn::ANY_TYPE.clone()))
                            .particle(xs::TypeDefParticle::Sequence(
                                xs::SequenceType::builder()
                                    .content(vec![parent_seq.into(), child_choice.into()])
                                    .build(),
                            ))
                            .build()
                            .into(),
                    )
                    .build(),
            ))
            .build();

        let mut xmlns_context = XmlnsContext::new();

        let mut compiled_namespace = CompiledNamespace::new(test_namespace.clone());

        compiled_namespace
            .import_top_level_complex_type(&product_type)
            .unwrap();
        compiled_namespace
            .import_top_level_complex_type(&derived_shirt_type)
            .unwrap();

        xmlns_context.add_namespace(compiled_namespace);

        let transform_changed = xmlns_context
            .transform(&test_namespace, ExpandExtensionFragments::new())
            .unwrap();

        assert_eq!(transform_changed, TransformChange::Changed);

        let transform_changed = xmlns_context
            .transform(&test_namespace, ExpandExtensionFragments::new())
            .unwrap();

        assert_eq!(transform_changed, TransformChange::Unchanged);

        let compiled_namespace = xmlns_context.namespaces.get(&test_namespace).unwrap();

        let actual_flattened_shirt_type = compiled_namespace
            .export_top_level_complex_type(&LocalName::new_dangerous("ShirtType"))
            .unwrap()
            .unwrap();

        assert_eq!(expected_flattened_shirt_type, actual_flattened_shirt_type);
    }

    #[test]
    fn basic_attribute_only_expand_extension() {
        let test_namespace = XmlNamespace::new_dangerous("http://localhost");

        // <xs:complexType name="ProductType">
        //   <xs:complexContent>
        //     <xs:restriction base="xs:anyType">
        //       <xs:attribute name="number" type="xs:integer" use="required" />
        //       <xs:attribute name="name" type="xs:string" use="required" />
        //     </xs:restriction>
        //   </xs:complexContent>
        // </xs:complexType>
        let product_type = xs::TopLevelComplexType::builder()
            .name(LocalName::new_dangerous("ProductType"))
            .content(
                xs::ComplexContent::builder()
                    .content(
                        xs::ComplexRestrictionType::builder()
                            .base(xs::QName(xsn::ANY_TYPE.clone()))
                            .attr_decls(
                                xs::AttrDecls::builder()
                                    .declarations(vec![
                                        xs::LocalAttribute::builder()
                                            .name(LocalName::new_dangerous("number"))
                                            .type_(xs::QName(xsn::INTEGER.clone()))
                                            .use_(xs::AttributeUseType::Required)
                                            .build()
                                            .into(),
                                        xs::LocalAttribute::builder()
                                            .name(LocalName::new_dangerous("name"))
                                            .type_(xs::QName(xsn::STRING.clone()))
                                            .use_(xs::AttributeUseType::Required)
                                            .build()
                                            .into(),
                                    ])
                                    .build(),
                            )
                            .build()
                            .into(),
                    )
                    .build()
                    .into(),
            )
            .build();

        // <xs:complexType name="ShirtType">
        //   <xs:complexContent>
        //     <xs:extension base="ProductType">
        //       <xs:attribute name="number" type="xs:integer" use="optional" />
        //       <xs:attribute name="color" type="xs:string" use="required" />
        //     </xs:extension>
        //   </xs:complexContent>
        // </xs:complexType>
        let derived_shirt_type = xs::TopLevelComplexType::builder()
            .name(LocalName::new_dangerous("ShirtType"))
            .content(xs::ComplexTypeModel::ComplexContent(
                xs::ComplexContent::builder()
                    .content(
                        xs::ExtensionType::builder()
                            .base(xs::QName(ExpandedName::new(
                                LocalName::new_dangerous("ProductType"),
                                Some(test_namespace.clone()),
                            )))
                            .attr_decls(
                                xs::AttrDecls::builder()
                                    .declarations(vec![
                                        xs::LocalAttribute::builder()
                                            .name(LocalName::new_dangerous("number"))
                                            .type_(xs::QName(xsn::INTEGER.clone()))
                                            .use_(xs::AttributeUseType::Optional)
                                            .build()
                                            .into(),
                                        xs::LocalAttribute::builder()
                                            .name(LocalName::new_dangerous("color"))
                                            .type_(xs::QName(xsn::STRING.clone()))
                                            .use_(xs::AttributeUseType::Required)
                                            .build()
                                            .into(),
                                    ])
                                    .build(),
                            )
                            .build()
                            .into(),
                    )
                    .build(),
            ))
            .build();

        // <xs:complexType name="ShirtType">
        //   <xs:complexContent>
        //     <xs:restriction base="xs:anyType">
        //       <xs:attribute name="number" type="xs:integer" use="optional" />
        //       <xs:attribute name="name" type="xs:string" use="required" />
        //       <xs:attribute name="color" type="xs:string" use="required" />
        //     </xs:restriction>
        //   </xs:complexContent>
        // </xs:complexType>
        let expected_flattened_shirt_type = xs::TopLevelComplexType::builder()
            .name(LocalName::new_dangerous("ShirtType"))
            .content(xs::ComplexTypeModel::ComplexContent(
                xs::ComplexContent::builder()
                    .content(
                        xs::ComplexRestrictionType::builder()
                            .base(xs::QName(xsn::ANY_TYPE.clone()))
                            .attr_decls(
                                xs::AttrDecls::builder()
                                    .declarations(vec![
                                        xs::LocalAttribute::builder()
                                            .name(LocalName::new_dangerous("number"))
                                            .type_(xs::QName(xsn::INTEGER.clone()))
                                            .use_(xs::AttributeUseType::Optional)
                                            .build()
                                            .into(),
                                        xs::LocalAttribute::builder()
                                            .name(LocalName::new_dangerous("name"))
                                            .type_(xs::QName(xsn::STRING.clone()))
                                            .use_(xs::AttributeUseType::Required)
                                            .build()
                                            .into(),
                                        xs::LocalAttribute::builder()
                                            .name(LocalName::new_dangerous("color"))
                                            .type_(xs::QName(xsn::STRING.clone()))
                                            .use_(xs::AttributeUseType::Required)
                                            .build()
                                            .into(),
                                    ])
                                    .build(),
                            )
                            .build()
                            .into(),
                    )
                    .build(),
            ))
            .build();

        let mut xmlns_context = XmlnsContext::new();

        let mut compiled_namespace = CompiledNamespace::new(test_namespace.clone());

        compiled_namespace
            .import_top_level_complex_type(&product_type)
            .unwrap();
        compiled_namespace
            .import_top_level_complex_type(&derived_shirt_type)
            .unwrap();

        xmlns_context.add_namespace(compiled_namespace);

        let transform_changed = xmlns_context
            .transform(&test_namespace, ExpandExtensionFragments::new())
            .unwrap();

        assert_eq!(transform_changed, TransformChange::Changed);

        let transform_changed = xmlns_context
            .transform(&test_namespace, ExpandExtensionFragments::new())
            .unwrap();

        assert_eq!(transform_changed, TransformChange::Unchanged);

        let compiled_namespace = xmlns_context.namespaces.get(&test_namespace).unwrap();

        let actual_flattened_shirt_type = compiled_namespace
            .export_top_level_complex_type(&LocalName::new_dangerous("ShirtType"))
            .unwrap()
            .unwrap();

        assert_eq!(expected_flattened_shirt_type, actual_flattened_shirt_type);
    }

    #[test]
    fn expand_extension_type_element_no_fragment() {
        // <xs:complexType name="Block">
        //     <xs:choice minOccurs="0" maxOccurs="unbounded">
        //     <xs:group ref="block"/>
        //     <xs:element ref="form"/>
        //     <xs:group ref="misc"/>
        //     </xs:choice>
        // </xs:complexType>
        let block = xs::TopLevelComplexType::builder()
            .name(LocalName::new_dangerous("Block"))
            .content(xs::ComplexTypeModel::ComplexContent(
                xs::ComplexContent::builder()
                    .content(
                        xs::ComplexRestrictionType::builder()
                            .base(xs::QName(xsn::ANY_TYPE.clone()))
                            .particle(xs::TypeDefParticle::Choice(
                                xs::ChoiceType::builder()
                                    .min_occurs(xs::MinOccurs(0))
                                    .max_occurs(MaxOccurs(MaxOccursValue::Unbounded))
                                    .content(vec![
                                        xs::GroupRef::builder()
                                            .ref_(xs::QName(ExpandedName::new(
                                                LocalName::new_dangerous("block"),
                                                Some(XmlNamespace::XHTML),
                                            )))
                                            .build()
                                            .into(),
                                        xs::LocalElement::builder()
                                            .ref_(xs::QName(ExpandedName::new(
                                                LocalName::new_dangerous("form"),
                                                Some(XmlNamespace::XHTML),
                                            )))
                                            .build()
                                            .into(),
                                        xs::GroupRef::builder()
                                            .ref_(xs::QName(ExpandedName::new(
                                                LocalName::new_dangerous("misc"),
                                                Some(XmlNamespace::XHTML),
                                            )))
                                            .build()
                                            .into(),
                                    ])
                                    .build(),
                            ))
                            .build()
                            .into(),
                    )
                    .build(),
            ))
            .build();

        // <xs:element name="noscript">
        //     <xs:annotation>
        //     <xs:documentation>
        //     alternate content container for non script-based rendering
        //     </xs:documentation>
        //     </xs:annotation>
        //     <xs:complexType>
        //     <xs:complexContent>
        //         <xs:extension base="Block">
        //         <xs:attributeGroup ref="attrs"/>
        //         </xs:extension>
        //     </xs:complexContent>
        //     </xs:complexType>
        // </xs:element>
        let noscript = xs::TopLevelElement(
            xs::types::TopLevelElement::builder()
                .name(LocalName::new_dangerous("noscript"))
                .child_1(
                    xs::LocalComplexType::builder()
                        .content(
                            xs::ComplexContent::builder()
                                .content(
                                    xs::ExtensionType::builder()
                                        .base(xs::QName(ExpandedName::new(
                                            LocalName::new_dangerous("Block"),
                                            Some(XmlNamespace::XHTML),
                                        )))
                                        .attr_decls(
                                            xs::AttrDecls::builder()
                                                .declarations(vec![
                                                // xs::AttributeGroupRefType::builder()
                                                // .ref_(xs::QName(ExpandedName::new(
                                                //     LocalName::new_dangerous("attrs"),
                                                //     Some(XmlNamespace::XHTML),
                                                // )))
                                                // .build()
                                                // .into()
                                                ])
                                                .build(),
                                        )
                                        .build()
                                        .into(),
                                )
                                .build()
                                .into(),
                        )
                        .build()
                        .into(),
                )
                .build(),
        );

        let expected_flattened_noscript = xs::TopLevelElement(
            xs::types::TopLevelElement::builder()
                .name(LocalName::new_dangerous("noscript"))
                .child_1(
                    xs::LocalComplexType::builder()
                        .content(
                            xs::ComplexContent::builder()
                                .content(
                                    xs::ComplexRestrictionType::builder()
                                        .base(xs::QName(xsn::ANY_TYPE.clone()))
                                        .particle(xs::TypeDefParticle::Choice(
                                            xs::ChoiceType::builder()
                                                .min_occurs(xs::MinOccurs(0))
                                                .max_occurs(MaxOccurs(MaxOccursValue::Unbounded))
                                                .content(vec![
                                                    xs::GroupRef::builder()
                                                        .ref_(xs::QName(ExpandedName::new(
                                                            LocalName::new_dangerous("block"),
                                                            Some(XmlNamespace::XHTML),
                                                        )))
                                                        .build()
                                                        .into(),
                                                    xs::LocalElement::builder()
                                                        .ref_(xs::QName(ExpandedName::new(
                                                            LocalName::new_dangerous("form"),
                                                            Some(XmlNamespace::XHTML),
                                                        )))
                                                        .build()
                                                        .into(),
                                                    xs::GroupRef::builder()
                                                        .ref_(xs::QName(ExpandedName::new(
                                                            LocalName::new_dangerous("misc"),
                                                            Some(XmlNamespace::XHTML),
                                                        )))
                                                        .build()
                                                        .into(),
                                                ])
                                                .build(),
                                        ))
                                        .attr_decls(
                                            xs::AttrDecls::builder()
                                                .declarations(vec![
                                                    // xs::AttributeGroupRefType::builder()
                                                    // .ref_(xs::QName(ExpandedName::new(
                                                    //     LocalName::new_dangerous("attrs"),
                                                    //     Some(XmlNamespace::XHTML),
                                                    // )))
                                                    // .build()
                                                    // .into()
                                                ])
                                                .build(),
                                        )
                                        .build()
                                        .into(),
                                )
                                .build()
                                .into(),
                        )
                        .build()
                        .into(),
                )
                .build(),
        );

        let mut xmlns_context = XmlnsContext::new();

        let mut compiled_namespace = CompiledNamespace::new(XmlNamespace::XHTML);

        compiled_namespace
            .import_top_level_complex_type(&block)
            .unwrap();
        compiled_namespace
            .import_top_level_element(&noscript)
            .unwrap();

        xmlns_context.add_namespace(compiled_namespace);

        let transform_changed = xmlns_context
            .transform(&XmlNamespace::XHTML, ExpandExtensionFragments::new())
            .unwrap();

        assert_eq!(transform_changed, TransformChange::Changed);

        let transform_changed = xmlns_context
            .transform(&XmlNamespace::XHTML, ExpandExtensionFragments::new())
            .unwrap();

        assert_eq!(transform_changed, TransformChange::Unchanged);

        let compiled_namespace = xmlns_context.namespaces.get(&XmlNamespace::XHTML).unwrap();

        let actual_flattened_noscript = compiled_namespace
            .export_top_level_element(&LocalName::new_dangerous("noscript"))
            .unwrap()
            .unwrap();

        assert_eq!(expected_flattened_noscript, actual_flattened_noscript);
    }

    #[test]
    fn expand_xhtml_a() {
        // <xs:complexType name="a.content" mixed="true">
        //     <xs:annotation>
        //     <xs:documentation>
        //     a elements use "Inline" excluding a
        //     </xs:documentation>
        //     </xs:annotation>
        //     <xs:choice minOccurs="0" maxOccurs="unbounded">
        //     <xs:group ref="special"/>
        //     <xs:group ref="fontstyle"/>
        //     <xs:group ref="phrase"/>
        //     <xs:group ref="inline.forms"/>
        //     <xs:group ref="misc.inline"/>
        //     </xs:choice>
        // </xs:complexType>
        let a_content = xs::TopLevelComplexType::builder()
            .name(LocalName::new_dangerous("a.content"))
            .mixed(true)
            .content(xs::ComplexTypeModel::ComplexContent(
                xs::ComplexContent::builder()
                    .content(
                        xs::ComplexRestrictionType::builder()
                            .base(xs::QName(xsn::ANY_TYPE.clone()))
                            .particle(xs::TypeDefParticle::Choice(
                                xs::ChoiceType::builder()
                                    .min_occurs(xs::MinOccurs(0))
                                    .max_occurs(MaxOccurs(MaxOccursValue::Unbounded))
                                    .content(vec![
                                        xs::GroupRef::builder()
                                            .ref_(xs::QName(ExpandedName::new(
                                                LocalName::new_dangerous("special"),
                                                Some(XmlNamespace::XHTML),
                                            )))
                                            .build()
                                            .into(),
                                        xs::GroupRef::builder()
                                            .ref_(xs::QName(ExpandedName::new(
                                                LocalName::new_dangerous("fontstyle"),
                                                Some(XmlNamespace::XHTML),
                                            )))
                                            .build()
                                            .into(),
                                        xs::GroupRef::builder()
                                            .ref_(xs::QName(ExpandedName::new(
                                                LocalName::new_dangerous("phrase"),
                                                Some(XmlNamespace::XHTML),
                                            )))
                                            .build()
                                            .into(),
                                        xs::GroupRef::builder()
                                            .ref_(xs::QName(ExpandedName::new(
                                                LocalName::new_dangerous("inline.forms"),
                                                Some(XmlNamespace::XHTML),
                                            )))
                                            .build()
                                            .into(),
                                        xs::GroupRef::builder()
                                            .ref_(xs::QName(ExpandedName::new(
                                                LocalName::new_dangerous("misc.inline"),
                                                Some(XmlNamespace::XHTML),
                                            )))
                                            .build()
                                            .into(),
                                    ])
                                    .build(),
                            ))
                            .build()
                            .into(),
                    )
                    .build(),
            ))
            .build();

        // <xs:element name="a">
        //     <xs:annotation>
        //         <xs:documentation>
        //         content is "Inline" except that anchors shouldn't be nested
        //         </xs:documentation>
        //     </xs:annotation>
        //     <xs:complexType mixed="true">
        //         <xs:complexContent>
        //         <xs:extension base="a.content">
        //             <xs:attributeGroup ref="attrs"/>
        //             <xs:attributeGroup ref="focus"/>
        //             <xs:attribute name="charset" type="Charset"/>
        //             <xs:attribute name="type" type="ContentType"/>
        //             <xs:attribute name="name" type="xs:NMTOKEN"/>
        //             <xs:attribute name="href" type="URI"/>
        //             <xs:attribute name="hreflang" type="LanguageCode"/>
        //             <xs:attribute name="rel" type="LinkTypes"/>
        //             <xs:attribute name="rev" type="LinkTypes"/>
        //             <xs:attribute name="shape" default="rect" type="Shape"/>
        //             <xs:attribute name="coords" type="Coords"/>
        //         </xs:extension>
        //         </xs:complexContent>
        //     </xs:complexType>
        // </xs:element>
        let a = xs::TopLevelElement(
            xs::types::TopLevelElement::builder()
                .name(LocalName::new_dangerous("a"))
                .child_1(
                    xs::LocalComplexType::builder()
                        .mixed(true)
                        .content(
                            xs::ComplexContent::builder()
                                .content(
                                    xs::ExtensionType::builder()
                                        .base(xs::QName(ExpandedName::new(
                                            LocalName::new_dangerous("a.content"),
                                            Some(XmlNamespace::XHTML),
                                        )))
                                        .attr_decls(
                                            xs::AttrDecls::builder()
                                                .declarations(vec![
                                                    xs::LocalAttribute::builder()
                                                        .name(LocalName::new_dangerous("charset"))
                                                        .type_(xs::QName(ExpandedName::new(
                                                            LocalName::new_dangerous("Charset"),
                                                            Some(XmlNamespace::XHTML),
                                                        )))
                                                        .build()
                                                        .into(),
                                                    xs::LocalAttribute::builder()
                                                        .name(LocalName::new_dangerous("type"))
                                                        .type_(xs::QName(ExpandedName::new(
                                                            LocalName::new_dangerous("ContentType"),
                                                            Some(XmlNamespace::XHTML),
                                                        )))
                                                        .build()
                                                        .into(),
                                                    xs::LocalAttribute::builder()
                                                        .name(LocalName::new_dangerous("name"))
                                                        .type_(xs::QName(xsn::NMTOKEN.clone()))
                                                        .build()
                                                        .into(),
                                                    xs::LocalAttribute::builder()
                                                        .name(LocalName::new_dangerous("href"))
                                                        .type_(xs::QName(ExpandedName::new(
                                                            LocalName::new_dangerous("URI"),
                                                            Some(XmlNamespace::XHTML),
                                                        )))
                                                        .build()
                                                        .into(),
                                                    xs::LocalAttribute::builder()
                                                        .name(LocalName::new_dangerous("hreflang"))
                                                        .type_(xs::QName(ExpandedName::new(
                                                            LocalName::new_dangerous(
                                                                "LanguageCode",
                                                            ),
                                                            Some(XmlNamespace::XHTML),
                                                        )))
                                                        .build()
                                                        .into(),
                                                    xs::LocalAttribute::builder()
                                                        .name(LocalName::new_dangerous("rel"))
                                                        .type_(xs::QName(ExpandedName::new(
                                                            LocalName::new_dangerous("LinkTypes"),
                                                            Some(XmlNamespace::XHTML),
                                                        )))
                                                        .build()
                                                        .into(),
                                                    xs::LocalAttribute::builder()
                                                        .name(LocalName::new_dangerous("rev"))
                                                        .type_(xs::QName(ExpandedName::new(
                                                            LocalName::new_dangerous("LinkTypes"),
                                                            Some(XmlNamespace::XHTML),
                                                        )))
                                                        .build()
                                                        .into(),
                                                    xs::LocalAttribute::builder()
                                                        .name(LocalName::new_dangerous("shape"))
                                                        .default("rect".to_string())
                                                        .type_(xs::QName(ExpandedName::new(
                                                            LocalName::new_dangerous("Shape"),
                                                            Some(XmlNamespace::XHTML),
                                                        )))
                                                        .build()
                                                        .into(),
                                                    xs::LocalAttribute::builder()
                                                        .name(LocalName::new_dangerous("coords"))
                                                        .type_(xs::QName(ExpandedName::new(
                                                            LocalName::new_dangerous("Coords"),
                                                            Some(XmlNamespace::XHTML),
                                                        )))
                                                        .build()
                                                        .into(),
                                                ])
                                                .build(),
                                        )
                                        .build()
                                        .into(),
                                )
                                .build()
                                .into(),
                        )
                        .build()
                        .into(),
                )
                .build(),
        );

        let expected_flattened_a = xs::TopLevelElement(
            xs::types::TopLevelElement::builder()
                .name(LocalName::new_dangerous("a"))
                .child_1(
                    xs::LocalComplexType::builder()
                        .mixed(true)
                        .content(
                            xs::ComplexContent::builder()
                                .content(
                                    xs::ComplexRestrictionType::builder()
                                        .base(xs::QName(xsn::ANY_TYPE.clone()))
                                        .particle(xs::TypeDefParticle::Choice(
                                            xs::ChoiceType::builder()
                                                .min_occurs(xs::MinOccurs(0))
                                                .max_occurs(MaxOccurs(MaxOccursValue::Unbounded))
                                                .content(vec![
                                                    xs::GroupRef::builder()
                                                        .ref_(xs::QName(ExpandedName::new(
                                                            LocalName::new_dangerous("special"),
                                                            Some(XmlNamespace::XHTML),
                                                        )))
                                                        .build()
                                                        .into(),
                                                    xs::GroupRef::builder()
                                                        .ref_(xs::QName(ExpandedName::new(
                                                            LocalName::new_dangerous("fontstyle"),
                                                            Some(XmlNamespace::XHTML),
                                                        )))
                                                        .build()
                                                        .into(),
                                                    xs::GroupRef::builder()
                                                        .ref_(xs::QName(ExpandedName::new(
                                                            LocalName::new_dangerous("phrase"),
                                                            Some(XmlNamespace::XHTML),
                                                        )))
                                                        .build()
                                                        .into(),
                                                    xs::GroupRef::builder()
                                                        .ref_(xs::QName(ExpandedName::new(
                                                            LocalName::new_dangerous(
                                                                "inline.forms",
                                                            ),
                                                            Some(XmlNamespace::XHTML),
                                                        )))
                                                        .build()
                                                        .into(),
                                                    xs::GroupRef::builder()
                                                        .ref_(xs::QName(ExpandedName::new(
                                                            LocalName::new_dangerous("misc.inline"),
                                                            Some(XmlNamespace::XHTML),
                                                        )))
                                                        .build()
                                                        .into(),
                                                ])
                                                .build(),
                                        ))
                                        .attr_decls(
                                            xs::AttrDecls::builder()
                                                .declarations(vec![
                                                    xs::LocalAttribute::builder()
                                                        .name(LocalName::new_dangerous("charset"))
                                                        .type_(xs::QName(ExpandedName::new(
                                                            LocalName::new_dangerous("Charset"),
                                                            Some(XmlNamespace::XHTML),
                                                        )))
                                                        .build()
                                                        .into(),
                                                    xs::LocalAttribute::builder()
                                                        .name(LocalName::new_dangerous("type"))
                                                        .type_(xs::QName(ExpandedName::new(
                                                            LocalName::new_dangerous("ContentType"),
                                                            Some(XmlNamespace::XHTML),
                                                        )))
                                                        .build()
                                                        .into(),
                                                    xs::LocalAttribute::builder()
                                                        .name(LocalName::new_dangerous("name"))
                                                        .type_(xs::QName(xsn::NMTOKEN.clone()))
                                                        .build()
                                                        .into(),
                                                    xs::LocalAttribute::builder()
                                                        .name(LocalName::new_dangerous("href"))
                                                        .type_(xs::QName(ExpandedName::new(
                                                            LocalName::new_dangerous("URI"),
                                                            Some(XmlNamespace::XHTML),
                                                        )))
                                                        .build()
                                                        .into(),
                                                    xs::LocalAttribute::builder()
                                                        .name(LocalName::new_dangerous("hreflang"))
                                                        .type_(xs::QName(ExpandedName::new(
                                                            LocalName::new_dangerous(
                                                                "LanguageCode",
                                                            ),
                                                            Some(XmlNamespace::XHTML),
                                                        )))
                                                        .build()
                                                        .into(),
                                                    xs::LocalAttribute::builder()
                                                        .name(LocalName::new_dangerous("rel"))
                                                        .type_(xs::QName(ExpandedName::new(
                                                            LocalName::new_dangerous("LinkTypes"),
                                                            Some(XmlNamespace::XHTML),
                                                        )))
                                                        .build()
                                                        .into(),
                                                    xs::LocalAttribute::builder()
                                                        .name(LocalName::new_dangerous("rev"))
                                                        .type_(xs::QName(ExpandedName::new(
                                                            LocalName::new_dangerous("LinkTypes"),
                                                            Some(XmlNamespace::XHTML),
                                                        )))
                                                        .build()
                                                        .into(),
                                                    xs::LocalAttribute::builder()
                                                        .name(LocalName::new_dangerous("shape"))
                                                        .default("rect".to_string())
                                                        .type_(xs::QName(ExpandedName::new(
                                                            LocalName::new_dangerous("Shape"),
                                                            Some(XmlNamespace::XHTML),
                                                        )))
                                                        .build()
                                                        .into(),
                                                    xs::LocalAttribute::builder()
                                                        .name(LocalName::new_dangerous("coords"))
                                                        .type_(xs::QName(ExpandedName::new(
                                                            LocalName::new_dangerous("Coords"),
                                                            Some(XmlNamespace::XHTML),
                                                        )))
                                                        .build()
                                                        .into(),
                                                ])
                                                .build(),
                                        )
                                        .build()
                                        .into(),
                                )
                                .build()
                                .into(),
                        )
                        .build()
                        .into(),
                )
                .build(),
        );

        let mut xmlns_context = XmlnsContext::new();

        let mut compiled_namespace = CompiledNamespace::new(XmlNamespace::XHTML);

        compiled_namespace
            .import_top_level_complex_type(&a_content)
            .unwrap();
        compiled_namespace.import_top_level_element(&a).unwrap();

        xmlns_context.add_namespace(compiled_namespace);

        let transform_changed = xmlns_context
            .transform(&XmlNamespace::XHTML, ExpandExtensionFragments::new())
            .unwrap();

        assert_eq!(transform_changed, TransformChange::Changed);

        let transform_changed = xmlns_context
            .transform(&XmlNamespace::XHTML, ExpandExtensionFragments::new())
            .unwrap();

        assert_eq!(transform_changed, TransformChange::Unchanged);

        let compiled_namespace = xmlns_context.namespaces.get(&XmlNamespace::XHTML).unwrap();

        let actual_flattened_a = compiled_namespace
            .export_top_level_element(&LocalName::new_dangerous("a"))
            .unwrap()
            .unwrap();

        assert_eq!(expected_flattened_a, actual_flattened_a);
    }
}
