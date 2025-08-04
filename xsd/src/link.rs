use std::str::FromStr;

use url::Url;

/// Extension trait for [`Url`] handling in XML contexts.
pub trait UrlExt {
    /// Resolves a relative XML link against this URL.
    fn resolve_xml_url(&self, relative_xml_link: &str) -> Result<Url, url::ParseError>;
}

impl UrlExt for Url {
    fn resolve_xml_url(&self, relative_xml_link: &str) -> Result<Url, url::ParseError> {
        let (path, fragment) = match relative_xml_link.split_once('#') {
            Some((path, fragment)) => (path, Some(fragment)),
            None => (relative_xml_link, None),
        };

        match Url::from_str(relative_xml_link) {
            Err(url::ParseError::RelativeUrlWithoutBase) => {
                let mut url = self.clone();

                url = url.join(path)?;
                if let Some(fragment) = fragment {
                    url.set_fragment(Some(fragment));
                }

                Ok(url)
            }
            r => r,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::UrlExt as _;
    use std::str::FromStr as _;

    use rstest::rstest;
    use url::Url;

    #[rstest]
    #[case::relative_file(
        Url::from_str("file:///XBRL-CONF-2025-01-14/Common/100-schema/102-01-SpecExample.xml").unwrap(),
        "102-01-SpecExample.xsd",
        Url::from_str("file:///XBRL-CONF-2025-01-14/Common/100-schema/102-01-SpecExample.xsd").unwrap()
    )]
    #[case::relative_file(
        Url::from_str("file:///XBRL-CONF-2025-01-14/Common/100-schema/102-01-SpecExample.xml").unwrap(),
        "107-01-SchemaContainingALinkbase.xsd#labelLinkbase",
        Url::from_str("file:///XBRL-CONF-2025-01-14/Common/100-schema/107-01-SchemaContainingALinkbase.xsd#labelLinkbase").unwrap()
    )]
    #[case::relative_file(
        Url::from_str("https://xbrl.taxonomier.se/se/fr/gaap/k2-all/ab/risbs/2024-09-12/se-k2-ab-risbs-2024-09-12.xsd").unwrap(),
        "../../form/se-k2-ab-fdisc/2024-09-12/se-k2-ab-fdisc-2024-09-12.xsd",
        Url::from_str("https://xbrl.taxonomier.se/se/fr/gaap/k2-all/ab/form/se-k2-ab-fdisc/2024-09-12/se-k2-ab-fdisc-2024-09-12.xsd").unwrap()
    )]
    #[case::relative_file(
        Url::from_str("https://xbrl.taxonomier.se/se/fr/gaap/k2-all/ab/form/se-k2-ab-fdisc/2024-09-12/se-k2-ab-fdisc-2024-09-12.xsd").unwrap(),
        "../../../../../../../common/domain/gaap/se-k2-all-ext/ab/2024-09-12/se-k2-ab-ext-2024-09-12.xsd",
        Url::from_str("https://xbrl.taxonomier.se/se/common/domain/gaap/se-k2-all-ext/ab/2024-09-12/se-k2-ab-ext-2024-09-12.xsd").unwrap()
    )]
    fn resolve_xml_url_test(#[case] base: Url, #[case] relative: &str, #[case] expected: Url) {
        let actual = base.resolve_xml_url(relative).unwrap();

        assert_eq!(actual.to_string(), expected.to_string());
        assert_eq!(actual, expected);
    }
}
