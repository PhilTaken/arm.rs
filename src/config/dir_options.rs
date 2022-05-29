use serde::{Serialize, Deserialize};

use crate::config::default_one;

fn default_extras_name() -> String { "extras".into() }

#[derive(Debug, Serialize, Deserialize)]
pub struct DirectoryOptions {
    abcde_config_file: Option<String>,

    raw_rips_path: String,
    transcode_files_path: String,
    completed_files_path: String,

    #[serde(default = "default_extras_name")]
    extra_dir_name: String,

    //install_path: String,

    log_dir: Option<String>,
    log_level: LogLevel,

    #[serde(default = "default_one")]
    log_life: i32,

    //db_file: String
}

#[derive(Debug, Serialize, Deserialize, Default)]
enum LogLevel { #[default] Debug, Info, Warning, Error, Crititcal }


