//! This example is used to generate the `xmlity-ns-xml` crate.
//!
//! The `xmlity-ns-xml` crate can not use `xmlity-build` as a dependency, because it is itself a dependency of `xmlity-build`. Therefore, this example is used to generate the `xmlity-ns-xml` crate.
use std::env::current_dir;

use syn::parse_quote;
use url::Url;
use xsd_fragments::FragmentedXsdDocumentKey;

fn main() {
    println!("Building the engine...");

    let time = std::time::Instant::now();

    let xml_path = FragmentedXsdDocumentKey::Original(
        Url::from_file_path(
            current_dir()
                .unwrap()
                .join("schemas/xmlity-ns-xs/src/xml.xsd"),
        )
        .unwrap(),
    );

    let engine = xmlity_build::BuildEngine::builder()
        .allowed_files(vec!["schemas/**/*.xsd".to_string()])
        .allow_network_access(true)
        .bound_namespaces(vec![(xml_path.clone(), parse_quote!(crate))])
        .build();

    let engine = engine.start().expect("Failed to start the engine");
    println!(
        "Starting the engine took {:?}",
        time.elapsed().as_secs_f32()
    );

    let time = std::time::Instant::now();

    engine
        .generate_namespace(
            xmlity_build::GenerateNamespaceConfig::builder()
                .output_file(
                    "schemas/xmlity-ns-xml/src/lib.rs"
                        .parse()
                        .expect("Failed to parse output file path"),
                )
                .namespace(xml_path)
                .bon_builders(true)
                .enum_from_impls(true)
                .struct_from_impls(true)
                .build(),
        )
        .expect("Failed to generate namespace");

    println!(
        "Generating the xs namespace took {:?}",
        time.elapsed().as_secs_f32()
    );
}
