use serde::{Serialize, Deserialize};

use crate::config::{
    default_one,
    default_true,
    default_false
};

fn default_waittime() -> i32 { 60 }
fn default_minlength() -> i32 { 600 }
fn default_maxlength() -> i32 { 99999 }
fn default_date_format() -> String { "%m-%d-%Y %H:%M:%S".into() }

fn default_name() -> String { "default".into() }

#[derive(Debug, Serialize, Deserialize)]
#[allow(clippy::struct_excessive_bools)]
pub struct ArmOptions {
    #[serde(default = "default_name")]
    name: String,

    #[serde(default)]
    children: Vec<String>,

    #[serde(default = "default_true")]
    prevent_99: bool,

    #[serde(default = "default_true")]
    check_udf: bool,

    #[serde(default = "default_true")]
    get_video_title: bool,

    #[serde(default)]
    api_key: Option<String>,

    #[serde(default = "default_true")]
    disable_login: bool,

    #[serde(default = "default_false")]
    skip_transcode: bool,

    #[serde(default)]
    videotype: VideoType,

    #[serde(default = "default_minlength")]
    minlength: i32,

    #[serde(default = "default_maxlength")]
    maxlength: i32,

    #[serde(default = "default_true")]
    manual_wait: bool,

    #[serde(default = "default_waittime")]
    manual_wait_time: i32,

    #[serde(default = "default_date_format")]
    date_format: String,

    #[serde(default = "default_true")]
    allow_duplicates: bool,

    #[serde(default = "default_one")]
    max_concurrent_transcodes: i32,

    #[serde(default)]
    data_rip_parameters: Vec<String>,

    #[serde(default)]
    video_metadata_provider: VideoMetadataProvider,

    #[serde(default)]
    audio_metadata_provider: AudioMetadataProvider,

    #[serde(default = "default_false")]
    rip_posters: bool
}

#[derive(Debug, Serialize, Deserialize, Default)]
enum VideoMetadataProvider { #[default] OmDB, TmDB }

#[derive(Debug, Serialize, Deserialize, Default)]
enum AudioMetadataProvider { #[default] MusicBrainz, FreeCDDB, None }

#[derive(Debug, Serialize, Deserialize, Default)]
enum VideoType { #[default] Auto, Series, Movie }

impl Default for ArmOptions {
    fn default() -> Self {
        Self {
            name: default_name(),
            children: Vec::default(),
            prevent_99: default_true(),
            check_udf: default_true(),
            get_video_title: default_true(),
            api_key: Option::default(),
            disable_login: default_true(),
            skip_transcode: default_false(),
            videotype: VideoType::default(),
            minlength: default_minlength(),
            maxlength: default_maxlength(),
            manual_wait: default_true(),
            manual_wait_time: default_waittime(),
            date_format: default_date_format(),
            allow_duplicates: default_true(),
            max_concurrent_transcodes: default_one(),
            data_rip_parameters: Vec::default(),
            video_metadata_provider: VideoMetadataProvider::default(),
            audio_metadata_provider: AudioMetadataProvider::default(),
            rip_posters: default_false()
        }
    }
}
