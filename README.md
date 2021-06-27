# Demo of using embedded-holochain-runner in a binary
a demonstration of using the [embedded-holochain-runner](https://github.com/Sprillow/embedded-holochain-runner) to run holochain, and then building a customized binary around that, specifically for your app.

Makes it very easy to shut down a single process, and restart that single process,
and have the same admin websocket, app websocket, and **automatically generate an agent key on first run**.

For a more advanced application using your own tokio runtime, shutdown signal, and `StateSignal` listeners, you can check out the [Acorn Holochain application](https://github.com/h-be/acorn/blob/main/conductor/src/main.rs).

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


## Including a DNA in your binary

We have a super simple DNA (that really does nothing) in the [dna/sample](./dna/sample) folder. It gets embedded in the binary by using this code in the main `src` folder:
```rust
const SAMPLE_DNA: &'static [u8] = include_bytes!("../dna/sample/sample.dna");
```

`include_bytes!` is a native to Rust macro to help include files as bytes in a binary.

You can build the DNA file by running:

```bash
./build-dna.sh
```

It compiles your Rust "zome" code to WASM, then zips it up in a file called `dna/sample/sample.dna`.

## Holochain Version

The `rev` in [dna/sample/Cargo.toml](dna/sample/Cargo.toml) should match the one in [embedded-holochain-runner](https://github.com/Sprillow/embedded-holochain-runner#holochain-version), it is currently:

Currently bundles Holochain version: [bdb9c55d504135df39ccb1c75896557a788d0ac0 (June 25, 2021)](https://github.com/holochain/holochain/commit/bdb9c55d504135df39ccb1c75896557a788d0ac0).
