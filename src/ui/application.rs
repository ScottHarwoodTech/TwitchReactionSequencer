use std::collections::HashMap;
use tokio::fs;

use super::sequence::{Sequence, SequenceMessage};
use crate::sequencer::device::DeviceTrait;
use crate::triggers::triggers::TriggerSource;
use crate::ui::sequence;
use iced::{self, button, scrollable, Button, Column, Length, Text};
use iced::{Command, Element};
use iced::{Rule, Scrollable};
use iced_native::{window, Event};

#[derive(Debug)]
pub enum Application {
    Loading,
    Error(String),
    Ready(State),
    UnsavedCloseRequested(State),
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
    SequenceDeleted(Option<String>),
    SequenceCreated(Sequence),
    EventOccurred(iced_native::Event),
}

#[derive(Debug, Clone)]
pub enum LoadError {
    FileError,
    FormatError(String),
}

async fn load_sequences(
    devices: HashMap<String, Box<dyn DeviceTrait>>,
    triggers: HashMap<String, Box<dyn TriggerSource>>,
) -> Result<State, LoadError> {
    let paths = fs::read_dir("./sequences").await; //TODO: this path should be relative to a userdata folder
    let mut sequences = Vec::<Sequence>::new();
    if paths.is_ok() {
        let mut paths = paths.unwrap();

        while let Ok(Some(entry)) = paths.next_entry().await {
            if entry.metadata().await.unwrap().is_dir() {
                continue;
            }

            let path = entry.path();
            if let Ok(file_content) = fs::read(&path).await {
                let get_sequencer = serde_json::from_slice(&file_content);
                if get_sequencer.is_ok() {
                    let sequencer = get_sequencer.unwrap();
                    sequences.push(Sequence::from_existing(
                        sequencer,
                        path.clone(),
                        devices.clone(),
                        triggers.clone(),
                    ));
                } else {
                    return Err(LoadError::FormatError(
                        get_sequencer.err().unwrap().to_string(),
                    ));
                }
            } else {
                return Err(LoadError::FileError);
            }
        }
    } else {
        return Err(LoadError::FileError);
    };

    return Ok(State {
        sequences: sequences,
        scroll: scrollable::State::new(),
        add_sequence_button: button::State::new(),
        devices: devices.clone(),
        triggers: triggers.clone(),
    });
}

async fn delete_file(filename: String) -> Option<String> {
    if let Err(v) = fs::remove_file(filename).await {
        return Some(v.to_string());
    };

    return None;
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

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        iced_native::subscription::events().map(Message::EventOccurred)
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match self {
            Application::Loading => match message {
                Message::Loaded(Ok(state)) => {
                    *self = Application::Ready(state);
                }

                Message::Loaded(Err(load_error)) => match load_error {
                    LoadError::FormatError(msg) => *self = Application::Error(msg),
                    _ => {}
                },
                _ => {}
            },

            Application::Ready(state) => match message {
                Message::SequenceMessage(i, sequence_message) => match sequence_message {
                    SequenceMessage::Delete => {
                        let removed_item = state.sequences.remove(i);
                        return Command::perform(
                            delete_file(removed_item.get_filename()),
                            Message::SequenceDeleted,
                        );
                    }
                    _ => {
                        if let Some(sequence) = state.sequences.get_mut(i) {
                            sequence.update(sequence_message);
                        }
                    }
                },
                Message::AddSequence => {
                    return Command::perform(
                        Sequence::new(state.devices.clone(), state.triggers.clone()),
                        Message::SequenceCreated,
                    );
                }
                Message::SequenceCreated(sequence) => state.sequences.push(sequence),

                Message::SequenceDeleted(msg) => {
                    if msg.is_some() {
                        *self = Application::Error(msg.unwrap())
                    }
                }

                Message::EventOccurred(event) => {
                    if let Event::Window(window::Event::CloseRequested) = event {
                        *self = Application::UnsavedCloseRequested(state.clone());
                    }
                }

                _ => {}
            },

            _ => {}
        }

        Command::none()
    }

    fn view(&mut self) -> Element<Message> {
        match self {
            Application::Loading => Text::new("Loading").into(),
            Application::Error(msg) => Text::new(msg.clone()).into(),
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
            Application::UnsavedCloseRequested(_state) => {
                Text::new("You just tried to exit when unsaved").into()
            }
        }
    }
}
