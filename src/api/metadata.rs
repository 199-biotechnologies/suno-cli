use super::SunoClient;
use super::types::{SetMetadataRequest, SetVisibilityRequest};
use crate::errors::CliError;

impl SunoClient {
    /// Update clip metadata (title, lyrics, caption, cover image).
    pub async fn set_metadata(
        &self,
        clip_id: &str,
        req: &SetMetadataRequest,
    ) -> Result<(), CliError> {
        let resp = self
            .post(&format!("/api/gen/{clip_id}/set_metadata/"))
            .json(req)
            .send()
            .await?;
        self.check_response(resp).await?;
        Ok(())
    }

    /// Set clip visibility (public/private).
    pub async fn set_visibility(&self, clip_id: &str, is_public: bool) -> Result<(), CliError> {
        let resp = self
            .post(&format!("/api/gen/{clip_id}/set_visibility/"))
            .json(&SetVisibilityRequest { is_public })
            .send()
            .await?;
        self.check_response(resp).await?;
        Ok(())
    }

    /// Get word-level timestamped lyrics for a clip.
    pub async fn aligned_lyrics(
        &self,
        clip_id: &str,
    ) -> Result<Vec<super::types::AlignedWord>, CliError> {
        let resp = self
            .get(&format!("/api/gen/{clip_id}/aligned_lyrics/v2/"))
            .send()
            .await?;
        let resp = self.check_response(resp).await?;
        let body: serde_json::Value = resp.json().await?;
        // API returns {"aligned_words": [...], ...} — extract the array
        let words = body.get("aligned_words").ok_or_else(|| CliError::Api {
            code: "missing_field",
            message: "aligned_lyrics response missing 'aligned_words' field".into(),
        })?;
        Ok(serde_json::from_value(words.clone())?)
    }

    /// Check whether captcha is required before generation.
    pub async fn check_captcha(&self) -> Result<bool, CliError> {
        let resp = self
            .post("/api/c/check")
            .json(&serde_json::json!({"ctype": "generation"}))
            .send()
            .await?;
        let resp = self.check_response(resp).await?;
        // Try to parse; if the response doesn't have captcha_required, assume false
        let body: serde_json::Value = resp.json().await?;
        Ok(body
            .get("captcha_required")
            .and_then(|v| v.as_bool())
            .unwrap_or(false))
    }
}
