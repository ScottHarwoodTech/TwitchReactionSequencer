use pubsub::Topic;
use twitch_api2::pubsub;

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
        // We want to subscribe to moderator actions on channel with id 1234
        // as if we were a user with id 4321 that is moderator on the channel.
        let chat_mod_actions =
            pubsub::channel_points::ChannelPointsChannelV1 { channel_id: 1234 }.into_topic();
        // Create the topic command to send to twitch
        let command = pubsub::listen_command(
            &[chat_mod_actions],
            "authtoken",
            "super se3re7 random string",
        )
        .expect("serializing failed");
        // Send the message with your favorite websocket client
        send_command(command).unwrap();
        // To parse the websocket messages, use pubsub::Response::parse
    }
}
