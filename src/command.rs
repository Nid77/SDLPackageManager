use std::process::{Command, exit};

pub fn run_command(command: &str, args: &[&str]) -> Result<(), Box<dyn std::error::Error>> {
    let output = Command::new(command)
        .args(args)
        .output()?;

    if !output.status.success() {
        eprintln!("Command failed with status: {}", output.status);
        eprintln!("Error output: {}", String::from_utf8_lossy(&output.stderr));
        return Err("Command execution failed".into());
    }
    if !output.stdout.is_empty() {
        println!("Command output: {}", String::from_utf8_lossy(&output.stdout));
    }
    Ok(())
}
