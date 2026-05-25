mod art;
mod credit;
mod lyric;

use crate::typed;
use anyhow::bail;
use std::{
    env,
    path::Path,
    sync::atomic::{AtomicBool, Ordering},
    thread,
    time::{Duration, Instant},
};

const SOUND_FILE_PATH: &str = "./sa1.mp3";

#[cfg(target_os = "windows")]
#[link(name = "kernel32")]
unsafe extern "system" {
    unsafe fn GetStdHandle(nStdHandle: u32) -> std::os::windows::raw::HANDLE;
    unsafe fn GetConsoleMode(
        hConsoleHandle: std::os::windows::raw::HANDLE,
        lpMode: *mut u32,
    ) -> i32;
}

#[cfg(target_os = "windows")]
fn check_support() -> bool {
    unsafe {
        let handle = GetStdHandle(-11i32 as u32);
        let mut mode: u32 = 0;
        if GetConsoleMode(handle, &mut mode) > 0 {
            return (mode & 0x0004) != 0;
        }
        return false;
    }
}

pub fn init() -> anyhow::Result<Stage> {
    Stage::init()
}

pub struct Stage {
    is_end_draw: AtomicBool,
    credits_pos_x: i32,
    ascii_art_x: i32,
    ascii_art_y: i32,
    lyric_height: i32,
    lyric_width: i32,
    credits_height: i32,
    credits_width: i32,
    ascii_art_height: i32,
    is_vt_version: u16,
    enable_screen_buffer: bool,
    enable_color: bool,
    enable_sound: bool,
}

impl Stage {
    fn init() -> anyhow::Result<Self> {
        let enable_sound = !env::args().any(|arg| arg == "--no-sound");
        if enable_sound {
            if !Path::new(SOUND_FILE_PATH).exists() {
                bail!("sa1.mp3 not found");
            }
        }

        #[allow(unused_mut)]
        let mut term = env::var("TERM").unwrap_or("vt100".to_owned());

        #[cfg(target_os = "windows")]
        {
            if check_support() {
                term = "windows".to_owned();
            }
        }

        let is_vt_version: u16 = term
            .strip_prefix("vt")
            .and_then(|v| {
                if !v.is_empty() && v.bytes().all(|b| b.is_ascii_digit()) {
                    v.parse().ok()
                } else {
                    None
                }
            })
            .unwrap_or(0);

        // xterm, rxvt, konsole ...
        // but fbcon in linux kernel does not support screen buffer
        let enable_screen_buffer = !(is_vt_version > 0 || term == "linux");

        let enable_color = is_vt_version == 0 || is_vt_version >= 241;

        let mut term_columns: i32 = 80;
        let mut term_lines: i32 = 24;
        if is_vt_version == 0 {
            if let Some((w, h)) = term_size::dimensions() {
                term_columns = w as i32;
                term_lines = h as i32;
            };
        }

        if let Ok(env_col) = env::var("COLUMNS") {
            if let Ok(env_col) = env_col.parse::<i32>() {
                term_columns = env_col;
            }
        }

        if let Ok(env_line) = env::var("LINES") {
            if let Ok(env_line) = env_line.parse::<i32>() {
                term_lines = env_line;
            }
        }

        if term_columns < 80 || term_lines < 24 {
            bail!("the terminal size should be at least 80x24");
        }

        let ascii_art_width: i32 = 40;
        let ascii_art_height: i32 = 20;

        let credits_width = std::cmp::min((term_columns - 4) / 2, 56);

        let credits_height = term_lines - ascii_art_height - 2;

        let lyric_width = term_columns - 4 - credits_width;

        let lyric_height = term_lines - 2;

        let ascii_art_x = lyric_width + 4 + (credits_width - ascii_art_width) / 2;

        let ascii_art_y = credits_height + 3;

        let credits_pos_x = lyric_width + 4;

        let cfg = Stage {
            is_end_draw: AtomicBool::new(false),
            credits_pos_x,
            ascii_art_x,
            ascii_art_y,
            lyric_height,
            lyric_width,
            credits_height,
            credits_width,
            ascii_art_height,
            is_vt_version,
            enable_screen_buffer,
            enable_color,
            enable_sound,
        };

        Ok(cfg)
    }

    fn begin_draw(&self) {
        if self.enable_screen_buffer {
            typed!("\x1b[?1049h");
        }
        if self.enable_color {
            typed!("\x1b[33;40;1m");
        }
        typed!("\x1b[2J");
    }

