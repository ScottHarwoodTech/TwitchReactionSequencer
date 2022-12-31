use async_trait::async_trait;
use core::fmt;
use serde_json;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Device {
    id: String,
    name: String,
    actions: HashMap<String, Box<dyn DeviceAction>>,
}

impl Device {
    pub fn new(id: &str, name: &str, actions: HashMap<String, Box<dyn DeviceAction>>) -> Device {
        Device {
            id: String::from(id),
            name: String::from(name),
            actions,
        }
    }
}

#[async_trait]
pub trait DeviceAction: fmt::Debug + dyn_clone::DynClone + Send + Sync {
    async fn action(&self, arguments: Vec<serde_json::Value>) -> ();
}

dyn_clone::clone_trait_object!(DeviceAction);

pub trait DeviceTrait: fmt::Debug + dyn_clone::DynClone + Send + Sync {
    fn get_actions(&self) -> &HashMap<String, Box<dyn DeviceAction>>;
}

dyn_clone::clone_trait_object!(DeviceTrait);

impl DeviceTrait for Device {
    fn get_actions(&self) -> &HashMap<String, Box<dyn DeviceAction>> {
        &self.actions
    }
}

pub type DevicesCollection = HashMap<String, Box<dyn DeviceTrait>>;
