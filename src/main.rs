mod api_type_wrappers;
mod api_util;
mod commands;
mod config;
mod table;
use crate::api_type_wrappers::*;
use crate::api_util::PiHoleConfigImplementation;
use crate::config::HostKeyPair;
use crate::table::{ToTable, ToTableRows, ToTableTitle};
use clap::Parser;
use commands::{CnameCommands, Commands, DnsCommands, ListCommands, PiHoleCtlOptions};
use pi_hole_api::errors::APIError;
use serde::Serialize;
use std::collections::HashMap;

fn display<I, H, R>(results: I, hosts: &[H], json: bool)
where
    I: Iterator<Item = Result<R, APIError>>,
    // I::Item: T,
    // T: Result<R, APIError>,
    R: std::fmt::Debug + Serialize + ToTableRows + ToTableTitle,
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
        let errors = hosts
            .iter()
            .zip(&results)
            .filter_map(|(host, result)| match result {
                Ok(_) => None,
                Err(e) => Some((host, e)),
            })
            .map(|(host, error)| format!("{}: {:?}", host.as_ref(), error))
            .collect::<Vec<String>>();
        let table_rows = hosts
            .iter()
            .zip(results)
            .filter_map(|(host, result)| match result {
                Ok(ok) => Some((host.as_ref().to_owned(), ok)),
                Err(_) => None,
            })
            .collect::<Vec<(String, R)>>();
        println!("{}", table_rows.to_table().display().unwrap());

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
    let config = config::get_config_file(&opts.config_file_path);
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
        Commands::Enable => {
            let results = apis.iter().map(|api| {
                api.get_authenticated_api()
                    .and_then(|auth_api| auth_api.enable())
            });
            display(results, &opts.hosts, opts.json)
        }
        Commands::Disable { duration } => {
            let results = apis.iter().map(|api| {
                api.get_authenticated_api()
                    .and_then(|auth_api| auth_api.disable(duration.as_secs()))
            });
            display(results, &opts.hosts, opts.json)
        }
        Commands::Summary => {
            let results = apis.iter().map(|api| {
                api.get_unauthenticated_api()
                    .and_then(|unauth_api| unauth_api.get_summary())
            });
            display(results, &opts.hosts, opts.json)
        }

        Commands::SummaryRaw => {
            let results = apis.iter().map(|api| {
                api.get_unauthenticated_api()
                    .and_then(|unauth_api| unauth_api.get_summary_raw())
            });
            display(results, &opts.hosts, opts.json)
        }

        Commands::OverTime10Min => {
            let results = apis.iter().map(|api| {
                api.get_unauthenticated_api()
                    .and_then(|unauth_api| unauth_api.get_over_time_data_10_mins())
            });
            display(results, &opts.hosts, opts.json)
        }

        Commands::Version => {
            let results = apis.iter().map(|api| {
                api.get_unauthenticated_api()
                    .and_then(|unauth_api| unauth_api.get_version())
                    .map(VersionWrapper)
            });
            display(results, &opts.hosts, opts.json)
        }

        Commands::Versions => {
            let results = apis.iter().map(|api| {
                api.get_unauthenticated_api()
                    .and_then(|unauth_api| unauth_api.get_versions())
            });
            display(results, &opts.hosts, opts.json)
        }

        Commands::TopItems { count } => {
            let results = apis.iter().map(|api| {
                api.get_authenticated_api()
                    .and_then(|auth_api| auth_api.get_top_items(count))
            });
            display(results, &opts.hosts, opts.json)
        }

        Commands::TopClients { count } => {
            let results = apis.iter().map(|api| {
                api.get_authenticated_api()
                    .and_then(|auth_api| auth_api.get_top_clients(count))
            });
            display(results, &opts.hosts, opts.json)
        }

        Commands::TopClientsBlocked { count } => {
            let results = apis.iter().map(|api| {
                api.get_authenticated_api()
                    .and_then(|auth_api| auth_api.get_top_clients_blocked(count))
            });
            display(results, &opts.hosts, opts.json)
        }

        Commands::ForwardDestinations { unsorted } => {
            let results = apis.iter().map(|api| {
                api.get_authenticated_api()
                    .and_then(|auth_api| auth_api.get_forward_destinations(*unsorted))
            });
            display(results, &opts.hosts, opts.json)
        }

        Commands::QueryTypes => {
            let results = apis.iter().map(|api| {
                api.get_authenticated_api()
                    .and_then(|auth_api| auth_api.get_query_types())
            });
            display(results, &opts.hosts, opts.json)
        }

        Commands::AllQueries { count } => {
            let results = apis.iter().map(|api| {
                api.get_authenticated_api()
                    .and_then(|auth_api| auth_api.get_all_queries(*count))
            });
            display(results, &opts.hosts, opts.json)
        }

        Commands::Cache => {
            let results = apis.iter().map(|api| {
                api.get_authenticated_api()
                    .and_then(|auth_api| auth_api.get_cache_info())
            });
            display(results, &opts.hosts, opts.json)
        }
        Commands::ClientNames => {
            let results = apis.iter().map(|api| {
                api.get_authenticated_api()
                    .and_then(|auth_api| auth_api.get_client_names())
            });
            display(results, &opts.hosts, opts.json)
        }
        Commands::OverTimeDataClients => {
            let results = apis.iter().map(|api| {
                api.get_authenticated_api()
                    .and_then(|auth_api| auth_api.get_over_time_data_clients())
                    .map(|over_time_data_clients| {
                        OverTimeDataClientsWrapper(over_time_data_clients)
                    })
            });
            display(results, &opts.hosts, opts.json)
        }

        Commands::Network => {
            let results = apis.iter().map(|api| {
                api.get_authenticated_api()
                    .and_then(|auth_api| auth_api.get_network())
            });
            display(results, &opts.hosts, opts.json)
        }

        Commands::QueriesCount => {
            let results = apis.iter().map(|api| {
                api.get_authenticated_api()
                    .and_then(|auth_api| auth_api.get_queries_count())
                    .map(QueriesCountWrapper)
            });
            display(results, &opts.hosts, opts.json)
        }

        Commands::List { list, command } => match command {
            ListCommands::Show => {
                let results = apis.iter().map(|api| {
                    api.get_authenticated_api()
                        .and_then(|auth_api| auth_api.list_get_domains(&list))
                });
                display(results, &opts.hosts, opts.json)
            }
            ListCommands::Add { domain } => {
                let results = apis.iter().map(|api| {
                    api.get_authenticated_api()
                        .and_then(|auth_api| auth_api.list_add(&domain, &list))
                });
                display(results, &opts.hosts, opts.json)
            }
            ListCommands::Remove { domain } => {
                let results = apis.iter().map(|api| {
                    api.get_authenticated_api()
                        .and_then(|auth_api| auth_api.list_remove(&domain, &list))
                });
                display(results, &opts.hosts, opts.json)
            }
        },

        Commands::Dns { command } => match command {
            DnsCommands::Show => {
                let results = apis.iter().map(|api| {
                    api.get_authenticated_api()
                        .and_then(|auth_api| auth_api.get_custom_dns_records())
                });
                display(results, &opts.hosts, opts.json)
            }
            DnsCommands::Add { ip, domain } => {
                let results = apis.iter().map(|api| {
                    api.get_authenticated_api()
                        .and_then(|auth_api| auth_api.add_custom_dns_record(ip, &domain))
                });
                display(results, &opts.hosts, opts.json)
            }
            DnsCommands::Remove { ip, domain } => {
                let results = apis.iter().map(|api| {
                    api.get_authenticated_api()
                        .and_then(|auth_api| auth_api.delete_custom_dns_record(ip, &domain))
                });
                display(results, &opts.hosts, opts.json)
            }
        },

        Commands::Cname { command } => match command {
            CnameCommands::Show => {
                let results = apis.iter().map(|api| {
                    api.get_authenticated_api()
                        .and_then(|auth_api| auth_api.get_custom_cname_records())
                });
                display(results, &opts.hosts, opts.json)
            }
            CnameCommands::Add {
                domain,
                target_domain,
            } => {
                let results = apis.iter().map(|api| {
                    api.get_authenticated_api().and_then(|auth_api| {
                        auth_api.add_custom_cname_record(&domain, &target_domain)
                    })
                });
                display(results, &opts.hosts, opts.json)
            }
            CnameCommands::Remove {
                domain,
                target_domain,
            } => {
                let results = apis.iter().map(|api| {
                    api.get_authenticated_api().and_then(|auth_api| {
                        auth_api.delete_custom_cname_record(&domain, &target_domain)
                    })
                });
                display(results, &opts.hosts, opts.json)
            }
        },

        Commands::Logage => {
            let results = apis.iter().map(|api| {
                api.get_authenticated_api()
                    .and_then(|auth_api| auth_api.get_max_logage())
                    .map(LogageWrapper)
            });
            display(results, &opts.hosts, opts.json)
        }
    }
}
