use std::collections::HashMap;
use std::fs;

use chrono::{DateTime, Duration, Utc};
use google_youtube3::api::Video;
use google_youtube3::hyper_util::client::legacy::connect::HttpConnector;
use google_youtube3::yup_oauth2::{
    InstalledFlowAuthenticator, InstalledFlowReturnMethod, read_application_secret,
};
use google_youtube3::{
    common::NoToken,
    hyper_util::{self},
};
use hyper_rustls::HttpsConnector;

use crate::config::Config;
use crate::context::Context;
use crate::error::{Error, FetchData};

pub struct YouTube {
    hub: google_youtube3::YouTube<HttpsConnector<HttpConnector>>,
    _have_auth: bool,
}

pub enum LiveStatus {
    Live,
    Upcoming,
    #[allow(unused)]
    Completed,
}

impl YouTube {
    pub fn new(
        hub: google_youtube3::YouTube<HttpsConnector<HttpConnector>>,
        have_auth: bool,
    ) -> Self {
        Self {
            hub,
            _have_auth: have_auth,
        }
    }

    pub async fn new_youtube_client(config: &Config) -> color_eyre::Result<Self> {
        let auth = match read_application_secret(&config.client_secret).await {
            Ok(secret) => {
                let a = InstalledFlowAuthenticator::builder(
                    secret,
                    InstalledFlowReturnMethod::Interactive,
                )
                .persist_tokens_to_disk(&config.presist_token)
                .build()
                .await?;
                Some(a)
            }
            Err(err) => {
                println!(
                    "No OAuth client secret found at clientsecret.json; proceeding with API-key only NoToken. Error: {}",
                    err
                );
                None
            }
        };

        let client =
            hyper_util::client::legacy::Client::builder(hyper_util::rt::TokioExecutor::new())
                .build(
                    hyper_rustls::HttpsConnectorBuilder::new()
                        .with_native_roots()
                        .unwrap()
                        .https_or_http()
                        .enable_http1()
                        .build(),
                );
        let have_auth = auth.is_some();
        if let Some(auth) = auth {
            Ok(YouTube::new(
                google_youtube3::YouTube::new(client, auth),
                have_auth,
            ))
        } else {
            Ok(YouTube::new(
                google_youtube3::YouTube::new(client, NoToken),
                have_auth,
            ))
        }
    }

    pub async fn get_live_ids(
        &self,
        channel_id: &str,
        live_status: LiveStatus,
    ) -> Result<Vec<String>, Error> {
        // if self.have_auth {
        //     let (_, res) = self
        //         .hub
        //         .live_broadcasts()
        //         .list(&vec!["snippet".into()])
        //         .broadcast_status(match live_status {
        //             LiveStatus::Live => "active",
        //             LiveStatus::Upcoming => "upcoming",
        //             LiveStatus::Completed => "completed",
        //         })
        //         .param("channelId", channel_id)
        //         .doit()
        //         .await
        //         .inspect_err(|e| eprintln!("Can't fetch live broadcast {e}"))?;
        //     Ok(res
        //         .items
        //         .unwrap_or_default()
        //         .into_iter()
        //         .map(|ls| ls.id)
        //         .flatten()
        //         .collect())
        // } else {
        let (_, search_results) = self
            .hub
            .search()
            .list(&vec!["snippet".into()])
            .channel_id(channel_id)
            .event_type(match live_status {
                LiveStatus::Live => "live",
                LiveStatus::Upcoming => "upcoming",
                LiveStatus::Completed => "completed",
            })
            .add_type("video")
            .doit()
            .await
            .map_err(|e| Error::YTFetchLiveFailed(channel_id.to_string(), e))?;

        search_results
            .items
            .and_then(|searches| {
                searches
                    .into_iter()
                    .map(|s| s.id.and_then(|id| id.video_id))
                    .collect()
            })
            .ok_or(Error::NoBroadcast)
        // }
    }

    pub async fn get_videos_details(&self, ids: Vec<String>) -> Result<Vec<Video>, Error> {
        let mut videolistcall = self
            .hub
            .videos()
            .list(&vec!["snippet".into(), "liveStreamingDetails".into()])
            .param("key", "AIzaSyAyE_ojLL2NQpysLdupl_750_hv-ivvH3Y");

        for id in ids {
            videolistcall = videolistcall.add_id(&id);
        }

        let (_, video_details) = videolistcall
            .doit()
            .await
            .map_err(|e| Error::YTFailFetchVideoDetail(e))?;

        Ok(video_details.items.unwrap_or_default())
    }

    pub async fn get_channel_id(&self, handle: &str, ctx: &Context) -> color_eyre::Result<String> {
        let mut saved_ids = toml::from_str::<HashMap<String, String>>(&fs::read_to_string(
            &ctx.config.saved_yt_channel_ids,
        )?)?;
        if let Some(channel_id) = saved_ids.get(handle) {
            return Ok(channel_id.clone());
        }

        let (_, channel_list) = self
            .hub
            .channels()
            .list(&vec!["id".into()])
            .for_handle(handle)
            .param("key", "AIzaSyAyE_ojLL2NQpysLdupl_750_hv-ivvH3Y")
            .doit()
            .await
            .inspect_err(|e| eprintln!("Can't get channel ID: {e}"))?;

        if let Some(channel) = channel_list.items.and_then(|mut items| items.pop()) {
            let channel_id = channel.id.ok_or(Error::NoDataFound(FetchData::ChannelID))?;

            saved_ids.insert(handle.to_string(), channel_id.clone());
            fs::write(
                &ctx.config.saved_yt_channel_ids,
                toml::to_string(&saved_ids)?,
            )?;

            return Ok(channel_id);
        } else {
            Err(Error::NoChannelFound(handle.to_string()).into())
        }
    }

