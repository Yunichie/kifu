use axum::{
    Json, Router,
    extract::{Path, State},
    routing::get,
};
use domain::types::CareerStats;

use crate::{db::games, error::ApiError, state::AppState};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/players", get(list))
        .route("/api/players/{name}/career", get(career))
}

#[worker::send]
async fn list(State(state): State<AppState>) -> Result<Json<Vec<String>>, ApiError> {
    Ok(Json(
        games::list_players(state.db())
            .await
            .map_err(ApiError::internal)?,
    ))
}

#[worker::send]
async fn career(
    State(state): State<AppState>,
    Path(name): Path<String>,
) -> Result<Json<CareerStats>, ApiError> {
    validate_player_name(&name)?;
    Ok(Json(
        games::career(state.db(), &[name])
            .await
            .map_err(ApiError::internal)?,
    ))
}

fn validate_player_name(name: &str) -> Result<(), ApiError> {
    if !super::valid_player_name(name) {
        return Err(ApiError::bad_request("invalid player name"));
    }
    Ok(())
}
