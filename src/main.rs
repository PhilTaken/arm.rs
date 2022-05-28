mod media;
mod config;
mod devices;

use config::Config;

fn main() -> Result<(), anyhow::Error> {

    #[allow(unused_variables)]
    let config = Config::parse("test");

    devices::poll(|event| {
        println!("{:?}", event.event_type());
    })
}
