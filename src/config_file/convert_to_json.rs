use super::super::config::config_options::ConfigOptions;

pub fn convert_to_json(data: &String) -> Vec<ConfigOptions> {

    // dbg!(&data);
    
    let config: Vec<ConfigOptions> =
        serde_json::from_str(&data).expect("Failed to convert to JSON");

    // dbg!(&config);

    return config;
}
