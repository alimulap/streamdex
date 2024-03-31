use std::{ops::{Deref, DerefMut}, collections::HashMap};

use crate::config::Config;

pub enum ContextValue<'b> {
    String(&'b String),
    OptionString(Option<&'b String>),
    U32(&'b u32),
    Boolean(&'b bool),
    Config(Config),
}

impl<'a>  ContextValue<'a> {
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

impl<'a> Context<'a> {
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
