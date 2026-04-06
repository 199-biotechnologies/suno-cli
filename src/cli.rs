use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(
    name = "suno",
    version,
    about = "Suno AI music generation CLI — v5.5 support"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Output JSON (auto-detected when piped)
    #[arg(long, global = true)]
    pub json: bool,

    /// Suppress non-essential output
    #[arg(long, global = true)]
    pub quiet: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Generate music with custom lyrics, tags, and controls
    Generate(GenerateArgs),

    /// Generate music from a text description (Suno writes lyrics)
    Inspire(InspireArgs),

    /// Generate lyrics only (free, no credits used)
    Lyrics(LyricsArgs),

    /// Continue/extend a clip from a timestamp
    Extend(ExtendArgs),

    /// Concatenate clips into a full song
    Concat(ConcatArgs),

    /// Create a cover of an existing clip
    Cover(CoverArgs),

    /// Remaster a clip with a different model
    Remaster(RemasterArgs),

    /// Extract stems (vocals, instruments) from a clip
    Stems(StemsArgs),

    /// List your songs
    Feed(FeedArgs),

    /// Check generation status
    Status(StatusArgs),

    /// Download audio/video for a clip
    Download(DownloadArgs),

    /// Show credit balance and plan info
    Credits,

    /// List available models
    Models,

    /// Set up authentication
    Auth(AuthArgs),

    /// Manage configuration
    Config(ConfigArgs),

    /// Machine-readable capabilities (for AI agents)
    AgentInfo,
}

#[derive(clap::Args)]
pub struct GenerateArgs {
    /// Song title
    #[arg(short, long)]
    pub title: Option<String>,

    /// Style tags (comma-separated): "pop, synths, upbeat"
    #[arg(long)]
    pub tags: Option<String>,

    /// Exclude styles (comma-separated): "metal, heavy"
    #[arg(long)]
    pub exclude: Option<String>,

    /// Lyrics text (with [Verse], [Chorus] tags)
    #[arg(short, long, conflicts_with = "lyrics_file")]
    pub lyrics: Option<String>,

    /// Read lyrics from file
    #[arg(long)]
    pub lyrics_file: Option<String>,

    /// Model version
    #[arg(short, long, default_value = "v5.5")]
    pub model: ModelVersion,

    /// Vocal gender
    #[arg(long)]
    pub vocal: Option<VocalGender>,

    /// Weirdness level (0-100)
    #[arg(long)]
    pub weirdness: Option<f64>,

    /// Style influence strength (0-100)
    #[arg(long)]
    pub style_influence: Option<f64>,

    /// Variation level
    #[arg(long)]
    pub variation: Option<VariationCategory>,

    /// Generate instrumental only (no vocals)
    #[arg(long)]
    pub instrumental: bool,

    /// Wait for generation to complete
    #[arg(short, long)]
    pub wait: bool,

    /// Download output to directory after generation
    #[arg(long)]
    pub download: Option<String>,

    /// Captcha token (if required)
    #[arg(long)]
    pub token: Option<String>,
}

#[derive(clap::Args)]
pub struct InspireArgs {
    /// Description of the song you want
    #[arg(short, long)]
    pub prompt: String,

    /// Style tags (optional, guides the generation)
    #[arg(long)]
    pub tags: Option<String>,

    /// Model version
    #[arg(short, long, default_value = "v5.5")]
    pub model: ModelVersion,

    /// Vocal gender
    #[arg(long)]
    pub vocal: Option<VocalGender>,

    /// Weirdness level (0-100)
    #[arg(long)]
    pub weirdness: Option<f64>,

    /// Style influence strength (0-100)
    #[arg(long)]
    pub style_influence: Option<f64>,

    /// Generate instrumental only
    #[arg(long)]
    pub instrumental: bool,

    /// Wait for generation to complete
    #[arg(short, long)]
    pub wait: bool,

    /// Download output to directory
    #[arg(long)]
    pub download: Option<String>,
}

#[derive(clap::Args)]
pub struct LyricsArgs {
    /// What the song should be about
    #[arg(short, long)]
    pub prompt: String,
}

#[derive(clap::Args)]
pub struct ExtendArgs {
    /// Clip ID to extend
    pub clip_id: String,

    /// Timestamp in seconds to continue from
    #[arg(long)]
    pub at: f64,

    /// New lyrics for the extension
    #[arg(long)]
    pub lyrics: Option<String>,

    /// Style tags
    #[arg(long)]
    pub tags: Option<String>,

    /// Wait for completion
    #[arg(short, long)]
    pub wait: bool,
}

