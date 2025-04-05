use std::fs;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SdlConfig {
    pub version: String,
    pub sdl: SdlSection,
}

#[derive(Debug, Deserialize)]
pub struct SdlSection {
    pub arch: String,
    pub libs: Vec<LibEntry>,
}

#[derive(Debug, Deserialize)]
pub struct LibEntry {
    pub name: String,
    pub channel: String,
    pub version: String,
}


pub fn run_install() {

    let content = fs::read_to_string("sdlpkg.json").expect("Failed to read sdlpkg.json");
    let config: SdlConfig = serde_json::from_str(&content).expect("Failed to parse sdlpkg.json");

    println!("📦 Projet: v{}", config.version);
    println!("🏗️  Arch: {}", config.sdl.arch);
    for lib in config.sdl.libs {
        println!("→ Install Lib {} / channel {} / version {}", lib.name, lib.channel, lib.version);
    }
       
}
