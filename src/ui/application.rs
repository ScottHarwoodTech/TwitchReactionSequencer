use std::collections::HashMap;
use std::io::ErrorKind;
use tokio::fs;

use super::sequence::{Sequence, SequenceMessage};
use crate::sequencer::device::DeviceTrait;
use crate::triggers::triggers::TriggerSource;
use crate::ui::sequence;
use iced::{self, button, keyboard, scrollable, Button, Column, Length, Row, Text};
use iced::{Command, Element};
use iced::{Rule, Scrollable};
use iced_native::{window, Event};

#[derive(Debug)]
pub enum Application {
    Loading,
    Error(String),
    Ready(State),
    UnsavedCloseRequested(State),
    ShouldExit,
}

#[derive(Debug, Clone)]
pub struct State {
    sequences: Vec<sequence::Sequence>,
    scroll: scrollable::State,
    add_sequence_button: button::State,
    save_button: button::State,
    devices: HashMap<String, Box<dyn DeviceTrait>>,
    triggers: HashMap<String, Box<dyn TriggerSource>>,
    tainted: bool,
}

#[derive(Debug, Clone)]
pub enum Message {
    Loaded(Result<State, LoadError>),
    SequenceMessage(usize, SequenceMessage),
    AddSequence,
    SequenceDeleted(Option<String>),
    SequenceCreated(Sequence),
    EventOccurred(iced_native::Event),
    Saved(Option<SaveError>),
    Save,
}

#[derive(Debug, Clone)]
pub enum LoadError {
    FileError,
    FormatError(String),
}

#[derive(Debug, Clone)]
pub enum SaveError {
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
        save_button: button::State::new(),
        devices: devices.clone(),
        triggers: triggers.clone(),
        tainted: false,
    });
}

async fn save_sequences(sequences: Vec<Sequence>) -> Option<SaveError> {
    for sequence in sequences {
        let json = serde_json::to_string_pretty(&sequence.to_reaction_seqeunce());

        if json.is_err() {
            return Some(SaveError::FormatError(json.unwrap_err().to_string()));
        }

        fs::write(&sequence.get_filename(), &json.unwrap())
            .await
            .unwrap();
    }

    return None;
}

async fn delete_file(filename: String) -> Option<String> {
    if let Err(v) = fs::remove_file(filename).await {
        if v.kind() != ErrorKind::NotFound {
            return Some(v.to_string());
        }
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

    fn should_exit(&self) -> bool {
        match self {
            Application::ShouldExit => true,
            _ => false,
        }
    }

    fn title(&self) -> String {
        let tainted = match self {
            Application::Loading => false,
            Application::Ready(state) | Application::UnsavedCloseRequested(state) => state.tainted,
            _ => false,
        };

        format!(
            "Twitch Reaction Sequencer{}",
            if tainted { "*" } else { "" }
        )
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

            Application::UnsavedCloseRequested(state) | Application::Ready(state) => {
                match message {
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

                    Message::SequenceCreated(sequence) => {
                        state.sequences.push(sequence);
                        *self = Application::Ready(State {
                            tainted: true,
                            ..state.clone()
                        })
                    }

                    Message::SequenceDeleted(msg) => {
                        if msg.is_some() {
                            *self = Application::Error(msg.unwrap())
                        } else {
                            *self = Application::Ready(State {
                                tainted: true,
                                ..state.clone()
                            });
                        }
                    }

                    Message::EventOccurred(event) => {
                        if let Event::Window(window::Event::CloseRequested) = event {
                            if state.tainted {
                                *self = Application::UnsavedCloseRequested(state.clone());
                            } else {
                                *self = Application::ShouldExit
                            }
                        } else if let Event::Keyboard(keyboard::Event::KeyPressed {
                            key_code: keyboard::KeyCode::S,
                            modifiers: keyboard::Modifiers::CTRL,
                        }) = event
                        {
                            return try_save(state);
                        }
                    }
                    Message::Save => return try_save(state),

                    Message::Saved(_) => {
                        *self = Application::Ready(State {
                            tainted: false,
                            ..state.clone()
                        })
                    }

                    _ => {}
                }
            }

            _ => {}
        }

        Command::none()
    }

    fn view(&mut self) -> Element<Message> {
        match self {
            Application::Loading => Text::new("Loading").into(),
            Application::Error(msg) => Text::new(msg.clone()).into(),
            Application::Ready(state) => render_when_ready(state).into(),
            Application::ShouldExit => Text::new("exiting").into(),
            Application::UnsavedCloseRequested(state) => {
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

                let contents = Scrollable::new(&mut state.scroll).push(c);
                return Column::new()
                    .push(
                        Button::new(
                            &mut state.save_button,
                            Row::new()
                                .push(Text::new("You just tried to exit when unsaved"))
                                .push(Text::new("+")),
                        )
                        .on_press(Message::Save),
                    )
                    .push(contents)
                    .into(); //TODO change this to a modal
            }
        }
    }
}

fn render_when_ready(state: &mut State) -> Scrollable<Message> {
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

    return Scrollable::new(&mut state.scroll).push(c);
}

fn try_save(state: &mut State) -> Command<Message> {
    if state.tainted {
        return Command::perform(save_sequences(state.sequences.clone()), Message::Saved);
    }

    return Command::none();
}
