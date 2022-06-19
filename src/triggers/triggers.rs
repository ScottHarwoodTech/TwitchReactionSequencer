use crate::sequencer::QueueEvent;
use async_trait::async_trait;
use std::error::Error;
use tokio::sync::mpsc;

#[async_trait]
pub trait TriggerSource {
    async fn watch(&self, trigger_sequence: mpsc::Sender<QueueEvent>)
        -> Result<(), Box<dyn Error>>;
}
