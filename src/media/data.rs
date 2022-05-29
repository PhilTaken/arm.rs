#![allow(unused_variables)]

use std::path::{Path, PathBuf};
use anyhow::Error;
use crate::media::MediaType;
use crate::config::Config;

// ----------------------------------------------------------

pub struct DataDisc {
    path: PathBuf,
}

impl DataDisc {
    pub fn new(path: &Path) -> Self {
        Self { path: path.to_path_buf() }
    }
}

impl MediaType for DataDisc {
    #[allow(clippy::unused_self)]
    fn process(&self, config: &Config) -> Result<(), Error> {
        //self.rip(config);
        //self.encode(config);
        //Ok(())

        todo!("implement");
    }

    fn path(&self) -> String {
        self.path.to_str().unwrap().to_string()
    }
}

// ----------------------------------------------------------
