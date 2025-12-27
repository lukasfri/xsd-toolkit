use rstest::rstest;
use xmlity::Deserialize;

mod xhtml_schema;

#[rstest]
#[case(include_str!(
    "./data/simple.xhtml"
))]
fn html(#[case] inline_instance_str: &str) {
    let mut deserializer = xmlity_quick_xml::Deserializer::from(inline_instance_str.as_bytes());

    println!("Parsing ixbrl instance...");
    let inline_instance = xhtml_schema::xhtml::Html::deserialize(&mut deserializer)
        // .map(root_to_value)
        .expect("Failed to parse inline_instance from string");
    println!("Parsed ixbrl instance successfully.");

    println!("{:?}", inline_instance);
}

#[rstest]
#[case(include_str!(
    "./data/title.xhtml"
))]
fn title(#[case] inline_instance_str: &str) {
    let mut deserializer = xmlity_quick_xml::Deserializer::from(inline_instance_str.as_bytes());

    println!("Parsing ixbrl instance...");
    let inline_instance = xhtml_schema::xhtml::groups::xhtml_head_content_items::variant_variants::Variant0::deserialize(&mut deserializer)
        // .map(root_to_value)
        .expect("Failed to parse inline_instance from string");
    println!("Parsed ixbrl instance successfully.");

    println!("{:?}", inline_instance);
}

#[rstest]
#[case(include_str!(
    "./data/body.xhtml"
))]
fn body(#[case] inline_instance_str: &str) {
    let mut deserializer = xmlity_quick_xml::Deserializer::from(inline_instance_str.as_bytes());

    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone,
    )]
    #[xelement(name = "body", namespace = "http://www.w3.org/1999/xhtml")]
    pub struct Html(#[xgroup] ::std::boxed::Box<crate::xhtml_schema::xhtml::types::XhtmlBodyType>);

    println!("Parsing ixbrl instance...");
    let inline_instance = Html::deserialize(&mut deserializer)
        // .map(root_to_value)
        .expect("Failed to parse inline_instance from string");
    println!("Parsed ixbrl instance successfully.");

    println!("{:?}", inline_instance);
}

const XHTML_BODY_CONTENT: &str = r###"

  some content here...

"###;

#[rstest]
#[case(XHTML_BODY_CONTENT)]
fn body_content(#[case] inline_instance_str: &str) {
    let mut deserializer = xmlity_quick_xml::Deserializer::from(inline_instance_str.as_bytes());

    println!("Parsing ixbrl instance...");
    let inline_instance =
        Vec::<xhtml_schema::xhtml::groups::XhtmlBlockMix>::deserialize(&mut deserializer)
            // .map(root_to_value)
            .expect("Failed to parse inline_instance from string");
    println!("Parsed ixbrl instance successfully.");

    println!("{:?}", inline_instance);
}

#[rstest]
#[case(XHTML_BODY_CONTENT)]
fn body_content2(#[case] inline_instance_str: &str) {
    let mut deserializer = xmlity_quick_xml::Deserializer::from(inline_instance_str.as_bytes());

    println!("Parsing ixbrl instance...");
    let inline_instance =
        xhtml_schema::xhtml::groups::XhtmlBlockMix::deserialize(&mut deserializer)
            // .map(root_to_value)
            .expect("Failed to parse inline_instance from string");
    println!("Parsed ixbrl instance successfully.");

    println!("{:?}", inline_instance);
}
