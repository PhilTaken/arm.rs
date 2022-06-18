#![allow(unused_variables)]

use std::path::{Path, PathBuf};
use std::process::Command;
use anyhow::Error;
use udev::Device;
use crate::config::ext_options::{HandBrakeOptions, MakeMKVOptions};
use crate::media::MediaType;
use crate::config::Config;


fn in_path(path: &str) -> Result<bool, std::env::VarError> {
    std::env::var("PATH").map(|paths| paths
        .split(':')
        .map(|p| format!("{}/{}", p, path))
        .any(|p| Path::new(&p).exists()))
}

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
    pub fn rip(&self, config: &HandBrakeOptions, input_file: &Path, ripdir: &Path, title: &str) -> Result<PathBuf, Error> {
        // check if handbrake cli is available
        let handbrakepath = if let Some(path) = config.binary.clone() {
            assert!(Path::new(&path).is_file());
            path
        } else {
            let def = "HandBrakeCLI";
            if !in_path(def)? {
                return Err(anyhow!("handbrake cli not in path"));
            };
            def.to_string()
        };

        let extra_args = match self.vtype {
            VideoType::Dvd => config.dvd_args.clone(),
            VideoType::Bluray => config.bluray_args.clone()
        };

        let mut config_args = vec!["--preset", &config.preset_bluray];
        if config.main_feature {
            config_args.insert(0, "--main-feature");
        }
        let extension = &config.extension;
        let outfile = format!("{}.{}", title, &config.extension);

        let infile = input_file.to_str().unwrap();
        let output = Command::new(handbrakepath)
            .args(config_args)
            .args(extra_args)
            .args(["-i", infile, "-o", &outfile])
            .output();

        Ok(PathBuf::from(outfile))
    }

    /// Encode the Video Disc
    ///
    /// # Arguments
    ///
    /// * `config` - config for the encoding process
    #[allow(clippy::unused_self)]
    pub fn encode(&self, config: &MakeMKVOptions, input_file: &Path, finished_dir: &Path) -> Result<PathBuf, Error> {
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
    fn process(&mut self, device: &Device, config: &Config) -> Result<PathBuf, Error> {
        let input = Path::new(device.devnode().unwrap());
        let title = device.property_value("ID_FS_LABEL").map_or("UNKOWN", |label| label.to_str().unwrap());
        let ripdir = Path::new(&config.directories.raw_rips_path);
        let finished_dir = Path::new(&config.directories.completed_files_path);

        let rippedfile = self.rip(&config.handbrake, input, ripdir, title)?;
        let finishedfile = self.encode(&config.make_mkv, &rippedfile, finished_dir)?;
        Ok(finishedfile)
    }

    fn path(&self) -> String {
        self.path.to_str().unwrap().to_string()
    }
}
