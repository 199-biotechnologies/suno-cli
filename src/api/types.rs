use serde::{Deserialize, Serialize};

// --- Billing / Account ---

#[derive(Debug, Deserialize, Serialize)]
pub struct BillingInfo {
    pub credits: u64,
    pub total_credits_left: u64,
    pub monthly_usage: u64,
    pub monthly_limit: u64,
    pub is_active: bool,
    pub plan: Plan,
    pub models: Vec<Model>,
    pub period: String,
    pub renews_on: Option<String>,
    #[serde(default)]
    pub remaster_model_types: Vec<RemasterModelInfo>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Plan {
    pub name: String,
    pub plan_key: String,
    #[serde(default)]
    pub usage_plan_features: Vec<Feature>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Feature {
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Model {
    pub name: String,
    pub external_key: String,
    pub can_use: bool,
    pub is_default_model: bool,
    pub description: String,
    #[serde(default)]
    pub max_lengths: MaxLengths,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct MaxLengths {
    #[serde(default)]
    pub title: u32,
    #[serde(default)]
    pub prompt: u32,
    #[serde(default)]
    pub tags: u32,
    #[serde(default)]
    pub negative_tags: u32,
    #[serde(default)]
    pub gpt_description_prompt: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RemasterModelInfo {
    pub name: String,
    pub external_key: String,
    pub is_default_model: bool,
    pub can_use: bool,
}

// --- Clips / Feed ---

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Clip {
    pub id: String,
    pub title: String,
    pub status: String,
    pub model_name: String,
    pub audio_url: Option<String>,
    pub video_url: Option<String>,
    pub image_url: Option<String>,
    pub created_at: String,
    #[serde(default)]
    pub play_count: u64,
    #[serde(default)]
    pub upvote_count: u64,
    #[serde(default)]
    pub metadata: ClipMetadata,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct ClipMetadata {
    pub tags: Option<String>,
    pub prompt: Option<String>,
    pub duration: Option<f64>,
    pub avg_bpm: Option<f64>,
    #[serde(default)]
    pub has_stem: bool,
    #[serde(default)]
    pub is_remix: bool,
    #[serde(default)]
    pub make_instrumental: bool,
    #[serde(rename = "type")]
    pub clip_type: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct FeedResponse {
    #[serde(default)]
    pub clips: Vec<Clip>,
    #[allow(dead_code)]
    pub next_cursor: Option<String>,
    #[serde(default)]
    #[allow(dead_code)]
    pub has_more: bool,
}

// --- Feed V3 Request ---

#[derive(Debug, Serialize)]
pub struct FeedV3Request {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<FeedFilters>,
}

#[derive(Debug, Serialize)]
pub struct FeedFilters {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "searchText")]
    pub search_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trashed: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "fullSong")]
    pub full_song: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stem: Option<FilterPresence>,
}

#[derive(Debug, Serialize)]
pub struct FilterPresence {
    pub presence: String,
}

// --- Generation ---

#[derive(Debug, Serialize)]
pub struct GenerateRequest {
    pub mv: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gpt_description_prompt: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub negative_tags: Option<String>,
    pub make_instrumental: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generation_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continue_clip_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continue_at: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task: Option<String>,
    /// Voice persona ID — used with task="vox"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub persona_id: Option<String>,
    /// Source clip for covers/remasters — used with task="cover"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cover_clip_id: Option<String>,
    /// Control sliders — nested correctly under metadata per xiliourt/Suno-Architect
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<GenerateMetadata>,
}

#[derive(Debug, Serialize)]
pub struct GenerateMetadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_sliders: Option<ControlSliders>,
}

#[derive(Debug, Serialize)]
pub struct ControlSliders {
    /// Weirdness: 0.0-1.0 (maps from 0-100 in UI)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weirdness_constraint: Option<f64>,
    /// Style weight: 0.0-1.0 (maps from 0-90 in UI)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style_weight: Option<f64>,
}

#[derive(Debug, Deserialize)]
pub struct GenerateResponse {
    #[serde(default)]
    pub clips: Vec<Clip>,
    #[allow(dead_code)]
    pub status: Option<String>,
}

// --- Lyrics ---

#[derive(Debug, Deserialize)]
pub struct LyricsSubmitResponse {
    pub id: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LyricsResult {
    pub text: String,
    pub title: String,
    pub status: String,
    #[serde(default)]
    pub error_message: String,
    #[serde(default)]
    pub tags: Vec<String>,
}

// --- Aligned / Timed Lyrics ---

#[derive(Debug, Deserialize, Serialize)]
pub struct AlignedWord {
    pub word: String,
    pub start_s: f64,
    pub end_s: f64,
    #[serde(default)]
    pub success: bool,
    #[serde(default)]
    pub p_align: Option<f64>,
}

// --- Captcha Check ---

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct CaptchaCheckResponse {
    #[serde(default)]
    pub captcha_required: bool,
    #[serde(default)]
    pub captcha_url: Option<String>,
}

// --- Set Metadata ---

#[derive(Debug, Serialize)]
pub struct SetMetadataRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lyrics: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_image_cover: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_video_cover: Option<bool>,
}

// --- Set Visibility ---

#[derive(Debug, Serialize)]
pub struct SetVisibilityRequest {
    pub is_public: bool,
}

// --- Concat ---

#[derive(Debug, Serialize)]
pub struct ConcatRequest {
    pub clip_id: String,
}

// --- Persona ---

#[derive(Debug, Deserialize, Serialize)]
pub struct PersonaResponse {
    pub persona: PersonaInfo,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PersonaInfo {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub image_s3_id: Option<String>,
    #[serde(default)]
    pub user_display_name: Option<String>,
    #[serde(default)]
    pub user_handle: Option<String>,
    #[serde(default)]
    pub persona_clips: Vec<serde_json::Value>,
}
