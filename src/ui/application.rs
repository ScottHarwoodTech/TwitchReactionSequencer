use std::collections::HashMap;

use crate::sequencer::device::DeviceTrait;
use crate::ui::sequence;
use iced::{self, Column, Renderer, Row, Text};
use iced::{Command, Element};

use super::sequence::{Sequence, SequenceMessage};

#[derive(Debug)]
pub enum Application {
    Loading,
    Ready(State),
}

#[derive(Debug, Clone)]
struct State {
    sequences: Vec<sequence::Sequence>,
}

#[derive(Debug, Clone)]
pub enum Message {
    Loaded(Result<State, LoadError>),
    SequenceMessage(usize, SequenceMessage),
}

#[derive(Debug, Clone)]
enum LoadError {
    FileError,
    FormatError,
}

async fn dummy(devices: HashMap<String, Box<dyn DeviceTrait>>) -> Result<State, LoadError> {
    return Ok(State {
        sequences: vec![
            Sequence::new(devices.clone()),
            Sequence::new(devices.clone()),
        ],
    });
}

impl iced::Application for Application {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = (HashMap<String, Box<dyn DeviceTrait>>,);

    fn new(flags: (HashMap<String, Box<dyn DeviceTrait>>,)) -> (Application, Command<Message>) {
        (
            Application::Loading,
            Command::perform(dummy(flags.0), Message::Loaded),
        )
    }

    fn title(&self) -> String {
        return String::from("Twich Reaction Sequencer");
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match self {
            Application::Loading => match message {
                Message::Loaded(Ok(state)) => {
                    *self = Application::Ready(State {
                        sequences: state.sequences,
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
            Application::Loading => Text::new("title").into(),
            Application::Ready(state) => {
                let c = Column::new().max_width(800).spacing(20);

                let seqs: Element<_> = state
                    .sequences
                    .iter_mut()
                    .enumerate()
                    .fold(
                        Column::new().spacing(20).padding(10),
                        |column: Column<_>, (i, sequence)| {
                            column.push(
                                sequence
                                    .view()
                                    .map(move |message| Message::SequenceMessage(i, message)),
                            )
                        },
                    )
                    .into();

                return c.push(seqs).into();
            }
        }
    }
}
