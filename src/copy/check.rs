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
    let config_path = format!("{}/{}", args[2], data.get_config_path().display());
    let config_path = PathBuf::from(config_path);
    if !config_path.exists() {
        println!("{} does not exist", { config_path.display() });
        exit(ExitCode::FileDoesNotExist.into());
    };
}
