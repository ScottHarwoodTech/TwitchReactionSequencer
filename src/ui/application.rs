use std::collections::HashMap;
use std::fs;
use std::time::Duration;

use crate::sequencer::device::DeviceTrait;
use crate::sequencer::reaction_sequence;
use crate::triggers::triggers::TriggerSource;
use crate::ui::sequence;
use iced::{self, button, scrollable, Button, Column, Length, Row, Text};
use iced::{Command, Element};
use iced::{Rule, Scrollable};
use tokio::time;

use super::sequence::{Sequence, SequenceMessage};

#[derive(Debug)]
pub enum Application {
    Loading,
    Ready(State),
}

#[derive(Debug, Clone)]
pub struct State {
    sequences: Vec<sequence::Sequence>,
    scroll: scrollable::State,
    add_sequence_button: button::State,
    devices: HashMap<String, Box<dyn DeviceTrait>>,
    triggers: HashMap<String, Box<dyn TriggerSource>>,
}

#[derive(Debug, Clone)]
pub enum Message {
    Loaded(Result<State, LoadError>),
    SequenceMessage(usize, SequenceMessage),
    AddSequence,
}

#[derive(Debug, Clone)]
pub enum LoadError {
    FileError,
    FormatError,
}

async fn load_sequences(
    devices: HashMap<String, Box<dyn DeviceTrait>>,
    triggers: HashMap<String, Box<dyn TriggerSource>>,
) -> Result<State, LoadError> {
    let paths = fs::read_dir("./");
    let mut sequences = Vec::<Sequence>::new();
    if paths.is_ok() {
        for entry in paths.unwrap() {
            let path = entry.unwrap().path();
            if let Ok(file_content) = fs::read_to_string(path) {
                let sequencer: reaction_sequence::ReactionSequence =
                    serde_json::from_str(&file_content.as_str()).unwrap();

                sequences.push(Sequence::from_existing(
                    sequencer,
                    devices.clone(),
                    triggers.clone(),
                ))
            } else {
                return Err(LoadError::FileError);
            }
        }
    } else {
        return Err(LoadError::FileError);
    };

    return Ok(State {
        sequences: vec![],
        scroll: scrollable::State::new(),
        add_sequence_button: button::State::new(),
        devices: devices.clone(),
        triggers: triggers.clone(),
    });
}

impl iced::Application for Application {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = (
        HashMap<String, Box<dyn DeviceTrait>>,
        HashMap<String, Box<dyn TriggerSource>>,
    );

    type Theme = iced::Theme;

    fn theme(&self) -> Self::Theme {
        iced::Theme::Dark
    }

    fn new(
        flags: (
            HashMap<String, Box<dyn DeviceTrait>>,
            HashMap<String, Box<dyn TriggerSource>>,
        ),
    ) -> (Application, Command<Message>) {
        (
            Application::Loading,
            Command::perform(load_sequences(flags.0, flags.1), Message::Loaded),
        )
    }

    fn title(&self) -> String {
        return String::from("Twitch Reaction Sequencer");
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match self {
            Application::Loading => match message {
                Message::Loaded(Ok(state)) => {
                    *self = Application::Ready(state);
                }

                Message::Loaded(Err(_)) => {}
                _ => {}
            },

            Application::Ready(state) => match message {
                Message::SequenceMessage(i, sequence_message) => match sequence_message {
                    SequenceMessage::Delete => {
                        state.sequences.remove(i);
                    }
                    _ => {
                        if let Some(sequence) = state.sequences.get_mut(i) {
                            sequence.update(sequence_message);
                        }
                    }
                },
                Message::AddSequence => state
                    .sequences
                    .push(Sequence::new(state.devices.clone(), state.triggers.clone())),
                _ => {}
            },
        }

        Command::none()
    }

    fn view(&mut self) -> Element<Message> {
        match self {
            Application::Loading => Text::new("Loading").into(),
            Application::Ready(state) => {
                let mut c = Column::new().width(Length::Fill).spacing(1);

                let seqs: Element<_> = state
                    .sequences
                    .iter_mut()
                    .enumerate()
                    .fold(
                        Column::new().spacing(20).padding(10),
                        |column: Column<_>, (i, sequence)| {
                            column
                                .push(
                                    sequence
                                        .view()
                                        .map(move |message| Message::SequenceMessage(i, message)),
                                )
                                .push(Rule::horizontal(5))
                        },
                    )
                    .into();

                c = c.push(seqs);

                c = c.push(
                    Button::new(
                        &mut state.add_sequence_button,
                        Text::new("Add Sequence +").size(20),
                    )
                    .on_press(Message::AddSequence),
                ); // Add Sequence Button

                return Scrollable::new(&mut state.scroll).push(c).into();
            }
        }
    }
}
