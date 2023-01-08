use super::panes::{
    sequences::{Sequences, SequencesMessage},
    settings::{Component, SettingsMessage, SettingsPane},
};
use crate::{
    sequencer::device::DevicesCollection, settings::Settings, triggers::TriggerCollection,
};
use iced::{button, Button, Column, Command, Row, Text};
use iced_native::{window, Event};

#[derive(Debug)]
pub enum Application {
    Loading,
    Sequences(State),
    Settings(State),
    ShouldExit,
}

#[derive(Debug, Clone)]
pub struct Buttons {
    sequences: button::State,
    settings: button::State,
}

#[derive(Debug, Clone)]
pub struct State {
    sequences: Sequences,
    settings: SettingsPane,
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
    devices: DevicesCollection,
    triggers: TriggerCollection,
) -> (Sequences, Command<SequencesMessage>) {
    Sequences::new((devices, triggers))
}

impl iced::Application for Application {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = (DevicesCollection, TriggerCollection, Settings);

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
        flags: (DevicesCollection, TriggerCollection, Settings),
    ) -> (Application, Command<Message>) {
        let sequences = init(flags.0, flags.1);
        let settings = SettingsPane::new(flags.2);
        (
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
        )
    }

    fn should_exit(&self) -> bool {
        match self {
            Application::Sequences(state) => state.sequences.should_exit(),
            Application::ShouldExit => true,
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
            Application::Loading => Command::none(),

            Application::Sequences(state) => match message {
                Message::EventOccurred(e) => state
                    .sequences
                    .update(SequencesMessage::EventOccurred(e))
                    .map(Message::SequencesMessage),
                Message::SequencesMessage(sequences_message) => {
                    { state.sequences.update(sequences_message) }.map(Message::SequencesMessage)
                }
                Message::SettingsMessage(settings_message) => {
                    { state.settings.update(settings_message) }.map(Message::SettingsMessage)
                }
                Message::ChangePane(_) => Command::none(),
            },

            Application::Settings(state) => match message {
                Message::SettingsMessage(settings_message) => {
                    state.settings.update(settings_message)
                }
                Message::EventOccurred(event) => {
                    if Event::Window(window::Event::CloseRequested) == event {
                        *self = Application::ShouldExit
                    }
                    return Command::none();
                }
                _ => Command::none(),
            }
            .map(Message::SettingsMessage),
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
            Application::ShouldExit => {}
        }

        page.into()
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
