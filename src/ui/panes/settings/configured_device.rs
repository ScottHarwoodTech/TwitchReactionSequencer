use std::collections::HashMap;

use serde_json::{Map, Value};

use crate::{
    sequencer::{
        device::{DeviceTrait, DevicesCollection, Parameter, ParameterName},
        devices::{
            ble::{ble_device::BleDevice, bunny_ears::BunnyEars, util::get_ble_peripherals},
            timer::Timer,
            DeviceTypes,
        },
    },
    settings::{Settings, SettingsItemConfiguredDevices},
};

pub async fn format_configured_devices(settings: Settings) -> DevicesCollection {
    let mut devices = HashMap::<String, Box<dyn DeviceTrait>>::new();
    for device in settings.configured_devices {
        let device_id = device.id.clone();
        let parsed_device = parse_device(device).await;

        devices.insert(String::from(device_id), parsed_device);
    }

    println!("{:?}", devices);
    devices
}

pub enum ValidationError {
    MissingValue(ParameterName),
}

fn validate_parameters(
    device: SettingsItemConfiguredDevices,
    params: Vec<Parameter>,
) -> Result<(), ValidationError> {
    for parameter in params {
        match parameter {
            Parameter::String(name) => match name {
                ParameterName::Address => {
                    //TODO: This is not a scalable solution
                    if device.address.is_none() {
                        return Err(ValidationError::MissingValue(ParameterName::Address));
                    }
                }
            },
        }
    }

    return Ok(());
}

pub async fn parse_device(device: SettingsItemConfiguredDevices) -> Box<dyn DeviceTrait> {
    let ble_ps = get_ble_peripherals().await.unwrap(); //make lazy?

    // Need to handle perameters
    let device: Box<dyn DeviceTrait> = match DeviceTypes::from_string(&device.device_type) {
        DeviceTypes::BunnyEars => {
            match validate_parameters(device.clone(), BunnyEars::get_device_parameters()) {
                Err(_v) => panic!("Invalid device"), //TODO: Correctly surface this error
                Ok(params) => params,
            };

            Box::new(
                BunnyEars::new(
                    device.id.clone(),
                    device.name.clone(),
                    device.address.unwrap(),
                    &ble_ps, //TODO Handle refresh!,
                )
                .await,
            )
        }
        DeviceTypes::Timer => Box::new(Timer::new(device.id.clone(), device.name.clone())),
        DeviceTypes::BleDevice => Box::new(BleDevice::new(
            device.id.clone(),
            device.name.clone(),
            HashMap::new(),
        )),
    };

    device
}
