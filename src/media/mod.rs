#![allow(dead_code)]

mod video;
mod data;

use std::path::PathBuf;
use anyhow::Error;
use udev::Device;

#[allow(unused_imports)]
use data::DataDisc;

use video::{VideoDisc, VideoType};
use crate::config::Config;


fn media_from_dev(dev: &Device) -> Box<dyn MediaType> {
    let dpath = dev.devpath().to_os_string();
    let res = VideoDisc::new(VideoType::Bluray, dpath);
    Box::new(res)
}

trait MediaType {
    fn process(&self, config: &Config) -> Result<(), Error>;
}

struct Media {
    mtype: Box<dyn MediaType>,
    path: PathBuf,
}

impl Media {
    pub fn new(dev: Device) -> Self {
        Self {
            mtype: media_from_dev(&dev),
            path: dev.devpath().into()
        }
    }
}
