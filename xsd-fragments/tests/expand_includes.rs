//! Integration tests for xsd-fragments library
use std::{env::current_dir, fs};

use pretty_assertions::assert_eq;
use rstest::rstest;
use std::convert::Infallible;
use url::Url;
use xmlity::types::utils::{XmlRoot, XmlRootTop};
use xsd::set::XmlSchemaSet;
use xsd_fragments::{transformers::context::complex::ExpandIncludeFragments, XmlnsContext};

#[rstest]
#[case::dxs_example_4_1(
    "definitive_xml_schema/example_4_1/ord1.xsd",
    "definitive_xml_schema/example_4_1/expanded.xsd",
    Vec::new()
)]
#[case::dxs_example_4_2(
    "definitive_xml_schema/example_4_2/ord1.xsd",
    "definitive_xml_schema/example_4_2/expanded.xsd",
    Vec::new()
)]
#[case::include_examples_sub_path("include_examples/parent.xsd", "include_examples/expected.xsd", vec![
    "include_examples/folder/sub.xsd"
])]
fn test(#[case] path: &str, #[case] result: &str, #[case] bypass: Vec<&str>) {
    use xsd::xs;
    use xsd_fragments::{transformers::TransformChange, FragmentedXsdDocumentKey};

    let current_dir = current_dir().unwrap().join("tests");

    let entry_path = current_dir.join(path);
    let result_path = current_dir.join(result);

    let mut map = XmlSchemaSet::new();
    let root_url = Url::from_file_path(&entry_path).unwrap();
    map.inform_location(&root_url);

    let resolver = |url: &Url| {
        let path = url.to_file_path().unwrap();

        let text = std::fs::read_to_string(&path)
            .unwrap_or_else(|_| panic!("Failed to read schema file at path: {}", path.display()));

        let schema: XmlRoot<xsd::xs::Schema> =
            xmlity_quick_xml::from_str(&text).expect("Failed to parse XML Schema");

        let schema = schema
            .elements
            .into_iter()
            .find_map(|a| match a {
                XmlRootTop::Value(x) => Some(x),
                _ => None,
            })
            .expect("No schema found in XML");

        Result::<_, Infallible>::Ok(schema)
    };
    map.explore_locations(&resolver)
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    let mut context = XmlnsContext::new();

    map.locations.keys().for_each(|key| {
        let _ = context.import_namespace_map(&map, key).unwrap();
    });

    let root_id = context
        .namespace_idxs
        .get(&FragmentedXsdDocumentKey(root_url.clone()))
        .expect("Failed to get root namespace ID")
        .clone();

    let bypass = bypass
        .into_iter()
        .map(|a| Url::from_file_path(&current_dir.join(a)).unwrap())
        .collect();

    let changed = context
        .context_transform(ExpandIncludeFragments::new().with_bypass(bypass))
        .unwrap();

    assert_eq!(changed, TransformChange::Changed);

    let actual_schema = context.export_schema(&root_id).unwrap();

    let expected_xml = fs::read_to_string(&result_path).unwrap();
    let expected_schema: xs::Schema =
        xmlity_quick_xml::from_str(&expected_xml).expect("Failed to parse XML Schema");

    assert_eq!(
        actual_schema, expected_schema,
        "Schemas do not match for path: {}",
        path
    );
}
