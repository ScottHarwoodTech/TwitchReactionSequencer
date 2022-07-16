pub mod action;
pub mod trigger;

use crate::custom_widgets::horizontal_scrollable::{self, HorizontalScrollable};
use crate::sequencer::device::DeviceTrait;
use crate::triggers::triggers::TriggerSource;
use iced::{self, button, Button, Text};
use iced::{Element, Row};
use std::collections::HashMap;

// Drop down list of trigger sources,
// Drop down list of actions on triggers

// Arguments?
// Container

#[derive(Debug, Clone)]
pub struct Sequence {
    devices: HashMap<String, Box<dyn DeviceTrait>>,
    trigger: trigger::Trigger,
    actions: Vec<action::Action>,
    state: SequenceState,
    add_action_button: button::State,
    scroll: horizontal_scrollable::State,
}

#[derive(Debug, Clone)]
pub enum SequenceState {
    Ready,
}

#[derive(Debug, Clone)]
pub enum SequenceMessage {
    TriggerMessage(trigger::TriggerMessage),
    ActionMessage(usize, action::ActionMessage),
    AddAction,
}

impl Sequence {
    pub fn new(
        devices: HashMap<String, Box<dyn DeviceTrait>>,
        triggers: HashMap<String, Box<dyn TriggerSource>>,
    ) -> Self {
        Sequence {
            trigger: trigger::Trigger::new(triggers),
            devices: devices.clone(),
            actions: vec![action::Action::new(devices.clone())],
            state: SequenceState::Ready,
            add_action_button: button::State::new(),
            scroll: horizontal_scrollable::State::new(),
        }
    }

    pub fn update(&mut self, message: SequenceMessage) {
        match message {
            SequenceMessage::TriggerMessage(trigger_message) => {
                self.trigger.update(trigger_message)
            }

            SequenceMessage::ActionMessage(i, action_message) => {
                if let Some(action) = self.actions.get_mut(i) {
                    action.update(action_message);
                }
            }

            SequenceMessage::AddAction => {
                self.actions.push(action::Action::new(self.devices.clone()))
            }
            _ => todo!(),
        }
    }

    pub fn view(&mut self) -> Element<SequenceMessage> {
        let mut r = Row::new().spacing(20).height(iced::Length::FillPortion(30));

        let trigger: Element<_> = self
            .trigger
            .view()
            .map(move |message| SequenceMessage::TriggerMessage(message));

        r = r.push(trigger);

        r = self
            .actions
            .iter_mut()
            .enumerate()
            .fold(r, |row, (i, action)| {
                row.push(
                    action
                        .view()
                        .map(move |message| SequenceMessage::ActionMessage(i, message)),
                )
            });

        r = r.push(
            Button::new(
                &mut self.add_action_button,
                Row::new().spacing(10).push(Text::new("+").size(20)),
            )
            .on_press(SequenceMessage::AddAction),
        );

        return HorizontalScrollable::new(&mut self.scroll).push(r).into();
    }
}
