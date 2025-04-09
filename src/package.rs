use std::fs;
use clap::ValueEnum;
use serde::Deserialize;
use serde::Serialize;
use crate::file::{copy_dll, copy_include, copy_lib, download_and_extract};
use crate::services::get_url_format;
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
    pub libs: Vec<LibEntry>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LibEntry {
    pub name: String,
    pub status: String,
    pub version: String,
}

pub struct SdlInstallation {
    pub libs: SdlConfig,
    pub only: Vec<LibTag>,
}

#[derive(Clone, Debug, ValueEnum, PartialEq, Display)]
pub enum LibTag {
    Bin,
    Include,
    Lib,
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
            arch: get_architecture(),
            libs: vec![
                LibEntry {
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

pub fn get_lib(lib_name: &str) -> Option<LibEntry> {
    let libs: SdlConfig = get_sdl_config();
    let mut new_lib: Option<LibEntry> = None;
    for lib in &libs.sdl.libs {
        if lib.name == lib_name {
            new_lib = Some(lib.clone());
            break;
        }
    }
   
    if new_lib.is_none() && SUPPORTED_LIBS.contains(&lib_name) {
        let latest_release = get_latest_release(lib_name).unwrap();
        let v: Vec<&str> = latest_release.split('-').collect();

        new_lib = Some(LibEntry {
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

pub fn process_installation(param: &SdlInstallation) -> Result<(), Box<dyn std::error::Error>> {
    let arch = get_architecture();
    for lib in &param.libs.sdl.libs {
        let lib_name = &lib.name;
        let version = &lib.version;
        let status = &lib.status;

        let first_digit = version.chars().next().unwrap();
        let name_without_sdl = lib_name.strip_prefix("SDL").unwrap_or(&lib_name);
        let name_sdl = format!("SDL{}{}", first_digit, name_without_sdl);

        let zip_file = format!("{}.zip", lib_name); // ex: SDL_image.zip
        let extract_dir = lib_name.clone(); // ex: SDL_image
        let zip_path = format!("{name_sdl}-{version}-win32-{arch}.zip"); // ex: SDL2image-2.0.5-win32-x64.zip
        let true_name = format!("{}-{}", name_sdl,version); // ex: SDL2image-2.0.5

        let zip_path_vc = format!("{}-devel-{}-VC.zip", name_sdl, version); // ex: SDL2image-devel-2.0.5-VC.zip
        let zip_file_vc = format!("{}-VC.zip", lib_name); // ex: SDL_image-VC.zip

        let url = get_url_format(
            &lib_name,&status,&version, &zip_path
        );

        download_and_extract(
            &url, &zip_file, &extract_dir
        )?;

        // SDL VC
        let url = get_url_format(
            &lib_name,&status,&version, &zip_path_vc
        );

        download_and_extract(
            &url, &zip_file_vc, &(extract_dir.clone()+"-VC")
        )?;

        //COPY
        if param.only.is_empty() || param.only.contains(&LibTag::Include) {
            copy_include(&extract_dir, &true_name)?;
        }
        if param.only.is_empty() || param.only.contains(&LibTag::Lib) {
            copy_lib(&extract_dir, &true_name, &arch)?;
        }
        if param.only.is_empty() || param.only.contains(&LibTag::Bin) {
            copy_dll(&extract_dir, &name_sdl)?;
        }

    }
    Ok(())
}

pub fn update_lib(lib: &mut LibEntry) -> Result<(), Box<dyn std::error::Error>> {
    let latest_v = get_latest_release(&lib.name)?;
    let version_parts: Vec<&str> = latest_v.split('-').collect();
    lib.version = version_parts[1].to_string();
    lib.status = version_parts[0].to_string();
    Ok(())
}