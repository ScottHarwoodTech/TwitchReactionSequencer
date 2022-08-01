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
}

fn get_device_by_id<'a>(
    device_set: &'a HashMap<String, Box<dyn device::DeviceTrait>>,
    id: &str,
) -> Option<&'a Box<dyn device::DeviceTrait>> {
    return device_set.get(id);
}

use tokio::sync::watch;

#[derive(Debug)]
pub struct QueueEvent {
    pub sequence_id: String,
}

use device::DeviceTrait;
use std::error::Error;

pub async fn watch_queue(
    device_set: HashMap<String, Box<dyn DeviceTrait>>,
    mut queue_reciever: watch::Receiver<QueueEvent>,
) -> Result<(), Box<dyn Error>> {
    print!("Started queue reciever");
    while queue_reciever.changed().await.is_ok() {
        println!("recieved = {:?}", *queue_reciever.borrow());
        // let data = include_str!("../../sequences/default.json");
        //let sequencer: reaction_sequence::ReactionSequence = serde_json::from_str(data)?;

        // sequencer.play(&device_set).await;
    }

    return Ok(());
}
