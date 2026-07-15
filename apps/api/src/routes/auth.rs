use axum::{
    Json, Router,
    extract::{State, rejection::JsonRejection},
    http::{HeaderMap, HeaderValue, StatusCode, header::SET_COOKIE},
    response::{IntoResponse, Response},
    routing::post,
};
use domain::types::{AuthCredentials, AuthResponse, User};

use crate::{
    auth::{middleware::AuthedUser, password, session},
    db::users,
    error::ApiError,
    state::{AppState, now_millis, now_seconds},
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/auth/signup", post(signup))
        .route("/api/auth/login", post(login))
        .route("/api/auth/logout", post(logout))
}

#[worker::send]
async fn signup(
    State(state): State<AppState>,
    body: Result<Json<AuthCredentials>, JsonRejection>,
) -> Result<impl IntoResponse, ApiError> {
    let Json(credentials) = body.map_err(|_| ApiError::bad_request("invalid JSON body"))?;
    let username = credentials.username.trim();
    validate_signup(username, &credentials.password)?;

    let password_hash =
        password::hash_password(&credentials.password).map_err(ApiError::internal)?;
    let user = users::create_user(state.db(), username, &password_hash, now_millis())
        .await
        .map_err(ApiError::internal)?
        .ok_or_else(|| ApiError::conflict("username is already taken"))?;

    authenticated_response(user, &state)
}

#[worker::send]
async fn login(
    State(state): State<AppState>,
    body: Result<Json<AuthCredentials>, JsonRejection>,
) -> Result<impl IntoResponse, ApiError> {
    let Json(credentials) = body.map_err(|_| ApiError::bad_request("invalid JSON body"))?;
    let username = credentials.username.trim();
    if !valid_username(username) || !(12..=128).contains(&credentials.password.len()) {
        return Err(ApiError::Unauthorized);
    }

    let Some(stored) = users::find_by_username(state.db(), username)
        .await
        .map_err(ApiError::internal)?
    else {
        password::hash_password(&credentials.password).map_err(ApiError::internal)?;
        return Err(ApiError::Unauthorized);
    };

    let valid = password::verify_password(&credentials.password, &stored.password_hash)
        .map_err(ApiError::internal)?;
    if !valid {
        return Err(ApiError::Unauthorized);
    }

    authenticated_response(
        User {
            id: stored.id,
            username: stored.username,
        },
        &state,
    )
}

async fn logout(_user: AuthedUser) -> Result<Response, ApiError> {
    let mut response = StatusCode::NO_CONTENT.into_response();
    response.headers_mut().insert(
        SET_COOKIE,
        HeaderValue::from_static(session::clear_cookie()),
    );
    Ok(response)
}

fn authenticated_response(
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

fn validate_signup(username: &str, password: &str) -> Result<(), ApiError> {
    if !valid_username(username) {
        return Err(ApiError::bad_request(
            "username must be 3-32 characters using letters, numbers, _ or -",
        ));
    }
    if !(12..=128).contains(&password.len()) {
        return Err(ApiError::bad_request(
            "password must be between 12 and 128 bytes",
        ));
    }
    Ok(())
}

fn valid_username(username: &str) -> bool {
    (3..=32).contains(&username.len())
        && username
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'-'))
}
