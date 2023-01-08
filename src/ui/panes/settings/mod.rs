use iced::{button, Button, Column, Command, Element, Text};
mod configured_device;
mod device_ui;

use crate::{sequencer::device::DevicesCollection, settings::Settings};

use device_ui::{DeviceMessage, DeviceUI};

use self::configured_device::format_configured_devices;

#[derive(Debug, Clone)]
pub struct SettingsPane {
    devices: Vec<DeviceUI>,
    add_device_button: button::State,
}

pub trait Component<Message>: Sized {
    fn update(&mut self, message: Message) -> Command<Message>;
    fn view(&mut self) -> iced::Element<'_, Message>;
    fn new(settings: Settings) -> (Self, Command<Message>);
}

#[derive(Debug, Clone)]
pub enum SettingsMessage {
    Loaded(DevicesCollection),
    OnDeviceMessage(usize, DeviceMessage),
}

impl Component<SettingsMessage> for SettingsPane {
    fn new(settings: Settings) -> (SettingsPane, Command<SettingsMessage>) {
        (
            SettingsPane {
                devices: vec![],
                add_device_button: button::State::new(),
            },
            Command::perform(format_configured_devices(settings), SettingsMessage::Loaded),
        )
    }

    fn update(&mut self, message: SettingsMessage) -> Command<SettingsMessage> {
        match message {
            SettingsMessage::Loaded(devices) => {
                *self = SettingsPane {
                    devices: devices
                        .iter()
                        .map(|(k, v)| DeviceUI::new(k.clone(), v.clone()))
                        .collect(),
                    ..self.clone()
                };
                Command::none()
            }

            SettingsMessage::OnDeviceMessage(_index, _devicemessage) => Command::none(),
        }
    }

    fn view(&mut self) -> Element<'_, SettingsMessage> {
        let devices: Element<SettingsMessage> = self
            .devices
            .iter()
            .enumerate()
            .fold(
                Column::new().push(Text::new("Configured Devices")),
                |acc: Column<_>, (i, device)| {
                    acc.push(
                        device
                            .view()
                            .map(move |x| SettingsMessage::OnDeviceMessage(i, x)),
                    )
                },
            )
            .push(Button::new(
                &mut self.add_device_button,
                Text::new("Configure New Device +").size(20),
            ))
            .into();

        return devices.into();
    }
}
