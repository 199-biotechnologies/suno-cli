---
name: suno
description: Generate AI music from the terminal using the `suno` CLI. Use when user asks to "generate a song", "make music", "create AI music", "make a track", "generate audio", or wants to programmatically use Suno for custom lyrics, tags, voice personas, covers, remasters, or stems. Also use when downloading Suno songs (auto-embeds lyrics into MP3). Run `suno agent-info` for the full machine-readable capability dump. NOT for writing song prompts/lyrics without generating audio — use `suno-song-generator` for that.
---

# suno CLI

Generate AI music from your terminal. Full Suno v5.5 API access including custom lyrics, style tags, voice personas, covers, remasters, stems extraction, and word-level timed lyrics.

## When to use

- User wants to **generate** AI music programmatically with the Suno API
- User wants to **download** Suno songs (auto-embeds USLT + SYLT lyrics into MP3 ID3 tags)
- User wants to **batch generate**, **script**, or **integrate** Suno into a music workflow
- User mentions Suno, AI music generation, or wants to control Suno parameters from the terminal

## When NOT to use

- Writing song lyrics or Suno-formatted prompts without actually generating audio → use the `suno-song-generator` skill instead
- General music theory, composition advice, or non-Suno music tasks

## Setup (first time on a new machine)

```bash
# Auto-extract auth from browser (Chrome / Arc / Brave / Firefox / Edge)
suno auth --login

# Verify it worked
suno credits
```

If `suno auth --login` fails on a headless box, fall back to:

```bash
suno auth --cookie '<clerk __client cookie>'   # paste from browser DevTools
suno auth --jwt '<jwt>'                        # ~1 hour lifetime
```

## Discovery

Always start by reading machine-readable capabilities:

```bash
suno agent-info        # JSON: commands, models, exit codes, features, env prefix
suno --help            # full subcommand list
suno <cmd> --help      # flags for a specific subcommand
```

## Core commands

```bash
# Generate with full control (custom mode)
suno generate \
  --title "Weekend Code" \
  --tags "indie rock, guitar, upbeat" \
  --exclude "metal, heavy" \
  --lyrics-file lyrics.txt \
  --vocal male \
  --weirdness 40 \
  --style-influence 65 \
  --wait --download ./songs/

# Generate from a free-text description (Suno writes the lyrics)
suno describe --prompt "a chill lo-fi track about rainy mornings" --wait --download ./

# Lyrics only — FREE, uses no credits
suno lyrics --prompt "song about coffee at sunrise"

# Generate using a voice persona (your own voice)
suno generate \
  --persona e483d2f0-50ca-4a09-8a74-b9e074646377 \
  --title "My Song" --tags "pop, warm" \
  --lyrics "[Verse]\nHello from the CLI"

# Inspect a specific clip
suno info <clip_id>
suno persona <persona_id>

# List / search your library
suno list
suno search "rainy"

# Cover or remaster an existing clip
suno cover <clip_id> --tags "jazz, smooth piano" --model v5.5 --wait
suno remaster <clip_id> --model v5.5 --wait --download ./remastered/

# Extract stems (vocals + instruments)
suno stems <clip_id> --wait

# Word-level timed lyrics (LRC format for synced display)
suno timed-lyrics <clip_id> --lrc > song.lrc

# Download with auto-embedded synced lyrics
suno download <clip_id_1> <clip_id_2> --output ./songs/

# Manage clips
suno set <clip_id> --title "New Title" --lyrics-file updated.txt
suno publish <clip_id_1> <clip_id_2>          # make public
suno publish <clip_id_1> --private            # make private
suno delete <clip_id> -y

# Account
suno credits
suno models
```

## Generation parameters reference

| Flag | What it does | Range / format |
|---|---|---|
| `--title` | Song title | ≤ 100 chars |
| `--tags` | Style direction | "pop, synths, upbeat" (≤ 1000 chars) |
| `--exclude` | Styles to avoid | "metal, heavy, dark" (≤ 1000 chars) |
| `--lyrics` / `--lyrics-file` | Custom lyrics with `[Verse]` `[Chorus]` tags | ≤ 5000 chars |
| `--prompt` (describe mode) | Free-text description | ≤ 500 chars |
| `--model` | Model version | v5.5 (default), v5, v4.5+, v4.5, v4 |
| `--vocal` | Vocal gender | male, female |
| `--persona` | Voice persona UUID | from Suno voice creation |
| `--weirdness` | How experimental | 0–100 |
| `--style-influence` | How strictly to follow tags | 0–100 |
| `--variation` | Output variation | high, normal, subtle |
| `--instrumental` | No vocals | flag |
| `--wait` | Block until generation completes | flag |
| `--download <dir>` | Auto-download after generation | directory path |

## Models

| Version | Codename | Notes |
|---|---|---|
| **v5.5** | chirp-fenix | Default, latest, best quality |
| v5 | chirp-crow | Previous gen |
| v4.5+ | chirp-bluejay | Extended capabilities |
| v4.5 | chirp-auk | Stable |
| v4 | chirp-v4 | Legacy |

Remaster models: v5.5 = chirp-flounder, v5 = chirp-carp, v4.5+ = chirp-bass.

## Agent-friendly output

- Every command supports `--json`. JSON is **auto-detected** when stdout is piped.
- Progress messages and errors go to **stderr** so they don't pollute JSON pipelines.
- Errors include actionable suggestions in the JSON envelope.

```bash
# Pipe-friendly: auto-JSON
suno list | jq '.data[0].title'
suno info <clip_id> --json | jq '.data.audio_url'
```

## Exit codes (semantic)

| Code | Meaning | What the agent should do |
|---|---|---|
| 0 | Success | Continue |
| 1 | Transient (network, API) | Retry with backoff |
| 2 | Config error | Fix config, do not retry blindly |
| 3 | Auth error | Run `suno auth --login` |
| 4 | Rate limited | Wait 30–60s, then retry |
| 5 | Not found | Verify the resource ID |

## Common workflows

### Generate and download a finished MP3 with synced lyrics

```bash
suno generate --title "Foo" --tags "ambient, piano" --lyrics-file foo.txt --wait --download ./out/
# → ./out/foo-<clipid8>.mp3 with USLT + SYLT lyrics embedded
```

### Resume / continue a clip

```bash
suno extend <clip_id> --at 60.0 --lyrics "[Verse 2]\n..." --wait
suno concat <new_clip_id>           # stitch into a full song
```

### Batch download yesterday's songs as JSON, then pull MP3s

```bash
ids=$(suno list --json | jq -r '.data[].id')
suno download $ids --output ./archive/
```

## Notes

- Auth refreshes automatically (~7-day session lifetime).
- Captcha is **not** required for Premier accounts with 200+ credits consumed.
- All generation paths (normal, voice persona, cover, extend) go through `/api/generate/v2-web/` — but you don't need to know that, just use the subcommands.
- When the CLI returns `schema_drift` (Suno changed their API), run `suno update` to pull the latest binary from GitHub Releases.
- When unsure about flags, run `suno <command> --help` or `suno agent-info`.
