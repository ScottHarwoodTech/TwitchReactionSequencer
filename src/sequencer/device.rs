use async_trait::async_trait;
use serde_json;
use std::collections::HashMap;

pub struct Device {
    id: String,
    name: String,
    actions: HashMap<String, Box<dyn DeviceAction>>,
}

impl Device {
    pub fn new(id: &str, name: &str, actions: HashMap<String, Box<dyn DeviceAction>>) -> Device {
        return Device {
            id: String::from(id),
            name: String::from(name),
            actions: actions,
        };
    }
}

#[async_trait]
pub trait DeviceAction {
    async fn action(&self, arguments: Vec<serde_json::Value>) -> ();
}

impl DeviceTrait for Device {
    fn get_actions(&self) -> &HashMap<String, Box<dyn DeviceAction>> {
        return &self.actions;
    }
}

pub trait DeviceTrait {
    fn get_actions(&self) -> &HashMap<String, Box<dyn DeviceAction>>;
}
