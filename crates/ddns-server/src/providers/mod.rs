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

type ProvidersResult<T> = std::result::Result<T, DnsError>;

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

pub struct DdnsService<P>
where
    P: Provider,
{
    provider: P,
    zone: ZoneName,
    record_name: RecordName,
    ttl: u32,
}

impl<P: Provider> DdnsService<P> {
    pub fn new(provider: P, zone: ZoneName, record_name: RecordName, ttl: u32) -> Self {
        Self { provider, zone, record_name, ttl }
    }

    pub async fn new_record(
        &self,
        content: RecordContent,
        proxied: Option<bool>,
    ) -> ProvidersResult<EnsureResult> {
        let desired_record = DnsRecord {
            name: self.record_name.clone(),
            zone: self.zone.clone(),
            content,
            ttl: self.ttl,
            proxied,
        };
        self.provider.create_record(&self.zone, &desired_record).await
    }

    pub async fn ensure_record(
        &self,
        content: RecordContent,
        proxied: Option<bool>,
    ) -> ProvidersResult<bool> {
        let records = self.provider.list_record(&self.zone).await?;
        let existing_record = records.iter().find(|r| r.name == self.record_name);

        let desired_record = DnsRecord {
            name: self.record_name.clone(),
            zone: self.zone.clone(),
            content,
            ttl: self.ttl,
            proxied,
        };

        match existing_record {
            Some(record) => {
                if record.content != desired_record.content
                    || record.ttl != desired_record.ttl
                    || record.proxied != desired_record.proxied
                {
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
            // None => self.provider.create_record(&self.zone, &desired_record).await,
            None => Err(DnsError::NotFound),
        }
    }

    pub async fn update_ipv4(&self, ip: std::net::Ipv4Addr) -> ProvidersResult<bool> {
        self.ensure_record(RecordContent::A(ip), None).await
        // self.provider.update_record(&self.zone, &desired_record).await
    }
}
