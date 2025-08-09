use crate::{
    complex, generator::Generator, misc::TypeReference, simple, BoundType, Error, Scope,
    ToIdentTypesExt, ToTypeTemplateData, TypeType,
};
use complex::ComplexToTypeTemplate;
use quote::format_ident;
use syn::{parse_quote, Ident};
use xmlity::{ExpandedName, LocalName, XmlNamespace};
use xsd_fragments::{
    fragments::{
        complex::ComplexTypeFragmentCompiler, simple::SimpleTypeFragmentCompiler, FragmentAccess,
        FragmentIdx, FragmentedXsdDocumentIdx,
    },
    FragmentedXsdDocument,
};

use crate::simple::SimpleToTypeTemplate;
use crate::Result;

#[derive(Debug)]
pub struct GeneratorContext<'a> {
    generator: &'a Generator<'a>,
    namespace: &'a XmlNamespace<'a>,
    key: &'a FragmentedXsdDocumentIdx,
    suggested_ident: Ident,
}

impl<'a> GeneratorContext<'a> {
    pub fn new(
        generator: &'a Generator<'a>,
        namespace: &'a XmlNamespace<'a>,
        key: &'a FragmentedXsdDocumentIdx,
        suggested_ident: Ident,
    ) -> Self {
        Self {
            generator,
            namespace,
            key,
            suggested_ident,
        }
    }

    pub fn current_namespace(&self) -> Result<&FragmentedXsdDocument> {
        self.generator
            .context
            .namespaces
            .get(self.key)
            .ok_or_else(|| Error::MissingKey {
                key: self.key.clone(),
            })
    }
}

impl<'c> simple::SimpleContext for GeneratorContext<'c> {
    type SubContext = GeneratorContext<'c>;

