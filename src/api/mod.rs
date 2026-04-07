pub mod billing;
pub mod concat;
pub mod cover;
pub mod delete;
pub mod feed;
pub mod generate;
pub mod lyrics;
pub mod metadata;
pub mod persona;
pub mod remaster;
pub mod stems;
pub mod types;

use reqwest::Client;

use crate::auth::{self, AuthState};
use crate::errors::CliError;

pub struct SunoClient {
    client: Client,
    auth: AuthState,
}

const BASE_URL: &str = "https://studio-api-prod.suno.com";

impl SunoClient {
    /// Create a new client. If JWT is expired but we have a Clerk cookie,
    /// auto-refresh the JWT transparently.
    pub async fn new_with_refresh(mut auth: AuthState) -> Result<Self, CliError> {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .user_agent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/146.0.0.0 Safari/537.36")
            .build()
            .map_err(|e| CliError::Config(format!("HTTP client: {e}")))?;

        if auth.is_jwt_expired() {
            // Try auto-refresh via Clerk cookie
            if let (Some(cookie), Some(session_id)) = (&auth.clerk_client_cookie, &auth.session_id)
            {
                eprintln!("JWT expired, refreshing via Clerk...");
                match auth::clerk_refresh_jwt(&client, cookie, session_id).await {
                    Ok(jwt) => {
                        auth.jwt = Some(jwt);
                        auth.save()?;
                        eprintln!("JWT refreshed successfully");
                    }
                    Err(e) => {
                        eprintln!("JWT refresh failed: {e}");
                        return Err(CliError::AuthExpired);
                    }
                }
            } else {
                return Err(CliError::AuthExpired);
            }
        }

        Ok(Self { client, auth })
    }

    pub(crate) fn get(&self, path: &str) -> reqwest::RequestBuilder {
        self.client
            .get(format!("{BASE_URL}{path}"))
            .headers(self.headers())
    }

    pub(crate) fn post(&self, path: &str) -> reqwest::RequestBuilder {
        self.client
            .post(format!("{BASE_URL}{path}"))
            .headers(self.headers())
    }

    fn headers(&self) -> reqwest::header::HeaderMap {
        let mut headers = reqwest::header::HeaderMap::new();
        if let Ok(jwt) = self.auth.jwt()
            && let Ok(val) = format!("Bearer {jwt}").parse()
        {
            headers.insert("authorization", val);
        }
        if let Ok(val) = self.auth.device_id().parse() {
            headers.insert("device-id", val);
        }
        if let Ok(val) = auth::browser_token().parse() {
            headers.insert("browser-token", val);
        }
        if let Ok(val) = "https://suno.com".parse() {
            headers.insert("origin", val);
        }
        if let Ok(val) = "https://suno.com/".parse() {
            headers.insert("referer", val);
        }
        headers
    }

    pub async fn check_response(
        &self,
        resp: reqwest::Response,
    ) -> Result<reqwest::Response, CliError> {
        let status = resp.status();
        if status == 401 {
            return Err(CliError::AuthExpired);
        }
        if status == 403 {
            let body = resp.text().await.unwrap_or_default();
            return Err(CliError::Api {
                code: "forbidden",
                message: format!("HTTP 403 Forbidden: {body}"),
            });
        }
        if status == 429 {
            return Err(CliError::RateLimited);
        }
        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            return Err(CliError::Api {
                code: "api_error",
                message: format!("HTTP {status}: {body}"),
            });
        }
        Ok(resp)
    }
}
