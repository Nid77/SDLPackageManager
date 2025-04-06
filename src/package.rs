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


pub fn get_architecture() -> String {
    let arch = if cfg!(target_arch = "x86_64") {
        "x64".to_string()
    } else {
        "x86".to_string()
    };
    arch
}

pub fn get_sdl_config() -> SdlConfig {
    let content = fs::read_to_string("sdlpkg.json").expect("Failed to read sdlpkg.json");
    let config: SdlConfig = serde_json::from_str(&content).expect("Failed to parse sdlpkg.json");
    config
}