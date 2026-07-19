use axum::{
    Json, Router,
    extract::{Path, Query, State, rejection::JsonRejection},
    http::StatusCode,
    response::Response,
    routing::get,
};
use domain::types::{AddGameInput, GameDetail, GameListPage};
use serde::Deserialize;

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

#[derive(Default, Deserialize)]
struct PaginationQuery {
    page: Option<u32>,
}

#[worker::send]
async fn list(
    State(state): State<AppState>,
    Query(query): Query<PaginationQuery>,
) -> Result<Json<GameListPage>, ApiError> {
    let page = super::valid_page(query.page)?;
    Ok(Json(
        games::list_all(state.db(), page)
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

    let game = match tenhou_fetch::fetch_via_queue(state.tenhou_queue(), &log_id, user.id).await {
        Ok(game) => game,
        Err(tenhou_fetch::FetchError::Unprocessable) => {
            return Err(ApiError::unprocessable("invalid Tenhou log"));
        }
        Err(tenhou_fetch::FetchError::Cache(error)) => {
            return Err(ApiError::internal(error));
        }
        Err(error) => return Err(ApiError::bad_gateway(error)),
    };
    Ok(Json(game))
}

#[worker::send]
async fn detail(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Response, ApiError> {
    let log_id = exact_log_id(&id)?;
    let game = games::find(state.db(), &log_id)
        .await
        .map_err(ApiError::internal)?
        .ok_or_else(|| ApiError::not_found("game not found"))?;
    Ok(super::cached_json(game, super::GAME_CACHE_CONTROL))
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
