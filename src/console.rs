use std::{
    env,
    io::{Write, stdout},
    str,
    sync::{
        OnceLock,
        atomic::{
            AtomicBool, AtomicI32,
            Ordering::{Acquire, Release},
        },
    },
    thread,
    time::Duration,
};

use crate::data;
use anyhow::{anyhow, bail};
use regex_lite::Regex;

struct Config {
    is_end_draw: AtomicBool,
    cursor_x: AtomicI32,
    cursor_y: AtomicI32,
    credits_pos_x: i32,
    ascii_art_x: i32,
    ascii_art_y: i32,
    lyric_height: i32,
    lyric_width: i32,
    credits_height: i32,
    credits_width: i32,
    ascii_art_height: i32,
    is_vt: Option<String>,
    enable_screen_buffer: bool,
    enable_color: bool,
}

static CFG: OnceLock<Config> = OnceLock::new();

pub fn init() -> anyhow::Result<()> {
    #[allow(unused_mut)]
    let mut term = env::var("TERM").unwrap_or("vt100".to_owned());

    #[cfg(target_os = "windows")]
    {
        if check_support() {
            term = "windows".to_owned();
        }
    }

    let is_vt = Regex::new(r"vt(\d+)")?
        .captures(&term)
        .map(|c| c[0].to_owned());

    // xterm, rxvt, konsole ...
    // but fbcon in linux kernel does not support screen buffer
    let enable_screen_buffer = !(is_vt.is_some() || term == "linux");

    let enable_color = !is_vt.is_some()
        || Regex::new(r"\d+")?
            .captures(is_vt.as_ref().unwrap())
            .unwrap()[0]
            .parse::<i32>()?
            >= 241;

    let mut term_columns: i32;
    let mut term_lines: i32;
    if is_vt.is_some() {
        term_columns = 80;
        term_lines = 24;
    } else {
        let (w, h) = term_size::dimensions().ok_or(anyhow!("无法获取控制台尺寸（非终端环境）"))?;
        term_columns = w as i32;
        term_lines = h as i32;
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

    let cursor_x = AtomicI32::new(0);
    let cursor_y = AtomicI32::new(0);
    let is_end_draw = AtomicBool::new(false);

    let cfg = Config {
        is_end_draw,
        cursor_x,
        cursor_y,
        credits_pos_x,
        ascii_art_x,
        ascii_art_y,
        lyric_height,
        lyric_width,
        credits_height,
        credits_width,
        ascii_art_height,
        is_vt,
        enable_screen_buffer,
        enable_color,
    };

    let _ = CFG.set(cfg);

    Ok(())
}

fn _print(str: &str, new_line: bool) {
    let cfg = CFG.get().unwrap();
    if new_line {
        println!("{}", str);
        cfg.cursor_x.store(1, Release);
        cfg.cursor_y.fetch_add(1, Release);
    } else {
        print!("{}", str);
        cfg.cursor_x.fetch_add(str.chars().count() as i32, Release);
    }
}

pub fn begin_draw() {
    let cfg = CFG.get().unwrap();
    if cfg.enable_screen_buffer {
        print!("\x1b[?1049h");
    }
    if cfg.enable_color {
        print!("\x1b[33;40;1m");
    }
}

pub fn end_draw() {
    let cfg = CFG.get().unwrap();
    cfg.is_end_draw.store(true, Release);

    if cfg.enable_color {
        print!("\x1b[0m");
    }

    if cfg.enable_screen_buffer {
        print!("\x1b[?1049l");
    } else {
        clear();
        r#move(1, 1, false);
    }
}

pub fn clear() {
    let cfg = CFG.get().unwrap();
    cfg.cursor_x.store(1, Release);
    cfg.cursor_y.store(1, Release);

    print!("\x1b[2J");
}

pub fn r#move(x: i32, y: i32, update_cursor: bool) {
    print!("\x1b[{};{}H", y, x);
    stdout().flush().unwrap();

    if update_cursor {
        let cfg = CFG.get().unwrap();
        cfg.cursor_x.store(x, Release);
        cfg.cursor_y.store(y, Release);
    }
}

pub fn draw_frame() {
    let cfg = CFG.get().unwrap();
    r#move(1, 1, true);

    let lyric_width = cfg.lyric_width as usize;
    let credits_width = cfg.credits_width as usize;
    let str = format!(
        " {}  {} ",
        "-".repeat(lyric_width),
        "-".repeat(credits_width)
    );
    _print(&str, !cfg.is_vt.is_some());

    for _ in 0..cfg.credits_height {
        let str: String = format!(
            "|{}||{}|",
            " ".repeat(lyric_width),
            " ".repeat(credits_width)
        );
        _print(&str, !cfg.is_vt.is_some());
    }

    let str = format!(
        "|{}| {} ",
        " ".repeat(lyric_width),
        "-".repeat(credits_width)
    );
    _print(&str, !cfg.is_vt.is_some());

    for _ in 0..(cfg.lyric_height - 1 - cfg.credits_height) {
        _print(&format!("|{}|", " ".repeat(lyric_width)), true);
    }

    _print(&format!(" {} ", "-".repeat(lyric_width)), false);

    r#move(2, 2, true);

    stdout().flush().unwrap();
    thread::sleep(Duration::from_millis(1000));
}

