mod triggers;
pub mod twitch_pub_sub;

use crate::sequencer::QueueEvent;
use futures_util::future;
use std::collections::HashMap;
use std::error::Error;
use tokio::sync::{mpsc, watch};
use tokio::time;

pub async fn watch_trigger_sources(
    trigger_sources_map: HashMap<String, Box<dyn triggers::TriggerSource>>,
    trigger_sequence_stream: watch::Sender<QueueEvent>,
) -> Result<(), Box<dyn Error>> {
    let mut watchers = Vec::new();

    let (tx, mut rx): (mpsc::Sender<QueueEvent>, mpsc::Receiver<QueueEvent>) = mpsc::channel(10);

    for trigger in trigger_sources_map.into_values() {
        let moveable_tx = tx.clone();

        watchers.push(async move {
            trigger.watch(moveable_tx).await.unwrap();
        })
    }

    let rx_join_handle = tokio::spawn(async move {
        while let Some(i) = rx.recv().await {
            println!("rx join handler: {:?}", i);
            trigger_sequence_stream.send(i).unwrap();
        }
    });

    let trigger_interval = tokio::spawn(async move {
        let mut interval = time::interval(time::Duration::from_secs(10));
        loop {
            interval.tick().await;
            tx.send(QueueEvent {
                sequence_id: String::from("default"),
            })
            .await
            .unwrap();
        }
    });

    let _ = future::join3(future::join_all(watchers), rx_join_handle, trigger_interval).await;
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
