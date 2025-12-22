use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use std::{fmt, net::IpAddr};

use crate::{DnsProvider, providers::RecordIdentifier};
use reqwest::{Client, Url};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value, json};

#[derive(Debug, Serialize, Deserialize)]
pub struct ARecord {
    pub id:          String,
    pub name:        String,
    #[serde(rename = "type")]
    pub r#type:      String,
    pub content:     String,
    pub proxiable:   bool,
    pub proxied:     bool,
    pub ttl:         u32,
    #[serde(default)]
    pub settings:    Value,
    #[serde(default)]
    pub meta:        Value,
    pub comment:     Option<String>,
    #[serde(default)]
    pub tags:        Vec<String>,
    pub created_on:  DateTime<Utc>,
    pub modified_on: DateTime<Utc>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CloudflareResponse {
    pub success:     bool,
    pub errors:      Value,
    pub messages:    Value,
    pub result:      Value,
    #[serde(default)]
    pub result_info: Option<Value>,
}
const API_BASE_URL: &str = "https://api.cloudflare.com/client/v4/";
const PROVIDER_NAME: &str = "Cloudflare";
#[derive(Clone, Copy, Debug)]
pub enum RequestMethod {
    Get,
    Post,
    Put,
    Delete,
    Patch,
}

pub enum ApiPath<'a> {
    /// 原始 path，不處理。
    Raw(&'a str),
    /// 會組成 `/zones/{zone_id}/{sub}`；例如 `Zone("settings")`
    Zone(&'a str),
    /// 集合 `/zones/{zone_id}/dns_records`
    Dns,
    /// 單筆 `/zones/{zone_id}/dns_records/{record_id}`；
    /// `None` 則自動讀 `self.domain_id`
    DnsRecord(Option<&'a str>),
}

#[derive(Debug)]
pub struct CloudflareApiError {
    pub status:  reqwest::StatusCode,
    pub success: bool,
    pub errors:  Value,
}
impl fmt::Display for CloudflareApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Cloudflare API error: status={}, success={}, errors={}",
            self.status, self.success, self.errors
        )
    }
}
impl std::error::Error for CloudflareApiError {}
#[derive(Debug, Default)]
pub struct CloudflareProvider {
    token:     String,
    // email:     String,
    domain_id: Option<String>,
    zone_id:   Option<String>,
    client:    Client,
    base_url:  String,
}
impl CloudflareProvider {
    pub fn new(
        token: String,
        // email: String,
        domain_id: Option<String>,
        zone_id: Option<String>,
    ) -> Self {
        let base_url = API_BASE_URL.to_string();
        Self::default()
            .with_token(token)
            // .with_email(email)
            .with_base_url(base_url)
            .with_default_client()
            .with_domain_id(domain_id)
            .with_zone_id(zone_id)
    }
    pub fn with_token(mut self, token: String) -> Self {
        self.token = token;
        self
    }
    // pub fn with_email(mut self, email: String) -> Self {
    //     self.email = email;
    //     self
    // }
    pub fn with_base_url(mut self, base_url: String) -> Self {
        self.base_url = base_url;
        self
    }
    pub fn with_domain_id(mut self, domain_id: Option<String>) -> Self {
        self.domain_id = domain_id;
        self
    }
    pub fn with_zone_id(mut self, zone_id: Option<String>) -> Self {
        self.zone_id = zone_id;
        self
    }
    pub fn with_client(mut self, client: Client) -> Self {
        self.client = client;
        self
    }
    pub fn set_domain_id(&mut self, domain_id: impl Into<String>) {
        self.domain_id = Some(domain_id.into());
    }
    pub fn set_zone_id(&mut self, zone_id: impl Into<String>) {
        self.zone_id = Some(zone_id.into());
    }
    pub fn clear_domain_id(&mut self) {
        self.domain_id = None;
    }
    pub fn clear_zone_id(&mut self) {
        self.zone_id = None;
    }
    pub fn with_default_client(mut self) -> Self {
        let mut headers = reqwest::header::HeaderMap::new();
        // If you use global API key, uncomment the following lines
        // headers.insert("X-Auth-Email", self.email.parse().unwrap());
        // headers.insert("X-Auth-Key", self.token.parse().unwrap());
        headers.insert(
            reqwest::header::AUTHORIZATION,
            format!("Bearer {}", self.token).parse().unwrap(),
        );
        headers.insert(reqwest::header::USER_AGENT, "Duacodie-DDNS/1.0".parse().unwrap());
        let c = Client::builder()
            .default_headers(headers)
            .build()
            .expect("Failed to create HTTP client");
        self.with_client(c)
    }
    fn require_zone(&self) -> Result<&str> {
        self.zone_id.as_deref().ok_or_else(|| anyhow::anyhow!("zone_id is not set"))
    }
    fn resolve_record_id<'a>(&'a self, input: Option<&'a str>) -> Result<&'a str> {
        if let Some(id) = input {
            if id.is_empty() {
                anyhow::bail!("record_id provided is empty");
            }
            Ok(id)
        } else {
            self.domain_id
                .as_deref()
                .ok_or_else(|| anyhow::anyhow!("record_id (domain_id) is not set"))
        }
    }
    pub async fn send_value(
        &self,
        method: RequestMethod,
        path: ApiPath<'_>,
        query: Option<&[(&str, &str)]>,
        body: Option<&Value>,
    ) -> Result<CloudflareResponse> {
        let full_path = match path {
            ApiPath::Raw(p) => p.to_string(),
            ApiPath::Zone(sub) => {
                let zone = self.require_zone()?;
                if sub.is_empty() {
                    format!("/zones/{zone}")
                } else {
                    format!("/zones/{}/{}", zone, sub.trim_start_matches('/'))
                }
            }
            ApiPath::Dns => {
                let zone = self.require_zone()?;
                format!("/zones/{zone}/dns_records")
            }
            ApiPath::DnsRecord(maybe_id) => {
                let zone = self.require_zone()?;
                let rid = self.resolve_record_id(maybe_id)?;
                format!("/zones/{zone}/dns_records/{rid}")
            }
        };
        let mut url = Url::parse(&self.base_url)?.join(full_path.trim_start_matches('/'))?;
        if let Some(q) = query {
            url.query_pairs_mut().extend_pairs(q);
        }
        let req = match method {
            RequestMethod::Get => self.client.get(url),
            RequestMethod::Post => self.client.post(url).json(body.unwrap_or(&Value::Null)),
            RequestMethod::Put => self.client.put(url).json(body.unwrap_or(&Value::Null)),
            RequestMethod::Delete => self.client.delete(url),
            RequestMethod::Patch => self.client.patch(url).json(body.unwrap_or(&Value::Null)),
        };
        let res = req.send().await?;
        let status = res.status();
        if !status.is_success() {
            return Err(CloudflareApiError {
                status,
                success: false,
                errors: Value::String(format!("HTTP error: {status}")),
            }
            .into());
        }
        let cf: CloudflareResponse =
            res.json().await.context("Failed to parse Cloudflare JSON response")?;

        if cf.success {
            Ok(cf)
        } else {
            Err(CloudflareApiError { status, success: cf.success, errors: cf.errors.clone() }
                .into())
        }
    }

