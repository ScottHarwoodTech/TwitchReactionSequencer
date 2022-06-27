pub mod ble;
pub mod timer;

use crate::sequencer::device::DeviceTrait;
use std::collections::HashMap;
use std::error::Error;

pub async fn setup_devices() -> Result<HashMap<String, Box<dyn DeviceTrait>>, Box<dyn Error>> {
    let device_set: HashMap<String, Box<dyn DeviceTrait>> = HashMap::new();

    let device_set = ble::util::setup_ble_devices(device_set).await?;
    let device_set = timer::setup(device_set);

    return Ok(device_set);
}
