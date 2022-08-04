#![feature(is_some_with)]

#[macro_use]
extern crate anyhow;

mod media;
mod config;
mod devices;

use core::time;
use std::thread;

use config::Config;

use crate::media::MediaType;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let config = match args.get(1) {
        Some(configfile) => Config::parse_file(configfile),
        _ => Config::minimal()
    };

    #[allow(unused_variables)]
    match config {
        Ok(conf) => {
            println!("parsed config, starting...");

            devices::poll(|event| {
                if let Ok(media) = TryInto::<Box<dyn MediaType>>::try_into(event.device()) {

                    println!("------------------------------------------------");
                    println!("{}: {}", media.path(), media.title());

                    media.process(&conf).unwrap();

                    thread::sleep(time::Duration::from_secs(2));

                    println!("------------------------------------------------");
                    media.eject();
                }
            });
        },
        Err(err) => eprintln!("Error in config: {}", err)
    }
}
