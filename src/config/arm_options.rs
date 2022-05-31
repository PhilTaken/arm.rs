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
    /// A friendly name for this machine
    /// used in notification titles
    ///
    /// # Default
    ///
    /// ```text
    /// "default"
    /// ```
    #[serde(default = "default_name")]
    pub name: String,

    /// List of child arm servers to be displayed on the home page
    /// should be full protocol, path and port:
    ///
    /// # Example
    ///
    /// ```text
    /// "http://192.168.0.100:8080"
    /// ```
    ///
    /// # Default
    ///
    /// ```text
    /// ""
    /// ```
    #[serde(default)]
    pub children: Vec<String>,

    /// prevent arm from wasting time on 99 Title protected DVDs
    /// A DRM scheme used on some DVDs which creates fake titles which confuse Handbrake
    /// When set to true, affected DVDs will be autoejected on insertion
    ///
    /// # Default
    ///
    /// ```
    /// true
    /// ```
    #[serde(default = "default_true")]
    pub prevent_99: bool,

    /// Distinguish UDF video discs from UDF data discs.
    /// Requires mounting discs so it will add a few seconds to the identify script
    ///
    /// # Default
    ///
    /// ```
    /// true
    /// ```
    #[serde(default = "default_true")]
    pub check_udf: bool,

    /// Attempt to get the video title of the disc
    /// For DVDs, dvdid is used
    /// For BluRays ARM will attempt to extract the title from an XML file on the disc
    ///
    /// # Default
    ///
    /// ```
    /// true
    /// ```
    #[serde(default = "default_true")]
    pub get_video_title: bool,

    /// crc64 api key
    /// only needed to send movies to the database
    ///
    /// # Default
    ///
    /// ```
    /// ""
    /// ```
    #[serde(default)]
    pub api_key: String,

    /// Enable auto login for the website
    /// setting to false will require you to login to access the ui
    ///
    /// # Default
    ///
    /// ```
    /// true
    /// ```
    #[serde(default = "default_true")]
    pub disable_login: bool,

    /// Skip transcoding if you just want the original source
    ///
    /// # Default
    ///
    /// ```
    /// false
    /// ```
    #[serde(default = "default_false")]
    pub skip_transcode: bool,

    /// Video type identification
    ///
    /// # Default
    ///
    /// ```
    /// VideType::Auto
    /// ```
    #[serde(default)]
    pub videotype: VideoType,

    /// Minimum length for a track to be ripped (in seconds)
    ///
    /// # Default
    ///
    /// ```
    /// 600
    /// ```
    #[serde(default = "default_minlength")]
    pub minlength: i32,

    /// Maximum length for a track to be ripped (in seconds)
    ///
    /// # Default
    ///
    /// ```
    /// 99999
    /// ```
    #[serde(default = "default_maxlength")]
    pub maxlength: i32,

    /// Wait for manual identification
    ///
    /// # Default
    ///
    /// ```
    /// true
    /// ```
    #[serde(default = "default_true")]
    pub manual_wait: bool,

    /// Wait time for manual identification
    ///
    /// # Default
    ///
    /// ```
    /// 60
    /// ```
    #[serde(default = "default_waittime")]
    pub manual_wait_time: i32,

    /// Date format string for the UI and logging
    ///
    /// # Default
    ///
    /// ```
    /// "%m-%d-%Y %H:%M:%S"
    /// ```
    #[serde(default = "default_date_format")]
    pub date_format: String,

    /// Allow duplicate rips
    /// Recommended to set to true for series
    ///
    /// # Default
    ///
    /// ```
    /// true
    /// ```
    #[serde(default = "default_true")]
    pub allow_duplicates: bool,

    /// Maximum concurrent transcodes
    ///
    /// # Default
    ///
    /// ```
    /// 1
    /// ```
    #[serde(default = "default_one")]
    pub max_concurrent_transcodes: i32,

    /// parameters for ripping data (using dd)
    ///
    /// # Example
    ///
    /// ```
    /// "conv=noerror,sync,staus=progress"
    /// ```
    ///
    /// # Default
    ///
    /// ```
    /// ""
    /// ```
    #[serde(default)]
    pub data_rip_parameters: Vec<String>,

    /// Metadata provider for Video discs
    ///
    /// # Default
    ///
    /// ```
    /// VideoMetadataProvider::OmDB
    /// ```
    #[serde(default)]
    pub video_metadata_provider: VideoMetadataProvider,

    /// Metdata provider for Audio discs
    ///
    /// # Default
    ///
    /// ```
    /// AudioMetadataProvider::MusicBrainz
    /// ```
    #[serde(default)]
    pub audio_metadata_provider: AudioMetadataProvider,

    /// Rip DVD Posters from the JACKET_P folder
    ///
    /// requires ffmpeg
    ///
    /// # Default
    ///
    /// ```
    /// false
    /// ```
    #[serde(default = "default_false")]
    pub rip_posters: bool
}

/// Video metatdata database to use as provider
///
/// # Default
///
/// ```
/// VideoMetadataProvider::OmDB
/// ```
#[derive(Debug, Serialize, Deserialize, Default)]
pub enum VideoMetadataProvider {
    /// <https://www.omdbapi.com//>
    #[default] OmDB,
    /// <https://www.themoviedb.org//>
    TmDB,
    /// don't identify Video
    None
}

/// Audio metadata database to use as provider
///
/// # Default
///
/// ```
/// AudioMetadataProvider::MusicBrainz
/// ```
#[derive(Debug, Serialize, Deserialize, Default)]
pub enum AudioMetadataProvider {
    /// <https://musicbrainz.org//>
    #[default] MusicBrainz,
    /// <https://gnudb.org//>
    FreeCDDB,
    /// don't identify audio
    None
}

/// Type of Video to rip
///
/// # Default
///
/// ```
/// VideoType::Auto
/// ```
#[derive(Debug, Serialize, Deserialize, Default)]
pub enum VideoType {
    /// Automatically identify the Type
    #[default] Auto,
    /// Always rip as Series
    Series,
    /// Always rip as movie
    Movie
}

impl Default for ArmOptions {
    fn default() -> Self {
        Self {
            name: default_name(),
            children: Vec::default(),
            prevent_99: default_true(),
            check_udf: default_true(),
            get_video_title: default_true(),
            api_key: String::default(),
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
