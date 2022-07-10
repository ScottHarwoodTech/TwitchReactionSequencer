mod application;
mod sequence;
pub mod util;

use std::collections::HashMap;

use iced::{Application, Settings};

use crate::sequencer::device::DeviceTrait;

pub fn ui(devices: HashMap<String, Box<dyn DeviceTrait>>) {
    application::Application::run(Settings::with_flags((devices,))).unwrap();
}
