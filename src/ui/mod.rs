mod application;
pub mod util;
mod sequence;


use iced::{Application, Settings};

pub fn ui() {
    application::Application::run(Settings::default()).unwrap();
}
