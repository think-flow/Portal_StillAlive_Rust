mod credit;
mod lyric;

use anyhow::{bail};
use regex_lite::Regex;
use std::{
    env,
    io::Write,
    path::Path,
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc,
    },
    thread,
    time::Duration,
};

const SOUND_FILE_PATH: &str = "./sa1.mp3";

macro_rules! pri {
    (true, $($arg:tt)*) => {{
        println!($($arg)*);
    }};

    (false, $($arg:tt)*) => {{
        pri!($($arg)*);
    }};

    // 匹配布尔表达式后跟参数的情况
    (if $cond:expr, $($arg:tt)*) => {{
        if $cond {
            println!($($arg)*);
        } else {
            pri!($($arg)*);
        }
    }};

    ($($arg:tt)*) => {{
        print!($($arg)*);
        std::io::stdout().flush().unwrap();
    }};
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

struct OutputMsg {
    position: Position,
    content: String,
}

struct Position {
    x: u16,
    y: u16,
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
    is_vt: Option<String>,
    enable_screen_buffer: bool,
    enable_color: bool,
    enable_sound: bool,
}

trait ChannelEx {
    fn print(&self, position: (i32, i32), content: String);
}

impl ChannelEx for mpsc::Sender<OutputMsg> {
    fn print(&self, position: (i32, i32), content: String) {
        let (x, y) = position;
        debug_assert!(x >= 0 && y >= 0);
        let msg = OutputMsg {
            position: Position {
                x: x as u16,
                y: y as u16,
            },
            content,
        };
        let _ = self.send(msg);
    }
}

impl Stage {
    pub fn init() -> anyhow::Result<Self> {
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

        let is_vt = Regex::new(r"vt(\d+)")?
            .captures(&term)
            .map(|c| c[0].to_owned());

        // xterm, rxvt, konsole ...
        // but fbcon in linux kernel does not support screen buffer
        let enable_screen_buffer = !(is_vt.is_some() || term == "linux");

        let enable_color = is_vt.is_none()
            || Regex::new(r"\d+")?
                .captures(is_vt.as_ref().unwrap())
                .unwrap()[0]
                .parse::<i32>()?
                >= 241;

        let mut term_columns: i32 = 80;
        let mut term_lines: i32 = 24;
        if is_vt.is_none() {
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
            is_vt,
            enable_screen_buffer,
            enable_color,
            enable_sound,
        };

        Ok(cfg)
    }

    fn begin_draw(&self) {
        if self.enable_screen_buffer {
            pri!("\x1b[?1049h");
        }
        if self.enable_color {
            pri!("\x1b[33;40;1m");
        }
        pri!("\x1b[2J");
    }

    fn draw_frame(&self) {
        self.move_to(1, 1);

        let lyric_width = self.lyric_width as usize;
        let credits_width = self.credits_width as usize;

        pri!(if self.is_vt.is_none(), " {}  {} ","-".repeat(lyric_width), "-".repeat(credits_width)
        );

        for _ in 0..self.credits_height {
            pri!(if self.is_vt.is_none(),"|{}||{}|"," ".repeat(lyric_width)," ".repeat(credits_width));
        }

        pri!(if self.is_vt.is_none(), "|{}| {} ", " ".repeat(lyric_width), "-".repeat(credits_width));

        for _ in 0..(self.lyric_height - 1 - self.credits_height) {
            pri!(true, "|{}|", " ".repeat(lyric_width));
        }

        pri!(false, " {} ", "-".repeat(lyric_width));
    }

    /// x 为列 y 为行  左上角坐标原点为1,1
    fn move_to(&self, x: i32, y: i32) {
        debug_assert!(x >= 0);
        debug_assert!(y >= 0);

        pri!("\x1b[{};{}H", y, x);
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
            pri!("\x1b[0m");
        }

        if self.enable_screen_buffer {
            pri!("\x1b[?1049l");
        } else {
            pri!("\x1b[2J");
            self.move_to(1, 1);
        }
    }

    /// block run untill the show is finished
    pub fn run(&self) -> anyhow::Result<()> {
        self.begin_draw();
        self.draw_frame();
        self.move_to(2, 2);
        thread::sleep(Duration::from_secs(2));

        thread::scope(|s| -> anyhow::Result<()> {
            let (tx, rx) = mpsc::channel::<OutputMsg>();

            // 启动lyric 线程
            let handle = lyric::draw(s, tx, self)?;

            // 主线程负责获取消息并打印
            for msg in rx.iter() {
                if self.is_end_draw.load(Ordering::Acquire) {
                    break;
                }
                let Position { x, y } = msg.position;
                if x > 0 && y > 0 {
                    self.move_to(x as i32, y as i32);
                }
                pri!("{}", msg.content);
            }

            // 等待lyric 线程执行完毕，并获得其返回的错误值，然后通过?传播出去
            if let Err(e) = handle.join().unwrap() {
                self.stop();
                return Err(e);
            }

            Ok(())
        })?;

        Ok(())
    }
}
