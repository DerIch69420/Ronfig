mod args;
mod config;
mod config_file;
mod modes;

use std::path::PathBuf;

use args::{args::get_args, handle_args::handle_mode};
use config::config_options::ConfigOptions;
use config_file::{
    convert_to_json::convert_to_json,
    handle_file::{get_content, get_file},
};

use modes::copy::copy::{copy_dir, copy_file};
use modes::help::help::help;
use modes::invalid::invalid::invalid;
use modes::modes::Mode;
use modes::{copy::check::check_exists, new::new::new};

fn main() {
    let args: Vec<String> = get_args();
    let mode: Mode = handle_mode(&args[1]);

    if mode == Mode::Indvalid {
        invalid();
    }

    if mode == Mode::Help {
        help();
    }

    if mode == Mode::New {
        if args.len() != 3 {
            println!("Wrong amount of arguments");
            return;
        }
        new(&args);
    }

    if mode == Mode::Copy {
        if args.len() != 2 && args.len() != 3 {
            println!("Wrong amount of arguments");
            return;
        }

        // Get configuration
        let file_path: PathBuf = get_file(&args);
        let data: String = get_content(&file_path);
        let config: Vec<ConfigOptions> = convert_to_json(&data);

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
