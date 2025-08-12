#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Failed to get public IP address")]
    IpError(#[from] public_ip_address::error::Error),
}
