use super::OutputMsg;
use crate::{
    data,
    stage::{ChannelEx, Stage},
};
use std::{
    borrow::Cow,
    sync::OnceLock,
    time::{Duration, Instant},
};
use std::{collections::VecDeque, sync::atomic::Ordering};
use std::{sync::mpsc, thread};

static CLEAR_CREDITS_STR: OnceLock<String> = OnceLock::new();
pub fn draw<'a>(
    s: &'a std::thread::Scope<'a, '_>,
    tx: mpsc::Sender<OutputMsg>,
    stage: &'a Stage,
) -> anyhow::Result<()> {
    let builder = thread::Builder::new().name("credit".to_owned());
    builder.spawn_scoped(s, move || {
        let mut i: f64 = 0.0;
        let mut credit_x: i32 = 0;
        let length: f64 = data::CREDITS.len() as f64;
        let mut credit_list = VecDeque::with_capacity(stage.credits_height as usize);
        credit_list.push_front("".to_owned());
        let instant = Instant::now();

        // 这里能确保职员名称只有ascii字符 所以可以安心逐字节遍历
        for j in 0..data::CREDITS.len() {
            let ch: &'static str = &data::CREDITS[j..=j];
            if stage.is_end_draw.load(Ordering::Acquire) {
                return;
            }

            let duration: f64 = 174.0 / length * i;
            i += 1.0;
            if ch == "\n" {
                credit_x = 0;

                credit_list.push_back("".to_owned());
                if credit_list.len() as i32 > stage.credits_height {
                    // 删掉前面多余不用显示的行
                    for _ in 0..credit_list.len() - stage.credits_height as usize {
                        // remove element
                        _ = credit_list.pop_front();
                    }
                }

                for y in 2..(2 + stage.credits_height - credit_list.len() as i32) {
                    tx.typed(
                        (stage.credits_pos_x, y),
                        Cow::Borrowed(CLEAR_CREDITS_STR.get_or_init(|| {
                            format!("{}", " ".repeat(stage.credits_width as usize))
                        })),
                    );
                }

                for k in 0..credit_list.len() as i32 {
                    let y = 2 + stage.credits_height - credit_list.len() as i32 + k;
                    let count = stage.credits_width - credit_list[k as usize].len() as i32;
                    tx.typed(
                        (stage.credits_pos_x, y),
                        Cow::Owned(format!(
                            "{}{}",
                            credit_list[k as usize],
                            " ".repeat(count as usize)
                        )),
                    );
                }
            } else {
                let str = credit_list.back_mut().unwrap();
                str.push_str(ch);
                tx.typed(
                    (stage.credits_pos_x + credit_x, stage.credits_height + 1),
                    Cow::Borrowed(ch),
                );

                credit_x += 1;
            }

            while instant.elapsed().as_secs_f64() < duration {
                thread::sleep(Duration::from_millis(10));
            }
        }
    })?;

    Ok(())
}
