pub mod device;
pub mod devices;
pub mod reaction_sequence;
use std::collections::HashMap;

impl reaction_sequence::ReactionSequence {
    pub async fn play(&self, device_set: &HashMap<String, Box<dyn device::DeviceTrait>>) {
        let sequence = &self.sequence;
        for method in sequence {
            let device = get_device_by_id(&device_set, &method.device_id);
            let method_arguments = method.arguments.clone();

            println!("{}", &method.device_action_id);
            device
                .unwrap()
                .get_actions()
                .get(&method.device_action_id)
                .unwrap()
                .action(method_arguments)
                .await;
        }
    }

    pub fn is_triggered_by_event(&self, event: QueueEvent) -> bool {
        return self.trigger.trigger_id == event.trigger_source.as_str()
            && self.trigger.trigger_event_id == event.trigger_event_id.as_str();
    }
}

fn get_device_by_id<'a>(
    device_set: &'a HashMap<String, Box<dyn device::DeviceTrait>>,
    id: &str,
) -> Option<&'a Box<dyn device::DeviceTrait>> {
    return device_set.get(id);
}

use tokio::sync::watch;

#[derive(Debug, Clone)]
pub struct QueueEvent {
    pub trigger_source: TriggerSource,
    pub trigger_event_id: String,
}

pub trait QueueEvent1 {
    fn get_trigger_source(&self) -> TriggerSource;
}

use device::DeviceTrait;
use std::error::Error;

use crate::triggers::TriggerSource;

use self::reaction_sequence::ReactionSequence;

pub async fn watch_queue(
    device_set: HashMap<String, Box<dyn DeviceTrait>>,
    sequences: Vec<ReactionSequence>,
    mut queue_reciever: watch::Receiver<QueueEvent>,
    task_handler_reciever: watch::Receiver<()>,
) -> Result<(), Box<dyn Error>> {
    println!("Started queue reciever");

    while queue_reciever.changed().await.is_ok() && !task_handler_reciever.has_changed().unwrap() {
        let event = (*queue_reciever.borrow()).clone();
        println!("recieved = {:?}", *queue_reciever.borrow());
        for sequence in sequences.iter() {
            if sequence.is_triggered_by_event(event.clone()) {
                println!("Played Sequence = {:?}", sequence.clone());
                sequence.play(&device_set).await;
            }
        }
    }

    return Ok(());
}
