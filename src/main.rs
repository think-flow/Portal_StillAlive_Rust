use anyhow::Context;
use portal_still_alive::stage;
use std::sync::Arc;

fn main() -> anyhow::Result<()> {
    let stage = stage::init()?;
    let stage = Arc::new(stage);
    let weak_stage = Arc::downgrade(&stage);

    ctrlc::set_handler(move || {
        if let Some(stage) = weak_stage.upgrade() {
            stage.stop();
        }
        println!("Got it! Exiting...");
        std::process::exit(0);
    })
    .context("Error setting Ctrl-C handler")?;

    stage.run()?;

    Ok(())
}
