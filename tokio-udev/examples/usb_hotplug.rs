// SPDX-FileCopyrightText: © 2020 Jean-Pierre De Jesus DIAZ <me@jeandudey.tech>
// SPDX-License-Identifier: MIT OR Apache-2.0

use futures_util::future::ready;
use futures_util::stream::StreamExt;
use std::convert::TryInto;
use tokio_udev::{AsyncMonitorSocket, MonitorBuilder};

#[tokio::main]
async fn main() {
    let builder = MonitorBuilder::new()
        .expect("Couldn't create builder")
        .match_subsystem_devtype("usb", "usb_device")
        .expect("Failed to add filter for USB devices");

    let monitor: AsyncMonitorSocket = builder
        .listen()
        .expect("Couldn't create MonitorSocket")
        .try_into()
        .expect("Couldn't create AsyncMonitorSocket");
    monitor
        .for_each(|event| {
            if let Ok(event) = event {
                println!(
                    "Hotplug event: {}: {}",
                    event.event_type(),
                    event.device().syspath().display()
                );
            }
            ready(())
        })
        .await
}
