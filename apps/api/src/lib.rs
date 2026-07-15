use axum::{Router, routing::get};
use tower_service::Service;
use worker::{Context, Env, HttpRequest, Result, event};

fn router() -> Router {
    Router::new().route("/", get(|| async { domain::SCAFFOLD_RESPONSE }))
}

#[event(fetch)]
pub async fn fetch(
    request: HttpRequest,
    _env: Env,
    _context: Context,
) -> Result<axum::response::Response> {
    Ok(router().call(request).await?)
}
