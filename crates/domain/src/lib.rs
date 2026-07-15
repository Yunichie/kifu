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

#[cfg(feature = "bindings")]
pub fn export_all_bindings() -> std::result::Result<(), ts_rs::ExportError> {
    use ts_rs::TS;

    let config = ts_rs::Config::new().with_out_dir(env!("CARGO_MANIFEST_DIR"));

    macro_rules! export {
        ($($type:ty),+ $(,)?) => {
            $(<$type as TS>::export(&config)?;)+
        };
    }

    export!(
        types::User,
        types::AuthCredentials,
        types::AuthResponse,
        types::MeResponse,
        types::PlayerNameInput,
        types::PlayerNamesResponse,
        types::ErrorResponse,
        types::AddGameInput,
        types::GameListItem,
        types::GameListPlayer,
        types::HealthResponse,
        types::GameDetail,
        types::Ruleset,
        types::PlayerSummary,
        types::PlayerStats,
        types::PlayerRates,
        types::CallStats,
        types::BonusStats,
        types::YakuCount,
        types::CountBucket,
        types::Kyoku,
        types::KyokuResult,
        types::WinResult,
        types::DrawReason,
        types::Yaku,
        types::TurnEvent,
        types::CallKind,
        types::DealInMatrix,
        types::CareerStats,
        types::ScoreTrendPoint,
    );

    Ok(())
}
