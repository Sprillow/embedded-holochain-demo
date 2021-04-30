# Demo of using embedded-holochain-runner in a binary
a demonstration of using holochain as a library, and then building a customized binary around that, specifically for your app

Makes it very easy to shut down a single process, and restart that single process,
and have the same admin websocket, app websocket, and only generate an agent key on first run.

## Installing and running

> Prereq: have a [stable version of rust](https://www.rust-lang.org/tools/install) installed

### Installing
```bash
cargo install --git https://github.com/holochain/lair
cargo install --git https://github.com/Sprillow/embedded-holochain-demo
```

### Running
Run the `--help` command to see the options first
```bash
$ embedded-holochain-demo --help
```

```bash
embedded-holochain-demo 0.0.1
demo of embedding holochain in a binary via embedded-holochain-runner

USAGE:
    embedded-holochain-demo [OPTIONS] [datastore-path]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --admin-ws-port <admin-ws-port>     [default: 1234]
        --app-id <app-id>                   [default: my_app_id]
        --app-ws-port <app-ws-port>         [default: 8888]
        --keystore-path <keystore-path>     [default: keystore]
        --proxy-url <proxy-url>             [default: kitsune-proxy://SYVd4CF3BdJ4DS7KwLLgeU3_DbHoZ34Y-
                                           qroZ79DOs8/kitsune-quic/h/165.22.32.11/p/5779/--]

ARGS:
    <datastore-path>    configuration values for `app_id` and `app_ws_port` will be overridden if an existing
                        configuration is found at this path [default: databases]
```