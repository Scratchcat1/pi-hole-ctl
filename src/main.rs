mod api_util;
mod commands;
mod config;
use crate::api_util::PiHoleConfigImplementation;
use crate::config::HostKeyPair;
use clap::Parser;
use commands::{CnameCommands, Commands, DNSCommands, ListCommands, PiHoleCtlOptions};
use serde::Serialize;
use std::collections::HashMap;

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

        // let serialised_yaml =
        //     serde_yaml::to_string(&map).expect("Unable to serialise results to YAML");
        // println!("{}", serialised_yaml);
    } else {
        for (result, host) in results.zip(hosts.iter()) {
            println!("{}: {:?}", host.as_ref(), result);
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
    let config = config::get_config_file(&opts.config_file_path);
    for HostKeyPair { host, key } in config.hosts {
        opts.hosts.push(host.clone());
        opts.keys.push(key.clone().unwrap_or("".to_string()));

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
        Commands::Summary => {
            let results = apis.iter().map(|api| {
                api.get_unauthenticated_api()
                    .and_then(|unauth_api| unauth_api.get_summary())
                    .map_err(|e| format!("{:?}", e))
            });
            display(results, &opts.hosts, opts.json)
        }

        Commands::SummaryRaw => {
            let results = apis.iter().map(|api| {
                api.get_unauthenticated_api()
                    .and_then(|unauth_api| unauth_api.get_summary_raw())
                    .map_err(|e| format!("{:?}", e))
            });
            display(results, &opts.hosts, opts.json)
        }

        Commands::OverTime10Min => {
            let results = apis.iter().map(|api| {
                api.get_unauthenticated_api()
                    .and_then(|unauth_api| unauth_api.get_over_time_data_10_mins())
                    .map_err(|e| format!("{:?}", e))
            });
            display(results, &opts.hosts, opts.json)
        }

        Commands::Version => {
            let results = apis.iter().map(|api| {
                api.get_unauthenticated_api()
                    .and_then(|unauth_api| unauth_api.get_version())
                    .map_err(|e| format!("{:?}", e))
            });
            display(results, &opts.hosts, opts.json)
        }

        Commands::Versions => {
            let results = apis.iter().map(|api| {
                api.get_unauthenticated_api()
                    .and_then(|unauth_api| unauth_api.get_versions())
                    .map_err(|e| format!("{:?}", e))
            });
            display(results, &opts.hosts, opts.json)
        }

        Commands::TopItems { count } => {
            let results = apis.iter().map(|api| {
                api.get_authenticated_api()
                    .and_then(|auth_api| auth_api.get_top_items(count))
                    .map_err(|e| format!("{:?}", e))
            });
            display(results, &opts.hosts, opts.json)
        }

        Commands::TopClients { count } => {
            let results = apis.iter().map(|api| {
                api.get_authenticated_api()
                    .and_then(|auth_api| auth_api.get_top_clients(count))
                    .map_err(|e| format!("{:?}", e))
            });
            display(results, &opts.hosts, opts.json)
        }

        Commands::TopClientsBlocked { count } => {
            let results = apis.iter().map(|api| {
                api.get_authenticated_api()
                    .and_then(|auth_api| auth_api.get_top_clients_blocked(count))
                    .map_err(|e| format!("{:?}", e))
            });
            display(results, &opts.hosts, opts.json)
        }

        Commands::ForwardDestinations { unsorted } => {
            let results = apis.iter().map(|api| {
                api.get_authenticated_api()
                    .and_then(|auth_api| auth_api.get_forward_destinations(unsorted))
                    .map_err(|e| format!("{:?}", e))
            });
            display(results, &opts.hosts, opts.json)
        }

        Commands::QueryTypes => {
            let results = apis.iter().map(|api| {
                api.get_authenticated_api()
                    .and_then(|auth_api| auth_api.get_query_types())
                    .map_err(|e| format!("{:?}", e))
            });
            display(results, &opts.hosts, opts.json)
        }

        Commands::AllQueries { count } => {
            let results = apis.iter().map(|api| {
                api.get_authenticated_api()
                    .and_then(|auth_api| auth_api.get_all_queries(count))
                    .map_err(|e| format!("{:?}", e))
            });
            display(results, &opts.hosts, opts.json)
        }

        Commands::Cache => {
            let results = apis.iter().map(|api| {
                api.get_authenticated_api()
                    .and_then(|auth_api| auth_api.get_cache_info())
                    .map_err(|e| format!("{:?}", e))
            });
            display(results, &opts.hosts, opts.json)
        }
        Commands::ClientNames => {
            let results = apis.iter().map(|api| {
                api.get_authenticated_api()
                    .and_then(|auth_api| auth_api.get_client_names())
                    .map_err(|e| format!("{:?}", e))
            });
            display(results, &opts.hosts, opts.json)
        }
        Commands::OverTimeDataClients => {
            let results = apis.iter().map(|api| {
                api.get_authenticated_api()
                    .and_then(|auth_api| auth_api.get_over_time_data_clients())
                    .map_err(|e| format!("{:?}", e))
            });
            display(results, &opts.hosts, opts.json)
        }

        Commands::Network => {
            let results = apis.iter().map(|api| {
                api.get_authenticated_api()
                    .and_then(|auth_api| auth_api.get_network())
                    .map_err(|e| format!("{:?}", e))
            });
            display(results, &opts.hosts, opts.json)
        }

        Commands::QueriesCount => {
            let results = apis.iter().map(|api| {
                api.get_authenticated_api()
                    .and_then(|auth_api| auth_api.get_queries_count())
                    .map_err(|e| format!("{:?}", e))
            });
            display(results, &opts.hosts, opts.json)
        }

        Commands::List { list, command } => match command {
            ListCommands::Show => {
                let results = apis.iter().map(|api| {
                    api.get_authenticated_api()
                        .and_then(|auth_api| auth_api.list_get_domains(&list))
                        .map_err(|e| format!("{:?}", e))
                });
                display(results, &opts.hosts, opts.json)
            }
            ListCommands::Add { domain } => {
                let results = apis.iter().map(|api| {
                    api.get_authenticated_api()
                        .and_then(|auth_api| auth_api.list_add(&domain, &list))
                        .map_err(|e| format!("{:?}", e))
                });
                display(results, &opts.hosts, opts.json)
            }
            ListCommands::Remove { domain } => {
                let results = apis.iter().map(|api| {
                    api.get_authenticated_api()
                        .and_then(|auth_api| auth_api.list_remove(&domain, &list))
                        .map_err(|e| format!("{:?}", e))
                });
                display(results, &opts.hosts, opts.json)
            }
        },

        Commands::DNS { command } => match command {
            DNSCommands::Show => {
                let results = apis.iter().map(|api| {
                    api.get_authenticated_api()
                        .and_then(|auth_api| auth_api.get_custom_dns_records())
                        .map_err(|e| format!("{:?}", e))
                });
                display(results, &opts.hosts, opts.json)
            }
            DNSCommands::Add { ip, domain } => {
                let results = apis.iter().map(|api| {
                    api.get_authenticated_api()
                        .and_then(|auth_api| auth_api.add_custom_dns_record(ip, &domain))
                        .map_err(|e| format!("{:?}", e))
                });
                display(results, &opts.hosts, opts.json)
            }
            DNSCommands::Remove { ip, domain } => {
                let results = apis.iter().map(|api| {
                    api.get_authenticated_api()
                        .and_then(|auth_api| auth_api.delete_custom_dns_record(ip, &domain))
                        .map_err(|e| format!("{:?}", e))
                });
                display(results, &opts.hosts, opts.json)
            }
        },

        Commands::Cname { command } => match command {
            CnameCommands::Show => {
                let results = apis.iter().map(|api| {
                    api.get_authenticated_api()
                        .and_then(|auth_api| auth_api.get_custom_cname_records())
                        .map_err(|e| format!("{:?}", e))
                });
                display(results, &opts.hosts, opts.json)
            }
            CnameCommands::Add {
                domain,
                target_domain,
            } => {
                let results = apis.iter().map(|api| {
                    api.get_authenticated_api()
                        .and_then(|auth_api| {
                            auth_api.add_custom_cname_record(&domain, &target_domain)
                        })
                        .map_err(|e| format!("{:?}", e))
                });
                display(results, &opts.hosts, opts.json)
            }
            CnameCommands::Remove {
                domain,
                target_domain,
            } => {
                let results = apis.iter().map(|api| {
                    api.get_authenticated_api()
                        .and_then(|auth_api| {
                            auth_api.delete_custom_cname_record(&domain, &target_domain)
                        })
                        .map_err(|e| format!("{:?}", e))
                });
                display(results, &opts.hosts, opts.json)
            }
        },

        Commands::Logage => {
            let results = apis.iter().map(|api| {
                api.get_authenticated_api()
                    .and_then(|auth_api| auth_api.get_max_logage())
                    .map_err(|e| format!("{:?}", e))
            });
            display(results, &opts.hosts, opts.json)
        }
    }
}
