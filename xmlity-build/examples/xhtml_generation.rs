use syn::parse_quote;
use url::Url;
use xmlity_build::{reexports::FragmentedXsdDocumentKey, BuildEngine, GenerateNamespaceConfig};

const XHTML: &str = "http://www.xbrl.org/2013/inlinexbrl/xhtml/xhtml11.xsd";
const XML: &str = "http://www.w3.org/2001/xml.xsd";
const XHTML_DATATYPES: &str = "http://www.xbrl.org/2013/inlinexbrl/xhtml/xhtml-datatypes-1.xsd";

fn main() {
    let out_dir = std::env::current_dir()
        .expect("Failed to get current dir")
        .join("xmlity-build/tests/xhtml_schema");

    let xhtml_url = Url::parse(XHTML).expect("Failed to parse XBRL xlink URL");
    let xml_url = Url::parse(XML).expect("Failed to parse XML URL");
    let xhtml_datatypes_url =
        Url::parse(XHTML_DATATYPES).expect("Failed to parse XML datatypes URL");

    let engine = BuildEngine::builder()
        .urls(vec![
            xhtml_url.clone(),
            xml_url.clone(),
            xhtml_datatypes_url.clone(),
        ])
        .bound_namespaces(vec![
            (
                FragmentedXsdDocumentKey(xhtml_url.clone()),
                parse_quote!(crate::xhtml_schema::xhtml),
            ),
            (
                FragmentedXsdDocumentKey(xml_url.clone()),
                parse_quote!(xmlity_ns_xml),
            ),
            (
                FragmentedXsdDocumentKey(xhtml_datatypes_url.clone()),
                parse_quote!(crate::xhtml_schema::xhtml_datatypes),
            ),
        ])
        .bound_types(
            xsd_codegen_xmlity_xsd_types::StringXsdTypes
                .into_iter()
                .collect(),
        )
        .build();

    let engine = engine.start().unwrap();

    engine
        .generate_namespace(
            GenerateNamespaceConfig::builder()
                .namespace(FragmentedXsdDocumentKey(xhtml_datatypes_url))
                .enum_from_impls(true)
                .struct_from_impls(true)
                .build(),
            &out_dir.join("xhtml_datatypes.rs"),
        )
        .unwrap();

    engine
        .generate_namespace(
            GenerateNamespaceConfig::builder()
                .namespace(FragmentedXsdDocumentKey(xhtml_url))
                .enum_from_impls(true)
                .struct_from_impls(true)
                .build(),
            &out_dir.join("xhtml.rs"),
        )
        .unwrap();
}
