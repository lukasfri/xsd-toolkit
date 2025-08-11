//! This module tries to import and then export XSD files into compiled form and ensure that it matches the original output.
use std::env::current_dir;

use rstest::rstest;
use xmlity::types::utils::{XmlRoot, XmlRootTop};
use xsd_fragments::fragments::{complex::SchemaFragment, FragmentedXsdDocumentIdx};

#[rstest]
#[case::dxs_example_18_1_prod2("definitive_xml_schema/example_18_1/prod2.xsd")]
#[case::dxs_example_18_2_prod2("definitive_xml_schema/example_18_2/prod2.xsd")]
#[case::dxs_example_18_3_prod2("definitive_xml_schema/example_18_3/prod2.xsd")]
#[case::dxs_example_18_4_prod2("definitive_xml_schema/example_18_4/prod2.xsd")]
#[case::dxs_example_18_5_prod2("definitive_xml_schema/example_18_5/prod2.xsd")]
#[case::dxs_example_18_6_prod2("definitive_xml_schema/example_18_6/prod2.xsd")]
#[case::dxs_example_18_7_prod2("definitive_xml_schema/example_18_7/prod2.xsd")]
#[case::dxs_example_18_8_prod2("definitive_xml_schema/example_18_8/prod2.xsd")]
#[case::dxs_example_18_9_prod2("definitive_xml_schema/example_18_9/prod2.xsd")]
#[case::dxs_example_18_10_prod2("definitive_xml_schema/example_18_10/prod2.xsd")]
#[case::dxs_example_18_11_prod2("definitive_xml_schema/example_18_11/prod2.xsd")]
#[case::dxs_example_18_12_prod2("definitive_xml_schema/example_18_12/prod2.xsd")]
#[case::dxs_example_18_13_prod2("definitive_xml_schema/example_18_13/prod2.xsd")]
#[case::dxs_example_18_1_prod1("definitive_xml_schema/example_18_1/prod1.xsd")]
#[case::dxs_example_18_2_prod1("definitive_xml_schema/example_18_2/prod1.xsd")]
#[case::dxs_example_18_3_prod1("definitive_xml_schema/example_18_3/prod1.xsd")]
#[case::dxs_example_18_4_prod1("definitive_xml_schema/example_18_4/prod1.xsd")]
#[case::dxs_example_18_5_prod1("definitive_xml_schema/example_18_5/prod1.xsd")]
#[case::dxs_example_18_6_prod1("definitive_xml_schema/example_18_6/prod1.xsd")]
#[case::dxs_example_18_7_prod1("definitive_xml_schema/example_18_7/prod1.xsd")]
#[case::dxs_example_18_8_prod1("definitive_xml_schema/example_18_8/prod1.xsd")]
#[case::dxs_example_18_9_prod1("definitive_xml_schema/example_18_9/prod1.xsd")]
#[case::dxs_example_18_10_prod1("definitive_xml_schema/example_18_10/prod1.xsd")]
#[case::dxs_example_18_11_prod1("definitive_xml_schema/example_18_11/prod1.xsd")]
#[case::dxs_example_18_12_prod1("definitive_xml_schema/example_18_12/prod1.xsd")]
#[case::dxs_example_18_13_prod1("definitive_xml_schema/example_18_13/prod1.xsd")]
fn import_export_test(#[case] xsd_file_path: &str) {
    let xsd_file_path = current_dir().unwrap().join("tests").join(xsd_file_path);

    let text = std::fs::read_to_string(&xsd_file_path).unwrap();

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

    let document = SchemaFragment::from_schema(&schema, FragmentedXsdDocumentIdx::new(0)).unwrap();

    let actual = document.to_schema().unwrap();

    // let actual_xml = xmlity_quick_xml::to_string(&actual).unwrap();

    pretty_assertions::assert_eq!(
        schema, actual,
        // "Testing with xml: \"{text}\", reconstruction is \"{actual_xml}\""
    );
}
