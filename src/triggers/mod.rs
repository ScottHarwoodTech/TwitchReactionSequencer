mod triggers;
pub mod twitch_pub_sub;

use crate::sequencer::QueueEvent;
use futures_util::future;
use std::collections::HashMap;
use std::error::Error;
use tokio::sync::{mpsc, watch};

pub async fn watch_trigger_sources(
    trigger_sources_map: HashMap<String, Box<dyn triggers::TriggerSource>>,
    trigger_sequence_stream: watch::Sender<QueueEvent>,
) -> Result<(), Box<dyn Error>> {
    let mut watchers = Vec::new();

    let (tx, rx): (mpsc::Sender<QueueEvent>, mpsc::Receiver<QueueEvent>) = mpsc::channel(10);

    for trigger in trigger_sources_map.into_values() {
        watchers.push(async move {
            trigger.watch((&tx).clone()).await;
        })
    }

    future::join_all(watchers);
    Ok(())
}

pub async fn get_available_trigger_sources(
) -> Result<HashMap<String, Box<dyn triggers::TriggerSource>>, Box<dyn Error>> {
    let mut trigger_sources: HashMap<String, Box<dyn triggers::TriggerSource>> = HashMap::new();

    trigger_sources.insert(
        String::from("twitch_pub_sub"),
        Box::new(twitch_pub_sub::TwitchPubSub::new("lanaSidhe").await?),
    );

    return Ok(trigger_sources);
}
