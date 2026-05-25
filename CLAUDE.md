# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build & Run

```bash
cargo build              # Debug build
cargo build --release    # Release build (LTO, opt-level=z, strip, panic=abort)
cargo run                # Run in debug mode
cargo run -- --no-sound  # Run without audio playback
```

The build script (`build.rs`) copies `sa1.mp3` to the target directory. The binary expects `sa1.mp3` at runtime in its working directory.

## Architecture

A terminal-based animated "music video" for Portal's "Still Alive" song, using ANSI escape sequences for terminal graphics and platform-specific audio backends.

### Single-threaded event loop (`stage.rs` → `Stage::run()`)

All logic runs in one thread via a 10ms event loop. Three state machines are ticked cooperatively on each iteration — no threads, no `mpsc`, no `Cow`:

```
main loop @ 10ms:
  1. fire lyric events    →  start LyricTyping / ArtState / CreditState
  2. tick lyric typing    →  LyricTyping::tick()    (one char per tick)
  3. tick ascii art       →  ArtState::tick()       (one line per tick)
  4. tick credits         →  while is_ready() { CreditState::tick() }
  5. cursor refresh + sleep(10ms)
```

### Three state machines

| State machine | File | Role |
|---|---|---|
| `LyricTyping` | `stage/lyric.rs` | Types lyric lines character-by-character using `elapsed >= char_idx × interval` |
| `ArtState` | `stage/art.rs` | Draws ASCII art frames line-by-line at main-loop rate |
| `CreditState` | `stage/credit.rs` | Scrolls credit list with per-character timing + newline redraw |

Each state machine has `tick(&self, &Stage)` and `done() -> bool` — the main loop calls `tick()` until `done()`.

### Timeline system (`data/lyric.rs`)

The entire show is driven by a static array of 121 `Lyric` entries, each with a `time` (10ms units relative to start) and a `mode`:

| Mode | Action |
|------|--------|
| 0 | Type text character-by-character, then newline |
| 1 | Type text character-by-character, no newline (same-line continuation) |
| 2 | Display an ASCII art frame (indexed by `words` as integer) |
| 3 | Clear the entire lyric panel |
| 4 | Start audio playback (`player::play`) |
| 5 | Spawn credit state machine |

When `interval` is negative, the timing is calculated from the next lyric's `time`; otherwise, `interval` is the seconds-per-character delay.

### Audio (`player.rs`)

Platform-specific:
- **Windows**: FFI into `winmm.dll` (`mciSendStringW`) for MCI playback
- **Linux**: Spawns `mpg123 -q` as a subprocess, requires the `mpg123` package

### Terminal handling (`stage.rs`)

- Detects terminal type from `$TERM` (Unix) or `GetConsoleMode` (Windows)
- Checks for VT escape code support (VT >= 241 enables color)
- Uses alternate screen buffer on capable terminals (`\x1b[?1049h`/`\x1b[?1049l`)
- Requires minimum 80x24 terminal
- The `typed!` macro (defined in `lib.rs`) handles stdout output: `print!` + `flush`
- Use `typed!(if condition, ...)` to conditionally add newline with `println!`

### Static data

- `data::LYRICS` — `LazyLock<[Lyric; 121]>`, the timed script
- `data::CREDITS` — raw string of personnel names, newline-separated
- `data::ARTS` — `[[&str; 20]; 10]`, 10 frames of 20-line ASCII art (40 chars wide)

### After animation

The program waits for user input before exiting (`Press ENTER to exit...`). Ctrl+C is handled via `ctrlc` crate.

There are no tests in this project.
