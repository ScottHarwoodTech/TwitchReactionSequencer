use crate::sequencer::device::{DeviceTrait, DevicesCollection};
use crate::sequencer::devices::ble::bunny_ears;
use btleplug::api::{Central, Manager as _, ScanFilter};
use btleplug::platform::{Manager, Peripheral};
use std::collections::HashMap;
use std::error::Error;
use std::time::Duration;
use tokio::time;

pub async fn get_ble_peripherals() -> Result<Vec<Peripheral>, Box<dyn Error>> {
    let manager = Manager::new().await.unwrap();

    let adapters = manager.adapters().await?;
    let central = adapters.into_iter().nth(0).unwrap();

    central.start_scan(ScanFilter::default()).await?;
    // instead of waiting, you can use central.events() to get a stream which will
    // notify you of new devices, for an example of that see examples/event_driven_discovery.rs
    time::sleep(Duration::from_secs(2)).await;

    return Ok(central.peripherals().await.unwrap());
}

pub async fn setup_ble_devices(
    device_set: DevicesCollection,
) -> Result<DevicesCollection, Box<dyn Error>> {
    let ble_peripharals = get_ble_peripherals().await?;

    let device_set = bunny_ears::setup(device_set, ble_peripharals).await;

    return Ok(device_set);
}
