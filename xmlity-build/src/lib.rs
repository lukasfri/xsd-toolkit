use std::path::{Path, PathBuf};

use bon::Builder;
use syn::parse_quote;
use xmlity::{types::utils::XmlRoot, ExpandedName, XmlNamespace};
use xsd_codegen_xmlity::{
    augments::{
        AdditionalDerives, BonAugmentation, EnumFromAugmentation, ItemAugmentation,
        StructFromAugmentation,
    },
    misc::TypeReference,
    BoundType, XmlityCodegenTransformer,
};
use xsd_type_compiler::{CompiledNamespace, XmlnsContext};

#[derive(Debug, Builder)]
pub struct BuildEngine {
    #[builder(default)]
    pub glob_patterns: Vec<String>,
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
    #[display("glob pattern error at index {}", _0.index)]
    GlobPath(GlobError),
    #[display("file error at path {}", _0.path.display())]
    File(FileError),
}

#[derive(Debug, derive_more::derive::Display, derive_more::derive::Error)]
#[display("glob pattern error at index {}", index)]
pub struct GlobError {
    index: usize,
    kind: GlobErrorKind,
}

#[derive(Debug, derive_more::derive::Display, derive_more::derive::Error)]
pub enum GlobErrorKind {
    Pattern(glob::PatternError),
    Glob(glob::GlobError),
}

#[derive(Debug, derive_more::derive::Display, derive_more::derive::Error)]
#[display("File error at {}", path.to_str().unwrap_or("unknown path"))]
pub struct FileError {
    path: PathBuf,
    kind: FileErrorKind,
}

#[derive(Debug)]
pub enum FileErrorKind {
    Io(std::io::Error),
    XmlityQuickXml(xmlity_quick_xml::de::Error),
}

pub struct StartedBuildEngine {
    engine: BuildEngine,
    context: XmlnsContext,
}

impl BuildEngine {
    pub fn file_paths_from_glob<P: AsRef<str>>(pattern: P) -> Result<Vec<PathBuf>, GlobErrorKind> {
        let paths = glob::glob(pattern.as_ref()).map_err(GlobErrorKind::Pattern)?;

        let paths = paths
            .into_iter()
            .map(|r| r.map_err(GlobErrorKind::Glob))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(paths)
    }

    pub fn read_xsd_file<P: AsRef<Path>>(
        path: P,
    ) -> Result<XmlRoot<xsd::xs::Schema>, FileErrorKind> {
        let xml = std::fs::read_to_string(path.as_ref()).map_err(FileErrorKind::Io)?;

        let xsd = xmlity_quick_xml::from_str::<XmlRoot<xsd::xs::Schema>>(&xml)
            .map_err(FileErrorKind::XmlityQuickXml)?;

        Ok(xsd)
    }

    pub fn add_glob_pattern<T: Into<String>>(&mut self, path: T) {
        self.glob_patterns.push(path.into());
    }

    pub fn start(self) -> Result<StartedBuildEngine, Error> {
        let paths = self
            .glob_patterns
            .iter()
            .map(Self::file_paths_from_glob)
            .enumerate()
            .map(|(index, r)| r.map_err(|kind| GlobError { index, kind }))
            .collect::<Result<Vec<_>, _>>()?;

        let xsds = paths
            .into_iter()
            .flatten()
            .map(|path| Self::read_xsd_file(&path).map_err(|kind| FileError { kind, path }))
            .collect::<Result<Vec<_>, _>>()?;

        let mut context = xsds
            .into_iter()
            .filter_map(|schema| {
                schema.elements.into_iter().find_map(|a| match a {
                    xmlity::types::utils::XmlRootTop::Value(a) => Some(a),
                    _ => None,
                })
            })
            .map(xsd::XmlSchema::new)
            .map(|a| CompiledNamespace::from_schema(&a).unwrap())
            .fold(XmlnsContext::new(), |mut context, namespace| {
                context.add_namespace(namespace);
                context
            });

        for namespace in context.namespaces.keys().cloned().collect::<Vec<_>>() {
            context
                .transform(&namespace, XmlityCodegenTransformer::new())
                .unwrap();
        }

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
