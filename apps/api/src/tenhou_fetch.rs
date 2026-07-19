use std::sync::OnceLock;

use domain::types::GameDetail;
use regex::Regex;
use thiserror::Error;
use worker::ObjectNamespace;

const QUEUE_OBJECT_NAME: &str = "global";
const USER_AGENT: &str = "kifu/0.1 (Tenhou statistics tracker)";

#[derive(Debug, Error)]
pub enum FetchError {
    #[error("invalid Tenhou log ID or URL")]
    InvalidLogId,
    #[error("Tenhou queue request failed: {0}")]
    Queue(String),
    #[error("Tenhou returned HTTP {0}")]
    UpstreamStatus(u16),
    #[error("Tenhou returned an empty log")]
    EmptyBody,
    #[error("Tenhou log could not be parsed")]
    Unprocessable,
    #[error("game cache operation failed: {0}")]
    Cache(String),
    #[error("Tenhou request failed: {0}")]
    Request(#[from] reqwest::Error),
}

pub fn canonical_log_id(input: &str) -> Result<String, FetchError> {
    static LOG_ID: OnceLock<Regex> = OnceLock::new();
    let regex = LOG_ID.get_or_init(|| {
        Regex::new(r"[0-9]{10}gm-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{8}")
            .expect("constant Tenhou log ID regex is valid")
    });

    regex
        .find(input.trim())
        .map(|found| found.as_str().to_owned())
        .ok_or(FetchError::InvalidLogId)
}

pub async fn fetch_via_queue(
    namespace: &ObjectNamespace,
    log_id: &str,
    user_id: i32,
) -> Result<GameDetail, FetchError> {
    let stub = namespace
        .get_by_name(QUEUE_OBJECT_NAME)
        .map_err(|error| FetchError::Queue(error.to_string()))?;
    let mut response = stub
        .fetch_with_str(&format!(
            "https://tenhou-queue/fetch?log_id={log_id}&user_id={user_id}"
        ))
        .await
        .map_err(|error| FetchError::Queue(error.to_string()))?;
    let status = response.status_code();
    let body = response
        .text()
        .await
        .map_err(|error| FetchError::Queue(error.to_string()))?;

    queue_result(status, body)
}

pub async fn fetch_from_tenhou(log_id: &str) -> Result<String, FetchError> {
    let response = reqwest::Client::builder()
        .user_agent(USER_AGENT)
        .build()?
        .get(format!("https://tenhou.net/0/log/?{log_id}"))
        .send()
        .await?;
    let status = response.status();
    if !status.is_success() {
        return Err(FetchError::UpstreamStatus(status.as_u16()));
    }

    let body = response.text().await?;
    if body.trim().is_empty() {
        return Err(FetchError::EmptyBody);
    }
    Ok(body)
}

fn queue_result(status: u16, body: String) -> Result<GameDetail, FetchError> {
    match status {
        200 => serde_json::from_str(&body)
            .map_err(|error| FetchError::Queue(format!("invalid queue response: {error}"))),
        422 => Err(FetchError::Unprocessable),
        500 => Err(FetchError::Cache(body)),
        _ => Err(FetchError::Queue(body)),
    }
}

#[cfg(test)]
mod tests {
    use super::{FetchError, canonical_log_id, queue_result};

    const LOG_ID: &str = "2017010100gm-00a9-0000-003dbd5d";

    #[test]
    fn extracts_log_ids_from_raw_ids_and_tenhou_urls() {
        assert_eq!(canonical_log_id(LOG_ID).unwrap(), LOG_ID);
        assert_eq!(
            canonical_log_id(&format!("https://tenhou.net/0/?log={LOG_ID}&tw=0")).unwrap(),
            LOG_ID
        );
    }

    #[test]
    fn rejects_text_without_a_canonical_log_id() {
        assert!(matches!(
            canonical_log_id("https://example.com/not-a-log"),
            Err(FetchError::InvalidLogId)
        ));
    }

    #[test]
    fn decodes_game_and_error_responses_from_the_queue() {
        let game = domain::parse_game(
            LOG_ID,
            include_str!(
                "../../../crates/domain/tests/fixtures/2017010100gm-00a9-0000-003dbd5d.xml"
            ),
        )
        .expect("fixture should parse");
        let body = serde_json::to_string(&game).expect("game should serialize");

        assert_eq!(queue_result(200, body).unwrap(), game);
        assert!(matches!(
            queue_result(422, String::new()),
            Err(FetchError::Unprocessable)
        ));
        assert!(matches!(
            queue_result(500, "cache failed".into()),
            Err(FetchError::Cache(message)) if message == "cache failed"
        ));
        assert!(matches!(
            queue_result(200, "not JSON".into()),
            Err(FetchError::Queue(_))
        ));
    }
}
