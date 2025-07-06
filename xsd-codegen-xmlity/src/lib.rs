pub mod augments;
pub mod binds;
mod complex;
pub mod misc;
mod simple;
pub mod templates;

use std::{
    collections::{BTreeMap, HashSet},
    convert::Infallible,
    ops::Deref,
};

use complex::ComplexToTypeTemplate;
use inflector::Inflector;
use misc::TypeReference;
use quote::format_ident;
use syn::{parse_quote, Ident, Item, ItemMod};
use xmlity::{ExpandedName, LocalName, XmlNamespace};
use xsd_fragment_transformer::{
    complex::{
        ExpandAttributeDeclarations, ExpandExtensionFragments, ExpandRestrictionFragments,
        ExpandShortFormComplexTypes, FlattenNestedChoices, FlattenNestedSequences,
        RemoveProhibitedAttributes, SingleChoiceToSequence,
    },
    simple::ExpandSimpleRestriction,
    TransformChange, XmlnsContextExt, XmlnsContextTransformer,
};
use xsd_type_compiler::{
    fragments::{
        complex::ComplexTypeFragmentCompiler, simple::SimpleTypeFragmentCompiler, FragmentAccess,
    },
    CompiledNamespace, TopLevelType,
};

use crate::{
    augments::{ItemAugmentation, ItemAugmentationExt},
    simple::SimpleToTypeTemplate,
};

