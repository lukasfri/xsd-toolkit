use xmlity::types::utils::XmlRoot;
use xsd::schema as xs;

#[test]
fn schema_deserialize() {
    let xml = include_str!("../../../schemas/xhtml1-strict.xsd");

    let _schema: XmlRoot<xs::Schema> = xmlity_quick_xml::from_str(xml).unwrap();
}
