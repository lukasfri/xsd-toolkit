use xmlity::types::utils::XmlRoot;
use xsd::schema as xs;

pub mod top_level_attribute_groups;
pub mod top_level_complex_type;
pub mod top_level_element;
pub mod top_level_group;
pub mod top_level_simple_type;

#[test]
fn schema_deserialize() {
    let xml = include_str!("../../../schemas/XMLSchema.xsd");

    let _schema: XmlRoot<xs::Schema> = xmlity_quick_xml::from_str(xml).unwrap();
}
