#![allow(unused)]
pub mod generated;
pub use generated::*;
pub mod error;
pub type Result<T> = std::result::Result<T, error::Error>;

pub async fn get_public_ip() -> Result<std::net::IpAddr> {
    let result = public_ip_address::perform_lookup(None).await?;
    Ok(result.ip)
}
