#![allow(unused)]

use std::fmt::Display;

use url::Url;

use crate::target::Platform;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("No broadcast found in channel")]
    NoBroadcast,
    #[error("No channel found with handle/username: {0}")]
    NoChannelFound(String),
    #[error("Fetching youtube live stream failed from channel_id {0}")]
    YTFetchLiveFailed(String, google_youtube3::Error),
    #[error("Fetching youtube video detail failed")]
    YTFailFetchVideoDetail(google_youtube3::Error),
    #[error("Data {0} not found in fetch result")]
    NoDataFound(FetchData),
    #[error("Failed to extract username from {0}({1}) url")]
    FailExtractUsername(Platform, Url),
    #[error("Anyhow Error {0}")]
    Anyhow(anyhow::Error),
    #[error("Eyre Error {0}")]
    Eyre(color_eyre::eyre::Error)
}

impl From<anyhow::Error> for Error {
    fn from(value: anyhow::Error) -> Self {
        Error::Anyhow(value)
    }
}

impl From<color_eyre::eyre::Error> for Error {
    fn from(value: color_eyre::eyre::Error) -> Self {
        Error::Eyre(value)
    }
}

#[derive(Debug)]
pub enum FetchData {
    ChannelID,
    VideoID,
    LiveDetail,
    Snippet,
}

impl Display for FetchData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FetchData::ChannelID => write!(f, "Channel Id"),
            FetchData::VideoID => write!(f, "Video Id"),
            FetchData::LiveDetail => write!(f, "Live detail"),
            FetchData::Snippet => write!(f, "Snippet"),
        }
    }
}
