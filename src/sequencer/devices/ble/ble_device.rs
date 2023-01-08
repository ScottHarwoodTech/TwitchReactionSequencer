use crate::sequencer::{
    device::{DeviceAction, DeviceTrait, Parameter},
    devices::DeviceTypes,
};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct BleDevice {
    id: String,
    name: String,
    actions: HashMap<String, Box<dyn DeviceAction>>,
    device_type: DeviceTypes,
}

impl BleDevice {
    pub fn new(
        id: String,
        name: String,
        actions: HashMap<String, Box<dyn DeviceAction>>,
    ) -> BleDevice {
        BleDevice {
            id: id,
            name: name,
            actions,
            device_type: DeviceTypes::BleDevice,
        }
    }
}

impl DeviceTrait for BleDevice {
    fn get_actions(&self) -> &HashMap<String, Box<dyn DeviceAction>> {
        &self.actions
    }

    fn get_name(&self) -> &String {
        &self.name
    }

    fn get_device_type(&self) -> &DeviceTypes {
        &self.device_type
    }

    fn get_device_parameters() -> Vec<crate::sequencer::device::Parameter> {
        vec![]
    }
}

unsafe impl Send for BleDevice {}
