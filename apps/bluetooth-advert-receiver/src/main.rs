use btleplug::api::{
    bleuuid::uuid_from_u16, Central, Manager as _, Peripheral as _, ScanFilter, WriteType,
};
use btleplug::platform::{Adapter, Manager, Peripheral};
use rand::{thread_rng, Rng};
use std::error::Error;
use std::thread;
use std::time::Duration;
use tokio::time;
use uuid::Uuid;

const LIGHT_CHARACTERISTIC_UUID: Uuid = uuid_from_u16(0xFFE9);

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

    // find the device we're interested in
    let light = find_light(&central).await;

    // // connect to the device
    // light.connect().await?;

    // // discover services and characteristics
    // light.discover_services().await?;

    // // find the characteristic we want
    // let chars = light.characteristics();
    // let cmd_char = chars
    //     .iter()
    //     .find(|c| c.uuid == LIGHT_CHARACTERISTIC_UUID)
    //     .unwrap();

    // // dance party
    // let mut rng = thread_rng();
    // for _ in 0..20 {
    //     let color_cmd = vec![0x56, rng.gen(), rng.gen(), rng.gen(), 0x00, 0xF0, 0xAA];
    //     light
    //         .write(&cmd_char, &color_cmd, WriteType::WithoutResponse)
    //         .await?;
    //     time::sleep(Duration::from_millis(200)).await;
    // }
    Ok(())
}

async fn find_light(central: &Adapter) {
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
                ));
            }
            // dbg!(&properties
            //     .service_data
            //     .values()
            //     .map(|v| String::from_utf8(v.clone()))
            //     .collect::<Vec<Result<String, _>>>()
            //     .into_iter()
            //     .filter_map(Result::ok)
            //     .collect::<Vec<String>>());
            // if properties
            //     .local_name
            //     .iter()
            //     .any(|name| name.contains("Galaxy"))
            // {
            //     dbg!(&properties);
            // }
        }
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}