    /// 直接拿 result 的陣列（如果不是陣列會報錯）
    pub fn result_as_array(cf: &CloudflareResponse) -> Result<&Vec<Value>> {
        cf.result.as_array().ok_or_else(|| anyhow::anyhow!("`result` is not an array"))
    }

    /// 直接拿 result 的物件（如果不是物件會報錯）
    pub fn result_as_object(cf: &CloudflareResponse) -> Result<&Map<String, Value>> {
        cf.result.as_object().ok_or_else(|| anyhow::anyhow!("`result` is not an object"))
    }

    /// 用 JSON Pointer 取值（例如：`/result/0/id`）
    pub fn result_at<'a>(cf: &'a CloudflareResponse, ptr: &str) -> Option<&'a Value> {
        if ptr.starts_with('/') {
            cf.result.pointer(ptr)
        } else {
            cf.result.pointer(&format!("/{ptr}"))
        }
    }

    /// 需要型別安全時，才把動態 JSON 轉為型別 T
    pub fn result_into<T: for<'de> Deserialize<'de>>(cf: CloudflareResponse) -> Result<T> {
        Ok(serde_json::from_value::<T>(cf.result)?)
    }
}

#[tonic::async_trait]
impl DnsProvider for CloudflareProvider {
    fn provider_name(&self) -> &str {
        PROVIDER_NAME
    }

    async fn list_zone(&self) -> Result<Vec<(String, String)>> {
        let zones = self.send_value(RequestMethod::Get, ApiPath::Raw("/zones"), None, None).await?;
        let zones_array = CloudflareProvider::result_as_array(&zones)?;
        let ret: Vec<(String, String)> = zones_array
            .iter()
            .filter_map(|v| {
                let id = v.get("id").and_then(|id| id.as_str());
                let name = v.get("name").and_then(|n| n.as_str());
                match (id, name) {
                    (Some(id), Some(name)) => Some((id.to_string(), name.to_string())),
                    _ => None,
                }
            })
            .collect();
        Ok(ret)
    }

