use std::collections::HashMap;

use clap::ArgMatches;
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Serialize, Deserialize)]
pub struct Links {
    pub youtube: Option<String>,
    pub twitch: Option<String>,
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
pub enum Target {
    Url(Url),
    YoutubeChannelHandle(String),
    MaubeAlias(String),
}

pub trait ToTarget {
    fn to_target(&mut self) -> Target;
}

impl ToTarget for String {
    fn to_target(&mut self) -> Target {
        if let Ok(url) = Url::parse(self) {
            Target::Url(url)
        } else if self.starts_with('@') {
            self.remove(0);
            Target::YoutubeChannelHandle(self.clone())
        } else {
            Target::MaubeAlias(self.clone())
        }
    }
}

impl Default for Target {
    fn default() -> Self {
        Target::MaubeAlias("DEFAULT_NO_TARGET".to_string())
    }
}
