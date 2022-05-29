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
    #[serde(default)]
    arm: ArmOptions,

    //#[serde(default)]
    directories: DirectoryOptions,

    #[serde(default)]
    make_mkv: MakeMKVOptions,

    #[serde(default)]
    handbrake: HandBrakeOptions,

    #[serde(default)]
    notifications: Option<NotificationOptions>,

    #[serde(default)]
    web_server: Option<WebServerOptions>,

    #[serde(default)]
    file_permissions: Option<FilePermissionOptions>,
}

impl Config {
    pub fn parse(conf: &str) -> Result<Self, Error> {
        Ok(toml::from_str(conf)?)
    }
}


#[derive(Debug, Serialize, Deserialize)]
struct WebServerOptions { ip: String, port: i32 }

#[derive(Debug, Serialize, Deserialize)]
struct FilePermissionOptions {
    set_media_permissions: bool,
    chmod_value: i32,
    set_media_owner: bool,
    chown_user: String,
    chown_group: String,
}


#[test]
fn minimal_config() {
    let testconf = r#"
        [directories]
        raw_rips_path = ""
        transcode_files_path = ""
        completed_files_path = ""
    "#;

    let armconf = Config::parse(testconf);
    assert!(armconf.is_ok());
}
