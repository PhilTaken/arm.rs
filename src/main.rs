#[macro_use]
extern crate anyhow;

mod media;
mod config;
mod devices;

use core::time;
use std::{thread, fs};

use config::Config;

use crate::media::MediaType;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let config = {
        if let Some(configfile) = args.get(0) {
            Config::parse_file(configfile)
        } else {
            Config::minimal()
        }
    };

    match config {
        Ok(conf) => {
            println!("parsed config, starting...");

            devices::poll(|event| {
                match TryInto::<Box<dyn MediaType>>::try_into(event.device()) {
                    Ok(media) => {

                        //let _ = media.process(&conf);

                        println!("------------------------------------------------");
                        println!("{}: {}", media.path(), media.title());

                        thread::sleep(time::Duration::from_secs(2));

                        println!("------------------------------------------------");
                        media.eject();
                    },
                    _ => {}
                }
            });
        }
        Err(err) => {
            panic!("error in config: {:?}", err);
        }
    }
}