    pub fn get_one_that_actually_live(&self, videos: &[Video]) -> Result<Option<Video>, Error> {
        for video in videos {
            let live_status = video
                .snippet
                .as_ref()
                .and_then(|s| s.live_broadcast_content.as_ref())
                .ok_or(Error::NoDataFound(FetchData::Snippet))?;
            let has_actual_start_time = video
                .live_streaming_details
                .as_ref()
                .map(|live_detail| live_detail.actual_start_time.is_some())
                .ok_or(Error::NoDataFound(FetchData::LiveDetail))?;

            if live_status == "live" && has_actual_start_time {
                return Ok(Some(video.clone()));
            }
        }

        return Ok(None);
    }

    pub fn get_ones_that_actually_upcoming(&self, videos: &[Video]) -> Result<Vec<Video>, Error> {
        let mut upcoming_videos = Vec::new();
        for video in videos {
            let live_status = video
                .snippet
                .as_ref()
                .and_then(|s| s.live_broadcast_content.as_ref())
                .ok_or(Error::NoDataFound(FetchData::Snippet))?;
            let has_actual_start_time = video
                .live_streaming_details
                .as_ref()
                .map(|live_detail| live_detail.actual_start_time.is_some())
                .ok_or(Error::NoDataFound(FetchData::LiveDetail))?;

            if live_status == "upcoming" && !has_actual_start_time {
                upcoming_videos.push(video.clone());
            }
        }
        return Ok(upcoming_videos);
    }

    pub fn choose_closest_to_start(
        &self,
        videos: &[Video],
        threshold: i64,
    ) -> Option<(Video, DateTime<Utc>)> {
        let now = Utc::now();
        let cutoff = Duration::hours(24);
        let threshold = Duration::minutes(threshold);

        let mut scheduled: Vec<(&Video, DateTime<Utc>)> = Vec::new();
        for v in videos.iter() {
            if let Some(dt) = v
                .live_streaming_details
                .as_ref()
                .and_then(|d| d.scheduled_start_time.clone())
            {
                scheduled.push((v, dt));
            }
        }

        if scheduled.is_empty() {
            return None;
        }

        let (recent, very_old): (Vec<(&Video, DateTime<Utc>)>, Vec<(&Video, DateTime<Utc>)>) =
            scheduled
                .into_iter()
                .partition(|(_, dt)| *dt >= now - cutoff);

        if !very_old.is_empty() {
            if let Some((_, dt)) = very_old.iter().max_by_key(|(_, dt)| *dt) {
                let hours = (now.signed_duration_since(*dt)).num_hours();
                println!(
                    "There's a scheduled stream, but it's way past {} hours and probably not worth waiting",
                    hours
                );
            }
        }

        if recent.is_empty() {
            return None;
        }

        let on_threshold = recent
            .iter()
            .filter(|(_, dt)| *dt >= now - threshold)
            .collect::<Vec<&(&Video, DateTime<Utc>)>>();

        if let Some((vid, dt)) = on_threshold.iter().max_by_key(|(_, dt)| *dt) {
            return Some(((*vid).clone(), *dt));
        }

        let mut future: Vec<(&Video, DateTime<Utc>)> = recent
            .iter()
            .cloned()
            .filter(|(_, dt)| *dt >= now)
            .collect();

        if !future.is_empty() {
            future.sort_by_key(|(_, dt)| *dt);
            let soonest = &future[0];

            let maybe_past = recent
                .iter()
                .cloned()
                .filter(|(_, dt)| *dt < now)
                .max_by_key(|(_, dt)| *dt);

            if let Some((_, dt)) = maybe_past {
                println!("Choosing closest upcoming video at {:?}", soonest.1);
                println!("But there is also a past video at {:?}", dt);
            }

            return Some(((*soonest.0).clone(), soonest.1));
        }

        if let Some((v, dt)) = recent.into_iter().max_by_key(|(_, dt)| *dt) {
            return Some(((*v).clone(), dt));
        }

        None
    }
}

#[rustfmt::skip]
fn _get_youtube_channel_id(url: &str) -> Option<&str> {
    url.trim_end_matches('/')
        .strip_prefix("https://www.youtube.com/channel/")
        .or_else(|| url.trim_end_matches('/').strip_prefix("http://www.youtube.com/channel/"))
        .or_else(|| url.trim_end_matches('/').strip_prefix("https://youtube.com/channel/"))
        .or_else(|| url.trim_end_matches('/').strip_prefix("http://youtube.com/channel/"))
        .and_then(|s| {
            // Extract only the channel ID (before any path or query params)
            s.split('/').next()
                .and_then(|id| id.split('?').next())
                .filter(|id| !id.is_empty())
        })
}

#[rustfmt::skip]
pub fn get_youtube_channel_handle(url: &str) -> Option<&str> {
    url.trim_end_matches('/')
        .strip_prefix("https://www.youtube.com/@")
        .or_else(|| url.trim_end_matches('/').strip_prefix("http://www.youtube.com/@"))
        .or_else(|| url.trim_end_matches('/').strip_prefix("https://youtube.com/@"))
        .or_else(|| url.trim_end_matches('/').strip_prefix("http://youtube.com/@"))
        .and_then(|s| {
            // Extract only the handle (before any path or query params)
            s.split('/').next()
                .and_then(|handle| handle.split('?').next())
                .filter(|h| !h.is_empty())
        })
}
