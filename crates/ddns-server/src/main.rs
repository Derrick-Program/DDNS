use anyhow::Result;
use ddns_server::cli;
#[tokio::main]
async fn main() -> Result<()> {
    cli().await?;
    Ok(())
}