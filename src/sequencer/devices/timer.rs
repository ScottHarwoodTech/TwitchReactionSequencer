use crate::sequencer::device;

use async_trait::async_trait;
use std::collections::HashMap;
use std::time::Duration;
use tokio::time;

const DEVICE_ID: &str = "timer";
const DEVICE_NAME: &str = "Timer";

const ACTION_DELAY_ID: &str = "delay";
const ACTION_DELAY_NAME: &str = "Delay";

#[derive(Debug)]
struct Delay {
    id: String,
    name: String,
}

impl Delay {
    pub fn new(id: &str, name: &str) -> Delay {
        return Delay {
            id: String::from(id),
            name: String::from(name),
        };
    }
}

#[async_trait]
impl device::DeviceAction for Delay {
    async fn action(&self, _arguments: Vec<serde_json::Value>) {
        time::sleep(Duration::from_secs(1)).await;
    }
}

pub fn setup(
    mut devices: HashMap<String, Box<dyn device::DeviceTrait>>,
) -> HashMap<String, Box<dyn device::DeviceTrait>> {
    devices.insert(
        DEVICE_ID.to_string(),
        Box::new(device::Device::new(
            DEVICE_ID,
            DEVICE_NAME,
            create_actions(),
        )),
    );

    return devices;
}

fn create_actions() -> HashMap<String, Box<dyn device::DeviceAction>> {
    let mut actions: HashMap<String, Box<dyn device::DeviceAction>> = HashMap::new();

    actions.insert(
        String::from("delay"),
        Box::new(Delay::new(ACTION_DELAY_ID, ACTION_DELAY_NAME)),
    );

    return actions;
}
