use crate::complex::{ComplexTypeFragment, FragmentId as ComplexFragmentId};
use crate::simple::{FragmentId as SimpleFragmentId, SimpleTypeFragment};
use crate::ComplexTypeIdent;

use xmlity::ExpandedName;
use xmlity::XmlNamespace;

pub trait XmlnsContextTransformer {
    fn transform(self, context: Context<'_>);
}

pub struct Context<'a> {
    pub namespace: &'a XmlNamespace<'static>,
    pub xmlns_context: &'a mut crate::XmlnsContext,
}

impl Context<'_> {
    pub fn current_namespace(&self) -> &crate::CompiledNamespace {
        self.xmlns_context
            .namespaces
            .get(self.namespace)
            .expect("Current namespace not found")
    }

    pub fn current_namespace_mut(&mut self) -> &mut crate::CompiledNamespace {
        self.xmlns_context
            .namespaces
            .get_mut(self.namespace)
            .expect("Current namespace not found")
    }

    pub fn iter_complex_fragment_ids(&self) -> impl Iterator<Item = ComplexFragmentId> {
        self.current_namespace().complex_type.iter_fragment_ids()
    }

    pub fn get_complex_fragment(
        &self,
        fragment_idx: &ComplexFragmentId,
    ) -> Option<&ComplexTypeFragment> {
        self.current_namespace()
            .complex_type
            .get_fragment(fragment_idx)
    }

    pub fn get_complex_fragment_mut(
        &mut self,
        fragment_idx: &ComplexFragmentId,
    ) -> Option<&mut ComplexTypeFragment> {
        self.current_namespace_mut()
            .complex_type
            .get_fragment_mut(fragment_idx)
    }

    pub fn iter_simple_fragment_ids(&self) -> impl Iterator<Item = SimpleFragmentId> {
        self.current_namespace()
            .complex_type
            .simple_type_fragments
            .iter_fragment_ids()
    }

    pub fn get_simple_fragment(
        &self,
        fragment_idx: &SimpleFragmentId,
    ) -> Option<&SimpleTypeFragment> {
        self.current_namespace()
            .complex_type
            .simple_type_fragments
            .get_fragment(fragment_idx)
    }

    pub fn simple_fragment_mut(
        &mut self,
        fragment_idx: &SimpleFragmentId,
    ) -> Option<&mut SimpleTypeFragment> {
        self.current_namespace_mut()
            .complex_type
            .simple_type_fragments
            .get_fragment_mut(fragment_idx)
    }

    pub fn get_named_type<'a>(
        &'a self,
        name: &'a ExpandedName<'_>,
    ) -> Option<&'a crate::TopLevelType> {
        self.xmlns_context
            .namespaces
            .get(name.namespace().as_ref().unwrap())
            .unwrap()
            .top_level_types
            .get(name.local_name())
    }

    pub fn get_complex_fragment_from_ident<'a>(
        &'a self,
        name: &'a ComplexTypeIdent,
    ) -> Option<&'a ComplexTypeFragment> {
        let fragment_id = match name {
            ComplexTypeIdent::Named(expanded_name) => {
                match self.get_named_type(expanded_name).unwrap() {
                    crate::TopLevelType::Complex(complex) => &complex.root_fragment,
                    crate::TopLevelType::Simple(_) => unreachable!(),
                }
            }
            ComplexTypeIdent::Anonymous(fragment_id) => fragment_id,
        };

        self.get_complex_fragment(fragment_id)
    }
}
