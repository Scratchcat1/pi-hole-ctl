mod api_type_wrappers;
mod api_util;
mod commands;
mod config;
mod output;
mod table;
use crate::api_util::{CallApi, PiHoleConfigImplementation};
use crate::table::ToTableTitleDynamic;
use clap::Parser;
use commands::PiHoleCtlOptions;
use std::collections::HashSet;

fn main() {
    // Parse the command line options
    let mut opts = PiHoleCtlOptions::parse();
    if opts.verbose {
        println!("{:#?}", opts);
    }

    // Throw an error if the hosts and keys are not the same length
    if opts.hosts.len() != opts.keys.len() {
        panic!(
            "Hosts and keys lengths do not match ({} != {})",
            opts.hosts.len(),
            opts.keys.len()
        );
    }

    // Load config and extend hosts and keys
    let config = config::get_config_file(&opts.config_file_path, opts.verbose);
    let mut included_hosts = HashSet::new();

    // Select the group named "default" if no group is explicitly provided
    let default_groups = vec!["default".to_string()];
    let selected_groups = if opts.groups.is_empty() {
        &default_groups
    } else {
        &opts.groups
    };
    if opts.verbose {
        println!("Selected groups: {:?}", selected_groups);
    }

    for group in selected_groups {
        let named_hosts = config
            .groups
            .get(group)
            .expect(&format!("Group '{}' not found", group));

        for named_host in named_hosts {
            // Only add each host once
            if included_hosts.contains(named_host) {
                continue;
            }
            included_hosts.insert(named_host);

            let host_key_pair = config.hosts.get(named_host).expect(&format!(
                "Named host {} not found for group {}",
                named_host, group
            ));

            opts.hosts.push(host_key_pair.host.clone());
            opts.keys
                .push(host_key_pair.key.clone().unwrap_or_default());

            if opts.verbose {
                println!(
                    "Adding host: {} with key: {:?}",
                    host_key_pair.host, host_key_pair.key
                );
            }
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

    // Call the API and output the results
    let results = apis.iter().map(|api| opts.command.call(api));
    let title = opts.command.to_table_title();
    crate::output::display(results, title, &opts.hosts, opts.json);
}
