use crate::sequencer::device::{DeviceAction, DeviceTrait};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct BleDevice {
    id: String,
    name: String,
    actions: HashMap<String, Box<dyn DeviceAction>>,
}

impl BleDevice {
    pub fn new(id: &str, name: &str, actions: HashMap<String, Box<dyn DeviceAction>>) -> BleDevice {
        BleDevice {
            id: String::from(id),
            name: String::from(name),
            actions,
        }
    }
}

impl DeviceTrait for BleDevice {
    fn get_actions(&self) -> &HashMap<String, Box<dyn DeviceAction>> {
        &self.actions
    }
}

unsafe impl Send for BleDevice {}
