

use iced::{Command, Element};
use iced_native::widget::Column;
mod configured_device;
use crate::{
    sequencer::device::{DevicesCollection},
    settings::Settings,
};

use self::configured_device::format_configured_devices;

#[derive(Debug, Clone)]
pub struct SettingsPane {
    devices: Option<DevicesCollection>,
}

pub trait Component<Message>: Sized {
    fn update(&mut self, message: Message) -> Command<Message>;
    fn view(&self) -> iced::Element<'_, Message>;
    fn new(settings: Settings) -> (Self, Command<Message>);
}

#[derive(Debug, Clone)]
pub enum SettingsMessage {
    Loaded(DevicesCollection),
}

impl Component<SettingsMessage> for SettingsPane {
    fn new(settings: Settings) -> (SettingsPane, Command<SettingsMessage>) {
        (
            SettingsPane {
                devices: Option::None,
            },
            Command::perform(format_configured_devices(settings), SettingsMessage::Loaded),
        )
    }

    fn update(&mut self, message: SettingsMessage) -> Command<SettingsMessage> {
        match message {
            SettingsMessage::Loaded(devices) => {
                *self = SettingsPane {
                    devices: Option::Some(devices.clone()),
                };
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<'_, SettingsMessage> {
        return Column::new().into();
    }
}
