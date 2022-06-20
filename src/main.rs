use std::error::Error;

mod sequencer;
mod triggers;
mod util;

use dotenv::dotenv;
use futures_util::future;
use tokio::sync::watch;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let (trigger_sequence, trigger_sequence_reciever) = watch::channel(sequencer::QueueEvent {
        sequence_id: String::from("empty"),
    });
    let device_set = sequencer::devices::setup_devices().await?;
    let sequencer_queue = sequencer::watch_queue(device_set, trigger_sequence_reciever);

    let triggers = triggers::get_available_trigger_sources().await?;
    let trigger_manager = triggers::watch_trigger_sources(triggers, trigger_sequence);

    let _ = future::join(trigger_manager, sequencer_queue).await;

    Ok(())
}
