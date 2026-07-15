use axum::{Json, Router, routing::get};
use domain::types::HealthResponse;

use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new().route("/api/health", get(health))
}

async fn health() -> Json<HealthResponse> {
    Json(HealthResponse { ok: true })
}
