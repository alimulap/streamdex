#![allow(dead_code)]

use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
};

use crate::config::Config;

pub enum ContextValue<'b> {
    String(&'b String),
    OptionString(Option<&'b String>),
    U32(&'b u32),
    Boolean(&'b bool),
    Config(Config),
}

impl ContextValue<'_> {
    pub fn as_string(&self) -> Option<&String> {
        match self {
            Self::String(s) => Some(s),
            _ => None,
        }
    }

    pub fn as_u32(&self) -> Option<&u32> {
        match self {
            Self::U32(u) => Some(u),
            _ => None,
        }
    }
    pub fn as_boolean(&self) -> Option<&bool> {
        match self {
            Self::Boolean(b) => Some(b),
            _ => None,
        }
    }

    pub fn as_option_string(&self) -> Option<Option<&String>> {
        match self {
            Self::OptionString(s) => Some(*s),
            _ => None,
        }
    }

    pub fn as_config(&self) -> Option<&Config> {
        match self {
            Self::Config(c) => Some(c),
            _ => None,
        }
    }
}

pub struct Context<'a>(HashMap<&'a str, ContextValue<'a>>);

impl Context<'_> {
    pub fn new() -> Self {
        Self(HashMap::new())
    }
}

impl<'a> Deref for Context<'a> {
    type Target = HashMap<&'a str, ContextValue<'a>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Context<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Default)]
pub struct Context2 {
    pub config: Config,
    pub subcommand: String,
    pub url: Option<String>,
    pub resolution: Option<String>,
    pub tool: Option<String>,
    pub room: Option<String>,
    pub wait_for_video: Option<bool>,
    pub from: Option<String>,
    pub to: Option<String>,
    pub id: Option<String>,
    pub delay: Option<u32>,
}

impl Context2 {
    pub fn new(config: Config, subcommand: &str) -> Self {
        Self {
            config,
            subcommand: subcommand.into(),
            ..Default::default()
        }
    }

    pub fn url(&self) -> String {
        match self.subcommand.as_str() {
            "live" => self.url.clone().unwrap(),
            "video" => self.id.clone().unwrap(),
            _ => panic!("Invalid subcommand"),
        }
    }

    pub fn resolution(&self) -> String {
        match self.subcommand.as_str() {
            "live" => self.resolution.clone().unwrap(),
            "video" => self.resolution.clone().unwrap(),
            _ => panic!("Invalid subcommand"),
        }
    }

    pub fn tool(&self) -> String {
        match self.subcommand.as_str() {
            "live" => self.tool.clone().unwrap(),
            "video" => self.tool.clone().unwrap(),
            _ => panic!("Invalid subcommand"),
        }
    }

    pub fn room(&self) -> Option<String> {
        match self.subcommand.as_str() {
            "live" => self.room.clone(),
            "video" => self.room.clone(),
            _ => panic!("Invalid subcommand"),
        }
    }

    pub fn wait_for_video(&self) -> bool {
        match self.subcommand.as_str() {
            "live" => self.wait_for_video.unwrap_or(false),
            "video" => false,
            _ => panic!("Invalid subcommand"),
        }
    }

    pub fn from(&self) -> Option<String> {
        match self.subcommand.as_str() {
            "live" => self.from.clone(),
            "video" => self.from.clone(),
            _ => panic!("Invalid subcommand"),
        }
    }

    pub fn to(&self) -> Option<String> {
        match self.subcommand.as_str() {
            "live" => self.to.clone(),
            "video" => self.to.clone(),
            _ => panic!("Invalid subcommand"),
        }
    }
}
