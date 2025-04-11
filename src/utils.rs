use std::{fs, process::Command};

use toml::Value;
use url::Url;

use crate::context::Context;

pub trait FromAlias: Sized {
    fn from_alias(ctx: &Context, maybe_alias: &str) -> Option<Self>;
    fn from_alias2(ctx: &Context, maybe_alias: &str) -> Option<Self>;
}

impl FromAlias for Url {
    fn from_alias(ctx: &Context, maybe_alias: &str) -> Option<Self> {
        let print_command = ctx.print_command.unwrap_or(false);
        if !print_command {
            println!("Checking for alias: {}", maybe_alias);
        }
        let config = &ctx.config;
        let aliases_path = config.aliases_path.as_ref().unwrap();
        let aliases_str = fs::read_to_string(aliases_path).unwrap();
        let aliases = toml::from_str::<Value>(&aliases_str).unwrap();
        //let aliases: Value = serde_json::from_str(include_str!("aliases.json")).unwrap();
        match aliases.get(maybe_alias.to_ascii_lowercase()) {
            Some(url) => {
                if !print_command {
                    println!(
                        "Found url for alias {}: {}",
                        maybe_alias,
                        url.as_str().unwrap()
                    );
                }
                Some(Url::parse(url.as_str().unwrap()).expect("Invalid url from aliases.json"))
            }
            None => None,
        }
    }

    fn from_alias2(ctx: &Context, maybe_alias: &str) -> Option<Self> {
        let print_command = ctx.print_command.unwrap_or(false);
        if !print_command {
            println!("Checking for alias: {}", maybe_alias);
        }
        let aliases_path = ctx.config.aliases_path.as_ref().unwrap();
        let aliases_str = fs::read_to_string(aliases_path).unwrap();
        let aliases = toml::from_str::<Value>(&aliases_str).unwrap();
        //let aliases: Value = serde_json::from_str(include_str!("aliases.json")).unwrap();
        match aliases.get(maybe_alias.to_ascii_lowercase()) {
            Some(url) => {
                if !print_command {
                    println!(
                        "Found url for alias {}: {}",
                        maybe_alias,
                        url.as_str().unwrap()
                    );
                }
                Some(Url::parse(url.as_str().unwrap()).expect("Invalid url from aliases.json"))
            }
            None => None,
        }
    }
}

pub fn get_webpage_url(url: &str) -> Option<Url> {
    let output = Command::new("yt-dlp")
        .arg(url)
        .arg("--print")
        .arg("webpage_url")
        .arg("--cookies-from-browser")
        .arg("firefox")
        .output()
        .unwrap()
        .stdout;
    match String::from_utf8(output) {
        Ok(url) => match Url::parse(url.trim()) {
            Ok(url) => Some(url),
            Err(e) => {
                println!("Invalid url after yt-dlp: {}", url.trim());
                println!("Error: {}", e);
                None
            }
        },
        Err(_) => None,
    }
}

pub fn get_format(url: &Url, res: Resolution, ctype: ContentType) -> String {
    match url.host_str() {
        Some(host) => match host {
            "www.youtube.com" | "youtube.com" | "youtu.be" => {
                get_format_from_res(Host::Youtube, res, ctype)
            }
            "www.twitch.tv" => get_format_from_res(Host::Twitch, res, ctype),
            "holodex.net" => match get_webpage_url(url.as_ref()) {
                Some(url) => get_format(&url, res, ctype),
                None => panic!("Invalid url: {}", url),
            },
            _ => res.unwrap_custom(),
        },
        None => panic!("Invalid url: {}", url),
    }
}

pub fn get_list_formats(url: &Url) {
    let output = Command::new("yt-dlp")
        .arg(url.as_str())
        .arg("--print")
        .arg("formats_table")
        .arg("--cookies-from-browser")
        .arg("firefox")
        .output()
        .unwrap();
    if output.status.success() {
        let formats_table = String::from_utf8(output.stdout).unwrap();
        println!("{}", formats_table);
    } else {
        eprintln!("Failed to get formats table");
        eprintln!("{}", String::from_utf8(output.stderr).unwrap());
    }
}

