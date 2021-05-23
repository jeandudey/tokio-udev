// Copyright 2020 Jean Pierre Dudey. See the LICENSE-MIT and
// LICENSE-APACHE files at the top-level directory of this
// distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::convert::TryInto;
use futures_util::future::ready;
use futures_util::stream::StreamExt;
use tokio_udev::{MonitorBuilder, AsyncMonitorSocket};

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
