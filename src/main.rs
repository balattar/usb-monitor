#![allow(unused_imports)]
#![allow(unused_must_use)]
#![allow(unused_variables)]

use std::collections::HashSet;
use std::thread;
use std::time::Duration;

use hidapi::HidApi;

use log::{error, info, warn};
use flexi_logger::Logger;


fn main() {
    let logger = Logger::try_with_str("info")
        .unwrap()
        .format(flexi_logger::detailed_format)
        .start()
        .unwrap();

    info!("Initializing USB Monitor.");

    let api = match HidApi::new() {
        Ok(api) => api,
        Err(err) => {
            error!("{}", err);
            return;
        }
    };

    let mut devices: HashSet<&str> = HashSet::new();
    for device_info in api.device_list() {
        devices.insert(device_info.product_string().unwrap_or("Unknown"));
    }
    let mut iteration = 0;

    loop {
        iteration = iteration + 1;
        info!("Running Iteration {}.", iteration);
        let mut devices: HashSet<&str> = HashSet::new();
        for device_info in api.device_list() {
            devices.insert(device_info.product_string().unwrap_or("Unknown"));
        }

        for device in &devices {
            if *device != "" {
                info!("Found device {:?}", device);
            } else {
                warn!("Device Unknown");
            }
        }

        thread::sleep(Duration::from_secs(2));
    }
}
