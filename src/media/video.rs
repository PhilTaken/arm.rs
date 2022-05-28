#![allow(unused_variables)]

use std::ffi::OsString;
use anyhow::Error;
use crate::media::MediaType;
use crate::config::Config;

// ----------------------------------------------------------

pub enum VideoType {
    Bluray,
    Dvd
}

pub struct VideoDisc {
    vtype: VideoType,
    path: OsString,
}

impl VideoDisc {
    pub fn new(vtype: VideoType, path: OsString) -> Self {
        Self { vtype, path }
    }

    pub fn rip(&self, config: &Config) -> Result<(), Error> {
        todo!("implement")
    }

    pub fn encode(&self, config: &Config) -> Result<(), Error> {
        todo!("implement")
    }

    // identify media using online sources (optional)
    pub fn identify(&self, config: &Config) {
        todo!("implement")

    }
}

impl MediaType for VideoDisc {
    fn process(&self, config: &Config) -> Result<(), Error> {
        self.rip(config);
        self.encode(config);

        todo!("implement");
        Ok(())
    }
}
