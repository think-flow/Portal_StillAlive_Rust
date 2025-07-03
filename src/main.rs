mod console;
mod data;
mod player;

use console::*;
use std::{env, path::Path, process, sync::mpsc::channel, thread, time::Duration};

const SOUND_FILE_PATH: &str = "./sa1.mp3";

fn main() {
    let enable_sound = !env::args().any(|arg| arg == "--no-sound");
    if enable_sound {
        if !Path::new(SOUND_FILE_PATH).exists() {
            println!("sa1.mp3 not found");
            process::exit(2);
        }
    }

    let (tx, rx) = channel();
    ctrlc::set_handler(move || tx.send(()).expect("Could not send signal on channel."))
        .expect("Error setting Ctrl-C handler");

    thread::spawn(move || {
        rx.recv().expect("Could not receive from channel.");
        end_draw();
        println!("Got it! Exiting...");
        process::exit(1)
    });

    begin_draw();
    clear();
    draw_frame();
    r#move(2, 2, true);
    thread::sleep(Duration::from_millis(1000));

    let start_time = get_unix_timestamp_ms() as f64 / 10.0;
    let mut current_lyric = 0;
    let mut x = 0;
    let mut y = 0;
    let lyrics = &*data::LYRICS;

    while lyrics[current_lyric].mode != 9 {
        let current_time = get_unix_timestamp_ms() as f64 / 10.0 - start_time;

        if current_time > lyrics[current_lyric].time as f64 {
            let mut word_count = 0;
            let interval: f64;

            if lyrics[current_lyric].mode <= 1 || lyrics[current_lyric].mode >= 5 {
                match lyrics[current_lyric].words {
                    data::WordsContent::Str(v) => {
                        word_count = v.chars().count();
                    }
                    _ => panic!("在此处WordsContent不可能为Int"),
                }
            }

            if word_count == 0 {
                word_count = 1;
            }

            if lyrics[current_lyric].interval < 0.0 {
                interval = (lyrics[current_lyric + 1].time - lyrics[current_lyric].time) as f64
                    / 100.0
                    / word_count as f64;
            } else {
                interval = lyrics[current_lyric].interval / word_count as f64;
            }

            if lyrics[current_lyric].mode == 0 {
                match lyrics[current_lyric].words {
                    data::WordsContent::Str(v) => {
                        x = draw_lyrics(v, x, y, interval, true);
                        y += 1;
                    }
                    _ => panic!("在此处WordsContent不可能为Int"),
                }
            } else if lyrics[current_lyric].mode == 1 {
                match lyrics[current_lyric].words {
                    data::WordsContent::Str(v) => {
                        x = draw_lyrics(v, x, y, interval, false);
                    }
                    _ => panic!("在此处WordsContent不可能为Int"),
                }
            } else if lyrics[current_lyric].mode == 2 {
                match lyrics[current_lyric].words {
                    data::WordsContent::Int(v) => {
                        draw_arts(v);
                        r#move(x + 2, y + 2, true);
                    }
                    _ => panic!("在此处WordsContent不可能为Str"),
                }
            } else if lyrics[current_lyric].mode == 3 {
                clear_lyrics();
                x = 0;
                y = 0;
            } else if lyrics[current_lyric].mode == 4 {
                if enable_sound {
                    if let Err(e) = player::play(SOUND_FILE_PATH) {
                        end_draw();
                        println!("{:?}", e);
                        process::exit(2);
                    }
                }
            } else if lyrics[current_lyric].mode == 5 {
                draw_credits();
            }

            current_lyric += 1;
        }

        thread::sleep(Duration::from_millis(10));
    }

    end_draw();
}
