#![allow(dead_code)]
use clap::{App, AppSettings, Arg};
use git2::Repository;
use run_script::ScriptOptions;
use std::process::Command;
use std::sync::mpsc::channel;
fn main() {
    let matches = App::new("\x1b[0;92mcncli\x1b[0m")
        .about("\x1b[0;94mcyphernode admin control.\x1b[0m")
        .version("\x1b[0;1m0.0.6\x1b[0m")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .author("ishi: BC5A D8A2 6AAC D383 EF63 0D45 5AE8 AC51 D171 F109")
        .subcommand(
            App::new("init")
                // .setting(AppSettings::SubcommandRequiredElseHelp)
                .about("Setup cyphernode locally")
                .display_order(0)
                .arg(
                    Arg::with_name("repo")
                        .takes_value(true)
                        .required(false)
                        .short("r")
                        .help("Url to cyphernode repo")
                        .default_value("https://github.com/SatoshiPortal/cyphernode.git"),
                )
                .arg(
                    Arg::with_name("path")
                        .takes_value(true)
                        .short("p")
                        .help("Choose path to repo."),
                ),
        )
        .subcommand(
            App::new("build")
                .about("Build cyphernode images")
                .display_order(1),
        )
        .subcommand(
            App::new("setup")
                .about("Run setup script")
                .display_order(2)
                .arg(
                    Arg::with_name("irc")
                        .takes_value(false)
                        .help("Use irc flag with setup"),
                ),
        )
        .subcommand(App::new("start").about("Run start script").display_order(3))
        .subcommand(App::new("stop").about("Run stop script").display_order(4))
        .subcommand(
            App::new("ps")
                .about("List all running services")
                .display_order(5),
        )
        .subcommand(
            App::new("info")
                .about("Returns data stored in db")
                .display_order(6),
        )
        .subcommand(
            App::new("guide")
            .about("Display usage guide")
            .display_order(7),

        )
        .get_matches();

    match matches.subcommand() {
        ("info", Some(_)) => {
            let db_path = std::env::var("HOME").unwrap() + "/.cncli";
            let db: sled::Db = sled::open(db_path).unwrap();
            let path = db.get(b"path").unwrap();
            if path.is_some() {
                println!(
                    "Path: {:#?}",
                    std::str::from_utf8(&path.clone().unwrap())
                        .unwrap()
                        .to_string()
                );

            } else {
                println!("Repo not initialized. use cncli init -p $path -r $repo");
                std::process::exit(1)
            }
        }
        ("init", Some(submatches)) => {
            let db_path = std::env::var("HOME").unwrap() + "/.cncli";
            let db: sled::Db = sled::open(db_path).unwrap();
            let path = db.get(b"path").unwrap();
            if path.is_some() {
                let exists = std::path::Path::new(
                    (std::str::from_utf8(&path.clone().unwrap())
                        .unwrap()
                        .to_string())
                    .as_str(),
                )
                .exists();
                if exists {
                    println!(
                        "Repo already exists at {:#?}",
                        std::str::from_utf8(&path.clone().unwrap())
                            .unwrap()
                            .to_string()
                    );
                    std::process::exit(1)
                }
            }
            let repo = submatches.value_of("repo").unwrap();
            let path = submatches.value_of("path");
            if path.is_none() {
                println!("-p <path> is required. Either use an existing path or specify a new path where the repo will be cloned.");
                std::process::exit(1)
            } else {
                let fmtd_path = path.clone().unwrap().to_string().replace("/cyphernode", "");
                let exists =
                    std::path::Path::new((fmtd_path.clone() + "/cyphernode").as_str()).exists();
                if exists {
                    db.insert(b"path", (fmtd_path.clone() + "/cyphernode").as_bytes())
                        .unwrap();
                    println!("Saved EXISTING repo path {}/cyphernode.", fmtd_path);
                } else {
                    match Repository::clone(repo, fmtd_path.clone()) {
                        Ok(repo) => repo,
                        Err(e) => {
                            println!("Failed to clone repo: {}", e);
                            std::process::exit(1)
                        }
                    };
                    println!("Pulled {} into {}/cyphernode.", repo, fmtd_path.clone());
                    db.insert(b"path", (fmtd_path.clone() + "/cyphernode").as_bytes()).unwrap();
                    println!("Saved NEW repo path {}/cyphernode.", fmtd_path);
                }
            }
        }
        ("build", Some(_)) => {
            let db_path = std::env::var("HOME").unwrap() + "/.cncli";
            let db: sled::Db = sled::open(db_path).unwrap();
            let path = db.get(b"path").unwrap();
            if path.is_none() {
                println!("Repo not initialized. use cncli init -p $path -r $repo");
                std::process::exit(1)
            }

            let mut child = Command::new("bash")
                .arg("build.sh")
                .current_dir(std::str::from_utf8(&path.unwrap()).unwrap())
                .spawn()
                .expect("bash command failed to start");

            println!("PID: {:#?}", child.id());

            let (tx, rx) = channel();
            ctrlc::set_handler(move || {
                tx.send(child.kill())
                    .expect("Could not send signal on channel.")
            })
            .expect("Error setting Ctrl-C handler");

            println!("Waiting for Ctrl-C...");
            rx.recv().expect("Could not receive from channel.").unwrap();
            println!("Got SIGINT! Exiting...");
        }
        ("setup", Some(subcommands)) => {
            let db_path = std::env::var("HOME").unwrap() + "/.cncli";
            let db: sled::Db = sled::open(db_path).unwrap();
            let path = db.get(b"path").unwrap();
            if path.is_none() {
                println!("Repo not initialized. use cncli init -p $path -r $repo");
                std::process::exit(1)
            }

           
            let irc = if subcommands.is_present("irc"){
                "-irc"
            }else{
                ""
            };

            let mut child = Command::new("bash")
                .arg("setup.sh")
                .arg(irc)
                .current_dir(std::str::from_utf8(&path.unwrap()).unwrap().to_string() + "/dist")
                .spawn()
                .expect("bash command failed to start");

            println!("PID: {:#?}", child.id());

            let (tx, rx) = channel();
            ctrlc::set_handler(move || {
                tx.send(child.kill())
                    .expect("Could not send signal on channel.")
            })
            .expect("Error setting Ctrl-C handler");

            println!("Waiting for Ctrl-C...");
            rx.recv().expect("Could not receive from channel.").unwrap();
            println!("Got SIGINT! Exiting...");
        }
        ("start", Some(_)) => {
            let db_path = std::env::var("HOME").unwrap() + "/.cncli";
            let db: sled::Db = sled::open(db_path).unwrap();
            let path = db.get(b"path").unwrap();
            if path.is_none() {
                println!("Repo not initialized. use cncli init -p $path -r $repo");
                std::process::exit(1)
            }

            let mut child = Command::new("bash")
                .arg("start.sh")
                .current_dir(std::str::from_utf8(&path.unwrap()).unwrap().to_string() + "/dist")
                .spawn()
                .expect("bash command failed to start");

            let (tx, rx) = channel();
            ctrlc::set_handler(move || tx.send(true).expect("Could not send signal on channel."))
                .expect("Error setting Ctrl-C handler");

            println!("Waiting for Ctrl-C...");
            let result = rx.recv().expect("Could not receive from channel.");
            if result {
                child.kill().expect("failed to kill child");
            }
            // child.wait().expect("failed to wait on child");
            // std::process::exit(0)
            // println!("DONE");
            // println!("Got SIGINT! Exiting...");
        }
        ("stop", Some(_)) => {
            let db_path = std::env::var("HOME").unwrap() + "/.cncli";
            let db: sled::Db = sled::open(db_path).unwrap();
            let path = db.get(b"path").unwrap();
            if path.is_none() {
                println!("Repo not initialized. use cncli init -p $path -r $repo");
                std::process::exit(1)
            }

            let mut child = match Command::new("bash")
                .arg("stop.sh")
                .current_dir(std::str::from_utf8(&path.unwrap()).unwrap().to_string() + "/dist")
                .spawn()
            {
                Ok(child) => child,
                Err(_) => {
                    panic!("Failed to execute command.");
                }
            };

            let (tx, rx) = channel();
            ctrlc::set_handler(move || tx.send(true).expect("Could not send signal on channel."))
                .expect("Error setting Ctrl-C handler");

            println!("Waiting for Ctrl-C...");
            let result = rx.recv().expect("Could not receive from channel.");
            if result {
                child.kill().expect("failed to kill child");
            }

            // println!("DONE. YOU CAN Ctrl+C to Exit now: {:#?}.", status);
        }
        ("ps", Some(_)) => {
            let db_path = std::env::var("HOME").unwrap() + "/.cncli";
            let db: sled::Db = sled::open(db_path).unwrap();
            let path = db.get(b"path").unwrap();
            if path.is_none() {
                println!("Repo not initialized. use cncli init -p $path -r $repo");
                std::process::exit(1)
            }
            let options = ScriptOptions::new();

            let args = vec![];

            let (_, output, _) = run_script::run(
                r#"
                docker ps --filter 'network=cyphernodenet' --filter='network=cyphernodeappsnet' --format 'table {{.ID}} \t {{.Names}} \t {{.Status}} \t {{.Ports}}'
                 "#,
                &args,
                &options,
            )
            .unwrap();
            print!("{}", output);
        }
        ("guide", Some(_)) => {
            let base_path = env!("CARGO_MANIFEST_DIR").to_string() + "/art/cn.ascii";
            let title = "cyphernode admin client";
            let contents = std::fs::read_to_string(&base_path)
                .expect("Should have been able to read the file");
            println!("\x1b[93;1m{}\x1b[0m", title);
            println!("\x1b[92;1m{}\x1b[0m", contents);
            println!("\x1b[93;1mGETTING STARTED:\x1b[0m");

            println!("cncli must be initialized with the init command. The init command sets the path to your cyphernode repo. If this is your first time using cyphernode, chose a new path. If you have an existing repo, use that path.");
            println!("The -p <PATH> argument is required by the init command. An optional repo command allows you to pull a custom repo. The default repo is the master branch from Satoshi Portal.");
            println!("~/.cncli is where working data is stored - primarily the path to your cyphernode repo. If you delete this, you will have to call init again.");
            println!("You can only manage a single cyphernode repo instance at a time.");


            println!("\x1b[93;1mEXAMPLE:\x1b[0m");


            println!("Initialize the default repo:");
            println!("\x1b[92;1mcncli -p /home/ishi/\x1b[0m");

            println!("Initialize a custom repo:");
            println!("\x1b[92;1mcncli -p /home/ishi/ -r https://github.com/i5hi/cyphernode.git\x1b[0m");

            println!("\x1b[93;1mBEHAVIOUR NOTES:\x1b[0m");

            println!("The outputs of commands that call scripts (build,setup,start,stop) do not exit by themselves and will leave the terminal locked after completion.");
            println!("\x1b[92;1mYou have to manually exit such commands using Ctrl+C.\x1b[0m");


        }
        ("", None) => println!("No subcommand was used. try `cncli help`."),
        _ => unreachable!(),
    }
}

