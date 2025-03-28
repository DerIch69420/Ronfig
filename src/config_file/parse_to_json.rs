use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ConfigOptions {
    filename: String,
    desired_path: String,
}

pub fn convert_to_json(data: String) -> ConfigOptions {
    let config: ConfigOptions = serde_json::from_str(&data).expect("Failed to convert to JSON");

    return config;
}
