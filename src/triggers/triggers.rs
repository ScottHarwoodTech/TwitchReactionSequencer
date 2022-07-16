use crate::sequencer::QueueEvent;
use async_trait::async_trait;
use core::fmt;
use std::{collections::HashMap, error::Error};
use tokio::sync::mpsc;

pub trait TriggerEvent: fmt::Debug + Send + Sync + dyn_clone::DynClone {}

dyn_clone::clone_trait_object!(TriggerEvent);

#[async_trait]
pub trait TriggerSource: fmt::Debug + Send + Sync + dyn_clone::DynClone {
    async fn watch(&self, trigger_sequence: mpsc::Sender<QueueEvent>)
        -> Result<(), Box<dyn Error>>;

    fn get_events(&self) -> &HashMap<String, Box<dyn TriggerEvent>>;
}

dyn_clone::clone_trait_object!(TriggerSource);
