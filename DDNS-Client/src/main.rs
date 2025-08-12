use anyhow::{Ok, Result};
use ddns_core::get_public_ip;
#[tokio::main]
async fn main() -> Result<()> {
    let public_ip = get_public_ip().await?;
    println!("Public IP: {}", public_ip);
    Ok(())
}
