mod media;
mod config;
mod devices;

//use config::Config;

fn main() -> Result<(), anyhow::Error> {
    devices::poll(|event| {
        println!("{:?}", event.event_type());
    })
}
