use super::panes::{
    sequences::{Sequences, SequencesMessage},
    settings::{Component, Settings, SettingsMessage},
};
use crate::{sequencer::device::DeviceTrait, triggers::triggers::TriggerSource};
use iced::{button, Button, Column, Command, Row, Text};
use std::collections::HashMap;

#[derive(Debug)]
pub enum Application {
    Loading,
    Sequences(State),
    Settings(State),
}

#[derive(Debug, Clone)]
pub struct Buttons {
    sequences: button::State,
    settings: button::State,
}

#[derive(Debug, Clone)]
pub struct State {
    sequences: Sequences,
    settings: Settings,
    buttons: Buttons,
}

#[derive(Debug, Clone)]
pub enum Pane {
    Sequences,
}

#[derive(Debug, Clone, Copy)]
pub enum ChangePane {
    MoveToSequences,
    MoveToSettings,
}

#[derive(Debug, Clone)]
pub enum Message {
    SequencesMessage(SequencesMessage),
    SettingsMessage(SettingsMessage),
    EventOccurred(iced_native::Event),
    ChangePane(ChangePane),
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
        let sequences = init(flags.0, flags.1);
        let settings = Settings::new();
        return (
            Application::Sequences(State {
                sequences: sequences.0,
                settings: settings.0,
                buttons: Buttons {
                    sequences: button::State::new(),
                    settings: button::State::new(),
                },
            }),
            Command::batch(vec![
                sequences.1.map(Message::SequencesMessage),
                settings.1.map(Message::SettingsMessage),
            ]),
        );
    }

    fn should_exit(&self) -> bool {
        match self {
            Application::Sequences(state) => state.sequences.should_exit(),
            _ => false,
        }
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        // TODO: UI Event stream
        iced_native::subscription::events().map(Message::EventOccurred)
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::ChangePane(change_pane) => match self {
                Application::Sequences(state) | Application::Settings(state) => match change_pane {
                    ChangePane::MoveToSettings => {
                        *self = Application::Settings(state.clone());
                        return Command::none();
                    }
                    ChangePane::MoveToSequences => {
                        *self = Application::Sequences(state.clone());
                        return Command::none();
                    }
                },
                _ => {}
            },
            _ => {}
        }
        match self {
            Application::Loading => match message {
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
        let mut page: Column<_> = Column::new();

        match self {
            Application::Loading => {}
            Application::Sequences(state) => {
                page = page.push(header(&mut state.buttons));
                page = page.push(state.sequences.view().map(Message::SequencesMessage))
            }
            Application::Settings(state) => {
                page = page.push(header(&mut state.buttons));
                page = page.push(state.settings.view().map(Message::SettingsMessage));
            }
        }

        return page.into();
    }
}

fn header(buttons: &mut Buttons) -> Row<Message> {
    return Row::new()
        .push(
            Button::new(&mut buttons.sequences, Text::new("Sequences"))
                .on_press(Message::ChangePane(ChangePane::MoveToSequences)),
        )
        .push(
            Button::new(&mut buttons.settings, Text::new("Settings"))
                .on_press(Message::ChangePane(ChangePane::MoveToSettings)),
        );
}
