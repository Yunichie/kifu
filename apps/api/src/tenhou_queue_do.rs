use std::time::Duration;

use futures_util::lock::Mutex;
use worker::{Date, Delay, DurableObject, Env, Request, Response, Result, State, durable_object};

use crate::{db::games, tenhou_fetch};

const MINIMUM_SPACING_MS: u64 = 1_000;
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
        if url.path() != "/fetch" {
            return Response::error("not found", 404);
        }
        let user_id = url
            .query_pairs()
            .find_map(|(key, value)| (key == "user_id").then(|| value.into_owned()))
            .and_then(|value| value.parse::<i32>().ok())
            .filter(|value| *value > 0);
        let Some(user_id) = user_id else {
            return Response::error("invalid user ID", 400);
        };

        // This guard is the required global serialization point for Tenhou fetches.
        let _fetch_guard = self.fetch_lock.lock().await;
        let db = self.env.d1("DB")?;
        match games::find(&db, &log_id).await {
            Ok(Some(game)) => {
                if let Err(error) =
                    games::save_for_user(&db, user_id, &log_id, Date::now().as_millis()).await
                {
                    worker::console_error!(
                        "tenhou_queue cache link failed log_id={log_id}: {error}"
                    );
                    return Response::error("game cache update failed", 500);
                }
                worker::console_log!("tenhou_queue cache hit log_id={log_id}");
                return Response::from_json(&game);
            }
            Ok(None) => {}
            Err(error) => {
                worker::console_error!("tenhou_queue cache lookup failed log_id={log_id}: {error}");
                return Response::error("game cache lookup failed", 500);
            }
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
        let result = tenhou_fetch::fetch_from_tenhou(&log_id).await;
        let finished_at = Date::now().as_millis();
        self.state
            .storage()
            .put(LAST_FINISHED_AT_KEY, finished_at)
            .await?;
        worker::console_log!("tenhou_queue finish log_id={log_id} at={finished_at}");

        match result {
            Ok(xml) => {
                let game = match domain::parse_game(&log_id, &xml) {
                    Ok(game) => game,
                    Err(error) => {
                        worker::console_error!(
                            "tenhou_queue parse failed log_id={log_id}: {error}"
                        );
                        return Response::error("Tenhou log could not be parsed", 422);
                    }
                };
                if let Err(error) =
                    games::persist_and_save(&db, user_id, &game, Date::now().as_millis()).await
                {
                    worker::console_error!(
                        "tenhou_queue cache update failed log_id={log_id}: {error}"
                    );
                    return Response::error("game cache update failed", 500);
                }
                Response::from_json(&game)
            }
            Err(error) => {
                worker::console_error!("tenhou_queue fetch failed log_id={log_id}: {error}");
                Response::error("Tenhou fetch failed", 502)
            }
        }
    }
}

fn spacing_delay_ms(now: u64, last_finished_at: u64) -> u64 {
    MINIMUM_SPACING_MS.saturating_sub(now.saturating_sub(last_finished_at))
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
