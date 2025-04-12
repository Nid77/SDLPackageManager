use std::collections::HashMap;
use std::process::Command;
use std::io::{self, Write};
use std::error::Error;

const INTERNAL_COMMANDS: &[&str] = &[
    "sudo",
    "cd",
    "ls",
    "mkdir",
];

pub fn run_command(command: &str, args: &[&str]) -> Result<(), Box<dyn std::error::Error>> {
    println!("Running command: {} {:?}", command, args);
    if command == "cd" {
        std::env::set_current_dir(args[0])?;
        return Ok(());
    }

    let output = Command::new(&command)
        .args(&*args)
        .output()?;

    if !output.status.success() {
        eprintln!("Command failed: {}", command);
        io::stderr().write_all(&output.stderr)?;
    }

    if !output.stdout.is_empty() {
        println!("Output of command {}: ", command);
        io::stdout().write_all(&output.stdout)?;
    }
    
    Ok(())
}

fn check_command(command: &str) -> Result<(), Box<dyn std::error::Error>> {

    if INTERNAL_COMMANDS.contains(&command) {
        return Ok(());
    }

    let output = Command::new(&command)
        .arg("--version") 
        .output();

    match output {
        Ok(_) => {
            Ok(())
        }
        Err(_) => {
            println!("{} is not installed or not found in PATH.", command);
            Ok(())
        }
    }
}

pub fn check_commands(cm_list: &[(&str, &[&str])]) -> Result<i32, Box<dyn Error>> {
    let mut cache: HashMap<&str, bool> = HashMap::new();
    let mut num = 0;
    for &(command, _) in cm_list {
        if !cache.get(command).copied().unwrap_or(false) {
            match check_command(command){
                Ok(_) => {
                }
                Err(_) => {
                    num = -1;
                }
            }
            cache.insert(command, true);
        }
    }
    Ok(num)
}
