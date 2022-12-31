use iced;
use iced::{pick_list, Column, Element, PickList};
use std::collections::HashMap;
use std::hash::Hash;

use crate::sequencer::reaction_sequence::{self, ReactionSequence, ReactionSequenceTrigger};
use crate::triggers::triggers::TriggerSource;
use crate::triggers::TriggerCollection;

// Drop down list of trigger sources,
// Drop down list of actions on triggers

// Arguments?
// Container
#[derive(Debug, Clone)]
pub struct Trigger {
    selected_trigger: Option<String>,
    selected_event: Option<String>,
    triggers: TriggerCollection,
    triggers_pick_list: pick_list::State<String>,
    action_pick_list: pick_list::State<String>,
}

#[derive(Debug, Clone)]
pub enum TriggerMessage {
    TriggerSelected(String),
    TriggerEventSelected(String),
}

impl Trigger {
    pub fn from_existing(
        triggers: TriggerCollection,
        trigger: reaction_sequence::ReactionSequenceTrigger,
    ) -> Self {
        Trigger {
            selected_trigger: Some(trigger.trigger_id),
            selected_event: Some(trigger.trigger_event_id),
            triggers: triggers.clone(),
            triggers_pick_list: pick_list::State::new(),
            action_pick_list: pick_list::State::new(),
        }
    }

    pub fn to_reaction_sequence_trigger(&self) -> reaction_sequence::ReactionSequenceTrigger {
        ReactionSequenceTrigger {
            trigger_event_id: self.selected_event.clone().unwrap_or_default(),
            trigger_id: self.selected_trigger.clone().unwrap_or_default(),
        }
    }
    pub fn new(triggers: TriggerCollection) -> Self {
        Trigger {
            selected_trigger: Some(String::from("twitch_pub_sub")),
            selected_event: None,
            triggers: triggers.clone(),
            triggers_pick_list: pick_list::State::new(),
            action_pick_list: pick_list::State::new(),
        }
    }

    pub fn update(&mut self, message: TriggerMessage) {
        match message {
            TriggerMessage::TriggerSelected(selected_device) => {
                self.selected_trigger = Some(selected_device.clone());

                if let Some(device) = self.triggers.get(&selected_device) {
                    let mut device_action_keys = device.get_events().keys().into_iter();

                    let device_action = device_action_keys.next();

                    if device_action.is_some() {
                        self.selected_event = Some(device_action.unwrap().to_string());
                    } else {
                        self.selected_event = None;
                    }
                }
            }
            TriggerMessage::TriggerEventSelected(selected_event) => {
                self.selected_event = Some(selected_event)
            }
        }
    }

    pub fn view(&mut self) -> Element<TriggerMessage> {
        let mut keys: Vec<String> = Vec::new();

        for key in self.triggers.keys() {
            keys.push(key.to_string());
        }

        let device_pick_list = PickList::new(
            &mut self.triggers_pick_list,
            keys,
            self.selected_trigger.clone(),
            |v| TriggerMessage::TriggerSelected(v),
        );

        let device = self
            .triggers
            .get(&(self.selected_trigger.clone().unwrap()))
            .unwrap();

        let mut trigger_events: Vec<String> = Vec::new();

        for key in device.get_events().keys() {
            trigger_events.push(key.to_string());
        }

        let event_pick_list = PickList::new(
            &mut self.action_pick_list,
            trigger_events,
            self.selected_event.clone(),
            TriggerMessage::TriggerEventSelected,
        );

        Column::new()
            .push(device_pick_list)
            .push(event_pick_list)
            .into()
    }
}
