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

## future

If this client works out well, we will consider creating a desktop GUI using dart/flutter.

## test run

### NOTE: THIS SOFTWARE DOES NOTHING RIGHT NOW. It only contains the api wireframe.

Once you have rust installed and cloned the project. You can use `cargo run` to test run commands.

```bash
# see all available commands
cargo run

# see all available service sub commands
cargo run services

# run build.sh
cargo services main build

# run setup.sh (works with flags like -irc too)
cargo services main setup

# run start.sh
cargo services main start

# tail bitcoin logs
cargo run services bitcoin log

# view bitcoin conf
cargo run services bitcoin conf

# view proxy logs
cargo run services proxy log
```

## building the binary

```bash
cargo build --release
# this will build a binary in the target folder called cncli

mv target/release/cncli /bin
# you can strip this binary if you want to make it smaller by removing debug symbols etc.
strip /bin/cncli

cncli info
```



