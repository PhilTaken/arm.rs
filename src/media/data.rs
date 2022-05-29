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
}

impl MediaType for DataDisc {
    #[allow(clippy::all)]
    fn rip(&self, config: &Config) -> Result<(), Error> {
        todo!("implement")
    }

    #[allow(clippy::all)]
    fn process(&self, config: &Config) -> Result<(), Error> {
        //self.rip(config);
        //self.encode(config);
        //Ok(())

        todo!("implement");
    }
}

// ----------------------------------------------------------
