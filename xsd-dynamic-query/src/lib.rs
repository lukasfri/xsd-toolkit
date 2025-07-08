use std::{collections::HashMap, num::NonZeroUsize};

use xmlity::ExpandedName;
use xsd_fragments::{
    fragments::{
        complex::{self as cx},
        simple::{self as sm, Assertion, ExplicitTimezoneValue, Pattern, Value},
        FragmentAccess, FragmentIdx, NamespaceIdx,
    },
    CompiledNamespace, NamedOrAnonymous,
};

pub struct XmlnsContextQueryContext<'a> {
    pub named_handlers: HashMap<ExpandedName<'static>, Box<dyn IdentifySimpleType>>,
    pub xmlns_context: &'a xsd_fragments::XmlnsContext,
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
    ) -> Option<&'a xsd_fragments::TopLevelType> {
        self.xmlns_context
            .get_namespace(name.namespace()?)?
            .top_level_types
            .get(name.local_name())
    }

    pub fn get_named_attribute_group<'a>(
        &'a self,
        name: &'a ExpandedName<'_>,
    ) -> Option<&'a xsd_fragments::TopLevelAttributeGroup> {
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

#[derive(Default, Debug)]
pub struct ParsedFacets<'a> {
    pub min_length: Option<&'a usize>,
    pub max_length: Option<&'a usize>,
    pub length: Option<&'a usize>,
    pub min_inclusive: Option<&'a Value>,
    pub min_exclusive: Option<&'a Value>,
    pub max_inclusive: Option<&'a Value>,
    pub max_exclusive: Option<&'a Value>,
    pub white_space: Option<&'a sm::WhiteSpaceValue>,
    pub enumerations: Vec<&'a Value>,
    pub patterns: Vec<&'a Pattern>,
    pub assertions: Vec<&'a Assertion>,
    pub total_digits: Option<&'a NonZeroUsize>,
    pub fraction_digits: Option<&'a usize>,
    pub explicit_timezone: Option<&'a ExplicitTimezoneValue>,
}

impl<'a> ParsedFacets<'a> {
    /// Add a facet to the parsed facets
    /// Returns true if the facet was already present
    pub fn add_facet(&mut self, facet: &'a sm::FacetFragment) -> bool {
        match facet {
            sm::FacetFragment::Length { value } => self.length.replace(value).is_some(),
            sm::FacetFragment::MinLength { value } => self.min_length.replace(value).is_some(),
            sm::FacetFragment::MaxLength { value } => self.max_length.replace(value).is_some(),
            sm::FacetFragment::MinExclusive { value } => {
                self.min_exclusive.replace(value).is_some()
            }
            sm::FacetFragment::MinInclusive { value } => {
                self.min_inclusive.replace(value).is_some()
            }
            sm::FacetFragment::MaxExclusive { value } => {
                self.max_exclusive.replace(value).is_some()
            }
            sm::FacetFragment::MaxInclusive { value } => {
                self.max_inclusive.replace(value).is_some()
            }
            sm::FacetFragment::Enumeration { value } => {
                if self
                    .enumerations
                    .iter()
                    .any(|e| e.0.trim() == value.0.trim())
                {
                    true // The enumeration is already present
                } else {
                    self.enumerations.push(value);
                    false
                }
            }
            sm::FacetFragment::TotalDigits { value } => self.total_digits.replace(value).is_some(),
            sm::FacetFragment::FractionDigits { value } => {
                self.fraction_digits.replace(value).is_some()
            }
            sm::FacetFragment::WhiteSpace { value } => self.white_space.replace(value).is_some(),
            sm::FacetFragment::Pattern { value } => {
                if self.patterns.iter().any(|e| e.0.trim() == value.0.trim()) {
                    true // The pattern is already present
                } else {
                    self.patterns.push(value);
                    false
                }
            }
            sm::FacetFragment::Assertion { test } => {
                let Some(test) = test.as_ref() else {
                    return false; // No assertion to add
                };

                if self.assertions.iter().any(|e| *e == test) {
                    true // The assertion is already present
                } else {
                    self.assertions.push(test);
                    false
                }
            }
            sm::FacetFragment::ExplicitTimezone { value } => {
                if self.explicit_timezone.replace(value).is_some() {
                    true // The explicit timezone is already present
                } else {
                    false // The explicit timezone was added
                }
            }
        }
    }
}

impl<'a> FromIterator<&'a sm::FacetFragment> for ParsedFacets<'a> {
    fn from_iter<T: IntoIterator<Item = &'a sm::FacetFragment>>(iter: T) -> Self {
        let mut facets = ParsedFacets::default();
        facets.extend(iter);
        facets
    }
}

