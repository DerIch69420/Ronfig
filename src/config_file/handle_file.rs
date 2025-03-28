use std::env;

use std::fs::File;
use std::io::prelude::*;

pub fn get_file() -> String {
    // Get passed arguments
    let args: Vec<String> = env::args().collect();
    // arg[0] -> location of binary
    // dbg!(&args);

    // Create path of the config file
    let file_path = format!("{}{}", { &args[1] }, { "/config" });
    // dbg!(args);

    return file_path;
}

pub fn get_content(file_path: &str) -> String {
    let mut file = File::open(file_path).expect("Failed to open file");
    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("Failed to read file");

    // dbg!(&config);

    return content;
}
