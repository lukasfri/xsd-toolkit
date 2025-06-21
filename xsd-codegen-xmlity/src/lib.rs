pub mod augments;
pub mod binds;
mod complex;
pub mod misc;
mod simple;
pub mod templates;

use std::{collections::BTreeMap, convert::Infallible, ops::Deref};

use complex::{Scope, ToTypeTemplate, ToTypeTemplateData};
use inflector::Inflector;
use misc::TypeReference;
use quote::format_ident;
use syn::{parse_quote, Ident, Item, ItemMod};
use xmlity::{ExpandedName, LocalName, XmlNamespace};
use xsd_type_compiler::{
    complex::{
        transformers::{
            ExpandAttributeDeclarations, ExpandExtensionFragments, ExpandRestrictionFragments,
            FlattenNestedAll, FlattenNestedChoices, FlattenNestedSequences,
            RemoveProhibitedAttributes,
        },
        ComplexTypeFragmentCompiler, FragmentAccess,
    },
    transformers::{TransformChange, XmlnsLocalTransformer},
    CompiledNamespace, TopLevelType,
};

use crate::augments::{ItemAugmentation, ItemAugmentationExt};

#[derive(Debug, Clone, PartialEq)]
pub struct Error {}
pub type Result<T> = std::result::Result<T, Error>;

#[non_exhaustive]
/// This transformer is used to transform the XSD into a form that is required for the codegen to work.
pub struct XmlityCodegenTransformer {}

impl XmlityCodegenTransformer {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {}
    }
}

impl XmlnsLocalTransformer for XmlityCodegenTransformer {
    type Error = Infallible;
    fn transform(
        self,
        mut context: xsd_type_compiler::transformers::XmlnsLocalTransformerContext<'_>,
    ) -> std::result::Result<TransformChange, Self::Error> {
        for i in 0..100 {
            let mut total_change = TransformChange::Unchanged;

            total_change |= context
                .current_namespace_mut()
                .transform(FlattenNestedSequences::new())
                .unwrap();

            total_change |= context
                .current_namespace_mut()
                .transform(FlattenNestedChoices::new())
                .unwrap();

            total_change |= context
                .current_namespace_mut()
                .transform(FlattenNestedAll::new())
                .unwrap();

            total_change |= context
                .current_namespace_mut()
                .transform(ExpandAttributeDeclarations::new())
                .unwrap();

            total_change |= context
                .current_namespace_mut()
                .transform(ExpandExtensionFragments::new())
                .unwrap();

            total_change |= context
                .current_namespace_mut()
                .transform(ExpandRestrictionFragments::new())
                .unwrap();

            total_change |= context
                .current_namespace_mut()
                .transform(RemoveProhibitedAttributes::new())
                .unwrap();

            if total_change == TransformChange::Unchanged {
                return Ok(TransformChange::from(i > 0));
            }
        }

        panic!("Maximum number of transformation loops reached")
    }
}

#[derive(Debug)]
pub struct Generator<'a> {
    pub context: &'a xsd_type_compiler::XmlnsContext,
    pub bound_namespaces: BTreeMap<XmlNamespace<'static>, syn::Path>,
    pub bound_types: BTreeMap<ExpandedName<'static>, BoundType>,
    pub bound_elements: BTreeMap<ExpandedName<'static>, TypeReference<'static>>,
    pub bound_attributes: BTreeMap<ExpandedName<'static>, TypeReference<'static>>,
    pub bound_groups: BTreeMap<ExpandedName<'static>, TypeReference<'static>>,
    pub augmenter: Box<dyn ItemAugmentation>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TypeType {
    Simple,
    Complex,
}

pub trait ToIdentTypesExt {
    fn to_item_ident(&self) -> Ident;
    fn to_field_ident(&self) -> Ident;
    fn to_variant_ident(&self) -> Ident;
    fn to_path_ident(&self) -> Ident;
}

