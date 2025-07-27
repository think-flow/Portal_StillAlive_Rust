use super::{OutputMsg, credit};
use crate::{
    data, player,
    stage::{ChannelEx, Stage},
};
use std::{
    sync::{atomic::Ordering, mpsc},
    thread::{self, ScopedJoinHandle},
    time::Duration,
};

pub fn draw<'a>(
    s: &'a std::thread::Scope<'a, '_>,
    tx: mpsc::Sender<OutputMsg>,
    stage: &'a Stage,
) -> anyhow::Result<ScopedJoinHandle<'a, anyhow::Result<()>>> {
    let builder = thread::Builder::new().name("lyric".to_owned());
    let handle = builder.spawn_scoped(s, move || -> anyhow::Result<()> {
        let instant = std::time::Instant::now();
        let mut index = 0;
        let mut cursor_x = 2;
        let mut cursor_y = 2;
        let lyrics = &data::LYRICS;
        while lyrics[index].mode != 9 {
            if stage.is_end_draw.load(Ordering::Acquire) {
                return Ok(());
            }

            let current_lyric = &lyrics[index];
            let past_time = (instant.elapsed().as_millis() / 10) as i32;
            if past_time > current_lyric.time {
                let mut word_count: f64 = 0.0;
                let interval: f64;

                if current_lyric.mode <= 1 || current_lyric.mode >= 5 {
                    match current_lyric.words {
                        data::WordsContent::Str(v) => {
                            word_count = v.chars().count() as f64;
                        }
                        _ => unreachable!("在此处WordsContent不可能为Int"),
                    }
                }

                if word_count == 0.0 {
                    word_count = 1.0;
                }

                if current_lyric.interval < 0.0 {
                    let next_lyric = &lyrics[index + 1];
                    interval = (next_lyric.time - current_lyric.time) as f64 / 100.0 / word_count;
                } else {
                    interval = current_lyric.interval / word_count;
                }

                if current_lyric.mode == 0 {
                    match current_lyric.words {
                        data::WordsContent::Str(v) => {
                            draw_lyrics(&tx, v, &mut cursor_x, &mut cursor_y, interval, true);
                        }
                        _ => unreachable!("在此处WordsContent不可能为Int"),
                    }
                } else if current_lyric.mode == 1 {
                    match current_lyric.words {
                        data::WordsContent::Str(v) => {
                            draw_lyrics(&tx, v, &mut cursor_x, &mut cursor_y, interval, false);
                        }
                        _ => unreachable!("在此处WordsContent不可能为Int"),
                    }
                } else if current_lyric.mode == 2 {
                    match current_lyric.words {
                        data::WordsContent::Int(v) => {
                            draw_arts(v, stage, &tx);
                        }
                        _ => unreachable!("在此处WordsContent不可能为Str"),
                    }
                } else if current_lyric.mode == 3 {
                    clear_lyrics(&tx, stage);
                    cursor_x = 2;
                    cursor_y = 2;
                } else if current_lyric.mode == 4 {
                    if stage.enable_sound {
                        player::play(super::SOUND_FILE_PATH)?;
                    }
                } else if current_lyric.mode == 5 {
                    let tx1 = tx.clone();
                    //启动credit线程
                    credit::draw(s, tx1, stage)?;
                }

                index += 1;
            }
            thread::sleep(Duration::from_millis(10));
        }
        Ok(())
    })?;

    Ok(handle)
}

fn draw_arts(ch: i32, stage: &Stage, tx: &mpsc::Sender<OutputMsg>) {
    let art = data::ARTS[ch as usize];
    for dy in 0..stage.ascii_art_height {
        tx.print(
            (stage.ascii_art_x, stage.ascii_art_y + dy),
            art[dy as usize].to_owned(),
        );
        thread::sleep(Duration::from_millis(10));
    }
}

fn draw_lyrics(
    tx: &mpsc::Sender<OutputMsg>,
    str: &str,
    cursor_x: &mut i32,
    cursor_y: &mut i32,
    interval: f64,
    new_line: bool,
) {
    for c in str.chars() {
        tx.print((*cursor_x, *cursor_y), c.to_string());
        thread::sleep(Duration::from_secs_f64(interval));
        if c != '\0' {
            *cursor_x += 1;
        }
    }

    if new_line {
        *cursor_x = 2;
        *cursor_y += 1;
    }
}

fn clear_lyrics(tx: &mpsc::Sender<OutputMsg>, stage: &Stage) {
    let mut y = 2;
    for _ in 0..stage.lyric_height {
        tx.print(
            (2, y),
            format!("{}\n", " ".repeat(stage.lyric_width as usize)),
        );
        y += 1;
    }
}
