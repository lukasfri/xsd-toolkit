use std::collections::HashMap;
use std::collections::VecDeque;
use std::ops::Deref;

use xmlity::ExpandedName;

use crate::complex::AttributeDeclarationId;
use crate::complex::ComplexContentChildId;
use crate::complex::ComplexContentFragment;
use crate::complex::ComplexTypeModelId;
use crate::complex::ComplexTypeRootFragment;
use crate::complex::ExtensionFragment;
use crate::complex::FragmentAccess;
use crate::complex::FragmentIdx;
use crate::complex::LocalAttributeFragment;
use crate::complex::RestrictionFragment;
use crate::complex::SequenceFragment;
use crate::complex::ANY_TYPE_EXPANDED_NAME;
use crate::transformers::Context;
use crate::transformers::TransformChange;
use crate::transformers::XmlnsContextTransformer;
use crate::TopLevelType;

/// Expands restriction and extension fragments to their base fragments, with the modifications applied.
#[non_exhaustive]
pub struct ExpandExtensionFragments {}

impl ExpandExtensionFragments {
    pub fn new() -> Self {
        Self {}
    }

    fn expand_attribute(
        ctx: &mut Context<'_>,
        attribute: &FragmentIdx<LocalAttributeFragment>,
        base_attribute: &FragmentIdx<LocalAttributeFragment>,
    ) -> Result<(), ()> {
        let base_attribute = ctx.get_complex_fragment(base_attribute).unwrap().clone();
        let attribute = ctx.get_complex_fragment_mut(attribute).unwrap();

        let base_attribute = match base_attribute {
            LocalAttributeFragment::Declared(local) => local,
            _ => todo!(),
        };
        let attribute = match attribute {
            LocalAttributeFragment::Declared(local) => local,
            _ => todo!(),
        };

        if attribute.use_.is_none() {
            attribute.use_ = base_attribute.use_;
        }
        if attribute.type_.is_none() {
            attribute.type_ = base_attribute.type_;
        }

        Ok(())
    }

