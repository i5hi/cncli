#![allow(dead_code)]
use clap::{App, AppSettings, Arg};
use std::str::FromStr;


fn main() {


    let matches = App::new("\x1b[0;92mcncli\x1b[0m")
        .about("\x1b[0;94mCyphernode admin tools.\x1b[0m")
        .version("\x1b[0;1m0.0.1\x1b[0m")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .author("Bitcoin Watchdog: BC5A D8A2 6AAC D383 EF63 0D45 5AE8 AC51 D171 F109")
        .subcommand(
            App::new("init")
                .about("Setup cyphernode working directory")
                .display_order(0)
        )
        .subcommand(
            App::new("info")
                .about("Get info about cyphernode")
                .display_order(1)
        )
        .subcommand(
            App::new("service")
                .about("Service level subcommands")
                .display_order(2)
                .setting(AppSettings::SubcommandRequiredElseHelp)
                .subcommand(App::new("list").about("List all running services.")).display_order(0)
                .subcommand(
                    App::new("gatekeeper").about("Entry point for all requests made to cyphernode.").display_order(1)
                    // .arg(
                    //     Arg::with_name("uid")
                    //     .required(true)
                    //     .help("The uid of the client to delete."),
                // )
            )
            .subcommand(
                    App::new("proxy").about("Core module in cyphernode. Handles all requests from client via gatekeeper and dispatches to other other services.").display_order(2)

                ).subcommand(
                App::new("bitcoin").about("Bitcoin Core Node").display_order(4)
                // .arg(
                //     Arg::with_name("uid")
                //     .required(true)
                //     .help("The uid of the client to delete."),
            // )
        ).subcommand(
            App::new("pycoin").about("Bitcoin keys and addresses tool").display_order(5)
            // .arg(
            //     Arg::with_name("uid")
            //     .required(true)
            //     .help("The uid of the client to delete."),
        // )
    )
.subcommand(
    App::new("tor").about("Used to serve traefik, bitcoin and/or lightning as a HiddenService as well as Internet Gateway").display_order(6)
    // .arg(
    //     Arg::with_name("uid")
    //     .required(true)
    //     .help("The uid of the client to delete."),
// )
)
.subcommand(
    App::new("ots").about("Used to stamp hashes on the Bitcoin blockchain").display_order(7)
    // .arg(
    //     Arg::with_name("uid")
    //     .required(true)
    //     .help("The uid of the client to delete."),
// )
).subcommand(
    App::new("eps").about("Electrum Personal Server").display_order(8)
    // .arg(
    //     Arg::with_name("uid")
    //     .required(true)
    //     .help("The uid of the client to delete."),
// )
).subcommand(
    App::new("ln").about("Core Lightning Node").display_order(9)
    // .arg(
    //     Arg::with_name("uid")
    //     .required(true)
    //     .help("The uid of the client to delete."),
// )
),

        ).get_matches();
  


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
                        println!("LISTS ALL RUNNING SERVICES");
                    }
                    ("gatekeeper", Some(_)) => {
                        println!("GATEKEEPER COMMANDS")

                    }
                    ("proxy", Some(_)) => {
                        println!("PROXY COMMANDS")

                    }                    
                    ("bitcoin", Some(_)) => {
                        println!("BITCOIN COMMANDS")

                    }
                    ("pycoin", Some(_)) => {
                        println!("PYCOIN COMMANDS")

                    }
                    ("tor", Some(_)) => {
                        println!("TOR COMMANDS")

                    }
                    ("ots", Some(_)) => {
                        println!("OTS CLIENT COMMANDS")

                    }
                    ("eps", Some(_)) => {
                        println!("ELECTRUM COMMANDS")

                    }
                    ("ln", Some(_)) => {
                        println!("LIGHTNING COMMANDS")

                    }

                _ => unreachable!(),
            }
        }
        ("",None) => println!("No subcommand was used. try `cncli help`."), 
        _ => unreachable!(),


    }

}