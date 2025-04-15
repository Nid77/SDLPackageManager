use std::fs;
use clap::ValueEnum;
use serde::Deserialize;
use serde::Serialize;
use crate::services::get_latest_release;
use strum_macros::Display;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SdlConfig {
    pub version: String,
    pub sdl: SdlSection,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SdlSection {
    pub arch: String,
    pub libs: Vec<Lib>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Lib {
    pub name: String,
    pub status: String,
    pub version: String,
}

#[derive(Clone, Debug, ValueEnum, PartialEq, Display)]
pub enum LibTag {
    Bin,
    Include,
    Lib,
}

impl LibTag {
    pub fn to_str(&self) -> String {
        match self {
            LibTag::Bin => "bin".to_string(),
            LibTag::Include => "include".to_string(),
            LibTag::Lib => "lib".to_string(),
        }
    }
}

pub const SUPPORTED_LIBS: &[&str] = &[
    "SDL",
    "SDL_image",
    "SDL_mixer",
    "SDL_ttf",
];

pub const SUPPORTED_VERSIONS: &[&str] = &[
    "1.0.0",
];

pub fn get_sdl_config() -> SdlConfig {
    let content = fs::read_to_string("sdlpkg.json").expect("Failed to read sdlpkg.json");
    let config: SdlConfig = serde_json::from_str(&content).expect("Failed to parse sdlpkg.json");
    config
}

pub fn update_package(libs: &SdlConfig) -> Result<(), Box<dyn std::error::Error>> {
    let updated_json = serde_json::to_string_pretty(&libs)?;
    fs::write("sdlpkg.json", updated_json)?;
    println!("Updated sdlpkg.json.");
    Ok(())
}

pub fn init_package() -> Result<(), Box<dyn std::error::Error>> {
    let latest_release = get_latest_release("SDL")?;
    let version_parts: Vec<&str> = latest_release.split('-').collect();

    let libs: SdlConfig = SdlConfig {
        version: "1.0.0".to_string(),
        sdl: SdlSection {
            arch: "auto".to_string(),
            libs: vec![
                Lib {
                    name: "SDL".to_string(),
                    status: version_parts[0].to_string(),
                    version: version_parts[1].to_string()
                }
            ],
        },
    };
    let json = serde_json::to_string_pretty(&libs)?;
    fs::write("sdlpkg.json", json)?;
    println!("Created sdlpkg.json.");

    Ok(())
}

pub fn get_lib(lib_name: &str) -> Option<Lib> {
    let libs: SdlConfig = get_sdl_config();
    let mut new_lib: Option<Lib> = None;
    for lib in &libs.sdl.libs {
        if lib.name == lib_name {
            new_lib = Some(lib.clone());
            break;
        }
    }
   
    if new_lib.is_none() && SUPPORTED_LIBS.contains(&lib_name) {
        let latest_release = get_latest_release(lib_name).unwrap();
        let v: Vec<&str> = latest_release.split('-').collect();

        new_lib = Some(Lib {
            name: lib_name.to_string(),
            status: v[0].to_string(),
            version: v[1].to_string(),
        });
    } 
    new_lib
}

pub fn check_libs(libs: &SdlConfig) -> Result<(), Box<dyn std::error::Error>> {
    match SUPPORTED_VERSIONS.contains(&libs.version.as_str()) {
        true => {}
        false => {
            return Err(format!("Unsupported version: {}", libs.version).into());
        }
    }
    
    for lib in &libs.sdl.libs {
        if !SUPPORTED_LIBS.contains(&lib.name.as_str()) {
            return Err(format!("Unsupported library: {}", lib.name).into());
        }
    }
    Ok(())
}

pub fn update_lib(lib: &mut Lib) -> Result<(), Box<dyn std::error::Error>> {
    let latest_v = get_latest_release(&lib.name)?;
    let version_parts: Vec<&str> = latest_v.split('-').collect();
    lib.version = version_parts[1].to_string();
    lib.status = version_parts[0].to_string();
    Ok(())
}