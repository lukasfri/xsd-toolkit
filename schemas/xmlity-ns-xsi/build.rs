use std::env;
use syn::parse_quote;
use xmlity::XmlNamespace;

fn main() {
    println!("cargo::rerun-if-changed=src/xsi.xsd");

    // let engine = xmlity_build::BuildEngine::builder()
    //     .allowed_files(vec!["../**/*.xsd".to_string()])
    //     .allow_network_access(true)
    //     .bound_namespaces(vec![
    //         (XmlNamespace::XML, parse_quote!(xmlity_ns_xml)),
    //         (XmlNamespace::XS, parse_quote!(xmlity_ns_xs)),
    //         (XmlNamespace::XSI, parse_quote!(xmlity_ns_xsi)),
    //     ])
    //     .build();

    // let engine = engine.start().unwrap();

    // let out_dir = env::var("OUT_DIR").unwrap();
    // let output_file_path: std::path::PathBuf = format!("{out_dir}/xsi.rs").parse().unwrap();

    // engine
    //     .generate_namespace(
    //         xmlity_build::GenerateNamespaceConfig::builder()
    //             .output_file(output_file_path)
    //             .namespace(XmlNamespace::XSI)
    //             .enum_from_impls(true)
    //             .struct_from_impls(true)
    //             .build(),
    //     )
    //     .unwrap();
}
