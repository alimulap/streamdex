use std::{collections::HashMap, fmt::Display, fs};

use clap::ArgMatches;
use color_eyre::eyre::{Context as EyreContext, Result};
use indexmap::IndexMap;
use serde::{Deserialize, Deserializer, Serialize};
use url::Url;

use crate::{
    context::Context, error::Error, twitch::get_twitch_username,
    youtube::get_youtube_channel_handle,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Links {
    #[serde(default, deserialize_with = "deserialize_optional_link")]
    pub youtube: Option<LinkItem>,
    #[serde(default, deserialize_with = "deserialize_optional_link")]
    pub twitch: Option<LinkItem>,
}

impl Display for Links {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let something = match (&self.youtube, &self.twitch) {
            (Some(yt), Some(twitch)) => &format!("{} |{}", yt.display, twitch.display),
            (Some(yt), None) => &yt.display,
            (None, Some(twitch)) => &twitch.display,
            _ => "",
        };
        write!(f, "{something}")
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct LinkItem {
    pub url: Url,
    pub username: String,
    pub display: String,
}

fn deserialize_optional_link<'de, D>(deserializer: D) -> Result<Option<LinkItem>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Option<String> = Option::deserialize(deserializer)?;

    match s {
        Some(url_str) => match parse_link_item(&url_str) {
            Ok(item) => Ok(Some(item)),
            Err(_) => Ok(None),
        },
        None => Ok(None),
    }
}

fn parse_link_item(s: &str) -> Result<LinkItem, Box<dyn std::error::Error>> {
    let url = Url::parse(s)?;
    let platform = detect_platform(&url);

    let (username, display) = match platform {
        Platform::YouTube => {
            let username = get_youtube_channel_handle(url.as_str())
                .ok_or(Error::FailExtractUsername(platform, url.clone()))?;
            let display = format!(" YouTube(@{})", username);
            (username.to_string(), display)
        }
        Platform::Twitch => {
            let username = get_twitch_username(url.as_str())
                .ok_or(Error::FailExtractUsername(platform, url.clone()))?;
            let display = format!(" Twitch({})", username);
            (username.to_string(), display)
        }
        Platform::Unknown => return Err("Invalid url".into()),
    };

    Ok(LinkItem {
        url,
        username,
        display,
    })
}

pub type Aliases = HashMap<String, Links>;

pub struct PlatformFlags {
    pub youtube: bool,
    pub twitch: bool,
}

impl PlatformFlags {
    pub fn from_cli(cli: &ArgMatches) -> Self {
        Self {
            youtube: cli.get_one::<bool>("youtube").cloned().unwrap_or(false),
            twitch: cli.get_one::<bool>("twitch").cloned().unwrap_or(false),
        }
    }

    pub fn is_all(&self) -> bool {
        self.youtube && self.twitch || (!self.youtube && !self.twitch)
    }
}

#[derive(Debug)]
pub enum CliTarget {
    Url(Url),
    YoutubeChannelHandle(String),
    MaubeAlias(String),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TuiTarget {
    Url(Url),
    Links(Links),
}

impl TuiTarget {
    pub fn get_all(ctx: &Context) -> Result<IndexMap<String, TuiTarget>> {
        Ok(toml::from_str(
            &fs::read_to_string(&ctx.config.tui_targets).wrap_err("Failed to read tui_targets")?,
        )
        .unwrap_or_default())
    }
}

impl Display for TuiTarget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let something = match self {
            TuiTarget::Url(url) => url.to_string(),
            TuiTarget::Links(links) => links.to_string()
        };
        write!(f, "{something}")
    }
}

#[allow(unused)]
pub trait SaveTuiTargets {
    fn save(&self, ctx: &Context) -> Result<()>;
}

impl SaveTuiTargets for HashMap<String, TuiTarget> {
    fn save(&self, ctx: &Context) -> Result<()> {
        Ok(fs::write(&ctx.config.tui_targets, toml::to_string(self)?)?)
    }
}

pub trait ToCliTarget {
    fn to_target(&mut self) -> CliTarget;
}

impl ToCliTarget for String {
    fn to_target(&mut self) -> CliTarget {
        if let Ok(url) = Url::parse(self) {
            CliTarget::Url(url)
        } else if self.starts_with('@') {
            self.remove(0);
            CliTarget::YoutubeChannelHandle(self.clone())
        } else {
            CliTarget::MaubeAlias(self.clone())
        }
    }
}

impl Default for CliTarget {
    fn default() -> Self {
        CliTarget::MaubeAlias("DEFAULT_NO_TARGET".to_string())
    }
}

#[derive(Debug)]
pub enum Platform {
    YouTube,
    Twitch,
    Unknown,
}

fn detect_platform(url: &Url) -> Platform {
    match url.host_str() {
        Some(host) => {
            if host == "youtube.com" || host == "www.youtube.com" || host == "m.youtube.com" {
                Platform::YouTube
            } else if host == "twitch.tv" || host == "www.twitch.tv" {
                Platform::Twitch
            } else {
                Platform::Unknown
            }
        }
        None => Platform::Unknown,
    }
}

impl Display for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Platform::YouTube => write!(f, "YouTube"),
            Platform::Twitch => write!(f, "Twitch"),
            Platform::Unknown => write!(f, "Unknown"),
        }
    }
}
