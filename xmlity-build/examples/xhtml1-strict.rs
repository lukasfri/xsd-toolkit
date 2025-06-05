use syn::parse_quote;
use xmlity::XmlNamespace;

fn main() {
    let engine = xmlity_build::BuildEngine::builder()
        .glob_patterns(vec!["./schemas/**/*.xsd".to_string()])
        .url_net_resolution(true)
        .bound_namespaces(vec![
            (XmlNamespace::XML, parse_quote!(crate::xml)),
            (XmlNamespace::XHTML, parse_quote!(crate::xhtml)),
            (XmlNamespace::XS, parse_quote!(crate::xs)),
        ])
        .build();
    println!("{:?}", engine);

    engine
        .generate_namespace(
            xmlity_build::GenerateNamespace::builder()
                .output_file("xmlity-build/tests/output/xhtml.rs".parse().unwrap())
                .namespace(XmlNamespace::new_dangerous("http://www.w3.org/1999/xhtml"))
                .build(),
        )
        .unwrap();

    engine
        .generate_namespace(
            xmlity_build::GenerateNamespace::builder()
                .output_file("xmlity-build/tests/output/xs.rs".parse().unwrap())
                .namespace(XmlNamespace::XS)
                .build(),
        )
        .unwrap();

    println!("{}", std::env::current_dir().unwrap().display());
}
