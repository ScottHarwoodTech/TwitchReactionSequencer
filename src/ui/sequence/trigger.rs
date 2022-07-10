use iced;
use iced::{pick_list, Column, Element, PickList};
use std::collections::HashMap;

use crate::sequencer::device::DeviceTrait;

// Drop down list of trigger sources,
// Drop down list of actions on triggers

// Arguments?
// Container
#[derive(Debug, Clone)]
pub struct Trigger {
    selected_device: Option<String>,
    devices: HashMap<String, Box<dyn DeviceTrait>>,
    device_pick_list: pick_list::State<String>,
}

#[derive(Debug, Clone)]
pub enum TriggerMessage {
    DeviceSelected(String),
}

impl Trigger {
    pub fn new(devices: HashMap<String, Box<dyn DeviceTrait>>) -> Self {
        let _action = devices
            .get(&String::from("timer"))
            .unwrap()
            .get_actions()
            .get("delay")
            .unwrap();

        Trigger {
            selected_device: Some(String::from("timer")),
            devices: devices.clone(),
            device_pick_list: pick_list::State::new(),
        }
    }

    pub fn view(&mut self) -> Element<TriggerMessage> {
        let mut keys: Vec<String> = Vec::new();

        for key in self.devices.keys() {
            keys.push(key.to_string());
        }

        let pick_list = PickList::new(
            &mut self.device_pick_list,
            keys,
            Some(String::from("")),
            TriggerMessage::DeviceSelected,
        );

        Column::new().push(pick_list).into()
    }
}
