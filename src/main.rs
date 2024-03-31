use std::fs;

use clap::{Arg, ArgAction, Command};
use config::Config;
use context::{Context, ContextValue};

mod config;
mod context;
mod handler;
mod room;
mod runner;
mod utils;

fn main() {
    let matches = Command::new("streamdex")
        .subcommand_required(true)
        .subcommand(
            Command::new("live")
                .about("Watch a live stream")
                .args(&[
                    Arg::new("url").index(1).required(true).help("url or alias"),
                    Arg::new("resolution")
                        .short('r')
                        .long("res")
                        .default_value("92")
                        .required(false)
                        .help("resolution(or more like format)"),
                    Arg::new("tool")
                        .short('t')
                        .long("tool")
                        .value_parser(["yt-dlp", "vlc"])
                        .default_value("yt-dlp")
                        .required(false)
                        .help("tool"),
                    Arg::new("room")
                        .long("room")
                        .required(false)
                        .help("using an allocated room(supply with room id) instead of opening a new vlc instance"),
                    Arg::new("wait-for-video")
                        .short('w')
                        .long("wait-for-video")
                        .action(ArgAction::SetTrue)
                        .required(false)
                        .help("wait for video to be available")
                ])
        )
        .subcommand(
            Command::new("video")
                .about("Watch a video")
                .args(&[
                    Arg::new("url").index(1).required(true).help("url or alias"),
                    Arg::new("resolution")
                        .short('r')
                        .long("res")
                        .default_value("medium")
                        .required(false)
                        .help("resolution(or more like format)"),
                    Arg::new("tool")
                        .short('t')
                        .long("tool")
                        .value_parser(["yt-dlp", "vlc"])
                        .default_value("yt-dlp")
                        .required(false)
                        .conflicts_with("room")
                        .help("tool"),
                    Arg::new("room")
                        .long("room")
                        .required(false)
                        .conflicts_with("tool")
                        .help("using an allocated room(supply with room id) instead of opening a new vlc instance"),
                    Arg::new("wait-for-video")
                        .short('w')
                        .long("wait-for-video")
                        .action(ArgAction::SetTrue)
                        .required(false)
                        .help("wait for video to be available"),
                    Arg::new("from")
                        .long("from")
                        .required(false)
                        .help("start from a specific time"),
                    Arg::new("to")
                        .long("to")
                        .required(false)
                        .help("end at a specific time")
                ])
        )
        .subcommand(
            Command::new("playlist")
                .about("Watch a playlist")
                .args(&[
                    Arg::new("url").index(1).required(true).help("url for playlist"),
                    Arg::new("resolution")
                        .short('r')
                        .long("res")
                        .default_value("92")
                        .required(false)
                        .help("resolution(or more like format)"),
                ])
        )
        .subcommand(
            Command::new("allocate")
            .about("Allocate a room foa a streamer")
            .args(&[
                Arg::new("id")
                    .index(1)
                    .required(true)
                    .help("room id"),
                Arg::new("delay")
                    .short('d')
                    .long("delay")
                    .default_value("10")
                    .required(false)
                    .help("delay in seconds")
            ])
        )
        .subcommand(
            Command::new("print-formats")
            .about("Print available formats for a live stream or a video")
            .args(&[
                Arg::new("id")
                    .index(1)
                    .required(true)
                    .help("room id")
            ])
        )
        .get_matches();

    let mut context = Context::new();
    let config_str =
        fs::read_to_string("/home/alimulap/.config/streamdex/config.toml").unwrap();
    let config = toml::from_str::<Config>(&config_str).unwrap();
    context.insert("config", ContextValue::Config(config));
    match matches.subcommand() {
        Some(("live", sub_m)) => {
            context.insert(
                "url",
                ContextValue::String(sub_m.get_one::<String>("url").unwrap()),
            );
            context.insert(
                "resolution",
                ContextValue::String(sub_m.get_one::<String>("resolution").unwrap()),
            );
            context.insert(
                "tool",
                ContextValue::String(sub_m.get_one::<String>("tool").unwrap()),
            );
            context.insert(
                "room",
                ContextValue::OptionString(sub_m.get_one::<String>("room")),
            );
            context.insert(
                "wait-for-video",
                ContextValue::Boolean(sub_m.get_one::<bool>("wait-for-video").unwrap_or(&false)),
            );
            handler::live(&context);
        }
        Some(("video", sub_m)) => {
            context.insert(
                "url",
                ContextValue::String(sub_m.get_one::<String>("url").unwrap()),
            );
            context.insert(
                "resolution",
                ContextValue::String(sub_m.get_one::<String>("resolution").unwrap()),
            );
            context.insert(
                "tool",
                ContextValue::String(sub_m.get_one::<String>("tool").unwrap()),
            );
            context.insert(
                "room",
                ContextValue::OptionString(sub_m.get_one::<String>("room")),
            );
            context.insert(
                "wait-for-video",
                ContextValue::Boolean(sub_m.get_one::<bool>("wait-for-video").unwrap_or(&false)),
            );
            context.insert(
                "from",
                ContextValue::OptionString(sub_m.get_one::<String>("from")),
            );
            context.insert(
                "to",
                ContextValue::OptionString(sub_m.get_one::<String>("to")),
            );
            handler::video(&context);
        }
        Some(("playlist", _sub_m)) => {
            eprintln!("Not stable yet");
            //let url = sub_m.get_one::<String>("url").expect("No url given");
            //let res = sub_m.get_one::<String>("resolution").cloned().unwrap_or(String::from("92"));
            //handler::playlist(url, &res);
        }
        Some(("allocate", sub_m)) => {
            eprintln!("Can only allocate 1 room, i thought i can use 1 port multiple times lmao");
            context.insert(
                "id",
                ContextValue::String(sub_m.get_one::<String>("id").unwrap()),
            );
            let delay = sub_m
                .get_one::<String>("delay")
                .unwrap()
                .parse::<u32>()
                .expect("delay must be a positive integer");
            context.insert("delay", ContextValue::U32(&delay));
            room::allocate(&context);
        }
        Some(("print-formats", sub_m)) => {
            let id = sub_m.get_one::<String>("id").expect("No id given");
            context.insert("id", ContextValue::String(&id));
            handler::print_formats(&context);
        }
        _ => println!("No command given"),
    }
}
