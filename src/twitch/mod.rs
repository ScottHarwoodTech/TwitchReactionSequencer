use twitch_irc::login::StaticLoginCredentials;
use twitch_irc::TwitchIRCClient;
use twitch_irc::{ClientConfig, SecureTCPTransport};

pub struct TwitchController {
    target_channel: &'static str,
}

impl TwitchController {
    pub fn new(target_channel: &'static str) -> TwitchController {
        return TwitchController {
            target_channel: target_channel,
        };
    }

    pub async fn start(&self) -> () {
        // default configuration is to join chat as anonymous.
        let config = ClientConfig::default();
        let (mut incoming_messages, client) =
            TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(config);

        // first thing you should do: start consuming incoming messages,
        // otherwise they will back up.
        let join_handle = tokio::spawn(async move {
            while let Some(message) = incoming_messages.recv().await {
                println!("Received message: {:?}", message);
            }
        });

        // join a channel
        // This function only returns an error if the passed channel login name is malformed,
        // so in this simple case where the channel name is hardcoded we can ignore the potential
        // error with `unwrap`.
        client.join(self.target_channel.to_owned()).unwrap();
        return join_handle.await.unwrap();
    }
}
