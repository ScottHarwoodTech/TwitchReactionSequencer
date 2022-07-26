pub mod action;
pub mod trigger;

use crate::custom_widgets::horizontal_scrollable::{self};
use crate::sequencer::device::DeviceTrait;
use crate::sequencer::reaction_sequence::{self};
use crate::triggers::triggers::TriggerSource;
use iced::{self, button, Button, Column, Text};
use iced::{Element, Row};
use std::collections::HashMap;
use std::path::PathBuf;
use tokio::fs;

use self::action::ActionMessage;
use uuid;

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
    delete_sequence_button: button::State,
    scroll: horizontal_scrollable::State,
    filename: String,
    name: String,
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
    Delete,
}

impl Sequence {
    pub fn get_filename(self) -> String {
        return self.filename;
    }

    pub fn from_existing(
        sequence: reaction_sequence::ReactionSequence,
        filename: PathBuf,
        devices: HashMap<String, Box<dyn DeviceTrait>>,
        triggers: HashMap<String, Box<dyn TriggerSource>>,
    ) -> Self {
        return Sequence {
            devices: devices.clone(),
            trigger: trigger::Trigger::from_existing(triggers.clone(), sequence.trigger),
            actions: sequence
                .sequence
                .into_iter()
                .map(|a| action::Action::from_existing(devices.clone(), a.clone()))
                .collect(),
            state: SequenceState::Ready,
            add_action_button: button::State::new(),
            delete_sequence_button: button::State::new(),
            scroll: horizontal_scrollable::State::new(),
            filename: String::from(filename.to_str().unwrap()),
            name: sequence.name,
        };
    }

    pub async fn new(
        devices: HashMap<String, Box<dyn DeviceTrait>>,
        triggers: HashMap<String, Box<dyn TriggerSource>>,
    ) -> Self {
        let filename = format!(
            "./sequences/{}.json",
            uuid::Uuid::new_v4().to_hyphenated().to_string()
        ); //TODO: shouldnt be here
        fs::write(&filename, "").await.unwrap();

        Sequence {
            trigger: trigger::Trigger::new(triggers),
            devices: devices.clone(),
            actions: vec![action::Action::new(devices.clone())],
            state: SequenceState::Ready,
            add_action_button: button::State::new(),
            delete_sequence_button: button::State::new(),
            scroll: horizontal_scrollable::State::new(),
            filename: filename.clone(),
            name: String::from("Unnamed"),
        }
    }

    pub fn update(&mut self, message: SequenceMessage) {
        match message {
            SequenceMessage::TriggerMessage(trigger_message) => {
                self.trigger.update(trigger_message)
            }

            SequenceMessage::ActionMessage(i, action_message) => match action_message {
                ActionMessage::Delete => {
                    self.actions.remove(i);
                }
                _ => {
                    if let Some(action) = self.actions.get_mut(i) {
                        action.update(action_message);
                    }
                }
            },

            SequenceMessage::AddAction => {
                self.actions.push(action::Action::new(self.devices.clone()))
            }
            _ => todo!(),
        }
    }

    pub fn view(&mut self) -> Element<SequenceMessage> {
        let mut col = Column::new().spacing(20);

        let mut r = Row::new().spacing(20);

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
                Text::new("Add Action +").size(20),
            )
            .on_press(SequenceMessage::AddAction),
        );

        let delete_button = Button::new(&mut self.delete_sequence_button, Text::new("X"))
            .on_press(SequenceMessage::Delete);

        col = col.push(
            Row::new()
                .spacing(20)
                .push(delete_button)
                .push(Text::new(self.name.clone())),
        );

        col = col.push(r);
        return col.into();
    }
}
