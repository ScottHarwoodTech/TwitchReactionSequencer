use iced::{Command, Element};
use iced_native::widget::Column;

#[derive(Debug, Clone)]
pub struct Settings {}

pub trait Component<Message>: Sized {
    fn update(&mut self, message: Message) -> Command<Message>;
    fn view(&self) -> iced::Element<'_, Message>;
    fn new() -> (Self, Command<Message>);
}

#[derive(Debug, Clone)]
pub enum SettingsMessage {}

impl Component<SettingsMessage> for Settings {
    fn new() -> (Settings, Command<SettingsMessage>) {
        (Settings {}, Command::none())
    }

    fn update(&mut self, _message: SettingsMessage) -> Command<SettingsMessage> {
        return Command::none();
    }

    fn view(&self) -> Element<'_, SettingsMessage> {
        return Column::new().into();
    }
}
