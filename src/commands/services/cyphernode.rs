/// Contains all subcommands under cncli service main <SUBCOMMAND>

use crate::lib::e::{CNCError};

/// Runs build.sh
pub fn build()->Result<(), CNCError>{
    println!("Build cyphernode docker images");
    Ok(())
}

/// Runs dist/setup.sh
pub fn setup()->Result<(), CNCError>{
    println!("Setup cyphernode");
    Ok(())
}

/// Runs dist/start.sh
pub fn start()->Result<(), CNCError>{
    println!("Start cyphernode");
    Ok(())

}

/// Runs dist/stop.sh
pub fn stop()->Result<(), CNCError>{
    println!("Stop cyphernode");
    Ok(())

}

/// Runs dist/testdeployment.sh
pub fn test()->Result<(), CNCError>{
    println!("Test cyphernode");
    Ok(())
}