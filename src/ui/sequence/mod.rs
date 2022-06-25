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
    trigger: trigger::Trigger,
}

#[derive(Debug, Clone)]
pub enum SequenceMessage {
    TriggerMessage(trigger::TriggerMessage),
}

impl Sequence {
    pub fn new(devices: &'static HashMap<String, Box<dyn DeviceTrait>>) -> Self {
        Sequence {
            trigger: trigger::Trigger::new(devices),
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
