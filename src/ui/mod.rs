mod application;
pub mod fs_utils;
mod panes;
mod sequence;



use iced::{Application, Settings as IcedSettings};

use crate::sequencer::device::{DevicesCollection};
use crate::settings::Settings;


use crate::triggers::TriggerCollection;

pub fn ui(devices: DevicesCollection, triggers: TriggerCollection, settings: Settings) {
    application::Application::run(IcedSettings {
        exit_on_close_request: false,
        ..IcedSettings::with_flags((devices, triggers, settings))
    })
    .unwrap();
}