    fn expand_expanded_attributes<'a>(
        ctx: &mut Context<'_>,
        child_attributes: impl Iterator<Item = &'a AttributeDeclarationId> + Clone,
        base_attributes: impl Iterator<Item = &'a AttributeDeclarationId> + Clone,
    ) -> Result<VecDeque<AttributeDeclarationId>, ()> {
        fn resolve_attr_name(
            ctx: &Context,
            a: &FragmentIdx<LocalAttributeFragment>,
        ) -> ExpandedName<'static> {
            let fragment = ctx.get_complex_fragment(a).unwrap();
            match fragment {
                LocalAttributeFragment::Declared(local) => {
                    ExpandedName::new(local.name.clone(), None)
                }
                LocalAttributeFragment::Reference(ref_) => ref_.name.clone(),
            }
        }

        let resolved_base_attributes = base_attributes
            .clone()
            .map(|a| match a {
                AttributeDeclarationId::Attribute(a) => (a.clone(), resolve_attr_name(&ctx, a)),
                AttributeDeclarationId::AttributeGroupRef(_) => todo!(),
            })
            .collect::<HashMap<_, _>>();
        let resolved_child_attributes = child_attributes
            .clone()
            .map(|a| match a {
                AttributeDeclarationId::Attribute(a) => (a.clone(), resolve_attr_name(&ctx, a)),
                AttributeDeclarationId::AttributeGroupRef(_) => todo!(),
            })
            .collect::<HashMap<_, _>>();

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

            Self::expand_attribute(ctx, matching_child_attribute, base_attribute)?;

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

    fn expand_extension(
        ctx: &mut Context<'_>,
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

        if &base == ANY_TYPE_EXPANDED_NAME.deref() {
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

        // Checks if base content is either a restriction of xs:anyType or an extension. If it is a non-anyType restriction, we cannot expand it since it could create a type that is not a valid derivative of the base's base type.
        let (base_content_content_fragment_id, base_content_base, base_attributes) =
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

                    if &base_restriction_fragment.base != ANY_TYPE_EXPANDED_NAME.deref() {
                        todo!("Error - cannot expand complex content of restriction type.")
                    }

                    (
                        base_restriction_fragment.content_fragment.clone(),
                        base_restriction_fragment.base.clone(),
                        base_restriction_fragment.attribute_declarations.clone(),
                    )
                }
            };

        let child_attributes = child_fragment.attribute_declarations.clone();

        let new_content_fragment =
            child_fragment
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
                });

        let new_attribute_declarations =
            Self::expand_expanded_attributes(ctx, child_attributes.iter(), base_attributes.iter())?;

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

    fn transform(self, mut ctx: Context<'_>) -> Result<TransformChange, ()> {
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

    use crate::{
        complex::{transformers::ExpandExtensionFragments, ANY_TYPE_EXPANDED_NAME},
        transformers::TransformChange,
        CompiledNamespace, XmlnsContext,
    };

    #[test]
    fn basic_child_only_expand_extension() {
        let test_namespace = XmlNamespace::new_dangerous("http://localhost");

        let parent_seq = xs::SequenceType::builder()
            .content(vec![
                xs::LocalElement::new_ref_typed(
                    LocalName::new_dangerous("number"),
                    ExpandedName::new(
                        LocalName::new_dangerous("integer"),
                        XmlNamespace::XMLNS.into(),
                    ),
                )
                .into(),
                xs::LocalElement::new_ref_typed(
                    LocalName::new_dangerous("name"),
                    ExpandedName::new(
                        LocalName::new_dangerous("string"),
                        XmlNamespace::XMLNS.into(),
                    ),
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
                            .base(xs::Base(xs::QName(ANY_TYPE_EXPANDED_NAME.clone())))
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
                    ExpandedName::new(
                        LocalName::new_dangerous("integer"),
                        XmlNamespace::XMLNS.into(),
                    ),
                )
                .into(),
                xs::LocalElement::new_ref_typed(
                    LocalName::new_dangerous("color"),
                    ExpandedName::new(
                        LocalName::new_dangerous("string"),
                        XmlNamespace::XMLNS.into(),
                    ),
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
                            .base(xs::Base(xs::QName(ExpandedName::new(
                                LocalName::new_dangerous("ProductType"),
                                Some(test_namespace.clone()),
                            ))))
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
                            .base(xs::Base(xs::QName(ANY_TYPE_EXPANDED_NAME.clone())))
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

        let integer_expanded_name = ExpandedName::new(
            LocalName::new_dangerous("integer"),
            XmlNamespace::XMLNS.into(),
        );
        let string_expanded_name = ExpandedName::new(
            LocalName::new_dangerous("string"),
            XmlNamespace::XMLNS.into(),
        );

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
                            .base(xs::Base(xs::QName(ANY_TYPE_EXPANDED_NAME.clone())))
                            .attributes(vec![
                                xs::LocalAttribute::builder()
                                    .name(xs::Name(LocalName::new_dangerous("number")))
                                    .type_(xs::Type(xs::QName(integer_expanded_name.clone())))
                                    .use_(xs::AttrUse(xs::AttributeUseType::Required))
                                    .build()
                                    .into(),
                                xs::LocalAttribute::builder()
                                    .name(xs::Name(LocalName::new_dangerous("name")))
                                    .type_(xs::Type(xs::QName(string_expanded_name.clone())))
                                    .use_(xs::AttrUse(xs::AttributeUseType::Required))
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
                            .base(xs::Base(xs::QName(ExpandedName::new(
                                LocalName::new_dangerous("ProductType"),
                                Some(test_namespace.clone()),
                            ))))
                            .attributes(vec![
                                xs::LocalAttribute::builder()
                                    .name(xs::Name(LocalName::new_dangerous("number")))
                                    .type_(xs::Type(xs::QName(integer_expanded_name.clone())))
                                    .use_(xs::AttrUse(xs::AttributeUseType::Optional))
                                    .build()
                                    .into(),
                                xs::LocalAttribute::builder()
                                    .name(xs::Name(LocalName::new_dangerous("color")))
                                    .type_(xs::Type(xs::QName(string_expanded_name.clone())))
                                    .use_(xs::AttrUse(xs::AttributeUseType::Required))
                                    .build()
                                    .into(),
                            ])
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
                            .base(xs::Base(xs::QName(ANY_TYPE_EXPANDED_NAME.clone())))
                            .attributes(vec![
                                xs::LocalAttribute::builder()
                                    .name(xs::Name(LocalName::new_dangerous("number")))
                                    .type_(xs::Type(xs::QName(integer_expanded_name.clone())))
                                    .use_(xs::AttrUse(xs::AttributeUseType::Optional))
                                    .build()
                                    .into(),
                                xs::LocalAttribute::builder()
                                    .name(xs::Name(LocalName::new_dangerous("name")))
                                    .type_(xs::Type(xs::QName(string_expanded_name.clone())))
                                    .use_(xs::AttrUse(xs::AttributeUseType::Required))
                                    .build()
                                    .into(),
                                xs::LocalAttribute::builder()
                                    .name(xs::Name(LocalName::new_dangerous("color")))
                                    .type_(xs::Type(xs::QName(string_expanded_name.clone())))
                                    .use_(xs::AttrUse(xs::AttributeUseType::Required))
                                    .build()
                                    .into(),
                            ])
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
}
