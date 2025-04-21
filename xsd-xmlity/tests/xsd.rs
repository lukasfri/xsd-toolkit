use xmlity::types::utils::XmlRoot;
use xsd_xmlity::schema::Schema;

#[test]
fn deserialize_total_schema_file() {
    let schema = include_str!("../../schemas/XMLSchema.xsd");
    let schema: XmlRoot<Schema> = xmlity_quick_xml::from_str(schema).unwrap();

    std::fs::write("./schema.debug", format!("{:#?}", schema)).unwrap();

    std::fs::write(
        "./backto.xml",
        xmlity_quick_xml::ser::to_string_pretty(&schema, 2).unwrap(),
    )
    .unwrap();
}
