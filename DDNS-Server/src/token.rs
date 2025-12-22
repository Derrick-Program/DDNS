use anyhow::{anyhow, Result};
use base64::{engine::general_purpose::URL_SAFE_NO_PAD as B64, Engine};
use chrono::{DateTime, Duration, Utc};
use hmac::{Hmac, Mac};
use serde::{Deserialize, Serialize};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;
#[derive(Debug, Serialize, Deserialize)]
pub struct TokenPayload {
    sub:   String,
    exp:   i64,
    nonce: String,
}
#[derive(Debug, Default)]
pub struct TokenService {
    secret: Vec<u8>,
    ttl:    Duration,
}

impl TokenService {
    pub fn new(secret: impl AsRef<[u8]>, ttl: Duration) -> Self {
        Self { secret: secret.as_ref().to_vec(), ttl }
    }
    pub fn issue(&self, user_id: &str) -> Result<(String, DateTime<Utc>)> {
        let exp = Utc::now() + self.ttl;
        let payload = TokenPayload {
            sub:   user_id.to_string(),
            exp:   exp.timestamp(),
            nonce: uuid::Uuid::new_v4().to_string(),
        };
        let payload_json = serde_json::to_vec(&payload)?;
        let payload_b64 = B64.encode(payload_json);
        let mut mac = HmacSha256::new_from_slice(&self.secret)?;
        mac.update(payload_b64.as_bytes());
        let sig = mac.finalize().into_bytes();
        let sig_b64 = B64.encode(sig);
        Ok((format!("{payload_b64}.{sig_b64}"), exp))
    }
    pub fn verify(&self, token: &str) -> Result<TokenPayload> {
        let (p_b64, sig_b64) = token.split_once('.').ok_or_else(|| anyhow!("bad format"))?;
        let sig = B64.decode(sig_b64)?;
        let mut mac = HmacSha256::new_from_slice(&self.secret)?;
        mac.update(p_b64.as_bytes());
        mac.verify_slice(&sig).map_err(|_| anyhow!("bad signature"))?;

        let bytes = B64.decode(p_b64)?;
        let payload: TokenPayload = serde_json::from_slice(&bytes)?;
        if payload.exp < Utc::now().timestamp() {
            return Err(anyhow!("expired"));
        }
        Ok(payload)
    }
}
