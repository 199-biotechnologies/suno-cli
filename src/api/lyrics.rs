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

        // Poll until complete (lyrics are fast, ~5-10 seconds)
        let timeout = std::time::Duration::from_secs(60);
        let start = std::time::Instant::now();

        loop {
            tokio::time::sleep(std::time::Duration::from_secs(2)).await;

            let resp = self
                .get(&format!("/api/generate/lyrics/{}", submit.id))
                .send()
                .await?;
            let resp = self.check_response(resp).await?;
            let result: LyricsResult = resp.json().await?;

            if result.status == "complete" || !result.error_message.is_empty() {
                return Ok(result);
            }
            if start.elapsed() > timeout {
                return Err(CliError::GenerationFailed(
                    "lyrics generation timed out".into(),
                ));
            }
        }
    }
}
