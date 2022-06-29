#![allow(unused_imports)]
#![allow(unused_must_use)]
#![allow(unused_variables)]

use core::num;
use std::collections::HashSet;
use std::thread;
use std::time::Duration;

use hidapi::HidApi;

use flexi_logger::Logger;
use log::{error, info, warn};

fn main() {
    let logger = Logger::try_with_str("info")
        .unwrap()
        .format(flexi_logger::detailed_format)
        .start()
        .unwrap();

    info!("Initializing USB Monitor.");

    let mut api = match HidApi::new() {
        Ok(api) => api,
        Err(err) => {
            error!("{}", err);
            return;
        }
    };

    let mut iteration = 0;

    loop {
        iteration = iteration + 1;
        api.refresh_devices();

        let mut devices: HashSet<&str> = HashSet::new();
        let mut device_list = api.device_list();
        for device_info in device_list.by_ref().into_iter() {
            devices.insert(device_info.product_string().unwrap_or("Unknown"));
        }

        info!(
            "Iteration {}: Found {} devices.",
            iteration,
            device_list.by_ref().count(),
        );

        for device in &devices {
            if *device != "" {
                info!("Found device {:?}", device);
            } else {
                warn!("Device Unknown");
            }
        }

        thread::sleep(Duration::from_millis(1000));
    }
}
