use std::collections::HashMap;

use async_trait::async_trait;
use btleplug::api::{Peripheral as _, WriteType};
use btleplug::platform::Peripheral;
use uuid::Uuid;

const DEVICE_NAME: &str = "Bunny Ears";
const DEVICE_ID: &str = "bunnyEars";
const RX_CHARACTERISTIC: &str = "6E400003-B5A3-F393-E0A9-E50E24DCCA9E";

use crate::sequencer::device;
use crate::sequencer::device::DeviceAction;

struct LeftEar {
    id: String,
    name: String,
    mb: Peripheral,
}

impl LeftEar {
    pub fn new(id: &str, name: &str, mb: Peripheral) -> LeftEar {
        return LeftEar {
            id: String::from(id),
            name: String::from(name),
            mb: mb,
        };
    }
}

#[async_trait]
impl DeviceAction for LeftEar {
    async fn action(&self, _arguments: Vec<serde_json::Value>) {
        let chars = self.mb.characteristics();
        let rx_char = chars
            .iter()
            .find(|c| c.uuid == Uuid::parse_str(RX_CHARACTERISTIC).unwrap())
            .unwrap();
        let cmd = vec![0x48, 0x31, 0x38, 0x30, 0xA];
        self.mb
            .write(&rx_char, &cmd, WriteType::WithoutResponse)
            .await
            .unwrap();
    }
}

pub fn setup(
    mut devices: HashMap<&str, device::Device>,
    peripheral: Peripheral,
) -> HashMap<&str, device::Device> {
    devices.insert(
        DEVICE_ID,
        device::Device::new(DEVICE_ID, DEVICE_NAME, create_actions(&peripheral)),
    );

    return devices;
}

fn create_actions(mb: &Peripheral) -> HashMap<String, Box<dyn device::DeviceAction>> {
    let mut actions: HashMap<String, Box<dyn device::DeviceAction>> = HashMap::new();
    let local_mb = mb.clone();
    actions.insert(
        String::from("1"),
        Box::new(LeftEar::new("leftEar", "leftEar", local_mb)),
    );

    return actions;
}