#[derive(clap::Args)]
pub struct ConcatArgs {
    /// Clip ID to concatenate into a full song
    pub clip_id: String,

    /// Wait for completion
    #[arg(short, long)]
    pub wait: bool,
}

#[derive(clap::Args)]
pub struct CoverArgs {
    /// Clip ID to create a cover of
    pub clip_id: String,

    /// Style tags for the cover
    #[arg(long)]
    pub tags: Option<String>,

    /// Wait for completion
    #[arg(short, long)]
    pub wait: bool,
}

#[derive(clap::Args)]
pub struct RemasterArgs {
    /// Clip ID to remaster
    pub clip_id: String,

    /// Remaster model version
    #[arg(long, default_value = "v5.5")]
    pub model: RemasterModel,

    /// Wait for completion
    #[arg(short, long)]
    pub wait: bool,
}

#[derive(clap::Args)]
pub struct StemsArgs {
    /// Clip ID to extract stems from
    pub clip_id: String,

    /// Wait for completion
    #[arg(short, long)]
    pub wait: bool,
}

#[derive(clap::Args)]
pub struct FeedArgs {
    /// Page number (0-indexed)
    #[arg(short, long, default_value = "0")]
    pub page: u32,
}

#[derive(clap::Args)]
pub struct StatusArgs {
    /// Clip ID(s) to check
    pub ids: Vec<String>,
}

#[derive(clap::Args)]
pub struct DownloadArgs {
    /// Clip ID to download
    pub id: String,

    /// Output directory
    #[arg(short, long, default_value = ".")]
    pub output: String,

    /// Download video instead of audio
    #[arg(long)]
    pub video: bool,
}

#[derive(clap::Args)]
pub struct AuthArgs {
    /// JWT token from browser DevTools
    #[arg(long)]
    pub jwt: Option<String>,

    /// Browser cookie string (for persistent auth)
    #[arg(long)]
    pub cookie: Option<String>,

    /// Clerk session ID
    #[arg(long)]
    pub session: Option<String>,

    /// Device ID
    #[arg(long)]
    pub device: Option<String>,
}

#[derive(clap::Args)]
pub struct ConfigArgs {
    #[command(subcommand)]
    pub action: ConfigAction,
}

#[derive(Subcommand)]
pub enum ConfigAction {
    /// Show current configuration
    Show,
    /// Set a configuration value
    Set { key: String, value: String },
    /// Validate configuration
    Check,
}

#[derive(ValueEnum, Clone, Debug, Default)]
pub enum ModelVersion {
    #[value(name = "v5.5")]
    #[default]
    V55,
    #[value(name = "v5")]
    V5,
    #[value(name = "v4.5+")]
    V45Plus,
    #[value(name = "v4.5")]
    V45,
    #[value(name = "v4")]
    V4,
    #[value(name = "v3.5")]
    V35,
    #[value(name = "v3")]
    V3,
    #[value(name = "v2")]
    V2,
}

impl ModelVersion {
    pub fn to_api_key(&self) -> &'static str {
        match self {
            Self::V55 => "chirp-fenix",
            Self::V5 => "chirp-crow",
            Self::V45Plus => "chirp-bluejay",
            Self::V45 => "chirp-auk",
            Self::V4 => "chirp-v4",
            Self::V35 => "chirp-v3-5",
            Self::V3 => "chirp-v3-0",
            Self::V2 => "chirp-v2-xxl-alpha",
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            Self::V55 => "v5.5",
            Self::V5 => "v5",
            Self::V45Plus => "v4.5+",
            Self::V45 => "v4.5",
            Self::V4 => "v4",
            Self::V35 => "v3.5",
            Self::V3 => "v3",
            Self::V2 => "v2",
        }
    }
}

#[derive(ValueEnum, Clone, Debug)]
pub enum VocalGender {
    Male,
    Female,
}

#[derive(ValueEnum, Clone, Debug)]
pub enum VariationCategory {
    High,
    Normal,
    Subtle,
}

impl VariationCategory {
    pub fn to_api_value(&self) -> &'static str {
        match self {
            Self::High => "high",
            Self::Normal => "normal",
            Self::Subtle => "subtle",
        }
    }
}

#[derive(ValueEnum, Clone, Debug, Default)]
pub enum RemasterModel {
    #[value(name = "v5.5")]
    #[default]
    V55,
    #[value(name = "v5")]
    V5,
    #[value(name = "v4.5+")]
    V45Plus,
}

impl RemasterModel {
    pub fn to_api_key(&self) -> &'static str {
        match self {
            Self::V55 => "chirp-flounder",
            Self::V5 => "chirp-carp",
            Self::V45Plus => "chirp-bass",
        }
    }
}
