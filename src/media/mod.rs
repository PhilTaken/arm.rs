#![allow(dead_code)]

mod video;
mod data;

use std::{process::Command, ffi::OsStr, path::PathBuf};

use anyhow::Error;
use udev::Device;

#[allow(unused_imports)]
use data::DataDisc;

use video::{VideoDisc, VideoType};
use crate::config::Config;


/// Derives a `MediaType` from the given udev Device
///
/// # Arguments
///
/// * `dev` - the udev device to handle
pub fn media_from_dev(dev: &Device) -> Option<Box<dyn MediaType>> {

    // ID_FS_USAGE is only set when a disc is in the drive
    if dev.property_value("ID_FS_USAGE").and_then(OsStr::to_str).is_some() {
        println!("injected");
        let dpath = dev.devnode().unwrap();
        let title = dev.property_value("ID_FS_LABEL").map_or("UNKOWN", |label| label.to_str().unwrap()).to_string();

        // for bluray discs, ID_CDROM_MEDIA and ID_CDROM_MEDIA_BD are set to 1, for dvds only ID_CDROM_MEDIA
        let disc: Box<dyn MediaType> = match dev.property_value("ID_CDROM_MEDIA_BD") {
            Some(_) => Box::new(VideoDisc::new(VideoType::Bluray, dpath, title)),
            None => {
                match dev.property_value("ID_CDROM_MEDIA") {
                    Some(_) => Box::new(VideoDisc::new(VideoType::Dvd, dpath, title)),
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

/// represents any type of Disc inserted into your drive
pub trait MediaType {
    /// process the medium, a rip in most cases
    fn process(&mut self, config: &Config) -> Result<PathBuf, Error>;

    /// get the discs devnode
    fn path(&self) -> String;

    /// eject the disc
    fn eject(&self) {
        Command::new("eject").spawn().unwrap();
    }
}
