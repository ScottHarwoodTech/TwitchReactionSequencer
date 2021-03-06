use std::collections::HashMap;

use crate::sequencer::device::DeviceTrait;
use crate::ui::sequence;
use iced::{self, Column, Renderer, Row, Text};
use iced::{Command, Element};

use super::sequence::Sequence;

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
        return String::from("hello world");
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
            },

            Application::Ready(_state) => {}
        }

        Command::none()
    }

    fn view(&mut self) -> Element<Message> {
        match self {
            Application::Loading => Text::new("title").into(),
            Application::Ready(state) => {
                let sequences: Element<_> = state
                    .sequences
                    .iter()
                    .fold(Row::new().spacing(20), |row: Row<_>, sequence| {
                        row.push(Text::new(sequence.trigger.selected_device.clone().unwrap()))
                    })
                    .into();

                Column::new()
                    .max_width(800)
                    .spacing(20)
                    .push(sequences)
                    .into()
            }
        }
    }
}
