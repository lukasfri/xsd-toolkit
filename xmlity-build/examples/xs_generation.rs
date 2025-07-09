//! This example is used to generate the `xmlity-ns-xs` crate.
//!
//! The `xmlity-ns-xs` crate can not use `xmlity-build` as a dependency, because it is itself a dependency of `xmlity-build`. Therefore, this example is used to generate the `xmlity-ns-xs` crate.
use syn::parse_quote;
use xmlity::{ExpandedName, LocalName, XmlNamespace};
use xsd_codegen_xmlity::{misc::TypeReference, BoundType, TypeType};

fn main() {
    println!("Building the engine...");

    let time = std::time::Instant::now();

    let engine = xmlity_build::BuildEngine::builder()
        .glob_patterns(vec!["schemas/**/*.xsd".to_string()])
        .url_net_resolution(true)
        .bound_namespaces(vec![
            (XmlNamespace::XML, parse_quote!(xmlity_ns_xml)),
            (XmlNamespace::XS, parse_quote!(crate)),
        ])
        .bound_types(vec![
            (
                ExpandedName::new(LocalName::new_dangerous("QName"), Some(XmlNamespace::XS)),
                BoundType {
                    ty: TypeReference::new_static(parse_quote!(crate::types::QName)),
                    ty_type: TypeType::Simple,
                    serialize_with: None,
                    deserialize_with: None,
                },
            ),
            (
                ExpandedName::new(LocalName::new_dangerous("anyURI"), Some(XmlNamespace::XS)),
                BoundType {
                    ty: TypeReference::new_static(parse_quote!(crate::types::TargetNamespace)),
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
            TypeReference::new_static(parse_quote!(crate::Facet)),
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
                .output_file("schemas/xmlity-ns-xs/src/xs_generated.rs".parse().unwrap())
                .namespace(XmlNamespace::XS)
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
