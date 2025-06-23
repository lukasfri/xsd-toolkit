use syn::parse_quote;
use xmlity::{ExpandedName, LocalName, XmlNamespace};
use xsd_codegen_xmlity::{misc::TypeReference, BoundType, TypeType};

fn main() {
    let time = std::time::Instant::now();
    println!("Building the engine... {}", time.elapsed().as_secs_f32());

    let custom_xs_types: Vec<(&'static str, syn::Ident)> = vec![
        // ("TODO", parse_quote!(AttributeUseType)),
        // ("TODO", parse_quote!(BasicNamespaceListItemType)),
        ("basicNamespaceList", parse_quote!(BasicNamespaceListType)),
        // ("TODO", parse_quote!(BlockSetItemList)),
        // ("TODO", parse_quote!(BlockSetItemType)),
        ("blockSet", parse_quote!(BlockSetType)),
        // ("TODO", parse_quote!(DefaultOpenContentModeType)),
        ("derivationSet", parse_quote!(DerivationSetType)),
        // ("TODO", parse_quote!(ElementSubstitutionGroupType)),
        ("formChoice", parse_quote!(FormChoiceType)),
        ("fullDerivationSet", parse_quote!(FullDerivationSetType)),
        ("allNNI", parse_quote!(AllNNI)),
        ("namespaceList", parse_quote!(NamespaceListType)),
        // ("TODO", parse_quote!(OpenContentModeType)),
        // ("TODO", parse_quote!(ProcessContentsType)),
        ("QName", parse_quote!(QName)),
        // ("TODO", parse_quote!(QnameListAItemType)),
        ("qnameListA", parse_quote!(QnameListAType)),
        // ("TODO", parse_quote!(QnameListItemType)),
        ("qnameList", parse_quote!(QnameListType)),
        // ("TODO", parse_quote!(ReducedDerivationControlList)),
        (
            "reducedDerivationControl",
            parse_quote!(ReducedDerivationControlType),
        ),
        // ("TODO", parse_quote!(SimpleDerivationSetItemList)),
        // ("TODO", parse_quote!(SimpleDerivationSetItemType)),
        // ("TODO", parse_quote!(SimpleDerivationSetType)),
        ("anyURI", parse_quote!(TargetNamespace)),
        // ("TODO", parse_quote!(TypeDerivationControlList)),
        (
            "typeDerivationControl",
            parse_quote!(TypeDerivationControlType),
        ),
        (
            "xpathDefaultNamespace",
            parse_quote!(XpathDefaultNamespaceType),
        ),
    ];

    let engine = xmlity_build::BuildEngine::builder()
        .glob_patterns(vec!["./schemas/**/*.xsd".to_string()])
        .url_net_resolution(true)
        .bound_namespaces(vec![
            (XmlNamespace::XML, parse_quote!(crate::xml)),
            (XmlNamespace::XHTML, parse_quote!(crate::xhtml)),
            (XmlNamespace::XS, parse_quote!(crate::xs)),
        ])
        .bound_types(
            custom_xs_types
                .into_iter()
                .map(|(name, type_ident)| {
                    (
                        ExpandedName::new(LocalName::new_dangerous(name), Some(XmlNamespace::XS)),
                        BoundType {
                            ty: TypeReference::new_static(
                                parse_quote!(crate::xs::types::#type_ident),
                            ),
                            ty_type: TypeType::Simple,
                            serialize_with: None,
                            deserialize_with: None,
                        },
                    )
                })
                .chain(Some((
                    ExpandedName::new(LocalName::new_dangerous("NCName"), Some(XmlNamespace::XS)),
                    BoundType {
                        ty: TypeReference::new_static(parse_quote!(::xmlity::LocalName<'static>)),
                        ty_type: TypeType::Simple,
                        serialize_with: None,
                        deserialize_with: None,
                    },
                )))
                .collect(),
        )
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
                .output_file("xsd/src/xs_generated.rs".parse().unwrap())
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
