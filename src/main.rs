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

    let join_handle = sequencer::watch_queue(trigger_sequence_reciever);

    trigger_sequence
        .send(sequencer::QueueEvent {
            sequence_id: String::from("default"),
        })
        .unwrap();

    join_handle.await.unwrap();

    Ok(())
}
