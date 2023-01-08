use crate::sequencer::device::{self, DeviceTrait, Parameter};
use crate::sequencer::devices::DeviceTypes;

use async_trait::async_trait;
use std::collections::HashMap;
use std::time::Duration;
use tokio::time;

const DEVICE_ID: &str = "timer";
const DEVICE_NAME: &str = "Timer";

const ACTION_DELAY_ID: &str = "delay";
const ACTION_DELAY_NAME: &str = "Delay";

#[derive(Debug, Clone)]
pub struct Timer {
    id: String,
    name: String,
    actions: HashMap<String, Box<dyn device::DeviceAction>>,
    device_type: DeviceTypes,
}

impl Timer {
    pub fn new(id: String, name: String) -> Self {
        return Timer {
            id,
            name,
            actions: create_actions(),
            device_type: DeviceTypes::Timer,
        };
    }
}

impl DeviceTrait for Timer {
    fn get_actions(&self) -> &HashMap<String, Box<dyn device::DeviceAction>> {
        &self.actions
    }

    fn get_name(&self) -> &String {
        &self.name
    }

    fn get_device_type(&self) -> &super::DeviceTypes {
        &self.device_type
    }

    fn get_device_parameters() -> Vec<device::Parameter> {
        vec![]
    }
}

#[derive(Debug, Clone)]
struct Delay {
    id: String,
    name: String,
}

impl Delay {
    pub fn new(id: String, name: String) -> Delay {
        Delay { id, name }
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
        Box::new(Timer::new(
            String::from(DEVICE_ID),
            String::from(DEVICE_NAME),
        )),
    );

    devices
}

fn create_actions() -> HashMap<String, Box<dyn device::DeviceAction>> {
    let mut actions: HashMap<String, Box<dyn device::DeviceAction>> = HashMap::new();

    actions.insert(
        String::from("delay"),
        Box::new(Delay::new(
            String::from(ACTION_DELAY_ID),
            String::from(ACTION_DELAY_NAME),
        )),
    );

    actions
}
