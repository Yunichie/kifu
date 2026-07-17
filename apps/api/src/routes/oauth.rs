use axum::{
    Json, Router,
    extract::{State, rejection::JsonRejection},
    response::IntoResponse,
    routing::post,
};
use domain::types::OAuthExchangeInput;
use serde::Deserialize;

use crate::{
    db::users,
    error::ApiError,
    state::{AppState, now_millis},
};

const GOOGLE_TOKEN_URL: &str = "https://oauth2.googleapis.com/token";
const GOOGLE_USERINFO_URL: &str = "https://openidconnect.googleapis.com/v1/userinfo";
const PROVIDER: &str = "google";

#[derive(Deserialize)]
struct TokenResponse {
    access_token: String,
}

#[derive(Deserialize)]
struct GoogleUser {
    sub: String,
    name: Option<String>,
    given_name: Option<String>,
}

pub fn router() -> Router<AppState> {
    Router::new().route("/api/auth/google", post(exchange))
}

#[worker::send]
async fn exchange(
    State(state): State<AppState>,
    body: Result<Json<OAuthExchangeInput>, JsonRejection>,
) -> Result<impl IntoResponse, ApiError> {
    let Json(input) = body.map_err(|_| ApiError::bad_request("invalid JSON body"))?;
    validate_input(&input)?;

    let token = exchange_code(&state, &input).await?;
    let google_user = fetch_user(&token.access_token).await?;
    let subject = valid_subject(&google_user.sub)?;
    let display_name = display_name(google_user.name, google_user.given_name);
    let user = users::upsert_oauth_user(state.db(), PROVIDER, subject, &display_name, now_millis())
        .await
        .map_err(ApiError::internal)?;

    super::auth::authenticated_response(user, &state)
}

async fn exchange_code(
    state: &AppState,
    input: &OAuthExchangeInput,
) -> Result<TokenResponse, ApiError> {
    let response = reqwest::Client::new()
        .post(GOOGLE_TOKEN_URL)
        .form(&[
            ("code", input.code.as_str()),
            ("client_id", state.google_client_id()),
            ("client_secret", state.google_client_secret()),
            ("redirect_uri", state.google_redirect_uri()),
            ("grant_type", "authorization_code"),
            ("code_verifier", input.code_verifier.as_str()),
        ])
        .send()
        .await
        .map_err(|error| ApiError::upstream("Google login is temporarily unavailable", error))?;

    if !response.status().is_success() {
        worker::console_error!("Google token exchange returned HTTP {}", response.status());
        return Err(ApiError::bad_request(
            "Google authorization could not be completed",
        ));
    }

    parse_json(response, "Google token response").await
}

async fn fetch_user(access_token: &str) -> Result<GoogleUser, ApiError> {
    let response = reqwest::Client::new()
        .get(GOOGLE_USERINFO_URL)
        .bearer_auth(access_token)
        .send()
        .await
        .map_err(|error| ApiError::upstream("Google login is temporarily unavailable", error))?;

    if !response.status().is_success() {
        worker::console_error!("Google UserInfo returned HTTP {}", response.status());
        return Err(ApiError::bad_request(
            "Google authorization could not be completed",
        ));
    }

    parse_json(response, "Google UserInfo response").await
}

async fn parse_json<T: serde::de::DeserializeOwned>(
    response: reqwest::Response,
    context: &str,
) -> Result<T, ApiError> {
    let body = response
        .text()
        .await
        .map_err(|error| ApiError::upstream("Google login is temporarily unavailable", error))?;
    serde_json::from_str(&body).map_err(|error| {
        worker::console_error!("invalid {context}: {error}");
        ApiError::Internal
    })
}

fn validate_input(input: &OAuthExchangeInput) -> Result<(), ApiError> {
    if input.code.is_empty() || input.code.len() > 2048 {
        return Err(ApiError::bad_request("invalid OAuth authorization code"));
    }
    if !(43..=128).contains(&input.code_verifier.len())
        || !input
            .code_verifier
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'.' | b'_' | b'~'))
    {
        return Err(ApiError::bad_request("invalid OAuth PKCE verifier"));
    }
    Ok(())
}

fn valid_subject(subject: &str) -> Result<&str, ApiError> {
    if subject.is_empty() || subject.len() > 255 || !subject.bytes().all(|byte| byte.is_ascii()) {
        return Err(ApiError::bad_request("invalid Google account identity"));
    }
    Ok(subject)
}

fn display_name(name: Option<String>, given_name: Option<String>) -> String {
    name.or(given_name)
        .map(|value| value.trim().chars().take(100).collect::<String>())
        .filter(|value| !value.is_empty() && !value.chars().any(char::is_control))
        .unwrap_or_else(|| "Google user".into())
}

#[cfg(test)]
mod tests {
    use domain::types::OAuthExchangeInput;

    use super::{display_name, valid_subject, validate_input};

    #[test]
    fn validates_oauth_inputs() {
        let valid = OAuthExchangeInput {
            code: "authorization-code".into(),
            code_verifier: "a".repeat(43),
        };
        assert!(validate_input(&valid).is_ok());

        let invalid_verifier = OAuthExchangeInput {
            code_verifier: "not valid".into(),
            ..valid
        };
        assert!(validate_input(&invalid_verifier).is_err());
        assert!(valid_subject("google-subject-123").is_ok());
        assert!(valid_subject("").is_err());
    }

    #[test]
    fn normalizes_google_display_names() {
        assert_eq!(
            display_name(Some("  A Google User  ".into()), None),
            "A Google User"
        );
        assert_eq!(display_name(Some("\n".into()), None), "Google user");
    }
}
