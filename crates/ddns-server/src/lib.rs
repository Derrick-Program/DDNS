// mod parser;
// mod token;
mod cli;
mod providers;
mod store;
pub use cli::cli;
use ddns_shared::error::DdnsServerError;
pub type Result<T> = std::result::Result<T, DdnsServerError>;
