use std::path::{Path, PathBuf};

use bon::Builder;
use syn::parse_quote;
use xmlity::{types::utils::XmlRoot, ExpandedName, LocalName, XmlNamespace};
use xsd_codegen_xmlity::{misc::TypeReference, BoundType};
use xsd_type_compiler::{
    complex::transformers::{
        ExpandAttributeGroups, ExpandExtensionFragments, ExpandRestrictionFragments,
        RemoveProhibitedAttributes,
    },
    transformers::TransformChange,
    CompiledNamespace, XmlnsContext,
};

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
}

#[derive(Debug, Builder)]
pub struct GenerateNamespace {
    pub namespace: XmlNamespace<'static>,
    pub output_file: PathBuf,
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
    ) -> Result<XmlRoot<xsd::schema::Schema>, FileErrorKind> {
        let xml = std::fs::read_to_string(path.as_ref()).map_err(FileErrorKind::Io)?;

        let xsd = xmlity_quick_xml::from_str::<XmlRoot<xsd::schema::Schema>>(&xml)
            .map_err(FileErrorKind::XmlityQuickXml)?;

        Ok(xsd)
    }

    pub fn add_glob_pattern<T: Into<String>>(&mut self, path: T) {
        self.glob_patterns.push(path.into());
    }

    pub fn transform_namespace(context: &mut XmlnsContext, namespace: &XmlNamespace<'static>) {
        for _ in 0..100 {
            let mut total_change = TransformChange::Unchanged;

            total_change |= context
                .transform(&namespace, ExpandAttributeGroups::new())
                .unwrap();

            // total_change |= context.transform(&namespace, ExpandGroups::new()).unwrap();

            total_change |= context
                .transform(&namespace, ExpandExtensionFragments::new())
                .unwrap();

            total_change |= context
                .transform(&namespace, ExpandRestrictionFragments::new())
                .unwrap();

            total_change |= context
                .transform(&namespace, RemoveProhibitedAttributes::new())
                .unwrap();

            if total_change == TransformChange::Unchanged {
                return;
            }
        }

        panic!("Maximum number of transformation loops reached")
    }

    pub fn generate_namespace(&self, generate_namespace: GenerateNamespace) -> Result<(), Error> {
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
            .map(|schema| xsd::XmlSchema::new(schema))
            .map(|a| CompiledNamespace::from_schema(&a).unwrap())
            .fold(XmlnsContext::new(), |mut context, namespace| {
                context.add_namespace(namespace);
                context
            });

        for namespace in context.namespaces.keys().cloned().collect::<Vec<_>>() {
            Self::transform_namespace(&mut context, &namespace);
        }

        let mut generator = xsd_codegen_xmlity::Generator::new(&context);

        generator.bind_attribute(
            ExpandedName::new(LocalName::new_dangerous("lang"), Some(XmlNamespace::XML)),
            TypeReference::new_static(parse_quote!(crate::xml::Lang)),
        );

        generator.bind_attribute(
            ExpandedName::new(LocalName::new_dangerous("space"), Some(XmlNamespace::XML)),
            TypeReference::new_static(parse_quote!(crate::xml::Space)),
        );

        generator.bind_types(xsd_codegen_xmlity::binds::StdXsdTypes);
        self.bound_namespaces.iter().for_each(|(namespace, path)| {
            generator.bind_namespace(namespace.clone(), path.clone())
        });

        generator.bind_types(self.bound_types.iter().cloned());

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
