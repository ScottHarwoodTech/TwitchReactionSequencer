mod application;
pub mod util;

use iced::{Application, Settings};

pub fn ui() {
    application::Application::run(Settings::default()).unwrap();
}
