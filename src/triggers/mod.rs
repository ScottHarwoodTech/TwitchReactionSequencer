pub mod triggers;
pub mod twitch_chat;
pub mod twitch_pub_sub;

use crate::sequencer::QueueEvent;
use futures_util::future::{self};
use futures_util::{select, FutureExt};
use std::collections::HashMap;
use std::error::Error;

use tokio::sync::{mpsc, watch};

pub async fn watch_for_events(
    mut rx: mpsc::Receiver<QueueEvent>,
    trigger_sequence_stream: watch::Sender<QueueEvent>,
) {
    while let Some(i) = rx.recv().await {
        println!("rx join handler: {:?}", i);
        trigger_sequence_stream.send(i).unwrap();
    }
}

//TODO: this shit is a mess
async fn race(
    rx: mpsc::Receiver<QueueEvent>,
    trigger_sequence_stream: watch::Sender<QueueEvent>,
    mut task_handler_reciever: watch::Receiver<()>,
) {
    select!(
       _x = watch_for_events(rx, trigger_sequence_stream).fuse() => println!("handler finished"),
        _v = task_handler_reciever.changed().fuse() => println!("dropped rx join_handle"),
    )
}

pub async fn watch_trigger_sources(
    trigger_sources_map: TriggerCollection,
    trigger_sequence_stream: watch::Sender<QueueEvent>,
    task_handler_reciever: watch::Receiver<()>,
) -> Result<(), Box<dyn Error>> {
    let mut watchers = Vec::new();

    let (tx, rx): (mpsc::Sender<QueueEvent>, mpsc::Receiver<QueueEvent>) = mpsc::channel(10);

    for trigger in trigger_sources_map.into_values() {
        let moveable_tx = tx.clone();
        let movable_watcher = task_handler_reciever.clone();
        watchers.push(async move {
            trigger.watch(moveable_tx, movable_watcher).await.unwrap();
        })
    }

    let rx_join_handle = tokio::spawn(race(
        rx,
        trigger_sequence_stream,
        task_handler_reciever.clone(),
    ));

    let _ = future::join(future::join_all(watchers), rx_join_handle).await;
    Ok(())
}
const TARGET_CHANNEL: &str = "scootscoot2000";

pub async fn get_available_trigger_sources() -> Result<TriggerCollection, Box<dyn Error>> {
    let mut trigger_sources: TriggerCollection = HashMap::new();

    trigger_sources.insert(
        String::from(TriggerSource::TwitchPubSub.as_str()),
        Box::new(twitch_pub_sub::TwitchPubSub::new(TARGET_CHANNEL).await?),
    );

    trigger_sources.insert(
        String::from(TriggerSource::TwitchChat.as_str()),
        Box::new(twitch_chat::TwitchChat::new(String::from(TARGET_CHANNEL))),
    );

    Ok(trigger_sources)
}

#[derive(Debug, Clone)]
pub enum TriggerSource {
    TwitchPubSub,
    TwitchChat,
}

const TWITCH_CHAT: &str = "twitch_chat";
const TWITCH_PUB_SUB: &str = "twitch_pub_sub";
impl TriggerSource {
    pub fn as_str(&self) -> &'static str {
        match self {
            TriggerSource::TwitchChat => TWITCH_CHAT,
            TriggerSource::TwitchPubSub => TWITCH_PUB_SUB,
        }
    }

    pub fn from_str(val: &str) -> Self {
        if val == TWITCH_CHAT {
            TriggerSource::TwitchChat
        } else if val == TWITCH_PUB_SUB {
            return TriggerSource::TwitchPubSub;
        } else {
            panic!("Tried to construct an TriggerSource using an invalid string");
        }
    }
}

pub type TriggerCollection = HashMap<String, Box<dyn triggers::TriggerSource>>;
