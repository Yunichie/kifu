use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "bindings", derive(ts_rs::TS))]
#[cfg_attr(feature = "bindings", ts(export_to = "../../packages/api-types/src/"))]
pub struct User {
    pub id: i32,
    pub display_name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "bindings", derive(ts_rs::TS))]
#[cfg_attr(feature = "bindings", ts(export_to = "../../packages/api-types/src/"))]
pub struct OAuthExchangeInput {
    pub code: String,
    pub code_verifier: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "bindings", derive(ts_rs::TS))]
#[cfg_attr(feature = "bindings", ts(export_to = "../../packages/api-types/src/"))]
pub struct AuthResponse {
    pub user: User,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "bindings", derive(ts_rs::TS))]
#[cfg_attr(feature = "bindings", ts(export_to = "../../packages/api-types/src/"))]
pub struct MeResponse {
    pub user: User,
    pub player_names: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "bindings", derive(ts_rs::TS))]
#[cfg_attr(feature = "bindings", ts(export_to = "../../packages/api-types/src/"))]
pub struct PlayerNameInput {
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "bindings", derive(ts_rs::TS))]
#[cfg_attr(feature = "bindings", ts(export_to = "../../packages/api-types/src/"))]
pub struct PlayerNamesResponse {
    pub player_names: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "bindings", derive(ts_rs::TS))]
#[cfg_attr(feature = "bindings", ts(export_to = "../../packages/api-types/src/"))]
pub struct ErrorResponse {
    pub error: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "bindings", derive(ts_rs::TS))]
#[cfg_attr(feature = "bindings", ts(export_to = "../../packages/api-types/src/"))]
pub struct AddGameInput {
    pub log_id: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "bindings", derive(ts_rs::TS))]
#[cfg_attr(feature = "bindings", ts(export_to = "../../packages/api-types/src/"))]
pub struct GameListItem {
    pub log_id: String,
    pub added_at: f64,
    pub rules: Ruleset,
    pub players: Vec<GameListPlayer>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "bindings", derive(ts_rs::TS))]
#[cfg_attr(feature = "bindings", ts(export_to = "../../packages/api-types/src/"))]
pub struct GameListPage {
    pub items: Vec<GameListItem>,
    pub page: u32,
    pub has_next: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "bindings", derive(ts_rs::TS))]
#[cfg_attr(feature = "bindings", ts(export_to = "../../packages/api-types/src/"))]
pub struct PlayerSearchPage {
    pub items: Vec<String>,
    pub page: u32,
    pub has_next: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "bindings", derive(ts_rs::TS))]
#[cfg_attr(feature = "bindings", ts(export_to = "../../packages/api-types/src/"))]
pub struct GameListPlayer {
    pub seat: u8,
    pub name: String,
    pub final_score: Option<i32>,
    pub placement: Option<u8>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "bindings", derive(ts_rs::TS))]
#[cfg_attr(feature = "bindings", ts(export_to = "../../packages/api-types/src/"))]
pub struct HealthResponse {
    pub ok: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "bindings", derive(ts_rs::TS))]
#[cfg_attr(feature = "bindings", ts(export_to = "../../packages/api-types/src/"))]
pub struct GameDetail {
    pub log_id: String,
    pub rules: Ruleset,
    pub players: Vec<PlayerSummary>,
    pub kyoku: Vec<Kyoku>,
    pub deal_in_matrix: DealInMatrix,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "bindings", derive(ts_rs::TS))]
#[cfg_attr(feature = "bindings", ts(export_to = "../../packages/api-types/src/"))]
pub struct Ruleset {
    pub raw_type: u16,
    pub lobby: String,
    pub test: bool,
    pub aka_dora: bool,
    pub kuitan: bool,
    pub hanchan: bool,
    pub sanma: bool,
    pub fast: bool,
    pub table: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "bindings", derive(ts_rs::TS))]
#[cfg_attr(feature = "bindings", ts(export_to = "../../packages/api-types/src/"))]
pub struct PlayerSummary {
    pub seat: u8,
    pub name: String,
    pub rank_id: u8,
    pub rank: String,
    pub rating: f64,
    pub sex: String,
    pub final_score: Option<i32>,
    pub uma: Option<f64>,
    pub placement: Option<u8>,
    pub stats: PlayerStats,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "bindings", derive(ts_rs::TS))]
#[cfg_attr(feature = "bindings", ts(export_to = "../../packages/api-types/src/"))]
pub struct PlayerStats {
    pub hands: u32,
    pub wins: u32,
    pub tsumo_wins: u32,
    pub ron_wins: u32,
    pub deal_ins: u32,
    pub riichi: u32,
    pub dealer_hands: u32,
    pub dealer_repeats: u32,
    pub exhaustive_draws: u32,
    pub tenpai_draws: u32,
    pub noten_draws: u32,
    pub called_hands: u32,
    pub calls: CallStats,
    pub bonuses: BonusStats,
    pub rates: PlayerRates,
    pub yaku_frequency: Vec<YakuCount>,
    pub han_distribution: Vec<CountBucket>,
    pub fu_distribution: Vec<CountBucket>,
    pub hand_value_distribution: Vec<CountBucket>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "bindings", derive(ts_rs::TS))]
#[cfg_attr(feature = "bindings", ts(export_to = "../../packages/api-types/src/"))]
pub struct PlayerRates {
    pub win_rate: f64,
    pub tsumo_win_rate: f64,
    pub ron_win_rate: f64,
    pub tsumo_share: f64,
    pub ron_share: f64,
    pub deal_in_rate: f64,
    pub riichi_rate: f64,
    pub call_rate: f64,
    pub dealer_repeat_rate: f64,
    pub tenpai_rate: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "bindings", derive(ts_rs::TS))]
