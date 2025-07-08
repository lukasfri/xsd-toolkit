use std::{collections::HashSet, convert::Infallible, path::PathBuf, str::FromStr};

use bon::Builder;
use syn::parse_quote;
use url::Url;
use xmlity::{ExpandedName, XmlNamespace};
use xsd::xsn;
use xsd_codegen_xmlity::{
    augments::{
        AdditionalDerives, BonAugmentation, EnumFromAugmentation, ItemAugmentation,
        StructFromAugmentation,
    },
    misc::TypeReference,
    BoundType, XmlityCodegenTransformer,
};
use xsd_fragments::XmlnsContext;
use xsd_fragments_transformer::XmlnsContextExt;
use xsd_namespace_map::{
    resolvers::{
        reqwest::BlockingReqwestXmlSchemaResolver, std_fs::StdFsSchemaResolver, XmlSchemaResolver,
    },
    GlobError, XmlNamespaceMap,
};

pub struct MultiReader {
    reqwest: BlockingReqwestXmlSchemaResolver,
    fs: StdFsSchemaResolver,
}

impl MultiReader {
    pub fn new() -> Self {
        Self {
            reqwest: BlockingReqwestXmlSchemaResolver::default(),
            fs: StdFsSchemaResolver::default(),
        }
    }
}

impl XmlSchemaResolver for MultiReader {
    type Error = Infallible;

    fn resolve_schema(&self, location: &Url) -> Result<xsd::XmlSchema, Self::Error> {
        match location.scheme() {
            "file" => self
                .fs
                .resolve_schema(location)
                .map_err(|e| panic!("Failed to resolve schema from file {}: {}", location, e,)),
            _ => self
                .reqwest
                .resolve_schema(location)
                .map_err(|e| panic!("Failed to resolve schema from URL {}: {}", location, e)),
        }
    }
}

#[derive(Debug, Builder)]
pub struct BuildEngine {
    #[builder(default)]
    pub glob_patterns: Vec<String>,
    #[builder(default)]
    pub urls: Vec<String>,
    #[builder(default = true)]
    pub url_net_resolution: bool,
    #[builder(default)]
    pub bound_namespaces: Vec<(XmlNamespace<'static>, syn::Path)>,
    #[builder(default)]
    pub bound_types: Vec<(ExpandedName<'static>, BoundType)>,
    #[builder(default)]
    pub bound_elements: Vec<(ExpandedName<'static>, TypeReference<'static>)>,
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
    GlobPath(GlobError),
}

pub struct StartedBuildEngine {
    engine: BuildEngine,
    context: XmlnsContext,
}

impl BuildEngine {
    pub fn start(self) -> Result<StartedBuildEngine, Error> {
        let mut map = XmlNamespaceMap::new();
        self.glob_patterns
            .iter()
            .try_for_each(|pattern| map.inform_glob_pattern(pattern))?;

        let urls = self.urls.iter().map(|url| Url::from_str(url).unwrap());

        map.inform_locations(urls);

        map.explore_locations(MultiReader::new());

        let mut context = XmlnsContext::new();
        context.import_namespace_map(&map).unwrap();

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

        context
            .context_transform(XmlityCodegenTransformer::new(allowed_simple_bases.clone()))
            .unwrap();

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
                Box::new(
                    generate_namespace
                        .bon_builders
                        .then(|| BonAugmentation::new()),
                ) as Box<dyn ItemAugmentation>,
                Box::new(
                    generate_namespace
                        .enum_from
                        .then(|| EnumFromAugmentation::new()),
                ) as Box<dyn ItemAugmentation>,
                Box::new(
                    generate_namespace
                        .struct_from
                        .then(|| StructFromAugmentation::new()),
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

        let items = generator
            .generate_namespace(&generate_namespace.namespace)
            .unwrap();

        let file = syn::File {
            attrs: Vec::new(),
            shebang: None,
            items,
        };

        let output = prettyplease::unparse(&file);

        std::fs::write(&generate_namespace.output_file, output).unwrap();

        Ok(())
    }
}