    async fn get_zone_id(&self, zone_name: &str) -> Result<Option<String>> {
        let zones = self.list_zone().await?;
        zones
            .iter()
            .find(|(_, name)| name.eq_ignore_ascii_case(zone_name))
            .map(|(id, _)| id.clone())
            .ok_or_else(|| anyhow::anyhow!("Zone '{}' not found", zone_name))
            .map(Some)
    }

    async fn list_records(&self) -> Result<Vec<(String, String, String)>> {
        let records =
            self.send_value(RequestMethod::Get, ApiPath::Dns, Some(&[("type", "A")]), None).await?;
        let records_array = CloudflareProvider::result_as_array(&records)?;
        let ret: Vec<(String, String, String)> = records_array
            .iter()
            .map(|r| {
                let id = r.get("id").and_then(|id| id.as_str());
                let name = r.get("name").and_then(|n| n.as_str());
                let content = r.get("content").and_then(|c| c.as_str());
                match (id, name, content) {
                    (Some(id), Some(name), Some(content)) => {
                        (id.to_string(), name.to_string(), content.to_string())
                    }
                    _ => ("".to_string(), "".to_string(), "".to_string()),
                }
            })
            .collect();
        Ok(ret)
    }

    async fn get_record_id(&self, record_name: &str) -> Result<Option<String>> {
        let records = self.list_records().await?;
        records
            .iter()
            .find(|(_, name, _)| name.eq_ignore_ascii_case(record_name))
            .map(|(id, _, _)| id.clone())
            .ok_or_else(|| anyhow::anyhow!("Record '{}' not found", record_name))
            .map(Some)
    }

    async fn create_record(
        &self,
        record_name: &str,
        record_ttl: Option<u32>,
        record_type: &str,
        record_comment: Option<&str>,
        record_content: &str,
        proxied: bool,
    ) -> Result<String> {
        let body = serde_json::json!({
            "name": record_name,
            "ttl": record_ttl.unwrap_or(1),
            "type": record_type,
            "comment": record_comment.unwrap_or("Created by Duacodie DDNS client"),
            "content": record_content,
            "proxied": proxied,
        });
        let records = self.send_value(RequestMethod::Post, ApiPath::Dns, None, Some(&body)).await?;
        let records_obj = CloudflareProvider::result_as_object(&records)?;
        let id = records_obj
            .get("id")
            .ok_or_else(|| anyhow::anyhow!("Record '{}' create failed", record_name))?;
        match id {
            Value::String(id) => Ok(id.clone()),
            _ => Err(anyhow::anyhow!("Record ID is not a string")),
        }
    }

    async fn update_record(
        &self,
        identifier: RecordIdentifier<'_>,
        record_content: &str,
    ) -> Result<String> {
        let id = match identifier {
            RecordIdentifier::Id(id) => id.to_string(),
            RecordIdentifier::Name(name) => match self.get_record_id(name).await? {
                Some(id) => id,
                None => return Err(anyhow::anyhow!("Record '{}' not found", name)),
            },
        };
        let body = json!({
            "content": record_content,
        });
        let records = self
            .send_value(
                RequestMethod::Patch,
                ApiPath::DnsRecord(Some(id.as_str())),
                None,
                Some(&body),
            )
            .await?;
        let records_obj = CloudflareProvider::result_as_object(&records)?;
        let id = records_obj
            .get("id")
            .ok_or_else(|| anyhow::anyhow!("Record ID not found in response"))?;
        match id {
            Value::String(id) => Ok(id.clone()),
            _ => Err(anyhow::anyhow!("Record ID is not a string")),
        }
    }

    async fn delete_record(&self, identifier: RecordIdentifier<'_>) -> Result<String> {
        let id = match identifier {
            RecordIdentifier::Id(id) => id.to_string(),
            RecordIdentifier::Name(name) => match self.get_record_id(name).await? {
                Some(id) => id,
                None => return Err(anyhow::anyhow!("Record '{}' not found", name)),
            },
        };
        let res = self
            .send_value(RequestMethod::Delete, ApiPath::DnsRecord(Some(id.as_str())), None, None)
            .await?;
        let records_obj = CloudflareProvider::result_as_object(&res)?;
        let id = records_obj
            .get("id")
            .ok_or_else(|| anyhow::anyhow!("Record ID not found in response"))?;
        match id {
            Value::String(id) => Ok(id.clone()),
            _ => Err(anyhow::anyhow!("Record ID is not a string")),
        }
    }
}
