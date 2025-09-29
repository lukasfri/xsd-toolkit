//! Integration tests for xsd-fragments library
use std::{env::current_dir, fs};

use pretty_assertions::assert_eq;
use rstest::rstest;
use std::convert::Infallible;
use url::Url;
use xmlity::types::utils::{XmlRoot, XmlRootTop};
use xsd::set::XmlSchemaSet;
use xsd_fragments::XmlnsContext;

#[rstest]
#[case::example1_root_included(
    "attribute_groups/example1/root.xsd",
    "attribute_groups/example1/result.xsd"
)]
#[case::example1_only_redefined(
    "attribute_groups/example1/xhtml-modules-1.xsd",
    "attribute_groups/example1/result_only_redefine.xsd"
)]
#[case::example2_root_included(
    "attribute_groups/example2/root.xsd",
    "attribute_groups/example2/result.xsd"
)]
#[case::example2_only_redefined(
    "attribute_groups/example2/top.xsd",
    "attribute_groups/example2/result_only_redefine.xsd"
)]
fn test(#[case] path: &str, #[case] result: &str) {
    use std::collections::HashSet;

    use xmlity::ExpandedName;
    use xsd::{xs, xsn};
    use xsd_codegen_xmlity::CodegenTransformer;
    use xsd_fragments::{transformers::TransformChange, FragmentedXsdDocumentKey};

    let root_folder = current_dir().unwrap().join("tests");
    let entry_path = root_folder.join(path);
    let result_path = root_folder.join(result);

    let mut map = XmlSchemaSet::new();
    let root_url = Url::from_file_path(&entry_path).unwrap();
    map.inform_location(&root_url);

    let resolver = |url: &Url| {
        let path = url.to_file_path().unwrap_or_else(|()| {
            panic!(
                "Failed to convert URL to file path: {} (current dir: {})",
                url,
                current_dir().unwrap().display()
            )
        });

        let text = std::fs::read_to_string(&path).unwrap();

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

    let allowed_simple_bases: HashSet<ExpandedName<'static>> = [
        &xsn::DECIMAL,
        &xsn::FLOAT,
        &xsn::DOUBLE,
        &xsn::INTEGER,
        &xsn::NON_POSITIVE_INTEGER,
        &xsn::NEGATIVE_INTEGER,
        &xsn::LONG,
        &xsn::INT,
        &xsn::SHORT,
        &xsn::BYTE,
        &xsn::NON_NEGATIVE_INTEGER,
        &xsn::UNSIGNED_LONG,
        &xsn::UNSIGNED_INT,
        &xsn::UNSIGNED_SHORT,
        &xsn::UNSIGNED_BYTE,
        &xsn::POSITIVE_INTEGER,
        &xsn::STRING,
        &xsn::NORMALIZED_STRING,
        &xsn::TOKEN,
        &xsn::LANGUAGE,
        &xsn::NAME,
        &xsn::NCNAME,
        &xsn::ID,
        &xsn::IDREF,
        &xsn::IDREFS,
        &xsn::ENTITY,
        &xsn::ENTITIES,
        &xsn::NMTOKEN,
        &xsn::NMTOKENS,
        &xsn::DATE_TIME,
        &xsn::DATE,
        &xsn::DATE_TIME_STAMP,
        &xsn::DAY_TIME_DURATION,
        &xsn::ANY_URI,
    ]
    .iter()
    .map(|a| (***a).clone())
    .collect();

    let changed = context
        .context_transform(CodegenTransformer::new(allowed_simple_bases))
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
