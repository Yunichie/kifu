use domain::{
    stats,
    types::{
        CareerStats, GameDetail, GameListItem, GameListPage, GameListPlayer, PlayerSearchPage,
        Ruleset,
    },
};
use serde::Deserialize;
use worker::{D1Database, D1Result, D1Type, Error, Result};

pub const PAGE_SIZE: u32 = 20;

#[derive(Deserialize)]
struct DetailRow {
    detail_json: String,
}

#[derive(Deserialize)]
struct ExistsRow {
    present: i32,
}

#[derive(Deserialize)]
struct ListRow {
    log_id: String,
    added_at: f64,
    ruleset_json: String,
    seat: Option<i32>,
    player_name: Option<String>,
    final_score: Option<i32>,
    placement: Option<i32>,
}

#[derive(Deserialize)]
struct PlayerNameRow {
    player_name: String,
}

#[derive(Default, Deserialize)]
struct CareerAggregateRow {
    games: i32,
    hands: i32,
    wins: i32,
    riichi: i32,
    deal_ins: i32,
    calls: i32,
    average_placement: Option<f64>,
}

pub async fn find(db: &D1Database, log_id: &str) -> Result<Option<GameDetail>> {
    let args = [D1Type::Text(log_id)];
    let row = db
        .prepare("SELECT detail_json FROM games WHERE log_id = ?1 LIMIT 1")
        .bind_refs(&args)?
        .first::<DetailRow>(None)
        .await?;

    row.map(|row| deserialize(&row.detail_json)).transpose()
}

pub async fn exists(db: &D1Database, log_id: &str) -> Result<bool> {
    let args = [D1Type::Text(log_id)];
    Ok(db
        .prepare("SELECT 1 AS present FROM games WHERE log_id = ?1 LIMIT 1")
        .bind_refs(&args)?
        .first::<ExistsRow>(None)
        .await?
        .is_some_and(|row| row.present == 1))
}

pub async fn list_all(db: &D1Database, page: u32) -> Result<GameListPage> {
    let (limit, offset) = page_values(page);
    let args = [D1Type::Integer(limit), D1Type::Integer(offset)];
    let result = db
        .prepare(
            "WITH game_page AS ( \
                 SELECT id, log_id, added_at, ruleset_json FROM games \
                 ORDER BY added_at DESC, id DESC LIMIT ?1 OFFSET ?2 \
             ) \
             SELECT g.log_id, g.added_at, g.ruleset_json, gp.seat, \
                    gp.player_name, gp.final_score, gp.placement \
             FROM game_page g \
             LEFT JOIN game_players gp ON gp.game_id = g.id \
             ORDER BY g.added_at DESC, g.id DESC, gp.seat ASC",
        )
        .bind_refs(&args)?
        .all()
        .await?;
    game_page(result, page)
}

pub async fn list_saved(db: &D1Database, user_id: i32, page: u32) -> Result<GameListPage> {
    let (limit, offset) = page_values(page);
    let args = [
        D1Type::Integer(user_id),
        D1Type::Integer(limit),
        D1Type::Integer(offset),
    ];
    let result = db
        .prepare(
            "WITH game_page AS ( \
                 SELECT g.id, g.log_id, g.added_at, g.ruleset_json, usg.saved_at \
                 FROM user_saved_games usg JOIN games g ON g.id = usg.game_id \
                 WHERE usg.user_id = ?1 \
                 ORDER BY usg.saved_at DESC, g.id DESC LIMIT ?2 OFFSET ?3 \
             ) \
             SELECT g.log_id, g.added_at, g.ruleset_json, gp.seat, \
                    gp.player_name, gp.final_score, gp.placement \
             FROM game_page g \
             LEFT JOIN game_players gp ON gp.game_id = g.id \
             ORDER BY g.saved_at DESC, g.id DESC, gp.seat ASC",
        )
        .bind_refs(&args)?
        .all()
        .await?;
    game_page(result, page)
}

