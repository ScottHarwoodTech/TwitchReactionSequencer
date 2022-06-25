use std::collections::HashMap;

use crate::sequencer::device::DeviceTrait;
use crate::sequencer::devices;
use crate::ui::sequence;
use iced;
use iced::{Column, Command, Element, Text};

use super::sequence::trigger::Trigger;
use super::sequence::Sequence;

#[derive(Debug)]
pub enum Application {
    Ready,
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

async fn dummy(
    devices: &'static HashMap<String, Box<dyn DeviceTrait>>,
) -> Result<State, LoadError> {
    return Ok(State {
        sequences: vec![Sequence::new(devices)],
    });
}

impl iced::Application for Application {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = (&'static HashMap<String, Box<dyn DeviceTrait>>,);

    fn new(
        flags: (&'static HashMap<String, Box<dyn DeviceTrait>>,),
    ) -> (Application, Command<Message>) {
        (
            Application::Ready,
            Command::perform(dummy(flags.0), Message::Loaded),
        )
    }

    fn title(&self) -> String {
        return String::from("hello world");
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        Command::none()
    }

    fn view(&mut self) -> Element<Message> {
        match self {
            Application::Ready => self.se,
        }
    }
}
