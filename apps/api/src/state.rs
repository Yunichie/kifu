use std::sync::Arc;

use worker::{D1Database, Date, Env, Error, Result};

#[derive(Clone)]
pub struct AppState {
    db: Arc<D1Database>,
    session_secret: Arc<[u8]>,
}

impl AppState {
    pub fn from_env(env: &Env) -> Result<Self> {
        let secret = env.secret("SESSION_SECRET")?.to_string();
        if secret.len() < 32 {
            return Err(Error::RustError(
                "SESSION_SECRET must contain at least 32 bytes".into(),
            ));
        }

        Ok(Self {
            db: Arc::new(env.d1("DB")?),
            session_secret: Arc::from(secret.into_bytes()),
        })
    }

    pub fn db(&self) -> &D1Database {
        &self.db
    }

    pub fn session_secret(&self) -> &[u8] {
        &self.session_secret
    }
}

pub fn now_millis() -> u64 {
    Date::now().as_millis()
}

pub fn now_seconds() -> u64 {
    now_millis() / 1_000
}