pub async fn search_players(db: &D1Database, query: &str, page: u32) -> Result<PlayerSearchPage> {
    if query.is_empty() {
        return Ok(PlayerSearchPage {
            items: Vec::new(),
            page,
            has_next: false,
        });
    }

    let (limit, offset) = page_values(page);
    let pattern = like_prefix(query);
    let args = [
        D1Type::Text(&pattern),
        D1Type::Integer(limit),
        D1Type::Integer(offset),
    ];
    let result = db
        .prepare(
            "SELECT DISTINCT player_name FROM game_players \
             WHERE player_name LIKE ?1 ESCAPE '\\' COLLATE NOCASE \
             ORDER BY player_name COLLATE NOCASE, player_name \
             LIMIT ?2 OFFSET ?3",
        )
        .bind_refs(&args)?
        .all()
        .await?;
    let mut items = result
        .results::<PlayerNameRow>()?
        .into_iter()
        .map(|row| row.player_name)
        .collect::<Vec<_>>();
    let has_next = items.len() > PAGE_SIZE as usize;
    items.truncate(PAGE_SIZE as usize);
    Ok(PlayerSearchPage {
        items,
        page,
        has_next,
    })
}

pub async fn persist_and_save(
    db: &D1Database,
    user_id: i32,
    game: &GameDetail,
    now_millis: u64,
) -> Result<()> {
    let ruleset_json = serde_json::to_string(&game.rules).map_err(data_error)?;
    let detail_json = serde_json::to_string(game).map_err(data_error)?;
    let game_args = [
        D1Type::Text(&game.log_id),
        D1Type::Real(now_millis as f64),
        D1Type::Text(&ruleset_json),
        D1Type::Text(&detail_json),
    ];
    let mut statements = vec![
        db.prepare(
            "INSERT OR IGNORE INTO games (log_id, added_at, ruleset_json, detail_json) \
             VALUES (?1, CAST(?2 AS INTEGER), ?3, ?4)",
        )
        .bind_refs(&game_args)?,
    ];

    for player in &game.players {
        let player_args = [
            D1Type::Text(&game.log_id),
            D1Type::Integer(i32::from(player.seat)),
            D1Type::Text(&player.name),
            optional_integer(player.final_score),
            optional_integer(player.placement.map(i32::from)),
            D1Type::Integer(as_i32(player.stats.wins)?),
            D1Type::Integer(as_i32(player.stats.riichi)?),
            D1Type::Integer(as_i32(player.stats.deal_ins)?),
            D1Type::Integer(as_i32(player.stats.calls.total)?),
            D1Type::Integer(as_i32(player.stats.hands)?),
        ];
        statements.push(
            db.prepare(
                "INSERT OR IGNORE INTO game_players \
                 (game_id, seat, player_name, final_score, placement, wins, riichi, deal_ins, calls, hands) \
                 SELECT id, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10 \
                 FROM games WHERE log_id = ?1",
            )
            .bind_refs(&player_args)?,
        );
    }

    let saved_args = [
        D1Type::Integer(user_id),
        D1Type::Text(&game.log_id),
        D1Type::Real(now_millis as f64),
    ];
    statements.push(
        db.prepare(
            "INSERT OR IGNORE INTO user_saved_games (user_id, game_id, saved_at) \
             SELECT ?1, id, CAST(?3 AS INTEGER) FROM games WHERE log_id = ?2",
        )
        .bind_refs(&saved_args)?,
    );

    db.batch(statements).await?;
    Ok(())
}

pub async fn save_for_user(
    db: &D1Database,
    user_id: i32,
    log_id: &str,
    now_millis: u64,
) -> Result<()> {
    let args = [
        D1Type::Integer(user_id),
        D1Type::Text(log_id),
        D1Type::Real(now_millis as f64),
    ];
    db.prepare(
        "INSERT OR IGNORE INTO user_saved_games (user_id, game_id, saved_at) \
         SELECT ?1, id, CAST(?3 AS INTEGER) FROM games WHERE log_id = ?2",
    )
    .bind_refs(&args)?
    .run()
    .await?;
    Ok(())
}

pub async fn remove_saved(db: &D1Database, user_id: i32, log_id: &str) -> Result<()> {
    let args = [D1Type::Integer(user_id), D1Type::Text(log_id)];
    db.prepare(
        "DELETE FROM user_saved_games \
         WHERE user_id = ?1 AND game_id = (SELECT id FROM games WHERE log_id = ?2)",
    )
    .bind_refs(&args)?
    .run()
    .await?;
    Ok(())
}

