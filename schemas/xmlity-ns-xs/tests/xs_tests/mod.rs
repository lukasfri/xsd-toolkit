use xmlity::types::utils::XmlRoot;
use xmlity_ns_xs as xs;

pub mod top_level_attribute_groups;
pub mod top_level_complex_type;
pub mod top_level_element;
pub mod top_level_group;
pub mod top_level_simple_type;

#[test]
fn schema_deserialize() {
    let xml = include_str!("../../src/XMLSchema.xsd");

    let schema: XmlRoot<xs::Schema> = xmlity_quick_xml::from_str(xml).unwrap();

    println!("{schema:?}");
}
