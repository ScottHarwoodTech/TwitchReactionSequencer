use iced::{self, button, Button, Text};
use iced::{pick_list, Column, Element, PickList};

use crate::sequencer::device::DevicesCollection;
use crate::sequencer::reaction_sequence::{self, ReactionSequenceItemSequence};

// Drop down list of trigger sources,
// Drop down list of actions on triggers

// Arguments?
// Container
#[derive(Debug, Clone)]
pub struct Action {
    selected_device: Option<String>,
    selected_action: Option<String>,
    devices: DevicesCollection,
    devices_pick_list: pick_list::State<String>,
    action_pick_list: pick_list::State<String>,
    delete_button: button::State,
    id: String,
    arguments: Vec<serde_json::Value>,
}

#[derive(Debug, Clone)]
pub enum ActionMessage {
    DeviceSelected(String),
    DeviceActionSelected(String),
    Delete,
}

impl Action {
    pub fn from_existing(
        devices: DevicesCollection,
        sequence_event: reaction_sequence::ReactionSequenceItemSequence,
    ) -> Self {
        Action {
            selected_device: Some(sequence_event.device_id),
            selected_action: Some(sequence_event.device_action_id),
            devices: devices.clone(),
            devices_pick_list: pick_list::State::new(),
            action_pick_list: pick_list::State::new(),
            delete_button: button::State::new(),
            id: sequence_event.id,
            arguments: sequence_event.arguments,
        }
    }
    pub fn to_reaction_sequence_item(&self) -> reaction_sequence::ReactionSequenceItemSequence {
        ReactionSequenceItemSequence {
            device_action_id: self.selected_action.clone().unwrap_or_default(),
            device_id: self.selected_device.clone().unwrap_or_default(),
            id: self.id.clone(),
            arguments: self.arguments.clone(),
        }
    }

    pub fn new(devices: DevicesCollection) -> Self {
        Action {
            selected_device: Some(String::from("timer")),
            selected_action: None,
            devices: devices.clone(),
            devices_pick_list: pick_list::State::new(),
            action_pick_list: pick_list::State::new(),
            delete_button: button::State::new(),
            id: uuid::Uuid::new_v4().to_hyphenated().to_string(),
            arguments: vec![],
        }
    }

    pub fn update(&mut self, message: ActionMessage) {
        match message {
            ActionMessage::DeviceSelected(selected_device) => {
                self.selected_device = Some(selected_device.clone());

                if let Some(device) = self.devices.get(&selected_device) {
                    let mut device_action_keys = device.get_actions().keys();
                    self.selected_action = Some(device_action_keys.next().unwrap().to_string());
                }
            }
            ActionMessage::DeviceActionSelected(selected_action) => {
                self.selected_action = Some(selected_action)
            }
            _ => {}
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
            .push(
                Button::new(&mut self.delete_button, Text::new("X"))
                    .on_press(ActionMessage::Delete),
            )
            .push(device_pick_list)
            .push(action_pick_list)
            .into()
    }
}
