use anyhow::Result;
#[tokio::main]
async fn main() -> Result<()> {
    // cli().await?;
    ddns_server::entry()?;
    Ok(())
}
