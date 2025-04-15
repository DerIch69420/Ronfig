// Enum for handling the Mode of the application
// For the first argument passed after binary
#[derive(PartialEq)]
pub enum Mode {
    Help,
    New,
    Copy,
    Indvalid,
}

impl Mode {
    pub fn from_str(arg: &str) -> Self {
        match arg {
            "help" => Mode::Help,
            "new" => Mode::New,
            "copy" => Mode::Copy,
            _ => Mode::Indvalid,
        }
    }
}
