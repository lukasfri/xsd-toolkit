use xmlity::{ExpandedName, LocalName, XmlNamespace};
use xmlity_ns::SubstitutionGroup;
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

    use xmlity_ns::SubstitutionGroupContext;

    let allowed_ns = ExpandedName::new(
        LocalName::new_dangerous("d"),
        Some(XmlNamespace::new_dangerous("http://example.com/ns")),
    );

    let mut external_data = ExternalData::new();

    external_data.insert(SubstitutionGroupContext::<A>::new(vec![allowed_ns]));

    let mut deserializer = xmlity_quick_xml::Deserializer::from(input.trim().as_bytes())
        .with_external_data(external_data);

    let root: Root = xmlity::Deserialize::deserialize(&mut deserializer).unwrap();

    println!("{:#?}", root);

    assert_eq!(root, val);
}
