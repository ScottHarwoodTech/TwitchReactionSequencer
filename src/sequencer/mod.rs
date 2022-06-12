pub mod device;
pub mod devices;
pub mod reaction_sequence;

use std::collections::HashMap;

impl reaction_sequence::ReactionSequence {
    pub async fn play(&self, device_set: &HashMap<&str, device::Device>) {
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
    device_set: &'a HashMap<&str, device::Device>,
    id: &str,
) -> Option<&'a device::Device> {
    return device_set.get(id);
}
