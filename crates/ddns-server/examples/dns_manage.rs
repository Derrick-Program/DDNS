use cloudflare::endpoints::dns::dns::{ListDnsRecords, ListDnsRecordsParams};
use cloudflare::framework::{Environment, OrderDirection};
use cloudflare::framework::client::ClientConfig;
use cloudflare::framework::client::async_api::Client as AsyncClient;
use cloudflare::framework::auth::Credentials;

#[tokio::main]
async fn main() {
    let auth_key =
        std::env::var("CF_RS_AUTH_KEY").unwrap();
    let auth_email =
        std::env::var("CF_RS_AUTH_EMAIL").unwrap();
    let zone_identifier =
        std::env::var("CF_RS_ZONE_ID").unwrap();
    // let credentials = Credentials::UserAuthToken { token: "".to_string() };
    let credentials = Credentials::UserAuthKey { email: auth_email, key: auth_key };
    let client = AsyncClient::new(credentials,ClientConfig::default(),Environment::Production).unwrap();
    // get_zones(&client).await;
    get_dns_records(&client, &zone_identifier).await;
}

// async fn get_zones(client: &AsyncClient) {
//     let parms = ListZones {
//         params: ListZonesParams {
//             name: None,
//             status: None,
//             page: None,
//             per_page: None,
//             order: None,
//             direction: None,
//             search_match: None,
//         },
//     };
//     let response = client.request(&parms).await.unwrap();
//     dbg!(response);
// }

async fn get_dns_records(client: &AsyncClient, zone_identifier: &str) {
    let params = ListDnsRecords {
        zone_identifier,
        params: ListDnsRecordsParams {
            direction: Some(OrderDirection::Ascending),
            ..Default::default()
        },
    };
    let response = client.request(&params).await.unwrap();
    response.result.iter().for_each(|record| {
        println!("{}: {} -> {:#?}", record.id, record.name, record.content);
    });
    // dbg!(response);
}