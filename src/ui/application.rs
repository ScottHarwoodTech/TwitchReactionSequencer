use std::collections::HashMap;

use iced::{Command, Text};
use iced_native::command;

use crate::{sequencer::device::DeviceTrait, triggers::triggers::TriggerSource};

use super::panes::sequences::{sequence, Sequences, SequencesMessage};
use super::sequence::{Sequence, SequenceMessage};

#[derive(Debug)]
pub enum Application {
    Loading,
    Sequences(State),
}

#[derive(Debug, Clone)]
pub struct State {
    sequences: Sequences,
}

#[derive(Debug, Clone)]
pub enum Message {
    SequencesMessage(SequencesMessage),
    Loaded(State),
    EventOccurred(iced_native::Event),
}

fn init(
    devices: HashMap<String, Box<dyn DeviceTrait>>,
    triggers: HashMap<String, Box<dyn TriggerSource>>,
) -> (Sequences, Command<SequencesMessage>) {
    return Sequences::new((devices, triggers));
}

impl iced::Application for Application {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = (
        HashMap<String, Box<dyn DeviceTrait>>,
        HashMap<String, Box<dyn TriggerSource>>,
    );

    type Theme = iced::Theme;

    fn title(&self) -> String {
        match self {
            Application::Sequences(state) => state.sequences.title(),
            _ => String::from("Twitch Reaction Sequencer"),
        }
    }

    fn theme(&self) -> Self::Theme {
        iced::Theme::Dark
    }

    fn new(
        flags: (
            HashMap<String, Box<dyn DeviceTrait>>,
            HashMap<String, Box<dyn TriggerSource>>,
        ),
    ) -> (Application, Command<Message>) {
        let i = init(flags.0, flags.1);

        return (
            Application::Sequences(State { sequences: i.0 }),
            i.1.map(Message::SequencesMessage),
        );
    }

    fn should_exit(&self) -> bool {
        match self {
            Application::Sequences(state) => state.sequences.should_exit(),
            _ => false,
        }
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        iced_native::subscription::events().map(Message::EventOccurred)
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match self {
            Application::Loading => match message {
                Message::Loaded(state) => {
                    *self = Application::Sequences(state);
                    return Command::none();
                }
                _ => Command::none(),
            },
            Application::Sequences(state) => match message {
                Message::EventOccurred(e) => {
                    state.sequences.update(SequencesMessage::EventOccurred(e))
                }
                Message::SequencesMessage(sequences_message) => {
                    state.sequences.update(sequences_message)
                }
                _ => Command::none(),
            }
            .map(Message::SequencesMessage),
            _ => Command::none(),
        }
    }

    fn view(&mut self) -> iced::Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        return match self {
            Application::Sequences(state) => state.sequences.view().map(Message::SequencesMessage),
            _ => Text::new("label").into(),
        };
    }
}
