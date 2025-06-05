use std::collections::BTreeMap;
use std::collections::VecDeque;
use std::ops::Deref;

use xmlity::ExpandedName;

use crate::complex::AttributeDeclarationId;
use crate::complex::ComplexContentChildId;
use crate::complex::ComplexTypeModelId;
use crate::complex::ComplexTypeRootFragment;
use crate::complex::FragmentIdx;
use crate::complex::LocalAttributeFragment;
use crate::complex::LocalAttributeFragmentTypeMode;
use crate::complex::RestrictionFragment;
use crate::transformers::Context;
use crate::transformers::TransformChange;
use crate::transformers::XmlnsContextTransformer;
use crate::TopLevelType;
use xsd::schema_names as xsn;

/// Expands restriction and extension fragments to their base fragments, with the modifications applied.
#[non_exhaustive]
pub struct ExpandRestrictionFragments {}

impl ExpandRestrictionFragments {
    pub fn new() -> Self {
        Self {}
    }

    fn restrict_attribute(
        ctx: &mut Context<'_>,
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

    fn expand_restricted_attributes<'a>(
        ctx: &mut Context<'_>,
        child_attributes: impl Iterator<Item = &'a AttributeDeclarationId> + Clone,
        base_attributes: impl Iterator<Item = &'a AttributeDeclarationId> + Clone,
    ) -> Result<VecDeque<AttributeDeclarationId>, ()> {
        fn resolve_attr_name(
            ctx: &Context,
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

        let resolved_base_attributes = base_attributes
            .clone()
            .map(|a| match a {
                AttributeDeclarationId::Attribute(a) => (*a, resolve_attr_name(&ctx, a)),
                AttributeDeclarationId::AttributeGroupRef(_) => todo!(),
            })
            .collect::<BTreeMap<_, _>>();
        let resolved_child_attributes = child_attributes
            .clone()
            .map(|a| match a {
                AttributeDeclarationId::Attribute(a) => (*a, resolve_attr_name(&ctx, a)),
                AttributeDeclarationId::AttributeGroupRef(_) => todo!(),
            })
            .collect::<BTreeMap<_, _>>();

        let mut new_attribute_declarations = VecDeque::new();

        for base_attribute in base_attributes.clone() {
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

            Self::restrict_attribute(ctx, matching_child_attribute, base_attribute)?;

            new_attribute_declarations
                .push_back(AttributeDeclarationId::Attribute(*matching_child_attribute));
        }

        // Now we iterate through children attributes and only add those that have not been added yet because they were in the base.
        for child_attribute in child_attributes.clone() {
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

        Ok(new_attribute_declarations)
    }

    fn expand_restriction(
        ctx: &mut Context<'_>,
        child_fragment_idx: &FragmentIdx<RestrictionFragment>,
    ) -> Result<TransformChange, ()> {
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

        let ComplexTypeModelId::ComplexContent(base_complex_content_id) =
            base_root_fragment.content
        else {
            todo!("Error - cannot expand complex content of simple type.")
        };

        let base_content_fragment = ctx.get_complex_fragment(&base_complex_content_id).unwrap();

        let ComplexContentChildId::Restriction(base_restriction_id) =
            base_content_fragment.content_fragment
        else {
            todo!("Error - cannot expand complex content of extension type.")
        };

        let base_restriction = ctx
            .get_complex_fragment(&base_restriction_id)
            .unwrap()
            .clone();

        let child_restriction = ctx.get_complex_fragment(child_fragment_idx).unwrap();

        let base_attributes = base_restriction.attribute_declarations.clone();

        let child_attributes = child_restriction.attribute_declarations.clone();

        let new_attribute_declarations = Self::expand_restricted_attributes(
            ctx,
            child_attributes.iter(),
            base_attributes.iter(),
        )?;

        let child_restriction = ctx.get_complex_fragment_mut(child_fragment_idx).unwrap();
        child_restriction.base = base_restriction.base.clone();
        child_restriction.attribute_declarations = new_attribute_declarations;

        Ok(TransformChange::Changed)
    }
}

impl XmlnsContextTransformer for ExpandRestrictionFragments {
    type Error = ();

    fn transform(self, mut ctx: Context<'_>) -> Result<TransformChange, Self::Error> {
        ctx.iter_complex_fragment_ids()
            .into_iter()
            .map(|f| Self::expand_restriction(&mut ctx, &f))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use xmlity::{ExpandedName, LocalName, XmlNamespace};
    use xsd::schema::{self as xs};
    use xsd::schema_names as xsn;

    use crate::{
        complex::transformers::ExpandRestrictionFragments, transformers::TransformChange,
        CompiledNamespace, XmlnsContext,
    };

    #[test]
    fn basic_child_only_expand_restriction() {
        let test_namespace = XmlNamespace::new_dangerous("http://localhost");

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
        let product_type = xs::TopLevelComplexType::builder()
            .name(LocalName::new_dangerous("ProductType"))
            .content(
                xs::ComplexContent::builder()
                    .content(
                        xs::ComplexRestrictionType::builder()
                            .base(xs::QName(xsn::ANY_TYPE.clone()))
                            .particle(
                                xs::SequenceType::builder()
                                    .content(vec![
                                        xs::LocalElement::new_ref_typed(
                                            LocalName::new_dangerous("number"),
                                            ExpandedName::new(
                                                LocalName::new_dangerous("integer"),
                                                XmlNamespace::XS.into(),
                                            ),
                                        )
                                        .into(),
                                        xs::LocalElement::new_ref_typed(
                                            LocalName::new_dangerous("name"),
                                            ExpandedName::new(
                                                LocalName::new_dangerous("string"),
                                                XmlNamespace::XS.into(),
                                            ),
                                        )
                                        .into(),
                                    ])
                                    .build()
                                    .into(),
                            )
                            .build()
                            .into(),
                    )
                    .build()
                    .into(),
            )
            .build();

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
        let derived_shirt_type = xs::TopLevelComplexType::builder()
            .name(LocalName::new_dangerous("ShirtType"))
            .content(
                xs::ComplexContent::builder()
                    .content(
                        xs::ComplexRestrictionType::builder()
                            .base(xs::QName(ExpandedName::new(
                                LocalName::new_dangerous("ProductType"),
                                Some(test_namespace.clone()),
                            )))
                            .particle(
                                xs::SequenceType::builder()
                                    .content(vec![xs::LocalElement::new_ref_typed(
                                        LocalName::new_dangerous("number"),
                                        ExpandedName::new(
                                            LocalName::new_dangerous("integer"),
                                            XmlNamespace::XS.into(),
                                        ),
                                    )
                                    .into()])
                                    .build()
                                    .into(),
                            )
                            .build()
                            .into(),
                    )
                    .build()
                    .into(),
            )
            .build();

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
        let expected_flattened_shirt_type = xs::TopLevelComplexType::builder()
            .name(LocalName::new_dangerous("ShirtType"))
            .content(
                xs::ComplexContent::builder()
                    .content(
                        xs::ComplexRestrictionType::builder()
                            .base(xs::QName(xsn::ANY_TYPE.clone()))
                            .particle(
                                xs::SequenceType::builder()
                                    .content(vec![xs::LocalElement::new_ref_typed(
                                        LocalName::new_dangerous("number"),
                                        ExpandedName::new(
                                            LocalName::new_dangerous("integer"),
                                            XmlNamespace::XS.into(),
                                        ),
                                    )
                                    .into()])
                                    .build()
                                    .into(),
                            )
                            .build()
                            .into(),
                    )
                    .build()
                    .into(),
            )
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
            .transform(&test_namespace, ExpandRestrictionFragments::new())
            .unwrap();

        assert_eq!(transform_changed, TransformChange::Changed);

        let transform_changed = xmlns_context
            .transform(&test_namespace, ExpandRestrictionFragments::new())
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
    fn basic_attribute_only_expand_restriction() {
        let test_namespace = XmlNamespace::new_dangerous("http://localhost");

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
        let product_type = xs::TopLevelComplexType::builder()
            .name(LocalName::new_dangerous("ProductType"))
            .content(
                xs::ComplexContent::builder()
                    .content(
                        xs::ComplexRestrictionType::builder()
                            .base(xs::QName(xsn::ANY_TYPE.clone()))
                            .attributes(vec![
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
                            ])
                            .build()
                            .into(),
                    )
                    .build()
                    .into(),
            )
            .build();

        // ```xml
        // <xs:complexType name="ShirtType">
        //   <xs:complexContent>
        //     <xs:restriction base="ProductType">
        //       <xs:attribute name="number" type="xs:integer" use="required" />
        //     </xs:restriction>
        //   </xs:complexContent>
        // </xs:complexType>
        // ```
        let derived_shirt_type = xs::TopLevelComplexType::builder()
            .name(LocalName::new_dangerous("ShirtType"))
            .content(
                xs::ComplexContent::builder()
                    .content(
                        xs::ComplexRestrictionType::builder()
                            .base(xs::QName(ExpandedName::new(
                                LocalName::new_dangerous("ProductType"),
                                Some(test_namespace.clone()),
                            )))
                            .attributes(vec![xs::LocalAttribute::builder()
                                .name(LocalName::new_dangerous("number"))
                                .type_(xs::QName(xsn::INTEGER.clone()))
                                .use_(xs::AttributeUseType::Required)
                                .build()
                                .into()])
                            .build()
                            .into(),
                    )
                    .build()
                    .into(),
            )
            .build();

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
        let expected_flattened_shirt_type = xs::TopLevelComplexType::builder()
            .name(LocalName::new_dangerous("ShirtType"))
            .content(
                xs::ComplexContent::builder()
                    .content(
                        xs::ComplexRestrictionType::builder()
                            .base(xs::QName(xsn::ANY_TYPE.clone()))
                            .attributes(vec![
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
                            .build()
                            .into(),
                    )
                    .build()
                    .into(),
            )
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
            .transform(&test_namespace, ExpandRestrictionFragments::new())
            .unwrap();

        assert_eq!(transform_changed, TransformChange::Changed);

        let transform_changed = xmlns_context
            .transform(&test_namespace, ExpandRestrictionFragments::new())
            .unwrap();

        assert_eq!(transform_changed, TransformChange::Unchanged);

        let compiled_namespace = xmlns_context.namespaces.get(&test_namespace).unwrap();

        let actual_flattened_shirt_type = compiled_namespace
            .export_top_level_complex_type(&LocalName::new_dangerous("ShirtType"))
            .unwrap()
            .unwrap();

        assert_eq!(expected_flattened_shirt_type, actual_flattened_shirt_type);
    }
}
