use btleplug::api::{Central, Manager as _, Peripheral as _, ScanFilter};
use btleplug::platform::{Adapter, Manager};
use std::error::Error;
use std::time::Duration;
use tokio::time;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let manager = Manager::new().await.unwrap();

    // get the first bluetooth adapter
    let adapters = manager.adapters().await?;
    let central = adapters.into_iter().nth(0).unwrap();

    // start scanning for devices
    central.start_scan(ScanFilter::default()).await?;
    // instead of waiting, you can use central.events() to get a stream which will
    // notify you of new devices, for an example of that see examples/event_driven_discovery.rs
    time::sleep(Duration::from_secs(2)).await;

    scan_for_devices(&central).await;

    Ok(())
}

async fn scan_for_devices(central: &Adapter) {
    println!("Scanning for 10 seconds...");
    central.start_scan(ScanFilter::default()).await.unwrap();
    println!("Scanned");
    loop {
        for p in central.peripherals().await.unwrap() {
            let properties = p.properties().await.unwrap().unwrap();
            if properties.services.len() > 0 {
                dbg!(&properties.services);
                dbg!(String::from_utf8(
                    properties
                        .service_data
                        .get(properties.service_data.keys().next().unwrap())
                        .unwrap()
                        .clone()
                )
                .unwrap());
            }
        }
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}
