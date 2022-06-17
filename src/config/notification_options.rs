use serde::{Serialize, Deserialize};

use crate::config::default_false;


#[derive(Debug, Serialize, Deserialize)]
pub struct NotificationOptions {
    #[serde(default = "default_false")]
    rip: bool,

    #[serde(default = "default_false")]
    transcode: bool,

    #[serde(default = "default_false")]
    jobid: bool,

    // TODO: add notification backends
}

#[derive(Debug, Serialize, Deserialize)]
struct SimpleNotificationOptions {
    key: String
}

#[derive(Debug, Serialize, Deserialize)]
struct IFTTTOptions {
    key: String,
    event: String
}

#[derive(Debug, Serialize, Deserialize)]
struct PushOverOptions {
    user_key: String,
    app_key: String
}

#[derive(Debug, Serialize, Deserialize)]
struct AppriseOptions {
    config_file: String
}
