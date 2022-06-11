use btleplug::api::{Central, Manager as _, Peripheral as _, ScanFilter, WriteType};
use btleplug::platform::{Adapter, Manager, Peripheral};
use std::error::Error;
use std::time::Duration;
use tokio::time;
use uuid::Uuid;

const TX_CHARACTERISTIC: &str = "6E400002-B5A3-F393-E0A9-E50E24DCCA9E";
const RX_CHARACTERISTIC: &str = "6E400003-B5A3-F393-E0A9-E50E24DCCA9E";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let manager = Manager::new().await.unwrap();

    let adapters = manager.adapters().await?;
    let central = adapters.into_iter().nth(0).unwrap();

    central.start_scan(ScanFilter::default()).await?;
    // instead of waiting, you can use central.events() to get a stream which will
    // notify you of new devices, for an example of that see examples/event_driven_discovery.rs
    time::sleep(Duration::from_secs(2)).await;

    //    central
    //        .add_peripheral(BDAddr::from_str_delim("EB:A0:B4:C2:82:C8").unwrap())
    //        .await?;

    let mb = find_mb(&central).await.unwrap();

    mb.connect().await?;
    mb.discover_services().await?;

    println!(
        "Device Mac Address {}",
        mb.properties().await.unwrap().unwrap().address
    );
    for s in mb.services().iter() {
        println!("  Service uuid {}", s.uuid);
        for c in s.characteristics.iter() {
            println!("      Serivce characteristics {}", c.uuid);
        }
    }

    let chars = mb.characteristics();
    let rx_char = chars
        .iter()
        .find(|c| c.uuid == Uuid::parse_str(RX_CHARACTERISTIC).unwrap())
        .unwrap();
    let cmd = vec![0x48, 0x31, 0x38, 0x30, 0xA];
    mb.write(&rx_char, &cmd, WriteType::WithoutResponse).await?;
    time::sleep(Duration::from_secs(2)).await;

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
