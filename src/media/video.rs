#![allow(unused_variables)]

use std::path::{Path, PathBuf};
use anyhow::Error;
use crate::media::MediaType;
use crate::config::Config;

// ----------------------------------------------------------

#[derive(Debug, Clone)]
pub enum VideoType {
    Bluray,
    Dvd
}

#[derive(Debug, Clone)]
pub struct VideoDisc {
    vtype: VideoType,
    path: PathBuf,
}

impl VideoDisc {
    pub fn new(vtype: VideoType, path: &Path) -> Self {
        Self { vtype, path: path.to_path_buf() }
    }

    #[allow(clippy::unused_self)]
    pub fn rip(&self, config: &Config) -> Result<(), Error> {
        todo!("implement")
    }

    #[allow(clippy::unused_self)]
    pub fn encode(&self, config: &Config) -> Result<(), Error> {
        todo!("implement")
    }

    // identify media using online sources (optional)
    #[allow(clippy::unused_self)]
    pub fn identify(&self, config: &Config) {
        todo!("implement")
    }
}

impl MediaType for VideoDisc {
    #[allow(clippy::all)]
    fn process(&self, config: &Config) -> Result<(), Error> {
        todo!("implement");
        //self.rip(config);
        //self.encode(config);

        //Ok(())
    }

    fn path(&self) -> String {
        self.path.to_str().unwrap().to_string()
    }
}
