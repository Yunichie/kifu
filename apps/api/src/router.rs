use axum::Router;

use crate::{routes, state::AppState};

pub fn router() -> Router<AppState> {
    Router::new()
        .merge(routes::auth::router())
        .merge(routes::me::router())
}
