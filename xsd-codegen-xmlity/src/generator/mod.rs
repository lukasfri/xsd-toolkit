mod context;
use std::collections::BTreeMap;

pub use context::GeneratorContext;

mod scope;

use quote::format_ident;
pub use scope::GeneratorScope;

mod handler_container;
pub use handler_container::handler_container;
use syn::{parse_quote, Ident, Item, ItemMod};
use xmlity::ExpandedName;
use xsd_fragments::{
    fragments::{FragmentAccess, FragmentedXsdDocumentIdx},
    FragmentedXsdDocumentKey, TopLevelComplexType, TopLevelSimpleType, TopLevelType,
};

use crate::{
    augments::{self, ItemAugmentation, ItemAugmentationExt},
    complex::ComplexToTypeTemplate,
    generator::handler_container::HandlerContainer,
    misc::TypeReference,
    simple::SimpleToTypeTemplate,
    BoundType, Error, Result, ToIdentTypesExt, TypeType,
};

#[derive(Debug)]
pub struct Generator<'a> {
    pub context: &'a xsd_fragments::XmlnsContext,
    pub bound_namespaces: BTreeMap<FragmentedXsdDocumentIdx, syn::Path>,
    pub bound_types: BTreeMap<ExpandedName<'static>, BoundType>,
    pub bound_elements: BTreeMap<ExpandedName<'static>, TypeReference<'static>>,
    pub bound_attributes: BTreeMap<ExpandedName<'static>, TypeReference<'static>>,
    pub bound_groups: BTreeMap<ExpandedName<'static>, TypeReference<'static>>,
    pub augmenter: Box<dyn ItemAugmentation>,
    pub handlers: HandlerContainer,
}

