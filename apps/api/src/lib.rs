use axum::http::{HeaderValue, header::CACHE_CONTROL};
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
    let mut response = router::router().with_state(state).call(request).await?;
    apply_default_cache_policy(&mut response);
    Ok(response)
}

fn apply_default_cache_policy(response: &mut axum::response::Response) {
    response
        .headers_mut()
        .entry(CACHE_CONTROL)
        .or_insert(HeaderValue::from_static("private, no-store"));
}

#[cfg(test)]
mod tests {
    use axum::{
        body::Body,
        http::{HeaderValue, header::CACHE_CONTROL},
        response::Response,
    };

    use super::apply_default_cache_policy;

    #[test]
    fn defaults_responses_to_private_no_store() {
        let mut response = Response::new(Body::empty());

        apply_default_cache_policy(&mut response);

        assert_eq!(response.headers()[CACHE_CONTROL], "private, no-store");
    }

    #[test]
    fn preserves_explicit_cache_policy() {
        let mut response = Response::new(Body::empty());
        response.headers_mut().insert(
            CACHE_CONTROL,
            HeaderValue::from_static("public, max-age=86400, immutable"),
        );

        apply_default_cache_policy(&mut response);

        assert_eq!(
            response.headers()[CACHE_CONTROL],
            "public, max-age=86400, immutable"
        );
    }
}
