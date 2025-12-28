use std::{collections::BTreeMap, str::FromStr};

use rstest::rstest;
use url::Url;
use xsd_fragments::FragmentedXsdDocumentKey;

use std::collections::HashSet;

use xmlity::{ExpandedName, Serialize};
use xsd::xsn;
use xsd_codegen_xmlity::CodegenTransformer;
use xsd_fragments::XmlnsContext;

fn serialize_with_ns_prefix<T: Serialize>(value: &T) -> String {
    let writer = quick_xml::Writer::new_with_indent(Vec::new(), b' ', 2);
    let mut serializer = xmlity_quick_xml::Serializer::new_with_namespaces(
        writer,
        BTreeMap::from_iter([(
            "http://www.w3.org/2001/XMLSchema".parse().unwrap(),
            "xs".parse().unwrap(),
        )]),
    );
    value.serialize(&mut serializer).unwrap();
    let bytes = serializer.into_inner();

    String::from_utf8(bytes).unwrap()
}

#[rstest]
#[case(include_str!("type_collapsing/simple/in1.xsd"), include_str!("type_collapsing/simple/out1.xsd"))]
fn collapse(#[case] in_xml: &str, #[case] out_xml: &str) {
    let in_schema = xmlity_quick_xml::from_str::<xmlity_ns_xs::Schema>(in_xml).unwrap();

    let mut context = XmlnsContext::new();

    let key = FragmentedXsdDocumentKey(Url::from_str("http://example.com/schema1").unwrap());
    let (id, _) = context.import_schema(key, &in_schema).unwrap();

    let allowed_simple_bases: HashSet<ExpandedName<'static>> = [&xsn::TOKEN, &xsn::NMTOKEN]
        .iter()
        .map(|a| (***a).clone())
        .collect();

    context
        .context_transform(CodegenTransformer::new(allowed_simple_bases.clone()))
        .unwrap();

    let actual = context
        .export_schema(&id)
        .expect("Transformed schema not found");

    let actual_xml = serialize_with_ns_prefix(&actual);

    pretty_assertions::assert_eq!(out_xml, actual_xml);
}