#[cfg_attr(feature = "bindings", ts(export_to = "../../packages/api-types/src/"))]
pub struct CallStats {
    pub total: u32,
    pub chi: u32,
    pub pon: u32,
    pub open_kan: u32,
    pub closed_kan: u32,
    pub added_kan: u32,
    pub nuki: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "bindings", derive(ts_rs::TS))]
#[cfg_attr(feature = "bindings", ts(export_to = "../../packages/api-types/src/"))]
pub struct BonusStats {
    pub dora: u32,
    pub ura_dora: u32,
    pub aka_dora: u32,
    pub wins_with_dora: u32,
    pub wins_with_ura_dora: u32,
    pub wins_with_aka_dora: u32,
    pub dora_hit_rate: f64,
    pub ura_dora_hit_rate: f64,
    pub aka_dora_hit_rate: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "bindings", derive(ts_rs::TS))]
#[cfg_attr(feature = "bindings", ts(export_to = "../../packages/api-types/src/"))]
pub struct YakuCount {
    pub id: u8,
    pub name: String,
    pub count: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "bindings", derive(ts_rs::TS))]
#[cfg_attr(feature = "bindings", ts(export_to = "../../packages/api-types/src/"))]
pub struct CountBucket {
    pub value: u32,
    pub count: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "bindings", derive(ts_rs::TS))]
#[cfg_attr(feature = "bindings", ts(export_to = "../../packages/api-types/src/"))]
pub struct Kyoku {
    pub round_index: u8,
    pub bakaze: u8,
    pub kyoku_number: u8,
    pub honba: u8,
    pub riichi_sticks: u8,
    pub dealer: u8,
    pub start_scores: Vec<i32>,
    pub end_scores: Vec<i32>,
    pub start_hands: Vec<Vec<u16>>,
    pub dora_indicators: Vec<u16>,
    pub events: Vec<TurnEvent>,
    pub result: KyokuResult,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "type", rename_all = "camelCase")]
#[cfg_attr(feature = "bindings", derive(ts_rs::TS))]
#[cfg_attr(feature = "bindings", ts(export_to = "../../packages/api-types/src/"))]
pub enum KyokuResult {
    Win {
        wins: Vec<WinResult>,
    },
    Draw {
        reason: DrawReason,
        tenpai_seats: Vec<u8>,
        score_deltas: Vec<i32>,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "bindings", derive(ts_rs::TS))]
#[cfg_attr(feature = "bindings", ts(export_to = "../../packages/api-types/src/"))]
pub struct WinResult {
    pub winner: u8,
    pub from_seat: u8,
    pub tsumo: bool,
    pub fu: u16,
    pub han: u8,
    pub points: u32,
    pub limit: u8,
    pub winning_tiles: Vec<u16>,
    pub wait: u16,
    pub yaku: Vec<Yaku>,
    pub dora_indicators: Vec<u16>,
    pub score_deltas: Vec<i32>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "camelCase")]
#[cfg_attr(feature = "bindings", derive(ts_rs::TS))]
#[cfg_attr(feature = "bindings", ts(export_to = "../../packages/api-types/src/"))]
pub enum DrawReason {
    Exhaustive,
    NineTerminals,
    FourRiichi,
    TripleRon,
    FourKans,
    FourWinds,
    NagashiMangan,
    Other { code: String },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "bindings", derive(ts_rs::TS))]
#[cfg_attr(feature = "bindings", ts(export_to = "../../packages/api-types/src/"))]
pub struct Yaku {
    pub id: u8,
    pub name: String,
    pub han: u8,
    pub yakuman: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(tag = "type")]
#[cfg_attr(feature = "bindings", derive(ts_rs::TS))]
#[cfg_attr(feature = "bindings", ts(export_to = "../../packages/api-types/src/"))]
pub enum TurnEvent {
    Draw {
        seat: u8,
        tile: u16,
    },
    Discard {
        seat: u8,
        tile: u16,
        tsumogiri: bool,
    },
    Call {
        seat: u8,
        kind: CallKind,
        tiles: Vec<u16>,
        from_seat: u8,
    },
    Riichi {
        seat: u8,
    },
    NewDora {
        tile: u16,
    },
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "bindings", derive(ts_rs::TS))]
#[cfg_attr(feature = "bindings", ts(export_to = "../../packages/api-types/src/"))]
pub enum CallKind {
    Chi,
    Pon,
    OpenKan,
    ClosedKan,
    AddedKan,
    Nuki,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "bindings", derive(ts_rs::TS))]
#[cfg_attr(feature = "bindings", ts(export_to = "../../packages/api-types/src/"))]
pub struct DealInMatrix {
    pub players: Vec<String>,
    pub counts: Vec<Vec<u32>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CareerGameInput {
    pub log_id: String,
    pub players: Vec<PlayerSummary>,
    pub deal_in_matrix: DealInMatrix,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "bindings", derive(ts_rs::TS))]
#[cfg_attr(feature = "bindings", ts(export_to = "../../packages/api-types/src/"))]
pub struct CareerStats {
    pub player_names: Vec<String>,
    pub games: u32,
    pub average_placement: Option<f64>,
    pub stats: PlayerStats,
    pub placement_distribution: Vec<CountBucket>,
    pub score_trend: Vec<ScoreTrendPoint>,
    pub deal_in_matrix: DealInMatrix,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "bindings", derive(ts_rs::TS))]
#[cfg_attr(feature = "bindings", ts(export_to = "../../packages/api-types/src/"))]
pub struct ScoreTrendPoint {
    pub log_id: String,
    pub player_name: String,
    pub final_score: i32,
    pub placement: u8,
}
