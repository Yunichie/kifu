pub mod auth;
pub mod games;
pub mod health;
pub mod me;
pub mod players;

fn valid_player_name(name: &str) -> bool {
    !name.is_empty() && name.chars().count() <= 64 && !name.chars().any(char::is_control)
}

#[cfg(test)]
mod tests {
    use super::valid_player_name;

    #[test]
    fn validates_player_names() {
        assert!(valid_player_name("CLS"));
        assert!(valid_player_name("\u{77f3}\u{6a4b}"));
        assert!(!valid_player_name(""));
        assert!(!valid_player_name(&"x".repeat(65)));
        assert!(!valid_player_name("line\nbreak"));
    }
}
