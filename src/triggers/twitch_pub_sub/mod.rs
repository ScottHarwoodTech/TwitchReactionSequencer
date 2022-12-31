use crate::sequencer::QueueEvent;
use crate::triggers::triggers::TriggerSource;
use async_trait::async_trait;
use futures_util::FutureExt;
use futures_util::{future, select, SinkExt, StreamExt};
use native_tls::TlsConnector;
use pubsub::Topic;
use std::collections::HashMap;
use std::error::Error;
use textnonce::TextNonce;
use tokio::sync::mpsc::Sender;
use tokio::sync::watch;
use tokio_tungstenite::tungstenite::protocol::{Message, WebSocketConfig};
use tokio_tungstenite::{connect_async_tls_with_config, Connector};
use twitch_api2::twitch_oauth2::{
    url, AccessToken, ClientId, ClientSecret, RefreshToken, Scope, TwitchToken, UserToken,
};
use twitch_api2::{pubsub, TWITCH_PUBSUB_URL};

#[derive(Debug, Clone)]
pub struct TwitchPubSub {
    target_channel: &'static str,
    user_token: UserToken,
    trigger_events: HashMap<String, Box<dyn TriggerEvent>>,
}
use std::fs::OpenOptions;
use std::io::prelude::*;

use super::triggers::TriggerEvent;

impl TwitchPubSub {
    pub async fn new(target_channel: &'static str) -> Result<TwitchPubSub, Box<dyn Error>> {
        let user_token = TwitchPubSub::get_user_token().await?;
        Ok(TwitchPubSub {
            target_channel,
            user_token,
            trigger_events: HashMap::new(),
        })
    }

    async fn get_user_token() -> Result<UserToken, Box<dyn Error>> {
        let twitch_token = std::env::var_os("TWITCH_TOKEN");
        let refresh_token = std::env::var_os("TWITCH_REFRESH_TOKEN");
        let secret = std::env::var_os("TWITCH_SECRET")
            .unwrap()
            .into_string()
            .unwrap();
        let client_id = std::env::var_os("TWITCH_CLIENT_ID");

        if twitch_token.is_none() || refresh_token.is_none() {
            return Ok(TwitchPubSub::auth_flow().await.unwrap());
        }

        let (twitch_token, _, refresh_token) = twitch_api2::twitch_oauth2::refresh_token(
            &reqwest::Client::builder()
                .redirect(reqwest::redirect::Policy::none())
                .build()?,
            &RefreshToken::new(refresh_token.unwrap().into_string().unwrap()),
            &ClientId::new(client_id.unwrap().into_string().unwrap()),
            &ClientSecret::new(secret.clone()),
        )
        .await?;

        let user_token = UserToken::from_existing(
            &reqwest::Client::builder()
                .redirect(reqwest::redirect::Policy::none())
                .build()?,
            AccessToken::new(twitch_token.into_string()),
            RefreshToken::new(refresh_token.unwrap().into_string()),
            ClientSecret::new(secret),
        )
        .await;

        let user_token = user_token.unwrap();

        println!("{:?}", user_token.is_elapsed());
        Ok(user_token)
    }

    async fn auth_flow() -> Result<UserToken, Box<dyn Error>> {
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

        // TODO: Be Naughty and save straight into .env
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open("./.env")
            .unwrap();

        writeln!(
            file,
            "{}",
            format!("TWITCH_TOKEN=\"{}\"", user_token.token().secret())
        )?;

        writeln!(
            file,
            "{}",
            format!(
                "TWITCH_REFRESH_TOKEN=\"{}\"",
                user_token.clone().refresh_token.unwrap().into_string()
            )
        )?;

        Ok(user_token)
    }
}

#[async_trait]
impl TriggerSource for TwitchPubSub {
    async fn watch(
        &self,
        _send_trigger: Sender<QueueEvent>,
        mut watcher: watch::Receiver<()>,
    ) -> Result<(), Box<dyn Error>> {
        let channel_points_actions = pubsub::channel_points::ChannelPointsChannelV1 {
            channel_id: 216053282,
        }
        .into_topic();

        // Create the topic command to send to twitch
        let command = pubsub::listen_command(
            &[channel_points_actions],
            self.user_token.token().secret(),
            TextNonce::new().into_string().as_str(),
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

        select!(
            _x = fut.fuse() => println!("ws_stream stopped"),
            _y = watcher.changed().fuse() => {
                println!("ws_stream_killed by watcher");
            }
        );

        return Ok(());
    }

    fn get_events(&self) -> &HashMap<String, Box<dyn TriggerEvent>> {
        &self.trigger_events
    }
}
