use std::time::Duration;

use futures_util::lock::Mutex;
use worker::{Date, Delay, DurableObject, Env, Request, Response, Result, State, durable_object};

use crate::tenhou_fetch;

const MINIMUM_SPACING_MS: u64 = 1_000;

#[durable_object]
pub struct TenhouQueueDO {
    last_finished_at: Mutex<u64>,
}

impl DurableObject for TenhouQueueDO {
    fn new(_state: State, _env: Env) -> Self {
        Self {
            last_finished_at: Mutex::new(0),
        }
    }

    async fn fetch(&self, request: Request) -> Result<Response> {
        let log_id = request
            .url()?
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

        // Holding this guard across the fetch is the global serialization point.
        let mut last_finished_at = self.last_finished_at.lock().await;
        let wait_ms = MINIMUM_SPACING_MS
            .saturating_sub(Date::now().as_millis().saturating_sub(*last_finished_at));
        if wait_ms > 0 {
            Delay::from(Duration::from_millis(wait_ms)).await;
        }

        let started_at = Date::now().as_millis();
        worker::console_log!(
            "tenhou_queue start log_id={log_id} at={started_at} wait_ms={wait_ms}"
        );
        let result = tenhou_fetch::fetch_from_tenhou(&log_id).await;
        let finished_at = Date::now().as_millis();
        *last_finished_at = finished_at;
        worker::console_log!("tenhou_queue finish log_id={log_id} at={finished_at}");

        match result {
            Ok(xml) => Response::ok(xml),
            Err(error) => {
                worker::console_error!("tenhou_queue fetch failed log_id={log_id}: {error}");
                Response::error("Tenhou fetch failed", 502)
            }
        }
    }
}
