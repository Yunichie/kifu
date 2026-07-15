pub mod error;
pub mod mjlog;
pub mod stats;
pub mod types;

pub use error::{DomainError, Result};
pub use types::GameDetail;

pub const SCAFFOLD_RESPONSE: &str = "kifu api scaffold";

pub fn parse_game(log_id: impl Into<String>, xml: &str) -> Result<GameDetail> {
    let parsed = mjlog::parser::parse_mjlog(xml)?;
    Ok(stats::summarize_game(log_id.into(), parsed))
}
