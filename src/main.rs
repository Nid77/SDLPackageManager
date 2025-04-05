mod package;
mod file;
use crate::package::run_install;
use crate::file::download_file;
use crate::file::extract_zip;
use crate::file::init;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    init()?;
    download_file("https://github.com/libsdl-org/SDL_mixer/releases/download/release-2.8.1/SDL2_mixer-2.8.1-win32-x86.zip","dest.zip")?;
    extract_zip("dest.zip", "dest")?;
    Ok(())
}
