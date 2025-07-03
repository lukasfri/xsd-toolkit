use std::env;
use syn::parse_quote;
use xmlity::XmlNamespace;

fn generate_xhtml1_strict() {
    println!("cargo::rerun-if-changed=../schemas/xhtml1-strict.xsd");

    let engine = xmlity_build::BuildEngine::builder()
        .glob_patterns(vec!["../**/*.xsd".to_string()])
        .url_net_resolution(true)
        .bound_namespaces(vec![
            (XmlNamespace::XML, parse_quote!(xmlity_ns_xml)),
            (XmlNamespace::XS, parse_quote!(xmlity_ns_xs)),
            (XmlNamespace::XHTML, parse_quote!(crate::xhtml1_strict)),
        ])
        .build();

    let engine = engine.start().unwrap();

    let out_dir = env::var("OUT_DIR").unwrap();
    let output_file_path: std::path::PathBuf =
        format!("{out_dir}/xhtml1_strict.rs").parse().unwrap();

    engine
        .generate_namespace(
            xmlity_build::GenerateNamespace::builder()
                .output_file(output_file_path)
                .namespace(XmlNamespace::XHTML)
                .enum_from(true)
                .struct_from(true)
                .build(),
        )
        .unwrap();
}

fn main() {
    #[cfg(feature = "xhtml1_strict")]
    generate_xhtml1_strict();
}
