# cncli

An admin command line control panel for cyphernode.

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

ALTERNATIVELY, build the binary and move it to your /bin directory and then use `cncli` to run commands:

```bash
cargo build --release -j 4
# -j sets number of cpu cores
sudo cp target/release/cncli /usr/bin
sudo strip /usr/bin/cncli
# strips size of XCOFF object files; resulting in 3x size reduction
```

```bash
# see all available commands
cargo run

# sets up a wokring directory at $HOME/.cncli where path information about cyphernode repo is cloned
cargo run init -p /home/ishi

# run build.sh
cargo run build

# run setup.sh 
cargo run setup
cargo run setup irc

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

## reset

You can reset cncli by deleting its working directory

```bash
rm -rf ~/.cncli
```