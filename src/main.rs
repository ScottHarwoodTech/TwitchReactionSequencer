use std::error::Error;

mod custom_widgets;
mod sequencer;
mod settings;
mod triggers;
mod ui;

use crate::settings::Settings;
use dotenv::dotenv;
use jsonschema::JSONSchema;
use serde_json::json;
use tokio::fs;

#[derive(Debug, Clone)]
pub enum ThreadActions {
    Stop,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let settings_content = fs::read(&"./TRS/settings.json").await?;
    let s = json!(include_str!("../schemas/settings.json"));
    // let settings_schema = JSONSchema::compile(&s).unwrap();

    let settings = serde_json::from_slice::<Settings>(&settings_content)?;

    // match settings_schema.validate(&json!(settings_content)) {
    //     Err(_x) => panic!("Settings were not valid"),
    //     Ok(_x) => {} //TODO: fix this
    // }

    println!("{:?}", settings.configured_devices);
    // TODO: These should be read from a settings file on disk
    let device_set = sequencer::devices::setup_devices().await?;
    let triggers = triggers::get_available_trigger_sources().await?;

    ui::ui(device_set.clone(), triggers.clone(), settings.clone());
    Ok(())
}
