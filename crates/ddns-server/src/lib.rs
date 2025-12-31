// mod parser;
// mod token;
mod cli;
mod store;
mod errors;
use errors::DdnsServerError;
pub use cli::cli;
pub type Result<T> = std::result::Result<T, DdnsServerError>;
