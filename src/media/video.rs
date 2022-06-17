#![allow(unused_variables)]

use std::path::{Path, PathBuf};
use anyhow::Error;
use crate::media::MediaType;
use crate::config::Config;


// ----------------------------------------------------------

/// Type of Video Disc
#[derive(Debug, Clone)]
pub enum VideoType {
    Bluray,
    Dvd
}

/// Video disc representation
#[derive(Debug, Clone)]
pub struct VideoDisc {
    vtype: VideoType,
    path: PathBuf,
    title: String,
    year: i32,
}

impl Default for VideoDisc {
    fn default() -> Self {
        Self {
            vtype: VideoType::Bluray,
            path: PathBuf::new(),
            title: String::new(),
            year: 0
        }
    }
}

impl VideoDisc {
    /// Construct a new Video Disc
    ///
    /// # Arguments
    ///
    /// * `vtype` - the type of Video Disc
    /// * `path` - the device's devnode
    pub fn new(vtype: VideoType, path: &Path) -> Self {
        Self {
            vtype, path: path.to_path_buf(), ..Self::default()
        }
    }

    /// Rip the Video Disc
    ///
    /// # Arguments
    ///
    /// * `config` - config for the ripping process
    #[allow(clippy::unused_self)]
    pub fn rip(&self, config: &Config) -> Result<(), Error> {
        todo!("implement")
    }

    /// Encode the Video Disc
    ///
    /// # Arguments
    ///
    /// * `config` - config for the encoding process
    #[allow(clippy::unused_self)]
    pub fn encode(&self, config: &Config) -> Result<(), Error> {
        todo!("implement")
    }

    /// Identify the Medium (name, year)
    #[allow(clippy::unused_self)]
    pub fn identify(&mut self, config: &Config) -> Result<(), Error> {
        todo!("implement")
    }
}

impl MediaType for VideoDisc {
    /// process the Video Disc by first ripping it and then optionally encoding the ripped files
    #[allow(clippy::all)]
    fn process(&mut self, config: &Config) -> Result<(), Error> {
        self.rip(config)?;
        self.encode(config)?;
        Ok(())
    }

    fn path(&self) -> String {
        self.path.to_str().unwrap().to_string()
    }
}
