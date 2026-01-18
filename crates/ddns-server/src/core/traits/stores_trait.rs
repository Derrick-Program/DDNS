use std::{collections::HashMap, fmt, net::IpAddr, sync::Arc};

use chrono::{DateTime, Utc};
use ddns_shared::error::StoreError;
use uuid::Uuid;
use zeroize::Zeroize;

pub type StoresResult<T> = std::result::Result<T, StoreError>;
#[async_trait::async_trait]
pub trait Store: Send + Sync {
    async fn get_all_users(&self) -> StoresResult<Vec<String>>;
    async fn get_info_with_user(&self, user: &str) -> StoresResult<UserInfo>;
    async fn get_all_devices(&self) -> StoresResult<HashMap<String, Vec<(Uuid, String)>>>; // HashMap<user, Vec<(device_uuid, device_name)>>
    async fn get_devices_with_user(&self, user: &str) -> StoresResult<Vec<String>>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UserId(pub Uuid);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DeviceId(pub Uuid);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RecordId(pub Uuid);

pub struct TokenHash([u8; 32]);

impl TokenHash {
    pub fn from_bytes(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

impl Drop for TokenHash {
    fn drop(&mut self) {
        self.0.zeroize();
    }
}

impl fmt::Debug for TokenHash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "TokenHash([REDACTED; 32])")
    }
}

#[derive(Debug, Clone)]
pub struct UserInfo {
    pub id: UserId,
    pub username: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct DeviceInfo {
    pub id: DeviceId,
    pub user_id: UserId,
    pub device_name: Option<String>,
    pub created_at: DateTime<Utc>,
    pub last_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone)]
pub struct RefreshToken {
    pub id: i64,
    pub user_id: UserId,
    pub device_id: DeviceId,
    pub token_hash: Arc<TokenHash>,
    pub issued_at: DateTime<Utc>,
    pub expired_at: Option<DateTime<Utc>>,
    pub revoked_at: Option<DateTime<Utc>>,
    pub revoked_reason: Option<String>,
    pub last_used_at: Option<DateTime<Utc>>,
    pub last_ip: Option<String>,
    pub user_agent: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecordType {
    A,
    AAAA,
}

#[derive(Debug, Clone)]
pub struct RecordInfo {
    pub id: RecordId,
    pub user_id: UserId,
    pub providers_record_id: String,
    pub domain: String,
    pub record_type: RecordType,
    pub proxied: Option<bool>,
    pub ttl: u32,
    pub allow_update: bool,
    pub last_content: Option<String>,
    pub last_updated_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DetectType {
    Ifconfig,
    Stun,
    Http,
    CloudflareTrace,
}

#[derive(Debug, Clone)]
pub struct RecordUpdateInfo {
    pub id: i64,
    pub record_id: RecordId,
    pub user_id: UserId,
    pub old_content: Option<String>,
    pub new_content: String,
    pub detected_by: DetectType,
    pub source_ip: Option<IpAddr>,
    pub user_agent: Option<String>,
    pub providers_request_id: Option<String>,
    pub success: bool,
    pub error_code: Option<String>,
    pub error_message: Option<String>,
    pub created_at: DateTime<Utc>,
}