    fn namespace(&self) -> &XmlNamespace<'_> {
        self.namespace
    }

    fn to_expanded_name(&self, local_name: xmlity::LocalName<'static>) -> ExpandedName<'static> {
        ExpandedName::new(local_name, Some(self.namespace().clone().into_owned()))
    }

    fn sub_context(&self, suggested_ident: Ident) -> Self::SubContext {
        Self::new(self.generator, self.namespace, self.key, suggested_ident)
    }

    fn suggested_ident(&self) -> &Ident {
        &self.suggested_ident
    }

    fn get_fragment<F>(&self, fragment_id: &xsd_fragments::fragments::FragmentIdx<F>) -> Result<&F>
    where
        SimpleTypeFragmentCompiler: FragmentAccess<F>,
    {
        let namespace = self.current_namespace()?;
        namespace
            .complex_type_compiler
            .simple_type_compiler
            .get_fragment(fragment_id)
            .ok_or_else(|| Error::FragmentNotFound {
                fragment_type: "simple type fragment".to_string(),
            })
    }

    fn resolve_type_template<I, H: SimpleToTypeTemplate<I>, S: Scope>(
        &self,
        fragment_id: &FragmentIdx<I>,
        scope: &mut S,
        handler: &H,
    ) -> Result<ToTypeTemplateData<H::TypeTemplate>>
    where
        SimpleTypeFragmentCompiler: FragmentAccess<I>,
    {
        let fragment = self.get_fragment(fragment_id)?;

        handler.to_type_template(self, scope, fragment)
    }

    fn resolve_named_type(
        &self,
        key: &FragmentedXsdDocumentIdx,
        name: &ExpandedName<'_>,
    ) -> Result<BoundType> {
        if let Some(bound_type) = self.generator.bound_types.get(&name.as_ref()) {
            return Ok(bound_type.clone());
        }

        let type_mod_ident = format_ident!("types");

        let referenced_namespace_idx = self
            .generator
            .context
            .resolve_ref_namespace(key, name.namespace())
            .expect("Namespace should exist");

        let namespace_crate = self
            .generator
            .bound_namespaces
            .get(referenced_namespace_idx)
            .ok_or_else(|| Error::UnboundNamespace {
                namespace: *referenced_namespace_idx,
                item_name: Some(name.local_name().to_string()),
            })?;

        let name = name.local_name().to_item_ident();
        let ty: syn::Type = parse_quote!(#namespace_crate::#type_mod_ident::#name);

        let ty = TypeReference::new_static(ty).wrap(TypeReference::box_wrapper);

        Ok(BoundType {
            ty,
            ty_type: TypeType::Simple,
            serialize_with: None,
            deserialize_with: None,
        })
    }

    fn namespace_idx(&self) -> &FragmentedXsdDocumentIdx {
        &self.key
    }
}

impl<'c> complex::ComplexContext for GeneratorContext<'c> {
    type SimpleContext = GeneratorContext<'c>;
    type SubContext = GeneratorContext<'c>;

    fn simple_context(&self) -> &Self::SimpleContext {
        self
    }

    fn sub_context(&self, suggested_ident: Ident) -> Self::SubContext {
        Self::new(self.generator, self.namespace, self.key, suggested_ident)
    }

    fn suggested_ident(&self) -> &Ident {
        <Self as simple::SimpleContext>::suggested_ident(self)
    }

    fn namespace(&self) -> &XmlNamespace<'_> {
        <Self as simple::SimpleContext>::namespace(self)
    }

    fn to_expanded_name(&self, name: LocalName<'static>) -> ExpandedName<'static> {
        <Self as simple::SimpleContext>::to_expanded_name(self, name)
    }

    fn get_fragment<F>(&self, fragment_id: &FragmentIdx<F>) -> Result<&F>
    where
        ComplexTypeFragmentCompiler: FragmentAccess<F>,
    {
        let namespace = self
            .generator
            .context
            .namespaces
            .get(&fragment_id.namespace_idx())
            .ok_or_else(|| Error::FragmentNotFound {
                fragment_type: "namespace for complex type fragment".to_string(),
            })?;

        namespace
            .complex_type_compiler
            .get_fragment(fragment_id)
            .ok_or_else(|| Error::FragmentNotFound {
                fragment_type: "complex type fragment".to_string(),
            })
    }

    fn resolve_type_template<I, H: ComplexToTypeTemplate<I>, S: Scope>(
        &self,
        fragment_id: &xsd_fragments::fragments::FragmentIdx<I>,
        scope: &mut S,
        handler: &H,
    ) -> Result<ToTypeTemplateData<H::TypeTemplate>>
    where
        ComplexTypeFragmentCompiler: FragmentAccess<I>,
    {
        handler.to_type_template(self, scope, self.get_fragment(fragment_id)?)
    }

    fn resolve_named_type(
        &self,
        namespace_idx: &FragmentedXsdDocumentIdx,
        name: &ExpandedName<'_>,
    ) -> Result<BoundType> {
        if let Some(bound_type) = self.generator.bound_types.get(&name.as_ref()) {
            return Ok(bound_type.clone());
        }

        let type_mod_ident = format_ident!("types");

        let referenced_namespace_idx = self
            .generator
            .context
            .resolve_ref_namespace(namespace_idx, name.namespace())
            .expect("Namespace should exist");

        let namespace_crate = self
            .generator
            .bound_namespaces
            .get(referenced_namespace_idx)
            .ok_or_else(|| Error::UnboundNamespace {
                namespace: *referenced_namespace_idx,
                item_name: Some(name.local_name().to_string()),
            })?;

        let name = name.local_name().to_item_ident();
        let ty: syn::Type = parse_quote!(#namespace_crate::#type_mod_ident::#name);

        let ty = TypeReference::new_static(ty).wrap(TypeReference::box_wrapper);

        Ok(BoundType {
            ty,
            ty_type: TypeType::Complex,
            serialize_with: None,
            deserialize_with: None,
        })
    }

    fn resolve_named_element(
        &self,
        namespace_idx: &FragmentedXsdDocumentIdx,
        name: &ExpandedName<'_>,
    ) -> Result<TypeReference<'static>> {
        if let Some(ty) = self.generator.bound_elements.get(&name.as_ref()).cloned() {
            return Ok(ty);
        }

        let referenced_namespace_idx = self
            .generator
            .context
            .resolve_ref_namespace(namespace_idx, name.namespace())
            .expect("Namespace should exist");

        let namespace_crate = self
            .generator
            .bound_namespaces
            .get(referenced_namespace_idx)
            .ok_or_else(|| Error::UnboundNamespace {
                namespace: *referenced_namespace_idx,
                item_name: Some(name.local_name().to_string()),
            })?;

        let name = name.local_name().to_item_ident();
        let ty: syn::Type = parse_quote!(#namespace_crate::#name);

        let ty = TypeReference::new_static(ty).wrap(TypeReference::box_wrapper);

        Ok(ty)
    }

    fn resolve_named_attribute(
        &self,
        namespace_idx: &FragmentedXsdDocumentIdx,
        name: &ExpandedName<'_>,
    ) -> Result<TypeReference<'static>> {
        if let Some(ty) = self.generator.bound_attributes.get(&name.as_ref()).cloned() {
            return Ok(ty);
        }

        let attribute_mod_ident = format_ident!("attributes");

        let referenced_namespace_idx = self
            .generator
            .context
            .resolve_ref_namespace(namespace_idx, name.namespace())
            .expect("Namespace should exist");

        let namespace_crate = self
            .generator
            .bound_namespaces
            .get(referenced_namespace_idx)
            .ok_or_else(|| Error::UnboundNamespace {
                namespace: *referenced_namespace_idx,
                item_name: Some(format!("attribute {}", name.local_name())),
            })?;

        let name = name.local_name().to_item_ident();
        let ty: syn::Type = parse_quote!(#namespace_crate::#attribute_mod_ident::#name);

        let ty = TypeReference::new_static(ty).wrap(TypeReference::box_wrapper);

        Ok(ty)
    }

    fn resolve_named_group(
        &self,
        namespace_idx: &FragmentedXsdDocumentIdx,
        name: &ExpandedName<'_>,
    ) -> Result<TypeReference<'static>> {
        if let Some(ty) = self.generator.bound_groups.get(&name.as_ref()).cloned() {
            return Ok(ty);
        }

        let group_mod_ident = format_ident!("groups");

        let referenced_namespace_idx = self
            .generator
            .context
            .resolve_ref_namespace(namespace_idx, name.namespace())
            .expect("Namespace should exist");

        let namespace_crate = self
            .generator
            .bound_namespaces
            .get(namespace_idx)
            .ok_or_else(|| Error::UnboundNamespace {
                namespace: *referenced_namespace_idx,
                item_name: Some(format!("group {}", name.local_name())),
            })?;

        let name = name.local_name().to_item_ident();
        let ty: syn::Type = parse_quote!(#namespace_crate::#group_mod_ident::#name);

        let ty = TypeReference::new_static(ty).wrap(TypeReference::box_wrapper);

        Ok(ty)
    }

    fn substitution_group_members(
        &self,
        name: &ExpandedName<'_>,
    ) -> Result<impl Iterator<Item = ExpandedName<'_>>> {
        let members: Vec<ExpandedName<'_>> = self
            .generator
            .context
            .namespaces
            .iter()
            .flat_map(|(_, namespace)| {
                namespace
                    .top_level_elements
                    .iter()
                    .filter_map(|(key, fragment_id)| {
                        let fragment = namespace
                            .complex_type_compiler
                            .get_fragment(&fragment_id.root_fragment)?;

                        if fragment.substitution_groups.contains(name) {
                            Some(ExpandedName::new(
                                key.as_ref(),
                                Some(namespace.namespace.as_ref()),
                            ))
                        } else {
                            None
                        }
                    })
            })
            .collect();

        Ok(members.into_iter())
    }
}
