use clap::ArgMatches;

use crate::{
    config::Config,
    target::{CliTarget, ToCliTarget},
};

pub struct Context {
    pub config: Config,
    pub target: Option<CliTarget>,
    pub format: Option<String>,
    pub interval: Option<u64>,
    pub wait_for_live: bool,
    pub threshold: Option<i64>,
    // pub from: Option<String>,
    // pub to: Option<String>,
    pub print_command: bool,
}

impl Context {
    pub fn new(config: Config, cli: &ArgMatches) -> color_eyre::Result<Self> {
        let target = cli
            .get_one::<String>("target")
            .cloned()
            .map(|mut string| string.to_target());
        let format = cli.get_one::<String>("format").cloned();
        let interval = cli
            .get_one::<String>("interval")
            .cloned()
            .map(|s| s.parse::<u64>())
            .transpose()?;
        let threshold = cli.get_one("threshold").cloned();
        let wait_for_live = if cli.get_flag("wait-for-live") {
            true
        } else {
            config.default_parameters.wait_for_live
        };
        let print_command = cli.get_flag("print-command");

        Ok(Self {
            config,
            target,
            format,
            interval,
            wait_for_live,
            threshold,
            print_command,
        })
    }
}
