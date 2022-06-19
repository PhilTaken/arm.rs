#![allow(unused_variables)]

use std::path::{Path, PathBuf};
use std::process::Command;
use anyhow::Error;
use crate::config::ext_options::{HandBrakeOptions, MakeMKVOptions, RipMethod};
use crate::media::MediaType;
use crate::config::Config;
use std::{str, any};

fn in_path(path: &str) -> Result<bool, std::env::VarError> {
    std::env::var("PATH").map(|paths| paths
        .split(':')
        .map(|p| format!("{}/{}", p, path))
        .any(|p| Path::new(&p).exists()))
}

// ----------------------------------------------------------

/// Type of Video Disc
#[derive(Debug, Clone, PartialEq, Eq)]
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
    /// * `title` - the discs fs label
    pub fn new(vtype: VideoType, path: &Path, title: String) -> Self {
        Self {
            vtype, title, path: path.to_path_buf(), ..Self::default()
        }
    }

    /// Rip the Video Disc
    ///
    /// # Arguments
    ///
    /// * `config` - config for the ripping process
    pub fn rip(&self, config: &MakeMKVOptions, ripdir: &Path, minlength: i32) -> Result<PathBuf, Error> {
        // check if handbrake cli is available
        let makemkvpath = if let Some(path) = config.mkv_bin.clone() {
            assert!(Path::new(&path).is_file());
            path
        } else {
            let def = "HandBrakeCLI";
            if !in_path(def)? {
                return Err(anyhow!("handbrake cli not in path"));
            };
            def.to_string()
        };

        let infile = self.path.to_str().unwrap();
        let outpath = ripdir.join(self.title.clone());
        let outfile = outpath.join(format!("{}.mkv", self.title));

        let cmdout = Command::new(&makemkvpath)
            .args(["-r", "info", "disc:9999"])
            .output().unwrap()
            .stdout;
        let discinfo = str::from_utf8(&cmdout).map_err(|_| anyhow!("failed to parse disc info"))?;

        // extract the disc number
        let discnr = discinfo.lines()
            .find(|line| line.contains(infile)).unwrap()
            .split(',').next().unwrap()
            .split(':').last().unwrap();


        let discnr_arg = format!("disc:{}", discnr);
        let minlength_arg = format!("--minlength={}", minlength);
        let args = if config.ripmethod == RipMethod::Backup && self.vtype == VideoType::Bluray {
            // backup --decrypt ${mkv_args} -r disc:${discnr} ${rawpath} >> ${logfile}
            vec!["backup", "--decrypt", "-r", &discnr_arg, outpath.to_str().unwrap()]
        } else {
            // makemkvcon mkv ${mkv_args} -r --progress=-stdout --messages=-stdout dev:${devpath} all ${rawpath} --minlength=${minlength} >> ${logfile}
            vec!["mkv", "-r", "--progress=-stdout", "--messages=-stdout", &discnr_arg, "all", outfile.to_str().unwrap(), &minlength_arg]
        };

        Command::new(&makemkvpath)
            .args(args)
            // TODO: .stdout(logfile)
            .status()
            .map(|_| outfile)
            .map_err(|err| anyhow!(err))

    }

    /// Encode the Video Disc
    ///
    /// # Arguments
    ///
    /// * `config` - config for the encoding process
    #[allow(clippy::unused_self)]
    pub fn encode(&self, config: &HandBrakeOptions, input_file: &Path, finished_dir: &Path) -> Result<PathBuf, Error> {
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
            config_args.push("--main-feature");
        }
        let extension = &config.extension;
        let outfile = format!("{}.{}", self.title, &config.extension);

        let infile = input_file.to_str().unwrap();
        Command::new(handbrakepath)
            .args(config_args)
            .args(extra_args)
            .args(["-i", infile, "-o", &outfile])
            // TODO: .stdout(logfile)
            .status()
            .map(|_| PathBuf::from(outfile))
            .map_err(|err| anyhow!(err))
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
    fn process(&mut self, config: &Config) -> Result<PathBuf, Error> {
        let ripdir = Path::new(&config.directories.raw_rips_path);
        let finished_dir = Path::new(&config.directories.completed_files_path);

        let rippedfile = self.rip(&config.make_mkv, ripdir, config.arm.minlength)?;
        let finishedfile = self.encode(&config.handbrake, &rippedfile, finished_dir)?;
        Ok(finishedfile)
    }

    fn path(&self) -> String {
        self.path.to_str().unwrap().to_string()
    }
}