impl<'a> Generator<'a> {
    pub fn new_with_augmenter<A: augments::ItemAugmentation + 'static>(
        context: &'a xsd_fragments::XmlnsContext,
        augmentation: A,
    ) -> Self {
        Self {
            context,
            bound_namespaces: BTreeMap::new(),
            bound_types: BTreeMap::new(),
            bound_elements: BTreeMap::new(),
            bound_attributes: BTreeMap::new(),
            bound_groups: BTreeMap::new(),
            augmenter: Box::new(augmentation),
            handlers: handler_container(),
        }
    }

    pub fn new(context: &'a xsd_fragments::XmlnsContext) -> Self {
        Self::new_with_augmenter(context, augments::NoopAugmentation::new())
    }

    pub fn bind_namespace(&mut self, namespace: FragmentedXsdDocumentKey, path: syn::Path) {
        let namespace_idx = self
            .context
            .namespace_idxs
            .get(&namespace)
            .expect("Namespace should exist");

        self.bound_namespaces.insert(*namespace_idx, path);
    }

    pub fn bind_namespace_idx(&mut self, namespace_idx: FragmentedXsdDocumentIdx, path: syn::Path) {
        self.bound_namespaces.insert(namespace_idx, path);
    }

    pub fn bind_type(
        &mut self,
        name: ExpandedName<'static>,
        bound_type: BoundType,
    ) -> Option<BoundType> {
        self.bound_types.insert(name, bound_type)
    }

    pub fn bind_types<T: IntoIterator<Item = (ExpandedName<'static>, BoundType)>>(
        &mut self,
        types: T,
    ) {
        types.into_iter().for_each(|(name, bound_type)| {
            self.bind_type(name, bound_type);
        });
    }

    pub fn bind_element(&mut self, name: ExpandedName<'static>, ty: TypeReference<'static>) {
        self.bound_elements.insert(name, ty);
    }

    pub fn bind_elements<
        T: IntoIterator<Item = (ExpandedName<'static>, TypeReference<'static>)>,
    >(
        &mut self,
        types: T,
    ) {
        types
            .into_iter()
            .for_each(|(name, bound_type)| self.bind_element(name, bound_type));
    }

    pub fn bind_attribute(&mut self, name: ExpandedName<'static>, ty: TypeReference<'static>) {
        self.bound_attributes.insert(name, ty);
    }

    pub fn bind_attributes<
        T: IntoIterator<Item = (ExpandedName<'static>, TypeReference<'static>)>,
    >(
        &mut self,
        types: T,
    ) {
        types
            .into_iter()
            .for_each(|(name, bound_type)| self.bind_attribute(name, bound_type));
    }

    pub fn bind_group(&mut self, name: ExpandedName<'static>, ty: TypeReference<'static>) {
        self.bound_groups.insert(name, ty);
    }

    pub fn bind_groups<T: IntoIterator<Item = (ExpandedName<'static>, TypeReference<'static>)>>(
        &mut self,
        types: T,
    ) {
        types
            .into_iter()
            .for_each(|(name, bound_type)| self.bind_group(name, bound_type));
    }

    pub fn generate_namespace(&mut self, key: &FragmentedXsdDocumentIdx) -> Result<Vec<Item>> {
        let mut items = Vec::new();

        let compiled_namespace = self
            .context
            .namespaces
            .get(key)
            .ok_or_else(|| Error::MissingKey { key: key.clone() })?;

        let types_module_name = format_ident!("types");

        items.extend(
            self.generate_types_module(key, &types_module_name)?
                .map(Item::Mod),
        );

        let attributes_module_name = format_ident!("attributes");

        items.extend(
            self.generate_attributes_module(key, &attributes_module_name)?
                .map(Item::Mod),
        );

        let groups_module_name = format_ident!("groups");

        items.extend(
            self.generate_groups_module(key, &groups_module_name)?
                .map(Item::Mod),
        );

        for expanded_name in compiled_namespace
            .top_level_elements
            .keys()
            .map(|local_name| {
                ExpandedName::new(local_name.as_ref(), compiled_namespace.namespace.clone())
                    .into_owned()
            })
        {
            if self.bound_elements.contains_key(&expanded_name) {
                continue;
            }
            let (mut ty, i) = self.generate_element(key, &expanded_name)?;

            let bound_namespace = self
                .bound_namespaces
                .get(key)
                .ok_or_else(|| Error::MissingKey { key: key.clone() })?;

            let path: syn::Path = parse_quote!(#bound_namespace);

            ty = TypeReference::new_static(ty.into_type(Some(&path)));

            self.bind_element(expanded_name, ty);
            items.extend(i)
        }

        Ok(items)
    }

    pub fn generate_simple_type(
        &self,
        key: &FragmentedXsdDocumentIdx,
        name: &xmlity::ExpandedName<'_>,
        simple_type: &TopLevelSimpleType,
    ) -> Result<(BoundType, Vec<Item>)> {
        let compiled_namespace = self
            .context
            .namespaces
            .get(key)
            .ok_or_else(|| Error::MissingKey { key: key.clone() })?;

        let fragment = compiled_namespace
            .complex_type_compiler
            .simple_type_compiler
            .get_fragment(&simple_type.root_fragment)
            .ok_or_else(|| Error::FragmentNotFound {
                fragment_type: "simple type fragment".to_string(),
            })?;

        let item_name = name.local_name().to_item_ident();
        let module_name = format_ident!("{}Items", item_name).to_path_ident();
        let namespace = name.namespace().ok_or(Error::NoNamespace)?;
        let context = GeneratorContext::new(self, namespace, key, item_name.clone());
        let mut scope = GeneratorScope::new(&self.augmenter);

        let type_ = self
            .handlers
            .simple_type_root_handler
            .to_type_template(&context, &mut scope, fragment)?;

        let mut items = Vec::new();

        items.extend(scope.finish_mod(&module_name).map(Item::Mod));

        let ty = type_.template.into_type(Some(&parse_quote!(#module_name)));

        let ty_item: syn::ItemType = parse_quote!(
            pub type #item_name = #ty;
        );

        items.push(Item::Type(ty_item));

        let ty = TypeReference::new_prefixed_type(parse_quote!(#item_name))
            .wrap(TypeReference::box_non_boxed_wrapper);

        let bound_type = BoundType {
            ty,
            ty_type: TypeType::Simple,
            serialize_with: None,
            deserialize_with: None,
        };

        Ok((bound_type, items))
    }

    pub fn generate_complex_type(
        &self,
        key: &FragmentedXsdDocumentIdx,
        name: &xmlity::ExpandedName<'_>,
        complex_type: &TopLevelComplexType,
    ) -> Result<(BoundType, Vec<Item>)> {
        let compiled_namespace = self
            .context
            .namespaces
            .get(key)
            .ok_or_else(|| Error::MissingKey { key: key.clone() })?;

        let fragment = compiled_namespace
            .complex_type_compiler
            .get_fragment(&complex_type.root_fragment)
            .ok_or_else(|| Error::FragmentNotFound {
                fragment_type: "complex type fragment".to_string(),
            })?;

        let item_name = name.local_name().to_item_ident();
        let module_name = format_ident!("{}Items", item_name).to_path_ident();
        let namespace = name.namespace().ok_or(Error::NoNamespace)?;
        let context = GeneratorContext::new(self, namespace, key, item_name.clone());
        let mut scope = GeneratorScope::new(&self.augmenter);

        let type_ = self
            .handlers
            .complex_type_root_handler
            .to_type_template(&context, &mut scope, fragment)?;

        let mut items = Vec::new();

        let mut item = type_
            .template
            .to_struct(&item_name, Some(&parse_quote!(#module_name)));

        let augment_items = self.augmenter.augment_struct(&mut item);

        items.extend(scope.finish_mod(&module_name).map(Item::Mod));

        items.push(Item::Struct(item));

        items.extend(augment_items);

        let ty = TypeReference::new_prefixed_type(parse_quote!(#item_name))
            .wrap(TypeReference::box_non_boxed_wrapper);

        let bound_type = BoundType {
            ty,
            ty_type: TypeType::Complex,
            serialize_with: None,
            deserialize_with: None,
        };

        Ok((bound_type, items))
    }

    pub fn generate_type(
        &self,
        key: &FragmentedXsdDocumentIdx,
        name: &ExpandedName<'_>,
    ) -> Result<(BoundType, Vec<Item>)> {
        let compiled_namespace = self
            .context
            .namespaces
            .get(key)
            .ok_or_else(|| Error::MissingKey { key: key.clone() })?;

        let type_ = compiled_namespace
            .top_level_types
            .get(name.local_name())
            .ok_or_else(|| Error::MissingType {
                name: name.clone().into_owned(),
            })?;

        match type_ {
            xsd_fragments::TopLevelType::Simple(simple_type) => {
                self.generate_simple_type(key, name, simple_type)
            }
            xsd_fragments::TopLevelType::Complex(complex_type) => {
                self.generate_complex_type(key, name, complex_type)
            }
        }
    }

    pub fn generate_types_module(
        &mut self,
        key: &FragmentedXsdDocumentIdx,
        module_name: &Ident,
    ) -> Result<Option<ItemMod>> {
        let compiled_namespace = self
            .context
            .namespaces
            .get(key)
            .ok_or_else(|| Error::MissingKey { key: key.clone() })?;

        let simple_types = compiled_namespace
            .top_level_types
            .iter()
            .filter_map(|(key, type_)| match type_ {
                TopLevelType::Simple(simple_type) => Some((
                    ExpandedName::new(key.as_ref(), compiled_namespace.namespace.clone())
                        .into_owned(),
                    simple_type,
                )),
                _ => None,
            })
            .filter_map(|(expanded_name, type_)| {
                if self.bound_types.contains_key(&expanded_name) {
                    // We don't want to generate types that are already bound.
                    return None;
                }

                let (mut bound_type, i) =
                    match self.generate_simple_type(key, &expanded_name, type_) {
                        Ok(ok) => ok,
                        Err(err) => return Some(Err(err)),
                    };

                let bound_namespace = self
                    .bound_namespaces
                    .get(key)
                    .ok_or_else(|| Error::MissingKey { key: key.clone() });

                let bound_namespace = match bound_namespace {
                    Ok(bound_namespace) => bound_namespace,
                    Err(err) => return Some(Err(err)),
                };

                let path: syn::Path = parse_quote!(#bound_namespace::#module_name);

                bound_type.ty = TypeReference::new_static(bound_type.ty.into_type(Some(&path)));

                self.bind_type(expanded_name, bound_type);

                Some(Ok(i))
            })
            .collect::<Result<Vec<_>>>()?;

        let complex_types = compiled_namespace
            .top_level_types
            .iter()
            .filter_map(|(key, type_)| match type_ {
                TopLevelType::Complex(complex_type) => Some((
                    ExpandedName::new(key.as_ref(), compiled_namespace.namespace.clone())
                        .into_owned(),
                    complex_type,
                )),
                _ => None,
            })
            // Remove abstract types
            .filter(|(_, complex_type)| {
                let fragment = compiled_namespace
                    .complex_type_compiler
                    .get_fragment(&complex_type.root_fragment)
                    .expect("Fragment should exist");

                matches!(fragment.abstract_, Some(false) | None)
            })
            .map(|(expanded_name, complex_type)| {
                let (mut bound_type, i) =
                    self.generate_complex_type(key, &expanded_name, complex_type)?;

                let bound_namespace = self
                    .bound_namespaces
                    .get(key)
                    .ok_or_else(|| Error::MissingKey { key: key.clone() })?;

                let path: syn::Path = parse_quote!(#bound_namespace::#module_name);

                bound_type.ty = TypeReference::new_static(bound_type.ty.into_type(Some(&path)));

                self.bind_type(expanded_name, bound_type);

                Ok(i)
            })
            .collect::<Result<Vec<_>>>()?;

        // First we resolve simple types
        let type_items = simple_types
            .into_iter()
            .chain(complex_types)
            .flatten()
            .collect::<Vec<_>>();

        if type_items.is_empty() {
            return Ok(None);
        }

        let item_mod: ItemMod = parse_quote!(
            pub mod #module_name {
                #(#type_items)*
            }
        );

        Ok(Some(item_mod))
    }

    pub fn generate_attribute(
        &self,
        key: &FragmentedXsdDocumentIdx,
        name: &xmlity::ExpandedName<'_>,
    ) -> Result<(TypeReference<'static>, Vec<Item>)> {
        let compiled_namespace = self
            .context
            .namespaces
            .get(key)
            .ok_or_else(|| Error::MissingKey { key: key.clone() })?;

        let attribute = compiled_namespace
            .top_level_attributes
            .get(name.local_name())
            .ok_or_else(|| Error::MissingAttribute {
                name: name.clone().into_owned(),
            })?;

        let fragment = compiled_namespace
            .complex_type_compiler
            .get_fragment(&attribute.root_fragment)
            .ok_or_else(|| Error::FragmentNotFound {
                fragment_type: "attribute fragment".to_string(),
            })?;

        let item_name = name.local_name().to_item_ident();
        let module_name = format_ident!("{}Items", item_name).to_path_ident();
        let namespace = name.namespace().ok_or(Error::NoNamespace)?;
        let context = GeneratorContext::new(self, namespace, key, item_name.clone());
        let mut scope = GeneratorScope::new(&self.augmenter);

        let type_ = self
            .handlers
            .top_level_attribute_handler
            .to_type_template(&context, &mut scope, fragment)?;

        let mut items = Vec::new();

        let mut item = type_
            .template
            .to_struct(&item_name, Some(&parse_quote!(#module_name)));

        let augment_items = self.augmenter.augment_struct(&mut item);

        items.extend(scope.finish_mod(&module_name).map(Item::Mod));

        items.push(Item::Struct(item));

        items.extend(augment_items);

        let type_ = TypeReference::new_prefixed_type(parse_quote!(#item_name));

        Ok((type_, items))
    }

    pub fn generate_attributes_module(
        &mut self,
        key: &FragmentedXsdDocumentIdx,
        module_name: &syn::Ident,
    ) -> Result<Option<ItemMod>> {
        let compiled_namespace = self
            .context
            .namespaces
            .get(key)
            .ok_or_else(|| Error::MissingKey { key: key.clone() })?;

        let attributes_items = compiled_namespace
            .top_level_attributes
            .keys()
            .map(|local_name| {
                ExpandedName::new(local_name.as_ref(), compiled_namespace.namespace.clone())
                    .into_owned()
            })
            .map(|expanded_name| {
                let (mut bound_type, i) = self.generate_attribute(key, &expanded_name)?;

                let bound_namespace = self
                    .bound_namespaces
                    .get(key)
                    .ok_or_else(|| Error::MissingKey { key: key.clone() })?;

                let path: syn::Path = parse_quote!(#bound_namespace::#module_name);

                bound_type = TypeReference::new_static(bound_type.into_type(Some(&path)));

                self.bind_attribute(expanded_name, bound_type);

                Ok(i)
            })
            .collect::<Result<Vec<_>>>()?
            .into_iter()
            .flatten()
            .collect::<Vec<_>>();

        if attributes_items.is_empty() {
            return Ok(None);
        }

        let attributes_mod: ItemMod = parse_quote!(
            pub mod #module_name {
                #(#attributes_items)*
            }
        );

        Ok(Some(attributes_mod))
    }

    pub fn generate_element(
        &self,
        key: &FragmentedXsdDocumentIdx,
        name: &xmlity::ExpandedName<'_>,
    ) -> Result<(TypeReference<'static>, Vec<Item>)> {
        let compiled_namespace = self
            .context
            .namespaces
            .get(key)
            .ok_or_else(|| Error::MissingKey { key: key.clone() })?;

        let element = compiled_namespace
            .top_level_elements
            .get(name.local_name())
            .ok_or_else(|| Error::MissingElement {
                name: name.clone().into_owned(),
            })?;

        let item_name = name.local_name().to_item_ident();
        let module_name = format_ident!("{}Items", item_name).to_path_ident();
        let namespace = name.namespace().ok_or(Error::NoNamespace)?;
        let context = GeneratorContext::new(self, namespace, key, item_name.clone());
        let mut scope = GeneratorScope::new(&self.augmenter);

        let type_ = self.handlers.top_level_element_handler.to_type_template(
            &context,
            &mut scope,
            &element.root_fragment,
        )?;

        let mut items = Vec::new();

        let mut item = type_
            .template
            .to_item(&item_name, Some(&parse_quote!(#module_name)));

        let augment_items = self.augmenter.augment_item(&mut item);

        items.extend(scope.finish_mod(&module_name).map(Item::Mod));

        items.push(item);

        items.extend(augment_items);

        let type_ = TypeReference::new_prefixed_type(parse_quote!(#item_name));

        Ok((type_, items))
    }

    pub fn generate_group(
        &self,
        key: &FragmentedXsdDocumentIdx,
        name: &xmlity::ExpandedName<'_>,
    ) -> Result<(TypeReference<'static>, Vec<Item>)> {
        let compiled_namespace = self
            .context
            .namespaces
            .get(key)
            .ok_or_else(|| Error::MissingKey { key: key.clone() })?;

        let group = compiled_namespace
            .top_level_groups
            .get(name.local_name())
            .ok_or_else(|| Error::MissingGroup {
                name: name.clone().into_owned(),
            })?;

        let fragment = compiled_namespace
            .complex_type_compiler
            .get_fragment(&group.root_fragment)
            .ok_or_else(|| Error::FragmentNotFound {
                fragment_type: "group fragment".to_string(),
            })?;

        let item_name = name.local_name().to_item_ident();
        let module_name = format_ident!("{}Items", item_name).to_path_ident();
        let namespace = name.namespace().ok_or(Error::NoNamespace)?;
        let context = GeneratorContext::new(self, namespace, key, item_name.clone());
        let mut scope = GeneratorScope::new(&self.augmenter);

        let type_ = self
            .handlers
            .top_level_group_handler
            .to_type_template(&context, &mut scope, fragment)?;

        let mut items = Vec::new();

        let mut item = type_
            .template
            .to_item(&item_name, Some(&parse_quote!(#module_name)));

        let augment_items = self.augmenter.augment_item(&mut item);

        items.extend(scope.finish_mod(&module_name).map(Item::Mod));

        items.push(item);

        items.extend(augment_items);

        let type_ = TypeReference::new_prefixed_type(parse_quote!(#item_name));

        Ok((type_, items))
    }

    pub fn generate_groups_module(
        &mut self,
        key: &FragmentedXsdDocumentIdx,
        groups_module_name: &syn::Ident,
    ) -> Result<Option<ItemMod>> {
        let compiled_namespace = self
            .context
            .namespaces
            .get(key)
            .ok_or_else(|| Error::MissingKey { key: key.clone() })?;

        let group_items = compiled_namespace
            .top_level_groups
            .keys()
            .map(|local_name| {
                ExpandedName::new(local_name.as_ref(), compiled_namespace.namespace.clone())
                    .into_owned()
            })
            .map(|expanded_name| {
                let (mut bound_type, i) = self.generate_group(key, &expanded_name)?;

                let bound_namespace = self
                    .bound_namespaces
                    .get(key)
                    .ok_or_else(|| Error::MissingKey { key: key.clone() })?;

                let path: syn::Path = parse_quote!(#bound_namespace::#groups_module_name);

                bound_type = TypeReference::new_static(bound_type.into_type(Some(&path)));

                self.bind_group(expanded_name, bound_type);

                Ok(i)
            })
            .collect::<Result<Vec<_>>>()?
            .into_iter()
            .flatten()
            .collect::<Vec<_>>();

        if group_items.is_empty() {
            return Ok(None);
        }

        let groups_mod: ItemMod = parse_quote!(
            pub mod #groups_module_name {
                #(#group_items)*
            }
        );

        Ok(Some(groups_mod))
    }
}
