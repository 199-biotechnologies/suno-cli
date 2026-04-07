use base64::Engine;
use base64::engine::general_purpose::STANDARD as BASE64;
use base64::engine::general_purpose::URL_SAFE_NO_PAD as BASE64URL;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::errors::CliError;

const CLERK_BASE: &str = "https://auth.suno.com";
const CLERK_JS_VERSION: &str = "5.117.0";

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct AuthState {
    pub jwt: Option<String>,
    pub cookie: Option<String>,
    pub session_id: Option<String>,
    pub device_id: Option<String>,
    /// The __client cookie from clerk domain — long-lived (~7 days)
    pub clerk_client_cookie: Option<String>,
}

impl AuthState {
    pub fn load() -> Result<Self, CliError> {
        let path = Self::path();
        if !path.exists() {
            return Err(CliError::AuthMissing);
        }
        let data = std::fs::read_to_string(&path)?;
        serde_json::from_str(&data).map_err(|e| CliError::Config(format!("corrupt auth file: {e}")))
    }

    pub fn save(&self) -> Result<(), CliError> {
        let path = Self::path();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let data = serde_json::to_string_pretty(self)?;

        // Atomic write: create temp file with restricted permissions, then rename
        let tmp = path.with_extension("json.tmp");

        #[cfg(unix)]
        {
            use std::io::Write;
            use std::os::unix::fs::OpenOptionsExt;
            let mut file = std::fs::OpenOptions::new()
                .create(true)
                .truncate(true)
                .write(true)
                .mode(0o600)
                .open(&tmp)?;
            file.write_all(data.as_bytes())?;
            file.sync_all()?;
        }

        #[cfg(not(unix))]
        {
            std::fs::write(&tmp, &data)?;
        }

        std::fs::rename(&tmp, &path)?;
        Ok(())
    }

    pub fn jwt(&self) -> Result<&str, CliError> {
        self.jwt.as_deref().ok_or(CliError::AuthMissing)
    }

    pub fn device_id(&self) -> &str {
        self.device_id
            .as_deref()
            .unwrap_or("00000000-0000-0000-0000-000000000000")
    }

    pub fn is_jwt_expired(&self) -> bool {
        let Some(jwt) = &self.jwt else { return true };
        let parts: Vec<&str> = jwt.split('.').collect();
        if parts.len() != 3 {
            return true;
        }
        let claims = parts[1];
        // JWT claims use Base64URL encoding, not standard Base64
        let Ok(decoded) = BASE64URL.decode(claims) else {
            return true;
        };
        let Ok(value) = serde_json::from_slice::<serde_json::Value>(&decoded) else {
            return true;
        };
        let Some(exp) = value.get("exp").and_then(|v| v.as_u64()) else {
            return true;
        };
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        // Expired or within 30 seconds of expiry
        now + 30 >= exp
    }

    fn path() -> PathBuf {
        directories::ProjectDirs::from("com", "suno-cli", "suno-cli")
            .map(|dirs| dirs.config_dir().join("auth.json"))
            .unwrap_or_else(|| PathBuf::from("~/.config/suno-cli/auth.json"))
    }
}

/// Generate the dynamic browser-token header value.
pub fn browser_token() -> String {
    let ms = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis();
    let payload = format!(r#"{{"timestamp":{ms}}}"#);
    let encoded = BASE64.encode(payload.as_bytes());
    format!(r#"{{"token":"{encoded}"}}"#)
}

/// Extract the __client cookie for auth.suno.com from the user's browsers.
/// Tries Chrome, Firefox, Safari, Arc, Brave, Edge in order.
pub fn extract_clerk_cookie() -> Result<String, CliError> {
    let domains = vec!["auth.suno.com".into(), ".suno.com".into()];

    for (name, result) in [
        ("Chrome", rookie::chrome(Some(domains.clone()))),
        ("Arc", rookie::arc(Some(domains.clone()))),
        ("Brave", rookie::brave(Some(domains.clone()))),
        ("Firefox", rookie::firefox(Some(domains.clone()))),
        ("Edge", rookie::edge(Some(domains.clone()))),
    ] {
        if let Ok(cookies) = result
            && let Some(cookie) = cookies
                .into_iter()
                .find(|c| c.name == "__client" && !c.value.is_empty())
        {
            eprintln!("Found Suno session in {name}");
            return Ok(cookie.value);
        }
    }

    Err(CliError::Config(
        "No Suno session found in any browser. Log into suno.com first, then retry.".into(),
    ))
}

/// Exchange the __client cookie for a session ID and JWT via Clerk.
pub async fn clerk_token_exchange(
    client: &reqwest::Client,
    clerk_cookie: &str,
) -> Result<(String, String), CliError> {
    // Step 1: Get session ID
    let resp = client
        .get(format!(
            "{CLERK_BASE}/v1/client?_clerk_js_version={CLERK_JS_VERSION}"
        ))
        .header("cookie", format!("__client={clerk_cookie}"))
        .send()
        .await
        .map_err(CliError::Http)?;

    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        return Err(CliError::Api {
            code: "clerk_exchange_failed",
            message: format!("Clerk token exchange failed ({status}): {body}"),
        });
    }

    let body: serde_json::Value = resp.json().await.map_err(CliError::Http)?;
    let session_id = body
        .get("response")
        .and_then(|r| r.get("last_active_session_id"))
        .and_then(|s| s.as_str())
        .ok_or_else(|| CliError::Api {
            code: "no_session",
            message: "No active session found — log into suno.com in your browser first".into(),
        })?
        .to_string();

    // Step 2: Exchange for JWT
    let jwt = clerk_refresh_jwt(client, clerk_cookie, &session_id).await?;

    Ok((session_id, jwt))
}

/// Refresh JWT using stored Clerk cookie + session ID.
pub async fn clerk_refresh_jwt(
    client: &reqwest::Client,
    clerk_cookie: &str,
    session_id: &str,
) -> Result<String, CliError> {
    let resp = client
        .post(format!(
            "{CLERK_BASE}/v1/client/sessions/{session_id}/tokens?_clerk_js_version={CLERK_JS_VERSION}"
        ))
        .header("cookie", format!("__client={clerk_cookie}"))
        .header("content-type", "application/x-www-form-urlencoded")
        .send()
        .await
        .map_err(CliError::Http)?;

    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        return Err(CliError::Api {
            code: "clerk_refresh_failed",
            message: format!("Clerk JWT refresh failed ({status}): {body}"),
        });
    }

    let body: serde_json::Value = resp.json().await.map_err(CliError::Http)?;
    body.get("jwt")
        .and_then(|j| j.as_str())
        .map(String::from)
        .ok_or_else(|| CliError::Api {
            code: "no_jwt",
            message:
                "Clerk returned no JWT — session may have expired, run `suno auth login` again"
                    .into(),
        })
}
