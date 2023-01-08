pub mod ble;
pub mod timer;

use std::collections::HashMap;
use std::error::Error;

use super::device::DevicesCollection;

pub async fn setup_devices() -> Result<DevicesCollection, Box<dyn Error>> {
    let device_set: DevicesCollection = HashMap::new();

    let device_set = ble::util::setup_ble_devices(device_set).await?;
    let device_set = timer::setup(device_set);

    Ok(device_set)
}

#[derive(Debug, Clone)]
pub enum DeviceTypes {
    BunnyEars,
    Timer,
    BleDevice,
}

impl DeviceTypes {
    pub fn from_string(value: &String) -> Self {
        match value.as_str() {
            "BUNNY_EARS" => DeviceTypes::BunnyEars,
            "TIMER" => DeviceTypes::Timer,
            "BLE_DEVICE" => DeviceTypes::BleDevice,
            _ => panic!("Invalid"),
        }
    }
}
