use std::collections::HashMap;

use crate::sequencer::device::{DeviceTrait, DevicesCollection};
use async_trait::async_trait;
use btleplug::api::{Peripheral as _, WriteType};
use btleplug::platform::Peripheral;
use uuid::Uuid;

const DEVICE_NAME: &str = "Bunny Ears";
const DEVICE_ID: &str = "bunnyEars";
const RX_CHARACTERISTIC: &str = "6E400003-B5A3-F393-E0A9-E50E24DCCA9E";

use crate::sequencer::device::DeviceAction;

use crate::sequencer::devices::ble::ble_device;

#[derive(Debug, Clone)]
struct LeftEar {
    id: String,
    name: String,
    mb: Peripheral,
}

impl LeftEar {
    pub fn new(id: &str, name: &str, mb: Peripheral) -> LeftEar {
        LeftEar {
            id: String::from(id),
            name: String::from(name),
            mb,
        }
    }
}

#[async_trait]
impl DeviceAction for LeftEar {
    async fn action(&self, _arguments: Vec<serde_json::Value>) {
        self.mb.connect().await.unwrap();
        self.mb.discover_services().await.unwrap();
        let chars = self.mb.characteristics();
        let rx_char = chars
            .iter()
            .find(|c| c.uuid == Uuid::parse_str(RX_CHARACTERISTIC).unwrap())
            .unwrap();
        let cmd = vec![0x48, 0x31, 0x38, 0x30, 0xA];
        self.mb
            .write(rx_char, &cmd, WriteType::WithoutResponse)
            .await
            .unwrap();
    }
}

#[derive(Debug, Clone)]
struct RightEar {
    id: String,
    name: String,
    mb: Peripheral,
}

impl RightEar {
    pub fn new(id: &str, name: &str, mb: Peripheral) -> RightEar {
        RightEar {
            id: String::from(id),
            name: String::from(name),
            mb,
        }
    }
}

#[async_trait]
impl DeviceAction for RightEar {
    async fn action(&self, _arguments: Vec<serde_json::Value>) {
        let chars = self.mb.characteristics();
        let rx_char = chars
            .iter()
            .find(|c| c.uuid == Uuid::parse_str(RX_CHARACTERISTIC).unwrap())
            .unwrap();
        let cmd = vec![0x48, 0x32, 0x38, 0x30, 0xA];
        self.mb
            .write(rx_char, &cmd, WriteType::WithoutResponse)
            .await
            .unwrap();
    }
}

#[derive(Debug, Clone)]
pub struct BunnyEars {
    id: String,
    name: String,
    actions: HashMap<String, Box<dyn DeviceAction>>,
}

impl BunnyEars {
    pub async fn new(id: String, name: String, peripherals: &Vec<Peripheral>) -> Self {
        let microbit = find_mb(peripherals).await.unwrap(); //TODO: Handle is none
                                                            //TODO: Recreate as find device by id?

        BunnyEars {
            id,
            name,
            actions: create_actions(&microbit),
        }
    }
}

impl DeviceTrait for BunnyEars {
    fn get_actions(&self) -> &HashMap<String, Box<dyn DeviceAction>> {
        &self.actions
    }
}

pub async fn setup(
    mut devices: DevicesCollection,
    peripherals: Vec<Peripheral>,
) -> DevicesCollection {
    let mb = find_mb(&peripherals).await.unwrap();
    println!("found mb");
    devices.insert(
        DEVICE_ID.to_string(),
        Box::new(ble_device::BleDevice::new(
            DEVICE_ID,
            DEVICE_NAME,
            create_actions(&mb),
        )),
    );

    devices
}

async fn find_mb(peripherals: &Vec<Peripheral>) -> Option<Peripheral> {
    for p in peripherals {
        if p.properties()
            .await
            .unwrap()
            .unwrap()
            .local_name
            .iter()
            .any(|name| name.contains("BBC micro:bit"))
        {
            return Some(p.clone());
        }
    }

    None
}

fn create_actions(mb: &Peripheral) -> HashMap<String, Box<dyn DeviceAction>> {
    let mut actions: HashMap<String, Box<dyn DeviceAction>> = HashMap::new();
    actions.insert(
        String::from("leftEar"),
        Box::new(LeftEar::new("leftEar", "Left Ear", mb.clone())),
    );

    actions.insert(
        String::from("rightEar"),
        Box::new(RightEar::new("rightEar", "Right Ear", mb.clone())),
    );

    actions
}
