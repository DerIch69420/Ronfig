use std::{env, path::PathBuf, process::exit};

use crate::{config::config_options::ConfigOptions, exit_codes::codes::ExitCode};

pub fn check_exists(data: &ConfigOptions, args: &Vec<String>) {
    let home = env::var("HOME").expect("Failed to get HOME env variable");
    let desired_path = PathBuf::from(home).join(data.get_desired_path());

    if !desired_path.is_dir() {
        println!("{} does not exist", desired_path.display());
        exit(ExitCode::DirectoryDoesNotExist.into());
    }

    // args[2] -> directory which contains the file
    let filename = format!("{}/{}", args[2], data.get_filename().display());
    let filename = PathBuf::from(filename);
    if !filename.exists() {
        println!("{} does not exist", { filename.display() });
        exit(ExitCode::FileDoesNotExist.into());
    };
}
