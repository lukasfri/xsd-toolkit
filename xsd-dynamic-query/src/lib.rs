use std::collections::HashSet;

use xmlity::ExpandedName;
use xsd_type_compiler::{
    fragments::{
        complex::{self as cx},
        simple::{self as sm},
        FragmentAccess, FragmentIdx, NamespaceIdx,
    },
    CompiledNamespace, NamedOrAnonymous,
};

pub struct XmlnsContextQueryContext<'a> {
    pub temp_allow_base: HashSet<ExpandedName<'static>>,
    pub xmlns_context: &'a xsd_type_compiler::XmlnsContext,
}

impl XmlnsContextQueryContext<'_> {
    fn get_namespace(&self, namespace_idx: &NamespaceIdx) -> Option<&CompiledNamespace> {
        self.xmlns_context.namespaces.get(namespace_idx)
    }

    pub fn iter_complex_fragment_ids<F: 'static>(&self) -> impl Iterator<Item = FragmentIdx<F>> + '_
    where
        cx::ComplexTypeFragmentCompiler: FragmentAccess<F>,
    {
        self.xmlns_context
            .namespaces
            .iter()
            .flat_map(|(_, ns)| ns.complex_type.iter_fragment_ids())
    }

    pub fn get_complex_fragment<F>(&self, fragment_idx: &FragmentIdx<F>) -> Option<&F>
    where
        cx::ComplexTypeFragmentCompiler: FragmentAccess<F>,
    {
        self.get_namespace(&fragment_idx.namespace_idx())?
            .complex_type
            .get_fragment(fragment_idx)
    }

    pub fn iter_simple_fragment_ids<F: 'static>(&self) -> impl Iterator<Item = FragmentIdx<F>> + '_
    where
        sm::SimpleTypeFragmentCompiler: FragmentAccess<F>,
    {
        self.xmlns_context
            .namespaces
            .iter()
            .flat_map(|(_, ns)| ns.complex_type.simple_type_fragments.iter_fragment_ids())
    }

    pub fn get_simple_fragment<F>(&self, fragment_idx: &FragmentIdx<F>) -> Option<&F>
    where
        sm::SimpleTypeFragmentCompiler: FragmentAccess<F>,
    {
        self.get_namespace(&fragment_idx.namespace_idx())?
            .complex_type
            .simple_type_fragments
            .get_fragment(fragment_idx)
    }

    pub fn get_named_type<'a>(
        &'a self,
        name: &'a ExpandedName<'_>,
    ) -> Option<&'a xsd_type_compiler::TopLevelType> {
        self.xmlns_context
            .get_namespace(name.namespace()?)?
            .top_level_types
            .get(name.local_name())
    }

    pub fn get_named_attribute_group<'a>(
        &'a self,
        name: &'a ExpandedName<'_>,
    ) -> Option<&'a xsd_type_compiler::TopLevelAttributeGroup> {
        self.xmlns_context
            .get_namespace(name.namespace()?)?
            .top_level_attribute_groups
            .get(name.local_name())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Fragment not found in the context")]
    TypeNotFound { name: ExpandedName<'static> },
    #[error("Type {name} is not a simple type")]
    TypeNotSimpleType { name: ExpandedName<'static> },
    #[error("Fragment not found in the context")]
    FragmentNotFound {},
}

pub trait IdentifySimpleType {
    fn identify_simple_type(
        &self,
        ctx: &XmlnsContextQueryContext<'_>,
        value: &str,
    ) -> Result<Option<NamedOrAnonymous<FragmentIdx<sm::RestrictionFragment>>>, Error>;
}

impl IdentifySimpleType for FragmentIdx<sm::RestrictionFragment> {
    fn identify_simple_type(
        &self,
        ctx: &XmlnsContextQueryContext<'_>,
        value: &str,
    ) -> Result<Option<NamedOrAnonymous<FragmentIdx<sm::RestrictionFragment>>>, Error> {
        let fragment = ctx
            .get_simple_fragment(self)
            .ok_or(Error::FragmentNotFound {})?;

        if let Some(base) = fragment.base.as_ref() {
            // If the base is in the allowed bases, we skip checking the base type
            if !ctx.temp_allow_base.contains(base) {
                let base_fragment: FragmentIdx<sm::SimpleTypeRootFragment> = ctx
                    .get_named_type(base)
                    .ok_or_else(|| Error::TypeNotFound { name: base.clone() })
                    .and_then(|t| match t {
                        xsd_type_compiler::TopLevelType::Simple(type_) => Ok(type_.root_fragment),
                        xsd_type_compiler::TopLevelType::Complex(_) => {
                            Err(Error::TypeNotSimpleType { name: base.clone() })
                        }
                    })?;

                if base_fragment.identify_simple_type(ctx, value)?.is_none() {
                    // If the base type does not match, we return None
                    return Ok(None);
                }
            }
        }

        let mut min_length = None;
        let mut max_length = None;
        let mut length = None;
        let mut min_inclusive = None;
        let mut min_exclusive = None;
        let mut max_inclusive = None;
        let mut max_exclusive = None;
        let mut white_space = None;
        let mut enumerations = Vec::new();
        let mut patterns = Vec::new();
        let mut assertions = Vec::new();
        let mut total_digits = None;
        let mut fraction_digits = None;

        for facet in fragment
            .facets
            .iter()
            .filter_map(|id| ctx.get_simple_fragment(id))
        {
            match facet {
                sm::FacetFragment::MinExclusive { value } => {
                    min_exclusive = Some(value.0.trim());
                }
                sm::FacetFragment::MinInclusive { value } => {
                    min_inclusive = Some(value.0.trim());
                }
                sm::FacetFragment::MaxExclusive { value } => {
                    max_exclusive = Some(value.0.trim());
                }
                sm::FacetFragment::MaxInclusive { value } => {
                    max_inclusive = Some(value.0.trim());
                }
                sm::FacetFragment::Enumeration { value } => {
                    enumerations.push(value.0.trim());
                }
                sm::FacetFragment::TotalDigits { value } => {
                    total_digits = Some(*value);
                }
                sm::FacetFragment::FractionDigits { value } => {
                    fraction_digits = Some(*value);
                }
                sm::FacetFragment::Pattern { value } => {
                    patterns.push(value.clone());
                }
                sm::FacetFragment::Assertion { test } => {
                    if let Some(test) = test.as_ref() {
                        assertions.push(test.clone());
                    }
                }
                sm::FacetFragment::WhiteSpace { value } => {
                    white_space = Some(*value);
                }
                sm::FacetFragment::ExplicitTimezone { .. } => {}
                sm::FacetFragment::Length { value } => {
                    length = Some(*value);
                }
                sm::FacetFragment::MinLength { value } => {
                    min_length = Some(*value);
                }
                sm::FacetFragment::MaxLength { value } => {
                    max_length = Some(*value);
                }
            }
        }

        let white_space = white_space.unwrap_or(sm::WhiteSpaceValue::Collapse);

        let value = match white_space {
            // No normalization is done, the whitespace-normalized value is the ·initial value·
            sm::WhiteSpaceValue::Preserve => value.to_string(),
            // All occurrences of #x9 (tab), #xA (line feed) and #xD (carriage return) are replaced with #x20 (space).
            sm::WhiteSpaceValue::Replace => value
                .replace('\t', " ")
                .replace('\n', " ")
                .replace('\r', " "),
            // Subsequent to the replacements specified above under replace, contiguous sequences of #x20s are collapsed to a single #x20, and initial and/or final #x20s are deleted.
            sm::WhiteSpaceValue::Collapse => value
                .split_whitespace()
                .filter(|s| !s.is_empty())
                .collect::<Vec<&str>>()
                .join(" ")
                .trim()
                .to_string(),
        };

        // TODO: Depends on the type, if it should be counted as bytes or characters
        // For now, we will use characters
        fn count_len(value: &str) -> usize {
            value.chars().count()
        }

        let value_len = count_len(&value);

        if let Some(length) = length {
            if value_len != length {
                return Ok(None);
            }
        }
        if let Some(min_length) = min_length {
            if value_len < min_length {
                return Ok(None);
            }
        }
        if let Some(max_length) = max_length {
            if value_len > max_length {
                return Ok(None);
            }
        }

        if !enumerations.is_empty() {
            if enumerations.iter().any(|e| e == &value) {
                return Ok(Some(NamedOrAnonymous::Anonymous(*self)));
            } else {
                return Ok(None);
            }
        } else {
            return Ok(Some(NamedOrAnonymous::Anonymous(*self)));
        }
    }
}

impl IdentifySimpleType for FragmentIdx<sm::ListFragment> {
    fn identify_simple_type(
        &self,
        ctx: &XmlnsContextQueryContext<'_>,
        value: &str,
    ) -> Result<Option<NamedOrAnonymous<FragmentIdx<sm::RestrictionFragment>>>, Error> {
        let fragment = ctx
            .get_simple_fragment(self)
            .ok_or(Error::FragmentNotFound {})?;

        let item_type = &fragment.item_type;

        value
            .split([' ', '|', ',', ';'])
            .filter(|v| !v.is_empty())
            .map(|v| item_type.identify_simple_type(ctx, v))
            .find_map(|result| match result {
                Ok(Some(fragment)) => Some(Ok(fragment)),
                Ok(None) => None,
                Err(e) => Some(Err(e)),
            })
            .transpose()
    }
}

impl IdentifySimpleType for FragmentIdx<sm::UnionFragment> {
    fn identify_simple_type(
        &self,
        ctx: &XmlnsContextQueryContext<'_>,
        value: &str,
    ) -> Result<Option<NamedOrAnonymous<FragmentIdx<sm::RestrictionFragment>>>, Error> {
        let fragments = ctx
            .get_simple_fragment(self)
            .ok_or(Error::FragmentNotFound {})?;

        let identified_member_type = fragments
            .member_types
            .iter()
            .find_map(|name| {
                if ctx.temp_allow_base.contains(name) {
                    // If the base is in the allowed bases, we skip checking the base type
                    return Some(Ok(Some(NamedOrAnonymous::Named(name.clone()))));
                } else {
                    let Some(type_) = ctx.get_named_type(name) else {
                        return Some(Err(Error::TypeNotFound { name: name.clone() }));
                    };

                    match type_ {
                        xsd_type_compiler::TopLevelType::Simple(type_) => {
                            Some(type_.root_fragment.identify_simple_type(ctx, value))
                        }
                        xsd_type_compiler::TopLevelType::Complex(_) => None,
                    }
                }
            })
            .transpose()?
            .flatten();

        if let Some(fragment) = identified_member_type {
            return Ok(Some(fragment));
        }

        fragments
            .simple_types
            .iter()
            .map(|id| id.identify_simple_type(ctx, value))
            .find_map(|result| match result {
                Ok(Some(fragment)) => Some(Ok(fragment)),
                Ok(None) => None,
                Err(e) => Some(Err(e)),
            })
            .transpose()
    }
}

impl IdentifySimpleType for FragmentIdx<sm::SimpleTypeRootFragment> {
    fn identify_simple_type(
        &self,
        ctx: &XmlnsContextQueryContext<'_>,
        value: &str,
    ) -> Result<Option<NamedOrAnonymous<FragmentIdx<sm::RestrictionFragment>>>, Error> {
        let fragments = ctx
            .get_simple_fragment(self)
            .ok_or(Error::FragmentNotFound {})?;

        match fragments.simple_derivation {
            sm::SimpleDerivation::Restriction(fragment_idx) => {
                fragment_idx.identify_simple_type(ctx, value)
            }
            sm::SimpleDerivation::List(fragment_idx) => {
                fragment_idx.identify_simple_type(ctx, value)
            }
            sm::SimpleDerivation::Union(fragment_idx) => {
                fragment_idx.identify_simple_type(ctx, value)
            }
        }
    }
}

impl IdentifySimpleType for NamedOrAnonymous<FragmentIdx<sm::SimpleTypeRootFragment>> {
    fn identify_simple_type(
        &self,
        ctx: &XmlnsContextQueryContext<'_>,
        value: &str,
    ) -> Result<Option<NamedOrAnonymous<FragmentIdx<sm::RestrictionFragment>>>, Error> {
        match self {
            NamedOrAnonymous::Named(expanded_name) => ctx
                .get_named_type(expanded_name)
                .ok_or_else(|| Error::TypeNotFound {
                    name: expanded_name.clone(),
                })
                .and_then(|t| match t {
                    xsd_type_compiler::TopLevelType::Simple(type_) => {
                        type_.root_fragment.identify_simple_type(ctx, value)
                    }
                    xsd_type_compiler::TopLevelType::Complex(_) => Err(Error::TypeNotSimpleType {
                        name: expanded_name.clone(),
                    }),
                }),
            NamedOrAnonymous::Anonymous(id) => id.identify_simple_type(ctx, value),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use xmlity::{LocalName, XmlNamespace};
    use xsd::xs;
    use xsd_type_compiler::XmlnsContext;

    #[test]
    fn test_identify_simple_restriction() {
        let parent_type: &str = r###"
        <xs:simpleType xmlns:xs="http://www.w3.org/2001/XMLSchema" name="unbounded">
            <xs:restriction base="xs:NMTOKEN">
                <xs:enumeration value="unbounded"/>
            </xs:restriction>
        </xs:simpleType>
        "###;
        let parent_type: xs::SimpleType = xmlity_quick_xml::from_str(parent_type.trim()).unwrap();

        const TEST_NAMESPACE: XmlNamespace<'static> =
            XmlNamespace::new_dangerous("http://example.com/test");

        let mut ctx = XmlnsContext::new();

        let ns = ctx.init_namespace(TEST_NAMESPACE);

        ns.import_top_level_simple_type(&parent_type).unwrap();

        let query_ctx = XmlnsContextQueryContext {
            xmlns_context: &ctx,
            temp_allow_base: [ExpandedName::new(
                LocalName::new_dangerous("NMTOKEN"),
                Some(XmlNamespace::XS),
            )]
            .into_iter()
            .collect(),
        };

        let ns = query_ctx
            .xmlns_context
            .get_namespace(&TEST_NAMESPACE)
            .expect("Namespace should exist");

        let child_fragment = ns
            .top_level_types
            .get(&LocalName::new_dangerous("unbounded"))
            .unwrap();

        let child_fragment = match child_fragment {
            xsd_type_compiler::TopLevelType::Simple(type_) => type_.root_fragment,
            xsd_type_compiler::TopLevelType::Complex(_) => {
                panic!("Expected a simple type, got a complex type")
            }
        };

        child_fragment
            .identify_simple_type(&query_ctx, "unbounded")
            .unwrap()
            .unwrap();
    }

    #[test]
    fn test_identify_simple_union() {
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

        const TEST_NAMESPACE: XmlNamespace<'static> =
            XmlNamespace::new_dangerous("http://example.com/test");

        let mut ctx = XmlnsContext::new();

        let ns = ctx.init_namespace(TEST_NAMESPACE);

        ns.import_top_level_simple_type(&parent_type).unwrap();

        let query_ctx = XmlnsContextQueryContext {
            xmlns_context: &ctx,
            temp_allow_base: [
                ExpandedName::new(
                    LocalName::new_dangerous("nonNegativeInteger"),
                    Some(XmlNamespace::XS),
                ),
                ExpandedName::new(LocalName::new_dangerous("NMTOKEN"), Some(XmlNamespace::XS)),
            ]
            .into_iter()
            .collect(),
        };

        let ns = query_ctx
            .xmlns_context
            .get_namespace(&TEST_NAMESPACE)
            .expect("Namespace should exist");

        let child_fragment = ns
            .top_level_types
            .get(&LocalName::new_dangerous("allNNI"))
            .unwrap();

        let child_fragment = match child_fragment {
            xsd_type_compiler::TopLevelType::Simple(type_) => type_.root_fragment,
            xsd_type_compiler::TopLevelType::Complex(_) => {
                panic!("Expected a simple type, got a complex type")
            }
        };

        child_fragment
            .identify_simple_type(&query_ctx, "0")
            .unwrap();
    }

    #[test]
    fn test_identify() {
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

        let mut ctx = XmlnsContext::new();

        let ns = ctx.init_namespace(XmlNamespace::XS);

        ns.import_top_level_simple_type(&parent_type).unwrap();
        ns.import_top_level_simple_type(&child_type).unwrap();

        let query_ctx = XmlnsContextQueryContext {
            xmlns_context: &ctx,
            temp_allow_base: [
                ExpandedName::new(
                    LocalName::new_dangerous("nonNegativeInteger"),
                    Some(XmlNamespace::XS),
                ),
                ExpandedName::new(LocalName::new_dangerous("NMTOKEN"), Some(XmlNamespace::XS)),
            ]
            .into_iter()
            .collect(),
        };

        let ns = query_ctx
            .xmlns_context
            .get_namespace(&XmlNamespace::XS)
            .expect("Namespace should exist");

        let child_fragment = ns
            .top_level_types
            .get(&LocalName::new_dangerous("allNNIRestriction"))
            .unwrap();

        let child_fragment = match child_fragment {
            xsd_type_compiler::TopLevelType::Simple(type_) => type_.root_fragment,
            xsd_type_compiler::TopLevelType::Complex(_) => {
                panic!("Expected a simple type, got a complex type")
            }
        };

        child_fragment
            .identify_simple_type(&query_ctx, "0")
            .unwrap();
    }
}