    fn draw_frame(&self) {
        self.move_to(1, 1);

        let lyric_width = self.lyric_width as usize;
        let credits_width = self.credits_width as usize;

        typed!(if self.is_vt_version == 0, " {}  {} ","-".repeat(lyric_width), "-".repeat(credits_width)
        );

        for _ in 0..self.credits_height {
            typed!(if self.is_vt_version == 0,"|{}||{}|"," ".repeat(lyric_width)," ".repeat(credits_width));
        }

        typed!(if self.is_vt_version == 0, "|{}| {} ", " ".repeat(lyric_width), "-".repeat(credits_width));

        for _ in 0..(self.lyric_height - 1 - self.credits_height) {
            typed!(true, "|{}|", " ".repeat(lyric_width));
        }

        typed!(false, " {} ", "-".repeat(lyric_width));
    }

    /// x 为列 y 为行  左上角坐标原点为1,1
    fn move_to(&self, x: i32, y: i32) {
        debug_assert!(x >= 0);
        debug_assert!(y >= 0);

        typed!("\x1b[{};{}H", y, x);
    }
}

impl Drop for Stage {
    fn drop(&mut self) {
        self.stop();
    }
}

impl Stage {
    pub fn stop(&self) {
        self.is_end_draw.store(true, Ordering::Release);

        if self.enable_color {
            typed!("\x1b[0m");
        }

        if self.enable_screen_buffer {
            typed!("\x1b[?1049l");
        } else {
            typed!("\x1b[2J");
            self.move_to(1, 1);
        }
    }

    pub fn run(&self) -> anyhow::Result<()> {
        self.begin_draw();
        self.draw_frame();
        self.move_to(2, 2);
        thread::sleep(Duration::from_secs(2));

        let instant = Instant::now();
        let lyrics = &crate::data::LYRICS;
        let mut lyric_idx = 0;
        let mut cursor_x: i32 = 2;
        let mut cursor_y: i32 = 2;
        let mut lyric_state: Option<lyric::LyricState> = None;
        let mut art_state: Option<art::ArtState> = None;
        let mut credit_state: Option<credit::CreditState> = None;

        loop {
            if self.is_end_draw.load(Ordering::Acquire) {
                break;
            }
            if lyric_state.is_none() && art_state.is_none() && lyrics[lyric_idx].mode == 9 {
                break;
            }

            let past_time = (instant.elapsed().as_millis() / 10) as i32;

            // Fire new lyric events (only when idle)
            if lyric_state.is_none() && art_state.is_none() {
                let current = &lyrics[lyric_idx];
                if past_time > current.time {
                    match current.mode {
                        0 | 1 => {
                            if let crate::data::WordsContent::Str(v) = current.words {
                                let wc = v.chars().count().max(1) as f64;
                                let interval = if current.interval < 0.0 {
                                    (lyrics[lyric_idx + 1].time - current.time) as f64 / 100.0 / wc
                                } else {
                                    current.interval / wc
                                };
                                lyric_state = Some(lyric::LyricState::new(
                                    v,
                                    cursor_x,
                                    cursor_y,
                                    interval,
                                    current.mode == 0,
                                ));
                            }
                        }
                        2 => {
                            if let crate::data::WordsContent::Int(v) = current.words {
                                art_state = Some(art::ArtState::new(v));
                            }
                        }
                        3 => {
                            lyric::clear_lyrics(self);
                            cursor_x = 2;
                            cursor_y = 2;
                        }
                        4 => {
                            if self.enable_sound {
                                crate::player::play(SOUND_FILE_PATH)?;
                            }
                        }
                        5 => {
                            credit_state = Some(credit::CreditState::new());
                        }
                        _ => {}
                    }
                    lyric_idx += 1;
                }
            }

            // Tick lyric typing state machine
            if let Some(ref mut ts) = lyric_state {
                ts.tick(self);
                if ts.done() {
                    cursor_x = ts.cursor_x;
                    cursor_y = ts.cursor_y;
                    lyric_state = None;
                }
            }

            // Tick ASCII art state machine
            if let Some(ref mut as_) = art_state {
                as_.tick(self);
                if as_.done(self) {
                    art_state = None;
                }
            }

            // Tick credit state machine
            if let Some(ref mut cs) = credit_state {
                while cs.is_ready() {
                    cs.tick(self);
                }
            }

            // Cursor refresh (only when idle)
            if lyric_state.is_none() {
                self.move_to(cursor_x, cursor_y);
                typed!("\0");
            }

            thread::sleep(Duration::from_millis(10));
        }

        Ok(())
    }
}
