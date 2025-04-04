mod args;
mod config;
mod config_file;
mod copy;
mod exit_codes;

use args::args::Mode;
use args::{args::get_args, handle_args::handle_mode};
use config::config_options::ConfigOptions;
use config_file::{
    convert_to_json::convert_to_json,
    handle_file::{get_content, get_file},
};
use copy::check::check_exists;
use copy::copy::{copy_dir, copy_file};

fn main() {
    let args = get_args();
    let mode = handle_mode(args.clone());

    if mode == Mode::Copy {
        // Get configuration
        let file_path = get_file(&args);
        let data = get_content(&file_path);
        let config = convert_to_json(&data);

        // Copy everything to desired locations
        for configuration in &config {
            check_exists(&configuration, &args);

            match configuration {
                ConfigOptions::File {
                    config_file_path: _,
                    desired_path: _,
                } => {
                    copy_file(&configuration, &args);
                }
                ConfigOptions::Directory {
                    config_dir_path: _,
                    desired_path: _,
                } => {
                    copy_dir(&configuration, &args);
                }
            }

        }
    }
}
