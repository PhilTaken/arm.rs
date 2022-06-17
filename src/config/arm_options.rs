use serde::{Serialize, Deserialize};

use crate::config::{
    default_true,
    default_false
};

fn default_minlength() -> i32 { 600 }
fn default_maxlength() -> i32 { 99999 }
fn default_name() -> String { "ARM".into() }

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
            prevent_99: default_true(),
            get_video_title: default_true(),
            skip_transcode: default_false(),
            videotype: VideoType::default(),
            minlength: default_minlength(),
            maxlength: default_maxlength(),
            allow_duplicates: default_true(),
            data_rip_parameters: Vec::default(),
        }
    }
}