impl ToIdentTypesExt for LocalName<'_> {
    fn to_item_ident(&self) -> Ident {
        format_ident!(
            "{}",
            misc::unkeywordify(self.to_string().to_pascal_case().as_str())
        )
    }
    fn to_field_ident(&self) -> Ident {
        format_ident!(
            "{}",
            misc::unkeywordify(self.to_string().to_snake_case().as_str())
        )
    }

    fn to_variant_ident(&self) -> Ident {
        format_ident!(
            "{}",
            misc::unkeywordify(self.to_string().to_pascal_case().as_str())
        )
    }

    fn to_path_ident(&self) -> Ident {
        format_ident!(
            "{}",
            misc::unkeywordify(self.to_string().to_snake_case().as_str())
        )
    }
}

impl ToIdentTypesExt for Ident {
    fn to_item_ident(&self) -> Ident {
        format_ident!(
            "{}",
            misc::unkeywordify(self.to_string().to_pascal_case().as_str())
        )
    }

    fn to_field_ident(&self) -> Ident {
        format_ident!(
            "{}",
            misc::unkeywordify(self.to_string().to_snake_case().as_str())
        )
    }

    fn to_variant_ident(&self) -> Ident {
        format_ident!(
            "{}",
            misc::unkeywordify(self.to_string().to_pascal_case().as_str())
        )
    }

    fn to_path_ident(&self) -> Ident {
        format_ident!(
            "{}",
            misc::unkeywordify(self.to_string().to_snake_case().as_str())
        )
    }
}

#[derive(Debug)]
struct GeneratorContext<'a> {
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

    pub fn current_namespace(&self) -> &CompiledNamespace {
        self.generator
            .context
            .namespaces
            .get(self.namespace)
            .unwrap()
    }
}

impl<'c> complex::Context for GeneratorContext<'c> {
    type SubContext = GeneratorContext<'c>;

