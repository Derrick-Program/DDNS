// mod parser;
// mod token;
mod cli;
mod core;
mod providers;
mod store;
mod handlers;
pub use cli::cli;
use ddns_shared::error::DdnsServerError;
pub type Result<T> = std::result::Result<T, DdnsServerError>;

pub fn entry() -> Result<()> {
    println!("ddns-server library loaded.");
    Ok(())
}