#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    MissingNamespace { namespace: XmlNamespace<'static> },
    NoNamespace,
    MissingElement { name: ExpandedName<'static> },
    MissingAttribute { name: ExpandedName<'static> },
    MissingGroup { name: ExpandedName<'static> },
    MissingType { name: ExpandedName<'static> },
    UnsupportedFragment { fragment: String },
    UnsupportedSimpleBase { base: Option<ExpandedName<'static>> },
}
pub type Result<T> = std::result::Result<T, Error>;

#[non_exhaustive]
#[derive(Debug)]
/// This transformer is used to transform the XSD into a form that is required for the codegen to work.
pub struct XmlityCodegenTransformer {
    allowed_simple_bases: HashSet<ExpandedName<'static>>,
}

impl XmlityCodegenTransformer {
    #[allow(clippy::new_without_default)]
    pub fn new(allowed_simple_bases: HashSet<ExpandedName<'static>>) -> Self {
        Self {
            allowed_simple_bases,
        }
    }
}

impl XmlnsContextTransformer for XmlityCodegenTransformer {
    type Error = Infallible;
    fn transform(
        self,
        context: xsd_fragment_transformer::XmlnsContextTransformerContext<'_>,
    ) -> std::result::Result<TransformChange, Self::Error> {
        for i in 0..100 {
            let mut total_change = TransformChange::Unchanged;

            total_change |= context
                .xmlns_context
                .context_transform(ExpandSimpleRestriction::new(&self.allowed_simple_bases))
                .unwrap();

            total_change |= context
                .xmlns_context
                .local_transform_all(&ExpandShortFormComplexTypes::new())
                .unwrap();

            total_change |= context
                .xmlns_context
                .local_transform_all(&SingleChoiceToSequence::new())
                .unwrap();

            total_change |= context
                .xmlns_context
                .local_transform_all(&FlattenNestedSequences::new())
                .unwrap();

            total_change |= context
                .xmlns_context
                .local_transform_all(&FlattenNestedChoices::new())
                .unwrap();

            total_change |= context
                .xmlns_context
                .context_transform(ExpandAttributeDeclarations::new())
                .unwrap();

            total_change |= context
                .xmlns_context
                .context_transform(ExpandExtensionFragments::new())
                .unwrap();

            total_change |= context
                .xmlns_context
                .context_transform(ExpandRestrictionFragments::new())
                .unwrap();

            total_change |= context
                .xmlns_context
                .local_transform_all(&RemoveProhibitedAttributes::new())
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

pub trait Scope {
    fn add_item<I: Into<syn::Item>>(&mut self, item: I) -> Result<TypeReference<'static>>;

    fn add_raw_items<I: IntoIterator<Item = J>, J: Into<syn::Item>>(&mut self, items: I);

    fn augmenter(&self) -> &dyn ItemAugmentation;
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
            .get_namespace(self.namespace)
            .unwrap()
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

    fn resolve_fragment<F: SimpleToTypeTemplate, S: Scope>(
        &self,
        fragment: &F,
        scope: &mut S,
    ) -> Result<ToTypeTemplateData<F::TypeTemplate>> {
        fragment.to_type_template(self, scope)
    }

    fn get_fragment<F, S: Scope>(
        &self,
        fragment_id: &xsd_type_compiler::fragments::FragmentIdx<F>,
        _: &mut S,
    ) -> Result<&F>
    where
        SimpleTypeFragmentCompiler: FragmentAccess<F>,
    {
        Ok(self
            .current_namespace()
            .complex_type
            .simple_type_fragments
            .get_fragment(fragment_id)
            .unwrap())
    }

    fn resolve_fragment_id<F: SimpleToTypeTemplate, S: Scope>(
        &self,
        fragment_id: &xsd_type_compiler::fragments::FragmentIdx<F>,
        scope: &mut S,
    ) -> Result<ToTypeTemplateData<F::TypeTemplate>>
    where
        SimpleTypeFragmentCompiler: FragmentAccess<F>,
    {
        let fragment = self.get_fragment(fragment_id, scope)?;

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

    fn resolve_fragment<F: ComplexToTypeTemplate, S: Scope>(
        &self,
        fragment: &F,
        scope: &mut S,
    ) -> Result<ToTypeTemplateData<F::TypeTemplate>> {
        fragment.to_type_template(self, scope)
    }

    fn resolve_fragment_id<F: ComplexToTypeTemplate, S: Scope>(
        &self,
        fragment_id: &xsd_type_compiler::fragments::FragmentIdx<F>,
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
pub struct ToTypeTemplateData<T> {
    pub ident: Option<Ident>,
    pub template: T,
}

impl Scope for GeneratorScope<'_> {
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
        self.items.extend(items.into_iter().map(Into::into));
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

    pub fn bind_group(&mut self, name: ExpandedName<'static>, ty: TypeReference<'static>) {
        self.bound_groups.insert(name, ty);
    }

    pub fn generate_namespace(
        &mut self,
        namespace: &xmlity::XmlNamespace<'_>,
    ) -> Result<Vec<Item>> {
        let mut items = Vec::new();

        let compiled_namespace =
            self.context
                .get_namespace(namespace)
                .ok_or_else(|| Error::MissingNamespace {
                    namespace: namespace.clone().into_owned(),
                })?;

        let types_module_name = format_ident!("types");

        self.generate_types_module(namespace, &types_module_name)?
            .map(Item::Mod)
            .map(|mod_item| {
                items.push(mod_item);
            });

        let attributes_module_name = format_ident!("attributes");

        self.generate_attributes_module(namespace, &attributes_module_name)?
            .map(Item::Mod)
            .map(|mod_item| {
                items.push(mod_item);
            });

        let groups_module_name = format_ident!("groups");

        self.generate_groups_module(namespace, &groups_module_name)?
            .map(Item::Mod)
            .map(|mod_item| {
                items.push(mod_item);
            });

        for expanded_name in compiled_namespace
            .top_level_elements
            .keys()
            .map(|local_name| ExpandedName::new(local_name.as_ref(), Some(namespace.as_ref())))
        {
            if self.bound_elements.contains_key(&expanded_name) {
                continue;
            }
            let (mut bound_type, i) = self.generate_element(&expanded_name)?;

            let bound_namespace = self.bound_namespaces.get(namespace).unwrap();

            let path: syn::Path = parse_quote!(#bound_namespace);

            bound_type = TypeReference::new_static(bound_type.into_type(Some(&path)));

            self.bind_element(expanded_name.into_owned(), bound_type);
            items.extend(i)
        }

        Ok(items)
    }

    pub fn generate_type(&self, name: &xmlity::ExpandedName<'_>) -> Result<(BoundType, Vec<Item>)> {
        let namespace = name.namespace().ok_or_else(|| Error::NoNamespace)?;

        let compiled_namespace =
            self.context
                .get_namespace(namespace)
                .ok_or_else(|| Error::MissingNamespace {
                    namespace: namespace.clone().into_owned(),
                })?;

        let type_ = compiled_namespace
            .top_level_types
            .get(name.local_name())
            .ok_or_else(|| Error::MissingType {
                name: name.clone().into_owned(),
            })?;

        match type_ {
            xsd_type_compiler::TopLevelType::Simple(type_) => {
                let fragment = compiled_namespace
                    .complex_type
                    .simple_type_fragments
                    .get_fragment(&type_.root_fragment)
                    .unwrap();

                let item_name = name.local_name().to_item_ident();
                let module_name = format_ident!("{}Items", item_name).to_path_ident();
                let context =
                    GeneratorContext::new(self, name.namespace().unwrap(), item_name.clone());
                let mut scope = GeneratorScope::new(self.augmenter.deref());

                let type_ = fragment.to_type_template(&context, &mut scope)?;

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
            xsd_type_compiler::TopLevelType::Complex(type_) => {
                let fragment = compiled_namespace
                    .complex_type
                    .get_fragment(&type_.root_fragment)
                    .unwrap();

                let item_name = name.local_name().to_item_ident();
                let module_name = format_ident!("{}Items", item_name).to_path_ident();
                let context =
                    GeneratorContext::new(self, name.namespace().unwrap(), item_name.clone());
                let mut scope = GeneratorScope::new(self.augmenter.deref());

                let type_ = fragment.to_type_template(&context, &mut scope)?;

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
        }
    }

    pub fn generate_types_module(
        &mut self,
        namespace: &XmlNamespace<'_>,
        module_name: &Ident,
    ) -> Result<Option<ItemMod>> {
        let compiled_namespace =
            self.context
                .get_namespace(namespace)
                .ok_or_else(|| Error::MissingNamespace {
                    namespace: namespace.clone().into_owned(),
                })?;

        let simple_types = compiled_namespace
            .top_level_types
            .iter()
            .filter(|(_key, type_)| matches!(type_, TopLevelType::Simple(_)))
            .map(|(key, _)| ExpandedName::new(key.as_ref(), Some(namespace.as_ref())))
            .filter_map(|expanded_name| {
                if self.bound_types.contains_key(&expanded_name) {
                    // We don't want to generate types that are already bound.
                    return None;
                }

                let (mut bound_type, i) = match self.generate_type(&expanded_name) {
                    Ok(ok) => ok,
                    Err(err) => return Some(Err(err)),
                };

                let bound_namespace =
                    self.bound_namespaces
                        .get(namespace)
                        .ok_or_else(|| Error::MissingNamespace {
                            namespace: namespace.clone().into_owned(),
                        });

                let bound_namespace = match bound_namespace {
                    Ok(bound_namespace) => bound_namespace,
                    Err(err) => return Some(Err(err)),
                };

                let path: syn::Path = parse_quote!(#bound_namespace::#module_name);

                bound_type.ty = TypeReference::new_static(bound_type.ty.into_type(Some(&path)));

                self.bind_type(expanded_name.into_owned(), bound_type);

                Some(Ok(i))
            })
            .collect::<Result<Vec<_>>>()?;

        let complex_types = compiled_namespace
            .top_level_types
            .iter()
            .filter(|(_key, type_)| matches!(type_, TopLevelType::Complex(_)))
            .map(|(key, _)| ExpandedName::new(key.as_ref(), Some(namespace.as_ref())))
            .map(|expanded_name| {
                let (mut bound_type, i) = self.generate_type(&expanded_name)?;

                let bound_namespace = self.bound_namespaces.get(namespace).unwrap();

                let path: syn::Path = parse_quote!(#bound_namespace::#module_name);

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
        name: &xmlity::ExpandedName<'_>,
    ) -> Result<(TypeReference<'static>, Vec<Item>)> {
        let namespace = name.namespace().ok_or_else(|| Error::NoNamespace)?;

        let compiled_namespace =
            self.context
                .get_namespace(namespace)
                .ok_or_else(|| Error::MissingNamespace {
                    namespace: namespace.clone().into_owned(),
                })?;

        let attribute = compiled_namespace
            .top_level_attributes
            .get(name.local_name())
            .ok_or_else(|| Error::MissingAttribute {
                name: name.clone().into_owned(),
            })?;

        let fragment = compiled_namespace
            .complex_type
            .get_fragment(&attribute.root_fragment)
            .unwrap();

        let item_name = name.local_name().to_item_ident();
        let module_name = format_ident!("{}Items", item_name).to_path_ident();
        let context = GeneratorContext::new(self, name.namespace().unwrap(), item_name.clone());
        let mut scope = GeneratorScope::new(self.augmenter.deref());

        let type_ = fragment.to_type_template(&context, &mut scope)?;

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
        namespace: &xmlity::XmlNamespace,
        module_name: &syn::Ident,
    ) -> Result<Option<ItemMod>> {
        let compiled_namespace =
            self.context
                .get_namespace(namespace)
                .ok_or_else(|| Error::MissingNamespace {
                    namespace: namespace.clone().into_owned(),
                })?;

        let attributes_items = compiled_namespace
            .top_level_attributes
            .keys()
            .map(|local_name| ExpandedName::new(local_name.as_ref(), Some(namespace.as_ref())))
            .map(|expanded_name| {
                let (mut bound_type, i) = self.generate_attribute(&expanded_name)?;

                let bound_namespace = self.bound_namespaces.get(namespace).unwrap();

                let path: syn::Path = parse_quote!(#bound_namespace::#module_name);

                bound_type = TypeReference::new_static(bound_type.into_type(Some(&path)));

                self.bind_attribute(expanded_name.into_owned(), bound_type);

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
        name: &xmlity::ExpandedName<'_>,
    ) -> Result<(TypeReference<'static>, Vec<Item>)> {
        let namespace = name.namespace().ok_or_else(|| Error::NoNamespace)?;

        let compiled_namespace =
            self.context
                .get_namespace(namespace)
                .ok_or_else(|| Error::MissingNamespace {
                    namespace: namespace.clone().into_owned(),
                })?;

        let element = compiled_namespace
            .top_level_elements
            .get(name.local_name())
            .ok_or_else(|| Error::MissingElement {
                name: name.clone().into_owned(),
            })?;

        let fragment = compiled_namespace
            .complex_type
            .get_fragment(&element.root_fragment)
            .unwrap();

        let item_name = name.local_name().to_item_ident();
        let module_name = format_ident!("{}Items", item_name).to_path_ident();
        let context = GeneratorContext::new(self, name.namespace().unwrap(), item_name.clone());
        let mut scope = GeneratorScope::new(self.augmenter.deref());

        let type_ = fragment.to_type_template(&context, &mut scope)?;

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

    pub fn generate_group(
        &self,
        name: &xmlity::ExpandedName<'_>,
    ) -> Result<(TypeReference<'static>, Vec<Item>)> {
        let namespace = name.namespace().ok_or_else(|| Error::NoNamespace)?;

        let compiled_namespace =
            self.context
                .get_namespace(namespace)
                .ok_or_else(|| Error::MissingNamespace {
                    namespace: namespace.clone().into_owned(),
                })?;

        let group = compiled_namespace
            .top_level_groups
            .get(name.local_name())
            .ok_or_else(|| Error::MissingGroup {
                name: name.clone().into_owned(),
            })?;

        let fragment = compiled_namespace
            .complex_type
            .get_fragment(&group.root_fragment)
            .unwrap();

        let item_name = name.local_name().to_item_ident();
        let module_name = format_ident!("{}Items", item_name).to_path_ident();
        let context = GeneratorContext::new(self, name.namespace().unwrap(), item_name.clone());
        let mut scope = GeneratorScope::new(self.augmenter.deref());

        let type_ = fragment.to_type_template(&context, &mut scope)?;

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
        namespace: &xmlity::XmlNamespace,
        groups_module_name: &syn::Ident,
    ) -> Result<Option<ItemMod>> {
        let compiled_namespace =
            self.context
                .get_namespace(namespace)
                .ok_or_else(|| Error::MissingNamespace {
                    namespace: namespace.clone().into_owned(),
                })?;

        let group_items = compiled_namespace
            .top_level_groups
            .keys()
            .map(|local_name| ExpandedName::new(local_name.as_ref(), Some(namespace.as_ref())))
            .map(|expanded_name| {
                let (mut bound_type, i) = self.generate_group(&expanded_name)?;

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
