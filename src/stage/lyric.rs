use crate::stage::Stage;
use crate::typed;
use std::sync::OnceLock;
use std::time::Instant;

static CLEAR_LYRICS_STR: OnceLock<String> = OnceLock::new();

pub struct LyricState {
    pub text: &'static str,
    pub char_idx: usize,
    pub cursor_x: i32,
    pub cursor_y: i32,
    interval: f64,
    new_line: bool,
    started_at: Instant,
}

impl LyricState {
    pub fn new(
        text: &'static str,
        cursor_x: i32,
        cursor_y: i32,
        interval: f64,
        new_line: bool,
    ) -> Self {
        Self {
            text,
            char_idx: 0,
            cursor_x,
            cursor_y,
            interval,
            new_line,
            started_at: Instant::now(),
        }
    }

    pub fn done(&self) -> bool {
        self.char_idx >= self.text.len()
    }

    pub fn tick(&mut self, stage: &Stage) {
        let elapsed = self.started_at.elapsed().as_secs_f64();
        while self.char_idx < self.text.len() {
            let expected = self.char_idx as f64 * self.interval;
            if elapsed < expected {
                return;
            }
            let c = &self.text[self.char_idx..=self.char_idx];
            stage.move_to(self.cursor_x, self.cursor_y);
            typed!("{}", c);
            if c != "\0" {
                self.cursor_x += 1;
            }
            self.char_idx += 1;
        }
        if self.new_line {
            self.cursor_x = 2;
            self.cursor_y += 1;
        }
    }
}

pub fn clear_lyrics(stage: &Stage) {
    let clear =
        CLEAR_LYRICS_STR.get_or_init(|| format!("{}\n", " ".repeat(stage.lyric_width as usize)));
    for y in 0..stage.lyric_height {
        stage.move_to(2, y + 2);
        typed!("{}", clear);
    }
}
