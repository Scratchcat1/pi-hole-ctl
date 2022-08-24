use crate::api_type_wrappers::*;
use crate::api_util::{APIResult, CallApi, PiHoleConfigImplementation};
use crate::table::{ToTableTitle, ToTableTitleDynamic};
use clap::{Parser, Subcommand};
use cli_table::CellStruct;
use pi_hole_api::api_types::*;
use pi_hole_api::errors::APIError;
use std::net::IpAddr;
use std::path::PathBuf;
use std::time::Duration;
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct PiHoleCtlOptions {
    /// print help message
    #[clap(short, long)]
    pub help: bool,

    /// Be verbose
    #[clap(short, long)]
    pub verbose: bool,

    /// Output as JSON
    #[clap(short, long)]
    pub json: bool,

    /// Hosts to manage
    #[clap(long)]
    pub hosts: Vec<String>,

    /// API key for a pihole instance. Anything with a length < 10 is considered no key.
    #[clap(long)]
    pub keys: Vec<String>,

    /// Path to config file
    #[clap(short, long)]
    pub config_file_path: Option<PathBuf>,

    /// Named groups to use from the config file
    #[clap(short, long)]
    pub groups: Vec<String>,

    #[clap(subcommand)]
    pub command: ApiCommands,
}

// #[derive(Debug, Subcommand)]
// pub enum Commands {
//     /// Perform API request
//     API {
//         #[clap(subcommand)]
//         command: ApiCommands,
//     },
// }

#[derive(Debug, Subcommand)]
pub enum ApiCommands {
    /// Enable ad blocking
    Enable,
    /// Disable ad blocking
    Disable {
        /// Duration to block for e.g. 60s, 1m, 2h
        #[clap(parse(try_from_str = parse_duration), default_value = "60s")]
        duration: Duration,
    },
    /// Get summary information
    Summary,
    /// Get raw (numerical) summary information
    SummaryRaw,
    /// Number of queries, binned into 10 minute blocks
    OverTime10Min,
    /// Simple PiHole Version
    Version,
    /// Versions of core, FTL and web and if updates are available
    Versions,
    /// Most queries items
    TopItems {
        /// Limit number of results
        count: Option<u32>,
    },
    /// Clients with the most queries
    TopClients {
        /// Limit number of results
        count: Option<u32>,
    },
    /// Clients with the most blocked queries
    TopClientsBlocked {
        /// Limit number of results
        count: Option<u32>,
    },
    /// Percentage of queries forwarded to each target
    ForwardDestinations {
        /// Optionally sort the output
        #[clap(default_value_t = false)]
        unsorted: bool,
    },
    /// Number of queries per type
    QueryTypes,
    /// DNS query data
    AllQueries {
        /// Limit number of results
        #[clap(default_value_t = 100)]
        count: u32,
    },
    /// Cache statistics
    Cache,
    /// Hostname and IP for clients
    ClientNames,
    /// Get queries over time by client
    OverTimeDataClients,
    /// Network clients
    Network,
    /// Total number of queries
    QueriesCount,
    /// Show/Modify a black/whitelist
    List {
        /// List to use
        /// Acceptable lists are: `white`, `black`, `white_regex`, `black_regex`, `white_wild`, `black_wild`, `audit`.
        list: String,

        #[clap(subcommand)]
        command: ListCommands,
    },
    /// Custom DNS records
    Dns {
        #[clap(subcommand)]
        command: DnsCommands,
    },
    /// Custom DNS records
    Cname {
        #[clap(subcommand)]
        command: CnameCommands,
    },
    /// Logage info
    Logage,
}

