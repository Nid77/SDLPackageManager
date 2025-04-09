mod package;
mod file;
mod services;
use clap::{Parser, Subcommand};
use file::{cleanup, init};
use package::{get_lib, update_package, LibTag, SdlConfig};
use crate::file::clean_lib;
use crate::package::init_package;
use crate::package::process_installation;
use crate::package::SdlInstallation;
use crate::services::get_latest_release;
use crate::package::check_libs;
use crate::package::get_sdl_config;
use std::collections::HashMap;
use std::io::{self, Write};


#[derive(Parser)]
#[command(name = "sdlpkg", version = "1.0", author = "Nid77", about = "SDL Package Manager")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Init,
    Install {
        lib: Option<String>,
        #[arg(long, value_enum, value_parser, num_args = 1..)]
        only: Vec<LibTag>,
    },
    Remove,
    Update,
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init => {
            println!("Initializing...");
            init_package()?;
        }
        Commands::Install { lib , only } => {
            let mut libs = get_sdl_config();
            
            if let Some(lib) = lib {
                match get_lib(&lib) {
                    Some(lib) => {
                        libs.sdl.libs = vec![lib];
                    },
                    None => {
                        println!("Lib not found");
                        return Ok(());
                    }
                }
            }

            let param : SdlInstallation = SdlInstallation {
                libs,
                only,
            };

            install(param)?;

        }
        Commands::Remove => {
            println!("Removing...");
            clean_lib()?;
        }
        Commands::Update => {
            println!("Updating...");
            update()?;
        }
    }
    Ok(())
}


pub fn install(param: SdlInstallation) -> Result<(), Box<dyn std::error::Error>> {
    check_libs(&param.libs)?;
    init()?;
    process_installation(&param)?;
    cleanup()?;
    Ok(())
}

pub fn update() -> Result<(), Box<dyn std::error::Error>> {
    let libs: SdlConfig = get_sdl_config();
    check_libs(&libs)?;
    let mut versions = HashMap::new();
    for lib in &libs.sdl.libs {
        let v = get_latest_release(&lib.name)?;
        versions.insert(lib.name.clone(), v.clone());
        println!("{} latest version - {} ", lib.name, v);
    }

    print!("➡️  Do you want to update all libs? (y/n): ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input = input.trim();

    if input != "y" {
        println!("Abandon");
        return Ok(());
    }

    println!("Updating...");
    let mut updated_libs = libs.clone();
    for lib in &mut updated_libs.sdl.libs {
        let version_parts: Vec<&str> = versions.get(&lib.name).unwrap().split('-').collect();
        lib.version = version_parts[1].to_string();
        lib.status = version_parts[0].to_string();
    }
    update_package(&updated_libs)?;

    Ok(())
}

