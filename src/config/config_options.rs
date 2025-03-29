use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ConfigOptions {
    filename: String,
    desired_path: String,
}

impl ConfigOptions {
    pub fn get_filename(&self) -> String {
        self.filename.clone()
    }

    pub fn get_desired_path(&self) -> String {
        self.desired_path.clone()
    }
}
