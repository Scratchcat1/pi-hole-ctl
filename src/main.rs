mod api_util;
mod config;
use crate::api_util::PiHoleConfigImplementation;
use crate::config::HostKeyPair;
use clap::{Parser, Subcommand};
use pi_hole_api::{
    AuthenticatedPiHoleAPI, PiHoleAPIConfig, PiHoleAPIConfigWithKey, UnauthenticatedPiHoleAPI,
};
use serde::Serialize;
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::Duration;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct PiHoleCtlOptions {
    /// print help message
    #[clap(short, long)]
    help: bool,

    /// Be verbose
    #[clap(short, long)]
    verbose: bool,

    /// Output as JSON
    #[clap(short, long)]
    json: bool,

    /// Pairs of hosts/keys
    #[clap(long)]
    hosts: Vec<String>,

    /// Path to config file
    #[clap(short, long)]
    config_file_path: Option<PathBuf>,

    #[clap(long)]
    keys: Vec<String>,

    #[clap(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Enable,
    Disable {
        #[clap(parse(try_from_str = parse_duration), default_value = "60s")]
        duration: Duration,
    },
}

fn parse_duration(arg: &str) -> Result<std::time::Duration, humantime::DurationError> {
    arg.parse::<humantime::Duration>().map(Into::into)
}

fn display<T, I, H>(results: I, hosts: &[H], json: bool)
where
    I: Iterator<Item = T>,
    // I::Item: T,
    T: std::fmt::Debug + Serialize,
    H: AsRef<str>,
{
    if json {
        let map: HashMap<String, T> = hosts
            .iter()
            .zip(results)
            .map(|(host, result)| (host.as_ref().to_owned(), result))
            .collect();
        let serialised_json =
            serde_json::to_string_pretty(&map).expect("Unable to serialise results to JSON");
        println!("{}", serialised_json);
    } else {
        for (result, host) in results.zip(hosts.iter()) {
            println!("{}: {:?}", host.as_ref(), result);
        }
    }
}

fn main() {
    // Parse the command line options
    let mut opts = PiHoleCtlOptions::parse();
    println!("{:#?}", opts);

    // Load config and extend hosts and keys
    let config = config::get_config_file(&opts.config_file_path);
    for HostKeyPair { host, key } in config.hosts {
        opts.hosts.push(host);
        opts.keys.push(key.unwrap_or("".to_string()));
    }

    let apis: Vec<PiHoleConfigImplementation> = opts
        .hosts
        .iter()
        .zip(opts.keys.iter())
        .map(|(host, key)| {
            let api_key = if key.len() > 10 {
                Some(key.clone())
            } else {
                None
            };
            PiHoleConfigImplementation::new(host.clone(), api_key)
        })
        .collect();

    match opts.command {
        Commands::Enable => {
            let results = apis.iter().map(|api| {
                api.get_authenticated_api()
                    .and_then(|auth_api| auth_api.enable())
                    .map_err(|e| format!("{:?}", e))
            });
            display(results, &opts.hosts, opts.json)
        }
        Commands::Disable { duration } => {
            let results = apis.iter().map(|api| {
                api.get_authenticated_api()
                    .and_then(|auth_api| auth_api.disable(duration.as_secs()))
                    .map_err(|e| format!("{:?}", e))
            });
            display(results, &opts.hosts, opts.json)
        }
    }
}
