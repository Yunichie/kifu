use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use hmac::{Hmac, Mac};
use sha2::Sha256;

pub const COOKIE_NAME: &str = "kifu_session";
pub const SESSION_TTL_SECONDS: u64 = 30 * 24 * 60 * 60;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SessionClaims {
    pub user_id: i32,
    pub expires_at: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SessionError {
    Invalid,
    Expired,
}

type HmacSha256 = Hmac<Sha256>;

pub fn issue(user_id: i32, now_seconds: u64, secret: &[u8]) -> Result<String, SessionError> {
    if user_id <= 0 {
        return Err(SessionError::Invalid);
    }

    let expires_at = now_seconds
        .checked_add(SESSION_TTL_SECONDS)
        .ok_or(SessionError::Invalid)?;
    let payload = format!("{user_id}.{expires_at}");
    let signature = sign(payload.as_bytes(), secret)?;
    Ok(format!("{payload}.{}", URL_SAFE_NO_PAD.encode(signature)))
}

pub fn verify(token: &str, now_seconds: u64, secret: &[u8]) -> Result<SessionClaims, SessionError> {
    let mut parts = token.split('.');
    let user_id = parts.next().ok_or(SessionError::Invalid)?;
    let expires_at = parts.next().ok_or(SessionError::Invalid)?;
    let signature = parts.next().ok_or(SessionError::Invalid)?;
    if parts.next().is_some() {
        return Err(SessionError::Invalid);
    }

    let payload = format!("{user_id}.{expires_at}");
    let signature = URL_SAFE_NO_PAD
        .decode(signature)
        .map_err(|_| SessionError::Invalid)?;
    let mut mac = HmacSha256::new_from_slice(secret).map_err(|_| SessionError::Invalid)?;
    mac.update(payload.as_bytes());
    mac.verify_slice(&signature)
        .map_err(|_| SessionError::Invalid)?;

    let user_id = user_id.parse::<i32>().map_err(|_| SessionError::Invalid)?;
    let expires_at = expires_at
        .parse::<u64>()
        .map_err(|_| SessionError::Invalid)?;
    if user_id <= 0 {
        return Err(SessionError::Invalid);
    }
    if expires_at <= now_seconds {
        return Err(SessionError::Expired);
    }

    Ok(SessionClaims {
        user_id,
        expires_at,
    })
}

pub fn session_cookie(token: &str) -> String {
    format!(
        "{COOKIE_NAME}={token}; Path=/; Max-Age={SESSION_TTL_SECONDS}; HttpOnly; Secure; SameSite=Lax"
    )
}

pub fn clear_cookie() -> &'static str {
    "kifu_session=; Path=/; Max-Age=0; Expires=Thu, 01 Jan 1970 00:00:00 GMT; HttpOnly; Secure; SameSite=Lax"
}

pub fn token_from_cookie_header(header: &str) -> Option<&str> {
    header.split(';').find_map(|cookie| {
        let (name, value) = cookie.trim().split_once('=')?;
        (name == COOKIE_NAME && !value.is_empty()).then_some(value)
    })
}

fn sign(payload: &[u8], secret: &[u8]) -> Result<Vec<u8>, SessionError> {
    let mut mac = HmacSha256::new_from_slice(secret).map_err(|_| SessionError::Invalid)?;
    mac.update(payload);
    Ok(mac.finalize().into_bytes().to_vec())
}

#[cfg(test)]
mod tests {
    use super::{
        COOKIE_NAME, SESSION_TTL_SECONDS, SessionError, issue, session_cookie,
        token_from_cookie_header, verify,
    };

    const SECRET: &[u8] = b"a local test secret with at least 32 bytes";

    #[test]
    fn round_trips_a_signed_session_and_cookie() {
        let token = issue(42, 1_000, SECRET).unwrap();
        let claims = verify(&token, 1_001, SECRET).unwrap();
        let cookie = session_cookie(&token);

        assert_eq!(claims.user_id, 42);
        assert_eq!(claims.expires_at, 1_000 + SESSION_TTL_SECONDS);
        assert_eq!(token_from_cookie_header(&cookie), Some(token.as_str()));
        assert!(cookie.starts_with(&format!("{COOKIE_NAME}=")));
        assert!(cookie.contains("HttpOnly; Secure; SameSite=Lax"));
    }

    #[test]
    fn rejects_tampered_and_expired_sessions() {
        let token = issue(42, 1_000, SECRET).unwrap();
        let tampered = token.replacen("42.", "43.", 1);

        assert_eq!(verify(&tampered, 1_001, SECRET), Err(SessionError::Invalid));
        assert_eq!(
            verify(&token, 1_000 + SESSION_TTL_SECONDS, SECRET),
            Err(SessionError::Expired)
        );
    }
}
