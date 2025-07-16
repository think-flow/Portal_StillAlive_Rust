use std::{
    env,
    io::{Write, stdout},
    str,
    sync::{
        LazyLock,
        atomic::{
            AtomicBool, AtomicI32,
            Ordering::{Acquire, Release},
        },
    },
    thread,
    time::Duration,
};

use crate::data;
use regex_lite::Regex;

#[cfg(target_os = "windows")]
static TERM: LazyLock<String> = LazyLock::new(|| {
    let mut term = env::var("TERM").unwrap_or("vt100".to_owned());
    if check_support() {
        term = "windows".to_owned();
    }
    term
});

#[cfg(target_os = "linux")]
static TERM: LazyLock<String> = LazyLock::new(|| env::var("TERM").unwrap_or("vt100".to_owned()));

// xterm, rxvt, konsole ...
// but fbcon in linux kernel does not support screen buffer
static ENABLE_SCREEN_BUFFER: LazyLock<bool> = LazyLock::new(|| {
    let term = TERM.as_str();
    !(IS_VT.is_some() || term == "linux")
});

// color support is after VT241
static ENABLE_COLOR: LazyLock<bool> = LazyLock::new(|| {
    !IS_VT.is_some()
        || Regex::new(r"\d+")
            .unwrap()
            .captures(IS_VT.as_ref().unwrap())
            .unwrap()[0]
            .parse::<i32>()
            .unwrap()
            >= 241
});

static IS_VT: LazyLock<Option<String>> = LazyLock::new(|| {
    let term = TERM.as_str();
    Regex::new(r"vt(\d+)")
        .unwrap()
        .captures(term)
        .map(|c| c[0].to_owned())
});

static TERM_COLUMNS: LazyLock<i32> = LazyLock::new(|| {
    let mut width: i32;
    if IS_VT.is_some() {
        width = 80;
    } else {
        let w = term_size::dimensions()
            .map(|(width, _)| width)
            .expect("无法获取控制台宽度（非终端环境）");
        width = w as i32;
    }

    if let Ok(env_col) = env::var("COLUMNS") {
        if let Ok(env_col) = env_col.parse::<i32>() {
            width = env_col;
        }
    }

    if width < 80 {
        panic!("the terminal size should be at least 80x24");
    }

    width
});

static TERM_LINES: LazyLock<i32> = LazyLock::new(|| {
    let mut height: i32;
    if IS_VT.is_some() {
        height = 24;
    } else {
        let h = term_size::dimensions()
            .map(|(_, height)| height)
            .expect("无法获取控制台高度（非终端环境）");
        height = h as i32;
    }

    if let Ok(env_line) = env::var("LINES") {
        if let Ok(env_line) = env_line.parse::<i32>() {
            height = env_line;
        }
    }

    if height < 24 {
        panic!("the terminal size should be at least 80x24");
    }

    height
});

static ASCII_ART_WIDTH: i32 = 40;
static ASCII_ART_HEIGHT: i32 = 20;

static CREDITS_WIDTH: LazyLock<i32> = LazyLock::new(|| std::cmp::min((*TERM_COLUMNS - 4) / 2, 56));

static CREDITS_HEIGHT: LazyLock<i32> = LazyLock::new(|| *TERM_LINES - ASCII_ART_HEIGHT - 2);

static LYRIC_WIDTH: LazyLock<i32> = LazyLock::new(|| *TERM_COLUMNS - 4 - *CREDITS_WIDTH);

static LYRIC_HEIGHT: LazyLock<i32> = LazyLock::new(|| *TERM_LINES - 2);

static ASCII_ART_X: LazyLock<i32> =
    LazyLock::new(|| *LYRIC_WIDTH + 4 + (*CREDITS_WIDTH - ASCII_ART_WIDTH) / 2);

static ASCII_ART_Y: LazyLock<i32> = LazyLock::new(|| *CREDITS_HEIGHT + 3);

static CREDITS_POS_X: LazyLock<i32> = LazyLock::new(|| *LYRIC_WIDTH + 4);

static CURSOR_X: AtomicI32 = AtomicI32::new(0);
static CURSOR_Y: AtomicI32 = AtomicI32::new(0);
static IS_END_DRAW: AtomicBool = AtomicBool::new(false);

