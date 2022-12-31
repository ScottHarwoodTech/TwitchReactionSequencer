use std::collections::HashMap;

use serde_json::{Map, Value};

use crate::{
    sequencer::{
        device::{DeviceTrait, DevicesCollection},
        devices::{
            ble::{bunny_ears::BunnyEars, util::get_ble_peripherals},
            timer::Timer,
            DeviceTypes,
        },
    },
    settings::Settings,
};

pub async fn format_configured_devices(settings: Settings) -> DevicesCollection {
    let mut devices = HashMap::<String, Box<dyn DeviceTrait>>::new();
    for device in settings.configured_devices {
        let map = match device.as_object() {
            Some(x) => x,
            None => panic!("Invalid object"),
        };

        let device_id = map.get("id").unwrap().as_str().unwrap();
        let parsed_device = parse_device(map).await;

        devices.insert(String::from(device_id), parsed_device);
    }
    return devices;
}

pub async fn parse_device(map: &Map<String, Value>) -> Box<dyn DeviceTrait> {
    let id = String::from(map.get("id").unwrap().as_str().unwrap());
    let name = String::from(map.get("name").unwrap().as_str().unwrap());
    let ble_ps = get_ble_peripherals().await.unwrap(); //make lazy?

    // Need to handle perameters
    let device: Box<dyn DeviceTrait> =
        match DeviceTypes::from_string(map.get("type").unwrap().as_str().unwrap()) {
            DeviceTypes::BunnyEars => {
                Box::new(
                    BunnyEars::new(
                        id.clone(),
                        name.clone(),
                        &ble_ps, //TODO Handle refresh!,
                    )
                    .await,
                )
            }
            DeviceTypes::Delay => Box::new(Timer::new(id.clone(), name.clone())),
        };

    return device;
}
