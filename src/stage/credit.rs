use crate::data;
use crate::stage::Stage;
use crate::typed;
use std::time::Instant;

const PADDING: &str = "                                                        ";

pub struct CreditState {
    j: usize,
    line_idx: usize,
    char_pos: usize,
    started_at: Instant,
    /// 每行在 CREDITS 中的起始字节偏移。line_starts[0] = 0，
    /// 遇到\n时记录下一行的起始位置。第i行内容为 CREDITS[start..end-1]。
    line_starts: Vec<u32>,
}

impl CreditState {
    pub fn new() -> Self {
        Self {
            j: 0,
            line_idx: 0,
            char_pos: 0,
            started_at: Instant::now(),
            line_starts: vec![0],
        }
    }

    /// 从 CREDITS 中提取第 idx 行的文本（不包含结尾的 \n）
    fn line_text(&self, idx: usize) -> &'static str {
        let start = self.line_starts[idx] as usize;
        let end = self
            .line_starts
            .get(idx + 1)
            .map(|&pos| pos as usize - 1) // 跳过\n
            .unwrap_or(data::CREDITS.len());
        &data::CREDITS[start..end]
    }

    pub fn is_done(&self) -> bool {
        self.j >= data::CREDITS.len()
    }

    fn char_seconds(&self) -> f64 {
        174.0 * self.j as f64 / data::CREDITS.len() as f64
    }

    pub fn is_ready(&self) -> bool {
        !self.is_done() && self.started_at.elapsed().as_secs_f64() >= self.char_seconds()
    }

    pub fn tick(&mut self, stage: &Stage) {
        let credits_width = stage.credits_width as usize;
        let num_visible = stage.credits_height as usize;

        let ch = &data::CREDITS[self.j..=self.j];
        self.j += 1;

        if ch == "\n" {
            self.line_starts.push(self.j as u32);

            // Redraw all visible rows (bottom-aligned)
            let completed_count = self.line_idx + 1;
            let total_entries = completed_count + 1;
            let visible_entries = total_entries.min(num_visible);
            let first_visible = total_entries - visible_entries;
            let start_y = 2 + (num_visible - visible_entries) as i32;

            for y in 2..start_y {
                stage.move_to(stage.credits_pos_x, y);
                typed!("{}", &PADDING[..credits_width]);
            }

            for k in 0..visible_entries {
                let y = start_y + k as i32;
                let entry_idx = first_visible + k;
                stage.move_to(stage.credits_pos_x, y);
                if entry_idx <= self.line_idx {
                    let line = self.line_text(entry_idx);
                    typed!("{}", line);
                    let line_len = line.len();
                    if line_len < credits_width {
                        typed!("{}", &PADDING[..credits_width - line_len]);
                    }
                } else {
                    typed!("{}", &PADDING[..credits_width]);
                }
            }

            self.line_idx += 1;
            self.char_pos = 0;
        } else {
            stage.move_to(
                stage.credits_pos_x + self.char_pos as i32,
                stage.credits_height + 1,
            );
            typed!("{}", ch);
            self.char_pos += 1;
        }
    }
}
