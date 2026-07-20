pub mod auth;
pub mod game_visibility;
pub mod games;
pub mod health;
pub mod me;
pub mod oauth;
pub mod players;
pub mod public_games;

const MAX_PAGE: u32 = 100_000;

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
    use super::{valid_page, valid_player_name};

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
