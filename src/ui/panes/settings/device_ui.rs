use iced::{Column, Element, Row, Text};

use crate::sequencer::device::DeviceImpler;

#[derive(Debug, Clone)]
pub struct DeviceUI {
    name: String,
    id: String,
}

#[derive(Debug, Clone)]
pub enum DeviceMessage {}

impl DeviceUI {
    pub fn new(id: String, device: DeviceImpler) -> Self {
        return DeviceUI {
            name: device.get_name().clone(),
            id: id,
        };
    }

    pub fn view(&self) -> Element<'_, DeviceMessage> {
        return Row::new()
            .spacing(5)
            .push(Text::new(self.id.clone()))
            .push(Text::new(self.name.clone()))
            .into();
    }
}
