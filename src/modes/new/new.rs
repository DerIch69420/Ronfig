use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

pub fn new(args: &Vec<String>) {
    let directory: PathBuf = PathBuf::from(&args[2]);

    if directory.exists() {
        println!("{} already exists!", directory.display());

        return;
    }

    fs::create_dir(&directory).expect("Couldn't create directory");

    let file_path: PathBuf = directory.join("config.json");
    let mut file: File = File::create(&file_path).expect("Couldn't create config.json");

    let config_data: &'static str = r#"[
  {
    "config_file_path": "my_config_file.conf",
    "desired_path": "my/config/location"
  },
  {
    "config_dir_path": "my_dir_to_copy",
    "desired_path": "other/config/location"
  }
]"#;

    file.write_all(config_data.as_bytes())
        .expect("Couldn't write to config.json");
}
