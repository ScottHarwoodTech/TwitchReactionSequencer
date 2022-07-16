use iced;
use iced::{pick_list, Column, Element, PickList};
use std::collections::HashMap;

use crate::sequencer::device::DeviceTrait;

// Drop down list of trigger sources,
// Drop down list of actions on triggers

// Arguments?
// Container
#[derive(Debug, Clone)]
pub struct Action {
    selected_device: Option<String>,
    selected_action: Option<String>,
    devices: HashMap<String, Box<dyn DeviceTrait>>,
    devices_pick_list: pick_list::State<String>,
    action_pick_list: pick_list::State<String>,
}

#[derive(Debug, Clone)]
pub enum ActionMessage {
    DeviceSelected(String),
    DeviceActionSelected(String),
}

impl Action {
    pub fn new(devices: HashMap<String, Box<dyn DeviceTrait>>) -> Self {
        Action {
            selected_device: Some(String::from("timer")),
            selected_action: None,
            devices: devices.clone(),
            devices_pick_list: pick_list::State::new(),
            action_pick_list: pick_list::State::new(),
        }
    }

    pub fn update(&mut self, message: ActionMessage) {
        match message {
            ActionMessage::DeviceSelected(selected_device) => {
                self.selected_device = Some(selected_device.clone());

                if let Some(device) = self.devices.get(&selected_device) {
                    let mut device_action_keys = device.get_actions().keys().into_iter();
                    self.selected_action = Some(device_action_keys.next().unwrap().to_string());
                }
            }
            ActionMessage::DeviceActionSelected(selected_action) => {
                self.selected_action = Some(selected_action)
            }
        }
    }

    pub fn view(&mut self) -> Element<ActionMessage> {
        let mut keys: Vec<String> = Vec::new();

        for key in self.devices.keys() {
            keys.push(key.to_string());
        }

        let device_pick_list = PickList::new(
            &mut self.devices_pick_list,
            keys,
            self.selected_device.clone(),
            ActionMessage::DeviceSelected,
        );

        let device = self
            .devices
            .get(&(self.selected_device.clone().unwrap()))
            .unwrap();

        let mut trigger_events: Vec<String> = Vec::new();

        for key in device.get_actions().keys() {
            trigger_events.push(key.to_string());
        }

        let action_pick_list = PickList::new(
            &mut self.action_pick_list,
            trigger_events,
            self.selected_action.clone(),
            ActionMessage::DeviceActionSelected,
        );

        Column::new()
            .push(device_pick_list)
            .push(action_pick_list)
            .into()
    }
}
