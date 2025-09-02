use anyhow::Result;
use ddns_core::{DnsProvider, RecordIdentifier, provider::*};
#[tokio::main]
async fn main() -> Result<()> {
    let token = std::env::var("CF_API_TOKEN").expect("環境變數 CF_API_TOKEN 未設置");
    let zone_id = std::env::var("CF_ZONE_ID").expect("環境變數 CF_ZONE_ID 未設置");
    let provider = CloudflareProvider::new(token, None, Some(zone_id));
    list_records(&provider).await?;
    create_new_record(&provider).await?;
    search_record(&provider, "test.duacodie.com").await?;
    update_record(&provider).await?;
    delete_record(&provider).await?;
    Ok(())
}

async fn create_new_record(provider: &CloudflareProvider) -> Result<()> {
    // Create a new DNS record
    let binding = ddns_core::get_public_ip().await?.to_string();
    let ip = binding.as_str();
    let id = provider.create_record("test.duacodie.com", Some(3600), "A", None, ip, false).await?;
    println!("Created record: {id}");
    Ok(())
}

async fn update_record(provider: &CloudflareProvider) -> Result<()> {
    // Update the record with a new IP address
    let id = provider
        .update_record(RecordIdentifier::Name("test.duacodie.com"), "163.20.165.59")
        .await?;
    println!("Updated record: {id}");
    Ok(())
}
async fn delete_record(provider: &CloudflareProvider) -> Result<()> {
    // Delete the record
    let id = provider.delete_record(RecordIdentifier::Name("test.duacodie.com")).await?;
    println!("Deleted record: {id}");
    Ok(())
}

async fn list_records(provider: &CloudflareProvider) -> Result<()> {
    let records = provider.list_records().await?;
    println!("可用的 DNS 記錄列表：");
    for (id, name, content) in records {
        println!("ID: {id}, Name: {name}, Content: {content}");
    }
    Ok(())
}

async fn search_record(provider: &CloudflareProvider, name: &str) -> Result<Option<String>> {
    // Search for a specific record by name
    let record_id = provider.get_record_id(name).await?;
    match record_id {
        Some(id) => {
            println!("Record ID for '{name}': {id}");
            Ok(Some(id))
        }
        None => {
            println!("未找到 '{name}' 的記錄 ID");
            Ok(None)
        }
    }
}
