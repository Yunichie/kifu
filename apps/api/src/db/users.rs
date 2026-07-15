use domain::types::User;
use serde::Deserialize;
use worker::{D1Database, D1Type, Result};

#[derive(Deserialize)]
pub struct StoredUser {
    pub id: i32,
    pub username: String,
    pub password_hash: String,
}

#[derive(Deserialize)]
struct PlayerNameRow {
    player_name: String,
}

pub async fn create_user(
    db: &D1Database,
    username: &str,
    password_hash: &str,
    created_at: u64,
) -> Result<Option<User>> {
    let args = [
        D1Type::Text(username),
        D1Type::Text(password_hash),
        D1Type::Real(created_at as f64),
    ];
    db.prepare(
        "INSERT OR IGNORE INTO users (username, password_hash, created_at) \
         VALUES (?1, ?2, CAST(?3 AS INTEGER)) RETURNING id, username",
    )
    .bind_refs(&args)?
    .first(None)
    .await
}

pub async fn find_by_username(db: &D1Database, username: &str) -> Result<Option<StoredUser>> {
    let args = [D1Type::Text(username)];
    db.prepare("SELECT id, username, password_hash FROM users WHERE username = ?1 LIMIT 1")
        .bind_refs(&args)?
        .first(None)
        .await
}

pub async fn find_by_id(db: &D1Database, user_id: i32) -> Result<Option<User>> {
    let args = [D1Type::Integer(user_id)];
    db.prepare("SELECT id, username FROM users WHERE id = ?1 LIMIT 1")
        .bind_refs(&args)?
        .first(None)
        .await
}

pub async fn list_player_names(db: &D1Database, user_id: i32) -> Result<Vec<String>> {
    let args = [D1Type::Integer(user_id)];
    let result = db
        .prepare(
            "SELECT player_name FROM user_player_names \
             WHERE user_id = ?1 ORDER BY claimed_at, player_name",
        )
        .bind_refs(&args)?
        .all()
        .await?;

    Ok(result
        .results::<PlayerNameRow>()?
        .into_iter()
        .map(|row| row.player_name)
        .collect())
}

pub async fn add_player_name(
    db: &D1Database,
    user_id: i32,
    player_name: &str,
    claimed_at: u64,
) -> Result<()> {
    let args = [
        D1Type::Integer(user_id),
        D1Type::Text(player_name),
        D1Type::Real(claimed_at as f64),
    ];
    db.prepare(
        "INSERT OR IGNORE INTO user_player_names (user_id, player_name, claimed_at) \
         VALUES (?1, ?2, CAST(?3 AS INTEGER))",
    )
    .bind_refs(&args)?
    .run()
    .await?;
    Ok(())
}

pub async fn remove_player_name(db: &D1Database, user_id: i32, player_name: &str) -> Result<()> {
    let args = [D1Type::Integer(user_id), D1Type::Text(player_name)];
    db.prepare("DELETE FROM user_player_names WHERE user_id = ?1 AND player_name = ?2")
        .bind_refs(&args)?
        .run()
        .await?;
    Ok(())
}
