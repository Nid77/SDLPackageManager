mod package;
use crate::package::run_install;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    run_install();
    Ok(())
}
