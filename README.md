<div align="center">

# suno

**Generate AI music from your terminal — full Suno v5.5 support**

<br />

[![Star this repo](https://img.shields.io/github/stars/199-biotechnologies/suno-cli?style=for-the-badge&logo=github&label=%E2%AD%90%20Star%20this%20repo&color=yellow)](https://github.com/199-biotechnologies/suno-cli/stargazers)
&nbsp;&nbsp;
[![Follow @longevityboris](https://img.shields.io/badge/Follow_%40longevityboris-000000?style=for-the-badge&logo=x&logoColor=white)](https://x.com/longevityboris)

<br />

[![License: MIT](https://img.shields.io/badge/License-MIT-blue?style=for-the-badge)](LICENSE)
&nbsp;
[![Rust](https://img.shields.io/badge/Rust-2024-orange?style=for-the-badge&logo=rust)](https://www.rust-lang.org/)
&nbsp;
[![crates.io](https://img.shields.io/crates/v/suno?style=for-the-badge)](https://crates.io/crates/suno)
&nbsp;
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen?style=for-the-badge)](https://github.com/199-biotechnologies/suno-cli/pulls)

---

A single Rust binary that talks directly to Suno's API. Generate songs with custom lyrics, style tags, your own voice persona, vocal control, weirdness/style sliders, covers, remasters, and every v5.5 feature. Zero-friction auth — one command extracts credentials from your browser automatically.

[Install](#install) | [Quick Start](#quick-start) | [Commands](#commands) | [Features](#features) | [Contributing](#contributing)

</div>

## Why

Suno has no official API. The web UI works, but you can't script it, pipe lyrics from a file, batch-generate, or integrate it into a music production workflow.

This CLI fixes that. Auto-auth from your browser, every generation parameter exposed as a flag, dual JSON/table output for both humans and AI agents. Downloads auto-embed synced lyrics into MP3 files.

## Install

### Homebrew (macOS/Linux)

```bash
brew tap 199-biotechnologies/tap
brew install suno
```

### Cargo (any platform)

```bash
cargo install suno
```

### Pre-built binaries

Download from [GitHub Releases](https://github.com/199-biotechnologies/suno-cli/releases) — binaries for macOS (Apple Silicon + Intel), Linux (x86_64 + ARM), and Windows.

### Self-update

Already have `suno` installed? Pull the latest binary from GitHub Releases without touching your package manager:

```bash
suno update --check    # see what's available
suno update            # install the latest release
```

> Tip: when Suno changes their API mid-cycle, run `suno update` first — it's faster than `cargo install suno` or waiting for the Homebrew bottle to refresh.

## Quick Start

```bash
# 1. Authenticate (auto-extracts from Chrome/Arc/Brave/Firefox/Edge)
suno auth --login

# 2. Check your credits
suno credits

# 3. Generate a song with full control
suno generate \
  --title "Weekend Code" \
  --tags "indie rock, guitar, upbeat" \
  --exclude "metal, heavy" \
  --lyrics-file lyrics.txt \
  --vocal male \
  --weirdness 40 \
  --style-influence 65 \
  --wait --download ./songs/

# 4. Generate with your voice persona
suno generate \
  --title "My Song" \
  --tags "pop, warm" \
  --persona e483d2f0-50ca-4a09-8a74-b9e074646377 \
  --lyrics "[Verse]\nHello from the CLI"

# 5. Let Suno write the lyrics for you
suno describe --prompt "a chill lo-fi track about rainy mornings" --wait
```

## Commands

### Create

```
suno generate        Custom mode — lyrics + tags + title + sliders + voice persona
suno describe        Description mode — Suno writes lyrics from your prompt
suno lyrics          Generate lyrics only (free, no credits)
suno extend          Continue a clip from a timestamp
suno concat          Stitch clips into a full song
suno cover           Create a cover with different style/model
suno remaster        Remaster with a different model version
suno stems           Extract vocals and instruments
```

### Browse & Inspect

```
suno list            List your songs
suno search <query>  Search songs by title or tags
suno info <id>       Detailed view of a single clip
suno persona <id>    View a voice persona
suno status <ids>    Check generation progress
suno credits         Show balance and plan info
suno models          List available models with limits
```

### Manage

```
suno download <ids>  Download audio/video with embedded lyrics
suno delete <ids>    Delete/trash clips
suno set <id>        Update title, lyrics, caption, or remove cover
suno publish <ids>   Toggle public/private visibility
suno timed-lyrics    Get word-level timestamped lyrics (--lrc for LRC format)
```

### Config & Auth

```
suno auth            Set up authentication
suno config          show | set | check
suno agent-info      Machine-readable capabilities JSON
suno install-skill   Install agent skill into Claude Code / Cursor
suno update          Self-update from GitHub Releases (--check to peek first)
```

## Features

### Zero-Friction Auth

```bash
suno auth --login    # Extracts session from your browser automatically
```

Reads the Clerk auth cookie from Chrome, Arc, Brave, Firefox, or Edge. Exchanges it for a JWT via Clerk token exchange. Auto-refreshes when expired (~7 day session lifetime). One macOS Keychain dialog on first run, then silent.

Three auth methods (in order of convenience):
1. `suno auth --login` — automatic browser extraction (recommended)
2. `suno auth --cookie <clerk_cookie>` — manual paste for headless servers
3. `suno auth --jwt <token>` — direct JWT, expires in ~1 hour

### Generation Parameters

| Flag | What it does | Values |
|---|---|---|
| `--title` | Song title | up to 100 chars |
| `--tags` | Style direction | `"pop, synths, upbeat"` (1000 chars) |
| `--exclude` | Styles to avoid | `"metal, heavy, dark"` (1000 chars) |
| `--lyrics` / `--lyrics-file` | Custom lyrics with `[Verse]` tags | up to 5000 chars |
| `--prompt` (describe) | Free text description | up to 500 chars |
| `--model` | Model version | v5.5, v5, v4.5+, v4.5, v4, v3.5, v3, v2 |
| `--vocal` | Vocal gender | male, female |
| `--persona` | Voice persona ID | UUID from Suno voice creation |
| `--weirdness` | How experimental | 0-100 |
| `--style-influence` | How strictly to follow tags | 0-100 |
| `--variation` | Output variation | high, normal, subtle |
| `--instrumental` | No vocals | flag |
| `--wait` | Block until done | flag |
| `--download <dir>` | Auto-download after generation | directory path |

### Voice Personas

Generate songs using your own voice. Create a voice in Suno's web UI, then use the persona ID:

```bash
# View persona details
suno persona <persona_id>

# Generate with your voice
suno generate --persona <persona_id> --title "My Song" --tags "pop" --lyrics "[Verse]\nHello world"

# Works with describe mode too
suno describe --persona <persona_id> --prompt "a warm ballad about starlight"
```

### Covers & Remasters

Create covers with different styles or remaster clips with newer models:

```bash
# Cover with different style tags
suno cover <clip_id> --tags "jazz, smooth piano" --model v5.5 --wait

# Remaster an old clip with the latest model
suno remaster <clip_id> --model v5.5 --wait --download ./remastered/
```

Both route through Suno's unified web generation endpoint (`/api/generate/v2-web/`).

### Clip Info

```bash
# Full details for any clip
suno info <clip_id>

# JSON for scripting
suno info <clip_id> --json | jq '.data.audio_url'
```

### Edit & Manage

```bash
# Update title and lyrics on an existing clip
suno set <clip_id> --title "New Title" --lyrics-file updated.txt

# Make clips public
suno publish <clip_id_1> <clip_id_2>

# Get timed lyrics in LRC format
suno timed-lyrics <clip_id> --lrc > song.lrc
```

### Downloads with Embedded Lyrics

Downloads automatically embed lyrics into MP3 files via ID3 tags:
- **USLT** (plain lyrics) — shown in most music players
- **SYLT** (synced word-by-word timestamps) — shown in Apple Music with timing

```bash
suno download <id1> <id2> --output ./songs/
```

Files use slug format: `title-slug-clipid8.mp3` — no overwrites when Suno generates 2 variations.

### Models

| Version | Codename | Default | Notes |
|---|---|---|---|
| **v5.5** | chirp-fenix | Yes | Latest, best quality |
| v5 | chirp-crow | | Previous generation |
| v4.5+ | chirp-bluejay | | Extended capabilities |
| v4.5 | chirp-auk | | Stable |
| v4 | chirp-v4 | | Legacy |

Remaster models: v5.5 = chirp-flounder, v5 = chirp-carp, v4.5+ = chirp-bass.

### Agent-Friendly

Every command supports `--json` for structured output. When stdout is piped, JSON is auto-detected. Progress and errors go to stderr. Exit codes are semantic:

| Code | Meaning | Agent action |
|---|---|---|
| 0 | Success | Continue |
| 1 | Runtime error (network, API) | Retry with backoff |
| 2 | Config error | Fix config, don't retry |
| 3 | Auth error | Run `suno auth --login` |
| 4 | Rate limited | Wait 30-60s, retry |
| 5 | Not found | Verify resource ID |

Error responses include actionable suggestions:

```json
{
  "version": "1",
  "status": "error",
  "error": {
    "code": "auth_expired",
    "message": "JWT expired — run `suno auth` to refresh",
    "suggestion": "Run `suno auth --login` to refresh your session"
  }
}
```

```bash
# Pipe-friendly: auto-JSON when piped
suno list | jq '.data[0].title'

# Agent capabilities discovery
suno agent-info
```

### Install as a Coding Agent Skill

Teach Claude Code (or Cursor) how to use `suno` with one command:

```bash
# Claude Code (~/.claude/skills/suno/SKILL.md)
suno install-skill

# Cursor (./.cursor/rules/suno.mdc in the current workspace)
suno install-skill --target cursor

# Print the skill content without writing
suno install-skill --print

# Custom path
suno install-skill --path ~/my-agents/suno.md --force
```

After installation, your coding agent automatically picks up the skill on the next session and knows how to invoke `suno` for music generation, downloads, stems, covers, and remasters.

### API Endpoint Versions (Confirmed)

| Endpoint | Version | Status |
|---|---|---|
| Feed | **v3** (`POST /api/feed/v3`) | Latest |
| Generate | **v2** (`POST /api/generate/v2/`) | Latest (only version) |
| Concat | **v2** (`POST /api/generate/concat/v2/`) | Latest |
| Aligned lyrics | **v2** (`GET /api/gen/{id}/aligned_lyrics/v2/`) | Latest |
| Persona | `GET /api/persona/get-persona-paginated/{id}/` | Confirmed |

All generation tasks (normal, voice persona, cover, extend) go through `/api/generate/v2/` with different `task` values.

## Contributing

1. Fork the repo
2. Create a branch (`git checkout -b feature/your-idea`)
3. Make your changes and test with `cargo test`
4. Open a PR

We especially welcome:
- Audio upload implementation (S3 presigned flow documented in `API_INTELLIGENCE.md`)
- Voice persona creation workflow (endpoints captured, request bodies needed)
- Integration tests with `assert_cmd`

## License

MIT — see [LICENSE](LICENSE).

---

<div align="center">

Built by [Boris Djordjevic](https://github.com/longevityboris) at [199 Biotechnologies](https://github.com/199-biotechnologies)

<br />

**If this saves you time:**

[![Star this repo](https://img.shields.io/github/stars/199-biotechnologies/suno-cli?style=for-the-badge&logo=github&label=%E2%AD%90%20Star%20this%20repo&color=yellow)](https://github.com/199-biotechnologies/suno-cli/stargazers)
&nbsp;&nbsp;
[![Follow @longevityboris](https://img.shields.io/badge/Follow_%40longevityboris-000000?style=for-the-badge&logo=x&logoColor=white)](https://x.com/longevityboris)

</div>
