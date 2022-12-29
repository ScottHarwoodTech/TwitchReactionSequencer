use std::time::SystemTime;
use std::{collections::HashMap, error::Error};

use async_trait::async_trait;
use futures_util::select;
use futures_util::FutureExt;
use tokio::sync::{mpsc::Sender, watch};
use twitch_irc::login::StaticLoginCredentials;
use twitch_irc::ClientConfig;
use twitch_irc::SecureTCPTransport;
use twitch_irc::TwitchIRCClient;

use crate::sequencer::{self, QueueEvent};

use super::{triggers::TriggerEvent, triggers::TriggerSource};

#[derive(Debug, Clone)]
pub struct TwitchChat {
    target_channel: String,
    trigger_events: HashMap<String, Box<dyn TriggerEvent>>,
}

impl TwitchChat {
    pub fn new(target_channel: String) -> Self {
        return TwitchChat {
            target_channel: target_channel.clone(),
            trigger_events: HashMap::new(),
        };
    }
}

#[async_trait]
impl TriggerSource for TwitchChat {
    async fn watch(
        &self,
        send_trigger: Sender<QueueEvent>,
        mut watcher: watch::Receiver<()>,
    ) -> Result<(), Box<dyn Error>> {
        let config = ClientConfig::default();
        let (mut incoming_messages, client) =
            TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(config);

        let fut = tokio::spawn(async move {
            let fut = async {
                while let Some(message) = incoming_messages.recv().await {
                    println!(
                        "Received twitch chat msg: {:?} {:?} ",
                        SystemTime::now(),
                        message
                    );
                    send_trigger
                        .send(QueueEvent {
                            trigger_source: crate::triggers::TriggerSource::TwitchChat,
                            trigger_event_id: String::from(""),
                        })
                        .await
                        .unwrap()
                }
            };
            select!(
            _x = fut.fuse()  => println!("Listener Crashed dafk"), //TODO: Should be a panic
            _y = watcher.changed().fuse() => println!("Stopped by controll flow")
            )
        });

        client.join(self.target_channel.to_owned()).unwrap();

        fut.await.unwrap();
        return Ok(());
    }

    fn get_events(&self) -> &HashMap<String, Box<dyn TriggerEvent>> {
        return &self.trigger_events;
    }
}
