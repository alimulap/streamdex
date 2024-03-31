use std::process::{Command, Stdio};

use url::Url;

use crate::{
    runner,
    utils::{self, ContentType, FromAlias, Resolution, Tool}, context::Context,
};

pub fn live(ctx: &Context) {
    println!("Watching a live stream");
    let maybe_url_or_alias = ctx.get("url").unwrap().as_string().unwrap();
    let url = match Url::parse(maybe_url_or_alias) {
        Ok(url) => url,
        Err(e) => {
            println!("Can't parse to a url, checking for alias");
            match Url::from_alias(ctx, maybe_url_or_alias) {
                Some(url) => url,
                None => {
                    if e == url::ParseError::RelativeUrlWithoutBase {
                        println!("Not an alias, attempting to parse to a url by adding https://");
                        match Url::parse(&format!("https://{}", maybe_url_or_alias)) {
                            Ok(url) => url,
                            Err(_) => panic!("Invalid url nor alias: {}", maybe_url_or_alias),
                        }
                    } else {
                        panic!("Invalid url nor alias: {}", maybe_url_or_alias);
                    }
                }
            }
        }
    };
    println!("Url: {}", url);
    let res = Resolution::from_str(ctx.get("resolution").unwrap().as_string().unwrap());
    println!("Resolution: {:?}", res);
    let format = utils::get_format(&url, res, ContentType::Live);
    let tool = ctx.get("tool").unwrap().as_string().unwrap();
    let room = ctx.get("room").unwrap().as_option_string().unwrap();
    let wait_for_video = ctx.get("wait-for-video").unwrap().as_boolean().unwrap();
    match utils::Tool::from_str(tool) {
        Tool::Ytdlp => runner::with_ytdlp(url.to_string(), format, room, *wait_for_video, None),
        Tool::Vlc => runner::only_vlc(url.to_string(), format, room),
    }
}

pub fn video(ctx: &Context) {
    println!("Watching a video");
    let url = ctx.get("url").unwrap().as_string().unwrap();
    let url = match Url::parse(url) {
        Ok(url) => url,
        Err(e) => {
            if e == url::ParseError::RelativeUrlWithoutBase {
                println!("Cannot parse to a url, attempting to parse to a url by adding https://");
                match Url::parse(&format!("https://{}", url)) {
                    Ok(url) => url,
                    Err(_) => panic!("Invalid url: {}", url),
                }
            } else {
                panic!("Invalid url: {}", url);
            }
        }
    };
    println!("Url: {}", url);
    let res = Resolution::from_str(ctx.get("resolution").unwrap().as_string().unwrap());
    println!("Resolution: {:?}", res);
    let format = utils::get_format(&url, res, ContentType::Video);
    let tool = ctx.get("tool").unwrap().as_string().unwrap();
    let room = ctx.get("room").unwrap().as_option_string().unwrap();
    let wait_for_video = ctx.get("wait-for-video").unwrap().as_boolean().unwrap();
    let from = ctx.get("from").unwrap().as_option_string().unwrap();
    let to = ctx.get("to").unwrap().as_option_string().unwrap();
    let range = match (from, to) {
        (Some(from), Some(to)) => Some(format!("*{}-{}", from, to)),
        (Some(from), None) => Some(format!("*{}-inf", from)),
        (None, Some(to)) => Some(format!("*0:0-{}", to)),
        (None, None) => None,
    };
    match utils::Tool::from_str(tool) {
        Tool::Ytdlp => runner::with_ytdlp(url.to_string(), format, room, *wait_for_video, range),
        Tool::Vlc => {
            if range.is_some() {
                panic!("Range is not supported with vlc(i think..)");
            }
            runner::only_vlc(url.to_string(), format, room)
        },
    }
}

#[allow(dead_code)]
pub fn playlist(_ctx: &Context, url: &str, res: &str) {
    println!("Playing a playlist");
    let _vlc = Command::new("vlc")
        .arg("");
    let mut ytdlp = Command::new("yt-dlp")
        .arg(url)
        .arg("-f")
        .arg(res)
        .arg("-q")
        .arg("--cookies-from-browser")
        .arg("edge")
        .arg("--mark-watched")
        .arg("--flat-playlist")
        .arg("-j")
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let jq = Command::new("jq")
        .arg("-r")
        .arg(".url")
        .stdin(Stdio::from(ytdlp.stdout.take().unwrap()))
        .stdout(Stdio::piped())
        .output()
        .unwrap();
    if let Ok(urls) = String::from_utf8(jq.stdout) {
        let urls = urls
            .lines()
            .map(|url| url.replace("music.", ""))
            .collect::<Vec<_>>();
        println!("{:?}", urls);
        let vlc = Command::new("vlc")
            .args(urls)
            .arg("-L")
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();
        let output = vlc.wait_with_output().unwrap();
        let result = String::from_utf8(output.stdout).unwrap();
        println!("{}", result);
    }
    ytdlp.wait().unwrap();
}

pub fn print_formats(ctx: &Context) {
    let maybe_url_or_alias = ctx.get("id").unwrap().as_string().unwrap();
    let url = match Url::parse(maybe_url_or_alias) {
        Ok(url) => url,
        Err(e) => {
            println!("Can't parse to a url, checking for alias");
            match Url::from_alias(ctx, maybe_url_or_alias) {
                Some(url) => url,
                None => {
                    if e == url::ParseError::RelativeUrlWithoutBase {
                        println!("Not an alias, attempting to parse to a url by adding https://");
                        match Url::parse(&format!("https://{}", maybe_url_or_alias)) {
                            Ok(url) => url,
                            Err(_) => panic!("Invalid url nor alias: {}", maybe_url_or_alias),
                        }
                    } else {
                        panic!("Invalid url nor alias: {}", maybe_url_or_alias);
                    }
                }
            }
        }
    };
    println!("Url: {}", url);
    utils::get_list_formats(&url);
}
