use std::collections::BTreeMap;
use std::collections::VecDeque;

use xmlity::ExpandedName;
use xsd::xsn;
use xsd_fragments::fragments::complex::AttributeDeclarationId;
use xsd_fragments::fragments::complex::AttributeDeclarationsFragment;
use xsd_fragments::fragments::complex::ComplexContentChildId;
use xsd_fragments::fragments::complex::ComplexTypeModelId;
use xsd_fragments::fragments::complex::ComplexTypeRootFragment;
use xsd_fragments::fragments::complex::LocalAttributeFragment;
use xsd_fragments::fragments::complex::LocalAttributeFragmentTypeMode;
use xsd_fragments::fragments::complex::RestrictionFragment;
use xsd_fragments::fragments::FragmentIdx;
use xsd_fragments::TopLevelType;

use crate::TransformChange;
use crate::XmlnsContextTransformer;
use crate::XmlnsContextTransformerContext;

/// Expands restriction and extension fragments to their base fragments, with the modifications applied.
#[non_exhaustive]
pub struct ExpandRestrictionFragments {}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(
        "Cannot restrict reference attribute {attribute:?} with base attribute {base_attribute:?}"
    )]
    CannotRestrictReferenceAttribute {
        attribute: FragmentIdx<LocalAttributeFragment>,
        base_attribute: FragmentIdx<LocalAttributeFragment>,
    },
    #[error(
        "Cannot handle attribute group reference in attribute declarations. This transformer does not support attribute group references."
    )]
    CannotHandleAttributeGroupRef {},
    #[error("Cannot restrict base type {base:?} to a simple type. Only complex types can be restricted.")]
    BaseCannotBeSimpleType { base: ExpandedName<'static> },
    #[error("Cannot restrict base type {base:?} to an extension type. Only complex types can be extended.")]
    BaseCannotBeExtensionType { base: ExpandedName<'static> },
}

impl ExpandRestrictionFragments {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {}
    }

    fn restrict_attribute(
        ctx: &mut XmlnsContextTransformerContext<'_>,
        attribute_id: &FragmentIdx<LocalAttributeFragment>,
        base_attribute_id: &FragmentIdx<LocalAttributeFragment>,
    ) -> Result<(), <Self as XmlnsContextTransformer>::Error> {
        let base_attribute = ctx.get_complex_fragment(base_attribute_id).unwrap().clone();
        let attribute = ctx.get_complex_fragment_mut(attribute_id).unwrap();

        use LocalAttributeFragmentTypeMode as TypeMode;

        match (base_attribute.type_mode, &mut attribute.type_mode) {
            (TypeMode::Declared(decl_base_attribute), TypeMode::Declared(decl_attribute)) => {
                if attribute.use_.is_none() {
                    attribute.use_ = base_attribute.use_;
                }
                if decl_attribute.type_.is_none() {
                    decl_attribute.type_ = decl_base_attribute.type_;
                }
            }
            (TypeMode::Reference(_decl_base_attribute), TypeMode::Reference(_decl_attribute)) => {
                if attribute.use_.is_none() {
                    attribute.use_ = base_attribute.use_;
                }
            }
            _ => {
                return Err(Error::CannotRestrictReferenceAttribute {
                    attribute: *attribute_id,
                    base_attribute: *base_attribute_id,
                });
            }
        };

        Ok(())
    }

    fn expand_restricted_attributes(
        ctx: &mut XmlnsContextTransformerContext<'_>,
        child_attributes: FragmentIdx<AttributeDeclarationsFragment>,
        base_attributes: FragmentIdx<AttributeDeclarationsFragment>,
    ) -> Result<FragmentIdx<AttributeDeclarationsFragment>, <Self as XmlnsContextTransformer>::Error>
    {
        fn resolve_attr_name(
            ctx: &XmlnsContextTransformerContext,
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
                AttributeDeclarationId::Attribute(a) => Ok((*a, resolve_attr_name(ctx, a))),
                AttributeDeclarationId::AttributeGroupRef(_) => {
                    Err(Error::CannotHandleAttributeGroupRef {})
                }
            })
            .collect::<Result<BTreeMap<_, _>, <Self as XmlnsContextTransformer>::Error>>()?;
        let resolved_child_attributes = child_attribute_fragment
            .declarations
            .iter()
            .map(|a| match a {
                AttributeDeclarationId::Attribute(a) => Ok((*a, resolve_attr_name(ctx, a))),
                AttributeDeclarationId::AttributeGroupRef(_) => {
                    Err(Error::CannotHandleAttributeGroupRef {})
                }
            })
            .collect::<Result<BTreeMap<_, _>, <Self as XmlnsContextTransformer>::Error>>()?;

        let mut new_attribute_declarations = VecDeque::new();

        for base_attribute in base_attribute_fragment.declarations.iter() {
            let AttributeDeclarationId::Attribute(base_attribute) = base_attribute else {
                unreachable!("If attribute group reference was present, it would have been handled in the previous map.");
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

            Self::restrict_attribute(ctx, matching_child_attribute, base_attribute)?;

            new_attribute_declarations
                .push_back(AttributeDeclarationId::Attribute(*matching_child_attribute));
        }

        // Now we iterate through children attributes and only add those that have not been added yet because they were in the base.
        for child_attribute in child_attribute_fragment.declarations.iter() {
            let AttributeDeclarationId::Attribute(child_attribute) = child_attribute else {
                unreachable!("If attribute group reference was present, it would have been handled in the previous map.");
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

    fn expand_restriction(
        ctx: &mut XmlnsContextTransformerContext<'_>,
        child_fragment_idx: &FragmentIdx<RestrictionFragment>,
    ) -> Result<TransformChange, <Self as XmlnsContextTransformer>::Error> {
        let child_fragment = ctx.get_complex_fragment(child_fragment_idx).unwrap();

        let base = child_fragment.base.clone();

        if base == *xsn::ANY_TYPE {
            return Ok(TransformChange::Unchanged);
        }

        let base_fragment = ctx.get_named_type(&base).unwrap();

        let base_fragment = match base_fragment {
            TopLevelType::Complex(complex) => complex,
            TopLevelType::Simple(_) => {
                return Err(Error::BaseCannotBeSimpleType { base: base.clone() });
            }
        };

        let base_root_fragment = ctx
            .get_complex_fragment::<ComplexTypeRootFragment>(&base_fragment.root_fragment)
            .unwrap();

        let base_complex_content_id = match base_root_fragment.content {
            ComplexTypeModelId::ComplexContent(base_complex_content_id) => base_complex_content_id,
            ComplexTypeModelId::SimpleContent(_) => todo!(),
            ComplexTypeModelId::Other {
                particle: _,
                attr_decls: _,
            } => todo!(),
        };

        let base_content_fragment = ctx.get_complex_fragment(&base_complex_content_id).unwrap();

        let base_restriction_id = match base_content_fragment.content_fragment {
            ComplexContentChildId::Restriction(base_restriction_id) => base_restriction_id,
            ComplexContentChildId::Extension(_) => {
                return Err(Error::BaseCannotBeExtensionType { base: base.clone() });
            }
        };

        let base = ctx
            .get_complex_fragment(&base_restriction_id)
            .unwrap()
            .clone();

        let child = ctx.get_complex_fragment(child_fragment_idx).unwrap();

        let new_attribute_declarations = Self::expand_restricted_attributes(
            ctx,
            child.attribute_declarations,
            base.attribute_declarations,
        )?;

        let child_restriction = ctx.get_complex_fragment_mut(child_fragment_idx).unwrap();
        child_restriction.base = base.base.clone();
        child_restriction.attribute_declarations = new_attribute_declarations;

        Ok(TransformChange::Changed)
    }
}

impl XmlnsContextTransformer for ExpandRestrictionFragments {
    type Error = Error;

    fn transform(
        self,
        mut ctx: XmlnsContextTransformerContext<'_>,
    ) -> Result<TransformChange, Self::Error> {
        ctx.iter_complex_fragment_ids()
            .collect::<Vec<_>>()
            .into_iter()
            .map(|f| Self::expand_restriction(&mut ctx, &f))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::XmlnsContextExt;

    use super::*;
    use pretty_assertions::assert_eq;

    use xmlity::{ExpandedName, LocalName, XmlNamespace};
    use xsd::{xs, xsn};
    use xsd_fragments::XmlnsContext;

    #[test]
    fn basic_child_only_expand_restriction() {
        const TEST_NAMESPACE: XmlNamespace<'static> =
            XmlNamespace::new_dangerous("http://localhost");

        // ```xml
        // <xs:complexType name="ProductType">
        //   <xs:complexContent>
        //     <xs:restriction base="xs:anyType">
        //       <xs:sequence>
        //         <xs:element name="number" type="xs:integer" />
        //         <xs:element name="name" type="xs:string" />
        //       </xs:sequence>
        //     </xs:restriction>
        //   </xs:complexContent>
        // </xs:complexType>
        // ```
        let product_type = xs::types::TopLevelComplexType::builder()
            .name(LocalName::new_dangerous("ProductType"))
            .complex_type_model(Box::new(
                xs::ComplexContent::from(
                    xs::complex_content_items::ComplexContent::builder()
                        .child_1(
                            xs::types::ComplexRestrictionType::builder()
                                .base(xs::types::QName(xsn::ANY_TYPE.clone()))
                                .child_1(
                                    xs::types::complex_restriction_type_items::Child1::builder()
                                        .type_def_particle(Box::new(
                                            xs::Sequence::from(
                                                xs::types::ExplicitGroup::builder()
                                                    .nested_particle(vec![
                                                        xs::types::LocalElement::builder()
                                                            .name(LocalName::new_dangerous(
                                                                "number",
                                                            ))
                                                            .type_attribute(xs::types::QName(
                                                                xsn::INTEGER.clone(),
                                                            ))
                                                            .build()
                                                            .into(),
                                                        xs::types::LocalElement::builder()
                                                            .name(LocalName::new_dangerous("name"))
                                                            .type_attribute(xs::types::QName(
                                                                xsn::STRING.clone(),
                                                            ))
                                                            .build()
                                                            .into(),
                                                    ])
                                                    .build(),
                                            )
                                            .into(),
                                        ))
                                        .build()
                                        .into(),
                                )
                                .attr_decls(xs::groups::AttrDecls::builder().build().into())
                                .assertions(xs::groups::Assertions::builder().build().into())
                                .build()
                                .into(),
                        )
                        .build(),
                )
                .into(),
            ))
            .build()
            .into();

        // ```xml
        // <xs:complexType name="ShirtType">
        //   <xs:complexContent>
        //     <xs:restriction base="ProductType">
        //       <xs:sequence>
        //       <xs:element name="number" type="xs:integer" />
        //       </xs:sequence>
        //     </xs:restriction>
        //   </xs:complexContent>
        // </xs:complexType>
        // ```
        let derived_shirt_type = xs::types::TopLevelComplexType::builder()
            .name(LocalName::new_dangerous("ShirtType"))
            .complex_type_model(Box::new(
                xs::ComplexContent::from(
                    xs::complex_content_items::ComplexContent::builder()
                        .child_1(
                            xs::types::ComplexRestrictionType::builder()
                                .base(xs::types::QName(ExpandedName::new(
                                    LocalName::new_dangerous("ProductType"),
                                    Some(TEST_NAMESPACE.clone()),
                                )))
                                .child_1(
                                    xs::types::complex_restriction_type_items::Child1::builder()
                                        .type_def_particle(Box::new(
                                            xs::Sequence::from(
                                                xs::types::ExplicitGroup::builder()
                                                    .nested_particle(vec![
                                                        xs::types::LocalElement::builder()
                                                            .name(LocalName::new_dangerous(
                                                                "number",
                                                            ))
                                                            .type_attribute(xs::types::QName(
                                                                xsn::INTEGER.clone(),
                                                            ))
                                                            .build()
                                                            .into(),
                                                    ])
                                                    .build(),
                                            )
                                            .into(),
                                        ))
                                        .build()
                                        .into(),
                                )
                                .attr_decls(xs::groups::AttrDecls::builder().build().into())
                                .assertions(xs::groups::Assertions::builder().build().into())
                                .build()
                                .into(),
                        )
                        .build(),
                )
                .into(),
            ))
            .build()
            .into();

        // ```xml
        // <xs:complexType name="ShirtType">
        //   <xs:complexContent>
        //     <xs:restriction base="xs:anyType">
        //       <xs:sequence>
        //       <xs:element name="number" type="xs:integer" />
        //       </xs:sequence>
        //     </xs:restriction>
        //   </xs:complexContent>
        // </xs:complexType>
        // ```
        let expected_flattened_shirt_type: xs::ComplexType =
            xs::types::TopLevelComplexType::builder()
                .name(LocalName::new_dangerous("ShirtType"))
                .complex_type_model(Box::new(
                    xs::ComplexContent::from(
                        xs::complex_content_items::ComplexContent::builder()
                            .child_1(
                                xs::types::ComplexRestrictionType::builder()
                                    .base(xs::types::QName(xsn::ANY_TYPE.clone()))
                                    .child_1(
                                        xs::types::complex_restriction_type_items::Child1::builder(
                                        )
                                        .type_def_particle(Box::new(
                                            xs::Sequence::from(
                                                xs::types::ExplicitGroup::builder()
                                                    .nested_particle(vec![
                                                        xs::types::LocalElement::builder()
                                                            .name(LocalName::new_dangerous(
                                                                "number",
                                                            ))
                                                            .type_attribute(xs::types::QName(
                                                                xsn::INTEGER.clone(),
                                                            ))
                                                            .build()
                                                            .into(),
                                                    ])
                                                    .build(),
                                            )
                                            .into(),
                                        ))
                                        .build()
                                        .into(),
                                    )
                                    .attr_decls(xs::groups::AttrDecls::builder().build().into())
                                    .assertions(xs::groups::Assertions::builder().build().into())
                                    .build()
                                    .into(),
                            )
                            .build(),
                    )
                    .into(),
                ))
                .build()
                .into();

        let mut ctx = XmlnsContext::new();
        let ns = ctx.init_namespace(TEST_NAMESPACE);
        ns.import_top_level_complex_type(&product_type).unwrap();
        ns.import_top_level_complex_type(&derived_shirt_type)
            .unwrap();

        let transform_changed = ctx
            .context_transform(ExpandRestrictionFragments::new())
            .unwrap();

        assert_eq!(transform_changed, TransformChange::Changed);

        let transform_changed = ctx
            .context_transform(ExpandRestrictionFragments::new())
            .unwrap();

        assert_eq!(transform_changed, TransformChange::Unchanged);

        let ns = ctx.get_namespace(&TEST_NAMESPACE).unwrap();

        let actual_flattened_shirt_type = ns
            .export_top_level_complex_type(&LocalName::new_dangerous("ShirtType"))
            .unwrap()
            .unwrap();

        assert_eq!(expected_flattened_shirt_type, actual_flattened_shirt_type);
    }

    #[test]
    fn basic_attribute_only_expand_restriction() {
        const TEST_NAMESPACE: XmlNamespace<'static> =
            XmlNamespace::new_dangerous("http://localhost");

        // ```xml
        // <xs:complexType name="ProductType">
        //   <xs:complexContent>
        //     <xs:restriction base="xs:anyType">
        //       <xs:attribute name="number" type="xs:integer" use="optional" />
        //       <xs:attribute name="name" type="xs:string" use="required" />
        //     </xs:restriction>
        //   </xs:complexContent>
        // </xs:complexType>
        // ```
        let product_type =
            xs::types::TopLevelComplexType::builder()
                .name(LocalName::new_dangerous("ProductType"))
                .complex_type_model(Box::new(
                    xs::ComplexContent::from(
                        xs::complex_content_items::ComplexContent::builder()
                            .child_1(
                                xs::types::ComplexRestrictionType::builder()
                                    .base(xs::types::QName(xsn::ANY_TYPE.clone()))
                                    .attr_decls(
                                        xs::groups::AttrDecls::builder()
                                            .attribute(vec![
                                        xs::types::Attribute::builder()
                                            .name(LocalName::new_dangerous("number"))
                                            .type_(xs::types::QName(xsn::INTEGER.clone()))
                                            // .use_(xs::AttributeUseType::Optional)
                                            .use_(xs::types::attribute_items::UseValue::Optional)
                                            .build()
                                            .into(),
                                        xs::types::Attribute::builder()
                                            .name(LocalName::new_dangerous("name"))
                                            .type_(xs::types::QName(xsn::STRING.clone()))
                                            // .use_(xs::AttributeUseType::Required)
                                            .use_(xs::types::attribute_items::UseValue::Required)
                                            .build()
                                            .into(),
                                    ])
                                            .build()
                                            .into(),
                                    )
                                    .assertions(xs::groups::Assertions::builder().build().into())
                                    .build()
                                    .into(),
                            )
                            .build(),
                    )
                    .into(),
                ))
                .build()
                .into();

        // ```xml
        // <xs:complexType name="ShirtType">
        //   <xs:complexContent>
        //     <xs:restriction base="ProductType">
        //       <xs:attribute name="number" type="xs:integer" use="required" />
        //     </xs:restriction>
        //   </xs:complexContent>
        // </xs:complexType>
        // ```
        let derived_shirt_type = xs::types::TopLevelComplexType::builder()
            .name(LocalName::new_dangerous("ShirtType"))
            .complex_type_model(Box::new(
                xs::ComplexContent::from(
                    xs::complex_content_items::ComplexContent::builder()
                        .child_1(
                            xs::types::ComplexRestrictionType::builder()
                                .base(xs::types::QName(ExpandedName::new(
                                    LocalName::new_dangerous("ProductType"),
                                    Some(TEST_NAMESPACE.clone()),
                                )))
                                .attr_decls(
                                    xs::groups::AttrDecls::builder()
                                        .attribute(vec![xs::types::Attribute::builder()
                                            .name(LocalName::new_dangerous("number"))
                                            .type_(xs::types::QName(xsn::INTEGER.clone()))
                                            // .use_(xs::AttributeUseType::Required)
                                            .use_(xs::types::attribute_items::UseValue::Required)
                                            .build()
                                            .into()])
                                        .build()
                                        .into(),
                                )
                                .assertions(xs::groups::Assertions::builder().build().into())
                                .build()
                                .into(),
                        )
                        .build(),
                )
                .into(),
            ))
            .build()
            .into();

        // ```xml
        // <xs:complexType name="ShirtType">
        //   <xs:complexContent>
        //     <xs:restriction base="xs:anyType">
        //       <xs:attribute name="number" type="xs:integer" use="required" />
        //       <xs:attribute name="name" type="xs:string" use="required" />
        //     </xs:restriction>
        //   </xs:complexContent>
        // </xs:complexType>
        // ```
        let expected_flattened_shirt_type: xs::ComplexType =
            xs::types::TopLevelComplexType::builder()
                .name(LocalName::new_dangerous("ShirtType"))
                .complex_type_model(Box::new(
                    xs::ComplexContent::from(
                        xs::complex_content_items::ComplexContent::builder()
                            .child_1(
                                xs::types::ComplexRestrictionType::builder()
                                    .base(xs::types::QName(xsn::ANY_TYPE.clone()))
                                    .attr_decls(
                                        xs::groups::AttrDecls::builder()
                                            .attribute(vec![
                                            xs::types::Attribute::builder()
                                                .name(LocalName::new_dangerous("number"))
                                                .type_(xs::types::QName(xsn::INTEGER.clone()))
                                                // .use_(xs::AttributeUseType::Required)
                                                .use_(
                                                    xs::types::attribute_items::UseValue::Required,
                                                )
                                                .build()
                                                .into(),
                                            xs::types::Attribute::builder()
                                                .name(LocalName::new_dangerous("name"))
                                                .type_(xs::types::QName(xsn::STRING.clone()))
                                                // .use_(xs::AttributeUseType::Required)
                                                .use_(
                                                    xs::types::attribute_items::UseValue::Required,
                                                )
                                                .build()
                                                .into(),
                                        ])
                                            .build()
                                            .into(),
                                    )
                                    .assertions(xs::groups::Assertions::builder().build().into())
                                    .build()
                                    .into(),
                            )
                            .build(),
                    )
                    .into(),
                ))
                .build()
                .into();

        let mut ctx = XmlnsContext::new();
        let ns = ctx.init_namespace(TEST_NAMESPACE);

        ns.import_top_level_complex_type(&product_type).unwrap();
        ns.import_top_level_complex_type(&derived_shirt_type)
            .unwrap();

        let transform_changed = ctx
            .context_transform(ExpandRestrictionFragments::new())
            .unwrap();

        assert_eq!(transform_changed, TransformChange::Changed);

        let transform_changed = ctx
            .context_transform(ExpandRestrictionFragments::new())
            .unwrap();

        assert_eq!(transform_changed, TransformChange::Unchanged);

        let ns = ctx.get_namespace(&TEST_NAMESPACE).unwrap();

        let actual_flattened_shirt_type = ns
            .export_top_level_complex_type(&LocalName::new_dangerous("ShirtType"))
            .unwrap()
            .unwrap();

        assert_eq!(expected_flattened_shirt_type, actual_flattened_shirt_type);
    }
}
