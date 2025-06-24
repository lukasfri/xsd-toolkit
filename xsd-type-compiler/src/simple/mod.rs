pub mod transformers;

use std::collections::{BTreeMap, VecDeque};
use xsd::xs;

use xmlity::XmlNamespace;

use crate::SimpleTypeIdent;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FragmentIdx(usize);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FragmentId(pub XmlNamespace<'static>, pub FragmentIdx);

#[derive(Debug, Clone, PartialEq)]
pub struct Extension {
    base: SimpleTypeIdent,
    content_fragment: Option<FragmentId>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Restriction {
    base: SimpleTypeIdent,
    content_fragment: Option<FragmentId>,
    facets: Vec<FragmentId>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SimpleTypeFragment {
    // GroupRef { ref_: SimpleTypeIdent },
    Restriction(Restriction),
    List { item_ident: SimpleTypeIdent },
    Union { fragments: VecDeque<FragmentId> },
    Facet(Facet),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Facet {
    MinExclusive { value: String },
    MinInclusive { value: String },
    MaxExclusive { value: String },
    MaxInclusive { value: String },
    Enumeration { value: String },
    TotalDigits { value: usize },
    FractionDigits { value: usize },
    Length { value: usize },
    MinLength { value: usize },
    MaxLength { value: usize },
    WhiteSpace { value: String },
    Pattern { value: String },
    Assertion { value: String },
    ExplicitTimezone { value: String },
}

#[derive(Debug, Clone)]
pub struct SimpleTypeFragmentCompiler {
    fragment_id_count: usize,
    pub namespace: XmlNamespace<'static>,
    pub fragments: BTreeMap<FragmentIdx, SimpleTypeFragment>,
}

impl SimpleTypeFragmentCompiler {
    pub fn new(namespace: XmlNamespace<'static>) -> Self {
        Self {
            fragment_id_count: 0,
            fragments: BTreeMap::new(),
            namespace,
        }
    }

    fn generate_fragment_id(&mut self) -> FragmentId {
        let fragment_id = FragmentId(self.namespace.clone(), FragmentIdx(self.fragment_id_count));
        self.fragment_id_count += 1;
        fragment_id
    }

    pub fn push_fragment(&mut self, fragment: SimpleTypeFragment) -> FragmentId {
        let fragment_id = self.generate_fragment_id();

        self.fragments.insert(fragment_id.1, fragment);

        fragment_id
    }

    pub fn get_fragment(&self, idx: &FragmentId) -> Option<&SimpleTypeFragment> {
        if self.namespace != idx.0 {
            return None;
        }

        self.fragments.get(&idx.1)
    }

    pub fn get_fragment_mut(&mut self, idx: &FragmentId) -> Option<&mut SimpleTypeFragment> {
        if self.namespace != idx.0 {
            return None;
        }

        self.fragments.get_mut(&idx.1)
    }

    pub fn iter_fragment_ids(&self) -> impl Iterator<Item = FragmentId> {
        self.fragments
            .keys()
            .map(|idx| FragmentId(self.namespace.clone(), *idx))
            .collect::<Vec<_>>()
            .into_iter()
    }
}

impl AsMut<SimpleTypeFragmentCompiler> for SimpleTypeFragmentCompiler {
    fn as_mut(&mut self) -> &mut SimpleTypeFragmentCompiler {
        self
    }
}

pub trait ToSimpleFragments {
    fn to_simple_fragments<T: AsMut<SimpleTypeFragmentCompiler>>(&self, compiler: T) -> FragmentId;
}

impl ToSimpleFragments for xs::types::TopLevelSimpleType {
    fn to_simple_fragments<T: AsMut<SimpleTypeFragmentCompiler>>(
        &self,
        mut compiler: T,
    ) -> FragmentId {
        let compiler = compiler.as_mut();

        self.simple_derivation.to_simple_fragments(compiler)
    }
}

impl ToSimpleFragments for xs::types::LocalSimpleType {
    fn to_simple_fragments<T: AsMut<SimpleTypeFragmentCompiler>>(
        &self,
        mut compiler: T,
    ) -> FragmentId {
        let compiler = compiler.as_mut();

        self.simple_derivation.to_simple_fragments(compiler)
    }
}

impl ToSimpleFragments for xs::types::SimpleRestrictionType {
    fn to_simple_fragments<T: AsMut<SimpleTypeFragmentCompiler>>(
        &self,
        mut compiler: T,
    ) -> FragmentId {
        let mut compiler = compiler.as_mut();

        let base = self.base.0.clone();

        let simple_restriction_model = self
            .simple_restriction_model
            .as_ref()
            .map(|a| &a.simple_restriction_model);

        let content_fragment = simple_restriction_model
            .and_then(|a| a.simple_type.as_ref())
            .map(|simple_type| simple_type.to_simple_fragments(&mut compiler));

        let facets = simple_restriction_model
            .map(|a| {
                a.child_1
                    .iter()
                    .filter_map(|a| match a {
                        //TODO
                        xs::groups::simple_restriction_model_items::Child1::Facet(facet) => {
                            Some(facet)
                        }
                        _ => None,
                    })
                    .map(|facet| facet.to_simple_fragments(&mut compiler))
                    .collect()
            })
            .unwrap_or_default();

        compiler.push_fragment(SimpleTypeFragment::Restriction(Restriction {
            base: SimpleTypeIdent::Named(base),
            content_fragment,
            facets,
        }))
    }
}

impl ToSimpleFragments for xs::Facet {
    fn to_simple_fragments<T: AsMut<SimpleTypeFragmentCompiler>>(&self, compiler: T) -> FragmentId {
        use xs::Facet as F;
        match self {
            F::MinExclusive(f) => f.to_simple_fragments(compiler),
            F::MinInclusive(f) => f.to_simple_fragments(compiler),
            F::MaxExclusive(f) => f.to_simple_fragments(compiler),
            F::MaxInclusive(f) => f.to_simple_fragments(compiler),
            F::Enumeration(f) => f.to_simple_fragments(compiler),
            F::TotalDigits(f) => f.to_simple_fragments(compiler),
            F::FractionDigits(f) => f.to_simple_fragments(compiler),
            F::Length(f) => f.to_simple_fragments(compiler),
            F::MinLength(f) => f.to_simple_fragments(compiler),
            F::MaxLength(f) => f.to_simple_fragments(compiler),
            F::WhiteSpace(f) => f.to_simple_fragments(compiler),
            F::Pattern(f) => f.to_simple_fragments(compiler),
            F::Assertion(f) => f.to_simple_fragments(compiler),
            F::ExplicitTimezone(f) => f.to_simple_fragments(compiler),
        }
    }
}

impl ToSimpleFragments for xs::MinExclusive {
    fn to_simple_fragments<T: AsMut<SimpleTypeFragmentCompiler>>(
        &self,
        mut compiler: T,
    ) -> FragmentId {
        let compiler = compiler.as_mut();

        compiler.push_fragment(SimpleTypeFragment::Facet(Facet::MinExclusive {
            value: self.0.value.clone(),
        }))
    }
}

impl ToSimpleFragments for xs::MinInclusive {
    fn to_simple_fragments<T: AsMut<SimpleTypeFragmentCompiler>>(
        &self,
        mut compiler: T,
    ) -> FragmentId {
        let compiler = compiler.as_mut();

        compiler.push_fragment(SimpleTypeFragment::Facet(Facet::MinInclusive {
            value: self.0.value.clone(),
        }))
    }
}

impl ToSimpleFragments for xs::MaxExclusive {
    fn to_simple_fragments<T: AsMut<SimpleTypeFragmentCompiler>>(
        &self,
        mut compiler: T,
    ) -> FragmentId {
        let compiler = compiler.as_mut();

        compiler.push_fragment(SimpleTypeFragment::Facet(Facet::MaxExclusive {
            value: self.0.value.clone(),
        }))
    }
}

impl ToSimpleFragments for xs::MaxInclusive {
    fn to_simple_fragments<T: AsMut<SimpleTypeFragmentCompiler>>(
        &self,
        mut compiler: T,
    ) -> FragmentId {
        let compiler = compiler.as_mut();

        compiler.push_fragment(SimpleTypeFragment::Facet(Facet::MaxInclusive {
            value: self.0.value.clone(),
        }))
    }
}

impl ToSimpleFragments for xs::Enumeration {
    fn to_simple_fragments<T: AsMut<SimpleTypeFragmentCompiler>>(
        &self,
        mut compiler: T,
    ) -> FragmentId {
        let compiler = compiler.as_mut();

        compiler.push_fragment(SimpleTypeFragment::Facet(Facet::Enumeration {
            value: self.0.value.clone(),
        }))
    }
}

impl ToSimpleFragments for xs::TotalDigits {
    fn to_simple_fragments<T: AsMut<SimpleTypeFragmentCompiler>>(
        &self,
        mut compiler: T,
    ) -> FragmentId {
        let compiler = compiler.as_mut();

        compiler.push_fragment(SimpleTypeFragment::Facet(Facet::TotalDigits {
            value: self.value.clone(),
        }))
    }
}

impl ToSimpleFragments for xs::FractionDigits {
    fn to_simple_fragments<T: AsMut<SimpleTypeFragmentCompiler>>(
        &self,
        mut compiler: T,
    ) -> FragmentId {
        let compiler = compiler.as_mut();

        compiler.push_fragment(SimpleTypeFragment::Facet(Facet::FractionDigits {
            value: self.0.value.clone(),
        }))
    }
}

impl ToSimpleFragments for xs::Length {
    fn to_simple_fragments<T: AsMut<SimpleTypeFragmentCompiler>>(
        &self,
        mut compiler: T,
    ) -> FragmentId {
        let compiler = compiler.as_mut();

        compiler.push_fragment(SimpleTypeFragment::Facet(Facet::Length {
            value: self.0.value.clone(),
        }))
    }
}

impl ToSimpleFragments for xs::MinLength {
    fn to_simple_fragments<T: AsMut<SimpleTypeFragmentCompiler>>(
        &self,
        mut compiler: T,
    ) -> FragmentId {
        let compiler = compiler.as_mut();

        compiler.push_fragment(SimpleTypeFragment::Facet(Facet::MinLength {
            value: self.0.value.clone(),
        }))
    }
}

impl ToSimpleFragments for xs::MaxLength {
    fn to_simple_fragments<T: AsMut<SimpleTypeFragmentCompiler>>(
        &self,
        mut compiler: T,
    ) -> FragmentId {
        let compiler = compiler.as_mut();

        compiler.push_fragment(SimpleTypeFragment::Facet(Facet::MaxLength {
            value: self.0.value.clone(),
        }))
    }
}

impl ToSimpleFragments for xs::WhiteSpace {
    fn to_simple_fragments<T: AsMut<SimpleTypeFragmentCompiler>>(
        &self,
        mut compiler: T,
    ) -> FragmentId {
        let compiler = compiler.as_mut();

        compiler.push_fragment(SimpleTypeFragment::Facet(Facet::WhiteSpace {
            value: self.value.clone(),
        }))
    }
}

impl ToSimpleFragments for xs::Pattern {
    fn to_simple_fragments<T: AsMut<SimpleTypeFragmentCompiler>>(
        &self,
        mut compiler: T,
    ) -> FragmentId {
        let compiler = compiler.as_mut();

        compiler.push_fragment(SimpleTypeFragment::Facet(Facet::Pattern {
            value: self.value.clone(),
        }))
    }
}

impl ToSimpleFragments for xs::Assertion {
    fn to_simple_fragments<T: AsMut<SimpleTypeFragmentCompiler>>(
        &self,
        mut compiler: T,
    ) -> FragmentId {
        let compiler = compiler.as_mut();

        compiler.push_fragment(SimpleTypeFragment::Facet(Facet::Assertion {
            value: todo!(),
        }))
    }
}

impl ToSimpleFragments for xs::ExplicitTimezone {
    fn to_simple_fragments<T: AsMut<SimpleTypeFragmentCompiler>>(
        &self,
        mut compiler: T,
    ) -> FragmentId {
        let compiler = compiler.as_mut();

        compiler.push_fragment(SimpleTypeFragment::Facet(Facet::ExplicitTimezone {
            value: self.value.clone(),
        }))
    }
}

impl ToSimpleFragments for xs::List {
    fn to_simple_fragments<T: AsMut<SimpleTypeFragmentCompiler>>(
        &self,
        mut compiler: T,
    ) -> FragmentId {
        let mut compiler = compiler.as_mut();

        let item_ident = if let Some(item_type) = self.item_type.as_ref() {
            SimpleTypeIdent::Named(item_type.0.clone())
        } else {
            let simple_type = self.simple_type.as_ref().unwrap();

            SimpleTypeIdent::Anonymous(simple_type.to_simple_fragments(&mut compiler))
        };

        compiler.push_fragment(SimpleTypeFragment::List { item_ident })
    }
}

impl ToSimpleFragments for xs::union_items::SimpleType {
    fn to_simple_fragments<T: AsMut<SimpleTypeFragmentCompiler>>(&self, compiler: T) -> FragmentId {
        self.0.to_simple_fragments(compiler)
    }
}

impl ToSimpleFragments for xs::Union {
    fn to_simple_fragments<T: AsMut<SimpleTypeFragmentCompiler>>(
        &self,
        mut compiler: T,
    ) -> FragmentId {
        let mut compiler = compiler.as_mut();

        let fragments = self
            .simple_type
            .iter()
            .map(|simple_type| simple_type.to_simple_fragments(&mut compiler))
            .collect();

        compiler.push_fragment(SimpleTypeFragment::Union { fragments })
    }
}

impl ToSimpleFragments for xs::Restriction {
    fn to_simple_fragments<T: AsMut<SimpleTypeFragmentCompiler>>(
        &self,
        mut compiler: T,
    ) -> FragmentId {
        let mut compiler = compiler.as_mut();

        let base = self
            .base
            .as_ref()
            .map(|a| a.0.clone())
            .unwrap_or_else(|| todo!());

        let content_fragment = self
            .simple_restriction_model
            .simple_type
            .as_ref()
            .map(|simple_type| simple_type.to_simple_fragments(&mut compiler));

        let facets = self
            .simple_restriction_model
            .child_1
            .iter()
            .filter_map(|a| match a {
                //TODO
                xs::groups::simple_restriction_model_items::Child1::Facet(facet) => Some(facet),
                _ => None,
            })
            .map(|facet| facet.to_simple_fragments(&mut compiler))
            .collect();

        compiler.push_fragment(SimpleTypeFragment::Restriction(Restriction {
            base: SimpleTypeIdent::Named(base),
            content_fragment,
            facets,
        }))
    }
}

impl ToSimpleFragments for xs::groups::SimpleDerivation {
    fn to_simple_fragments<T: AsMut<SimpleTypeFragmentCompiler>>(&self, compiler: T) -> FragmentId {
        use xs::groups::SimpleDerivation as S;
        match self {
            S::Restriction(local_restriction) => local_restriction.to_simple_fragments(compiler),
            S::List(list) => list.to_simple_fragments(compiler),
            S::Union(union) => union.to_simple_fragments(compiler),
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use xmlity::{ExpandedName, LocalName};
//     use xs::{
//         Facet as XsdFacet, QName, SimpleDerivation, SimpleRestrictionType, TopLevelSimpleType,
//     };

//     use super::*;

//     #[test]
//     fn convert_annotated_to_fragments() {
//         let namespace = XmlNamespace::new_dangerous("http://localhost");

//         let mut fragment_compiler = SimpleTypeFragmentCompiler::new(namespace.clone());

//         // <xs:simpleType name="formChoice">
//         //     <xs:annotation>
//         //         <xs:documentation>
//         //     A utility type, not for public use</xs:documentation>
//         //     </xs:annotation>
//         //     <xs:restriction base="xs:NMTOKEN">
//         //         <xs:enumeration value="qualified"/>
//         //         <xs:enumeration value="unqualified"/>
//         //     </xs:restriction>
//         // </xs:simpleType>
//         let id = TopLevelSimpleType {
//             id: None,
//             name: LocalName::new_dangerous("annotated"),
//             final_: None,
//             annotation: None,
//             content: SimpleDerivation::Restriction(Box::new(SimpleRestrictionType {
//                 id: None,
//                 base: QName(ExpandedName::new(
//                     LocalName::new_dangerous("NMTOKEN"),
//                     Some(XmlNamespace::XS),
//                 )),
//                 annotation: None,
//                 simple_type: None,
//                 facets: vec![
//                     XsdFacet::Enumeration(Box::new(xs::Enumeration {
//                         fixed: None,
//                         value: "qualified".to_string(),
//                     })),
//                     XsdFacet::Enumeration(Box::new(xs::Enumeration {
//                         fixed: None,
//                         value: "unqualified".to_string(),
//                     })),
//                 ],
//             })),
//         }
//         .to_simple_fragments(&mut fragment_compiler);

//         assert_eq!(id, FragmentId(namespace, FragmentIdx(2)));
//         assert_eq!(fragment_compiler.fragments.len(), 3);

//         assert!(matches!(
//             fragment_compiler.fragments[&FragmentIdx(0)],
//             SimpleTypeFragment::Facet(Facet::Enumeration { .. })
//         ));
//         assert!(matches!(
//             fragment_compiler.fragments[&FragmentIdx(1)],
//             SimpleTypeFragment::Facet(Facet::Enumeration { .. })
//         ));
//         assert!(matches!(
//             fragment_compiler.fragments[&FragmentIdx(2)],
//             SimpleTypeFragment::Restriction(_)
//         ));

//         println!("{:#?}", fragment_compiler);
//     }
// }
