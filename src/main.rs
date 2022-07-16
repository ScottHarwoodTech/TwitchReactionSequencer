use std::error::Error;

mod custom_widgets;
mod sequencer;
mod triggers;
mod ui;
mod util;

use dotenv::dotenv;
use futures_util::future;
use tokio::sync::watch;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    let device_set = sequencer::devices::setup_devices().await?;
    let triggers = triggers::get_available_trigger_sources().await?;

    ui::ui(device_set.clone(), triggers.clone());

    let (trigger_sequence, trigger_sequence_reciever) = watch::channel(sequencer::QueueEvent {
        sequence_id: String::from("empty"),
    });

    let sequencer_queue = sequencer::watch_queue(device_set, trigger_sequence_reciever);

    let trigger_manager = triggers::watch_trigger_sources(triggers, trigger_sequence);

    let _ = future::join(trigger_manager, sequencer_queue).await;

    Ok(())
}
