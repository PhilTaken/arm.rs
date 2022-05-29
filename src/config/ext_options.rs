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
    ripmethod: RipMethod,

    #[serde(default)]
    mkv_args: Vec<String>,

    #[serde(default = "default_true")]
    delete_raw_files: bool,

    #[serde(default = "default_false")]
    delete_hashed_keys: bool,
}

impl Default for MakeMKVOptions {
    fn default() -> Self {
        Self {
            ripmethod: RipMethod::default(),
            mkv_args: Vec::default(),
            delete_raw_files: default_true(),
            delete_hashed_keys: default_false(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
enum RipMethod { #[default] Backup, Mkv }

#[derive(Debug, Serialize, Deserialize)]
pub struct HandBrakeOptions {
    #[serde(default = "default_dvd_preset")]
    preset_dvd: String,

    #[serde(default = "default_bluray_preset")]
    preset_bluray: String,

    #[serde(default = "default_extension")]
    extension: String,

    #[serde(default)]
    binary: Option<String>,

    #[serde(default = "default_true")]
    main_feature: bool,

    #[serde(default)]
    dvd_args: Vec<String>,

    #[serde(default)]
    bluray_args: Vec<String>,
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
