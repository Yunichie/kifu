use std::time::Duration;

use futures_util::lock::Mutex;
use worker::{Date, Delay, DurableObject, Env, Request, Response, Result, State, durable_object};

use crate::{db::games, tenhou_fetch};

const MINIMUM_SPACING_MS: u64 = 1_000;
const CACHE_COMMIT_LEASE_MS: u64 = 120_000;
const CACHE_POLL_MS: u64 = 100;
const PENDING_PREFIX: &str = "pending:";
const LAST_FINISHED_AT_KEY: &str = "last_finished_at";

#[durable_object]
pub struct TenhouQueueDO {
    state: State,
    env: Env,
    fetch_lock: Mutex<()>,
}

impl DurableObject for TenhouQueueDO {
    fn new(state: State, env: Env) -> Self {
        Self {
            state,
            env,
            fetch_lock: Mutex::new(()),
        }
    }

    async fn fetch(&self, request: Request) -> Result<Response> {
        let url = request.url()?;
        let log_id = url
            .query_pairs()
            .find_map(|(key, value)| (key == "log_id").then(|| value.into_owned()))
            .and_then(|value| {
                tenhou_fetch::canonical_log_id(&value)
                    .ok()
                    .filter(|canonical| canonical == &value)
            });
        let Some(log_id) = log_id else {
            return Response::error("invalid log ID", 400);
        };

        let pending_key = format!("{PENDING_PREFIX}{log_id}");
        if url.path() == "/complete" {
            self.state.storage().delete(&pending_key).await?;
            return Ok(Response::empty()?.with_status(204));
        }
        if url.path() != "/fetch" {
            return Response::error("not found", 404);
        }

        // Holding this guard across the fetch is the global serialization point.
        let _fetch_guard = self.fetch_lock.lock().await;
        let db = self.env.d1("DB")?;
        if games::exists(&db, &log_id).await? {
            self.state.storage().delete(&pending_key).await?;
            return Ok(Response::empty()?.with_status(204));
        }
        if self
            .wait_for_cache_commit(&db, &log_id, &pending_key)
            .await?
        {
            return Ok(Response::empty()?.with_status(204));
        }

        let last_finished_at = self
            .state
            .storage()
            .get::<u64>(LAST_FINISHED_AT_KEY)
            .await?
            .unwrap_or_default();
        let wait_ms = spacing_delay_ms(Date::now().as_millis(), last_finished_at);
        if wait_ms > 0 {
            Delay::from(Duration::from_millis(wait_ms)).await;
        }

        let started_at = Date::now().as_millis();
        worker::console_log!(
            "tenhou_queue start log_id={log_id} at={started_at} wait_ms={wait_ms}"
        );
        self.state.storage().put(&pending_key, started_at).await?;
        let result = tenhou_fetch::fetch_from_tenhou(&log_id).await;
        let finished_at = Date::now().as_millis();
        self.state
            .storage()
            .put(LAST_FINISHED_AT_KEY, finished_at)
            .await?;
        worker::console_log!("tenhou_queue finish log_id={log_id} at={finished_at}");

        match result {
            Ok(xml) => {
                self.state.storage().put(&pending_key, finished_at).await?;
                Response::ok(xml)
            }
            Err(error) => {
                self.state.storage().delete(&pending_key).await?;
                worker::console_error!("tenhou_queue fetch failed log_id={log_id}: {error}");
                Response::error("Tenhou fetch failed", 502)
            }
        }
    }
}

fn spacing_delay_ms(now: u64, last_finished_at: u64) -> u64 {
    MINIMUM_SPACING_MS.saturating_sub(now.saturating_sub(last_finished_at))
}

impl TenhouQueueDO {
    async fn wait_for_cache_commit(
        &self,
        db: &worker::D1Database,
        log_id: &str,
        pending_key: &str,
    ) -> Result<bool> {
        let Some(fetched_at) = self.state.storage().get::<u64>(pending_key).await? else {
            return Ok(false);
        };

        loop {
            if games::exists(db, log_id).await? {
                self.state.storage().delete(pending_key).await?;
                return Ok(true);
            }

            let elapsed = Date::now().as_millis().saturating_sub(fetched_at);
            if elapsed >= CACHE_COMMIT_LEASE_MS {
                self.state.storage().delete(pending_key).await?;
                return Ok(false);
            }
            Delay::from(Duration::from_millis(
                CACHE_POLL_MS.min(CACHE_COMMIT_LEASE_MS - elapsed),
            ))
            .await;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::spacing_delay_ms;

    #[test]
    fn enforces_minimum_spacing_after_the_last_fetch() {
        assert_eq!(spacing_delay_ms(1_100, 1_000), 900);
        assert_eq!(spacing_delay_ms(2_000, 1_000), 0);
        assert_eq!(spacing_delay_ms(2_001, 1_000), 0);
    }
}
