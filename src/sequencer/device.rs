use async_trait::async_trait;
use core::fmt;
use serde_json;
use std::collections::HashMap;

use crate::sequencer::devices::DeviceTypes;

#[async_trait]
pub trait DeviceAction: fmt::Debug + dyn_clone::DynClone + Send + Sync {
    async fn action(&self, arguments: Vec<serde_json::Value>) -> ();
}

dyn_clone::clone_trait_object!(DeviceAction);

#[derive(Debug, Clone)]
pub enum ParameterName {
    Address,
}

#[derive(Debug, Clone)]
pub enum Parameter {
    String(ParameterName), //Name, optional default
}

pub trait DeviceTrait: fmt::Debug + dyn_clone::DynClone + Send + Sync {
    fn get_actions(&self) -> &HashMap<String, Box<dyn DeviceAction>>;
    fn get_name(&self) -> &String;
    fn get_device_type(&self) -> &DeviceTypes;
    fn get_device_parameters() -> Vec<Parameter>
    where
        Self: Sized;
}

dyn_clone::clone_trait_object!(DeviceTrait);

pub type DevicesCollection = HashMap<String, Box<dyn DeviceTrait>>;

pub type DeviceImpler = Box<dyn DeviceTrait>;
