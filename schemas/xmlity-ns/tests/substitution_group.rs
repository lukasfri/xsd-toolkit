use xmlity::{Deserialize, ExpandedName, LocalName, XmlNamespace};
use xmlity_ns::{SubstitutionGroup, SubstitutionGroupContext};
use xmlity_quick_xml::de::ExternalData;

#[derive(Debug, PartialEq, xmlity::Deserialize, xmlity::Serialize)]
#[xelement(name = "b", namespace = "http://example.com/ns")]
struct B {
    #[xelement(name = "c", namespace = "http://example.com/ns")]
    c: String,
}

#[derive(Debug, PartialEq, xmlity::Deserialize, xmlity::Serialize)]
enum A {
    #[xelement(name = "a", namespace = "http://example.com/ns")]
    Elem {
        b: B,
    },
    Sub(SubstitutionGroup<A>),
}

#[derive(Debug, PartialEq, xmlity::Deserialize, xmlity::Serialize)]
#[xelement(name = "root", namespace = "http://example.com/ns")]
struct Root {
    a: A,
}

#[rstest::rstest]
#[case::test_1(r###"
<root xmlns="http://example.com/ns">
    <a>
        <b>
            <c>Hello</c>
        </b>
    </a>
    </root>
"###, Root {
    a: A::Elem {
        b: B {
            c: "Hello".to_string(),
        },
    },
})]
#[case::test_1(r###"
    <root xmlns="http://example.com/ns">
        <d>
            <inner>abc</inner>
        </d>
    </root>
"###, Root {
    a: A::Sub(SubstitutionGroup::new(xmlity::xml!(
        <"d":"http://example.com/ns">["
            "<"inner":"http://example.com/ns">["abc"]</"inner">"
        "]</"d">
    ))),
})]
fn substitution_group_test(#[case] input: &str, #[case] val: Root) {
    use pretty_assertions::assert_eq;

    let allowed_ns = ExpandedName::new(
        LocalName::new("d").unwrap(),
        Some(XmlNamespace::new("http://example.com/ns").unwrap()),
    );

    let mut external_data = ExternalData::new();

    external_data.insert(SubstitutionGroupContext::<A>::new(vec![allowed_ns]));

    let mut deserializer = xmlity_quick_xml::Deserializer::from(input.trim().as_bytes())
        .with_external_data(external_data);

    let root: Root = Deserialize::deserialize(&mut deserializer).unwrap();

    println!("{:#?}", root);

    assert_eq!(root, val);
}

#[test]
fn test2() {
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone,
    )]
    enum Part {
        Dynamic(crate::SubstitutionGroup<Part>),
    }

    let mut data = ExternalData::new();
    data.insert(SubstitutionGroupContext::<Part>::new(vec![
        ExpandedName::new(
            LocalName::new("Page").unwrap(),
            Some(XmlNamespace::new("http://mycompany.com/xbrl/roleR").unwrap()),
        ),
        ExpandedName::new(
            LocalName::new("Paragraph").unwrap(),
            Some(XmlNamespace::new("http://mycompany.com/xbrl/roleR").unwrap()),
        ),
    ]));

    let mut deserializer = xmlity_quick_xml::Deserializer::from(
        r#"<mycomp:Page xmlns:mycomp="http://mycompany.com/xbrl/roleR">10</mycomp:Page>"#
            .as_bytes(),
    )
    .with_external_data(data);

    let part = Part::deserialize(&mut deserializer).expect("Failed to parse part XML");

    println!("Parsed part: {:?}", part);
}
