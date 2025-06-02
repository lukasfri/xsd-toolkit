use std::{ffi::OsStr, path::PathBuf};

use bon::Builder;
use xmlity::types::utils::XmlRoot;

#[derive(Debug, Builder)]
pub struct Generator {
    pub output_dir: PathBuf,
    #[builder(default)]
    pub glob_patterns: Vec<String>,
    #[builder(default = true)]
    pub url_net_resolution: bool,
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
    error: GlobErrorKind,
}

#[derive(Debug)]
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
    Glob(glob::GlobError),
    Io(std::io::Error),
    XmlityQuickXml(xmlity_quick_xml::de::Error),
}

impl Generator {
    pub fn add_glob_pattern<T: Into<String>>(&mut self, path: T) {
        self.glob_patterns.push(path.into());
    }

    pub fn build(&self) -> Result<(), Error> {
        use glob::glob;

        let paths = self
            .glob_patterns
            .iter()
            .map(|pattern| glob(pattern))
            .enumerate()
            .map(|(i, a)| {
                a.map_err(|e| GlobError {
                    index: i,
                    error: GlobErrorKind::Pattern(e),
                })
            })
            .collect::<Result<Vec<_>, _>>()
            .expect("Failed to read glob pattern");

        let paths = paths
            .into_iter()
            .enumerate()
            .flat_map(|(i, a)| {
                a.map(move |a| {
                    a.map_err(|e| GlobError {
                        index: i,
                        error: GlobErrorKind::Glob(e),
                    })
                })
            })
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .filter(|a| a.extension() == Some(OsStr::new("xsd")))
            .collect::<Vec<_>>();

        println!("Paths: {:?}", paths);

        let files = paths
            .into_iter()
            .map(|path| {
                std::fs::read_to_string(&path)
                    .map(|a| (path.clone(), a))
                    .map_err(|e| FileError {
                        path,
                        kind: FileErrorKind::Io(e),
                    })
            })
            .collect::<Result<Vec<_>, _>>()?;

        let xsd = files
            .into_iter()
            .map(|(path, xml)| {
                xmlity_quick_xml::from_str::<XmlRoot<xsd::schema::Schema>>(&xml).map_err(|e| {
                    FileError {
                        path,
                        kind: FileErrorKind::XmlityQuickXml(e),
                    }
                })
            })
            .collect::<Result<Vec<_>, _>>()?;

        println!("Xsds: {:?}", xsd);

        Ok(())
    }
}
