use std::path::{Path, PathBuf};

use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum ConfigOptions {
    File {
        config_file_path: PathBuf,
        desired_path: PathBuf,
    },
    Directory {
        config_dir_path: PathBuf,
        desired_path: PathBuf,
    },
}

impl ConfigOptions {
    pub fn get_desired_path(&self) -> &Path {
        match self {
            ConfigOptions::File { desired_path, .. } => desired_path,
            ConfigOptions::Directory { desired_path, .. } => desired_path,
        }
    }

    pub fn get_config_path(&self) -> &Path {
        match self {
            ConfigOptions::File {
                config_file_path, ..
            } => config_file_path,
            ConfigOptions::Directory {
                config_dir_path, ..
            } => config_dir_path,
        }
    }
}
