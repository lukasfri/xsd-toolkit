use syn::parse_quote;
use xmlity::{ExpandedName, LocalName, XmlNamespace};
use xsd_codegen_xmlity::{misc::TypeReference, BoundType, TypeType};

fn main() {
    println!("Building the engine...");

    let time = std::time::Instant::now();

    let engine = xmlity_build::BuildEngine::builder()
        .glob_patterns(vec!["./schemas/**/*.xsd".to_string()])
        .url_net_resolution(true)
        .bound_namespaces(vec![(XmlNamespace::XHTML, parse_quote!(crate::xhtml))])
        .bound_types(vec![
            (
                ExpandedName::new(LocalName::new_dangerous("QName"), Some(XmlNamespace::XS)),
                BoundType {
                    ty: TypeReference::new_static(parse_quote!(crate::xs::types::QName)),
                    ty_type: TypeType::Simple,
                    serialize_with: None,
                    deserialize_with: None,
                },
            ),
            (
                ExpandedName::new(LocalName::new_dangerous("anyURI"), Some(XmlNamespace::XS)),
                BoundType {
                    ty: TypeReference::new_static(parse_quote!(crate::xs::types::TargetNamespace)),
                    ty_type: TypeType::Simple,
                    serialize_with: None,
                    deserialize_with: None,
                },
            ),
            (
                ExpandedName::new(LocalName::new_dangerous("NCName"), Some(XmlNamespace::XS)),
                BoundType {
                    ty: TypeReference::new_static(parse_quote!(::xmlity::LocalName<'static>)),
                    ty_type: TypeType::Simple,
                    serialize_with: None,
                    deserialize_with: None,
                },
            ),
        ])
        .bound_elements(vec![(
            ExpandedName::new(LocalName::new_dangerous("facet"), Some(XmlNamespace::XS)),
            TypeReference::new_static(parse_quote!(crate::xs::Facet)),
        )])
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
                .output_file("xmlity-ns-xhtml/src/xhtml.rs".parse().unwrap())
                .namespace(XmlNamespace::XHTML)
                // .bon_builders(true)
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
