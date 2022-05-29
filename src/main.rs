mod media;
mod config;
mod devices;

use core::time;
use std::thread;

use media::media_from_dev;
use config::Config;

fn main() {
    let config = Config::minimal();

    #[allow(unused_variables)]
    if let Ok(conf) = config {
        println!("parsed config, starting...");

        devices::poll(|event| {
            println!("{:?}", event.event_type());
            if let Some(media) = media_from_dev(&event.device()) {

                //let _ = media.process(&conf);

                println!("------------------------------------------------");

                thread::sleep(time::Duration::from_secs(10));

                media.eject();
            };
        });
    } else {
        eprintln!("Error in config: {}", config.err().unwrap());
    }
}
