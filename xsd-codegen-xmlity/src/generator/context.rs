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
        FragmentIdx,
    },
    CompiledNamespace,
};

use crate::simple::SimpleToTypeTemplate;
use crate::Result;

#[derive(Debug)]
pub struct GeneratorContext<'a> {
    generator: &'a Generator<'a>,
    namespace: &'a XmlNamespace<'a>,
    suggested_ident: Ident,
}

impl<'a> GeneratorContext<'a> {
    pub fn new(
        generator: &'a Generator<'a>,
        namespace: &'a XmlNamespace<'a>,
        suggested_ident: Ident,
    ) -> Self {
        Self {
            generator,
            namespace,
            suggested_ident,
        }
    }

    pub fn current_namespace(&self) -> Result<&CompiledNamespace> {
        self.generator
            .context
            .get_namespace(self.namespace)
            .ok_or_else(|| Error::MissingNamespace {
                namespace: self.namespace.clone().into_owned(),
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
        Self::new(self.generator, self.namespace, suggested_ident)
    }

    fn suggested_ident(&self) -> &Ident {
        &self.suggested_ident
    }

    fn get_fragment<F, S: Scope>(
        &self,
        fragment_id: &xsd_fragments::fragments::FragmentIdx<F>,
        _: &mut S,
    ) -> Result<&F>
    where
        SimpleTypeFragmentCompiler: FragmentAccess<F>,
    {
        let namespace = self.current_namespace()?;
        namespace
            .complex_type
            .simple_type_fragments
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
        let fragment = self.get_fragment(fragment_id, scope)?;

        handler.to_type_template(self, scope, fragment)
    }

    fn resolve_named_type(&self, name: &ExpandedName<'_>) -> Result<BoundType> {
        if let Some(bound_type) = self.generator.bound_types.get(&name.as_ref()) {
            return Ok(bound_type.clone());
        }

        let type_mod_ident = format_ident!("types");

        let namespace = name.namespace().ok_or(Error::NoNamespace)?;
        let namespace_crate = self
            .generator
            .bound_namespaces
            .get(namespace)
            .ok_or_else(|| Error::UnboundNamespace {
                namespace: Some(namespace.clone().into_owned()),
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
}

impl<'c> complex::ComplexContext for GeneratorContext<'c> {
    type SimpleContext = GeneratorContext<'c>;
    type SubContext = GeneratorContext<'c>;

    fn simple_context(&self) -> &Self::SimpleContext {
        self
    }

    fn sub_context(&self, suggested_ident: Ident) -> Self::SubContext {
        Self::new(self.generator, self.namespace, suggested_ident)
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

    fn resolve_type_template<I, H: ComplexToTypeTemplate<I>, S: Scope>(
        &self,
        fragment_id: &xsd_fragments::fragments::FragmentIdx<I>,
        scope: &mut S,
        handler: &H,
    ) -> Result<ToTypeTemplateData<H::TypeTemplate>>
    where
        ComplexTypeFragmentCompiler: FragmentAccess<I>,
    {
        let namespace = self
            .generator
            .context
            .namespaces
            .get(&fragment_id.namespace_idx())
            .ok_or_else(|| Error::FragmentNotFound {
                fragment_type: "namespace for complex type fragment".to_string(),
            })?;

        let fragment = namespace
            .complex_type
            .get_fragment(fragment_id)
            .ok_or_else(|| Error::FragmentNotFound {
                fragment_type: "complex type fragment".to_string(),
            })?;

        handler.to_type_template(self, scope, fragment)
    }

    fn resolve_named_type(&self, name: &ExpandedName<'_>) -> Result<BoundType> {
        if let Some(bound_type) = self.generator.bound_types.get(&name.as_ref()) {
            return Ok(bound_type.clone());
        }

        let type_mod_ident = format_ident!("types");

        let namespace = name.namespace().ok_or(Error::NoNamespace)?;
        let namespace_crate = self
            .generator
            .bound_namespaces
            .get(namespace)
            .ok_or_else(|| Error::UnboundNamespace {
                namespace: Some(namespace.clone().into_owned()),
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

    fn resolve_named_element(&self, name: &ExpandedName<'_>) -> Result<TypeReference<'static>> {
        if let Some(ty) = self.generator.bound_elements.get(&name.as_ref()).cloned() {
            return Ok(ty);
        }

        let namespace = name.namespace().ok_or(Error::NoNamespace)?;
        let namespace_crate = self
            .generator
            .bound_namespaces
            .get(namespace)
            .ok_or_else(|| Error::UnboundNamespace {
                namespace: Some(namespace.clone().into_owned()),
                item_name: Some(name.local_name().to_string()),
            })?;

        let name = name.local_name().to_item_ident();
        let ty: syn::Type = parse_quote!(#namespace_crate::#name);

        let ty = TypeReference::new_static(ty).wrap(TypeReference::box_wrapper);

        Ok(ty)
    }

    fn resolve_named_attribute(&self, name: &ExpandedName<'_>) -> Result<TypeReference<'static>> {
        if let Some(ty) = self.generator.bound_attributes.get(&name.as_ref()).cloned() {
            return Ok(ty);
        }

        let attribute_mod_ident = format_ident!("attributes");

        let namespace = name.namespace().ok_or(Error::NoNamespace)?;
        let namespace_crate = self
            .generator
            .bound_namespaces
            .get(namespace)
            .ok_or_else(|| Error::UnboundNamespace {
                namespace: Some(namespace.clone().into_owned()),
                item_name: Some(format!("attribute {}", name.local_name())),
            })?;

        let name = name.local_name().to_item_ident();
        let ty: syn::Type = parse_quote!(#namespace_crate::#attribute_mod_ident::#name);

        let ty = TypeReference::new_static(ty).wrap(TypeReference::box_wrapper);

        Ok(ty)
    }

    fn resolve_named_group(&self, name: &ExpandedName<'_>) -> Result<TypeReference<'static>> {
        if let Some(ty) = self.generator.bound_groups.get(&name.as_ref()).cloned() {
            return Ok(ty);
        }

        let group_mod_ident = format_ident!("groups");

        let namespace = name.namespace().ok_or(Error::NoNamespace)?;
        let namespace_crate = self
            .generator
            .bound_namespaces
            .get(namespace)
            .ok_or_else(|| Error::UnboundNamespace {
                namespace: Some(namespace.clone().into_owned()),
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
                            .complex_type
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
