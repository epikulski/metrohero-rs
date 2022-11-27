# metrohero-rs

An unofficial rust client for the MetroHero API.

Using the [`MetroHeroClient`](MetroHeroClient) requires an API key. Information on how to
acquire one is published at: <https://dcmetrohero.com/apis>.

When using the CLI, provide your key via CLI argument or by setting the `METROHERO_API_KEY`
environment variable.

## Getting Started
To build, clone this repository and run:
```shell
git clone https://github.com/epikulski/metrohero-rs.git
cd metrohero-rs
cargo build
cargo test
```

## Using the metrohero-rs CLI
```text
Usage: metrohero_rs [OPTIONS] <COMMAND>

Commands:
  plan        Get information about a route
  departures  Get information about as station
  stations    Print a table of station names and their RTU code
  help        Print this message or the help of the given subcommand(s)

Options:
      --api-key <API_KEY>  MetroHero API key
  -h, --help               Print help information
  -V, --version            Print version information

```

Look up departures from a station
 ```shell
 // Stations can be accessed by name
 cargo run departures Rosslyn

 // Or by their RTU code
 cargo run departures C05
```


```text
 Departures for Rosslyn (C05)
  +------+-----------------------+-----+-------+
  | Line | Destination           | ETA | Notes |
  +============================================+
  |  BL  | Franconia-Springfield | BRD |       |
  |------+-----------------------+-----+-------|
  |  SV  | Downtown Largo        | ARR |       |
  |------+-----------------------+-----+-------|
  |  BL  | Huntington            | 3m  |       |
  +------+-----------------------+-----+-------+
  Source: MetroHero API (https://www.dcmetrohero.com)
```

Look up trip durations between stations.
(Does not support trips that require a transfer)
```shell
 cargo run plan Ballston Rosslyn
```
```text
 Ballston --> Rosslyn
 Expected ride:    7m (normally 6m)
 Next train:       BRD, 9m, 17m, 12:03

 Departures from Ballston
 +------+----------------+-----+-------+
 | Line | Destination    | ETA | Notes |
 +=====================================+
 |  SV  | Downtown Largo | BRD |       |
 |------+----------------+-----+-------|
 |  OR  | New Carrollton | 9m  |       |
 |------+----------------+-----+-------|
 |  SV  | Downtown Largo | 17m |       |
 +------+----------------+-----+-------+
```

Look up a station's name or RTU code
```shell
cargo run stations | grep -i rosslyn
```
```text
| C05  | Rosslyn                                          |
```

## Using metrohero-rs as a Library
Library documentation is availble by running `cargo doc --open`.
