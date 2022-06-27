use iced;
use iced::{pick_list, Column, Element, PickList};
use std::collections::HashMap;
use std::rc::Rc;

use crate::sequencer::device::{DeviceAction, DeviceTrait};

// Drop down list of trigger sources,
// Drop down list of actions on triggers

// Arguments?
// Container
#[derive(Debug, Clone)]
pub struct Trigger {
    selected_device: Option<String>,
    action: Rc<Box<dyn DeviceAction>>,
    devices: HashMap<String, Rc<Box<dyn DeviceTrait>>>,
    device_pick_list: pick_list::State<String>,
}

#[derive(Debug, Clone)]
pub enum TriggerMessage {
    DeviceSelected(String),
}

impl Trigger {
    pub fn new(devices: &'static HashMap<String, Rc<Box<dyn DeviceTrait>>>) -> Self {
        let action = devices
            .get(&String::from("timer"))
            .unwrap()
            .get_actions()
            .get("delay")
            .unwrap();

        Trigger {
            selected_device: Some(String::from("timer")),
            action: Rc::new<action>,
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
