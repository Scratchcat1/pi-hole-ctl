# piholectl
Command line tool to manage Pi-Hole instances

## Usage

```
$ piholectl help
piholectl 0.2.0
Command line tool to manage Pi-Hole instances

USAGE:
    piholectl [OPTIONS] <SUBCOMMAND>

OPTIONS:
    -c, --config-file-path <CONFIG_FILE_PATH>
            Path to config file

    -g, --groups <GROUPS>
            Named groups to use from the config file

    -h, --help
            print help message

        --hosts <HOSTS>
            Hosts to manage

    -j, --json
            Output as JSON

        --keys <KEYS>
            API key for a pihole instance. Anything with a length < 10 is considered no key

    -v, --verbose
            Be verbose

    -V, --version
            Print version information

SUBCOMMANDS:
    all-queries               DNS query data
    cache                     Cache statistics
    client-names              Hostname and IP for clients
    cname                     Custom DNS records
    disable                   Disable ad blocking
    dns                       Custom DNS records
    enable                    Enable ad blocking
    forward-destinations      Percentage of queries forwarded to each target
    help                      Print this message or the help of the given subcommand(s)
    list                      Show/Modify a black/whitelist
    logage                    Logage info
    network                   Network clients
    over-time-data-clients    Get queries over time by client
    over-time10-min           Number of queries, binned into 10 minute blocks
    queries-count             Total number of queries
    query-types               Number of queries per type
    summary                   Get summary information
    summary-raw               Get raw (numerical) summary information
    top-clients               Clients with the most queries
    top-clients-blocked       Clients with the most blocked queries
    top-items                 Most queries items
    version                   Simple PiHole Version
    versions                  Versions of core, FTL and web and if updates are available
```

Help information is provided per command.
```
$ piholectl list help
piholectl-list 
Show/Modify a black/whitelist

USAGE:
    piholectl list <LIST> <SUBCOMMAND>

ARGS:
    <LIST>    List to use Acceptable lists are: `white`, `black`, `white_regex`, `black_regex`,
              `white_wild`, `black_wild`, `audit`

OPTIONS:
    -h, --help    Print help information

SUBCOMMANDS:
    add       
    help      Print this message or the help of the given subcommand(s)
    remove    
    show    
```

Multiple hosts can be specified as parameters. Host/Key pairs are matched by order i.e. the nth host will be matched with the nth key. Keys must be specified but anything less than 10 characters in length is considered as "None" and will only be able to perform unauthenticated queries.
```
$ piholectl --hosts 'http://localhost' --keys <API Key> --hosts 'http://127.0.0.1' --keys none enable`
+------------------+---------+
| Host             | status  |
+------------------+---------+
| http://localhost | enabled |
+------------------+---------+

Errors:
http://127.0.0.1: MissingAPIKey
```
### Configuration
A configuration file can store frequently used combinations, which will be queried **in addition** to any hosts specified in the command line options. By default the platform specific configuration file location is used. Running with `-v` or `--verbose` will output the searched path.  
The JSON configuration file contains a list of named hosts with optional keys as well as groups. Groups contain a list of named hosts which will be queried. If no groups (multiple can be specified) are specified in the command line options, then the group named `default` is used.
```json
{
  "hosts": {
    "test_with_key": {
      "host": "http://localhost",
      "key": "<API KEY>"
    },
    "test_no_key": {
      "host": "http://127.0.0.1"
    }
  },
  "groups": {
    "default": [
      "test_with_key",
      "test_no_key"
    ],
    "example_group": [
      "test_no_key"
    ]
  }
}
```

For example with the above configuration file the following outputs are produced:
```
$ # no group is specified, "default" group is used
$ piholectl enable
+------------------+---------+
| Host             | status  |
+------------------+---------+
| http://localhost | enabled |
+------------------+---------+

Errors:
http://127.0.0.1: MissingAPIKey
```

```
$ # "example_group" group is specified so "default" group is not added automatically
$ piholectl enable -g example_group
+------------------+---------+
| Host             | status  |
+------------------+---------+
| http://localhost | enabled |
+------------------+---------+

Errors:
http://127.0.0.1: MissingAPIKey
```

```
# Group "example_group" is used in addition to the manually specified host
$ piholectl -g example_group --hosts 'http://192.168.0.54' --keys none enable
+------+--------+
| Host | status |
+------+--------+

Errors:
http://192.168.0.54: MissingAPIKey
http://127.0.0.1: MissingAPIKey

```

### JSON Output
The output can be set to be JSON using `-j` or `--json`:
```
$ piholectl -j list black show
{
  "http://localhost": {
    "Ok": [
      {
        "id": 255,
        "type": 1,
        "domain": "example.net",
        "enabled": false,
        "date_added": 1657367855,
        "date_modified": 1657367855,
        "comment": "chom",
        "groups": [
          0
        ]
      },
      {
        "id": 256,
        "type": 1,
        "domain": "example.com",
        "enabled": false,
        "date_added": 1657367898,
        "date_modified": 1657367898,
        "comment": "",
        "groups": [
          0
        ]
      }
    ]
  },
  "http://127.0.0.1": {
    "Err": "MissingAPIKey"
  }
}
```