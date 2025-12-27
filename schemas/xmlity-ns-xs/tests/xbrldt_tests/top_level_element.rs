use xmlity::{value::XmlText, ExpandedName, LocalName, XmlNamespace};
use xmlity_ns::{List, XmlAttribute};
use xmlity_ns_xs as xs;

#[rstest::rstest]
#[case::schema(HYPERCUBE_ITEM, None)]
#[case::any_attribute(DIMENSION_ITEM, Some(dimension_item()))]
#[ntest::timeout(1000)]
fn deserialize(#[case] xml: &str, #[case] expected: Option<xs::Element>) {
    let xml = xml.trim();
    let element: xs::Element = xmlity_quick_xml::de::from_str(xml).unwrap();

    if let Some(expected) = expected {
        pretty_assertions::assert_eq!(element, expected);
    }
}

const HYPERCUBE_ITEM: &str = r###"
<xs:element xmlns:xs="http://www.w3.org/2001/XMLSchema" xmlns:xbrli="http://www.xbrl.org/2003/instance" name="hypercubeItem" id="xbrldt_hypercubeItem" abstract="true"
  substitutionGroup="xbrli:item" type="xbrli:stringItemType" xbrli:periodType="duration" />
"###;

const DIMENSION_ITEM: &str = r###"
<xs:element xmlns:xs="http://www.w3.org/2001/XMLSchema" xmlns:xbrli="http://www.xbrl.org/2003/instance" name="dimensionItem" id="xbrldt_dimensionItem" abstract="true"
  substitutionGroup="xbrli:item" type="xbrli:stringItemType" xbrli:periodType="duration" />
"###;

const XBRLI_NAMESPACE: &'static XmlNamespace =
    unsafe { XmlNamespace::new_unchecked("http://www.xbrl.org/2003/instance") };

fn dimension_item() -> xs::Element {
    xs::types::TopLevelElement::builder()
        .name(LocalName::new("dimensionItem").unwrap().to_owned())
        .id("xbrldt_dimensionItem".to_string())
        .abstract_(true)
        .substitution_group(List::from_iter([xs::types::QName(
            ExpandedName::new(LocalName::new("item").unwrap(), Some(XBRLI_NAMESPACE)).into_owned(),
        )]))
        .type_attribute(xs::types::QName(
            ExpandedName::new(
                LocalName::new("stringItemType").unwrap(),
                Some(XBRLI_NAMESPACE),
            )
            .into_owned(),
        ))
        .any_attributes(xmlity_ns::AnyAttributes {
            attributes: vec![XmlAttribute::new(
                ExpandedName::new(LocalName::new("periodType").unwrap(), Some(XBRLI_NAMESPACE)),
                XmlText::new("duration"),
            )],
        })
        .build()
        .into()
}
