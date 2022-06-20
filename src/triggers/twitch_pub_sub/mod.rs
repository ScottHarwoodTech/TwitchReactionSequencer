use crate::sequencer::QueueEvent;
use crate::triggers::triggers::TriggerSource;
use async_trait::async_trait;
use futures_util::{future, SinkExt, StreamExt};
use native_tls::TlsConnector;
use pubsub::Topic;
use rand::{distributions::Alphanumeric, Rng};
use std::error::Error;
use tokio::sync::mpsc::Sender;
use tokio_tungstenite::tungstenite::protocol::{Message, WebSocketConfig};
use tokio_tungstenite::{connect_async_tls_with_config, Connector};
use twitch_api2::twitch_oauth2::{url, ClientId, ClientSecret, Scope, TwitchToken, UserToken};
use twitch_api2::{pubsub, TWITCH_PUBSUB_URL};

pub struct TwitchPubSub {
    target_channel: &'static str,
    user_token: UserToken,
}

impl TwitchPubSub {
    pub async fn new(target_channel: &'static str) -> Result<TwitchPubSub, Box<dyn Error>> {
        let mut builder = UserToken::builder(
            ClientId::from(std::env::var("TWITCH_CLIENT_ID").unwrap()),
            ClientSecret::from(std::env::var("TWITCH_SECRET").unwrap()),
            url::Url::parse("http://localhost")?,
        );

        builder.add_scope(Scope::ChannelReadRedemptions);

        let (url, _) = builder.generate_url();

        println!("Go to this page: {}", url);

        let input = rpassword::prompt_password(
            "Paste in the resulting adress after authenticating (input hidden): ",
        )?;

        println!("{}", input);

        let u = url::Url::parse(&input)?;

        let map: std::collections::HashMap<_, _> = u.query_pairs().collect();

        let user_token;
        match (map.get("state"), map.get("code")) {
            (Some(state), Some(code)) => {
                user_token = builder
                    .get_user_token(
                        &reqwest::Client::builder()
                            .redirect(reqwest::redirect::Policy::none())
                            .build()?,
                        state,
                        code,
                    )
                    .await?;
                println!("Got token: {:?}", user_token);
            }
            _ => match (map.get("error"), map.get("error_description")) {
                (
                    std::option::Option::Some(error),
                    std::option::Option::Some(error_description),
                ) => {
                    panic!(
                        "twitch errored with error: {} - {}",
                        error, error_description
                    );
                }
                _ => panic!("invalid url passed"),
            },
        };

        return Ok(TwitchPubSub {
            target_channel: target_channel,
            user_token: user_token,
        });
    }
}

fn nonce() -> String {
    let s: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(7)
        .map(char::from)
        .collect();
    return s;
}

#[async_trait]
impl TriggerSource for TwitchPubSub {
    async fn watch(&self, send_trigger: Sender<QueueEvent>) -> Result<(), Box<dyn Error>> {
        let channel_points_actions = pubsub::channel_points::ChannelPointsChannelV1 {
            channel_id: 216053282,
        }
        .into_topic();

        // Create the topic command to send to twitch
        let command = pubsub::listen_command(
            &[channel_points_actions],
            self.user_token.token().secret(),
            nonce(),
        )
        .expect("serializing failed");

        let (mut ws_stream, _) = connect_async_tls_with_config(
            TWITCH_PUBSUB_URL.as_str(),
            Some(WebSocketConfig::default()),
            Some(Connector::NativeTls(TlsConnector::new().unwrap())),
        )
        .await
        .expect("Failed to connect");
        println!("WebSocket handshake has been successfully completed");

        ws_stream.send(Message::text(&command)).await?;

        let fut = ws_stream.for_each(|msg| {
            let msg = msg.unwrap();
            println!("{}", msg);
            future::ready(())
        });

        fut.await;

        return Ok(());
    }
}
