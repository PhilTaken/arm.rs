use std::fs;

use anyhow::Error;
use serde::{Serialize, Deserialize};

mod arm_options;
mod notification_options;
mod dir_options;
mod ext_options;

use arm_options::ArmOptions;
use notification_options::NotificationOptions;
use dir_options::DirectoryOptions;
use ext_options::{
    MakeMKVOptions,
    HandBrakeOptions
};

fn default_true() -> bool { true }
fn default_false() -> bool { false }
fn default_one() -> i32 { 1 }

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    /// core options, optional
    #[serde(default)]
    pub arm: ArmOptions,

    /// directory setup, needs to be set
    pub directories: DirectoryOptions,

    /// config for running MakeMKV, a Disc ripping program, optional
    #[serde(default)]
    pub make_mkv: MakeMKVOptions,

    /// config for runnig HandBrake, a video encoder, optional
    #[serde(default)]
    pub handbrake: HandBrakeOptions,

    /// config for notifications, optional
    #[serde(default)]
    pub notifications: Option<NotificationOptions>,

    /// config for the web server (wip), optional
    #[serde(default)]
    pub web_server: Option<WebServerOptions>,

    /// config for setting file / directory permissions, optional
    #[serde(default)]
    pub file_permissions: FilePermissionOptions,
}

impl Config {
    #[allow(dead_code)]
    pub fn parse_file(filename: &str) -> Result<Self, Error> {
        let conf = fs::read_to_string(filename)?;
        Self::parse_str(&conf)
    }

    pub fn parse_str(conf: &str) -> Result<Self, Error> {
        Ok(toml::from_str(conf)?)
    }

    #[allow(dead_code)]
    pub fn minimal() -> Result<Self, Error> {
        Self::parse_str(r#"
            [directories]
            raw_rips_path = ""
            transcode_files_path = ""
            completed_files_path = ""
        "#)
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct WebServerOptions { ip: String, port: i32 }

#[derive(Debug, Serialize, Deserialize)]
pub struct FilePermissionOptions {
    pub set_media_permissions: bool,
    pub chmod_value: i32,
    pub set_media_owner: bool,
    pub chown_user: String,
    pub chown_group: String,
}

impl Default for FilePermissionOptions {
    fn default() -> Self {
        Self {
            set_media_permissions: false,
            chmod_value: 777,
            set_media_owner: false,
            chown_user: "".into(),
            chown_group: "".into(),
        }
    }
}


#[test]
fn minimal_config() {
    let armconf = Config::minimal();
    assert!(armconf.is_ok());
}
