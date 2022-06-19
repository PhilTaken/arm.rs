use serde::{Serialize, Deserialize};

use crate::config::{
    default_true,
    default_false,
};

fn default_dvd_preset() -> String { "HQ 720p30 Surround".into() }
fn default_bluray_preset() -> String { "HQ 1080p30 Surround".into() }
fn default_extension() -> String { "m4v".into() }


#[derive(Debug, Serialize, Deserialize)]
pub struct MakeMKVOptions {
    #[serde(default)]
    pub ripmethod: RipMethod,

    #[serde(default)]
    pub mkv_bin: Option<String>,

    #[serde(default)]
    pub mkv_args: Vec<String>,

    #[serde(default = "default_true")]
    pub delete_raw_files: bool,

    #[serde(default = "default_false")]
    pub delete_hashed_keys: bool,
}

impl Default for MakeMKVOptions {
    fn default() -> Self {
        Self {
            ripmethod: RipMethod::default(),
            mkv_args: Vec::default(),
            delete_raw_files: default_true(),
            delete_hashed_keys: default_false(),
            mkv_bin: Option::default()
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Default, PartialEq, Eq)]
pub enum RipMethod { #[default] Backup, Mkv }

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HandBrakeOptions {
    #[serde(default = "default_dvd_preset")]
    pub preset_dvd: String,

    #[serde(default = "default_bluray_preset")]
    pub preset_bluray: String,

    #[serde(default = "default_extension")]
    pub extension: String,

    #[serde(default)]
    pub binary: Option<String>,

    #[serde(default = "default_true")]
    pub main_feature: bool,

    #[serde(default)]
    pub dvd_args: Vec<String>,

    #[serde(default)]
    pub bluray_args: Vec<String>,
}

impl Default for HandBrakeOptions {
    fn default() -> Self {
        Self {
            bluray_args: Vec::default(),
            dvd_args: Vec::default(),
            binary: Option::default(),
            preset_dvd: default_dvd_preset(),
            preset_bluray: default_bluray_preset(),
            extension: default_extension(),
            main_feature: default_true(),
        }
    }
}
