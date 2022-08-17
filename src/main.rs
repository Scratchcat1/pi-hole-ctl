mod api_type_wrappers;
mod api_util;
mod commands;
mod config;
mod table;
use crate::api_util::{CallApi, PiHoleConfigImplementation};
use crate::config::HostKeyPair;
use crate::table::{ToTableRows, ToTableTitleDynamic};
use clap::Parser;
use cli_table::{CellStruct, Table};
use commands::{Commands, PiHoleCtlOptions};
use pi_hole_api::errors::APIError;
use serde::Serialize;
use std::collections::HashMap;

fn display<I, H, R>(results: I, title: Vec<CellStruct>, hosts: &[H], json: bool)
where
    I: Iterator<Item = Result<R, APIError>>,
    R: std::fmt::Debug + Serialize + ToTableRows,
    H: AsRef<str>,
{
    if json {
        let map: HashMap<String, Result<R, String>> = hosts
            .iter()
            .zip(results)
            .map(|(host, result)| {
                (
                    host.as_ref().to_owned(),
                    result.map_err(|e| format!("{:?}", e)),
                )
            })
            .collect();
        let serialised_json =
            serde_json::to_string_pretty(&map).expect("Unable to serialise results to JSON");
        println!("{}", serialised_json);

        // let serialised_yaml =
        //     serde_yaml::to_string(&map).expect("Unable to serialise results to YAML");
        // println!("{}", serialised_yaml);
    } else {
        let results: Vec<Result<R, APIError>> = results.collect();
        // Separate out the errors
        let errors = hosts
            .iter()
            .zip(&results)
            .filter_map(|(host, result)| match result {
                Ok(_) => None,
                Err(e) => Some((host, e)),
            })
            .map(|(host, error)| format!("{}: {:?}", host.as_ref(), error))
            .collect::<Vec<String>>();

        // Construct table rows from hosts and associated results
        let table_rows: Vec<Vec<CellStruct>> = hosts
            .iter()
            .zip(results)
            .filter_map(|(host, result)| match result {
                Ok(ok) => Some((host, ok)),
                Err(_) => None,
            })
            .flat_map(|(host, response_data)| response_data.to_table_rows(host.as_ref()))
            .collect();
        let table = table_rows.table().title(title);
        println!("{}", table.display().unwrap());

        if !errors.is_empty() {
            println!("Errors:");
            for error in errors {
                println!("{}", error);
            }
        }
    }
}

fn main() {
    // Parse the command line options
    let mut opts = PiHoleCtlOptions::parse();
    if opts.verbose {
        println!("{:#?}", opts);
    }

    // Load config and extend hosts and keys
    let config = config::get_config_file(&opts.config_file_path, opts.verbose);
    for HostKeyPair { host, key } in config.hosts {
        opts.hosts.push(host.clone());
        opts.keys.push(key.clone().unwrap_or_default());

        if opts.verbose {
            println!("Adding host: {host} with key: {key:?}");
        }
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

    match &opts.command {
        Commands::API { command } => {
            let results = apis.iter().map(|api| command.call(api));
            let title = command.to_table_title();
            display(results, title, &opts.hosts, opts.json);
        }
    }
}
