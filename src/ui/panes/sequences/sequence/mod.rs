pub mod action;
pub mod trigger;

use crate::custom_widgets::horizontal_scrollable::{self};
use crate::sequencer::device::DevicesCollection;
use crate::sequencer::reaction_sequence::{self, ReactionSequence};

use crate::triggers::TriggerCollection;
use iced::{self, button, Button, Column, Text};
use iced::{Element, Row};

use std::path::PathBuf;

use self::action::ActionMessage;
use uuid;

// Drop down list of trigger sources,
// Drop down list of actions on triggers

// Arguments?
// Container

#[derive(Debug, Clone)]
pub struct Sequence {
    devices: DevicesCollection,
    trigger: trigger::Trigger,
    actions: Vec<action::Action>,
    state: SequenceState,
    add_action_button: button::State,
    delete_sequence_button: button::State,
    scroll: horizontal_scrollable::State,
    filename: String,
    name: String,
    id: String,
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
        self.filename
    }

    pub fn from_existing(
        sequence: reaction_sequence::ReactionSequence,
        filename: PathBuf,
        devices: DevicesCollection,
        triggers: TriggerCollection,
    ) -> Self {
        return Sequence {
            devices: devices.clone(),
            trigger: trigger::Trigger::from_existing(triggers.clone(), sequence.trigger),
            actions: sequence
                .sequence
                .into_iter()
                .map(|a| action::Action::from_existing(devices.clone(), a))
                .collect(),
            state: SequenceState::Ready,
            add_action_button: button::State::new(),
            delete_sequence_button: button::State::new(),
            scroll: horizontal_scrollable::State::new(),
            filename: String::from(filename.to_str().unwrap()),
            name: sequence.name,
            id: sequence.id,
        };
    }
    pub fn to_reaction_seqeunce(&self) -> reaction_sequence::ReactionSequence {
        ReactionSequence {
            name: self.name.clone(),
            trigger: self.trigger.to_reaction_sequence_trigger(),
            sequence: self
                .actions
                .clone()
                .into_iter()
                .map(|a| a.to_reaction_sequence_item())
                .collect(),
            id: self.id.clone(),
        }
    }

    pub async fn new(devices: DevicesCollection, triggers: TriggerCollection) -> Self {
        let id = uuid::Uuid::new_v4().to_hyphenated().to_string();
        let filename = format!("./TRS/sequences/{}.json", &id); //TODO: shouldnt be here

        Sequence {
            trigger: trigger::Trigger::new(triggers),
            devices: devices.clone(),
            actions: vec![action::Action::new(devices.clone())],
            state: SequenceState::Ready,
            add_action_button: button::State::new(),
            delete_sequence_button: button::State::new(),
            scroll: horizontal_scrollable::State::new(),
            filename,
            name: String::from("Unnamed"),
            id,
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

        let trigger: Element<_> = self.trigger.view().map(SequenceMessage::TriggerMessage);

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
        col.into()
    }
}
