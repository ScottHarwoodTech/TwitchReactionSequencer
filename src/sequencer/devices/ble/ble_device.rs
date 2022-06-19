use crate::sequencer::device::{DeviceAction, DeviceTrait};
use std::collections::HashMap;

pub struct BleDevice {
    id: String,
    name: String,
    actions: HashMap<String, Box<dyn DeviceAction>>,
}

impl BleDevice {
    pub fn new(id: &str, name: &str, actions: HashMap<String, Box<dyn DeviceAction>>) -> BleDevice {
        return BleDevice {
            id: String::from(id),
            name: String::from(name),
            actions: actions,
        };
    }
}

impl DeviceTrait for BleDevice {
    fn get_actions(&self) -> &HashMap<String, Box<dyn DeviceAction>> {
        return &self.actions;
    }
}
