use serde::{Serialize, Deserialize};

use crate::config::default_one;

fn default_extras_name() -> String { "extras".into() }

#[derive(Debug, Serialize, Deserialize)]
pub struct DirectoryOptions {
    abcde_config_file: Option<String>,

    // no default values for these options
    raw_rips_path: String,
    transcode_files_path: String,
    completed_files_path: String,

    #[serde(default = "default_extras_name")]
    extra_dir_name: String,

    #[serde(default)]
    log_dir: Option<String>,

    #[serde(default)]
    log_level: LogLevel,

    #[serde(default = "default_one")]
    log_life: i32,

    //install_path: String,
    //db_file: String
}

#[derive(Debug, Serialize, Deserialize, Default)]
enum LogLevel { #[default] Debug, Info, Warning, Error, Crititcal }
