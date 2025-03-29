use std::path::{Path, PathBuf};

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ConfigOptions {
    filename: PathBuf,
    desired_path: PathBuf,
}

impl ConfigOptions {
    pub fn get_filename(&self) -> &Path {
        &self.filename
    }

    pub fn get_desired_path(&self) -> &Path {
        &self.desired_path
    }
}
