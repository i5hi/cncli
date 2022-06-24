#![allow(dead_code)]
use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};
use std::str::FromStr;


fn main() {
    let matches = App::new("\x1b[0;92mcncli\x1b[0m")
        .about("\x1b[0;94mCyphernode admin tools.\x1b[0m")
        .version("\x1b[0;1m0.0.1\x1b[0m")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .author("Bitcoin Watchdog: BC5A D8A2 6AAC D383 EF63 0D45 5AE8 AC51 D171 F109")
        .subcommand(
            App::new("fetch")
                .about("Get cyphernode release from SatoshiPortal")
                .display_order(0)
                .arg(
                    Arg::with_name("version")
                    .short("v")
                    .help("Choose version.")
                    .default_value("latest")
                )
        )
        .subcommand(
            App::new("init")
                .about("Setup cyphernode working directory")
                .display_order(1)
        )
        .subcommand(
            App::new("info")
                .about("Get info about cyphernode")
                .display_order(2)
        )
        .subcommand(
            App::new("service")
                .about("Service level subcommands")
                .display_order(3)
                .setting(AppSettings::SubcommandRequiredElseHelp)
                .subcommand(
                    App::new("list").about("List all running services."))
                    .display_order(0)
                .subcommand(
                    App::new("main")
                    .about("Main cyphernode commands.")
                    .display_order(1)
                    .setting(AppSettings::SubcommandRequiredElseHelp)
                    .subcommands(vec![
                        SubCommand::with_name("build")
                        .about("Build all service containers (build.sh)"),
                        SubCommand::with_name("setup")
                        .about("Setup cyphernode (setup.sh)"),
                        SubCommand::with_name("start")
                        .about("Start cyphernode (start.sh)"),
                        SubCommand::with_name("stop")
                        .about("Stop cyphernode (stop.sh)"),
                        SubCommand::with_name("test")
                        .about("Test cyphernode (testdeployment.sh)"),
                    ]))
                .subcommand(
                    App::new("gatekeeper")
                    .about("Entry point for all requests made to cyphernode.")
                    .display_order(2)
                    .setting(AppSettings::SubcommandRequiredElseHelp)
                    .subcommands(vec![
                        SubCommand::with_name("info")
                        .about("Information about running service"),
                        SubCommand::with_name("log")
                        .about("Debug log"),
                        SubCommand::with_name("exec")
                        .about("Executes a command inside service container"),
                        SubCommand::with_name("conf")
                        .about("Display service configuration"),
                        SubCommand::with_name("restart")
                        .about("Restart service"),
                    ]))
                .subcommand(
                    App::new("proxy")
                    .about("Core module in cyphernode. Handles all requests from client via gatekeeper and dispatches to other other services.")
                    .display_order(3)
                    .setting(AppSettings::SubcommandRequiredElseHelp)
                    .subcommands(vec![
                        SubCommand::with_name("info")
                        .about("Information about running service"),
                        SubCommand::with_name("log")
                        .about("Debug log"),
                        SubCommand::with_name("exec")
                        .about("Executes a command inside service container"),
                        SubCommand::with_name("conf")
                            .arg(
                                Arg::with_name("file")
                                .short("f")
                                .required(false)
                                .help("Specifies which conf file to open or edit. Select by index or full path.")
                            )
                            .arg(
                                Arg::with_name("edit")
                                .short("e")
                                .required(false)
                                .help("Edit mode opens the conf file in your editor of choice")
                                .default_value("nano")
                            )
                            .arg(
                                Arg::with_name("print")
                                .short("p")
                                .required(false)
                                .takes_value(false)
                                .help("Prints the conf file to stdout")
                            )
                        .about("Display all service configuration files (with index)."),
                        SubCommand::with_name("restart")
                        .about("Restart service"),
                    ])).
                subcommand(
                    App::new("bitcoin")
                    .about("Bitcoin Core Node")
                    .display_order(4)
                    .setting(AppSettings::SubcommandRequiredElseHelp)
                    .subcommands(vec![
                        SubCommand::with_name("info")
                        .about("Information about running service"),
                        SubCommand::with_name("log")
                        .about("Debug log"),
                        SubCommand::with_name("exec")
                        .about("Executes a command inside service container"),
                        SubCommand::with_name("conf")
                            .arg(
                                Arg::with_name("file")
                                .short("f")
                                .help("Specifies which conf file to open or edit. Select by index or full path.")
                            )
                            .arg(
                                Arg::with_name("edit")
                                .short("e")
                                .help("Edit mode opens the conf file in your editor of choice")
                                .default_value("nano")
                            )
                            .arg(
                                Arg::with_name("print")
                                .short("p")
                                .takes_value(false)
                                .help("Prints the conf file to stdout")
                            )
                        .about("Display all service configuration files (with index)."),
                        SubCommand::with_name("restart")
                        .about("Restart service"),
                    ]))
                .subcommand(
                    App::new("tor")
                    .about("Used to serve traefik, bitcoin and/or lightning as a HiddenService as well as Internet Gateway")
                    .display_order(6)
                    .setting(AppSettings::SubcommandRequiredElseHelp)
                    .subcommands(vec![
                        SubCommand::with_name("info")
                        .about("Information about running service"),
                        SubCommand::with_name("log")
                        .about("Debug log"),
                        SubCommand::with_name("exec")
                        .about("Executes a command inside service container"),
                        SubCommand::with_name("conf")
                            .arg(
                                Arg::with_name("file")
                                .required(false)
                                .short("f")
                                .help("Specifies which conf file to open or edit. Select by index or full path.")
                            )
                            .arg(
                                Arg::with_name("edit")
                                .short("e")
                                .required(false)
                                .help("Edit mode opens the conf file in your editor of choice")
                                .default_value("nano")
                            )
                            .arg(
                                Arg::with_name("print")
                                .short("p")
                                .required(false)
                                .takes_value(false)
                                .help("Prints the conf file to stdout")
                            )
                        .about("Display all service configuration files (with index)."),
                        SubCommand::with_name("restart")
                        .about("Restart service"),
                    ]))
                .subcommand(
                    App::new("eps")
                    .about("Electrum Personal Server")
                    .display_order(8)
                    .setting(AppSettings::SubcommandRequiredElseHelp)
                    .subcommands(vec![
                        SubCommand::with_name("info")
                        .about("Information about running service"),
                        SubCommand::with_name("log")
                        .about("Debug log"),
                        SubCommand::with_name("exec")
                        .about("Executes a command inside service container"),
                        SubCommand::with_name("conf")
                            .arg(
                                Arg::with_name("file")
                                .short("f")
                                .required(false)
                                .help("Specifies which conf file to open or edit. Select by index or full path.")
                            )
                            .arg(
                                Arg::with_name("edit")
                                .short("e")
                                .required(false)
                                .help("Edit mode opens the conf file in your editor of choice")
                                .default_value("nano")
                            )
                            .arg(
                                Arg::with_name("print")
                                .short("p")
                                .required(false)
                                .takes_value(false)
                                .help("Prints the conf file to stdout")
                            )
                        .about("Display all service configuration files (with index)."),
                        SubCommand::with_name("restart")
                        .about("Restart service"),
                    ]))

        )
        .get_matches();
    
    match matches.subcommand() {
        ("init", Some(_)) => {
            println!("CYPHERNODE INIT");
        }
        ("info", Some(_)) => {
            println!("CYPHERNODE INFO");
        }
        ("service", Some(service_matches)) => {
            match service_matches.subcommand() {
                    ("list", Some(_)) => {
                        println!("List running services")
                    }
                    ("gatekeeper", Some(subcommand_matches)) => {
                        match subcommand_matches.subcommand() {
                            ("info", Some(_)) => {
                                println!("Get service info");
                            }
                            ("log", Some(_)) => {
                                println!("Get service logs");
                            }
                            ("exec", Some(_)) => {
                                println!("Execute command in service container");
                            }
                            ("conf", Some(_)) => {
                                println!("List service configuration.");
                            }
                            ("restart", Some(_)) => {
                                println!("Restart Service");
                            }
                            _ => unreachable!(),
                        }
                    }
                    ("proxy", Some(subcommand_matches)) => {
                        match subcommand_matches.subcommand() {
                            ("info", Some(_)) => {
                                println!("Get service info");
                            }
                            ("log", Some(_)) => {
                                println!("Get service logs");
                            }
                            ("exec", Some(_)) => {
                                println!("Execute command in service container");
                            }
                            ("conf", Some(_)) => {
                                println!("List service configuration.");
                            }
                            ("restart", Some(_)) => {
                                println!("Restart Service");
                            }
                            _ => unreachable!(),
                        }
                    }                    
                    ("bitcoin", Some(subcommand_matches)) => {
                        match subcommand_matches.subcommand() {
                            ("info", Some(_)) => {
                                println!("Get service info");
                            }
                            ("log", Some(_)) => {
                                println!("Get service logs");
                            }
                            ("exec", Some(_)) => {
                                println!("Execute command in service container");
                            }
                            ("conf", Some(_)) => {
                                println!("List service configuration.");
                            }
                            ("restart", Some(_)) => {
                                println!("Restart Service");
                            }
                            _ => unreachable!(),
                        }
                    }

                    ("tor", Some(subcommand_matches)) => {
                        match subcommand_matches.subcommand() {
                            ("info", Some(_)) => {
                                println!("Get service info");
                            }
                            ("log", Some(_)) => {
                                println!("Get service logs");
                            }
                            ("exec", Some(_)) => {
                                println!("Execute command in service container");
                            }
                            ("conf", Some(_)) => {
                                println!("List service configuration.");
                            }
                            ("restart", Some(_)) => {
                                println!("Restart Service");
                            }
                            _ => unreachable!(),
                        }
                    }

                    ("eps", Some(subcommand_matches)) => {
                        match subcommand_matches.subcommand() {
                            ("info", Some(_)) => {
                                println!("Get service info");
                            }
                            ("log", Some(_)) => {
                                println!("Get service logs");
                            }
                            ("exec", Some(_)) => {
                                println!("Execute command in service container");
                            }
                            ("conf", Some(_)) => {
                                println!("List service configuration.");
                            }
                            ("restart", Some(_)) => {
                                println!("Restart Service");
                            }
                            _ => unreachable!(),
                        }
                    }
                _ => unreachable!(),
            }
        }
        ("",None) => println!("No subcommand was used. try `cncli help`."), 
        _ => unreachable!(),
    }
}