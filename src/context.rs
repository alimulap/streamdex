use clap::ArgMatches;

use crate::config::Config;

#[derive(Default)]
pub struct Context {
    pub config: Config,
    pub subcommand: Option<String>,
    pub url: Option<String>,
    pub resolution: Option<String>,
    pub tool: Option<String>,
    pub room: Option<String>,
    pub wait_for_video: WaitForVideo,
    pub from: Option<String>,
    pub to: Option<String>,
    pub id: Option<String>,
    pub delay: Option<u32>,
    pub print_command: Option<bool>,
}

impl Context {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            ..Default::default()
        }
    }

    pub fn handle(&mut self, subcommand: &str, sub_m: &ArgMatches) {
        self.subcommand = Some(subcommand.to_string());
        match subcommand {
            "live" => {
                self.url = sub_m.get_one("url").cloned();
                self.resolution = sub_m.get_one("resolution").cloned();
                self.tool = sub_m.get_one("tool").cloned();
                self.room = sub_m.get_one("room").cloned();
                self.print_command = sub_m.get_one("print-command").cloned();
                self.wait_for_video = if sub_m.contains_id("wait-for-video") {
                    match sub_m.get_one::<String>("wait-for-video") {
                        Some(range) => WaitForVideo::Wait(range.clone()),
                        None => WaitForVideo::wait_with_default(),
                    }
                } else {
                    WaitForVideo::NoWait
                };
            }
            "video" => {
                self.url = sub_m.get_one("url").cloned();
                self.resolution = sub_m.get_one("resolution").cloned();
                self.tool = sub_m.get_one("tool").cloned();
                self.room = sub_m.get_one("room").cloned();
                self.from = sub_m.get_one("from").cloned();
                self.to = sub_m.get_one("to").cloned();
                self.print_command = sub_m.get_one("print-command").cloned();
                self.wait_for_video = if sub_m.contains_id("wait-for-video") {
                    match sub_m.get_one::<String>("wait-for-video") {
                        Some(range) => WaitForVideo::Wait(range.clone()),
                        None => WaitForVideo::wait_with_default(),
                    }
                } else {
                    WaitForVideo::NoWait
                };
            }
            "allocate" => {
                self.id = sub_m.get_one("id").cloned();
                self.delay = sub_m.get_one("delay").cloned();
            }
            "print-formats" => {
                self.id = sub_m.get_one("id").cloned();
            }
            _ => {
                eprintln!("Invalid subcommand");
            }
        }
    }

    fn subcommand(&self) -> &str {
        self.subcommand.as_ref().expect("Subcommand not set")
    }

    pub fn url(&self) -> String {
        match self.subcommand() {
            "live" => self.url.clone().unwrap(),
            "video" => self.id.clone().unwrap(),
            _ => panic!("Invalid subcommand"),
        }
    }

    pub fn resolution(&self) -> String {
        match self.subcommand() {
            "live" => self.resolution.clone().unwrap(),
            "video" => self.resolution.clone().unwrap(),
            _ => panic!("Invalid subcommand"),
        }
    }

    pub fn tool(&self) -> String {
        match self.subcommand() {
            "live" => self.tool.clone().unwrap(),
            "video" => self.tool.clone().unwrap(),
            _ => panic!("Invalid subcommand"),
        }
    }

    pub fn room(&self) -> Option<String> {
        match self.subcommand() {
            "live" => self.room.clone(),
            "video" => self.room.clone(),
            _ => panic!("Invalid subcommand"),
        }
    }

    pub fn wait_for_video(&self) -> WaitForVideo {
        match self.subcommand() {
            "live" | "video" => self.wait_for_video.clone(),
            _ => panic!("Invalid subcommand"),
        }
    }

    pub fn from(&self) -> Option<String> {
        match self.subcommand() {
            "live" => self.from.clone(),
            "video" => self.from.clone(),
            _ => panic!("Invalid subcommand"),
        }
    }

    pub fn to(&self) -> Option<String> {
        match self.subcommand() {
            "live" => self.to.clone(),
            "video" => self.to.clone(),
            _ => panic!("Invalid subcommand"),
        }
    }
}

#[derive(Clone)]
pub enum WaitForVideo {
    Wait(String),
    NoWait,
}

impl WaitForVideo {
    fn wait_with_default() -> Self {
        Self::Wait("5".into())
    }
}

impl Default for WaitForVideo {
    fn default() -> Self {
        Self::NoWait
    }
}
