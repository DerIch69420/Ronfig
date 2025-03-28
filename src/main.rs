mod config_file;

use config_file::{handle_file::{get_content, get_file}, parse_to_json::convert_to_json};

fn main() {
    let file_path = get_file();
    let data = get_content(&file_path);
    let config = convert_to_json(data);

    dbg!(&config);
}
