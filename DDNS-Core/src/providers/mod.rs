use anyhow::Result;
#[cfg(feature = "provider-cloudflare")]
pub mod cloudflare;

#[tonic::async_trait]
pub trait DnsProvider {
    fn provider_name(&self) -> &str;
    // fn get_token(&self) -> &str;
    // fn get_email(&self) -> &str;
    async fn list_zone(&self) -> Result<Vec<(String, String)>>;
    async fn get_zone_id(&self, zone_name: &str) -> Result<Option<String>>;
    async fn list_records(&self) -> Result<Vec<(String, String, String)>>;
    async fn get_record_id(&self, record_name: &str) -> Result<Option<String>>;

    async fn create_record(
        &self,
        record_name: &str,
        record_ttl: Option<u32>,
        record_type: &str,
        record_comment: Option<&str>,
        record_content: &str,
        proxied: bool,
    ) -> Result<String>;
    async fn update_record(
        &self,
        identifier: RecordIdentifier<'_>,
        record_content: &str,
    ) -> Result<String>;
    async fn delete_record(&self, identifier: RecordIdentifier<'_>) -> Result<String>;
}

#[derive(Debug)]
pub enum RecordIdentifier<'a> {
    Id(&'a str),
    Name(&'a str),
}
