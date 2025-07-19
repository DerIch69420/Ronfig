use std::{env, path::PathBuf};

use crate::config::config_options::ConfigOptions;

pub fn check_exists(data: &ConfigOptions, args: &Vec<String>) -> bool {
    let home: String = env::var("HOME").expect("Failed to get HOME env variable");

    let desired_path: PathBuf = PathBuf::from(home).join(data.get_desired_path());

    if !desired_path.is_dir() {
        println!("{} does not exist", desired_path.display());
        return false;
    }

    // args[2] -> directory which contains the file
    let config_path: String = if args.len() == 3 {
        format!("{}/{}", args[2], data.get_config_path().display())
    } else {
        // If run without any directory use the current directory
        format!("{}", data.get_config_path().display())
    };
    let config_path: PathBuf = PathBuf::from(config_path);
    if !config_path.exists() {
        println!("{} does not exist", { config_path.display() });
        return false;
    };

    return true;
}
