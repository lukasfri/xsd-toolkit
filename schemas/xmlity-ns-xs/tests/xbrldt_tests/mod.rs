use rstest::rstest;
use xmlity::types::utils::XmlRoot;
use xmlity_ns_xs as xs;

pub mod top_level_element;

#[rstest]
#[ntest::timeout(100)]
fn schema_deserialize() {
    let xml = include_str!("./xbrldt-2005.xsd");

    let schema: XmlRoot<xs::Schema> = xmlity_quick_xml::from_str(xml).unwrap();

    println!("{schema:?}");
}