#[derive(Debug)]
pub enum Resolution {
    Lowest,
    Low,
    Medium,
    High,
    Highest,
    Custom(String),
}

impl Resolution {
    pub fn from_str(res: &str) -> Self {
        match res {
            "lowest" | "1" => Resolution::Lowest,
            "low" | "2" => Resolution::Low,
            "medium" | "3" => Resolution::Medium,
            "high" | "4" => Resolution::High,
            "highest" | "5" => Resolution::Highest,
            _ => Resolution::Custom(res.to_owned()),
        }
    }

    pub fn unwrap_custom(self) -> String {
        match self {
            Resolution::Custom(res) => res,
            _ => panic!("Resolution is not custom"),
        }
    }
}

pub enum ContentType {
    Live,
    Video,
    #[allow(dead_code)]
    Playlist,
}

pub enum Host {
    Youtube,
    Twitch,
}

pub fn get_format_from_res(host: Host, res: Resolution, ctype: ContentType) -> String {
    match host {
        Host::Youtube => match res {
            Resolution::Lowest => match ctype {
                ContentType::Live => "91".to_owned(),
                ContentType::Video => "160+139".to_owned(),
                ContentType::Playlist => "160+139".to_owned(),
            },
            Resolution::Low => match ctype {
                ContentType::Live => "93".to_owned(),
                ContentType::Video => "133+139".to_owned(),
                ContentType::Playlist => "133+139".to_owned(),
            },
            Resolution::Medium => match ctype {
                ContentType::Live => "94".to_owned(),
                ContentType::Video => "18".to_owned(),
                ContentType::Playlist => "18".to_owned(),
            },
            Resolution::High => match ctype {
                ContentType::Live => "300".to_owned(),
                ContentType::Video => "22".to_owned(),
                ContentType::Playlist => "22".to_owned(),
            },
            Resolution::Highest => match ctype {
                ContentType::Live => "301".to_owned(),
                ContentType::Video => "299+140".to_owned(),
                ContentType::Playlist => "299+140".to_owned(),
            },
            Resolution::Custom(res) => res,
        },
        Host::Twitch => match res {
            Resolution::Lowest => match ctype {
                ContentType::Live => "160p".to_owned(),
                ContentType::Video => "160p".to_owned(),
                ContentType::Playlist => "160p".to_owned(),
            },
            Resolution::Low => match ctype {
                ContentType::Live => "360p".to_owned(),
                ContentType::Video => "360p".to_owned(),
                ContentType::Playlist => "360p".to_owned(),
            },
            Resolution::Medium => match ctype {
                ContentType::Live => "480p".to_owned(),
                ContentType::Video => "480p".to_owned(),
                ContentType::Playlist => "480p".to_owned(),
            },
            Resolution::High => match ctype {
                ContentType::Live => "720p".to_owned(),
                ContentType::Video => "720p".to_owned(),
                ContentType::Playlist => "720p".to_owned(),
            },
            Resolution::Highest => match ctype {
                ContentType::Live => "1080p".to_owned(),
                ContentType::Video => "1080p".to_owned(),
                ContentType::Playlist => "1080p".to_owned(),
            },
            Resolution::Custom(res) => res,
        },
    }
}

pub enum Tool {
    Ytdlp,
    Vlc,
}

impl Tool {
    pub fn from_str(tool: &str) -> Self {
        match tool {
            "yt-dlp" => Tool::Ytdlp,
            "vlc" => Tool::Vlc,
            _ => panic!("Invalid tool"),
        }
    }
}

impl From<Option<&String>> for Tool {
    fn from(tool: Option<&String>) -> Self {
        match tool {
            Some(tool) => Tool::from_str(tool),
            None => Tool::Ytdlp,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    #[test]
    fn test_aliases() {
        let aliases_str = fs::read_to_string("/home/alimulap/void/aliases.toml").unwrap();
        let aliases = toml::from_str::<toml::Value>(&aliases_str).unwrap();
        assert!(aliases.get("sora").is_some());
    }
}
