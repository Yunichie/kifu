use axum::{
    Json, Router,
    extract::{Path, Query, State},
    routing::get,
};
use domain::types::{CareerStats, PlayerSearchPage};
use serde::Deserialize;

use crate::{db::games, error::ApiError, state::AppState};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/players", get(list))
        .route("/api/players/{name}/career", get(career))
}

#[derive(Default, Deserialize)]
struct PlayerSearchQuery {
    q: Option<String>,
    page: Option<u32>,
}

#[worker::send]
async fn list(
    State(state): State<AppState>,
    Query(search): Query<PlayerSearchQuery>,
) -> Result<Json<PlayerSearchPage>, ApiError> {
    let query = search.q.unwrap_or_default();
    let query = query.trim();
    if query.chars().count() > 64 || query.chars().any(char::is_control) {
        return Err(ApiError::bad_request("invalid player search"));
    }
    let page = super::valid_page(search.page)?;
    Ok(Json(
        games::search_players(state.db(), query, page)
            .await
            .map_err(ApiError::internal)?,
    ))
}

#[worker::send]
async fn career(
    State(state): State<AppState>,
    Path(name): Path<String>,
) -> Result<Json<CareerStats>, ApiError> {
    validate_career_player_name(&name)?;
    let career = games::public_career(state.db(), &[name])
        .await
        .map_err(ApiError::internal)?;
    Ok(Json(career))
}

fn validate_career_player_name(name: &str) -> Result<(), ApiError> {
    if name == crate::TENHOU_GUEST_NAME {
        return Err(ApiError::not_found("player not found"));
    }
    if !super::valid_player_name(name) {
        return Err(ApiError::bad_request("invalid player name"));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::error::ApiError;

    use super::validate_career_player_name;

    #[test]
    fn hides_the_tenhou_guest_career_path() {
        assert!(matches!(
            validate_career_player_name("NoName"),
            Err(ApiError::NotFound(_))
        ));
        assert!(validate_career_player_name("noname").is_ok());
        assert!(validate_career_player_name("CLS").is_ok());
    }
}
