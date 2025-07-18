//! This example is used to generate the `xmlity-ns-xml` crate.
//!
//! The `xmlity-ns-xml` crate can not use `xmlity-build` as a dependency, because it is itself a dependency of `xmlity-build`. Therefore, this example is used to generate the `xmlity-ns-xml` crate.
use syn::parse_quote;
use xmlity::XmlNamespace;

fn main() {
    println!("Building the engine...");

    let time = std::time::Instant::now();

    let engine = xmlity_build::BuildEngine::builder()
        .glob_patterns(vec!["schemas/**/*.xsd".to_string()])
        .url_net_resolution(true)
        .bound_namespaces(vec![(XmlNamespace::XML, parse_quote!(crate))])
        .build();

    let engine = engine.start().unwrap();
    println!(
        "Starting the engine took {:?}",
        time.elapsed().as_secs_f32()
    );

    let time = std::time::Instant::now();

    engine
        .generate_namespace(
            xmlity_build::GenerateNamespace::builder()
                .output_file("schemas/xmlity-ns-xml/src/lib.rs".parse().unwrap())
                .namespace(XmlNamespace::XML)
                .bon_builders(true)
                .enum_from(true)
                .struct_from(true)
                .build(),
        )
        .unwrap();

    println!(
        "Generating the xs namespace took {:?}",
        time.elapsed().as_secs_f32()
    );
}