impl<'a> Extend<&'a sm::FacetFragment> for ParsedFacets<'a> {
    fn extend<T: IntoIterator<Item = &'a sm::FacetFragment>>(&mut self, iter: T) {
        iter.into_iter().for_each(|facet| {
            self.add_facet(facet);
        });
    }
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
            if base.identify_simple_type(ctx, value)?.is_none() {
                // If the base type does not match, we return None
                return Ok(None);
            }
        }

        let parsed_facets = fragment
            .facets
            .iter()
            .map(|id| ctx.get_simple_fragment(id).unwrap())
            .collect::<ParsedFacets>();

        let white_space = parsed_facets
            .white_space
            .copied()
            .unwrap_or(sm::WhiteSpaceValue::Collapse);

        let value = match white_space {
            // No normalization is done, the whitespace-normalized value is the ·initial value·
            sm::WhiteSpaceValue::Preserve => value.to_string(),
            // All occurrences of #x9 (tab), #xA (line feed) and #xD (carriage return) are replaced with #x20 (space).
            sm::WhiteSpaceValue::Replace => value.replace(['\t', '\n', '\r'], " "),
            // Subsequent to the replacements specified above under replace, contiguous sequences of #x20s are collapsed to a single #x20, and initial and/or final #x20s are deleted.
            sm::WhiteSpaceValue::Collapse => value
                .replace(['\t', '\n', '\r'], " ")
                .split_whitespace()
                .filter(|s| !s.is_empty())
                .collect::<Vec<&str>>()
                .join(" "),
        };

        // TODO: Depends on the type, if it should be counted as bytes or characters
        // For now, we will use characters
        fn count_len(value: &str) -> usize {
            value.chars().count()
        }

        let value_len = count_len(&value);

        if let Some(length) = parsed_facets.length {
            if value_len != *length {
                return Ok(None);
            }
        }
        if let Some(min_length) = parsed_facets.min_length {
            if value_len < *min_length {
                return Ok(None);
            }
        }
        if let Some(max_length) = parsed_facets.max_length {
            if value_len > *max_length {
                return Ok(None);
            }
        }

        if !parsed_facets.enumerations.is_empty() {
            if parsed_facets
                .enumerations
                .iter()
                .any(|e| e.0.trim() == &value)
            {
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

        let member_types_id = fragments
            .member_types
            .iter()
            .map(|name| name.identify_simple_type(ctx, value));

        let simple_types_id = fragments
            .simple_types
            .iter()
            .map(|id| id.identify_simple_type(ctx, value));

        member_types_id
            .chain(simple_types_id)
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

impl IdentifySimpleType for ExpandedName<'_> {
    fn identify_simple_type(
        &self,
        ctx: &XmlnsContextQueryContext<'_>,
        value: &str,
    ) -> Result<Option<NamedOrAnonymous<FragmentIdx<sm::RestrictionFragment>>>, Error> {
        if let Some(handler) = ctx.named_handlers.get(self) {
            return handler.identify_simple_type(ctx, value);
        }

        ctx.get_named_type(self)
            .ok_or_else(|| Error::TypeNotFound {
                name: self.clone().into_owned(),
            })
            .and_then(|t| match t {
                xsd_fragments::TopLevelType::Simple(type_) => {
                    type_.root_fragment.identify_simple_type(ctx, value)
                }
                xsd_fragments::TopLevelType::Complex(_) => Err(Error::TypeNotSimpleType {
                    name: self.clone().into_owned(),
                }),
            })
    }
}

impl IdentifySimpleType for NamedOrAnonymous<FragmentIdx<sm::SimpleTypeRootFragment>> {
    fn identify_simple_type(
        &self,
        ctx: &XmlnsContextQueryContext<'_>,
        value: &str,
    ) -> Result<Option<NamedOrAnonymous<FragmentIdx<sm::RestrictionFragment>>>, Error> {
        match self {
            NamedOrAnonymous::Named(name) => name.identify_simple_type(ctx, value),
            NamedOrAnonymous::Anonymous(id) => id.identify_simple_type(ctx, value),
        }
    }
}

struct AllowAll(ExpandedName<'static>);

impl IdentifySimpleType for AllowAll {
    fn identify_simple_type(
        &self,
        _ctx: &XmlnsContextQueryContext<'_>,
        _value: &str,
    ) -> Result<Option<NamedOrAnonymous<FragmentIdx<sm::RestrictionFragment>>>, Error> {
        Ok(Some(NamedOrAnonymous::Named(self.0.clone())))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use xmlity::{LocalName, XmlNamespace};
    use xsd::{xs, xsn};
    use xsd_fragments::XmlnsContext;

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
            named_handlers: map_macro::hash_map! {
                xsn::NMTOKEN.clone() => Box::new(AllowAll(xsn::NMTOKEN.clone())) as Box<dyn IdentifySimpleType>,
            },
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
            xsd_fragments::TopLevelType::Simple(type_) => type_.root_fragment,
            xsd_fragments::TopLevelType::Complex(_) => {
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
            named_handlers: map_macro::hash_map! {
                xsn::NON_NEGATIVE_INTEGER.clone() => Box::new(AllowAll(xsn::NON_NEGATIVE_INTEGER.clone())) as Box<dyn IdentifySimpleType>,
                xsn::NMTOKEN.clone() => Box::new(AllowAll(xsn::NMTOKEN.clone())) as Box<dyn IdentifySimpleType>,
            },
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
            xsd_fragments::TopLevelType::Simple(type_) => type_.root_fragment,
            xsd_fragments::TopLevelType::Complex(_) => {
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
            named_handlers: map_macro::hash_map! {
                xsn::NON_NEGATIVE_INTEGER.clone() => Box::new(AllowAll(xsn::NON_NEGATIVE_INTEGER.clone())) as Box<dyn IdentifySimpleType>,
                xsn::NMTOKEN.clone() => Box::new(AllowAll(xsn::NMTOKEN.clone())) as Box<dyn IdentifySimpleType>,
            },
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
            xsd_fragments::TopLevelType::Simple(type_) => type_.root_fragment,
            xsd_fragments::TopLevelType::Complex(_) => {
                panic!("Expected a simple type, got a complex type")
            }
        };

        child_fragment
            .identify_simple_type(&query_ctx, "0")
            .unwrap();
    }
}