impl CallApi for ApiCommands {
    fn call(&self, api: &PiHoleConfigImplementation) -> Result<APIResult, APIError> {
        match self {
            Self::Enable => api.get_authenticated_api()?.enable().map(|a| a.into()),
            Self::Disable { duration } => api
                .get_authenticated_api()?
                .disable(duration.as_secs())
                .map(|a| a.into()),
            Self::Summary => api
                .get_unauthenticated_api()
                .get_summary()
                .map(|a| a.into()),

            Self::SummaryRaw => api
                .get_unauthenticated_api()
                .get_summary_raw()
                .map(|a| a.into()),

            Self::OverTime10Min => api
                .get_unauthenticated_api()
                .get_over_time_data_10_mins()
                .map(|a| a.into()),

            Self::Version => api
                .get_unauthenticated_api()
                .get_version()
                .map(VersionWrapper)
                .map(|a| a.into()),
            Self::Versions => api
                .get_unauthenticated_api()
                .get_versions()
                .map(|a| a.into()),
            Self::TopItems { count } => api
                .get_authenticated_api()?
                .get_top_items(count)
                .map(|a| a.into()),
            Self::TopClients { count } => api
                .get_authenticated_api()?
                .get_top_clients(count)
                .map(|a| a.into()),
            Self::TopClientsBlocked { count } => api
                .get_authenticated_api()?
                .get_top_clients_blocked(*count)
                .map(|a| a.into()),
            Self::ForwardDestinations { unsorted } => api
                .get_authenticated_api()?
                .get_forward_destinations(*unsorted)
                .map(|a| a.into()),
            Self::QueryTypes => api
                .get_authenticated_api()?
                .get_query_types()
                .map(|a| a.into()),
            Self::AllQueries { count } => api
                .get_authenticated_api()?
                .get_all_queries(*count)
                .map(|a| a.into()),
            Self::Cache => api
                .get_authenticated_api()?
                .get_cache_info()
                .map(|a| a.into()),
            Self::ClientNames => api
                .get_authenticated_api()?
                .get_client_names()
                .map(|a| a.into()),
            Self::OverTimeDataClients => api
                .get_authenticated_api()?
                .get_over_time_data_clients()
                .map(OverTimeDataClientsWrapper)
                .map(|a| a.into()),
            Self::Network => api.get_authenticated_api()?.get_network().map(|a| a.into()),
            Self::QueriesCount => api
                .get_authenticated_api()?
                .get_queries_count()
                .map(QueriesCountWrapper)
                .map(|a| a.into()),
            Self::List { list, command } => match command {
                ListCommands::Show => api
                    .get_authenticated_api()?
                    .list_get_domains(list)
                    .map(|a| a.into()),
                ListCommands::Add { domain } => api
                    .get_authenticated_api()?
                    .list_add(domain, list)
                    .map(|a| a.into()),
                ListCommands::Remove { domain } => api
                    .get_authenticated_api()?
                    .list_remove(domain, list)
                    .map(|a| a.into()),
            },
            Self::Dns { command } => match command {
                DnsCommands::Show => api
                    .get_authenticated_api()?
                    .get_custom_dns_records()
                    .map(|a| a.into()),
                DnsCommands::Add { ip, domain } => api
                    .get_authenticated_api()?
                    .add_custom_dns_record(ip, domain)
                    .map(|a| a.into()),
                DnsCommands::Remove { ip, domain } => api
                    .get_authenticated_api()?
                    .delete_custom_dns_record(ip, domain)
                    .map(|a| a.into()),
            },
            Self::Cname { command } => match command {
                CnameCommands::Show => api
                    .get_authenticated_api()?
                    .get_custom_cname_records()
                    .map(|a| a.into()),
                CnameCommands::Add {
                    domain,
                    target_domain,
                } => api
                    .get_authenticated_api()?
                    .add_custom_cname_record(domain, target_domain)
                    .map(|a| a.into()),
                CnameCommands::Remove {
                    domain,
                    target_domain,
                } => api
                    .get_authenticated_api()?
                    .delete_custom_cname_record(domain, target_domain)
                    .map(|a| a.into()),
            },
            Self::Logage => api
                .get_authenticated_api()?
                .get_max_logage()
                .map(LogageWrapper)
                .map(|a| a.into()),
        }
    }
}

impl ToTableTitleDynamic for ApiCommands {
    fn to_table_title(&self) -> Vec<CellStruct> {
        match self {
            Self::Enable => Status::to_table_title(),
            Self::Disable { duration: _ } => Status::to_table_title(),
            Self::Summary => Summary::to_table_title(),
            Self::SummaryRaw => SummaryRaw::to_table_title(),
            Self::OverTime10Min => OverTimeData::to_table_title(),
            Self::Version => VersionWrapper::to_table_title(),
            Self::Versions => Versions::to_table_title(),
            Self::TopItems { count: _ } => TopItems::to_table_title(),
            Self::TopClients { count: _ } => TopClients::to_table_title(),
            Self::TopClientsBlocked { count: _ } => TopClientsBlocked::to_table_title(),
            Self::ForwardDestinations { unsorted: _ } => ForwardDestinations::to_table_title(),
            Self::QueryTypes => QueryTypes::to_table_title(),
            Self::AllQueries { count: _ } => AllQueries::to_table_title(),
            Self::Cache => CacheInfo::to_table_title(),
            Self::ClientNames => ClientName::to_table_title(),
            Self::OverTimeDataClients => OverTimeDataClientsWrapper::to_table_title(),
            Self::Network => Network::to_table_title(),
            Self::QueriesCount => QueriesCountWrapper::to_table_title(),
            Self::List { list: _, command } => match command {
                ListCommands::Show => CustomListDomainDetails::to_table_title(),
                ListCommands::Add { domain: _ } => ListModificationResponse::to_table_title(),
                ListCommands::Remove { domain: _ } => ListModificationResponse::to_table_title(),
            },
            Self::Dns { command } => match command {
                DnsCommands::Show => CustomDNSRecord::to_table_title(),
                DnsCommands::Add { ip: _, domain: _ } => ListModificationResponse::to_table_title(),
                DnsCommands::Remove { ip: _, domain: _ } => {
                    ListModificationResponse::to_table_title()
                }
            },
            Self::Cname { command } => match command {
                CnameCommands::Show => CustomCNAMERecord::to_table_title(),
                CnameCommands::Add {
                    domain: _,
                    target_domain: _,
                } => ListModificationResponse::to_table_title(),
                CnameCommands::Remove {
                    domain: _,
                    target_domain: _,
                } => ListModificationResponse::to_table_title(),
            },
            Self::Logage => LogageWrapper::to_table_title(),
        }
    }
}

#[derive(Debug, Subcommand)]
pub enum ListCommands {
    Show,
    Add {
        /// Domain to add
        domain: String,
    },
    Remove {
        /// Domain to remove
        domain: String,
    },
}

#[derive(Debug, Subcommand)]
pub enum DnsCommands {
    Show,
    Add {
        /// IP address
        ip: IpAddr,
        /// Domain to associate
        domain: String,
    },
    Remove {
        /// IP address
        ip: IpAddr,
        /// Domain to associate
        domain: String,
    },
}

#[derive(Debug, Subcommand)]
pub enum CnameCommands {
    Show,
    Add {
        /// Domain
        domain: String,
        /// Target domain to associate
        target_domain: String,
    },
    Remove {
        /// Domain
        domain: String,
        /// Target domain to associate
        target_domain: String,
    },
}

fn parse_duration(arg: &str) -> Result<std::time::Duration, humantime::DurationError> {
    arg.parse::<humantime::Duration>().map(Into::into)
}
