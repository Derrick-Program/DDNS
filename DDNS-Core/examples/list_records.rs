use anyhow::Result;
use ddns_core::{DnsProvider, provider::*};
// use serde_json::Value;
#[tokio::main]
async fn main() -> Result<()> {
    // // 確保已設置環境變數 CF_API_TOKEN
    // let token = std::env::var("CF_API_TOKEN").expect("環境變數 CF_API_TOKEN
    // 未設置"); let zone_id = std::env::var("CF_ZONE_ID").ok();
    // let domain_id = std::env::var("CF_DOMAIN_ID").ok();
    // // 創建 CloudflareProvider
    // 實例，如果已經有zone_id或是domain_id，可以在這裡設置 let mut provider =
    // CloudflareProvider::new(token, zone_id, domain_id); let cf = provider.
    // send_value(RequestMethod::Get, ApiPath::Raw("/zones"), None, None).await?;
    // let first_zone_id = CloudflareProvider::result_at(&cf, "0/id")
    //     .and_then(|v| v.as_str())
    //     .ok_or_else(|| anyhow::anyhow!("missing zone id"))?;
    // println!("第一個 Zone ID: {}", first_zone_id);
    // provider.set_zone_id(first_zone_id);
    // let records =
    //     provider.send_value(RequestMethod::Get, ApiPath::Dns, Some(&[("type",
    // "A")]), None).await?;
    //
    // let arr = CloudflareProvider::result_as_array(&records)?;
    // println!("A 記錄清單：");
    // for v in arr {
    //     let id = v.get("id").and_then(|x| x.as_str()).unwrap_or("");
    //     let name = v.get("name").and_then(|x| x.as_str()).unwrap_or("");
    //     let ty = v.get("type").and_then(|x| x.as_str()).unwrap_or("");
    //     let content = v.get("content").and_then(|x| x.as_str()).unwrap_or("");
    //     println!("{id} {name} {ty} {content}");
    // }
    //
    // let tmp = records.result_info.get("count").unwrap_or(&Value::Null);
    // println!("count: {}", tmp);
    //
    // let temp: Vec<ARecord> = CloudflareProvider::result_into(records)?;
    // println!("第一個 A 記錄：{:#?}", temp);

    let token = std::env::var("CF_API_TOKEN").expect("環境變數 CF_API_TOKEN 未設置");
    let zone_id = std::env::var("CF_ZONE_ID").expect("環境變數 CF_ZONE_ID 未設置");
    let mut provider = CloudflareProvider::new(token, None, None);
    provider.set_zone_id(zone_id);
    let records = provider.list_records().await?;
    println!("可用的 DNS 記錄列表：");
    for (id, name, content) in records {
        println!("ID: {id}, Name: {name}, Content: {content}");
    }
    let record_id = provider.get_record_id("home.duacodie.com").await?;
    match record_id {
        Some(id) => {
            println!("Record ID for 'duacodie.com': {id}");
        }
        None => {
            println!("未找到 'duacodie.com' 的記錄 ID");
        }
    }
    Ok(())
}
