use axum::{
    Json, Router,
    extract::{Path, State, rejection::JsonRejection},
    routing::{delete, get, post},
};
use domain::types::{MeResponse, PlayerNameInput, PlayerNamesResponse};

use crate::{
    auth::middleware::AuthedUser,
    db::users,
    error::ApiError,
    state::{AppState, now_millis},
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/me", get(me))
        .route("/api/me/player-names", post(add_player_name))
        .route("/api/me/player-names/{name}", delete(remove_player_name))
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
    if name.is_empty() || name.chars().count() > 64 || name.chars().any(char::is_control) {
        return Err(ApiError::bad_request(
            "player name must be 1-64 characters without control characters",
        ));
    }
    Ok(())
}
