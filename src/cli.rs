use clap::{Arg, ArgAction, ArgMatches, Command};

pub fn parse() -> ArgMatches {
    Command::new("streamdex")
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
                        .num_args(0..=1)
                        .action(ArgAction::Set)
                        .required(false)
                        .help("wait for video to be available"),
                    Arg::new("print-command")
                        .long("print-command")
                        .short('p')
                        .action(ArgAction::SetTrue)
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
                        .num_args(0..=1)
                        .action(ArgAction::Set)
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
        .get_matches()
}
