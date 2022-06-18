use serde::{Serialize, Deserialize};

use crate::config::default_one;

fn default_extras_name() -> String { "extras".into() }

#[derive(Debug, Serialize, Deserialize)]
pub struct DirectoryOptions {
    pub abcde_config_file: Option<String>,

    // no default values for these options
    pub raw_rips_path: String,
    pub transcode_files_path: String,
    pub completed_files_path: String,

    #[serde(default = "default_extras_name")]
    pub extra_dir_name: String,

    #[serde(default)]
    pub log_dir: Option<String>,

    #[serde(default)]
    pub log_level: LogLevel,

    #[serde(default = "default_one")]
    pub log_life: i32,

    //install_path: String,
    //db_file: String
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub enum LogLevel { #[default] Debug, Info, Warning, Error, Crititcal }
