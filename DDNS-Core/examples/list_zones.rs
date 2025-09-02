use ddns_core::DnsProvider;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = std::env::var("CF_API_TOKEN").expect("環境變數 CF_API_TOKEN 未設置");
    let provider = ddns_core::provider::CloudflareProvider::new(token, None, None);
    let zones = provider.list_zone().await?;
    println!("可用的 Zone 列表：");
    for (id, name) in zones {
        println!("ID: {id}, Name: {name}");
    }
    // 如果需要獲取特定 Zone 的 ID，可以使用 get_zone_id 方法
    let zone_name = "example.com"; // 替換為實際的域名
    if let Some(zone_id) = provider.get_zone_id(zone_name).await? {
        println!("Zone ID for '{zone_name}': {zone_id}");
    } else {
        println!("未找到 Zone ID for '{zone_name}'");
    }
    Ok(())
}
