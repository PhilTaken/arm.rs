#![allow(unused_variables)]

use std::fs::{create_dir_all, read_dir};
use std::path::{Path, PathBuf};
use std::process::Command;
use anyhow::Error;
use crate::config::ext_options::{HandBrakeOptions, MakeMKVOptions, RipMethod};
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
    pub fn new(vtype: VideoType, path: &Path, title: &str) -> Self {
        Self {
            vtype, title: title.to_string(), path: path.to_path_buf(), ..Self::default()
        }
    }

    /// Rip the Video Disc
    ///
    /// # Arguments
    ///
    /// * `config` - config for the ripping process
    pub fn rip(&self, config: &MakeMKVOptions, ripdir: &Path, minlength: i32) -> Result<Vec<PathBuf>, Error> {
        // check if handbrake cli is available
        let makemkvpath = if let Some(path) = config.mkv_bin.clone() {
            assert!(Path::new(&path).is_file());
            path
        } else {
            let def = "makemkvcon";
            if !in_path(def)? {
                return Err(anyhow!("handbrake cli not in path"));
            };
            def.to_string()
        };


        let infile = self.path.to_str().unwrap();
        let outpath = ripdir.join(self.title.clone());
        let outfile = outpath.join(format!("{}.mkv", self.title));

        // TOOD: catch no license error
        let cmdout = Command::new(&makemkvpath)
            .args(["-r", "info", "disc:9999"])
            .output().unwrap()
            .stdout;
        let discinfo = std::str::from_utf8(&cmdout).map_err(|_| anyhow!("failed to parse disc info"))?;

        dbg!(discinfo);

        if discinfo.contains("registration key") {
            bail!("registration key missing");
        }

        // extract the disc number
        let discnr = discinfo.lines()
            .find(|line| line.contains(infile)).unwrap()
            .split(',').next().unwrap()
            .split(':').last().unwrap();

        let discnr_arg = format!("disc:{}", discnr);
        let minlength_arg = format!("--minlength={}", minlength);
        let args = if config.ripmethod == RipMethod::Backup && self.vtype == VideoType::Bluray {
            // backup --decrypt ${mkv_args} -r disc:${discnr} ${rawpath}
            vec!["backup", "--decrypt", "-r", &discnr_arg, outpath.to_str().unwrap()]
        } else {
            // mkv ${mkv_args} -r --progress=-stdout --messages=-stdout dev:${devpath} all ${rawpath} --minlength=${minlength}
            vec!["mkv", "-r", "--progress=-stdout", "--messages=-stdout", &discnr_arg, "all", outpath.to_str().unwrap(), &minlength_arg]
        };

        if ! outpath.exists() {
            create_dir_all(outpath.clone())?;
            Command::new(&makemkvpath)
                .args(args)
                .args(config.mkv_args.clone())
                // TODO: .stdout(logfile)
                .status()
                .map(|_| ())
                .map_err(|err| anyhow!(err))?;
        }

        Ok(read_dir(outpath).map(|it| {
            it.filter_map(|item| {
                match item {
                    Ok(entry) if entry.path().is_file() => Some(entry.path()),
                    _ => None,
                }
            })
        }).unwrap().collect())
    }

    /// Encode the Video Disc
    ///
    /// # Arguments
    ///
    /// * `config` - config for the encoding process
    #[allow(clippy::unused_self)]
    pub fn encode(&self, config: &HandBrakeOptions, input_file: &Path, finished_dir: &Path) -> Result<PathBuf, Error> {
        println!("encoding file: {}", input_file.display());

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
        let outfile = finished_dir.join(format!("{}.{}", self.title, &config.extension));

        let infile = input_file.to_str().unwrap();

        Command::new(handbrakepath)
            .args(config_args)
            .args(extra_args)
            .args(["-i", infile, "-o", outfile.to_str().unwrap()])
            // TODO: .stdout(logfile)
            .status()
            .map(|_| {
                // TODO: remove input file?
                println!("done encoding file {} to {}", input_file.display(), outfile.display());
                outfile
            })
            .map_err(|err| anyhow!(err))
    }

    /// Identify the Medium (name, year)
    #[allow(clippy::unused_self)]
    pub fn identify(&mut self, config: &Config) -> Result<(), Error> {
        todo!("implement")
    }
}

impl MediaType for VideoDisc {
    /// process the Video Disc by first ripping it and then optionally (wip) encoding the ripped files
    #[allow(clippy::all)]
    fn process(&self, config: &Config) -> Result<PathBuf, Error> {
        let ripdir = Path::new(&config.directories.raw_rips_path);
        let finished_dir = Path::new(&config.directories.completed_files_path);

        println!("ripping to:        {}", ripdir.display());
        println!("finished files in: {}", finished_dir.display());

        let rippedfiles = self.rip(&config.make_mkv, ripdir, config.arm.minlength)?;

        // filter out files that are much smaller than the biggest one
        let proper_files: Vec<PathBuf> = {
            let filesize = |file: PathBuf| file.metadata().unwrap().len();
            let files: Vec<PathBuf> = rippedfiles.into_iter().filter(|file| { file.as_path().is_file() }).collect();

            let biggest_file = files.clone().into_iter().map(filesize).max().unwrap();
            files.into_iter().filter(|file| {
                // TODO: remove filtered files?
                (file.metadata().unwrap().len() as f64 - biggest_file as f64).abs() < 0.3 * biggest_file as f64
            }).collect()
        };

        let ripped_files: Vec<Result<PathBuf, anyhow::Error>> = proper_files.iter().map(|rippedfile| {
            self.encode(&config.handbrake, &rippedfile, finished_dir)
        }).collect();

        Ok(finished_dir.to_path_buf())
    }

    fn path(&self) -> String {
        self.path.to_str().unwrap().to_string()
    }

    fn title(&self) -> String {
        self.title.clone()
    }
}
