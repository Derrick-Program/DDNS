use std::net::Ipv4Addr;
use ddns_shared::error::DnsError;

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
