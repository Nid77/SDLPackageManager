
use reqwest::blocking::get;
use std::fs::File;
use std::io::{self};
use std::path::Path;
use fs_extra::dir::{copy, CopyOptions};
use std::error::Error;
use std::fs;
use std::process;

pub const TMP_PATH: &str = ".\\tmp\\";
pub const DEST_DIR: &str = ".";

fn create_dir_if_not_exists(path: &str) {
    if !Path::new(path).exists() {
        fs::create_dir_all(path).unwrap_or_else(|err| {
            eprintln!("Erreur de création de répertoire: {}", err);
            process::exit(1);
        });
    }
}

pub fn init() -> Result<(), Box<dyn std::error::Error>> {
    std::fs::create_dir_all(TMP_PATH)?;
    create_dir_if_not_exists(DEST_DIR);
    create_dir_if_not_exists(&format!("{}/include", DEST_DIR));
    create_dir_if_not_exists(&format!("{}/lib", DEST_DIR));
    create_dir_if_not_exists(&format!("{}/bin", DEST_DIR));
    Ok(())
}

pub fn cleanup() -> Result<(), Box<dyn std::error::Error>> {
    if Path::new(TMP_PATH).exists() {
        std::fs::remove_dir_all(TMP_PATH)?;
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
    if Path::new(&(TMP_PATH.to_owned()+destination)).exists() {
        return Err(format!("file {} already exist", destination).into());
    }

    println!("Downloading {} to {}", url, destination);
    let mut response = get(url).expect("request failed");
    if !response.status().is_success() {
        return Err(format!("Failed to download file: {}", response.status()).into());
    }
    let mut file = File::create(TMP_PATH.to_owned()+destination).expect("failed to create file");
    io::copy(&mut response, &mut file)?;
    Ok(())
}

pub fn extract_zip(zip_path: &str, extract_to: &str) -> Result<(), Box<dyn std::error::Error>> {
    if Path::new(&(TMP_PATH.to_owned()+extract_to)).exists() {
        return Err(format!("directory {} already exist", TMP_PATH.to_owned()+extract_to).into());
    } 

    if !Path::new(&(TMP_PATH.to_owned()+zip_path)).exists() {
        return Err(format!("file {} not found", TMP_PATH.to_owned()+zip_path).into());
    }

    println!("Extracting {} to {}", TMP_PATH.to_owned()+zip_path, TMP_PATH.to_owned()+extract_to);
    let zip_file = File::open(TMP_PATH.to_owned()+zip_path)?;
    let mut archive = zip::ZipArchive::new(zip_file)?;
    archive.extract(TMP_PATH.to_owned()+extract_to)?;
    
    Ok(())
}

pub fn copy_file(src: &str, dest: &str) -> Result<(), Box<dyn std::error::Error>> {
    if !Path::new(&(TMP_PATH.to_owned()+src)).exists() {
        return Err(format!("file {} not found", TMP_PATH.to_owned()+src).into());  
    } 
    std::fs::copy(TMP_PATH.to_owned()+src, dest)?;
    Ok(())

}

pub fn copy_dir_recursive(src: &str, dest: &str) -> Result<(), Box<dyn Error>> {
    let mut options = CopyOptions::new();
    options.overwrite = true;
    options.copy_inside = true;
    copy(TMP_PATH.to_owned()+src, dest, &options)?;

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