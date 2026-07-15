use tower_service::Service;
use worker::{Context, Env, HttpRequest, Result, event};

mod auth;
mod db;
mod error;
mod router;
mod routes;
mod state;
mod tenhou_fetch;
mod tenhou_queue_do;

#[event(fetch)]
pub async fn fetch(
    request: HttpRequest,
    env: Env,
    _context: Context,
) -> Result<axum::response::Response> {
    let state = state::AppState::from_env(&env)?;
    Ok(router::router().with_state(state).call(request).await?)
}
