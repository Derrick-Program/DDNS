#![allow(unused)]
pub mod generated;
pub use generated::*;
pub mod error;
pub extern crate tonic;
pub extern crate tonic_prost;
pub extern crate tonic_types;
pub type Result<T> = std::result::Result<T, error::Error>;

pub async fn get_public_ip() -> Result<std::net::IpAddr> {
    let result = public_ip_address::perform_lookup(None).await?;
    Ok(result.ip)
}
