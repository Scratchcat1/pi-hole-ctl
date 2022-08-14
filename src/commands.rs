use clap::{Parser, Subcommand};
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

    /// Path to config file
    #[clap(short, long)]
    pub config_file_path: Option<PathBuf>,

    /// API key for a pihole instance. Anything with a length < 10 is considered no key.
    #[clap(long)]
    pub keys: Vec<String>,

    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
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
