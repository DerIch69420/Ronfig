use std::{
    env,
    fs::{self, create_dir},
    path::PathBuf,
};

use crate::config::config_options::ConfigOptions;

fn copy_from_to(source: PathBuf, destination: PathBuf) {
    if source.is_dir() {
        // Handle copying directories recursively
        for entry in fs::read_dir(source).expect("Failed to read directory") {
            let entry = entry.expect("Failed to read entry");
            let entry_path = entry.path();
            let new_entry_path = destination.join(entry.file_name());

            if entry_path.is_dir() {
                fs::create_dir_all(&new_entry_path).expect("Failed to create directory");
                copy_from_to(entry_path, new_entry_path);
            } else {
                fs::copy(entry_path, new_entry_path).expect("Failed to copy file");
            }
        }
    } else {
        // Handle copying individual files
        fs::copy(&source, &destination).expect("Failed to copy file");
    }
}

pub fn copy_file(data: &ConfigOptions, args: &Vec<String>) {
    let filename = format!("{}/{}", args[2], data.get_config_path().display());
    let filename = PathBuf::from(filename);

    let home = env::var("HOME").expect("Failed to get HOME env variable");
    // dbg!(&home);
    let new_file = PathBuf::from(home)
        .join(data.get_desired_path())
        .join(data.get_config_path());

    fs::copy(&filename, &new_file).unwrap_or_else(|e| {
        eprintln!("Failed to copy file {}: {}", filename.display(), e);
        std::process::exit(1); // Exit the program with error
    });
    println!("Copied {} to {}", { filename.display() }, {
        new_file.display()
    })
}

pub fn copy_dir(data: &ConfigOptions, args: &Vec<String>) {
    let directory = format!("{}/{}", args[2], data.get_config_path().display());
    let directory = PathBuf::from(directory);
    // dbg!(&directory);

    let home = env::var("HOME").expect("Failed to get HOME env variable");
    let new_dir = PathBuf::from(home)
        .join(data.get_desired_path())
        .join(data.get_config_path());
    // dbg!(&new_dir);

    let _ = create_dir(&new_dir);

    copy_from_to(directory.clone(), new_dir.clone());
    println!(
        "Copied directory from {} to {}",
        directory.display(),
        new_dir.display()
    );
}
