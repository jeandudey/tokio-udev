extern crate tokio_udev;
extern crate tokio;
extern crate futures;

use futures::{Future, stream::Stream};

use tokio_udev::{USB_SUBSYSTEM, USB_DEVICE, UDEV_MONITOR};

fn main() {
    let monitor = tokio_udev::Builder::new()
        .add_match(USB_SUBSYSTEM, USB_DEVICE)
        .build(UDEV_MONITOR)
        .expect("couldn't create monitor");

    let hotplug_stream = monitor.for_each(|device| {
        println!("=====================");
        println!("  Usb HotPlug Event  ");
        println!("=====================");
        println!("devpath: \"{:?}\"", device.get_devpath()?);
        println!("action: \"{}\"", device.get_action()?);

        Ok(())
    })
    .map_err(|e| {
        println!("error: {}", e);
    });

    tokio::run(hotplug_stream);
}
