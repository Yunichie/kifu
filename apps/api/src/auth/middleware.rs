use axum::{
    extract::FromRequestParts,
    http::{header::COOKIE, request::Parts},
};

use crate::{
    auth::session,
    error::ApiError,
    state::{AppState, now_seconds},
};

#[derive(Clone, Copy, Debug)]
pub struct AuthedUser {
    pub id: i32,
}

impl FromRequestParts<AppState> for AuthedUser {
    type Rejection = ApiError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let cookie = parts
            .headers
            .get(COOKIE)
            .and_then(|value| value.to_str().ok())
            .ok_or(ApiError::Unauthorized)?;
        let token = session::token_from_cookie_header(cookie).ok_or(ApiError::Unauthorized)?;
        let claims = session::verify(token, now_seconds(), state.session_secret())
            .map_err(|_| ApiError::Unauthorized)?;

        Ok(Self { id: claims.user_id })
    }
}
