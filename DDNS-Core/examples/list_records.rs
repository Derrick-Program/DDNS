use anyhow::Result;
use ddns_core::provider::*;
#[tokio::main]
async fn main() -> Result<()> {
    // 確保已設置環境變數 CF_API_TOKEN
    let token = std::env::var("CF_API_TOKEN").expect("環境變數 CF_API_TOKEN 未設置");
    let zone_id = std::env::var("CF_ZONE_ID").ok();
    let domain_id = std::env::var("CF_DOMAIN_ID").ok();
    // 創建 CloudflareProvider 實例，如果已經有zone_id或是domain_id，可以在這裡設置
    let mut provider = CloudflareProvider::new(token, zone_id, domain_id);
    let cf = provider.send_value(RequestMethod::Get, ApiPath::Raw("/zones"), None, None).await?;
    let first_zone_id = CloudflareProvider::result_at(&cf, "0/id")
        .and_then(|v| v.as_str())
        .ok_or_else(|| anyhow::anyhow!("missing zone id"))?;
    println!("第一個 Zone ID: {}", first_zone_id);
    provider.set_zone_id(first_zone_id);
    let records =
        provider.send_value(RequestMethod::Get, ApiPath::Dns, Some(&[("type", "A")]), None).await?;

    let arr = CloudflareProvider::result_as_array(&records)?;
    println!("A 記錄清單：");
    for v in arr {
        let id = v.get("id").and_then(|x| x.as_str()).unwrap_or("");
        let name = v.get("name").and_then(|x| x.as_str()).unwrap_or("");
        let ty = v.get("type").and_then(|x| x.as_str()).unwrap_or("");
        let content = v.get("content").and_then(|x| x.as_str()).unwrap_or("");
        println!("{id} {name} {ty} {content}");
    }

    let temp: Vec<ARecord> = CloudflareProvider::result_into(records)?;
    println!("第一個 A 記錄：{:#?}", temp);

    Ok(())
}
