mod package;
mod file;
use clap::{Parser, Subcommand};
use crate::package::install;
use crate::file::clean_lib;
use crate::package::update;
use crate::package::init_package;

#[derive(Parser)]
#[command(name = "sdlpkg", version = "1.0", author = "Nid77", about = "SDL Package Manager")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Init,
    Install,
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
        Commands::Install => {
            println!("Installing...");
            install()?;
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
