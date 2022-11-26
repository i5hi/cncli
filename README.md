# cncli

An admin command line control panel for cyphernode.

## motivation

Cyphernode is a modular bitcoin docker network that gives users a lot of flexibility in running their own bitcoin infrastructure; with a focus on being small and safe. 

As such it can be difficult for the average user to setup and manage their home node infrastrcuture without prior knowledge in system administration and specifically docker.

cncli aims to improve cyphernode's user interface by :

- keeping track of the entire project and data directory structure
- handling all the basic admin ops
- exposing a simple api

## goal

The primary goal is to create a client tool that makes it easy for anyone to run a safe torified electrum personal server at home to use with their mobile wallets.

In spirit of the original project, cncli aims not to reinvent the wheel. It simply redirects the user to existing parts of cyphernode to get their work done more easily through a more friendly interface.

## test run

First install the rust toolchain:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Clone the project:

```bash
git clone https://github.com/i5hi/cncli
cd cncli
```

Once you have rust installed and cloned the project. You can use `cargo run` to test run commands. (recommended since project is still in dev)

ALTERNATIVELY, build the binary and move it to your /bin directory:

```bash
cargo build --release -j <CPU CORES>
cp target/release/cnclie /bin
strip /bin/cncli
```

```bash
# see all available commands
cargo run / cncli

# sets up a wokring directory at $HOME/.cncli where path information about cyphernode repo is cloned
cargo run init --path /home/ishi --repo https://github.com/SatoshiPortal/cyphernode.git

# run build.sh
cargo run build

# run setup.sh (works with flags like -irc too)
cargo run setup

# run start.sh
cargo run start

# run stop.sh
cargo run stop

# runs docker ps with --filter and --format
cargo run list
```


## upgrades

- After setup is complete, datadir path should be stored in working dir $HOME/.cncli to allow easy inspection

- Process must automatically exit on completion. Currently Ctrl+C is required to free the terminal.

