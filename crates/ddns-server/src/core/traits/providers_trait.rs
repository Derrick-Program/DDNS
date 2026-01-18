use ddns_shared::error::DnsError;
use std::net::Ipv4Addr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordName(pub String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ZoneName(pub String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecordContent {
    A(Ipv4Addr),
    CNAME(String),
}

#[derive(Debug, Clone)]
pub struct DnsRecord {
    pub name: RecordName,
    pub zone: ZoneName,
    pub content: RecordContent,
    pub ttl: u32,
    pub proxied: Option<bool>,
}

#[derive(Debug, Clone)]
pub struct EnsureResult {
    pub changed: bool,
    pub provider_record_id: Option<String>,
}

pub type ProvidersResult<T> = std::result::Result<T, DnsError>;

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


