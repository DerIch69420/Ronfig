mod handle_args;
mod config_file;
mod execute;
mod config;

use config_file::{handle_file::{get_content, get_file}, convert_to_json::convert_to_json};
use execute::check::check_exists;
use handle_args::args::handle_args;

fn main() {
    let args = handle_args();
    let file_path = get_file(args);
    let data = get_content(file_path);
    let config = convert_to_json(data);

    check_exists(config);

}
