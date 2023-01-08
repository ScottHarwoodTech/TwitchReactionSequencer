use std::collections::HashMap;
use std::str::FromStr;

use crate::sequencer::device::{DeviceTrait, DevicesCollection, Parameter, ParameterName};
use crate::sequencer::devices::DeviceTypes;
use async_trait::async_trait;
use btleplug::api::{BDAddr, Peripheral as _, WriteType};
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
    device_type: DeviceTypes,
}

impl BunnyEars {
    pub async fn new(
        id: String,
        name: String,
        address: String,
        peripherals: &Vec<Peripheral>,
    ) -> Self {
        let microbit = find_mb(peripherals, address).await.unwrap(); //TODO: Handle is none

        BunnyEars {
            id,
            name,
            actions: create_actions(&microbit),
            device_type: DeviceTypes::BunnyEars,
        }
    }
}

impl DeviceTrait for BunnyEars {
    fn get_actions(&self) -> &HashMap<String, Box<dyn DeviceAction>> {
        &self.actions
    }

    fn get_name(&self) -> &String {
        &self.name
    }

    fn get_device_type(&self) -> &crate::sequencer::devices::DeviceTypes {
        &self.device_type
    }

    fn get_device_parameters() -> Vec<crate::sequencer::device::Parameter> {
        vec![Parameter::String(ParameterName::Address)]
    }
}

pub async fn setup(
    mut devices: DevicesCollection,
    peripherals: Vec<Peripheral>,
) -> DevicesCollection {
    let mb = find_mb(&peripherals, String::from("EB:A0:B4:C2:82:C8"))
        .await
        .unwrap();
    println!("found mb");
    devices.insert(
        DEVICE_ID.to_string(),
        Box::new(ble_device::BleDevice::new(
            String::from(DEVICE_ID),
            String::from(DEVICE_NAME),
            create_actions(&mb),
        )),
    );

    devices
}

async fn find_mb(peripherals: &Vec<Peripheral>, address: String) -> Option<Peripheral> {
    for p in peripherals {
        if p.properties().await.unwrap().unwrap().address
            == BDAddr::from_str(address.as_str()).unwrap()
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
