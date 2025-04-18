
use reqwest::blocking::get;
use std::fs::File;
use std::io::{self};
use std::path::Path;
use fs_extra::dir::{copy, CopyOptions};
use std::error::Error;
use std::fs;
use std::process;
use std::path::PathBuf;

pub fn tmp_path() -> &'static Path {
    Path::new("./tmp")
}
pub const DEST_DIR: &str = ".";

#[macro_export] // Make macro to use tmp_path() for all files
macro_rules! path {
    ($($seg:expr),+ $(,)?) => {{
        let mut p = std::path::PathBuf::from(crate::file::tmp_path());
        $( p.push($seg); )+
        p
    }};
}



fn create_dir_if_not_exists(path: &str) {
    if !Path::new(path).exists() {
        fs::create_dir_all(path).unwrap_or_else(|err| {
            eprintln!("Erreur de création de répertoire: {}", err);
            process::exit(1);
        });
    }
}

pub fn init() -> Result<(), Box<dyn std::error::Error>> {
    std::fs::create_dir_all(tmp_path())?;
    create_dir_if_not_exists(DEST_DIR);
    create_dir_if_not_exists(&format!("{}/include", DEST_DIR));
    create_dir_if_not_exists(&format!("{}/lib", DEST_DIR));
    create_dir_if_not_exists(&format!("{}/bin", DEST_DIR));
    Ok(())
}

pub fn cleanup() -> Result<(), Box<dyn std::error::Error>> {
    if Path::new(tmp_path()).exists() {
        std::fs::remove_dir_all(tmp_path())?;
    }
    Ok(())
}

pub fn clean_lib() -> Result<(), Box<dyn std::error::Error>> {
    std::fs::remove_dir_all(&format!("{}/bin", DEST_DIR))?;
    std::fs::remove_dir_all(&format!("{}/lib", DEST_DIR))?;
    std::fs::remove_dir_all(&format!("{}/include", DEST_DIR))?;
    Ok(())
}

pub fn download_file(url: &str, destination: &str) -> Result<(), Box<dyn std::error::Error>> {
    if Path::new(&(path!(destination))).exists() {
        return Err(format!("file {:?} already exists", path!(destination)).into());
    }

    println!("Downloading {} to {}", url, destination);
    let mut response = get(url).expect("request failed");
    if !response.status().is_success() {
        return Err(format!("Failed to download file: {}", response.status()).into());
    }
    let mut file = File::create(path!(destination)).expect("failed to create file");
    io::copy(&mut response, &mut file)?;
    Ok(())
}

pub fn extract_zip(zip_path: &str, extract_to: &str) -> Result<(), Box<dyn std::error::Error>> {
    if Path::new(&(path!(extract_to))).exists() {
        return Err(format!("directory {} already exists", path!(extract_to).display()).into());
    } 

    if !Path::new(&(path!(zip_path))).exists() {
        return Err(format!("file {} not found", path!(zip_path).display()).into());
    }

    println!("Extracting {:?} to {:?}", path!(zip_path), path!(extract_to));
    let zip_file = File::open(path!(zip_path))?;
    let mut archive = zip::ZipArchive::new(zip_file)?;
    archive.extract(path!(extract_to))?;
    
    Ok(())
}

pub fn copy_file(src: &str, dest: &str) -> Result<(), Box<dyn std::error::Error>> {
    if !Path::new(&(path!(src))).exists() {
        return Err(format!("file {} not found", path!(src).display()).into());  
    } 
    std::fs::copy(path!(src), dest)?;
    Ok(())

}

pub fn copy_dir_recursive(src: &str, dest: &str) -> Result<(), Box<dyn Error>> {
    let mut options = CopyOptions::new();
    options.overwrite = true;
    options.copy_inside = true;
    copy(path!(src), dest, &options)?;

    Ok(())
}

pub fn download_and_extract(
    url: &str,
    zip_file: &str,
    extract_dir: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    match download_file(url, zip_file) {
        Ok(_) => {}
        Err(e) => eprintln!("Error downloading file: {}", e),
    }

    match extract_zip(zip_file, extract_dir) {
        Ok(_) => {}
        Err(e) => eprintln!("Error extracting zip: {}", e),
    }

    Ok(())
}



pub fn copy_dll(extract_dir: &str, name_sdl: &str) -> Result<(), Box<dyn std::error::Error>> {
    let dll_path = PathBuf::from(extract_dir).join(format!("{name_sdl}.dll"));
    let target_dll = PathBuf::from(DEST_DIR).join("bin").join(format!("{name_sdl}.dll"));
    
    match copy_file(dll_path.to_str().unwrap(), target_dll.to_str().unwrap()) {
        Ok(_) => {}
        Err(e) => eprintln!("Error copying file: {}", e),
    }

    Ok(())
}

pub fn copy_include(extract_dir: &str, true_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let include_src = PathBuf::from(format!("{extract_dir}-VC"))
        .join(true_name)
        .join("include");
    let include_dst = PathBuf::from(DEST_DIR);

    match copy_dir_recursive(include_src.to_str().unwrap(), include_dst.to_str().unwrap()) {
        Ok(_) => {}
        Err(e) => eprintln!("Error copying directory: {}", e),
    }

    Ok(())
}

pub fn copy_lib(extract_dir: &str, true_name: &str, arch: &str) -> Result<(), Box<dyn std::error::Error>> {
    let lib_src = PathBuf::from(format!("{extract_dir}-VC"))
        .join(true_name)
        .join("lib")
        .join(arch);
    let lib_dst = PathBuf::from(DEST_DIR).join("lib");

    match copy_dir_recursive(lib_src.to_str().unwrap(), lib_dst.to_str().unwrap()) {
        Ok(_) => {}
        Err(e) => eprintln!("Error copying directory: {}", e),
    }

    Ok(())
}
