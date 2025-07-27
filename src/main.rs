mod data;
mod player;
mod stage;

use crate::stage::Stage;
use std::{process, sync::Arc};

fn main() {
    match Stage::init() {
        Err(e) => {
            eprintln!("{}", e);
            process::exit(2);
        }
        Ok(val) => {
            let stage = Arc::new(val);
            let weak_stage = Arc::downgrade(&stage);

            ctrlc::set_handler(move || {
                if let Some(stage) = weak_stage.upgrade() {
                    stage.stop();
                }
                println!("Got it! Exiting...");
                process::exit(1)
            })
            .expect("Error setting Ctrl-C handler");

            if let Err(e) = stage.run() {
                eprintln!("{}", e);
                process::exit(2);
            }
        }
    }
}