    fn namespace(&self) -> &XmlNamespace<'_> {
        self.namespace
    }

    fn sub_context(&self, suggested_ident: Ident) -> Self::SubContext {
        Self::new(self.generator, self.namespace, suggested_ident)
    }

    fn suggested_ident(&self) -> &Ident {
        &self.suggested_ident
    }

    fn resolve_fragment<F: ToTypeTemplate, S: Scope>(
        &self,
        fragment: &F,
        scope: &mut S,
    ) -> Result<ToTypeTemplateData<F::TypeTemplate>> {
        fragment.to_type_template(self, scope)
    }

    fn resolve_fragment_id<F: ToTypeTemplate, S: Scope>(
        &self,
        fragment_id: &xsd_type_compiler::complex::FragmentIdx<F>,
        scope: &mut S,
    ) -> Result<ToTypeTemplateData<F::TypeTemplate>>
    where
        ComplexTypeFragmentCompiler: FragmentAccess<F>,
    {
        let fragment = self
            .current_namespace()
            .complex_type
            .get_fragment(fragment_id)
            .unwrap();

        fragment.to_type_template(self, scope)
    }

    fn resolve_named_type(&self, name: &ExpandedName<'_>) -> Result<BoundType> {
        if let Some(bound_type) = self.generator.bound_types.get(&name.as_ref()) {
            return Ok(bound_type.clone());
        }

        let type_mod_ident = format_ident!("types");

        let namespace_crate = self
            .generator
            .bound_namespaces
            .get(name.namespace().unwrap())
            .unwrap_or_else(|| {
                panic!(
                    "unbound namespace: {}",
                    name.namespace()
                        .map(|a| a.to_string())
                        .unwrap_or("Unknown namespace.".to_string())
                )
            });

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

        let namespace_crate = self
            .generator
            .bound_namespaces
            .get(name.namespace().unwrap())
            .unwrap_or_else(|| {
                panic!(
                    "unbound namespace: {}",
                    name.namespace()
                        .map(|a| a.to_string())
                        .unwrap_or("Unknown namespace.".to_string())
                )
            });

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

        let namespace_crate = self
            .generator
            .bound_namespaces
            .get(name.namespace().unwrap())
            .unwrap_or_else(|| {
                panic!(
                    "unbound namespace: {} for attribute {}",
                    name.namespace()
                        .map(|a| a.to_string())
                        .unwrap_or("Unknown namespace.".to_string()),
                    name.local_name()
                )
            });

        let name = name.local_name().to_item_ident();
        let ty: syn::Type = parse_quote!(#namespace_crate::#attribute_mod_ident::#name);

        let ty = TypeReference::new_static(ty).wrap(TypeReference::box_wrapper);

        Ok(ty)
    }

    fn to_expanded_name(&self, local_name: xmlity::LocalName<'static>) -> ExpandedName<'static> {
        ExpandedName::new(local_name, Some(self.namespace().clone().into_owned()))
    }

    fn resolve_named_group(&self, name: &ExpandedName<'_>) -> Result<TypeReference<'static>> {
        if let Some(ty) = self.generator.bound_groups.get(&name.as_ref()).cloned() {
            return Ok(ty);
        }

        let group_mod_ident = format_ident!("groups");

        let namespace_crate = self
            .generator
            .bound_namespaces
            .get(name.namespace().unwrap())
            .unwrap_or_else(|| {
                panic!(
                    "unbound namespace: {} for group {}",
                    name.namespace()
                        .map(|a| a.to_string())
                        .unwrap_or("Unknown namespace.".to_string()),
                    name.local_name()
                )
            });

        let name = name.local_name().to_item_ident();
        let ty: syn::Type = parse_quote!(#namespace_crate::#group_mod_ident::#name);

        let ty = TypeReference::new_static(ty).wrap(TypeReference::box_wrapper);

        Ok(ty)
    }
}

#[derive(Debug)]
struct GeneratorScope<'a> {
    items: Vec<Item>,
    augmentation: &'a dyn augments::ItemAugmentation,
}

impl complex::Scope for GeneratorScope<'_> {
    fn add_item<I: Into<Item>>(&mut self, item: I) -> Result<TypeReference<'static>> {
        let mut item: Item = item.into();

        self.items.extend(self.augmentation.augment_item(&mut item));

        let ident = match &item {
            Item::Struct(item) => &item.ident,
            Item::Enum(item) => &item.ident,
            Item::Mod(item) => &item.ident,
            _ => panic!("Unsupported item type"),
        };

        let ref_ = TypeReference::new_prefixed_type(parse_quote!(#ident));

        self.items.push(item);

        Ok(ref_)
    }

    fn add_raw_items<I: IntoIterator<Item = J>, J: Into<syn::Item>>(&mut self, items: I) {
        self.items
            .extend(items.into_iter().map(Into::into).map(|item| item));
    }

    fn augmenter(&self) -> &dyn augments::ItemAugmentation {
        self.augmentation
    }
}

fn finish_mod(mod_name: &Ident, items: Vec<Item>) -> Option<ItemMod> {
    if items.is_empty() {
        return None;
    }
    Some(parse_quote!(
        pub mod #mod_name {
            #(#items)*
        }
    ))
}

impl<'a> GeneratorScope<'a> {
    fn new(augmentation: &'a dyn augments::ItemAugmentation) -> Self {
        Self {
            items: Vec::new(),
            augmentation,
        }
    }

    fn finish(self) -> Vec<Item> {
        self.items
    }

    fn finish_mod(self, mod_name: &Ident) -> Option<ItemMod> {
        finish_mod(mod_name, self.items)
    }
}

#[derive(Debug, Clone)]
pub struct BoundType {
    pub ty: TypeReference<'static>,
    pub ty_type: TypeType,
    pub serialize_with: Option<syn::Path>,
    pub deserialize_with: Option<syn::Path>,
}

impl<'a> Generator<'a> {
    pub fn new_with_augmenter<A: augments::ItemAugmentation + 'static>(
        context: &'a xsd_type_compiler::XmlnsContext,
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
        }
    }
    pub fn new(context: &'a xsd_type_compiler::XmlnsContext) -> Self {
        Self::new_with_augmenter(context, augments::NoopAugmentation::new())
    }

    pub fn bind_namespace(&mut self, namespace: XmlNamespace<'static>, path: syn::Path) {
        self.bound_namespaces.insert(namespace, path);
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
        for (name, bound_type) in types {
            self.bind_type(name, bound_type);
        }
    }

    pub fn bind_element(&mut self, name: ExpandedName<'static>, ty: TypeReference<'static>) {
        self.bound_elements.insert(name, ty);
    }

    pub fn bind_attribute(&mut self, name: ExpandedName<'static>, ty: TypeReference<'static>) {
        self.bound_attributes.insert(name, ty);
    }

    pub fn bind_group(&mut self, name: ExpandedName<'static>, ty: TypeReference<'static>) {
        self.bound_groups.insert(name, ty);
    }

    pub fn generate_namespace(
        &mut self,
        namespace: &xmlity::XmlNamespace<'_>,
    ) -> Result<Vec<Item>> {
        let mut items = Vec::new();

        let compiled_namespace = self.context.namespaces.get(namespace).unwrap(); // TODO: handle this error properly with a better error message

        let types_module_name = format_ident!("types");

        let simple_types = compiled_namespace
            .top_level_types
            .iter()
            .filter(|(_key, type_)| matches!(type_, TopLevelType::Simple(_)))
            .map(|(key, _)| ExpandedName::new(key.as_ref(), Some(namespace.as_ref())))
            .filter_map(|expanded_name| {
                if self.bound_types.contains_key(&expanded_name) {
                    //TODO: Probably should warn.
                    return None;
                }

                let (mut bound_type, i) = match self.generate_top_level_type(&expanded_name) {
                    Ok(ok) => ok,
                    Err(err) => return Some(Err(err)),
                };

                let bound_namespace = self.bound_namespaces.get(namespace).unwrap();

                let path: syn::Path = parse_quote!(#bound_namespace::#types_module_name);

                bound_type.ty = TypeReference::new_static(bound_type.ty.into_type(Some(&path)));

                self.bind_type(expanded_name.into_owned(), bound_type);

                Some(Ok(i))
            })
            .collect::<Result<Vec<_>>>()?;

        let complex_types = compiled_namespace
            .top_level_types
            .iter()
            .filter(|(_key, type_)| matches!(type_, TopLevelType::Complex(_)))
            .map(|(key, _)| key)
            .map(|local_name| {
                let expanded_name =
                    ExpandedName::new(local_name.as_ref(), Some(namespace.as_ref()));
                let (mut bound_type, i) = self.generate_top_level_type(&expanded_name)?;

                let bound_namespace = self.bound_namespaces.get(namespace).unwrap();

                let path: syn::Path = parse_quote!(#bound_namespace::#types_module_name);

                bound_type.ty = TypeReference::new_static(bound_type.ty.into_type(Some(&path)));

                self.bind_type(expanded_name.into_owned(), bound_type);

                Ok(i)
            })
            .collect::<Result<Vec<_>>>()?;

        // First we resolve simple types
        let type_items = simple_types
            .into_iter()
            .chain(complex_types)
            .flatten()
            .collect::<Vec<_>>();

        let item_mod = parse_quote!(
            pub mod #types_module_name {
                #(#type_items)*
            }
        );

        items.push(Item::Mod(item_mod));

        let attributes_module_name = format_ident!("attributes");

        let attributes_items = compiled_namespace
            .top_level_attributes
            .keys()
            .map(|local_name| {
                let expanded_name =
                    ExpandedName::new(local_name.as_ref(), Some(namespace.as_ref()));
                let (mut bound_type, i) = self.generate_top_level_attribute(&expanded_name)?;

                let bound_namespace = self.bound_namespaces.get(namespace).unwrap();

                let path: syn::Path = parse_quote!(#bound_namespace::#attributes_module_name);

                bound_type = TypeReference::new_static(bound_type.into_type(Some(&path)));

                self.bind_attribute(expanded_name.into_owned(), bound_type);

                Ok(i)
            })
            .collect::<Result<Vec<_>>>()?
            .into_iter()
            .flatten()
            .collect::<Vec<_>>();

        let attributes_mod = parse_quote!(
            pub mod #attributes_module_name {
                #(#attributes_items)*
            }
        );

        items.push(Item::Mod(attributes_mod));

        let groups_module_name = format_ident!("groups");

        let group_items = compiled_namespace
            .top_level_groups
            .keys()
            .map(|local_name| {
                let expanded_name =
                    ExpandedName::new(local_name.as_ref(), Some(namespace.as_ref()));
                let (mut bound_type, i) = self.generate_top_level_group(&expanded_name)?;

                let bound_namespace = self.bound_namespaces.get(namespace).unwrap();

                let path: syn::Path = parse_quote!(#bound_namespace::#groups_module_name);

                bound_type = TypeReference::new_static(bound_type.into_type(Some(&path)));

                self.bind_group(expanded_name.into_owned(), bound_type);

                Ok(i)
            })
            .collect::<Result<Vec<_>>>()?
            .into_iter()
            .flatten()
            .collect::<Vec<_>>();

        let groups_mod = parse_quote!(
            pub mod #groups_module_name {
                #(#group_items)*
            }
        );

        items.push(Item::Mod(groups_mod));

        for local_name in compiled_namespace.top_level_elements.keys() {
            let expanded_name = ExpandedName::new(local_name.as_ref(), Some(namespace.as_ref()));
            let (mut bound_type, i) = self.generate_top_level_element(&expanded_name)?;

            let bound_namespace = self.bound_namespaces.get(namespace).unwrap();

            let path: syn::Path = parse_quote!(#bound_namespace);

            bound_type = TypeReference::new_static(bound_type.into_type(Some(&path)));

            self.bind_element(expanded_name.into_owned(), bound_type);
            items.extend(i)
        }

        //TODO
        Ok(items)
    }

    pub fn generate_top_level_type(
        &self,
        name: &xmlity::ExpandedName<'_>,
    ) -> Result<(BoundType, Vec<Item>)> {
        let compiled_namespace = self
            .context
            .namespaces
            .get(name.namespace().unwrap())
            .unwrap_or_else(|| panic!("namespace not found: {}", name.namespace().unwrap()));
        // TODO: handle this error properly with a better error message

        let type_ = compiled_namespace
            .top_level_types
            .get(name.local_name())
            .unwrap_or_else(|| {
                panic!(
                    "type not found: {} in namespace: {}",
                    name.local_name(),
                    name.namespace().unwrap()
                )
            }); // TODO: handle this error properly with a better error message

        match type_ {
            xsd_type_compiler::TopLevelType::Simple(_type_) => {
                let ty = TypeReference::new_static(parse_quote!(String));

                let bound_type = BoundType {
                    ty,
                    ty_type: TypeType::Simple,
                    serialize_with: None,
                    deserialize_with: None,
                };

                Ok((bound_type, Vec::new()))
            }
            xsd_type_compiler::TopLevelType::Complex(type_) => {
                let fragment = compiled_namespace
                    .complex_type
                    .get_fragment(&type_.root_fragment)
                    .unwrap();

                let item_name = name.local_name().to_item_ident();
                let module_name = format_ident!("{}_items", name.local_name().to_path_ident());
                let context =
                    GeneratorContext::new(self, name.namespace().unwrap(), item_name.clone());
                let mut scope = GeneratorScope::new(self.augmenter.deref());

                let type_ = fragment.to_type_template(&context, &mut scope)?;

                let mut items = Vec::new();

                let mut item = type_
                    .template
                    .to_struct(&item_name, Some(&parse_quote!(#module_name)));

                let augment_items = self.augmenter.augment_struct(&mut item);

                items.extend(scope.finish_mod(&module_name).map(|i| Item::Mod(i)));

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
        }
    }

    pub fn generate_top_level_attribute(
        &self,
        name: &xmlity::ExpandedName<'_>,
    ) -> Result<(TypeReference<'static>, Vec<Item>)> {
        let compiled_namespace = self
            .context
            .namespaces
            .get(name.namespace().unwrap())
            .unwrap_or_else(|| panic!("namespace not found: {}", name.namespace().unwrap()));
        // TODO: handle this error properly with a better error messa

        let attribute = compiled_namespace
            .top_level_attributes
            .get(name.local_name())
            .unwrap(); // TODO: handle this error properly with a better error messa

        let fragment = compiled_namespace
            .complex_type
            .get_fragment(&attribute.root_fragment)
            .unwrap();

        let item_name = name.local_name().to_item_ident();
        let module_name = format_ident!("{}_items", name.local_name().to_path_ident());
        let context = GeneratorContext::new(self, name.namespace().unwrap(), item_name.clone());
        let mut scope = GeneratorScope::new(self.augmenter.deref());

        let type_ = fragment.to_type_template(&context, &mut scope)?;

        let mut items = Vec::new();

        let mut item = type_
            .template
            .to_struct(&item_name, Some(&parse_quote!(#module_name)));

        let augment_items = self.augmenter.augment_struct(&mut item);

        items.extend(scope.finish_mod(&module_name).map(|i| Item::Mod(i)));

        items.push(Item::Struct(item));

        items.extend(augment_items);

        let type_ = TypeReference::new_prefixed_type(parse_quote!(#item_name));

        Ok((type_, items))
    }

    pub fn generate_top_level_element(
        &self,
        name: &xmlity::ExpandedName<'_>,
    ) -> Result<(TypeReference<'static>, Vec<Item>)> {
        let compiled_namespace = self
            .context
            .namespaces
            .get(name.namespace().unwrap())
            .unwrap_or_else(|| panic!("namespace not found: {}", name.namespace().unwrap()));
        // TODO: handle this error properly with a better error messa

        let element = compiled_namespace
            .top_level_elements
            .get(name.local_name())
            .unwrap(); // TODO: handle this error properly with a better error messa

        let fragment = compiled_namespace
            .complex_type
            .get_fragment(&element.root_fragment)
            .unwrap();

        let item_name = name.local_name().to_item_ident();
        let module_name = format_ident!("{}_items", name.local_name().to_path_ident());
        let context = GeneratorContext::new(self, name.namespace().unwrap(), item_name.clone());
        let mut scope = GeneratorScope::new(self.augmenter.deref());

        let type_ = fragment.to_type_template(&context, &mut scope)?;

        let mut items = Vec::new();

        let mut item = type_
            .template
            .to_struct(&item_name, Some(&parse_quote!(#module_name)));

        let augment_items = self.augmenter.augment_struct(&mut item);

        items.extend(scope.finish_mod(&module_name).map(|i| Item::Mod(i)));

        items.push(Item::Struct(item));

        items.extend(augment_items);

        let type_ = TypeReference::new_prefixed_type(parse_quote!(#item_name));

        Ok((type_, items))
    }

    pub fn generate_top_level_group(
        &self,
        name: &xmlity::ExpandedName<'_>,
    ) -> Result<(TypeReference<'static>, Vec<Item>)> {
        let compiled_namespace = self
            .context
            .namespaces
            .get(name.namespace().unwrap())
            .unwrap_or_else(|| panic!("namespace not found: {}", name.namespace().unwrap()));
        // TODO: handle this error properly with a better error messa

        let group = compiled_namespace
            .top_level_groups
            .get(name.local_name())
            .unwrap(); // TODO: handle this error properly with a better error messa

        let fragment = compiled_namespace
            .complex_type
            .get_fragment(&group.root_fragment)
            .unwrap();

        let item_name = name.local_name().to_item_ident();
        let module_name = format_ident!("{}_items", name.local_name().to_path_ident());
        let context = GeneratorContext::new(self, name.namespace().unwrap(), item_name.clone());
        let mut scope = GeneratorScope::new(self.augmenter.deref());

        let type_ = fragment.to_type_template(&context, &mut scope)?;

        let mut items = Vec::new();

        let mut item = type_
            .template
            .to_struct(&item_name, Some(&parse_quote!(#module_name)));

        let augment_items = self.augmenter.augment_struct(&mut item);

        items.extend(scope.finish_mod(&module_name).map(|i| Item::Mod(i)));

        items.push(Item::Struct(item));

        items.extend(augment_items);

        let type_ = TypeReference::new_prefixed_type(parse_quote!(#item_name));

        Ok((type_, items))
    }
}
