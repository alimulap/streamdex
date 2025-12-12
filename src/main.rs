use std::fs;
use std::io::stdout;

use crossterm::ExecutableCommand;
use crossterm::event::{DisableMouseCapture, EnableMouseCapture};

use crate::config::Config;
use crate::context::Context;
use crate::runner::watch_with_ytdlp_and_vlc;
use crate::target::{Aliases, CliTarget, PlatformFlags};
use crate::tui::Tui;
use crate::twitch::{Twitch, get_twitch_username};
use crate::utils::extract_youtube_id_from_url;
use crate::youtube::YouTube;

mod cli;
mod config;
mod context;
mod error;
mod event;
mod handler;
mod runner;
mod target;
mod tui;
mod twitch;
mod utils;
mod youtube;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let _ = rustls::crypto::aws_lc_rs::default_provider().install_default();

    let cli = cli::parse();
    let config = Config::get();
    let ctx = Context::new(config, &cli)?;

    let tui = cli.get_flag("tui");

    if tui {
        let terminal = ratatui::init();
        stdout().execute(EnableMouseCapture)?;
        let mut tui = Tui::new(&ctx)?;
        tui.run(terminal).await?;
        stdout().execute(DisableMouseCapture)?;
        ratatui::restore();
    } else {
        let youtube = YouTube::new_youtube_client(&ctx.config).await?;

        let twitch = Twitch::new(&ctx.config).await?;

        let target = ctx.target.as_ref().expect("Required by clap");

        match target {
            CliTarget::Url(url) => {
                if let Some(_video_id) = extract_youtube_id_from_url(&url) {
                    // let videos = youtube.get_videos_details(vec![video_id]).await?;
                    // let video = videos
                    //     .first()
                    //     .ok_or(anyhow::anyhow!(
                    //         "Should get one video from extracted youtube video id"
                    //     ))?
                    //     .clone();
                    let format = ctx
                        .format
                        .clone()
                        .unwrap_or(ctx.config.default_parameters.format.youtube);
                    let print_command = ctx.print_command;
                    watch_with_ytdlp_and_vlc(url.to_string(), format, None, print_command)?;
                }
            }
            CliTarget::YoutubeChannelHandle(handle) => {
                youtube.handle_channel(&handle, &ctx).await?;
            }
            CliTarget::MaubeAlias(alias) => {
                let aliases_string =
                    fs::read_to_string(&ctx.config.new_aliases).unwrap_or("{}".to_string());
                let aliases = toml::from_str::<Aliases>(&aliases_string).unwrap_or_default();

                let links = aliases.get(alias);

                if let Some(links) = links {
                    let platform_flags = PlatformFlags::from_cli(&cli);

                    if let Some(youtube_link) = &links.youtube {
                        if platform_flags.is_all() || platform_flags.youtube {
                            youtube.handle_channel(&youtube_link.username, &ctx).await?;
                        }
                    }
                    if let Some(twitch_link) = &links.twitch {
                        if platform_flags.is_all() || platform_flags.twitch {
                            if let Some(username) =
                                get_twitch_username(&twitch_link.url.to_string())
                            {
                                twitch.handle_streamer(username, &ctx).await?;
                            }
                        }
                    }
                } else {
                    println!("No alias found for: {}", alias);
                }
            }
        }
    }

    Ok(())
}
