use std::collections::{HashMap, HashSet};

use xmlity::ExpandedName;
use xsd_dynamic_query::{IdentifySimpleType, ParsedFacets, XmlnsContextQueryContext};

use crate::{TransformChange, XmlnsContextTransformer, XmlnsContextTransformerContext};
use xsd_fragments::{
    fragments::{
        simple::{self as sm, RestrictionFragment, SimpleDerivation, SimpleTypeRootFragment},
        FragmentIdx,
    },
    NamedOrAnonymous,
};

pub struct ExpandSimpleRestriction<'a> {
    allowed_bases: &'a HashSet<ExpandedName<'static>>,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Base {base} not found in the context")]
    BaseNotFound { base: ExpandedName<'static> },
    #[error("Base {base} is not a simple type")]
    BaseNotSimpleType { base: ExpandedName<'static> },
}

impl<'a> ExpandSimpleRestriction<'a> {
    pub fn new(allowed_bases: &'a HashSet<ExpandedName<'static>>) -> Self {
        Self { allowed_bases }
    }

    fn flatten_restriction(
        &self,
        ctx: &mut XmlnsContextTransformerContext,
        fragment_idx: &FragmentIdx<RestrictionFragment>,
    ) -> Result<TransformChange, <Self as XmlnsContextTransformer>::Error> {
        let RestrictionFragment { base, facets, .. } =
            ctx.get_simple_fragment(fragment_idx).unwrap();

        let facets = facets
            .iter()
            .map(|f| ctx.get_simple_fragment(f).unwrap())
            .collect::<ParsedFacets>();

        let Some(base) = base.as_ref() else {
            // If the base is not set, we skip it
            return Ok(TransformChange::default());
        };

        if self.allowed_bases.iter().any(|b| b == base) {
            // If the base is not in the allowed bases, we skip it
            return Ok(TransformChange::default());
        }

        let xsd_fragments::TopLevelType::Simple(simple_type) = ctx
            .get_named_type(base)
            .ok_or(Error::BaseNotFound { base: base.clone() })?
        else {
            return Err(Error::BaseNotSimpleType { base: base.clone() });
        };

        let base_fragment: &SimpleTypeRootFragment = ctx
            .get_simple_fragment(&simple_type.root_fragment)
            .expect("Base fragment should exist");

        match &base_fragment.simple_derivation {
            SimpleDerivation::Restriction(base_restriction) => {
                let base_restriction = ctx
                    .get_simple_fragment(base_restriction)
                    .expect("Base restriction should exist")
                    .clone();

                // We need to replace the base with the base restriction and then flatten the facets
                let fragment = ctx.get_simple_fragment_mut(fragment_idx).unwrap();
                fragment.base = base_restriction.base.clone();

                Ok(TransformChange::Changed)
            }
            SimpleDerivation::List(base_list) => {
                let base_list = ctx
                    .get_simple_fragment(base_list)
                    .expect("Base list should exist")
                    .clone();

                todo!()
            }
            SimpleDerivation::Union(fragment_idx) => {
                let union_type = ctx
                    .get_simple_fragment(fragment_idx)
                    .expect("Base union should exist");

                let union_types = facets
                    .enumerations
                    .iter()
                    .map(|v| {
                        fragment_idx.identify_simple_type(
                            &XmlnsContextQueryContext {
                                named_handlers: HashMap::new(),
                                xmlns_context: &ctx.xmlns_context,
                            },
                            &v.0,
                        )
                    })
                    .collect::<Result<
                        HashSet<Option<NamedOrAnonymous<FragmentIdx<sm::RestrictionFragment>>>>,
                        xsd_dynamic_query::Error,
                    >>()
                    .unwrap();

                // let base_union = ctx
                //     .get_simple_fragment(fragment_idx)
                //     .expect("Base union should exist")
                //     .clone();

                todo!()
            }
        }
    }
}

impl XmlnsContextTransformer for ExpandSimpleRestriction<'_> {
    type Error = Error;

    fn transform(
        self,
        mut ctx: XmlnsContextTransformerContext<'_>,
    ) -> Result<TransformChange, Self::Error> {
        ctx.iter_simple_fragment_ids()
            .collect::<Vec<_>>()
            .into_iter()
            .map(|f| self.flatten_restriction(&mut ctx, &f))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    use xmlity::{ExpandedName, LocalName, XmlNamespace};
    use xsd::{xs, xsn};

    use xsd_fragments::XmlnsContext;

    use crate::{TransformChange, XmlnsContextExt};

    #[test]
    fn restrict_union_test_1() {
        let parent_type: &str = r###"
        <xs:simpleType xmlns:xs="http://www.w3.org/2001/XMLSchema" name="allNNI">
            <xs:union memberTypes="xs:nonNegativeInteger">
                <xs:simpleType>
                    <xs:restriction base="xs:NMTOKEN">
                        <xs:enumeration value="unbounded"/>
                    </xs:restriction>
                </xs:simpleType>
            </xs:union>
        </xs:simpleType>
        "###;
        let parent_type: xs::SimpleType = xmlity_quick_xml::from_str(parent_type.trim()).unwrap();

        let child_type: &str = r###"
        <xs:simpleType xmlns:xs="http://www.w3.org/2001/XMLSchema" name="allNNIRestriction">
            <xs:restriction base="xs:allNNI">
                <xs:enumeration value="0"/>
                <xs:enumeration value="1"/>
            </xs:restriction>
        </xs:simpleType>
        "###;
        let child_type: xs::SimpleType = xmlity_quick_xml::from_str(child_type.trim()).unwrap();

        let allowed_bases: HashSet<ExpandedName<'static>> =
            [&xsn::NMTOKEN, &xsn::NON_NEGATIVE_INTEGER]
                .into_iter()
                .map(|name| (*name).clone())
                .collect();

        const TEST_NAMESPACE: XmlNamespace<'static> =
            XmlNamespace::new_dangerous("http://example.com/test");

        let mut ctx = XmlnsContext::new();

        let ns = ctx.init_namespace(TEST_NAMESPACE);

        ns.import_top_level_simple_type(&parent_type).unwrap();
        ns.import_top_level_simple_type(&child_type).unwrap();

        let transform_changed = ctx
            .context_transform(ExpandSimpleRestriction::new(&allowed_bases))
            .unwrap();

        assert_eq!(transform_changed, TransformChange::Changed);

        let ns = ctx.get_namespace(&TEST_NAMESPACE).unwrap();

        let actual = ns
            .export_top_level_simple_type(&LocalName::new_dangerous("allNNIRestriction"))
            .unwrap()
            .unwrap();

        let expected: &str = r###"
        <xs:simpleType xmlns:xs="http://www.w3.org/2001/XMLSchema" name="allNNIRestriction">
            <xs:restriction base="xs:nonNegativeInteger">
                <xs:enumeration value="0"/>
                <xs:enumeration value="1"/>
            </xs:restriction>
        </xs:simpleType>
        "###;

        let expected: xs::SimpleType = xmlity_quick_xml::from_str(expected.trim()).unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn restrict_union_test_2() {
        let parent_type: &str = r###"
        <xs:simpleType xmlns:xs="http://www.w3.org/2001/XMLSchema" name="customAllNNI">
            <xs:union memberTypes="xs:nonNegativeInteger xs:float">
                <xs:simpleType>
                    <xs:restriction base="xs:NMTOKEN">
                        <xs:enumeration value="unbounded"/>
                    </xs:restriction>
                </xs:simpleType>
            </xs:union>
        </xs:simpleType>
        "###;
        let parent_type: xs::SimpleType = xmlity_quick_xml::from_str(parent_type.trim()).unwrap();

        let child_type: &str = r###"
        <xs:simpleType xmlns:xs="http://www.w3.org/2001/XMLSchema" name="allNNIRestriction">
            <xs:restriction base="xs:customAllNNI">
                <xs:enumeration value="0"/>
                <xs:enumeration value="1"/>
                <xs:enumeration value="2.0"/>
            </xs:restriction>
        </xs:simpleType>
        "###;
        let child_type: xs::SimpleType = xmlity_quick_xml::from_str(child_type.trim()).unwrap();

        let allowed_bases: HashSet<ExpandedName<'static>> =
            [&xsn::NMTOKEN, &xsn::NON_NEGATIVE_INTEGER]
                .into_iter()
                .map(|name| (*name).clone())
                .collect();

        const TEST_NAMESPACE: XmlNamespace<'static> =
            XmlNamespace::new_dangerous("http://example.com/test");

        let mut ctx = XmlnsContext::new();

        let ns = ctx.init_namespace(TEST_NAMESPACE);

        ns.import_top_level_simple_type(&parent_type).unwrap();
        ns.import_top_level_simple_type(&child_type).unwrap();

        let transform_changed = ctx
            .context_transform(ExpandSimpleRestriction::new(&allowed_bases))
            .unwrap();

        assert_eq!(transform_changed, TransformChange::Changed);

        let ns = ctx.get_namespace(&TEST_NAMESPACE).unwrap();

        let actual = ns
            .export_top_level_simple_type(&LocalName::new_dangerous("allNNIRestriction"))
            .unwrap()
            .unwrap();

        let expected: &str = r###"
        <xs:simpleType xmlns:xs="http://www.w3.org/2001/XMLSchema" name="allNNIRestriction">
            <xs:union>
                <xs:simpleType>
                    <xs:restriction base="xs:nonNegativeInteger">
                        <xs:enumeration value="0"/>
                        <xs:enumeration value="1"/>
                    </xs:restriction>
                </xs:simpleType>
                <xs:simpleType>
                    <xs:restriction base="xs:float">
                        <xs:enumeration value="2.0"/>
                    </xs:restriction>
                </xs:simpleType>
            </xs:union>
        </xs:simpleType>
        "###;

        let expected: xs::SimpleType = xmlity_quick_xml::from_str(expected.trim()).unwrap();

        assert_eq!(actual, expected);
    }
}
