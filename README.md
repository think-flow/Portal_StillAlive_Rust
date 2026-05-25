# Portal Still Alive

A terminal-based animated "music video" for Portal's *Still Alive* song, written in Rust.

Uses ANSI escape sequences for terminal graphics. Runs on Windows (via `winmm.dll`) and Linux (via `mpg123`).

## Requirements

- **Windows 10+**: Legacy console mode is sufficient; VT escape codes are auto-detected
- **Linux**: Requires `mpg123` package for audio (`sudo apt install mpg123`)

## Build & Run

```bash
cargo run                        # With audio
cargo run -- --no-sound          # Without audio (for testing)
cargo build --release            # Optimized build (~1.5MB binary)
```

## Architecture

Single-threaded event loop driving three state machines (`LyricTyping`, `ArtState`, `CreditState`). No threads, no channels, no locks. See `architecture.html` for a detailed walkthrough.

## Controls

- **Ctrl+C**: Exit at any time
