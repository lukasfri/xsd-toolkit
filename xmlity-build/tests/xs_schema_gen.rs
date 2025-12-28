//! This example is used to generate the `xmlity-ns-xs` crate.
//!
//! The `xmlity-ns-xs` crate can not use `xmlity-build` as a dependency, because it is itself a dependency of `xmlity-build`. Therefore, this example is used to generate the `xmlity-ns-xs` crate.
use std::env::current_dir;

use syn::parse_quote;
use url::Url;
use xmlity::{ExpandedName, LocalName, XmlNamespace};
use xsd_codegen_xmlity::{misc::TypeReference, BoundType, TypeType};
use xsd_fragments::FragmentedXsdDocumentKey;

#[test]
fn xml_schema_generation() {
    println!("Building the engine...");

    let time = std::time::Instant::now();

    let schema_dir = current_dir().unwrap().join("../schemas");

    println!(
        "Path: {}",
        schema_dir.join("xmlity-ns-xs/src/xml.xsd").display()
    );

    let xml_path = FragmentedXsdDocumentKey(
        Url::from_file_path(schema_dir.join("xmlity-ns-xs/src/xml.xsd")).unwrap(),
    );

    let xs_path = FragmentedXsdDocumentKey(
        Url::from_file_path(schema_dir.join("xmlity-ns-xs/src/XMLSchema.xsd")).unwrap(),
    );

    let engine = xmlity_build::BuildEngine::builder()
        .allowed_files(vec!["../schemas/**/*.xsd".to_string()])
        .allow_network_access(true)
        .bound_namespaces(vec![
            (xml_path, parse_quote!(xmlity_ns_xml)),
            (xs_path.clone(), parse_quote!(crate)),
        ])
        .bound_types(vec![
            (
                ExpandedName::new(LocalName::new("QName").unwrap(), Some(XmlNamespace::XS)),
                BoundType {
                    ty: TypeReference::new_static(parse_quote!(crate::types::QName)),
                    ty_type: TypeType::Simple,
                    serialize_with: None,
                    deserialize_with: None,
                    default_with: None,
                },
            ),
            (
                ExpandedName::new(LocalName::new("NCName").unwrap(), Some(XmlNamespace::XS)),
                BoundType {
                    ty: TypeReference::new_static(parse_quote!(::xmlity::LocalNameBuf)),
                    ty_type: TypeType::Simple,
                    serialize_with: None,
                    deserialize_with: None,
                    default_with: None,
                },
            ),
        ])
        .build();

    let engine = engine.start().unwrap();
    println!(
        "Starting the engine took {:?}",
        time.elapsed().as_secs_f32()
    );

    let time = std::time::Instant::now();

    let schema_code = engine
        .generate_namespace_output(
            xmlity_build::GenerateNamespaceConfig::builder()
                .namespace(xs_path)
                .bon_builders(true)
                .enum_from_impls(true)
                .struct_from_impls(true)
                .build(),
        )
        .unwrap();

    println!(
        "Generating the xs namespace took {:?}",
        time.elapsed().as_secs_f32()
    );

    insta::assert_snapshot!("xs_schema_rs_code", schema_code);
}
