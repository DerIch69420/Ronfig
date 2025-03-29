use std::{env, fs, path::PathBuf};

use crate::config::config_options::ConfigOptions;

pub fn copy_file(data: &ConfigOptions, args: &Vec<String>) {
    let filename = format!("{}/{}", args[2], data.get_filename().display());
    let filename = PathBuf::from(filename);

    let home = env::var("HOME").expect("Failed to get HOME env variable");
    // dbg!(&home);
    let new_file = PathBuf::from(home)
        .join(data.get_desired_path())
        .join(data.get_filename());

    fs::copy(filename, new_file).expect("Could not copy file");
}
