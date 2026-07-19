use axum::{
    Json,
    http::{HeaderValue, header::CACHE_CONTROL},
    response::{IntoResponse, Response},
};
use serde::Serialize;

pub mod auth;
pub mod games;
pub mod health;
pub mod me;
pub mod oauth;
pub mod players;

const MAX_PAGE: u32 = 100_000;
pub(super) const CAREER_CACHE_CONTROL: &str = "public, max-age=0, s-maxage=10, must-revalidate";
pub(super) const GAME_CACHE_CONTROL: &str = "public, max-age=86400, immutable";

fn cached_json<T: Serialize>(value: T, cache_control: &'static str) -> Response {
    let mut response = Json(value).into_response();
    response
        .headers_mut()
        .insert(CACHE_CONTROL, HeaderValue::from_static(cache_control));
    response
}

fn valid_page(page: Option<u32>) -> Result<u32, crate::error::ApiError> {
    let page = page.unwrap_or(1);
    if page == 0 || page > MAX_PAGE {
        return Err(crate::error::ApiError::bad_request("invalid page"));
    }
    Ok(page)
}

fn valid_player_name(name: &str) -> bool {
    !name.is_empty() && name.chars().count() <= 64 && !name.chars().any(char::is_control)
}

#[cfg(test)]
mod tests {
    use axum::http::header::CACHE_CONTROL;

    use super::{
        CAREER_CACHE_CONTROL, GAME_CACHE_CONTROL, cached_json, valid_page, valid_player_name,
    };

    #[test]
    fn public_cache_policies_are_explicit() {
        let career = cached_json((), CAREER_CACHE_CONTROL);
        let game = cached_json((), GAME_CACHE_CONTROL);

        assert_eq!(career.headers()[CACHE_CONTROL], CAREER_CACHE_CONTROL);
        assert_eq!(game.headers()[CACHE_CONTROL], GAME_CACHE_CONTROL);
    }

    #[test]
    fn validates_player_names() {
        assert!(valid_player_name("CLS"));
        assert!(valid_player_name("\u{77f3}\u{6a4b}"));
        assert!(!valid_player_name(""));
        assert!(!valid_player_name(&"x".repeat(65)));
        assert!(!valid_player_name("line\nbreak"));
    }

    #[test]
    fn validates_page_numbers() {
        assert_eq!(valid_page(None).ok(), Some(1));
        assert_eq!(valid_page(Some(2)).ok(), Some(2));
        assert!(valid_page(Some(0)).is_err());
        assert!(valid_page(Some(100_001)).is_err());
    }
}
