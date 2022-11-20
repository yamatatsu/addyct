mod app;
mod run_app;
mod ui;

use crate::run_app::run;
use std::{error::Error, time::Duration};

fn main() -> Result<(), Box<dyn Error>> {
    let tick_rate = 250u64;
    let enhanced_graphics = true;

    let tick_rate = Duration::from_millis(tick_rate);
    run(tick_rate, enhanced_graphics)?;
    Ok(())
}
