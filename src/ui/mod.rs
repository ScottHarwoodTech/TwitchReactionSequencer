mod application;
pub mod fs_utils;
mod panes;
mod sequence;

use std::collections::HashMap;

use iced::{Application, Settings as IcedSettings};

use crate::sequencer::device::{DeviceTrait, DevicesCollection};
use crate::settings::Settings;

use crate::triggers::triggers::TriggerSource;
use crate::triggers::TriggerCollection;

pub fn ui(devices: DevicesCollection, triggers: TriggerCollection, settings: Settings) {
    application::Application::run(IcedSettings {
        exit_on_close_request: false,
        ..IcedSettings::with_flags((devices, triggers, settings))
    })
    .unwrap();
}
