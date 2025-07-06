use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use std::process::exit;

pub fn get_file(args: &Vec<String>) -> PathBuf {
    // First argument is ronfig binary
    // Second argument is mode
    // Third argument is dir in which config.json is
    if args.len() > 3 {
        println!("Too many arguments!");
        println!("Try using \"help\"");
        exit(1);
    }

    // Form path of the config file
    let file_path: String = if args.len() == 2 {
        format!("{}", { "config.json" })
    } else {
        format!("{}{}", { &args[2] }, { "/config.json" })
    };
    // dbg!(&file_path);

    PathBuf::from(file_path)
}

pub fn get_content(file_path: &PathBuf) -> String {
    let mut file: File = File::open(file_path).expect("Failed to open file");
    let mut content: String = String::new();
    file.read_to_string(&mut content)
        .expect("Failed to read file");

    return content;
}
