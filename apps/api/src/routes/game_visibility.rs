use axum::{
    Json, Router,
    extract::{Path, State, rejection::JsonRejection},
    http::StatusCode,
    routing::patch,
};
use domain::types::UpdateGameVisibilityInput;

use crate::{auth::middleware::AuthedUser, db::games, error::ApiError, state::AppState};

pub fn router() -> Router<AppState> {
    Router::new().route("/api/me/library/{id}/visibility", patch(update))
}

#[worker::send]
async fn update(
    State(state): State<AppState>,
    user: AuthedUser,
    Path(id): Path<String>,
    body: Result<Json<UpdateGameVisibilityInput>, JsonRejection>,
) -> Result<StatusCode, ApiError> {
    let log_id = super::games::exact_log_id(&id)?;
    let Json(input) = body.map_err(|_| ApiError::bad_request("invalid JSON body"))?;
    let updated = games::set_visibility(state.db(), user.id, &log_id, input.is_public)
        .await
        .map_err(ApiError::internal)?;
    if !updated {
        return Err(ApiError::not_found("game not found"));
    }
    Ok(StatusCode::NO_CONTENT)
}