pub async fn career(db: &D1Database, player_names: &[String]) -> Result<CareerStats> {
    if player_names.is_empty() {
        return Ok(stats::aggregate_career(&[], player_names));
    }

    let placeholders = (1..=player_names.len())
        .map(|index| format!("?{index}"))
        .collect::<Vec<_>>()
        .join(", ");
    let args = player_names
        .iter()
        .map(|name| D1Type::Text(name))
        .collect::<Vec<_>>();
    let aggregate = db
        .prepare(format!(
            "SELECT COUNT(DISTINCT game_id) AS games, \
                    COALESCE(SUM(hands), 0) AS hands, \
                    COALESCE(SUM(wins), 0) AS wins, \
                    COALESCE(SUM(riichi), 0) AS riichi, \
                    COALESCE(SUM(deal_ins), 0) AS deal_ins, \
                    COALESCE(SUM(calls), 0) AS calls, \
                    AVG(placement) AS average_placement \
             FROM game_players WHERE player_name IN ({placeholders})"
        ))
        .bind_refs(&args)?
        .first::<CareerAggregateRow>(None)
        .await?
        .unwrap_or_default();
    let detail_rows = db
        .prepare(format!(
            "SELECT g.detail_json FROM games g \
             WHERE EXISTS ( \
                 SELECT 1 FROM game_players gp \
                 WHERE gp.game_id = g.id AND gp.player_name IN ({placeholders}) \
             ) ORDER BY g.added_at ASC"
        ))
        .bind_refs(&args)?
        .all()
        .await?
        .results::<DetailRow>()?;
    let details = detail_rows
        .into_iter()
        .map(|row| deserialize(&row.detail_json))
        .collect::<Result<Vec<GameDetail>>>()?;
    let mut career = stats::aggregate_career(&details, player_names);

    career.games = nonnegative(aggregate.games);
    career.average_placement = aggregate.average_placement;
    career.stats.hands = nonnegative(aggregate.hands);
    career.stats.wins = nonnegative(aggregate.wins);
    career.stats.riichi = nonnegative(aggregate.riichi);
    career.stats.deal_ins = nonnegative(aggregate.deal_ins);
    career.stats.calls.total = nonnegative(aggregate.calls);
    Ok(career)
}

fn list_items(result: D1Result) -> Result<Vec<GameListItem>> {
    let mut games: Vec<GameListItem> = Vec::new();
    for row in result.results::<ListRow>()? {
        if games.last().is_none_or(|game| game.log_id != row.log_id) {
            games.push(GameListItem {
                log_id: row.log_id.clone(),
                added_at: row.added_at,
                rules: deserialize::<Ruleset>(&row.ruleset_json)?,
                players: Vec::new(),
            });
        }

        if let (Some(seat), Some(name)) = (row.seat, row.player_name) {
            games
                .last_mut()
                .expect("game was just inserted")
                .players
                .push(GameListPlayer {
                    seat: u8::try_from(seat).map_err(data_error)?,
                    name,
                    final_score: row.final_score,
                    placement: row
                        .placement
                        .map(u8::try_from)
                        .transpose()
                        .map_err(data_error)?,
                });
        }
    }
    Ok(games)
}

fn game_page(result: D1Result, page: u32) -> Result<GameListPage> {
    let mut items = list_items(result)?;
    let has_next = items.len() > PAGE_SIZE as usize;
    items.truncate(PAGE_SIZE as usize);
    Ok(GameListPage {
        items,
        page,
        has_next,
    })
}

fn page_values(page: u32) -> (i32, i32) {
    let offset = (page.saturating_sub(1) * PAGE_SIZE) as i32;
    ((PAGE_SIZE + 1) as i32, offset)
}

fn like_prefix(query: &str) -> String {
    let mut pattern = String::with_capacity(query.len() + 1);
    for character in query.chars() {
        if matches!(character, '\\' | '%' | '_') {
            pattern.push('\\');
        }
        pattern.push(character);
    }
    pattern.push('%');
    pattern
}

fn deserialize<T: serde::de::DeserializeOwned>(json: &str) -> Result<T> {
    serde_json::from_str(json).map_err(data_error)
}

fn optional_integer(value: Option<i32>) -> D1Type<'static> {
    value.map(D1Type::Integer).unwrap_or(D1Type::Null)
}

fn as_i32(value: u32) -> Result<i32> {
    i32::try_from(value).map_err(data_error)
}

fn nonnegative(value: i32) -> u32 {
    value.max(0) as u32
}

fn data_error(error: impl std::fmt::Display) -> Error {
    Error::RustError(format!("invalid game data: {error}"))
}

#[cfg(test)]
mod tests {
    use super::like_prefix;

    #[test]
    fn escapes_like_metacharacters_in_player_prefixes() {
        assert_eq!(like_prefix("a%b_c\\d"), "a\\%b\\_c\\\\d%");
        assert_eq!(like_prefix("\u{77f3}\u{6a4b}"), "\u{77f3}\u{6a4b}%");
    }
}
