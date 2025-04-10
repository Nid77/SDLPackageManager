use crate::package::{Lib, LibTag, SdlConfig};
use crate::platform::{get_architecture, Platform};
use crate::file::{copy_dll, copy_include, copy_lib, download_and_extract};
use crate::services::get_url_format;
use crate::command::run_command;


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
               process_installation(self)?;
            }
            Platform::Linux => {
                /*
                
                git clone https://github.com/libsdl-org/SDL_mixer.git
                cd SDL_mixer
                mkdir build && cd build
                cmake ..
                make
                sudo make install

                 */
                let build_dir = ".";
                run_command("git",&["https://github.com/libsdl-org/SDL_mixer.git"])?; // clone
                run_command("cd", &["SDL_mixer"])?; // cd
                run_command("mkdir", &["build"])?; // mkdir build
                run_command("cd", &["build"])?; // cd build
                run_command("cmake", &[".."])?; // cmake ..
                run_command("make", &[])?; // make
                run_command("sudo", &["make", "install"])?; // sudo make install
                

            }
            _ => println!("Platform not supported."),
        }
        Ok(())
    }
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
