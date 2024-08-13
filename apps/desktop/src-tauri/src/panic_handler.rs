use std::panic::{self, take_hook};

use crate::SIDECARS;

pub fn close_sidecars() {
    if let Ok(mut sidecars) = SIDECARS.lock() {
        while let Some(sidecar) = sidecars.pop() {
            eprintln!("Killing sidecar: {:?}", sidecar.pid());

            if let Err(err) = &sidecar.kill() {
                eprintln!("Failed to lock SIDECARS {}", err);
            } else {
                eprintln!("Killed sidecar");
            }
        }
    } else {
        eprintln!("Failed to lock SIDECARS");
    }
}

pub fn initialize_graceful_panic_handler() {
    let default_panic_hook = take_hook();

    // Set a custom panic hook
    panic::set_hook(Box::new(move |panic_info| {
        close_sidecars();
        default_panic_hook(panic_info);
    }));

    ctrlc::set_handler(|| {
        close_sidecars();
        std::process::exit(0);
    })
    .expect("Failed to set termination signal handler");
}
