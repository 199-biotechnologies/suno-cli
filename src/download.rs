use std::path::Path;

use futures_util::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};

use id3::TagLike;

use crate::api::types::{AlignedWord, Clip};
use crate::errors::CliError;

pub async fn download_clip(clip: &Clip, output_dir: &str, video: bool) -> Result<String, CliError> {
    let url = if video {
        clip.video_url
            .as_deref()
            .ok_or_else(|| CliError::Download("no video URL available".into()))?
    } else {
        clip.audio_url
            .as_deref()
            .ok_or_else(|| CliError::Download("no audio URL available".into()))?
    };

    let ext = if video { "mp4" } else { "mp3" };
    let slug: String = clip
        .title
        .to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '-' })
        .collect::<String>()
        .replace("--", "-")
        .trim_matches('-')
        .to_string();
    let short_id = &clip.id[..8.min(clip.id.len())];
    let filename = format!("{slug}-{short_id}.{ext}");
    let path = Path::new(output_dir).join(&filename);

    let client = reqwest::Client::new();
    let resp = client
        .get(url)
        .send()
        .await
        .map_err(CliError::Http)?
        .error_for_status()
        .map_err(CliError::Http)?;

    let total = resp.content_length().unwrap_or(0);
    let pb = ProgressBar::new(total);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{msg} [{bar:40}] {bytes}/{total_bytes} ({eta})")
            .unwrap_or_else(|_| ProgressStyle::default_bar())
            .progress_chars("=> "),
    );
    pb.set_message(filename.clone());

    let mut file = tokio::fs::File::create(&path).await?;
    let mut stream = resp.bytes_stream();
    while let Some(chunk) = stream.next().await {
        let chunk = chunk.map_err(CliError::Http)?;
        pb.inc(chunk.len() as u64);
        tokio::io::AsyncWriteExt::write_all(&mut file, &chunk).await?;
    }
    pb.finish_with_message("done");

    Ok(path.display().to_string())
}

/// Embed lyrics and metadata into an MP3 file using ID3v2 tags.
/// - USLT: unsynchronized (plain) lyrics — shown in most players
/// - SYLT: synchronized lyrics with word timestamps — shown in Apple Music, Spotify, etc.
/// - TIT2: title, TPE1: artist
pub fn embed_lyrics_in_mp3(
    mp3_path: &str,
    title: &str,
    plain_lyrics: Option<&str>,
    aligned_words: Option<&[AlignedWord]>,
) -> Result<(), CliError> {
    let mut tag = id3::Tag::read_from_path(mp3_path).unwrap_or_else(|_| id3::Tag::new());

    // Set title
    tag.set_title(title);

    // Plain lyrics (USLT) — shown in most players
    if let Some(lyrics) = plain_lyrics {
        tag.add_frame(id3::frame::Lyrics {
            lang: "eng".to_string(),
            description: String::new(),
            text: lyrics.to_string(),
        });
    }

    // Synchronized lyrics (SYLT) — timed word-by-word display
    if let Some(words) = aligned_words {
        let content: Vec<(u32, String)> = words
            .iter()
            .filter(|w| w.success)
            .map(|w| ((w.start_s * 1000.0) as u32, w.word.clone()))
            .collect();

        if !content.is_empty() {
            tag.add_frame(id3::frame::SynchronisedLyrics {
                lang: "eng".to_string(),
                timestamp_format: id3::frame::TimestampFormat::Ms,
                content_type: id3::frame::SynchronisedLyricsType::Lyrics,
                description: String::new(),
                content,
            });
        }
    }

    tag.write_to_path(mp3_path, id3::Version::Id3v24)
        .map_err(|e| CliError::Download(format!("failed to write ID3 tags: {e}")))?;

    Ok(())
}
