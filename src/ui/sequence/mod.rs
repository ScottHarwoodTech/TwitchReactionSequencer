pub mod action;
pub mod trigger;

use iced;
use iced::{Element, Row};
use std::collections::HashMap;

use crate::sequencer::device::DeviceTrait;

// Drop down list of trigger sources,
// Drop down list of actions on triggers

// Arguments?
// Container

#[derive(Debug, Clone)]
pub struct Sequence {
    pub trigger: trigger::Trigger,

    state: SequenceState,
}

#[derive(Debug, Clone)]
pub enum SequenceState {
    Ready,
}

#[derive(Debug, Clone)]
pub enum SequenceMessage {
    TriggerMessage(trigger::TriggerMessage),
}

impl Sequence {
    pub fn new(devices: HashMap<String, Box<dyn DeviceTrait>>) -> Self {
        Sequence {
            trigger: trigger::Trigger::new(devices),
            state: SequenceState::Ready,
        }
    }

    pub fn update(&mut self, message: SequenceMessage) {
        match message {
            SequenceMessage::TriggerMessage(trigger_message) => {
                self.trigger.update(trigger_message)
            }
            _ => todo!(),
        }
    }

    pub fn view(&mut self) -> Element<SequenceMessage> {
        let trigger: Element<_> = self
            .trigger
            .view()
            .map(move |message| SequenceMessage::TriggerMessage(message));

        Row::new().push(trigger).into()
    }
}
