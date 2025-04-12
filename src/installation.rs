use crate::package::{LibTag, SdlConfig};
use crate::path;
use crate::platform::Platform;
use crate::file::{copy_dll, copy_include, copy_lib, FileManager};
use crate::services::get_url_format;
use crate::command::{run_command, check_commands};

pub trait Installable {
    fn install(&self) -> Result<(), Box<dyn std::error::Error>>;
}


pub struct SdlInstallation {
    pub libs: SdlConfig,
    pub only: Vec<LibTag>,
}


impl Installable for SdlInstallation {
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
            
                    let zip_file = format!("{}.zip", lib_name); // ex: SDL_image.zip
                    let extract_dir = lib_name.clone(); // ex: SDL_image
                    let zip_path = format!("{name_sdl}-{version}-win32-{arch}.zip"); // ex: SDL2image-2.0.5-win32-x64.zip
                    let true_name = format!("{}-{}", name_sdl,version); // ex: SDL2image-2.0.5
            
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

                let zip_file = format!("{}.tar.gz", "SDL"); 
                let extract_dir = "SDL";
                let zip_path = format!("SDL3-3.2.10.tar.gz");
                let lib_name = "SDL";
                let version = "3.2.10";
                let status = "release";

                let url = get_url_format(&lib_name,&status,&version, &zip_path);
        
                self.download_and_extract(
                    &url, &zip_file, &extract_dir
                )?;

                let cm_list: &[(&str, &[&str])] = &[
                    // ("cd", &[path!("SDL").to_str().unwrap()]),
                    // ("mkdir", &[path!("build").to_str().unwrap()]),
                    // ("cd", &[path!("build").to_str().unwrap()]),
                    ("cmake", &[".."]),
                    ("make", &[]),
                    ("sudo", &["make", "install"]),
                ];

                let result = check_commands(cm_list)?;
                if result != 0  {
                    println!("Please install commands and try again.");
                    return Ok(());
                }

                for (command, args) in cm_list {
                    run_command(command, args)?;
                    println!("Command {} executed successfully.", command);
                }
                

            }
            _ => println!("Platform not supported."),
        }
        Ok(())
    }
}
