use btleplug::api::{Central, Manager as _, Peripheral as _, ScanFilter};
use btleplug::platform::{Adapter, Manager, Peripheral};
use std::collections::HashMap;
use std::error::Error;
use std::time::Duration;
use tokio::time;

mod twitch;

pub mod sequencer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let manager = Manager::new().await.unwrap();

    let adapters = manager.adapters().await?;
    let central = adapters.into_iter().nth(0).unwrap();

    central.start_scan(ScanFilter::default()).await?;
    // instead of waiting, you can use central.events() to get a stream which will
    // notify you of new devices, for an example of that see examples/event_driven_discovery.rs
    time::sleep(Duration::from_secs(2)).await;

    let mb = find_mb(&central).await.unwrap();

    mb.connect().await?;
    mb.discover_services().await?;

    // println!(
    //     "Device Mac Address {}",
    //     mb.properties().await.unwrap().unwrap().address
    // );
    // for s in mb.services().iter() {
    //     println!("  Service uuid {}", s.uuid);
    //     for c in s.characteristics.iter() {
    //         println!("      Serivce characteristics {}", c.uuid);
    //     }
    // }

    let data = include_str!("./sequences/default.json");

    let sequencer: sequencer::reaction_sequence::ReactionSequence = serde_json::from_str(data)?;
    let device_set = setup_devices(mb);
    sequencer.play(&device_set).await;
    let controller = twitch::TwitchController::new("lanasidhe");

    controller.start().await;

    Ok(())
}

async fn find_mb(central: &Adapter) -> Option<Peripheral> {
    for p in central.peripherals().await.unwrap() {
        if p.properties()
            .await
            .unwrap()
            .unwrap()
            .local_name
            .iter()
            .any(|name| name.contains("BBC micro:bit"))
        {
            return Some(p);
        }
    }

    None
}

fn setup_devices(mb: Peripheral) -> HashMap<&'static str, sequencer::device::Device> {
    let device_set: HashMap<&'static str, sequencer::device::Device> = HashMap::new();

    let device_set = sequencer::devices::bunny_ears::setup(device_set, mb);
    let device_set = sequencer::devices::timer::setup(device_set);

    return device_set;
}
