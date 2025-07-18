use crate::modes::modes::Mode;

pub fn handle_mode(args: &String) -> Mode {
    let mode: Mode = Mode::from_str(&args.to_lowercase());

    match mode {
        Mode::Help => return Mode::Help,
        Mode::New => return Mode::New,
        Mode::Copy => return Mode::Copy,
        Mode::Indvalid => return Mode::Indvalid,
    }
}
