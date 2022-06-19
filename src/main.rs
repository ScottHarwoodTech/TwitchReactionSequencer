use std::error::Error;
use triggers::twitchPubSub;

mod sequencer;
mod triggers;
mod util;

use dotenv::dotenv;
use tokio::sync::watch;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    let twitch_client = twitchPubSub::TwitchController::new("lanaSidhe").await?;

    twitch_client.start().await?;

    let (trigger_sequence, trigger_sequence_reciever) = watch::channel(sequencer::QueueEvent {
        sequence_id: String::from("empty"),
    });

    let sequencer_queue = sequencer::watch_queue(trigger_sequence_reciever);

    let triggers = triggers::get_available_trigger_sources().await?;
    let trigger_manager = triggers::trigger_manager(triggers);

    trigger_sequence
        .send(sequencer::QueueEvent {
            sequence_id: String::from("default"),
        })
        .unwrap();

    trigger_manager.await.unwrap();
    sequencer_queue.await.unwrap();

    Ok(())
}
