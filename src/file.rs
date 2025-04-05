
use reqwest::blocking::get;
use std::fs::File;
use std::io::{self, Write};

const TMP_PATH: &str = "./tmp/";

pub fn init() -> Result<(), Box<dyn std::error::Error>> {
    std::fs::create_dir_all(TMP_PATH)?;
    Ok(())
}

pub fn download_file(url: &str, destination: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut response = get(url).expect("request failed");
    let mut file = File::create(TMP_PATH.to_owned()+destination).expect("failed to create file");
    io::copy(&mut response, &mut file)?;
    Ok(())
}

pub fn extract_zip(zip_path: &str, extract_to: &str) -> Result<(), Box<dyn std::error::Error>> {
    let zip_file = File::open(TMP_PATH.to_owned()+zip_path)?;
    let mut archive = zip::ZipArchive::new(zip_file)?;
    archive.extract(extract_to)?;
    Ok(())
}