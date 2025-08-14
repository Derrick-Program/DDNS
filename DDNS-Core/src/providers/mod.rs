use anyhow::Result;
#[cfg(feature = "provider-cloudflare")]
pub mod cloudflare;

pub trait DnsProvider {
    fn provider_name(&self) -> &str;
    // fn get_token(&self) -> &str;
    // fn get_email(&self) -> &str;
    fn list_zone(&self) -> Vec<String>;
    fn get_zone_id(&self, zone_name: &str) -> Option<String>;
    fn get_record_id(&self, zone_id: &str, record_name: &str) -> Option<String>;
    fn get_record(&self, zone_id: &str, record_name: &str) -> Option<String>;
    fn create_record(
        &self,
        zone_id: &str,
        record_name: &str,
        record_type: &str,
        record_content: &str,
    ) -> Result<String>;
    fn update_record(
        &self,
        zone_id: &str,
        record_id: &str,
        record_name: &str,
        record_type: &str,
        record_content: &str,
    ) -> Result<String>;
    fn delete_record(&self, zone_id: &str, record_id: &str) -> Result<()>;
}
