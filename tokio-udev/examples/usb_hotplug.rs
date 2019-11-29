use futures_util::future::ready;
use futures_util::stream::StreamExt;
use tokio_udev::{Context, MonitorBuilder};

#[tokio::main]
async fn main() {
    let context = Context::new().unwrap();
    let mut builder = MonitorBuilder::new(&context).unwrap();
    builder.match_subsystem_devtype("usb", "usb_device").unwrap();

    let monitor = builder.listen().unwrap();
    monitor.for_each(|event| {
        println!("Hotplug event: {}: {}", event.event_type(), event.device().syspath().display());
        ready(())
    }).await
}
