use thiserror::Error;

#[derive(Error, Debug)]
pub enum CliError {
    #[error("API error: {message}")]
    Api { code: &'static str, message: String },

    #[error("Authentication required — run `suno auth` first")]
    AuthMissing,

    #[error("JWT expired — run `suno auth` to refresh")]
    AuthExpired,

    #[error("Rate limited by Suno — wait and retry")]
    RateLimited,

    #[error("Generation failed: {0}")]
    GenerationFailed(String),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Download failed: {0}")]
    Download(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error(transparent)]
    Http(#[from] reqwest::Error),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Json(#[from] serde_json::Error),
}

impl CliError {
    pub fn exit_code(&self) -> i32 {
        match self {
            Self::Config(_) => 2,
            Self::AuthMissing | Self::AuthExpired => 3,
            Self::RateLimited => 4,
            Self::NotFound(_) => 5,
            Self::Api { .. }
            | Self::Http(_)
            | Self::GenerationFailed(_)
            | Self::Download(_)
            | Self::Io(_)
            | Self::Json(_) => 1,
        }
    }

    pub fn error_code(&self) -> &'static str {
        match self {
            Self::Api { code, .. } => code,
            Self::AuthMissing => "auth_missing",
            Self::AuthExpired => "auth_expired",
            Self::RateLimited => "rate_limited",
            Self::Config(_) => "config_error",
            Self::GenerationFailed(_) => "generation_failed",
            Self::Download(_) => "download_error",
            Self::NotFound(_) => "not_found",
            Self::Http(_) => "http_error",
            Self::Io(_) => "io_error",
            Self::Json(_) => "json_error",
        }
    }

    pub fn suggestion(&self) -> &'static str {
        match self {
            Self::AuthMissing => "Run `suno auth --login` to authenticate",
            Self::AuthExpired => "Run `suno auth --login` to refresh your session",
            Self::RateLimited => "Wait 30-60 seconds and retry",
            Self::Config(_) => "Check `suno config check` for configuration issues",
            Self::NotFound(_) => "Verify the ID exists with `suno list` or `suno search`",
            Self::Download(_) => {
                "Check that the clip has finished generating with `suno status <id>`"
            }
            Self::GenerationFailed(_) => "Check `suno credits` for remaining balance",
            Self::Api { .. } | Self::Http(_) => "Check your network connection and retry",
            Self::Io(_) => "Check file permissions and disk space",
            Self::Json(_) => "This may indicate an API change — update suno",
        }
    }
}
