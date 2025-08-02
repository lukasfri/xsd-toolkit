use std::{collections::HashSet, path::PathBuf};

use bon::Builder;
use syn::parse_quote;
pub mod reexports {
    pub use url;
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
use xsd_fragments::XmlnsContext;

#[derive(Debug, Builder)]
pub struct BuildEngine {
    #[builder(default)]
    pub glob_patterns: Vec<String>,
    #[builder(default)]
    pub urls: Vec<url::Url>,
    #[builder(default = true)]
    pub url_net_resolution: bool,
    #[builder(default)]
    pub bound_namespaces: Vec<(XmlNamespace<'static>, syn::Path)>,
    #[builder(default)]
    pub bound_types: Vec<(ExpandedName<'static>, BoundType)>,
    #[builder(default)]
    pub bound_elements: Vec<(ExpandedName<'static>, TypeReference<'static>)>,
    #[builder(default)]
    pub bound_attributes: Vec<(ExpandedName<'static>, TypeReference<'static>)>,
}

#[derive(Debug, Builder)]
pub struct GenerateNamespace {
    pub namespace: XmlNamespace<'static>,
    pub output_file: PathBuf,
    #[builder(default = false)]
    pub bon_builders: bool,
    #[builder(default = false)]
    pub enum_from: bool,
    #[builder(default = false)]
    pub struct_from: bool,
}

#[derive(Debug, derive_more::derive::From, derive_more::derive::Display)]
pub enum Error {
    #[display("glob pattern error {}", _0)]
    GlobPath(xsd::set::GlobError),
    #[display("Error when writing output file {}: {}", _1.display(), _0)]
    FileWriteError(std::io::Error, PathBuf),
    #[display("Error when importing namespace map: {}", _0)]
    XsdFragmentImportError(#[from] xsd_fragments::Error),
    #[display("Error when transforming XSD: {}", _0)]
    TransformationError(#[from] xsd_codegen_xmlity::CodegenTransformerError),
    #[display("Error when generating namespace: {}", _0)]
    GenerationError(#[from] xsd_codegen_xmlity::Error),
    #[display("Error when resolving URL: {}", _0)]
    ResolverError(#[from] ResolverError),
}

struct Resolver {
    client: reqwest::blocking::Client,
    cache_dir: PathBuf,
    net_resolution: bool,
}

#[derive(Debug, derive_more::derive::From, derive_more::derive::Display)]
pub enum ResolverError {
    #[display("Error when resolving URL using `reqwest`: {}", _0)]
    Reqwest(reqwest::Error),
    #[display("Error when reading from file: {}", _0)]
    Io(std::io::Error),
    #[display("Error when parsing XML: {}", _0)]
    XmlParse(xmlity_quick_xml::de::Error),
    XsdMissingRoot,
    #[display("Unsupported URL scheme: {}", _0)]
    UnsupportedUrlScheme(Url),
}

impl Resolver {
    fn new() -> Self {
        Self {
            client: reqwest::blocking::Client::new(),
            cache_dir: std::env::temp_dir().join("xsd-toolkit-built-cache"),
            net_resolution: true,
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
            "http" | "https" if self.net_resolution => {
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

pub struct StartedBuildEngine {
    engine: BuildEngine,
    context: XmlnsContext,
}

impl BuildEngine {
    pub fn start(self) -> Result<StartedBuildEngine, Error> {
        let mut map = XmlSchemaSet::new();
        self.glob_patterns
            .iter()
            .try_for_each(|pattern| map.inform_glob_pattern(pattern))?;

        map.inform_locations(self.urls.iter().cloned());

        let root_uris = map.locations.keys().cloned().collect::<Vec<_>>();

        let resolver = Resolver::new();

        map.explore_locations(&|url| resolver.resolve(url))
            .try_for_each(|a| a.map(|_| ()))?;

        let mut context = XmlnsContext::new();

        let mut imported_uris = Vec::new();
        root_uris.iter().try_for_each(|uri| {
            context.import_namespace_map(&map, uri, None, &mut imported_uris)
        })?;

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

impl StartedBuildEngine {
    pub fn generate_namespace<N: Into<GenerateNamespace>>(
        &self,
        generate_namespace: N,
    ) -> Result<(), Error> {
        let generate_namespace = generate_namespace.into();

        let mut generator = xsd_codegen_xmlity::Generator::new_with_augmenter(
            &self.context,
            vec![
                Box::new(generate_namespace.bon_builders.then(BonAugmentation::new))
                    as Box<dyn ItemAugmentation>,
                Box::new(generate_namespace.enum_from.then(EnumFromAugmentation::new))
                    as Box<dyn ItemAugmentation>,
                Box::new(
                    generate_namespace
                        .struct_from
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

        generator.bind_types(self.engine.bound_types.iter().cloned());

        generator.bind_elements(self.engine.bound_elements.iter().cloned());

        generator.bind_attributes(self.engine.bound_attributes.iter().cloned());

        let items = generator.generate_namespace(&generate_namespace.namespace)?;

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
