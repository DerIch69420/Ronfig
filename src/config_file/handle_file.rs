use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use std::process::exit;

use crate::exit_codes::codes::ExitCode;

pub fn get_file(args: &Vec<String>) -> PathBuf {
    // First argument is binary
    // Second argument is mode
    // Third argument is dir in which config.json is
    if args.len() > 3 {
        println!("Too many arguments!");
        println!("Try using \"help\"");
        exit(ExitCode::TooManyArguments.into());
    }

    // Form path of the config file
    let file_path = format!("{}{}", { &args[2] }, { "/config.json" });

    // dbg!(&file_path);

    PathBuf::from(file_path)
}

pub fn get_content(file_path: &PathBuf) -> String {
    let mut file = File::open(file_path).expect("Failed to open file");
    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("Failed to read file");

    content
}
