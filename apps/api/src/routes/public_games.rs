use axum::{
    Json, Router,
    extract::{Path, Query, State},
    routing::get,
};
use domain::types::{GameListPage, PublicGameDetail};
use serde::Deserialize;

use crate::{db::games, error::ApiError, state::AppState};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/public-games", get(list))
        .route("/api/public-games/{owner_id}/{id}", get(detail))
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
        games::list_public(state.db(), page)
            .await
            .map_err(ApiError::internal)?,
    ))
}

#[worker::send]
async fn detail(
    State(state): State<AppState>,
    Path((owner_id, id)): Path<(i32, String)>,
) -> Result<Json<PublicGameDetail>, ApiError> {
    if owner_id <= 0 {
        return Err(ApiError::not_found("game not found"));
    }
    let log_id = super::games::exact_log_id(&id)?;
    let game = games::find_public(state.db(), owner_id, &log_id)
        .await
        .map_err(ApiError::internal)?
        .ok_or_else(|| ApiError::not_found("game not found"))?;
    Ok(Json(game))
}
