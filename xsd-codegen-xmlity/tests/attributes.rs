use std::str::FromStr;

use pretty_assertions::assert_eq;

use syn::{parse_quote, File};
use url::Url;
use xmlity::{ExpandedName, LocalName, XmlNamespace};
use xsd::{ xs};
use xsd_fragments::XmlnsContext;

const SIMPLE_ATTRIBUTE_INPUT: &str = r###"
<xs:schema xmlns:xs="http://www.w3.org/2001/XMLSchema"
           xmlns:ns="http://example.com"
           targetNamespace="http://example.com">
  <xs:attribute name="SimpleAttribute" type="xs:string"/>
</xs:schema>
"###;

fn simple_attribute_expected() -> File {
    parse_quote!(
        #[derive(::core::fmt::Debug, ::xmlity::SerializeAttribute, ::xmlity::Deserialize)]
        #[xattribute(name = "SimpleAttribute", namespace = "http://example.com")]
        pub struct SimpleAttribute(pub String);
    )
}

#[rstest::rstest]
#[case::one_attribute_group(SIMPLE_ATTRIBUTE_INPUT, simple_attribute_expected(), 
    ExpandedName::new(LocalName::new("SimpleAttribute").unwrap(), Some(XmlNamespace::new("http://example.com").unwrap())),
    syn::parse_quote!(SimpleAttribute))]
fn attribute_generation(#[case] input: &str,
    #[case] expected_code: File,
    #[case] expanded_name: ExpandedName,
    #[case] expected_type: syn::Type ) {
    use xsd_codegen_xmlity::Generator;
    use xsd_fragments::FragmentedXsdDocumentKey;

    let input: xs::Schema =
        xmlity_quick_xml::from_str(input.trim()).expect("Failed to parse XML Schema");

    let mut ctx = XmlnsContext::new();

    let (id, _) = ctx
        .import_schema(
            FragmentedXsdDocumentKey(Url::from_str("http://example.com/test").unwrap()),
            &input,
        )
        .expect("Failed to import schema");

    let mut generator = Generator::new(&ctx);

    generator.bind_types(xsd_codegen_xmlity::binds::StdXsdTypes);

    let (ty, items) = generator
        .generate_attribute(
            &id,
            &expanded_name,
        )
        .expect("Failed to generate attribute");

    let actual = prettyplease::unparse(&syn::File {
        shebang: None,
        attrs: Vec::new(),
        items,
    });

    let expected_code = prettyplease::unparse(&expected_code);

    assert_eq!(actual, expected_code);

    assert_eq!(ty.into_type(None), expected_type);
}