fn _print(str: &str, new_line: bool) {
    if new_line {
        println!("{}", str);
        CURSOR_X.store(1, Release);
        CURSOR_Y.fetch_add(1, Release);
    } else {
        print!("{}", str);
        CURSOR_X.fetch_add(str.chars().count() as i32, Release);
    }
}

pub fn begin_draw() {
    if *ENABLE_SCREEN_BUFFER {
        print!("\x1b[?1049h");
    }
    if *ENABLE_COLOR {
        print!("\x1b[33;40;1m");
    }
}

pub fn end_draw() {
    IS_END_DRAW.store(true, Release);

    if *ENABLE_COLOR {
        print!("\x1b[0m");
    }

    if *ENABLE_SCREEN_BUFFER {
        print!("\x1b[?1049l");
    } else {
        clear();
        r#move(1, 1, false);
    }
}

pub fn clear() {
    CURSOR_X.store(1, Release);
    CURSOR_Y.store(1, Release);

    print!("\x1b[2J");
}

pub fn r#move(x: i32, y: i32, update_cursor: bool) {
    print!("\x1b[{};{}H", y, x);
    stdout().flush().unwrap();

    if update_cursor {
        CURSOR_X.store(x, Release);
        CURSOR_Y.store(y, Release);
    }
}

pub fn draw_frame() {
    r#move(1, 1, true);

    let lyric_width = *LYRIC_WIDTH as usize;
    let credits_width = *CREDITS_WIDTH as usize;
    let str = format!(
        " {}  {} ",
        "-".repeat(lyric_width),
        "-".repeat(credits_width)
    );
    _print(&str, !IS_VT.is_some());

    for _ in 0..*CREDITS_HEIGHT {
        let str: String = format!(
            "|{}||{}|",
            " ".repeat(lyric_width),
            " ".repeat(credits_width)
        );
        _print(&str, !IS_VT.is_some());
    }

    let str = format!(
        "|{}| {} ",
        " ".repeat(lyric_width),
        "-".repeat(credits_width)
    );
    _print(&str, !IS_VT.is_some());

    for _ in 0..(*LYRIC_HEIGHT - 1 - *CREDITS_HEIGHT) {
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
    let arts = &data::ARTS;
    for dy in 0..ASCII_ART_HEIGHT {
        {
            let mut lock = stdout().lock();
            r#move(*ASCII_ART_X, *ASCII_ART_Y + dy, true);
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
                    if last_credits.len() as i32 > *CREDITS_HEIGHT {
                        last_credits = (&last_credits
                            [last_credits.len() - *CREDITS_HEIGHT as usize..])
                            .to_vec();
                    }

                    if IS_END_DRAW.load(Acquire) {
                        break;
                    }

                    let mut lock = stdout().lock();
                    for y in 2..(2 + *CREDITS_HEIGHT - last_credits.len() as i32) {
                        r#move(*CREDITS_POS_X, y, false);
                        write!(lock, "{}", " ".repeat(*CREDITS_WIDTH as usize)).unwrap();
                    }

                    for k in 0..last_credits.len() as i32 {
                        let y = 2 + *CREDITS_HEIGHT - last_credits.len() as i32 + k;
                        r#move(*CREDITS_POS_X, y, false);
                        write!(lock, "{}", last_credits[k as usize]).unwrap();
                        let count =
                            *CREDITS_WIDTH - last_credits[k as usize].chars().count() as i32;
                        write!(lock, "{}", " ".repeat(count as usize)).unwrap();
                    }

                    r#move(CURSOR_X.load(Acquire), CURSOR_Y.load(Acquire), false);
                } else {
                    let str = last_credits.last_mut().unwrap();
                    str.push(ch);

                    if IS_END_DRAW.load(Acquire) {
                        break;
                    }

                    let mut lock = stdout().lock();
                    r#move(*CREDITS_POS_X + credit_x, *CREDITS_HEIGHT + 1, false);
                    write!(lock, "{}", ch.to_string()).unwrap();
                    r#move(CURSOR_X.load(Acquire), CURSOR_Y.load(Acquire), false);

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
    r#move(1, 2, true);
    for _ in 0..*LYRIC_HEIGHT {
        _print(&format!("|{}", " ".repeat(*LYRIC_WIDTH as usize)), true);
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
