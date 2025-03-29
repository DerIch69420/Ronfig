use std::fs::File;
use std::io::prelude::*;

pub fn get_file(args: Vec<String>) -> String {

    // Form path of the config file
    let file_path = format!("{}{}", { &args[1] }, { "/config.json" });
    // dbg!(args);



    return file_path;
}

pub fn get_content(file_path: String) -> String {
    let mut file = File::open(file_path).expect("Failed to open file");
    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("Failed to read file");

    // dbg!(&config);

    return content;
}
