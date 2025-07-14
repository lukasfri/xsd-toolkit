use xmlity_ns_xhtml::xhtml1_strict as xhtml;

#[rstest::rstest]
#[case::composition(XHTML_HTML, Some(xhtml_html()))]
#[ntest::timeout(1000)]
fn deserialize(#[case] xml: &str, #[case] expected: Option<xhtml::Html>) {
    let xml = xml.trim();
    let element: xhtml::Html = xmlity_quick_xml::de::from_str(xml).unwrap();

    if let Some(expected) = expected {
        pretty_assertions::assert_eq!(element, expected);
    }
}

const XHTML_HTML: &str = r###"
<html xmlns="http://www.w3.org/1999/xhtml">
  <head>
    <title>Title of the document</title>
  </head>
  <body>
    Content of the page
  </body>
</html>
"###;

fn xhtml_html() -> xhtml::Html {
    xhtml::html_items::Html {
        lang: None,
        lang_0: None,
        dir: None,
        id: None,
        head: xhtml::head_items::Head {
            lang: None,
            lang_0: None,
            dir: None,
            id: None,
            profile: None,
            head_misc: xhtml::groups::HeadMisc {
                child_0: Vec::new(),
            },
            variant: xhtml::head_items::variant_variants::Variant0 {
                title: Box::new(
                    xhtml::title_items::Title {
                        lang: None,
                        lang_0: None,
                        dir: None,
                        id: None,
                    }
                    .into(),
                ),
                head_misc: xhtml::groups::HeadMisc {
                    child_0: Vec::new(),
                },
                child_2: None,
            }
            .into(),
        }
        .into(),
        body: xhtml::body_items::Body {
            id: None,
            class: None,
            style: None,
            title: None,
            lang: None,
            lang_0: None,
            dir: None,
            onclick: None,
            ondblclick: None,
            onmousedown: None,
            onmouseup: None,
            onmouseover: None,
            onmousemove: None,
            onmouseout: None,
            onkeypress: None,
            onkeydown: None,
            onkeyup: None,
            onload: None,
            onunload: None,
            body_content: vec![],
        }
        .into(),
    }
    .into()
}
