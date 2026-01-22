mod data;
mod player;
mod stage;

use anyhow::Context;

use crate::stage::Stage;
use std::{process, sync::Arc};

fn main() -> anyhow::Result<()> {
    let stage = Stage::init()?;
    let stage = Arc::new(stage);
    let weak_stage = Arc::downgrade(&stage);

    ctrlc::set_handler(move || {
        if let Some(stage) = weak_stage.upgrade() {
            stage.stop();
        }
        println!("Got it! Exiting...");
        process::exit(0);
    })
    .context("Error setting Ctrl-C handler")?;

    stage.run()?;

    Ok(())
}
