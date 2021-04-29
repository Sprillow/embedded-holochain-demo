# embedded holochain runner with pre-installed DNA
a demonstration of using holochain as a library, and then building a customized binary around that, specifically for your app

Makes it very easy to shut down a single process, and restart that single process,
and have the same admin websocket, app websocket, and only generate an agent key on
first run.

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
cd /a/directory/to/install/persist/and/run
embedded-holochain-demo --help
```

Press Ctrl-C to quit.

The first time you run it, it will create a folder called `databases` *inside* of the folder from which you started the command, if it can. If that folder gets deleted, so does any data that has not been gossiped to peers, along with the ability to easily continue using the identity that you were previously using.

Next time you want to use it, make sure you `cd` into the same directory
as you initialized in, otherwise a new identity will be issued and new `databases` folder created.