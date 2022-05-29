#![allow(dead_code)]

mod video;
mod data;

use std::{process::Command, ffi::OsStr};

use anyhow::Error;
use udev::Device;

#[allow(unused_imports)]
use data::DataDisc;

use video::{VideoDisc, VideoType};
use crate::config::Config;


pub fn media_from_dev(dev: &Device) -> Option<Box<dyn MediaType>> {

    //dev.properties().for_each(|attr| {
        //println!("{:?}:\t{:?}", attr.name(), attr.value());
    //});

    if dev.property_value("ID_FS_USAGE").and_then(OsStr::to_str).is_some() {
        println!("injected");
        let dpath = dev.devnode().unwrap();
        let disc: Box<dyn MediaType> = match dev.property_value("ID_CDROM_MEDIA_BD") {
            Some(_) => Box::new(VideoDisc::new(VideoType::Bluray, dpath)),
            None => {
                match dev.property_value("ID_CDROM_MEDIA") {
                    Some(_) => Box::new(VideoDisc::new(VideoType::Dvd, dpath)),
                    None => Box::new(DataDisc::new(dpath)),
                }
            }
        };
        Some(disc)
    } else {
        println!("ejected");
        None
    }
}

pub trait MediaType {
    fn process(&self, config: &Config) -> Result<(), Error>;
    fn path(&self) -> String;

    fn eject(&self) {
        Command::new("eject").spawn().unwrap();
    }
}
