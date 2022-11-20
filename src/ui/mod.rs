mod application;
pub mod fs_utils;
mod panes;
mod sequence;
pub mod util;

use std::collections::HashMap;

use iced::{Application, Settings};

use crate::sequencer::device::DeviceTrait;
use crate::triggers::triggers::TriggerSource;

pub fn ui(
    devices: HashMap<String, Box<dyn DeviceTrait>>,
    triggers: HashMap<String, Box<dyn TriggerSource>>,
) {
    application::Application::run(Settings {
        exit_on_close_request: false,
        ..Settings::with_flags((devices, triggers))
    })
    .unwrap();
}
