use std::{
    fs::{self, File},
    path::PathBuf,
};
pub fn new(args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    let directory: PathBuf = PathBuf::from(&args[2]);

    if directory.exists() {
        println!("{} already exists!", directory.display());
        return Ok(());
    }

    fs::create_dir(&directory)?;
    let file_path: PathBuf = directory.join("config.json");
    File::create(&file_path)?;

    println!(
        "Created directory and config.json at {}",
        file_path.display()
    );
    Ok(())
}
