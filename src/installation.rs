use crate::package::{LibTag, SdlConfig};
use crate::path;
use crate::platform::Platform;
use crate::file::{clean_dir, cleanup, copy_dll, copy_include, copy_lib, init, init_dir, FileManager};
use crate::services::get_url_format;
use crate::command::{run_command, check_commands};

pub trait Installable {
    fn install(&self) -> Result<(), Box<dyn std::error::Error>>;
    fn init(&self) -> Result<(), Box<dyn std::error::Error>>;
    fn clean(&self) -> Result<(), Box<dyn std::error::Error>>;
}


pub struct SdlInstallation {
    pub libs: SdlConfig,
    pub only: Vec<LibTag>,
}


impl Installable for SdlInstallation {
    fn init(&self) -> Result<(), Box<dyn std::error::Error>> {
        
        match Platform::detect() {
            Platform::Windows => {
                init()?;
                let dirs: Vec<String> = vec![
                    "include".to_string(),
                    "lib".to_string(),
                    "bin".to_string(),
                ];
                init_dir(&dirs)?;
            }
            Platform::Linux => {
                init()?;
            }
            _ => println!("Platform not supported."),
            
        }
        Ok(())
    }
    fn install(&self) -> Result<(), Box<dyn std::error::Error>> {
        match Platform::detect() {
            Platform::Windows => {
                let arch = Platform::get_architecture();
                for lib in &self.libs.sdl.libs {
                    let lib_name = &lib.name;
                    let version = &lib.version;
                    let status = &lib.status;
            
                    let first_digit = version.chars().next().unwrap();
                    let name_without_sdl = lib_name.strip_prefix("SDL").unwrap_or(&lib_name);
                    let name_sdl = format!("SDL{}{}", first_digit, name_without_sdl);
                    let true_name = format!("{}-{}", name_sdl,version); // ex: SDL2image-2.0.5
            
                    let zip_file = format!("{}.zip", lib_name); // ex: SDL_image.zip
                    let extract_dir = lib_name.clone(); // ex: SDL_image
                    let zip_path = format!("{name_sdl}-{version}-win32-{arch}.zip"); // ex: SDL2image-2.0.5-win32-x64.zip
                    
                    let zip_path_vc = format!("{}-devel-{}-VC.zip", name_sdl, version); // ex: SDL2image-devel-2.0.5-VC.zip
                    let zip_file_vc = format!("{}-VC.zip", lib_name); // ex: SDL_image-VC.zip
            
                    let url = get_url_format(
                        &lib_name,&status,&version, &zip_path
                    );
            
                    self.download_and_extract(
                        &url, &zip_file, &extract_dir
                    )?;
            
                    // SDL VC
                    let url = get_url_format(
                        &lib_name,&status,&version, &zip_path_vc
                    );
            
                    self.download_and_extract(
                        &url, &zip_file_vc, &(extract_dir.clone()+"-VC")
                    )?;
            
                    //COPY
                    if self.only.is_empty() || self.only.contains(&LibTag::Include) {
                        copy_include(&extract_dir, &true_name)?;
                    }
                    if self.only.is_empty() || self.only.contains(&LibTag::Lib) {
                        copy_lib(&extract_dir, &true_name, &arch)?;
                    }
                    if self.only.is_empty() || self.only.contains(&LibTag::Bin) {
                        copy_dll(&extract_dir, &name_sdl)?;
                    }
            
                }
            }
            Platform::Linux => {
                
                let dependencies = [
                    "apt-get", "install", "-y",
                    "build-essential", "git", "make", "pkg-config", "cmake", "ninja-build",
                    "libasound2-dev", "libpulse-dev", "libx11-dev", "libxext-dev", "libxrandr-dev",
                    "libxcursor-dev", "libxfixes-dev", "libxi-dev", "libxss-dev", "libxkbcommon-dev",
                    "libdrm-dev", "libgbm-dev", "libgl1-mesa-dev", "libgles2-mesa-dev", "libegl1-mesa-dev",
                    "libdbus-1-dev"
                ];

                run_command("sudo", &dependencies)?;
                println!("Dependencies installed.");

                for lib in &self.libs.sdl.libs {
                    let lib_name = &lib.name;
                    let version = &lib.version;
                    let status = &lib.status;

                    let first_digit = version.chars().next().unwrap();
                    let name_without_sdl = lib_name.strip_prefix("SDL").unwrap_or(&lib_name).to_string();
                    let name_sdl = format!("SDL{}{}", first_digit, name_without_sdl);
                    let true_name = format!("{}-{}", name_sdl,version); 

                    let zip_file = format!("{}.tar.gz", lib_name); 
                    let extract_dir = "";
                    let zip_path = format!("{}-{}.tar.gz", name_sdl, version);
                    

                    let url = get_url_format(&lib_name,&status,&version, &zip_path);
            
                    self.download_and_extract(
                        &url, &zip_file, &extract_dir
                    )?;

                    let cm_list: &[(&str, &[String])] = &[
                        ("cd", &[path!(&true_name).to_str().unwrap_or("Invalid path").to_owned()]),
                        ("mkdir", &[String::from("build")]),
                        ("cd", &[String::from("build")]),
                        ("cmake", &[String::from("..")]),
                        ("make", &[]),
                        ("sudo", &[String::from("make"), String::from("install")]),
                    ];  

                    let commands: Vec<&str> = cm_list.iter().map(|(cmd, _)| *cmd).collect();
                    let result = check_commands(&commands)?;
                    if result != 0  {
                        println!("Please install commands and try again.");
                        return Ok(());
                    }

                    for (command, args) in cm_list {
                        run_command(command, &args.iter().map(|s| s.as_str()).collect::<Vec<_>>())?;
                    }

                    run_command("cd", &["../../.."])?;
                    if self.only.is_empty() || self.only.contains(&LibTag::Include) {
                        copy_include(&extract_dir, &true_name)?;
                    }
                } 

            }
            _ => println!("Platform not supported."),
        }
        Ok(())
    }
    fn clean(&self) -> Result<(), Box<dyn std::error::Error>> {
        match Platform::detect() {
            Platform::Windows => {
                cleanup()?;
                let dirs = vec![
                    "include".to_string(),
                    "lib".to_string(),
                    "bin".to_string(),
                ];
                clean_dir(&dirs)?;
            }
            Platform::Linux => {
                cleanup()?;
            }
            _ => println!("Platform not supported."),
        }
        Ok(())
    }
}
