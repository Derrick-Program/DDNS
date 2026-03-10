
use crate::core::domains::providers_domain::*;

#[async_trait::async_trait]
pub trait Provider: Send + Sync {
    fn name(&self) -> &'static str;
    async fn list_record(&self, zone: &ZoneName) -> ProvidersResult<Vec<DnsRecord>>;
    async fn create_record(
        &self,
        zone: &ZoneName,
        record: &DnsRecord,
    ) -> ProvidersResult<EnsureResult>;
    async fn update_record(
        &self,
        zone: &ZoneName,
        record: &DnsRecord,
    ) -> ProvidersResult<EnsureResult>;
    async fn delete_record(&self, zone: &ZoneName, record: &DnsRecord) -> ProvidersResult<bool>;
}


