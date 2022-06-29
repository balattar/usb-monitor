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
        let mut num_devices = 0;
        for device_info in api.device_list() {
            devices.insert(device_info.product_string().unwrap_or("Unknown"));
            num_devices += 1;
        }

        info!("Iteration {}: Found {} devices.", iteration, num_devices);

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
