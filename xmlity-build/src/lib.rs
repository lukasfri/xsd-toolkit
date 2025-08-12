//! This crate provides an easy API for generating XMLity-based Rust code from XSD schemas.
//!
//! It is designed to be able to be used in a build script, and provides a easy-to-use step-by-step API.
use std::str::FromStr;
use std::{collections::HashSet, path::PathBuf};

use bon::Builder;
use syn::parse_quote;

/// The `reexports` module provides re-exports of dependencies required during configuration.
pub mod reexports {
    pub use url;

    pub use xsd_fragments::FragmentedXsdDocumentKey;
}
use url::Url;

use xmlity::types::utils::XmlRoot;
use xmlity::{ExpandedName, XmlNamespace};
use xsd::set::XmlSchemaSet;
use xsd::{xs, xsn};
use xsd_codegen_xmlity::CodegenTransformer;
use xsd_codegen_xmlity::{
    augments::{
        AdditionalDerives, BonAugmentation, EnumFromAugmentation, ItemAugmentation,
        StructFromAugmentation,
    },
    misc::TypeReference,
    BoundType,
};
use xsd_fragments::{FragmentedXsdDocumentKey, XmlnsContext};

/// The [`BuildEngine`] struct is used to configure the build process, including allowed files, URLs, and bound namespaces.
#[derive(Debug, Builder)]
pub struct BuildEngine {
    /// A list of allowed files, which can be glob patterns.
    #[builder(default)]
    pub allowed_files: Vec<String>,
    /// A list of URLs to be used in the build process.
    #[builder(default)]
    pub urls: Vec<url::Url>,
    /// Whether to allow network access for resolving URLs. If urls are provided, this defaults to true.
    #[builder(default = true)]
    pub allow_network_access: bool,
    /// A list of namespaces to bind to specific paths.
    #[builder(default)]
    pub bound_namespaces: Vec<(FragmentedXsdDocumentKey, syn::Path)>,
    /// A map of globally bound namespaces to their keys.
    #[builder(default)]
    pub globally_bound_namespaces: Vec<(XmlNamespace<'static>, FragmentedXsdDocumentKey)>,
    /// A list of types to bind to specific types.
    #[builder(default)]
    pub bound_types: Vec<(ExpandedName<'static>, BoundType)>,
    /// A list of elements to bind to specific types.
    #[builder(default)]
    pub bound_elements: Vec<(ExpandedName<'static>, TypeReference<'static>)>,
    /// A list of attributes to bind to specific types.
    #[builder(default)]
    pub bound_attributes: Vec<(ExpandedName<'static>, TypeReference<'static>)>,
    /// A list of groups to bind to specific types.
    #[builder(default)]
    pub bound_groups: Vec<(ExpandedName<'static>, TypeReference<'static>)>,
}

/// Represents a configuration for generating a namespace.
#[derive(Debug, Builder)]
pub struct GenerateNamespaceConfig {
    /// The XML namespace to generate code for.
    pub namespace: FragmentedXsdDocumentKey,
    /// The output file where the generated code will be written.
    pub output_file: PathBuf,
    /// Derive `bon::builders::Builder` for structs.
    #[builder(default = false)]
    pub bon_builders: bool,
    /// Generate `From`-implementations for enums.
    #[builder(default = false)]
    pub enum_from_impls: bool,
    /// Generate `From`-implementations for structs.
    #[builder(default = false)]
    pub struct_from_impls: bool,
}

/// Error type for [`BuildEngine`] operations.
#[derive(Debug, derive_more::derive::From, derive_more::derive::Display)]
pub enum Error {
    /// Error when processing glob patterns.
    #[display("glob pattern error {}", _0)]
    GlobPath(xsd::set::GlobError),
    /// Error when writing output file.
    #[display("Error when writing output file {}: {}", _1.display(), _0)]
    FileWriteError(std::io::Error, PathBuf),
    /// Error when importing a namespace map.
    #[display("Error when importing namespace map: {}", _0)]
    XsdFragmentImportError(#[from] xsd_fragments::Error),
    /// Error when transforming XSD.
    #[display("Error when transforming XSD: {}", _0)]
    TransformationError(#[from] xsd_codegen_xmlity::CodegenTransformerError),
    /// Error when generating namespace.
    #[display("Error when generating namespace: {}", _0)]
    GenerationError(#[from] xsd_codegen_xmlity::Error),
    /// Error when loading XSD set.
    #[display("Error when loading XSD set: {}", _0)]
    SetError(#[from] xsd::set::Error<ResolverError>),
}

struct Resolver {
    client: reqwest::blocking::Client,
    cache_dir: PathBuf,
    allow_network_access: bool,
}

/// Error type used for resolving XSD schemas.
#[derive(Debug, derive_more::derive::From, derive_more::derive::Display)]
pub enum ResolverError {
    /// Error when resolving a URL using `reqwest`.
    #[display("Error when resolving URL using `reqwest`: {}", _0)]
    Reqwest(reqwest::Error),
    /// Error when reading from a file.
    #[display("Error when reading from file: {}", _0)]
    Io(std::io::Error),
    /// Error when parsing XML.
    #[display("Error when parsing XML: {}", _0)]
    XmlParse(xmlity_quick_xml::de::Error),
    /// A resolved XSD schema is missing the root element.
    XsdMissingRoot,
    /// Resolver tried to resolve a URL with an unsupported scheme.
    #[display("Unsupported URL scheme: {}", _0)]
    UnsupportedUrlScheme(Url),
    /// Unauthorized network access attempt.
    #[display("Unauthorized network access attempt")]
    UnauthorizedNetworkAccess,
}

impl Resolver {
    fn new(allow_network_access: bool) -> Self {
        Self {
            client: reqwest::blocking::Client::new(),
            cache_dir: std::env::temp_dir().join("xsd-toolkit-built-cache"),
            allow_network_access,
        }
    }

    fn url_to_file_name(url: &Url) -> String {
        url.as_str().replace('/', "__")
    }

    fn resolve(&self, url: &Url) -> Result<xs::Schema, ResolverError> {
        let potential_cache_file_path = self.cache_dir.join(Self::url_to_file_name(url));

        let schema_text = match url.scheme() {
            "http" | "https" if std::fs::exists(&potential_cache_file_path)? => {
                std::fs::read_to_string(&potential_cache_file_path)?
            }
            "http" | "https" => {
                if !self.allow_network_access {
                    return Err(ResolverError::UnauthorizedNetworkAccess);
                }

                let response = self.client.get(url.as_str()).send()?;
                let schema_text = response.text()?;

                std::fs::create_dir_all(&self.cache_dir)?;
                std::fs::write(&potential_cache_file_path, &schema_text)?;

                schema_text
            }
            "file" => std::fs::read_to_string(url.path())?,
            _ => {
                return Err(ResolverError::UnsupportedUrlScheme(url.clone()));
            }
        };

        let document = xmlity_quick_xml::from_str::<XmlRoot<xs::Schema>>(schema_text.as_str())?;

        let schema = document
            .elements
            .into_iter()
            .find_map(|e| match e {
                xmlity::types::utils::XmlRootTop::Value(e) => Some(e),
                _ => None,
            })
            .ok_or(ResolverError::XsdMissingRoot)?;

        Ok(schema)
    }
}

impl BuildEngine {
    /// Starts the build process, initializing the XML schema set and context.
    pub fn start(mut self) -> Result<StartedBuildEngine, Error> {
        if !self.urls.is_empty() {
            self.allow_network_access = true;
        }

        let mut map = XmlSchemaSet::new();
        self.allowed_files
            .iter()
            .try_for_each(|pattern| map.inform_glob_pattern(pattern))?;

        map.inform_locations(self.urls.iter().cloned());

        let root_uris = map.locations.keys().cloned().collect::<Vec<_>>();

        let resolver = Resolver::new(self.allow_network_access);

        map.explore_locations(&|url| resolver.resolve(url))
            .try_for_each(|a| a.map(|_| ()))?;

        let mut context = XmlnsContext::new();

        context
            .global_namespaces
            .extend(
                self.globally_bound_namespaces
                    .clone()
                    .into_iter()
                    .map(|(ns, key)| {
                        (
                            ns,
                            *context
                                .namespace_idxs
                                .get(&key)
                                .expect("Namespace should exist"),
                        )
                    }),
            );

        root_uris
            .iter()
            .try_for_each(|uri| context.import_namespace_map(&map, uri).map(|_| ()))?;

        let allowed_simple_bases: HashSet<ExpandedName<'static>> = [
            &xsn::DECIMAL,
            &xsn::FLOAT,
            &xsn::DOUBLE,
            &xsn::INTEGER,
            &xsn::NON_POSITIVE_INTEGER,
            &xsn::NEGATIVE_INTEGER,
            &xsn::LONG,
            &xsn::INT,
            &xsn::SHORT,
            &xsn::BYTE,
            &xsn::NON_NEGATIVE_INTEGER,
            &xsn::UNSIGNED_LONG,
            &xsn::UNSIGNED_INT,
            &xsn::UNSIGNED_SHORT,
            &xsn::UNSIGNED_BYTE,
            &xsn::POSITIVE_INTEGER,
            &xsn::STRING,
            &xsn::NORMALIZED_STRING,
            &xsn::TOKEN,
            &xsn::LANGUAGE,
            &xsn::NAME,
            &xsn::NCNAME,
            &xsn::ID,
            &xsn::IDREF,
            &xsn::IDREFS,
            &xsn::ENTITY,
            &xsn::ENTITIES,
            &xsn::NMTOKEN,
            &xsn::NMTOKENS,
            &xsn::DATE_TIME,
            &xsn::DATE,
            &xsn::DATE_TIME_STAMP,
            &xsn::DAY_TIME_DURATION,
            &xsn::ANY_URI,
        ]
        .iter()
        .map(|a| (***a).clone())
        .collect();

        context.context_transform(CodegenTransformer::new(allowed_simple_bases.clone()))?;

        Ok(StartedBuildEngine {
            engine: self,
            context,
        })
    }
}

/// Represents a started build engine with an initialized XML namespace context.
pub struct StartedBuildEngine {
    engine: BuildEngine,
    context: XmlnsContext,
}

impl StartedBuildEngine {
    /// Generates a namespace based on the provided configuration.
    pub fn generate_namespace<N: Into<GenerateNamespaceConfig>>(
        &self,
        generate_namespace: N,
    ) -> Result<(), Error> {
        let generate_namespace = generate_namespace.into();

        let mut generator = xsd_codegen_xmlity::Generator::new_with_augmenter(
            &self.context,
            vec![
                Box::new(generate_namespace.bon_builders.then(BonAugmentation::new))
                    as Box<dyn ItemAugmentation>,
                Box::new(
                    generate_namespace
                        .enum_from_impls
                        .then(EnumFromAugmentation::new),
                ) as Box<dyn ItemAugmentation>,
                Box::new(
                    generate_namespace
                        .struct_from_impls
                        .then(StructFromAugmentation::new),
                ) as Box<dyn ItemAugmentation>,
                Box::new(Some(AdditionalDerives {
                    structs: vec![
                        parse_quote!(::core::cmp::PartialEq),
                        parse_quote!(::core::clone::Clone),
                    ],
                    enums: vec![
                        parse_quote!(::core::cmp::PartialEq),
                        parse_quote!(::core::clone::Clone),
                    ],
                })) as Box<dyn ItemAugmentation>,
            ],
        );

        generator.bind_types(xsd_codegen_xmlity::binds::StdXsdTypes);
        self.engine
            .bound_namespaces
            .iter()
            .for_each(|(namespace, path)| {
                generator.bind_namespace(namespace.clone(), path.clone())
            });

        generator.bind_types(self.engine.bound_types.clone());

        generator.bind_elements(self.engine.bound_elements.clone());

        generator.bind_attributes(self.engine.bound_attributes.clone());

        generator.bind_groups(self.engine.bound_groups.clone());

        let namespace_idx = self
            .context
            .namespace_idxs
            .get(&generate_namespace.namespace)
            .expect("Namespace should exist");

        let items = generator.generate_namespace(namespace_idx)?;

        let file = syn::File {
            attrs: Vec::new(),
            shebang: None,
            items,
        };

        let output = prettyplease::unparse(&file);

        std::fs::write(&generate_namespace.output_file, output)
            .map_err(|e| Error::FileWriteError(e, generate_namespace.output_file.clone()))?;

        Ok(())
    }
}