pub fn draw_lyrics(str: &str, x: i32, y: i32, interval: f64, new_line: bool) -> i32 {
    let mut x = x;
    let mut y = y;
    r#move(x + 2, y + 2, true);
    for c in str.chars() {
        {
            let mut lock = stdout().lock();
            _print(&c.to_string(), false);
            lock.flush().unwrap();
        }
        thread::sleep(Duration::from_secs_f64(interval));
        x += 1;
    }

    if new_line {
        x = 0;
        y += 1;
        r#move(2, y + 2, true);
    }
    return x;
}

pub fn draw_arts(ch: i32, x: i32, y: i32) {
    let cfg = CFG.get().unwrap();
    let arts = &data::ARTS;
    for dy in 0..cfg.ascii_art_height {
        {
            let mut lock = stdout().lock();
            r#move(cfg.ascii_art_x, cfg.ascii_art_y + dy, true);
            print!("{}", arts[ch as usize][dy as usize]);
            lock.flush().unwrap();
        }
        thread::sleep(Duration::from_millis(10));
    }
    r#move(x + 2, y + 2, true);
}

pub fn draw_credits() {
    let build = thread::Builder::new().name("credits".to_owned());
    build
        .spawn(|| {
            let cfg = CFG.get().unwrap();
            let credits = data::CREDITS;
            let mut credit_x: i32 = 0;
            let mut i: f64 = 0.0;
            let length: f64 = credits.chars().count() as f64;
            let mut last_credits: Vec<String> = vec!["".to_owned()];
            let instant = std::time::Instant::now();

            for ch in credits.chars() {
                let duration: f64 = 174.0 / length * i;
                i += 1.0;
                if ch == '\n' {
                    credit_x = 0;
                    last_credits.push("".to_owned());
                    if last_credits.len() as i32 > cfg.credits_height {
                        last_credits = (&last_credits
                            [last_credits.len() - cfg.credits_height as usize..])
                            .to_vec();
                    }

                    if cfg.is_end_draw.load(Acquire) {
                        break;
                    }

                    let mut lock = stdout().lock();
                    for y in 2..(2 + cfg.credits_height - last_credits.len() as i32) {
                        r#move(cfg.credits_pos_x, y, false);
                        write!(lock, "{}", " ".repeat(cfg.credits_width as usize)).unwrap();
                    }

                    for k in 0..last_credits.len() as i32 {
                        let y = 2 + cfg.credits_height - last_credits.len() as i32 + k;
                        r#move(cfg.credits_pos_x, y, false);
                        write!(lock, "{}", last_credits[k as usize]).unwrap();
                        let count =
                            cfg.credits_width - last_credits[k as usize].chars().count() as i32;
                        write!(lock, "{}", " ".repeat(count as usize)).unwrap();
                    }

                    r#move(
                        cfg.cursor_x.load(Acquire),
                        cfg.cursor_y.load(Acquire),
                        false,
                    );
                } else {
                    let str = last_credits.last_mut().unwrap();
                    str.push(ch);

                    if cfg.is_end_draw.load(Acquire) {
                        break;
                    }

                    let mut lock = stdout().lock();
                    r#move(cfg.credits_pos_x + credit_x, cfg.credits_height + 1, false);
                    write!(lock, "{}", ch.to_string()).unwrap();
                    r#move(
                        cfg.cursor_x.load(Acquire),
                        cfg.cursor_y.load(Acquire),
                        false,
                    );

                    credit_x += 1;
                }

                while instant.elapsed().as_secs_f64() < duration {
                    thread::sleep(Duration::from_millis(10));
                }
            }
        })
        .unwrap();
}

pub fn clear_lyrics() {
    let cfg = CFG.get().unwrap();
    r#move(1, 2, true);
    for _ in 0..cfg.lyric_height {
        _print(&format!("|{}", " ".repeat(cfg.lyric_width as usize)), true);
    }
    r#move(2, 2, true);
}

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
pub fn check_support() -> bool {
    unsafe {
        let handle = GetStdHandle(-11i32 as u32);
        let mut mode: u32 = 0;
        if GetConsoleMode(handle, &mut mode) > 0 {
            return (mode & 0x0004) != 0;
        }
        return false;
    }
}
