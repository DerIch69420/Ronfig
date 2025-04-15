use crate::modes::modes::Mode;


pub fn handle_mode(args: Vec<String>) -> Mode {
    let mode = Mode::from_str(&args[1].to_lowercase());

    match mode {
        Mode::Help => return Mode::Help,
        Mode::New => return Mode::New,
        Mode::Copy => return Mode::Copy,
        Mode::Indvalid => return Mode::Indvalid,
    }
}

