use xmlity::types::utils::{XmlRoot, XmlRootTop};
use xsd::xs;

// pub mod top_level_element;

#[test]
fn schema_deserialize() {
    let xml = include_str!("../../../schemas/xhtml1-strict.xsd");

    let schema: XmlRoot<xs::Schema> = xmlity_quick_xml::from_str(xml).unwrap();

    println!("{:?}", schema);

    let schema = schema.elements.into_iter().find_map(|a| match a {
        XmlRootTop::Value(schema) => Some(schema),
        _ => None,
    });

    println!("{:?}", schema);
}
