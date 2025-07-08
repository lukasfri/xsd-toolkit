use std::str::FromStr;

use url::Url;

#[tokio::test]
async fn main() {
    println!("Testing xsd-namespace-map with reqwest...");
    let mut map = xsd_namespace_map::XmlNamespaceMap::new();
    let client = reqwest::Client::new();
    let resolver = xsd_namespace_map::resolvers::reqwest::ReqwestXmlSchemaResolver::new(client);
    map.inform_location(
        &Url::from_str("https://xbrl.org/2003/xbrl-instance-2003-12-31.xsd").unwrap(),
    );

    map.explore_locations_async(resolver).await;

    panic!(
        "{:#?}",
        map.locations.keys().map(|a| a.as_str()).collect::<Vec<_>>()
    );
}
