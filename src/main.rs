mod package;
mod file;
use crate::file::download_file;
use crate::file::extract_zip;
use crate::file::init;
use crate::file::cleanup;
use crate::file::copy_file;
use crate::file::copy_dir_recursive;
use crate::file::DEST_DIR;
use crate::package::get_architecture;
use crate::package::get_sdl_config;
use crate::package::SdlConfig;
use std::fs;



fn get_url_format(lib_name: &str, status: &str, version: &str, zip_path: &str) -> String {
    format!(
        "https://github.com/libsdl-org/{}/releases/download/{}-{}/{}",
        lib_name, status, version, zip_path
    )
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    init()?;

    let arch = get_architecture();

    let libs: SdlConfig = get_sdl_config();

    for lib in libs.sdl.libs {
        let lib_name = lib.name;
        let version = lib.version;
        let status = lib.channel;

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

        match download_file(&url, &zip_file) {
            Ok(_) => {}
            Err(e) => eprintln!("Error downloading file: {}", e),
        }

        match extract_zip(&zip_file, &extract_dir) {
            Ok(_) => {}
            Err(e) => eprintln!("Error extracting zip: {}", e),
        }

        // SDL VC
        let url = get_url_format(
            &lib_name,&status,&version, &zip_path_vc
        );

        match download_file(&url, &zip_file_vc) {
            Ok(_) => {}
            Err(e) => eprintln!("Error downloading file: {}", e),
        }

        match extract_zip(&zip_file_vc, &(extract_dir.clone()+"-VC")) {
            Ok(_) => {}
            Err(e) => eprintln!("Error extracting zip: {}", e),
        }

        //COPY
        let dll_path = format!("{extract_dir}\\{name_sdl}.dll");
        let target_dll = format!("{DEST_DIR}\\bin\\{name_sdl}.dll");
        
        match copy_file(&dll_path, &target_dll) {
            Ok(_) => {}
            Err(e) => eprintln!("Error copying file: {}", e),
        }

        let include_src = format!("{extract_dir}-VC\\{true_name}\\include");
        let include_dst = format!("{DEST_DIR}\\include");
        match copy_dir_recursive(&include_src, &include_dst) {
            Ok(_) => {}
            Err(e) => eprintln!("Error copying directory: {}", e),
        }

        let lib_src = format!("{extract_dir}-VC\\{true_name}\\lib\\{arch}");
        let lib_dst = format!("{DEST_DIR}\\lib");
        match  copy_dir_recursive(&lib_src, &lib_dst) {
            Ok(_) => {}
            Err(e) => eprintln!("Error copying directory: {}", e),
        }

        
    }

    cleanup()?;

    Ok(())
}
