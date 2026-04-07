use serde_json::json;

use super::SunoClient;
use super::types::{LyricsResult, LyricsSubmitResponse};
use crate::errors::CliError;

impl SunoClient {
    /// Submit lyrics generation and poll until complete.
    pub async fn generate_lyrics(&self, prompt: &str) -> Result<LyricsResult, CliError> {
        let resp = self
            .post("/api/generate/lyrics/")
            .json(&json!({ "prompt": prompt }))
            .send()
            .await?;
        let resp = self.check_response(resp).await?;
        let submit: LyricsSubmitResponse = resp.json().await?;

        let timeout = std::time::Duration::from_secs(60);
        let start = std::time::Instant::now();
        let mut delay = std::time::Duration::from_secs(2);

        loop {
            tokio::time::sleep(delay).await;

            let resp = self
                .get(&format!("/api/generate/lyrics/{}", submit.id))
                .send()
                .await?;
            let resp = self.check_response(resp).await?;
            let result: LyricsResult = resp.json().await?;

            if !result.error_message.is_empty() {
                return Err(CliError::GenerationFailed(result.error_message));
            }
            if result.status == "complete" {
                return Ok(result);
            }
            if start.elapsed() > timeout {
                return Err(CliError::GenerationFailed(
                    "lyrics generation timed out".into(),
                ));
            }
            delay = (delay * 2).min(std::time::Duration::from_secs(8));
        }
    }
}
