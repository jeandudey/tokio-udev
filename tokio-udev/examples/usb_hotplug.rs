use futures::{Future, stream::Stream};
use tokio;
use tokio_udev::{Context, MonitorBuilder};

fn main() {
    let context = Context::new().unwrap();
    let mut builder = MonitorBuilder::new(&context).unwrap();
    builder.match_subsystem_devtype("usb", "usb_device").unwrap();
    let monitor = builder.listen().unwrap();

    let hotplug_stream = monitor.for_each(|_device| {
        println!("Hotplug event!");
        Ok(())
    })
    .map_err(|e| {
        println!("error: {}", e);
    });

    tokio::run(hotplug_stream);
}
