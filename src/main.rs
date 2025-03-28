mod args;
mod config;
mod config_file;
mod copy;
mod exit_codes;

use args::args::Mode;
use args::{args::get_args, handle_args::handle_mode};
use config_file::{
    convert_to_json::convert_to_json,
    handle_file::{get_content, get_file},
};
use copy::check::check_exists;
use copy::copy_file::copy_file;

fn main() {
    let args = get_args();
    let mode = handle_mode(args.clone());

    if mode == Mode::Copy {
        let file_path = get_file(&args);
        let data = get_content(&file_path);
        let config = convert_to_json(&data);

        check_exists(&config, &args);
        copy_file(&config, &args);
    }
}
