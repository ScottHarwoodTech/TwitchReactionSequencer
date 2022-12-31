pub mod ble;
pub mod timer;

use crate::sequencer::device::DeviceTrait;
use std::collections::HashMap;
use std::error::Error;

use super::device::DevicesCollection;

pub async fn setup_devices() -> Result<DevicesCollection, Box<dyn Error>> {
    let device_set: DevicesCollection = HashMap::new();

    let device_set = ble::util::setup_ble_devices(device_set).await?;
    let device_set = timer::setup(device_set);

    return Ok(device_set);
}

pub enum DeviceTypes {
    BunnyEars,
    Delay,
}

impl DeviceTypes {
    pub fn from_string(value: &str) -> Self {
        match value {
            "BUNNY_EARS" => DeviceTypes::BunnyEars,
            "DELAY" => DeviceTypes::Delay,
            _ => panic!("Invalid"),
        }
    }
}
