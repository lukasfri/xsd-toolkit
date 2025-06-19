use syn::parse_quote;
use xmlity::XmlNamespace;

fn main() {
    let time = std::time::Instant::now();
    println!("Building the engine... {}", time.elapsed().as_secs_f32());
    let engine = xmlity_build::BuildEngine::builder()
        .glob_patterns(vec!["./schemas/**/*.xsd".to_string()])
        .url_net_resolution(true)
        .bound_namespaces(vec![
            (XmlNamespace::XML, parse_quote!(crate::xml)),
            (XmlNamespace::XHTML, parse_quote!(crate::xhtml)),
            (XmlNamespace::XS, parse_quote!(crate::xs)),
        ])
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
                .output_file("xmlity-build/tests/output/xhtml.rs".parse().unwrap())
                .namespace(XmlNamespace::new_dangerous("http://www.w3.org/1999/xhtml"))
                .build(),
        )
        .unwrap();

    println!(
        "Generating the xhtml namespace took {:?}",
        time.elapsed().as_secs_f32()
    );

    let time = std::time::Instant::now();

    engine
        .generate_namespace(
            xmlity_build::GenerateNamespace::builder()
                .output_file("xmlity-build/tests/output/xs.rs".parse().unwrap())
                .namespace(XmlNamespace::XS)
                .bon_builders(true)
                .enum_from(true)
                .build(),
        )
        .unwrap();

    println!(
        "Generating the xs namespace took {:?}",
        time.elapsed().as_secs_f32()
    );
}
