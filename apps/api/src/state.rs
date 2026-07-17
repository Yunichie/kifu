use std::sync::Arc;

use worker::{D1Database, Date, Env, Error, ObjectNamespace, Result};

#[derive(Clone)]
pub struct AppState {
    db: Arc<D1Database>,
    session_secret: Arc<[u8]>,
    google_client_id: Arc<str>,
    google_client_secret: Arc<str>,
    google_redirect_uri: Arc<str>,
    tenhou_queue: ObjectNamespace,
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
            google_client_id: Arc::from(env.secret("GOOGLE_CLIENT_ID")?.to_string()),
            google_client_secret: Arc::from(env.secret("GOOGLE_CLIENT_SECRET")?.to_string()),
            google_redirect_uri: Arc::from(env.secret("GOOGLE_REDIRECT_URI")?.to_string()),
            tenhou_queue: env.durable_object("TENHOU_QUEUE")?,
        })
    }

    pub fn db(&self) -> &D1Database {
        &self.db
    }

    pub fn session_secret(&self) -> &[u8] {
        &self.session_secret
    }

    pub fn google_client_id(&self) -> &str {
        &self.google_client_id
    }

    pub fn google_client_secret(&self) -> &str {
        &self.google_client_secret
    }

    pub fn google_redirect_uri(&self) -> &str {
        &self.google_redirect_uri
    }

    pub fn tenhou_queue(&self) -> &ObjectNamespace {
        &self.tenhou_queue
    }
}

pub fn now_millis() -> u64 {
    Date::now().as_millis()
}

pub fn now_seconds() -> u64 {
    now_millis() / 1_000
}
