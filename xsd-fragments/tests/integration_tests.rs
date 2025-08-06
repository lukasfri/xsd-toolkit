use std::env::current_dir;

use rstest::rstest;
use std::convert::Infallible;
use url::Url;
use xmlity::types::utils::{XmlRoot, XmlRootTop};
use xsd::set::XmlSchemaSet;
use xsd_fragments::XmlnsContext;

#[rstest]
#[case::dxs_example_18_1("definitive_xml_schema/example_18_1/prod2.xsd")]
#[case::dxs_example_18_2("definitive_xml_schema/example_18_2/prod2.xsd")]
#[case::dxs_example_18_3("definitive_xml_schema/example_18_3/prod2.xsd")]
#[case::dxs_example_18_4("definitive_xml_schema/example_18_4/prod2.xsd")]
#[case::dxs_example_18_5("definitive_xml_schema/example_18_5/prod2.xsd")]
#[case::dxs_example_18_6("definitive_xml_schema/example_18_6/prod2.xsd")]
#[case::dxs_example_18_7("definitive_xml_schema/example_18_7/prod2.xsd")]
#[case::dxs_example_18_8("definitive_xml_schema/example_18_8/prod2.xsd")]
#[case::dxs_example_18_9("definitive_xml_schema/example_18_9/prod2.xsd")]
#[case::dxs_example_18_10("definitive_xml_schema/example_18_10/prod2.xsd")]
#[case::dxs_example_18_11("definitive_xml_schema/example_18_11/prod2.xsd")]
#[case::dxs_example_18_12("definitive_xml_schema/example_18_12/prod2.xsd")]
#[case::dxs_example_18_13("definitive_xml_schema/example_18_13/prod2.xsd")]
fn test(#[case] path: &str) {
    let path = current_dir().unwrap().join("tests").join(path);

    let mut map = XmlSchemaSet::new();
    let root_url = Url::from_file_path(&path).unwrap();
    println!("Informing location: {}", root_url);
    map.inform_location(&root_url);

    let resolver = |url: &Url| {
        let path = url.to_file_path().unwrap();

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

    println!(
        "Schema set: {:?}",
        map.locations
            .keys()
            .map(|a| a.to_string())
            .collect::<Vec<_>>()
    );

    let mut context = XmlnsContext::new();
    context.import_namespace_map(&map, &root_url, None).unwrap();

    println!(
        "Context: {:?}",
        context.namespace_idxs.keys().collect::<Vec<_>>()
    );
}
