use std::error::Error;

mod custom_widgets;
mod sequencer;
mod triggers;
mod ui;
mod util;

use dotenv::dotenv;

#[derive(Debug, Clone)]
pub enum ThreadActions {
    Stop,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    let device_set = sequencer::devices::setup_devices().await?;
    let triggers = triggers::get_available_trigger_sources().await?;

    ui::ui(device_set.clone(), triggers.clone());
    Ok(())
}
