use axum::{
    Json, Router,
    http::{HeaderMap, HeaderValue, StatusCode, header::SET_COOKIE},
    response::{IntoResponse, Response},
    routing::post,
};
use domain::types::{AuthResponse, User};

use crate::{
    auth::{middleware::AuthedUser, session},
    error::ApiError,
    state::{AppState, now_seconds},
};

pub fn router() -> Router<AppState> {
    Router::new().route("/api/auth/logout", post(logout))
}

async fn logout(_user: AuthedUser) -> Result<Response, ApiError> {
    let mut response = StatusCode::NO_CONTENT.into_response();
    response.headers_mut().insert(
        SET_COOKIE,
        HeaderValue::from_static(session::clear_cookie()),
    );
    Ok(response)
}

pub(super) fn authenticated_response(
    user: User,
    state: &AppState,
) -> Result<(HeaderMap, Json<AuthResponse>), ApiError> {
    let token = session::issue(user.id, now_seconds(), state.session_secret())
        .map_err(|_| ApiError::Internal)?;
    let cookie =
        HeaderValue::from_str(&session::session_cookie(&token)).map_err(ApiError::internal)?;
    let mut headers = HeaderMap::new();
    headers.insert(SET_COOKIE, cookie);
    Ok((headers, Json(AuthResponse { user })))
}
