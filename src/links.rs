use std::fs;

use serde::Serialize;
use toml::Value;

use crate::context::Context2;

#[derive(Serialize, Clone)]
pub struct Link {
    pub alias: Option<String>,
    pub url: String,
}

pub fn get_primary(ctx: &Context2) -> Vec<Link> {
    let config_dir = ctx.config_dir();
    let primary_str = if let Ok(str) = fs::read_to_string(config_dir + "aliases.toml") {
        str
    } else {
        return Vec::new();
    };
    let primary = toml::from_str::<Value>(&primary_str).unwrap();
    let mut result = Vec::new();
    for (alias, url) in primary.as_table().unwrap() {
        result.push(Link {
            alias: Some(alias.clone()),
            url: url.as_str().unwrap().to_owned(),
        });
    }
    result
}

pub fn get_secondary(ctx: &Context2) -> Vec<Link> {
    let config_dir = ctx.config_dir();
    let secondary_str = if let Ok(str) = fs::read_to_string(config_dir + "aliases-2nd.toml") {
        str
    } else {
        return Vec::new();
    };
    let secondary = toml::from_str::<Value>(&secondary_str).unwrap();
    let mut result = Vec::new();
    for (alias, url) in secondary.as_table().unwrap() {
        result.push(Link {
            alias: Some(alias.clone()),
            url: url.as_str().unwrap().to_owned(),
        });
    }
    result
}

pub fn _save_secondary(ctx: &Context2, links: &Vec<Link>) {
    let secondary_str = toml::to_string(links).unwrap();
    fs::write(ctx.config_dir() + "aliases-2nd.toml", secondary_str).unwrap();
}

pub fn get_no_alias(ctx: &Context2) -> Vec<Link> {
    let config_dir = ctx.config_dir();
    let urls_str = if let Ok(str) = fs::read_to_string(config_dir + "urls.toml") {
        str
    } else {
        return Vec::new();
    };
    let urls = toml::from_str::<Value>(&urls_str).unwrap();
    let mut result = Vec::new();
    for url in urls["urls"].as_array().unwrap() {
        result.push(Link {
            alias: None,
            url: url.as_str().unwrap().to_owned(),
        });
    }
    result
}

pub fn _save_links_no_alias(ctx: &Context2, links: &Vec<Link>) {
    let urls_str = toml::to_string(links).unwrap();
    fs::write(ctx.config_dir() + "urls.toml", urls_str).unwrap();
}
