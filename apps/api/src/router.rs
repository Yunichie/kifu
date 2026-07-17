use axum::Router;

use crate::{routes, state::AppState};

pub fn router() -> Router<AppState> {
    Router::new()
        .merge(routes::auth::router())
        .merge(routes::games::router())
        .merge(routes::health::router())
        .merge(routes::me::router())
        .merge(routes::oauth::router())
        .merge(routes::players::router())
}
