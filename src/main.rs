mod package;
mod file;
mod services;
use clap::{Parser, Subcommand};
use file::{cleanup, init};
use package::update_lib;
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
use std::vec;
use crate::file::DEST_DIR;


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
    Clean{
        #[arg(long, value_enum, value_parser, num_args = 1..)]
        only: Vec<LibTag>,
    },
    Update{
        lib: Option<String>,
    },
    Uninstall{
        lib: Option<String>,
    }
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init => {
            init_package()?;
            println!("Package initialized.");
        }
        Commands::Install { lib , only } => {
            let mut libs = get_sdl_config();
            
            if let Some(lib) = lib {
                match get_lib(&lib) {
                    Some(lib) => {
                        if !libs.sdl.libs.iter().any(|l| l.name == lib.name) {
                            libs.sdl.libs.push(lib.clone());
                            update_package(&libs)?;
                        }
                        libs.sdl.libs = vec![lib.clone()];
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
        Commands::Clean { only } => {
            if only.is_empty() {
                clean_lib()?;
                println!("Cleaned all libs.");
            } else {
                for dir in only  {
                    std::fs::remove_dir_all(format!("{}/{}", DEST_DIR, dir.to_string().to_lowercase()))?;
                    println!("Cleaned {}.", dir.to_string().to_lowercase());
                }
            }
            
        }
        Commands::Update { lib } => {
            if let Some(lib) = lib {
                println!("Updating {}...", lib);
                let mut libs: SdlConfig = get_sdl_config();
                if libs.sdl.libs.iter().any(|l| l.name == lib) {
                    let lib = libs.sdl.libs.iter_mut().find(|l| l.name == lib).unwrap();
                    update_lib(lib)?;
                    update_package(&libs)?;
                } else {
                    println!("Lib not found");
                }
                return Ok(());
            }
           
            println!("Updating all libs...");
            update()?;
        }
        Commands::Uninstall { lib } => {
            let mut libs: SdlConfig = get_sdl_config();
            if let Some(lib) = lib {
                if libs.sdl.libs.iter().any(|l| l.name == lib) {
                    libs.sdl.libs.retain(|l| l.name != lib);
                    println!("Uninstalled {}.", lib);
                } else {
                    println!("Lib not found.");
                    return Ok(());
                }
                
            } else {
                println!("Uninstalling all libs...");
                libs.sdl.libs.clear();
            }
            update_package(&libs)?;
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
        update_lib(lib)?;
    }
    update_package(&updated_libs)?;

    Ok(())
}

