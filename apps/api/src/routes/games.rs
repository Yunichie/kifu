use axum::{
    Json, Router,
    extract::{Path, State, rejection::JsonRejection},
    http::StatusCode,
    routing::get,
};
use domain::types::{AddGameInput, GameDetail, GameListItem};

use crate::{
    auth::middleware::AuthedUser,
    db::games,
    error::ApiError,
    state::{AppState, now_millis},
    tenhou_fetch,
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/games", get(list).post(add))
        .route("/api/games/{id}", get(detail).delete(remove))
}

#[worker::send]
async fn list(State(state): State<AppState>) -> Result<Json<Vec<GameListItem>>, ApiError> {
    Ok(Json(
        games::list_all(state.db())
            .await
            .map_err(ApiError::internal)?,
    ))
}

#[worker::send]
async fn add(
    State(state): State<AppState>,
    user: AuthedUser,
    body: Result<Json<AddGameInput>, JsonRejection>,
) -> Result<Json<GameDetail>, ApiError> {
    let Json(input) = body.map_err(|_| ApiError::bad_request("invalid JSON body"))?;
    let log_id = tenhou_fetch::canonical_log_id(&input.log_id)
        .map_err(|_| ApiError::bad_request("invalid Tenhou log ID or URL"))?;

    if let Some(game) = games::find(state.db(), &log_id)
        .await
        .map_err(ApiError::internal)?
    {
        games::save_for_user(state.db(), user.id, &log_id, now_millis())
            .await
            .map_err(ApiError::internal)?;
        return Ok(Json(game));
    }

    let fetched = tenhou_fetch::fetch_via_queue(state.tenhou_queue(), &log_id)
        .await
        .map_err(ApiError::bad_gateway)?;
    let game = match fetched {
        tenhou_fetch::QueueFetch::Cached => {
            let game = games::find(state.db(), &log_id)
                .await
                .map_err(ApiError::internal)?
                .ok_or(ApiError::Internal)?;
            games::save_for_user(state.db(), user.id, &log_id, now_millis())
                .await
                .map_err(ApiError::internal)?;
            game
        }
        tenhou_fetch::QueueFetch::Fetched(xml) => {
            let game = match domain::parse_game(&log_id, &xml) {
                Ok(game) => game,
                Err(error) => {
                    finish_queue_fetch(&state, &log_id).await;
                    return Err(ApiError::unprocessable(error));
                }
            };
            if let Err(error) =
                games::persist_and_save(state.db(), user.id, &game, now_millis()).await
            {
                finish_queue_fetch(&state, &log_id).await;
                return Err(ApiError::internal(error));
            }
            finish_queue_fetch(&state, &log_id).await;
            game
        }
    };
    Ok(Json(game))
}

async fn finish_queue_fetch(state: &AppState, log_id: &str) {
    if let Err(error) = tenhou_fetch::complete_queue_fetch(state.tenhou_queue(), log_id).await {
        worker::console_error!("failed to release Tenhou fetch lease for {log_id}: {error}");
    }
}

#[worker::send]
async fn detail(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<GameDetail>, ApiError> {
    let log_id = exact_log_id(&id)?;
    let game = games::find(state.db(), &log_id)
        .await
        .map_err(ApiError::internal)?
        .ok_or_else(|| ApiError::not_found("game not found"))?;
    Ok(Json(game))
}

#[worker::send]
async fn remove(
    State(state): State<AppState>,
    user: AuthedUser,
    Path(id): Path<String>,
) -> Result<StatusCode, ApiError> {
    let log_id = exact_log_id(&id)?;
    games::remove_saved(state.db(), user.id, &log_id)
        .await
        .map_err(ApiError::internal)?;
    Ok(StatusCode::NO_CONTENT)
}

fn exact_log_id(value: &str) -> Result<String, ApiError> {
    let canonical = tenhou_fetch::canonical_log_id(value)
        .map_err(|_| ApiError::bad_request("invalid Tenhou log ID"))?;
    if canonical != value {
        return Err(ApiError::bad_request("invalid Tenhou log ID"));
    }
    Ok(canonical)
}
