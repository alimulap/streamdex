use std::fs;

use twitch_api::{
    TwitchClient,
    helix::streams::{GetStreamsRequest, Stream},
    twitch_oauth2::{AccessToken, AppAccessToken, ClientSecret},
    types::{Collection, UserName},
};

use crate::config::Config;

pub struct Twitch<'a> {
    pub client: TwitchClient<'a, reqwest::Client>,
    pub token: AppAccessToken,
}

impl<'a> Twitch<'a> {
    pub async fn new(config: &Config) -> anyhow::Result<Self> {
        let twitch_client_secret = fs::read_to_string(&config.twitch_client_secret)?;
        let twitch_access_token = fs::read_to_string(&config.twitch_access_token)?
            .trim()
            .to_string();

        let client = TwitchClient::<reqwest::Client>::new();
        let token = AppAccessToken::from_existing(
            &client,
            AccessToken::new(twitch_access_token),
            None,
            ClientSecret::new(twitch_client_secret),
        )
        .await
        .inspect_err(|e| eprintln!("The frigg {e}"))?;

        Ok(Self { client, token })
    }

    pub async fn get_streams(&self, username: &str) -> anyhow::Result<Vec<Stream>> {
        let usernames = Collection::from(vec![UserName::new(username.to_string())]);

        let req = GetStreamsRequest::user_logins(usernames);

        let response = self
            .client
            .helix
            .req_get(req, &self.token)
            .await
            .inspect_err(|e| println!("The frig {e}"))?;

        return Ok(response.data);
    }
}

pub fn get_twitch_username(url: &str) -> Option<&str> {
    url.trim_end_matches('/')
        .strip_prefix("https://www.twitch.tv/")
        .or_else(|| {
            url.trim_end_matches('/')
                .strip_prefix("http://www.twitch.tv/")
        })
        .or_else(|| url.trim_end_matches('/').strip_prefix("https://twitch.tv/"))
        .or_else(|| url.trim_end_matches('/').strip_prefix("http://twitch.tv/"))
        .and_then(|s| {
            // Extract only the username part (before any path or query params)
            s.split('/')
                .next()
                .and_then(|username| username.split('?').next())
                .filter(|u| !u.is_empty())
        })
}
