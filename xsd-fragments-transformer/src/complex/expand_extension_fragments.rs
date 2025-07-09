use std::collections::BTreeMap;
use std::collections::VecDeque;

use xmlity::ExpandedName;

use crate::TransformChange;
use crate::XmlnsContextTransformer;
use crate::XmlnsContextTransformerContext;
use xsd::xsn;
use xsd_fragments::fragments::complex::AttributeDeclarationId;
use xsd_fragments::fragments::complex::AttributeDeclarationsFragment;
use xsd_fragments::fragments::complex::ComplexContentChildId;
use xsd_fragments::fragments::complex::ComplexContentFragment;
use xsd_fragments::fragments::complex::ComplexTypeModelId;
use xsd_fragments::fragments::complex::ComplexTypeRootFragment;
use xsd_fragments::fragments::complex::ExtensionFragment;
use xsd_fragments::fragments::complex::LocalAttributeFragment;
use xsd_fragments::fragments::complex::LocalAttributeFragmentTypeMode;
use xsd_fragments::fragments::complex::RestrictionFragment;
use xsd_fragments::fragments::complex::SequenceFragment;
use xsd_fragments::fragments::FragmentAccess;
use xsd_fragments::fragments::FragmentIdx;
use xsd_fragments::TopLevelType;

/// Expands restriction and extension fragments to their base fragments, with the modifications applied.
#[non_exhaustive]
pub struct ExpandExtensionFragments {}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Base {base} not found in the context")]
    BaseNotFound { base: ExpandedName<'static> },
    #[error("Base {base} is not a complex type")]
    BaseNotComplexType { base: ExpandedName<'static> },
    #[error("Base {base} is not a restriction of \"xs:anyType\"")]
    ExpandingNonAnyTypeRestriction { base: ExpandedName<'static> },
}

impl ExpandExtensionFragments {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {}
    }

    fn expand_attribute(
        ctx: &mut XmlnsContextTransformerContext<'_>,
        attribute: &FragmentIdx<LocalAttributeFragment>,
        base_attribute: &FragmentIdx<LocalAttributeFragment>,
    ) -> Result<(), <Self as XmlnsContextTransformer>::Error> {
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

    fn expand_expanded_attributes(
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
                AttributeDeclarationId::Attribute(a) => (*a, resolve_attr_name(ctx, a)),
                AttributeDeclarationId::AttributeGroupRef(_) => todo!(),
            })
            .collect::<BTreeMap<_, _>>();
        let resolved_child_attributes = child_attribute_fragment
            .declarations
            .iter()
            .map(|a| match a {
                AttributeDeclarationId::Attribute(a) => (*a, resolve_attr_name(ctx, a)),
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
        ctx: &mut XmlnsContextTransformerContext<'_>,
        child_complex_content_fragment_idx: &FragmentIdx<ComplexContentFragment>,
    ) -> Result<TransformChange, <Self as XmlnsContextTransformer>::Error> {
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

        if base == *xsn::ANY_TYPE {
            return Ok(TransformChange::Unchanged);
        }

        let base_fragment = ctx
            .get_named_type(&base)
            .ok_or(Error::BaseNotFound { base: base.clone() })?;

        let base_fragment = match base_fragment {
            TopLevelType::Complex(complex) => complex,
            TopLevelType::Simple(_) => {
                return Err(Error::BaseNotComplexType { base: base.clone() });
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
                                base_extension_fragment.content_fragment,
                                base_extension_fragment.base.clone(),
                                base_extension_fragment.attribute_declarations,
                            )
                        }
                        ComplexContentChildId::Restriction(fragment_idx) => {
                            let base_restriction_fragment = ctx
                                .get_complex_fragment::<RestrictionFragment>(&fragment_idx)
                                .unwrap();

                            if base_restriction_fragment.base != *xsn::ANY_TYPE {
                                return Err(Error::ExpandingNonAnyTypeRestriction {
                                    base: base_restriction_fragment.base.clone(),
                                });
                            }

                            (
                                base_restriction_fragment.content_fragment,
                                base_restriction_fragment.base.clone(),
                                base_restriction_fragment.attribute_declarations,
                            )
                        }
                    }
                }
            };

        let child_attributes = child_fragment.attribute_declarations;

        let new_content_fragment = child_fragment
            .content_fragment
            .map(|child_content_content_fragment_id| {
                let Some(base_content_content_fragment_id) = base_content_content_fragment_id
                else {
                    return child_content_content_fragment_id;
                };

                let new_content_fragment = SequenceFragment {
                    id: None,
                    max_occurs: None,
                    min_occurs: None,
                    fragments: {
                        let mut fragments = VecDeque::with_capacity(2);
                        fragments.push_back(base_content_content_fragment_id.into());
                        fragments.push_back(child_content_content_fragment_id.into());
                        fragments
                    },
                };

                let ns = &mut ctx
                    .xmlns_context
                    .namespaces
                    .get_mut(&child_complex_content_fragment_idx.namespace_idx())
                    .unwrap();

                let new_content_fragment = ns.complex_type.push_fragment(new_content_fragment);

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

        let ns = &mut ctx
            .xmlns_context
            .namespaces
            .get_mut(&child_complex_content_fragment_idx.namespace_idx())
            .unwrap();

        let new_child_content = ns.complex_type.push_fragment(new_child_content);

        ctx.get_complex_fragment_mut(child_complex_content_fragment_idx)
            .unwrap()
            .content_fragment = ComplexContentChildId::Restriction(new_child_content);

        Ok(TransformChange::Changed)
    }
}

impl XmlnsContextTransformer for ExpandExtensionFragments {
    type Error = Error;

    fn transform(
        self,
        mut ctx: XmlnsContextTransformerContext<'_>,
    ) -> Result<TransformChange, Self::Error> {
        ctx.iter_complex_fragment_ids()
            .collect::<Vec<_>>()
            .into_iter()
            .map(|f| Self::expand_extension(&mut ctx, &f))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::XmlnsContextExt;

    use super::*;
    use pretty_assertions::assert_eq;

    use xmlity::{ExpandedName, LocalName, XmlNamespace};
    use xsd::xs;
    use xsd::xsn;
    use xsd_fragments::XmlnsContext;

    #[test]
    fn basic_child_only_expand_extension() {
        const TEST_NAMESPACE: XmlNamespace<'static> =
            XmlNamespace::new_dangerous("http://localhost");

        let parent_seq = xs::Sequence(Box::new(
            xs::types::ExplicitGroup::builder()
                .nested_particle(vec![
                    xs::types::LocalElement::builder()
                        .name(LocalName::new_dangerous("number"))
                        .type_attribute(xs::types::QName(xsn::INTEGER.clone()))
                        .build()
                        .into(),
                    xs::types::LocalElement::builder()
                        .name(LocalName::new_dangerous("name"))
                        .type_attribute(xs::types::QName(xsn::STRING.clone()))
                        .build()
                        .into(),
                ])
                .build()
                .into(),
        ));

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
        let product_type = xs::types::TopLevelComplexType::builder()
            .name(LocalName::new_dangerous("ProductType"))
            .complex_type_model(Box::new(
                xs::ComplexContent::builder()
                    .child_1(
                        xs::types::ComplexRestrictionType::builder()
                            .base(xs::types::QName(xsn::ANY_TYPE.clone()))
                            .child_1(
                                xs::types::complex_restriction_type_items::Child1::builder()
                                    .type_def_particle(Box::new(parent_seq.clone().into()))
                                    .build()
                                    .into(),
                            )
                            .attr_decls(xs::groups::AttrDecls::builder().build().into())
                            .assertions(xs::groups::Assertions::builder().build().into())
                            .build()
                            .into(),
                    )
                    .build()
                    .into(),
            ))
            .build()
            .into();

        let child_choice = xs::Choice(Box::new(
            xs::types::ExplicitGroup::builder()
                .max_occurs(
                    xs::types::AllNNI::from(
                        xs::types::all_nni_items::all_nni_variants::Variant0::Unbounded,
                    )
                    .into(),
                )
                .nested_particle(vec![
                    xs::types::LocalElement::builder()
                        .name(LocalName::new_dangerous("size"))
                        .type_attribute(xs::types::QName(xsn::INTEGER.clone()))
                        .build()
                        .into(),
                    xs::types::LocalElement::builder()
                        .name(LocalName::new_dangerous("color"))
                        .type_attribute(xs::types::QName(xsn::STRING.clone()))
                        .build()
                        .into(),
                ])
                .build()
                .into(),
        ));

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
        let derived_shirt_type = xs::types::TopLevelComplexType::builder()
            .name(LocalName::new_dangerous("ShirtType"))
            .complex_type_model(Box::new(
                xs::ComplexContent::builder()
                    .child_1(
                        xs::types::ExtensionType::builder()
                            .base(xs::types::QName(ExpandedName::new(
                                LocalName::new_dangerous("ProductType"),
                                Some(TEST_NAMESPACE.clone()),
                            )))
                            .type_def_particle(Box::new(child_choice.clone().into()))
                            .attr_decls(xs::groups::AttrDecls::builder().build().into())
                            .assertions(xs::groups::Assertions::builder().build().into())
                            .build()
                            .into(),
                    )
                    .build()
                    .into(),
            ))
            .build()
            .into();

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
        let expected_flattened_shirt_type: xs::ComplexType =
            xs::types::TopLevelComplexType::builder()
                .name(LocalName::new_dangerous("ShirtType"))
                .complex_type_model(Box::new(
                    xs::ComplexContent::builder()
                        .child_1(
                            xs::types::ComplexRestrictionType::builder()
                                .base(xs::types::QName(xsn::ANY_TYPE.clone()))
                                .child_1(
                                    xs::types::complex_restriction_type_items::Child1::builder()
                                        .type_def_particle(Box::new(
                                            xs::Sequence(
                                                xs::types::ExplicitGroup::builder()
                                                    .nested_particle(vec![
                                                        parent_seq.into(),
                                                        child_choice.into(),
                                                    ])
                                                    .build()
                                                    .into(),
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
                        .build()
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
            .context_transform(ExpandExtensionFragments::new())
            .unwrap();

        assert_eq!(transform_changed, TransformChange::Changed);

        let transform_changed = ctx
            .context_transform(ExpandExtensionFragments::new())
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
    fn basic_attribute_only_expand_extension() {
        const TEST_NAMESPACE: XmlNamespace<'static> =
            XmlNamespace::new_dangerous("http://localhost");

        // <xs:complexType name="ProductType">
        //   <xs:complexContent>
        //     <xs:restriction base="xs:anyType">
        //       <xs:attribute name="number" type="xs:integer" use="required" />
        //       <xs:attribute name="name" type="xs:string" use="required" />
        //     </xs:restriction>
        //   </xs:complexContent>
        // </xs:complexType>
        let product_type = xs::types::TopLevelComplexType::builder()
            .name(LocalName::new_dangerous("ProductType"))
            .complex_type_model(Box::new(
                xs::ComplexContent::builder()
                    .child_1(
                        xs::types::ComplexRestrictionType::builder()
                            .base(xs::types::QName(xsn::ANY_TYPE.clone()))
                            .attr_decls(
                                xs::groups::AttrDecls::builder()
                                    .attribute(vec![
                                        xs::types::Attribute::builder()
                                            .name(LocalName::new_dangerous("number"))
                                            .type_(xs::types::QName(xsn::INTEGER.clone()))
                                            .use_(xs::types::attribute_items::UseValue::Required)
                                            .build()
                                            .into(),
                                        xs::types::Attribute::builder()
                                            .name(LocalName::new_dangerous("name"))
                                            .type_(xs::types::QName(xsn::STRING.clone()))
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
                    .build()
                    .into(),
            ))
            .build()
            .into();

        // <xs:complexType name="ShirtType">
        //   <xs:complexContent>
        //     <xs:extension base="ProductType">
        //       <xs:attribute name="number" type="xs:integer" use="optional" />
        //       <xs:attribute name="color" type="xs:string" use="required" />
        //     </xs:extension>
        //   </xs:complexContent>
        // </xs:complexType>
        let derived_shirt_type = xs::types::TopLevelComplexType::builder()
            .name(LocalName::new_dangerous("ShirtType"))
            .complex_type_model(Box::new(
                xs::ComplexContent::builder()
                    .child_1(
                        xs::types::ExtensionType::builder()
                            .base(xs::types::QName(ExpandedName::new(
                                LocalName::new_dangerous("ProductType"),
                                Some(TEST_NAMESPACE.clone()),
                            )))
                            .attr_decls(
                                xs::groups::AttrDecls::builder()
                                    .attribute(vec![
                                        xs::types::Attribute::builder()
                                            .name(LocalName::new_dangerous("number"))
                                            .type_(xs::types::QName(xsn::INTEGER.clone()))
                                            .use_(xs::types::attribute_items::UseValue::Optional)
                                            .build()
                                            .into(),
                                        xs::types::Attribute::builder()
                                            .name(LocalName::new_dangerous("color"))
                                            .type_(xs::types::QName(xsn::STRING.clone()))
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
                    .build()
                    .into(),
            ))
            .build()
            .into();

        // <xs:complexType name="ShirtType">
        //   <xs:complexContent>
        //     <xs:restriction base="xs:anyType">
        //       <xs:attribute name="number" type="xs:integer" use="optional" />
        //       <xs:attribute name="name" type="xs:string" use="required" />
        //       <xs:attribute name="color" type="xs:string" use="required" />
        //     </xs:restriction>
        //   </xs:complexContent>
        // </xs:complexType>
        let expected_flattened_shirt_type: xs::ComplexType =
            xs::types::TopLevelComplexType::builder()
                .name(LocalName::new_dangerous("ShirtType"))
                .complex_type_model(Box::new(
                    xs::ComplexContent::builder()
                        .child_1(
                            xs::types::ComplexRestrictionType::builder()
                                .base(xs::types::QName(xsn::ANY_TYPE.clone()))
                                .attr_decls(
                                    xs::groups::AttrDecls::builder()
                                        .attribute(vec![
                                            xs::types::Attribute::builder()
                                                .name(LocalName::new_dangerous("number"))
                                                .type_(xs::types::QName(xsn::INTEGER.clone()))
                                                .use_(
                                                    xs::types::attribute_items::UseValue::Optional,
                                                )
                                                .build()
                                                .into(),
                                            xs::types::Attribute::builder()
                                                .name(LocalName::new_dangerous("name"))
                                                .type_(xs::types::QName(xsn::STRING.clone()))
                                                .use_(
                                                    xs::types::attribute_items::UseValue::Required,
                                                )
                                                .build()
                                                .into(),
                                            xs::types::Attribute::builder()
                                                .name(LocalName::new_dangerous("color"))
                                                .type_(xs::types::QName(xsn::STRING.clone()))
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
                        .build()
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
            .context_transform(ExpandExtensionFragments::new())
            .unwrap();

        assert_eq!(transform_changed, TransformChange::Changed);

        let transform_changed = ctx
            .context_transform(ExpandExtensionFragments::new())
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
    fn expand_extension_type_element_no_fragment() {
        const TEST_NAMESPACE: XmlNamespace<'static> =
            XmlNamespace::new_dangerous("http://localhost");

        // <xs:complexType name="Block">
        //     <xs:choice minOccurs="0" maxOccurs="unbounded">
        //     <xs:group ref="block"/>
        //     <xs:element ref="form"/>
        //     <xs:group ref="misc"/>
        //     </xs:choice>
        // </xs:complexType>
        let block = xs::types::TopLevelComplexType::builder()
            .name(LocalName::new_dangerous("Block"))
            .complex_type_model(Box::new(
                xs::ComplexContent::builder()
                    .child_1(
                        xs::types::ComplexRestrictionType::builder()
                            .base(xs::types::QName(xsn::ANY_TYPE.clone()))
                            .child_1(
                                xs::types::complex_restriction_type_items::Child1::builder()
                                    .type_def_particle(Box::new(
                                        xs::Choice(Box::new(
                                            xs::types::ExplicitGroup::builder()
                                                .min_occurs(0)
                                                .max_occurs(xs::types::AllNNI::from(xs::types::all_nni_items::all_nni_variants::Variant0::Unbounded).into())
                                                .nested_particle(vec![
                                                    xs::types::GroupRef::builder()
                                                        .ref_(xs::types::QName(ExpandedName::new(
                                                            LocalName::new_dangerous("block"),
                                                            Some(XmlNamespace::XHTML),
                                                        )))
                                                        .build()
                                                        .into(),
                                                    xs::types::LocalElement::builder()
                                                        .ref_(xs::types::QName(ExpandedName::new(
                                                            LocalName::new_dangerous("form"),
                                                            Some(XmlNamespace::XHTML),
                                                        )))
                                                        .build()
                                                        .into(),
                                                    xs::types::GroupRef::builder()
                                                        .ref_(xs::types::QName(ExpandedName::new(
                                                            LocalName::new_dangerous("misc"),
                                                            Some(XmlNamespace::XHTML),
                                                        )))
                                                        .build()
                                                        .into(),
                                                ])
                                                .build()
                                                .into(),
                                        ))
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
                    .build()
                    .into(),
            ))
            .build()
            .into();

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
        let noscript: xs::Element = xs::types::TopLevelElement::builder()
            .name(LocalName::new_dangerous("noscript"))
            .type_(
                xs::types::LocalComplexType::builder()
                    .complex_type_model(Box::new(
                        xs::ComplexContent::builder()
                            .child_1(
                                xs::types::ExtensionType::builder()
                                    .base(xs::types::QName(ExpandedName::new(
                                        LocalName::new_dangerous("Block"),
                                        Some(XmlNamespace::XHTML),
                                    )))
                                    .attr_decls(
                                        xs::groups::AttrDecls::builder()
                                            .attribute(vec![
                                            // xs::AttributeGroupRefType::builder()
                                            // .ref_(xs::types::QName(ExpandedName::new(
                                            //     LocalName::new_dangerous("attrs"),
                                            //     Some(XmlNamespace::XHTML),
                                            // )))
                                            // .build()
                                            // .into()
                                            ])
                                            .build()
                                            .into(),
                                    )
                                    .assertions(xs::groups::Assertions::builder().build().into())
                                    .build()
                                    .into(),
                            )
                            .build()
                            .into(),
                    ))
                    .build()
                    .into(),
            )
            .build()
            .into();

        let expected_flattened_noscript: xs::Element = xs::types::TopLevelElement::builder()
            .name(LocalName::new_dangerous("noscript"))
            .type_(
                xs::types::LocalComplexType::builder()
                    .complex_type_model(Box::new(
                        xs::ComplexContent::builder()
                            .child_1(
                                xs::types::ComplexRestrictionType::builder()
                                    .base(xs::types::QName(xsn::ANY_TYPE.clone()))
                                    .child_1(
                                        xs::types::complex_restriction_type_items::Child1::builder(
                                        )
                                        .type_def_particle(Box::new(
                                            xs::Choice(Box::new(
                                                xs::types::ExplicitGroup::builder()
                                                    .min_occurs(0)
                                                    .max_occurs(xs::types::AllNNI::from(xs::types::all_nni_items::all_nni_variants::Variant0::Unbounded).into())
                                                    .nested_particle(vec![
                                                        xs::types::GroupRef::builder()
                                                            .ref_(xs::types::QName(
                                                                ExpandedName::new(
                                                                    LocalName::new_dangerous(
                                                                        "block",
                                                                    ),
                                                                    Some(XmlNamespace::XHTML),
                                                                ),
                                                            ))
                                                            .build()
                                                            .into(),
                                                        xs::types::LocalElement::builder()
                                                            .ref_(xs::types::QName(
                                                                ExpandedName::new(
                                                                    LocalName::new_dangerous(
                                                                        "form",
                                                                    ),
                                                                    Some(XmlNamespace::XHTML),
                                                                ),
                                                            ))
                                                            .build()
                                                            .into(),
                                                        xs::types::GroupRef::builder()
                                                            .ref_(xs::types::QName(
                                                                ExpandedName::new(
                                                                    LocalName::new_dangerous(
                                                                        "misc",
                                                                    ),
                                                                    Some(XmlNamespace::XHTML),
                                                                ),
                                                            ))
                                                            .build()
                                                            .into(),
                                                    ])
                                                    .build()
                                                    .into(),
                                            ))
                                            .into(),
                                        ))
                                        .build()
                                        .into(),
                                    )
                                    .attr_decls(
                                        xs::groups::AttrDecls::builder()
                                            .attribute(vec![
                                                // xs::AttributeGroupRefType::builder()
                                                // .ref_(xs::types::QName(ExpandedName::new(
                                                //     LocalName::new_dangerous("attrs"),
                                                //     Some(XmlNamespace::XHTML),
                                                // )))
                                                // .build()
                                                // .into()
                                            ])
                                            .build()
                                            .into(),
                                    )
                                    .assertions(xs::groups::Assertions::builder().build().into())
                                    .build()
                                    .into(),
                            )
                            .build()
                            .into(),
                    ))
                    .build()
                    .into(),
            )
            .build()
            .into();

        let mut ctx = XmlnsContext::new();
        let ns = ctx.init_namespace(TEST_NAMESPACE);
        ns.import_top_level_complex_type(&block).unwrap();
        ns.import_top_level_element(&noscript).unwrap();

        let transform_changed = ctx
            .context_transform(ExpandExtensionFragments::new())
            .unwrap();

        assert_eq!(transform_changed, TransformChange::Changed);

        let transform_changed = ctx
            .context_transform(ExpandExtensionFragments::new())
            .unwrap();

        assert_eq!(transform_changed, TransformChange::Unchanged);

        let ns = ctx.get_namespace(&TEST_NAMESPACE).unwrap();

        let actual_flattened_noscript = ns
            .export_top_level_element(&LocalName::new_dangerous("noscript"))
            .unwrap()
            .unwrap();

        assert_eq!(expected_flattened_noscript, actual_flattened_noscript);
    }

    #[test]
    fn expand_xhtml_a() {
        const TEST_NAMESPACE: XmlNamespace<'static> =
            XmlNamespace::new_dangerous("http://localhost");

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
        let a_content = xs::types::TopLevelComplexType::builder()
            .name(LocalName::new_dangerous("a.content"))
            .mixed(true)
            .complex_type_model(Box::new(
                xs::ComplexContent::builder()
                    .child_1(
                        xs::types::ComplexRestrictionType::builder()
                            .base(xs::types::QName(xsn::ANY_TYPE.clone()))
                            .child_1(
                                xs::types::complex_restriction_type_items::Child1::builder()
                                    .type_def_particle(Box::new(
                                        xs::Choice(Box::new(
                                            xs::types::ExplicitGroup::builder()
                                                .min_occurs(0)
                                                .max_occurs(xs::types::AllNNI::from(xs::types::all_nni_items::all_nni_variants::Variant0::Unbounded).into())
                                                .nested_particle(vec![
                                                    xs::types::GroupRef::builder()
                                                        .ref_(xs::types::QName(ExpandedName::new(
                                                            LocalName::new_dangerous("special"),
                                                            Some(XmlNamespace::XHTML),
                                                        )))
                                                        .build()
                                                        .into(),
                                                    xs::types::GroupRef::builder()
                                                        .ref_(xs::types::QName(ExpandedName::new(
                                                            LocalName::new_dangerous("fontstyle"),
                                                            Some(XmlNamespace::XHTML),
                                                        )))
                                                        .build()
                                                        .into(),
                                                    xs::types::GroupRef::builder()
                                                        .ref_(xs::types::QName(ExpandedName::new(
                                                            LocalName::new_dangerous("phrase"),
                                                            Some(XmlNamespace::XHTML),
                                                        )))
                                                        .build()
                                                        .into(),
                                                    xs::types::GroupRef::builder()
                                                        .ref_(xs::types::QName(ExpandedName::new(
                                                            LocalName::new_dangerous(
                                                                "inline.forms",
                                                            ),
                                                            Some(XmlNamespace::XHTML),
                                                        )))
                                                        .build()
                                                        .into(),
                                                    xs::types::GroupRef::builder()
                                                        .ref_(xs::types::QName(ExpandedName::new(
                                                            LocalName::new_dangerous("misc.inline"),
                                                            Some(XmlNamespace::XHTML),
                                                        )))
                                                        .build()
                                                        .into(),
                                                ])
                                                .build()
                                                .into(),
                                        ))
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
                    .build()
                    .into(),
            ))
            .build()
            .into();

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
        let a: xs::Element = xs::types::TopLevelElement::builder()
            .name(LocalName::new_dangerous("a"))
            .type_(
                xs::types::LocalComplexType::builder()
                    .mixed(true)
                    .complex_type_model(Box::new(
                        xs::ComplexContent::builder()
                            .child_1(
                                xs::types::ExtensionType::builder()
                                    .base(xs::types::QName(ExpandedName::new(
                                        LocalName::new_dangerous("a.content"),
                                        Some(XmlNamespace::XHTML),
                                    )))
                                    .attr_decls(
                                        xs::groups::AttrDecls::builder()
                                            .attribute(vec![
                                                xs::types::Attribute::builder()
                                                    .name(LocalName::new_dangerous("charset"))
                                                    .type_(xs::types::QName(ExpandedName::new(
                                                        LocalName::new_dangerous("Charset"),
                                                        Some(XmlNamespace::XHTML),
                                                    )))
                                                    .build()
                                                    .into(),
                                                xs::types::Attribute::builder()
                                                    .name(LocalName::new_dangerous("type"))
                                                    .type_(xs::types::QName(ExpandedName::new(
                                                        LocalName::new_dangerous("ContentType"),
                                                        Some(XmlNamespace::XHTML),
                                                    )))
                                                    .build()
                                                    .into(),
                                                xs::types::Attribute::builder()
                                                    .name(LocalName::new_dangerous("name"))
                                                    .type_(xs::types::QName(xsn::NMTOKEN.clone()))
                                                    .build()
                                                    .into(),
                                                xs::types::Attribute::builder()
                                                    .name(LocalName::new_dangerous("href"))
                                                    .type_(xs::types::QName(ExpandedName::new(
                                                        LocalName::new_dangerous("URI"),
                                                        Some(XmlNamespace::XHTML),
                                                    )))
                                                    .build()
                                                    .into(),
                                                xs::types::Attribute::builder()
                                                    .name(LocalName::new_dangerous("hreflang"))
                                                    .type_(xs::types::QName(ExpandedName::new(
                                                        LocalName::new_dangerous("LanguageCode"),
                                                        Some(XmlNamespace::XHTML),
                                                    )))
                                                    .build()
                                                    .into(),
                                                xs::types::Attribute::builder()
                                                    .name(LocalName::new_dangerous("rel"))
                                                    .type_(xs::types::QName(ExpandedName::new(
                                                        LocalName::new_dangerous("LinkTypes"),
                                                        Some(XmlNamespace::XHTML),
                                                    )))
                                                    .build()
                                                    .into(),
                                                xs::types::Attribute::builder()
                                                    .name(LocalName::new_dangerous("rev"))
                                                    .type_(xs::types::QName(ExpandedName::new(
                                                        LocalName::new_dangerous("LinkTypes"),
                                                        Some(XmlNamespace::XHTML),
                                                    )))
                                                    .build()
                                                    .into(),
                                                xs::types::Attribute::builder()
                                                    .name(LocalName::new_dangerous("shape"))
                                                    .default("rect".to_string())
                                                    .type_(xs::types::QName(ExpandedName::new(
                                                        LocalName::new_dangerous("Shape"),
                                                        Some(XmlNamespace::XHTML),
                                                    )))
                                                    .build()
                                                    .into(),
                                                xs::types::Attribute::builder()
                                                    .name(LocalName::new_dangerous("coords"))
                                                    .type_(xs::types::QName(ExpandedName::new(
                                                        LocalName::new_dangerous("Coords"),
                                                        Some(XmlNamespace::XHTML),
                                                    )))
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
                            .build()
                            .into(),
                    ))
                    .build()
                    .into(),
            )
            .build()
            .into();

        let expected_flattened_a: xs::Element = xs::types::TopLevelElement::builder()
            .name(LocalName::new_dangerous("a"))
            .type_(
                xs::types::LocalComplexType::builder()
                    .mixed(true)
                    .complex_type_model(Box::new(
                        xs::ComplexContent::builder()
                            .child_1(
                                xs::types::ComplexRestrictionType::builder()
                                    .base(xs::types::QName(xsn::ANY_TYPE.clone()))
                                    .child_1(
                                        xs::types::complex_restriction_type_items::Child1::builder(
                                        )
                                        .type_def_particle(Box::new(
                                            xs::Choice(
                                                xs::types::ExplicitGroup::builder()
                                                    .min_occurs(0)
                                                    .max_occurs(xs::types::AllNNI::from(xs::types::all_nni_items::all_nni_variants::Variant0::Unbounded).into())
                                                    .nested_particle(vec![
                                                        xs::types::GroupRef::builder()
                                                            .ref_(xs::types::QName(
                                                                ExpandedName::new(
                                                                    LocalName::new_dangerous(
                                                                        "special",
                                                                    ),
                                                                    Some(XmlNamespace::XHTML),
                                                                ),
                                                            ))
                                                            .build()
                                                            .into(),
                                                        xs::types::GroupRef::builder()
                                                            .ref_(xs::types::QName(
                                                                ExpandedName::new(
                                                                    LocalName::new_dangerous(
                                                                        "fontstyle",
                                                                    ),
                                                                    Some(XmlNamespace::XHTML),
                                                                ),
                                                            ))
                                                            .build()
                                                            .into(),
                                                        xs::types::GroupRef::builder()
                                                            .ref_(xs::types::QName(
                                                                ExpandedName::new(
                                                                    LocalName::new_dangerous(
                                                                        "phrase",
                                                                    ),
                                                                    Some(XmlNamespace::XHTML),
                                                                ),
                                                            ))
                                                            .build()
                                                            .into(),
                                                        xs::types::GroupRef::builder()
                                                            .ref_(xs::types::QName(
                                                                ExpandedName::new(
                                                                    LocalName::new_dangerous(
                                                                        "inline.forms",
                                                                    ),
                                                                    Some(XmlNamespace::XHTML),
                                                                ),
                                                            ))
                                                            .build()
                                                            .into(),
                                                        xs::types::GroupRef::builder()
                                                            .ref_(xs::types::QName(
                                                                ExpandedName::new(
                                                                    LocalName::new_dangerous(
                                                                        "misc.inline",
                                                                    ),
                                                                    Some(XmlNamespace::XHTML),
                                                                ),
                                                            ))
                                                            .build()
                                                            .into(),
                                                    ])
                                                    .build()
                                                    .into(),
                                            )
                                            .into(),
                                        ))
                                        .build()
                                        .into(),
                                    )
                                    .attr_decls(
                                        xs::groups::AttrDecls::builder()
                                            .attribute(vec![
                                                xs::types::Attribute::builder()
                                                    .name(LocalName::new_dangerous("charset"))
                                                    .type_(xs::types::QName(ExpandedName::new(
                                                        LocalName::new_dangerous("Charset"),
                                                        Some(XmlNamespace::XHTML),
                                                    )))
                                                    .build()
                                                    .into(),
                                                xs::types::Attribute::builder()
                                                    .name(LocalName::new_dangerous("type"))
                                                    .type_(xs::types::QName(ExpandedName::new(
                                                        LocalName::new_dangerous("ContentType"),
                                                        Some(XmlNamespace::XHTML),
                                                    )))
                                                    .build()
                                                    .into(),
                                                xs::types::Attribute::builder()
                                                    .name(LocalName::new_dangerous("name"))
                                                    .type_(xs::types::QName(xsn::NMTOKEN.clone()))
                                                    .build()
                                                    .into(),
                                                xs::types::Attribute::builder()
                                                    .name(LocalName::new_dangerous("href"))
                                                    .type_(xs::types::QName(ExpandedName::new(
                                                        LocalName::new_dangerous("URI"),
                                                        Some(XmlNamespace::XHTML),
                                                    )))
                                                    .build()
                                                    .into(),
                                                xs::types::Attribute::builder()
                                                    .name(LocalName::new_dangerous("hreflang"))
                                                    .type_(xs::types::QName(ExpandedName::new(
                                                        LocalName::new_dangerous("LanguageCode"),
                                                        Some(XmlNamespace::XHTML),
                                                    )))
                                                    .build()
                                                    .into(),
                                                xs::types::Attribute::builder()
                                                    .name(LocalName::new_dangerous("rel"))
                                                    .type_(xs::types::QName(ExpandedName::new(
                                                        LocalName::new_dangerous("LinkTypes"),
                                                        Some(XmlNamespace::XHTML),
                                                    )))
                                                    .build()
                                                    .into(),
                                                xs::types::Attribute::builder()
                                                    .name(LocalName::new_dangerous("rev"))
                                                    .type_(xs::types::QName(ExpandedName::new(
                                                        LocalName::new_dangerous("LinkTypes"),
                                                        Some(XmlNamespace::XHTML),
                                                    )))
                                                    .build()
                                                    .into(),
                                                xs::types::Attribute::builder()
                                                    .name(LocalName::new_dangerous("shape"))
                                                    .default("rect".to_string())
                                                    .type_(xs::types::QName(ExpandedName::new(
                                                        LocalName::new_dangerous("Shape"),
                                                        Some(XmlNamespace::XHTML),
                                                    )))
                                                    .build()
                                                    .into(),
                                                xs::types::Attribute::builder()
                                                    .name(LocalName::new_dangerous("coords"))
                                                    .type_(xs::types::QName(ExpandedName::new(
                                                        LocalName::new_dangerous("Coords"),
                                                        Some(XmlNamespace::XHTML),
                                                    )))
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
                            .build()
                            .into(),
                    ))
                    .build()
                    .into(),
            )
            .build()
            .into();

        let mut ctx = XmlnsContext::new();
        let ns = ctx.init_namespace(TEST_NAMESPACE);

        ns.import_top_level_complex_type(&a_content).unwrap();
        ns.import_top_level_element(&a).unwrap();

        let transform_changed = ctx
            .context_transform(ExpandExtensionFragments::new())
            .unwrap();

        assert_eq!(transform_changed, TransformChange::Changed);

        let transform_changed = ctx
            .context_transform(ExpandExtensionFragments::new())
            .unwrap();

        assert_eq!(transform_changed, TransformChange::Unchanged);

        let ns = ctx.get_namespace(&TEST_NAMESPACE).unwrap();

        let actual_flattened_a = ns
            .export_top_level_element(&LocalName::new_dangerous("a"))
            .unwrap()
            .unwrap();

        assert_eq!(expected_flattened_a, actual_flattened_a);
    }
}
