use clap::{Arg, ArgAction, ArgMatches, Command};

pub fn parse() -> ArgMatches {
    Command::new("streamdex")
        .args(&[
            Arg::new("target")
                .index(1)
                .required(true)
                .help("URL or alias of the live stream or video"),
            Arg::new("format")
                .short('f')
                .long("format")
                .help("Format of the target video"),
            Arg::new("wait-for-live")
                .short('w')
                .long("wait-for-live")
                .action(ArgAction::SetTrue)
                .required(false)
                .help("Wait for the live stream to go live if it's not currently live"),
            Arg::new("interval")
                .index(2)
                .required_if_eq("wait-for-live", "true")
                .required(false)
                .help("Interval in seconds to check for new updates"),
            Arg::new("youtube")
                .long("youtube")
                .action(ArgAction::SetTrue)
                .help("Only wait for youtube live when alias has more than just youtube link"),
            Arg::new("twitch")
                .long("twitch")
                .action(ArgAction::SetTrue)
                .help("Only wait for twitch live when alias has more than just twitch link"),
            Arg::new("print-command")
                .short('p')
                .long("print-format")
                .action(ArgAction::SetTrue)
                .help("Print yt-dlp and vlc command to stdout"),
        ])
        .get_matches()
    // .subcommand_required(true)
    // .subcommand(
    //     Command::new("live").about("Watch a live stream").args(&[
    //         Arg::new("url").index(1).required(true).help("url or alias"),
    //         Arg::new("quality")
    //             .short('q')
    //             .long("res")
    //             .default_value("2")
    //             .required(false)
    //             .help("quality of the live stream | can also be format"),
    //         Arg::new("interval")
    //             .short('i')
    //             .long("interval")
    //             .default_value("30")
    //             .required(false)
    //             .help("interval in minutes to check for new video"),
    //         // Arg::new("print-command")
    //         //     .long("print-command")
    //         //     .short('p')
    //         //     .action(ArgAction::SetTrue)
    //     ]),
    // )
    // // .subcommand(
    // //     Command::new("video")
    // //         .about("Watch a video")
    // //         .args(&[
    // //             Arg::new("url").index(1).required(true).help("url or alias"),
    // //             Arg::new("resolution")
    // //                 .short('r')
    // //                 .long("res")
    // //                 .default_value("medium")
    // //                 .required(false)
    // //                 .help("resolution(or more like format)"),
    // //             Arg::new("tool")
    // //                 .short('t')
    // //                 .long("tool")
    // //                 .value_parser(["yt-dlp", "vlc"])
    // //                 .default_value("yt-dlp")
    // //                 .required(false)
    // //                 .conflicts_with("room")
    // //                 .help("tool"),
    // //             Arg::new("room")
    // //                 .long("room")
    // //                 .required(false)
    // //                 .conflicts_with("tool")
    // //                 .help("using an allocated room(supply with room id) instead of opening a new vlc instance"),
    // //             Arg::new("wait-for-video")
    // //                 .short('w')
    // //                 .long("wait-for-video")
    // //                 .num_args(0..=1)
    // //                 .action(ArgAction::Set)
    // //                 .required(false)
    // //                 .help("wait for video to be available"),
    // //             Arg::new("from")
    // //                 .long("from")
    // //                 .required(false)
    // //                 .help("start from a specific time"),
    // //             Arg::new("to")
    // //                 .long("to")
    // //                 .required(false)
    // //                 .help("end at a specific time")
    // //         ])
    // // )
    // .subcommand(
    //     Command::new("watch").about("Watch a video").args(&[
    //         Arg::new("url").index(1).required(true).help("url or alias"),
    //         Arg::new("quality")
    //             .short('r')
    //             .long("res")
    //             .default_value("medium")
    //             .required(false)
    //             .help("Quality of the video | can also be format"),
    //         Arg::new("tool")
    //             .short('t')
    //             .long("tool")
    //             .value_parser(["yt-dlp", "vlc"])
    //             .default_value("yt-dlp")
    //             .required(false)
    //             // .conflicts_with("room")
    //             .help("tool"),
    //         // Arg::new("room")
    //         //     .long("room")
    //         //     .required(false)
    //         //     .conflicts_with("tool")
    //         //     .help("using an allocated room(supply with room id) instead of opening a new vlc instance"),
    //         // Arg::new("wait-for-video")
    //         //     .short('w')
    //         //     .long("wait-for-video")
    //         //     .num_args(0..=1)
    //         //     .action(ArgAction::Set)
    //         //     .required(false)
    //         //     .help("wait for video to be available"),
    //         Arg::new("from")
    //             .long("from")
    //             .required(false)
    //             .help("start from a specific time"),
    //         Arg::new("to")
    //             .long("to")
    //             .required(false)
    //             .help("end at a specific time"),
    //     ]),
    // )
    // // .subcommand(
    // //     Command::new("playlist")
    // //         .about("Watch a playlist")
    // //         .args(&[
    // //             Arg::new("url").index(1).required(true).help("url for playlist"),
    // //             Arg::new("resolution")
    // //                 .short('r')
    // //                 .long("res")
    // //                 .default_value("92")
    // //                 .required(false)
    // //                 .help("resolution(or more like format)"),
    // //         ])
    // // )
    // // .subcommand(
    // //     Command::new("allocate")
    // //     .about("Allocate a room foa a streamer")
    // //     .args(&[
    // //         Arg::new("id")
    // //             .index(1)
    // //             .required(true)
    // //             .help("room id"),
    // //         Arg::new("delay")
    // //             .short('d')
    // //             .long("delay")
    // //             .default_value("10")
    // //             .required(false)
    // //             .help("delay in seconds")
    // //     ])
    // // )
    // .subcommand(
    //     Command::new("print-formats")
    //         .about("Print available formats for a live stream or a video")
    //         .args(&[Arg::new("id").index(1).required(true).help("room id")]),
    // )
    // .get_matches()
}
