#![allow(unused_variables)]

use std::ffi::OsString;
use anyhow::Error;
use crate::media::MediaType;
use crate::config::Config;

// ----------------------------------------------------------

pub struct DataDisc {
    path: OsString,
}

impl DataDisc {
    pub fn new(path: OsString) -> Self {
        Self { path }
    }

    pub fn rip(&self, config: &Config) -> Result<(), Error> {
        todo!("implement")
    }

    pub fn encode(&self, config: &Config) -> Result<(), Error> {
        todo!("implement")
    }
}

impl MediaType for DataDisc {
    fn process(&self, config: &Config) -> Result<(), Error> {
        self.rip(config);
        self.encode(config);

        todo!("implement");
        Ok(())
    }
}

// ----------------------------------------------------------
