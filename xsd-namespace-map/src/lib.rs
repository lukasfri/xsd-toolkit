use std::str::FromStr;

use url::Url;

mod map;
pub mod resolvers;
pub use map::{GlobError, SchemaLocation, XmlNamespaceMap};

pub fn resolve_xml_url(base_url: &Url, relative_url: &str) -> Result<Url, url::ParseError> {
    match Url::from_str(relative_url) {
        Err(url::ParseError::RelativeUrlWithoutBase) => {
            let mut url = base_url.clone();

            url.path_segments_mut().unwrap().pop();
            url.path_segments_mut().unwrap().push(relative_url);

            Ok(url)
        }
        r => r,
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr as _;

    use rstest::rstest;
    use url::Url;

    use crate::resolve_xml_url;

    #[rstest]
    #[case::relative_file(
        Url::from_str("file:///XBRL-CONF-2025-01-14/Common/100-schema/102-01-SpecExample.xml").unwrap(),
        "102-01-SpecExample.xsd",
        Url::from_str("file:///XBRL-CONF-2025-01-14/Common/100-schema/102-01-SpecExample.xsd").unwrap()
    )]
    fn resolve_xml_url_test(#[case] base: Url, #[case] relative: &str, #[case] expected: Url) {
        let actual = resolve_xml_url(&base, relative).unwrap();

        assert_eq!(actual, expected);
    }
}
