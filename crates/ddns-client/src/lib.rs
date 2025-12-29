use std::net::IpAddr;
use anyhow::Result;

pub async fn get_public_ip() -> Result<IpAddr> {
    let result = public_ip_address::perform_lookup(None).await?;
    Ok(result.ip)
}