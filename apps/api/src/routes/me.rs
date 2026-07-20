use axum::{
    Json, Router,
    extract::{Path, Query, State, rejection::JsonRejection},
    routing::{delete, get, post},
};
use domain::types::{CareerStats, GameListPage, MeResponse, PlayerNameInput, PlayerNamesResponse};
use serde::Deserialize;

use crate::{
    auth::middleware::AuthedUser,
    db::{games, users},
    error::ApiError,
    state::{AppState, now_millis},
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/me", get(me))
        .route("/api/me/career", get(career))
        .route("/api/me/library", get(library))
        .route("/api/me/player-names", post(add_player_name))
        .route("/api/me/player-names/{name}", delete(remove_player_name))
}

#[derive(Default, Deserialize)]
struct PaginationQuery {
    page: Option<u32>,
}

#[worker::send]
async fn career(
    State(state): State<AppState>,
    user: AuthedUser,
) -> Result<Json<CareerStats>, ApiError> {
    let player_names = users::list_player_names(state.db(), user.id)
        .await
        .map_err(ApiError::internal)?;
    Ok(Json(
        games::visible_career(state.db(), user.id, &player_names)
            .await
            .map_err(ApiError::internal)?,
    ))
}

#[worker::send]
async fn library(
    State(state): State<AppState>,
    user: AuthedUser,
    Query(query): Query<PaginationQuery>,
) -> Result<Json<GameListPage>, ApiError> {
    let page = super::valid_page(query.page)?;
    Ok(Json(
        games::list_saved(state.db(), user.id, page)
            .await
            .map_err(ApiError::internal)?,
    ))
}

#[worker::send]
async fn me(State(state): State<AppState>, user: AuthedUser) -> Result<Json<MeResponse>, ApiError> {
    let user_record = users::find_by_id(state.db(), user.id)
        .await
        .map_err(ApiError::internal)?
        .ok_or(ApiError::Unauthorized)?;
    let player_names = users::list_player_names(state.db(), user.id)
        .await
        .map_err(ApiError::internal)?;

    Ok(Json(MeResponse {
        user: user_record,
        player_names,
    }))
}

#[worker::send]
async fn add_player_name(
    State(state): State<AppState>,
    user: AuthedUser,
    body: Result<Json<PlayerNameInput>, JsonRejection>,
) -> Result<Json<PlayerNamesResponse>, ApiError> {
    let Json(input) = body.map_err(|_| ApiError::bad_request("invalid JSON body"))?;
    let name = input.name.trim();
    validate_player_name(name)?;

    users::add_player_name(state.db(), user.id, name, now_millis())
        .await
        .map_err(ApiError::internal)?;
    player_names_response(&state, user.id).await
}

#[worker::send]
async fn remove_player_name(
    State(state): State<AppState>,
    user: AuthedUser,
    Path(name): Path<String>,
) -> Result<Json<PlayerNamesResponse>, ApiError> {
    validate_player_name(&name)?;
    users::remove_player_name(state.db(), user.id, &name)
        .await
        .map_err(ApiError::internal)?;
    player_names_response(&state, user.id).await
}

async fn player_names_response(
    state: &AppState,
    user_id: i32,
) -> Result<Json<PlayerNamesResponse>, ApiError> {
    let player_names = users::list_player_names(state.db(), user_id)
        .await
        .map_err(ApiError::internal)?;
    Ok(Json(PlayerNamesResponse { player_names }))
}

fn validate_player_name(name: &str) -> Result<(), ApiError> {
    if !super::valid_player_name(name) {
        return Err(ApiError::bad_request(
            "player name must be 1-64 characters without control characters",
        ));
    }
    Ok(())
}
