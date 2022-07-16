use std::collections::HashMap;
use std::time::Duration;

use crate::sequencer::device::DeviceTrait;
use crate::triggers::triggers::TriggerSource;
use crate::ui::sequence;
use iced::{self, scrollable, Column, Text};
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
}

#[derive(Debug, Clone)]
pub enum Message {
    Loaded(Result<State, LoadError>),
    SequenceMessage(usize, SequenceMessage),
}

#[derive(Debug, Clone)]
pub enum LoadError {
    FileError,
    FormatError,
}

async fn dummy(
    devices: HashMap<String, Box<dyn DeviceTrait>>,
    triggers: HashMap<String, Box<dyn TriggerSource>>,
) -> Result<State, LoadError> {
    return Ok(State {
        sequences: vec![
            Sequence::new(devices.clone(), triggers.clone()),
            Sequence::new(devices.clone(), triggers.clone()),
        ],
        scroll: scrollable::State::new(),
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
            Command::perform(dummy(flags.0, flags.1), Message::Loaded),
        )
    }

    fn title(&self) -> String {
        return String::from("Twitch Reaction Sequencer");
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match self {
            Application::Loading => match message {
                Message::Loaded(Ok(state)) => {
                    *self = Application::Ready(State {
                        sequences: state.sequences,
                        scroll: state.scroll,
                    });
                }

                Message::Loaded(Err(_)) => {}
                _ => {}
            },

            Application::Ready(state) => match message {
                Message::SequenceMessage(i, sequence_message) => {
                    if let Some(sequence) = state.sequences.get_mut(i) {
                        sequence.update(sequence_message);
                    }
                }
                _ => {}
            },
        }

        Command::none()
    }

    fn view(&mut self) -> Element<Message> {
        match self {
            Application::Loading => Text::new("Loading").into(),
            Application::Ready(state) => {
                let c = Column::new().max_width(800).spacing(20);

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

                return c.push(seqs).into();
            }
        }
    }
}
