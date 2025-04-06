use reqwest::blocking::Client;
use serde::Deserialize;


#[derive(Debug, Deserialize)]
struct GithubRelease {
    tag_name: String,
}


pub fn get_url_format(lib_name: &str, status: &str, version: &str, zip_path: &str) -> String {
    format!(
        "https://github.com/libsdl-org/{}/releases/download/{}-{}/{}",
        lib_name, status, version, zip_path
    )
}


pub fn get_latest_release(repo: &str) -> Result<String, Box<dyn std::error::Error>> {
    let url = format!("https://api.github.com/repos/libsdl-org/{}/releases/latest", repo);
    let client = Client::new();

    let res = client
        .get(&url)
        .header("User-Agent", "sdlpkg-manager") 
        .send()?
        .json::<GithubRelease>()?;

    Ok(res.tag_name)
}