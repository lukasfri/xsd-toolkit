use syn::File;
use xmlity::types::utils::XmlRoot;
use xsd_xmlity::schema::Schema;

#[test]
fn generate_xsd_types() {
    let schema = include_str!("../../schemas/XMLSchema.xsd");
    let mut schema: XmlRoot<Schema> = xmlity_quick_xml::from_str(schema).unwrap();
    let schema_pos = schema
        .elements
        .iter()
        .position(|e| matches!(e, xmlity::types::utils::XmlRootTop::Value(_)))
        .unwrap();

    let schema = schema.elements.remove(schema_pos);
    let xmlity::types::utils::XmlRootTop::Value(schema) = schema else {
        unreachable!()
    };

    let schema = schema.into();

    // let schema = schema.
    let items = xsd_codegen_xmlity::Generator::generate(&schema).unwrap();

    let file = File {
        shebang: None,
        attrs: vec![syn::parse_quote! { #![allow(unused_imports, non_snake_case)] }],
        items,
    };

    let output = prettyplease::unparse(&file);

    std::fs::write("tests/output.rs", output.as_bytes()).unwrap();
}
