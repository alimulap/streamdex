use clap::ArgMatches;

use crate::{
    config::Config,
    target::{Target, ToTarget},
};

pub struct Context {
    pub config: Config,
    pub target: Target,
    pub format: Option<String>,
    pub interval: Option<u64>,
    pub _wait_for_live: bool,
    // pub from: Option<String>,
    // pub to: Option<String>,
    pub print_command: bool,
}

impl Context {
    pub fn new(config: Config, cli: &ArgMatches) -> anyhow::Result<Self> {
        let target = cli
            .get_one::<String>("target")
            .cloned()
            .expect("required by clap")
            .to_target();
        let format = cli.get_one::<String>("format").cloned();
        let interval = cli
            .get_one::<String>("interval")
            .cloned()
            .map(|s| s.parse::<u64>())
            .transpose()?;
        let wait_for_live = cli.get_flag("wait-for-live");
        let print_command = cli.get_flag("print-command");

        Ok(Self {
            config,
            target,
            format,
            interval,
            _wait_for_live: wait_for_live,
            print_command,
        })
    }
}
