use std::time::Duration;

use chrono::Utc;
use google_youtube3::api::Video;
use twitch_api::helix::streams::StreamType;

use crate::context::Context;
use crate::error::{Error, FetchData};
use crate::runner::watch_with_ytdlp_and_vlc;
use crate::twitch::Twitch;
use crate::youtube::{LiveStatus, YouTube};

impl YouTube {
    pub async fn check_youtube_live(&self, handle: &str, ctx: &Context) -> anyhow::Result<()> {
        let channel_id = self.get_channel_id(handle, &ctx).await?;

        let ids = self.get_live_ids(&channel_id, LiveStatus::Live).await?;

        if !ids.is_empty() {
            let videos = self.get_videos_details(ids).await?;
            let live_stream = self.get_one_that_actually_live(&videos)?;

            if let Some(video) = live_stream {
                self.handle_live(handle, video, ctx)?;
            }
        } else {
            let ids = self.get_live_ids(&channel_id, LiveStatus::Upcoming).await?;
            if ids.is_empty() {
                println!(
                    "Channel {} is not live and has no upcoming streams.",
                    handle
                );
                return Ok(());
            }

            let videos = self.get_videos_details(ids).await?;

            if let Some(live) = self.get_one_that_actually_live(&videos)? {
                self.handle_live(handle, live, ctx)?;
            }

            println!("Currently no live stream for youtube channel {handle}/{channel_id}");

            let upcoming_videos = self.get_ones_that_actually_upcoming(&videos)?;
            self.handle_upcoming(handle, &upcoming_videos, ctx).await?;
        }
        Ok(())
    }
    pub fn handle_live(
        &self,
        _channel_handle: &str,
        video: Video,
        ctx: &Context,
    ) -> anyhow::Result<()> {
        let video_id = video.id.ok_or(Error::NoDataFound(FetchData::VideoID))?;
        let format = ctx
            .format
            .clone()
            .unwrap_or(ctx.config.default_parameters.format.youtube.clone());
        let print_format = ctx.print_command;

        watch_with_ytdlp_and_vlc(
            format!("https://www.youtube.com/watch?v={video_id}"),
            format,
            None,
            print_format,
        )?;

        Ok(())
    }

    pub async fn handle_upcoming(
        &self,
        channel_handle: &str,
        videos: &[Video],
        ctx: &Context,
    ) -> Result<(), Error> {
        let closest = self.choose_closest_to_start(&videos);

        let threshold = ctx
            .threshold
            .unwrap_or(ctx.config.default_parameters.threshold);

        if let Some((video, start_time)) = closest {
            let video_id = video
                .id
                .as_ref()
                .ok_or(Error::NoDataFound(FetchData::VideoID))?;
            let title = video
                .snippet
                .as_ref()
                .and_then(|s| s.title.as_ref())
                .ok_or(Error::NoDataFound(FetchData::Snippet))?;
            let minutes_left = start_time.signed_duration_since(Utc::now()).num_minutes();
            let hours_left: f64 = minutes_left as f64 / 60.0;

            if minutes_left < threshold {
                self.handle_live(channel_handle, video.clone(), ctx)?;
                return Ok(());
            }

            println!("Closest stream to start is {video_id}");
            println!("- with title\t: {title}");

            if hours_left > 1.0 {
                println!("- starting in\t: {hours_left:.1} hours");
            } else {
                println!("- starting in\t: {minutes_left} minutes");
            }
        }

        let interval = ctx
            .interval
            .unwrap_or(ctx.config.default_parameters.interval);
        let channel_id = self.get_channel_id(channel_handle, ctx).await?;

        let mut first_loop = true;

        loop {
            if !first_loop {
                let live_ids = self.get_live_ids(&channel_id, LiveStatus::Live).await?;
                if !live_ids.is_empty() {
                    let videos = self.get_videos_details(live_ids).await?;
                    if let Some(live) = self.get_one_that_actually_live(&videos)? {
                        self.handle_live(channel_handle, live, ctx)?;
                        break;
                    }
                }

                let upcoming_ids = self.get_live_ids(&channel_id, LiveStatus::Upcoming).await?;
                if !upcoming_ids.is_empty() {
                    let videos = self.get_videos_details(upcoming_ids).await?;
                    let actually_upcoming = self.get_ones_that_actually_upcoming(&videos)?;
                    if let Some((video, start_time)) =
                        self.choose_closest_to_start(&actually_upcoming)
                    {
                        let minutes_left =
                            start_time.signed_duration_since(Utc::now()).num_minutes();
                        if minutes_left < threshold {
                            self.handle_live(channel_handle, video, ctx)?;
                            break;
                        }
                    }
                }
            }

            println!("Waiting {interval} minutes until fetching new data...");

            tokio::time::sleep(Duration::from_mins(interval)).await;

            first_loop = false;
        }

        Ok(())
    }
}

impl<'a> Twitch<'a> {
    pub async fn handle_streamer(&self, username: &str, ctx: &Context) -> anyhow::Result<()> {
        let streams = self.get_streams(username).await?;

        if !streams.is_empty() {
            for stream in &streams {
                if stream.type_ == StreamType::Live {
                    self.handle_live(username, ctx).await?;
                    break;
                }
            }
        } else {
            println!("Twtich account {username} is not currently live.");
            self.handle_wait_stream(username, ctx).await?;
        }

        Ok(())
    }

    pub async fn handle_live(&self, username: &str, ctx: &Context) -> anyhow::Result<()> {
        let url = format!("https://www.twitch.tv/{username}");

        println!("Watching {url}...");

        let format = ctx
            .format
            .clone()
            .unwrap_or(ctx.config.default_parameters.format.youtube.clone());
        let print_format = ctx.print_command;

        watch_with_ytdlp_and_vlc(url, format, None, print_format)?;
        Ok(())
    }

    pub async fn handle_wait_stream(&self, username: &str, ctx: &Context) -> anyhow::Result<()> {
        println!("Will wait until {username} start a live stream");

        let interval = ctx
            .interval
            .unwrap_or(ctx.config.default_parameters.interval);

        'outer: loop {
            println!("Waiting {interval} minutes until fetching new data...");
            tokio::time::sleep(Duration::from_mins(interval)).await;

            let streams = self.get_streams(username).await?;

            for stream in streams {
                if stream.type_ == StreamType::Live {
                    self.handle_live(username, ctx).await?;
                    break 'outer;
                }
            }
        }
        Ok(())
    }
}
